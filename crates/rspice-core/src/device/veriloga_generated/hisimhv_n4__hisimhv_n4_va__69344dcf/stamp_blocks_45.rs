#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_345(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92360_e141548, assign92360_e141548_d_n0, assign92360_e141548_d_n2, assign92360_e141548_d_n4, assign92360_e141548_d_n5, assign92360_e141548_d_n6, assign92360_e141548_d_n7, assign92360_e141548_d_n8, assign92360_e141548_d_n9, assign92360_e141548_d_n10, assign92360_e141548_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2144 == 0.0)) {
        let (assign92360_e141546, assign92360_e141546_d_n0, assign92360_e141546_d_n2, assign92360_e141546_d_n4, assign92360_e141546_d_n5, assign92360_e141546_d_n6, assign92360_e141546_d_n7, assign92360_e141546_d_n8, assign92360_e141546_d_n9, assign92360_e141546_d_n10, assign92360_e141546_d_n13,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
            }
        };
        (assign92360_e141546, assign92360_e141546_d_n0, assign92360_e141546_d_n2, assign92360_e141546_d_n4, assign92360_e141546_d_n5, assign92360_e141546_d_n6, assign92360_e141546_d_n7, assign92360_e141546_d_n8, assign92360_e141546_d_n9, assign92360_e141546_d_n10, assign92360_e141546_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign92360_e141548;
        locals.var_chi_1_dn0 = assign92360_e141548_d_n0;
        locals.var_chi_1_dn2 = assign92360_e141548_d_n2;
        locals.var_chi_1_dn4 = assign92360_e141548_d_n4;
        locals.var_chi_1_dn5 = assign92360_e141548_d_n5;
        locals.var_chi_1_dn6 = assign92360_e141548_d_n6;
        locals.var_chi_1_dn7 = assign92360_e141548_d_n7;
        locals.var_chi_1_dn8 = assign92360_e141548_d_n8;
        locals.var_chi_1_dn9 = assign92360_e141548_d_n9;
        locals.var_chi_1_dn10 = assign92360_e141548_d_n10;
        locals.var_chi_1_dn13 = assign92360_e141548_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign92370_e141564, assign92370_e141564_d_n0, assign92370_e141564_d_n2, assign92370_e141564_d_n4, assign92370_e141564_d_n5, assign92370_e141564_d_n6, assign92370_e141564_d_n7, assign92370_e141564_d_n8, assign92370_e141564_d_n9, assign92370_e141564_d_n10, assign92370_e141564_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) {
        let (assign92370_e141562, assign92370_e141562_d_n0, assign92370_e141562_d_n2, assign92370_e141562_d_n4, assign92370_e141562_d_n5, assign92370_e141562_d_n6, assign92370_e141562_d_n7, assign92370_e141562_d_n8, assign92370_e141562_d_n9, assign92370_e141562_d_n10, assign92370_e141562_d_n13,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign92370_e141562, assign92370_e141562_d_n0, assign92370_e141562_d_n2, assign92370_e141562_d_n4, assign92370_e141562_d_n5, assign92370_e141562_d_n6, assign92370_e141562_d_n7, assign92370_e141562_d_n8, assign92370_e141562_d_n9, assign92370_e141562_d_n10, assign92370_e141562_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign92370_e141564;
        locals.var_chi_1_dn0 = assign92370_e141564_d_n0;
        locals.var_chi_1_dn2 = assign92370_e141564_d_n2;
        locals.var_chi_1_dn4 = assign92370_e141564_d_n4;
        locals.var_chi_1_dn5 = assign92370_e141564_d_n5;
        locals.var_chi_1_dn6 = assign92370_e141564_d_n6;
        locals.var_chi_1_dn7 = assign92370_e141564_d_n7;
        locals.var_chi_1_dn8 = assign92370_e141564_d_n8;
        locals.var_chi_1_dn9 = assign92370_e141564_d_n9;
        locals.var_chi_1_dn10 = assign92370_e141564_d_n10;
        locals.var_chi_1_dn13 = assign92370_e141564_d_n13;
        locals.var_chi_1_rv = 0.0;

        let (assign92380_e141577, assign92380_e141577_d_n0, assign92380_e141577_d_n2, assign92380_e141577_d_n4, assign92380_e141577_d_n5, assign92380_e141577_d_n6, assign92380_e141577_d_n7, assign92380_e141577_d_n8, assign92380_e141577_d_n9, assign92380_e141577_d_n10, assign92380_e141577_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign92380_e141575: f64 = (locals.var_psi - locals.var_chi_1);
        (assign92380_e141575, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign92380_e141577;
        locals.var_psi_dn0 = assign92380_e141577_d_n0;
        locals.var_psi_dn2 = assign92380_e141577_d_n2;
        locals.var_psi_dn4 = assign92380_e141577_d_n4;
        locals.var_psi_dn5 = assign92380_e141577_d_n5;
        locals.var_psi_dn6 = assign92380_e141577_d_n6;
        locals.var_psi_dn7 = assign92380_e141577_d_n7;
        locals.var_psi_dn8 = assign92380_e141577_d_n8;
        locals.var_psi_dn9 = assign92380_e141577_d_n9;
        locals.var_psi_dn10 = assign92380_e141577_d_n10;
        locals.var_psi_dn13 = assign92380_e141577_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign92390_e141592, assign92390_e141592_d_n0, assign92390_e141592_d_n2, assign92390_e141592_d_n4, assign92390_e141592_d_n5, assign92390_e141592_d_n6, assign92390_e141592_d_n7, assign92390_e141592_d_n8, assign92390_e141592_d_n9, assign92390_e141592_d_n10, assign92390_e141592_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign92390_e141589: f64 = (locals.var_beta * 0.1);
        let assign92390_e141590: f64 = (locals.var_psi + assign92390_e141589);
        (assign92390_e141590, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn13 + (locals.var_beta_dn13 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign92390_e141592;
        locals.var_psi_dn0 = assign92390_e141592_d_n0;
        locals.var_psi_dn2 = assign92390_e141592_d_n2;
        locals.var_psi_dn4 = assign92390_e141592_d_n4;
        locals.var_psi_dn5 = assign92390_e141592_d_n5;
        locals.var_psi_dn6 = assign92390_e141592_d_n6;
        locals.var_psi_dn7 = assign92390_e141592_d_n7;
        locals.var_psi_dn8 = assign92390_e141592_d_n8;
        locals.var_psi_dn9 = assign92390_e141592_d_n9;
        locals.var_psi_dn10 = assign92390_e141592_d_n10;
        locals.var_psi_dn13 = assign92390_e141592_d_n13;
        locals.var_psi_rv = 0.0;

        let (assign92400_e141615, assign92400_e141615_d_n0, assign92400_e141615_d_n2, assign92400_e141615_d_n4, assign92400_e141615_d_n5, assign92400_e141615_d_n6, assign92400_e141615_d_n7, assign92400_e141615_d_n8, assign92400_e141615_d_n9, assign92400_e141615_d_n10, assign92400_e141615_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign92400_e141603: f64 = (locals.var_gammachi * locals.var_t0);
        let assign92400_e141606: f64 = (locals.var_psi * locals.var_psi);
        let assign92400_e141607: f64 = (assign92400_e141603 + assign92400_e141606);
        let assign92400_e141608: f64 = (assign92400_e141607).ln();
        let assign92400_e141611: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign92400_e141612: f64 = (assign92400_e141611).ln();
        let assign92400_e141613: f64 = (assign92400_e141608 - assign92400_e141612);
        (assign92400_e141613, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign92400_e141607) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign92400_e141611)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign92400_e141607) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign92400_e141611)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign92400_e141607) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign92400_e141611)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign92400_e141607) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign92400_e141611)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign92400_e141607) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign92400_e141611)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign92400_e141607) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign92400_e141611)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign92400_e141607) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign92400_e141611)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign92400_e141607) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign92400_e141611)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign92400_e141607) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign92400_e141611)), (((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign92400_e141607) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign92400_e141611)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign92400_e141615;
        locals.var_t1_dn0 = assign92400_e141615_d_n0;
        locals.var_t1_dn2 = assign92400_e141615_d_n2;
        locals.var_t1_dn4 = assign92400_e141615_d_n4;
        locals.var_t1_dn5 = assign92400_e141615_d_n5;
        locals.var_t1_dn6 = assign92400_e141615_d_n6;
        locals.var_t1_dn7 = assign92400_e141615_d_n7;
        locals.var_t1_dn8 = assign92400_e141615_d_n8;
        locals.var_t1_dn9 = assign92400_e141615_d_n9;
        locals.var_t1_dn10 = assign92400_e141615_d_n10;
        locals.var_t1_dn13 = assign92400_e141615_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign92410_e141630, assign92410_e141630_d_n0, assign92410_e141630_d_n2, assign92410_e141630_d_n4, assign92410_e141630_d_n5, assign92410_e141630_d_n6, assign92410_e141630_d_n7, assign92410_e141630_d_n8, assign92410_e141630_d_n9, assign92410_e141630_d_n10, assign92410_e141630_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) {
        let assign92410_e141627: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign92410_e141628: f64 = (locals.var_t1 + assign92410_e141627);
        (assign92410_e141628, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn13 + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign92410_e141630;
        locals.var_chi_b_dn0 = assign92410_e141630_d_n0;
        locals.var_chi_b_dn2 = assign92410_e141630_d_n2;
        locals.var_chi_b_dn4 = assign92410_e141630_d_n4;
        locals.var_chi_b_dn5 = assign92410_e141630_d_n5;
        locals.var_chi_b_dn6 = assign92410_e141630_d_n6;
        locals.var_chi_b_dn7 = assign92410_e141630_d_n7;
        locals.var_chi_b_dn8 = assign92410_e141630_d_n8;
        locals.var_chi_b_dn9 = assign92410_e141630_d_n9;
        locals.var_chi_b_dn10 = assign92410_e141630_d_n10;
        locals.var_chi_b_dn13 = assign92410_e141630_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign92420_e141646, assign92420_e141646_d_n0, assign92420_e141646_d_n2, assign92420_e141646_d_n4, assign92420_e141646_d_n5, assign92420_e141646_d_n6, assign92420_e141646_d_n7, assign92420_e141646_d_n8, assign92420_e141646_d_n9, assign92420_e141646_d_n10, assign92420_e141646_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) {
        let (assign92420_e141644, assign92420_e141644_d_n0, assign92420_e141644_d_n2, assign92420_e141644_d_n4, assign92420_e141644_d_n5, assign92420_e141644_d_n6, assign92420_e141644_d_n7, assign92420_e141644_d_n8, assign92420_e141644_d_n9, assign92420_e141644_d_n10, assign92420_e141644_d_n13,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign92420_e141644, assign92420_e141644_d_n0, assign92420_e141644_d_n2, assign92420_e141644_d_n4, assign92420_e141644_d_n5, assign92420_e141644_d_n6, assign92420_e141644_d_n7, assign92420_e141644_d_n8, assign92420_e141644_d_n9, assign92420_e141644_d_n10, assign92420_e141644_d_n13,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign92420_e141646;
        locals.var_chi_b_dn0 = assign92420_e141646_d_n0;
        locals.var_chi_b_dn2 = assign92420_e141646_d_n2;
        locals.var_chi_b_dn4 = assign92420_e141646_d_n4;
        locals.var_chi_b_dn5 = assign92420_e141646_d_n5;
        locals.var_chi_b_dn6 = assign92420_e141646_d_n6;
        locals.var_chi_b_dn7 = assign92420_e141646_d_n7;
        locals.var_chi_b_dn8 = assign92420_e141646_d_n8;
        locals.var_chi_b_dn9 = assign92420_e141646_d_n9;
        locals.var_chi_b_dn10 = assign92420_e141646_d_n10;
        locals.var_chi_b_dn13 = assign92420_e141646_d_n13;
        locals.var_chi_b_rv = 0.0;

        let (assign92430_e141657, assign92430_e141657_d_n0, assign92430_e141657_d_n2, assign92430_e141657_d_n4, assign92430_e141657_d_n5, assign92430_e141657_d_n6, assign92430_e141657_d_n7, assign92430_e141657_d_n8, assign92430_e141657_d_n9, assign92430_e141657_d_n10, assign92430_e141657_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign92430_e141657;
        locals.var_chi_a_dn0 = assign92430_e141657_d_n0;
        locals.var_chi_a_dn2 = assign92430_e141657_d_n2;
        locals.var_chi_a_dn4 = assign92430_e141657_d_n4;
        locals.var_chi_a_dn5 = assign92430_e141657_d_n5;
        locals.var_chi_a_dn6 = assign92430_e141657_d_n6;
        locals.var_chi_a_dn7 = assign92430_e141657_d_n7;
        locals.var_chi_a_dn8 = assign92430_e141657_d_n8;
        locals.var_chi_a_dn9 = assign92430_e141657_d_n9;
        locals.var_chi_a_dn10 = assign92430_e141657_d_n10;
        locals.var_chi_a_dn13 = assign92430_e141657_d_n13;
        locals.var_chi_a_rv = 0.0;

        let assign92440_e141660: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2145 = assign92440_e141660;
        locals.var_guard2145_rv = 0.0;

        let assign92450_e141665: f64 = (0.2 * locals.var_chi_b);
        let assign92450_e141666: f64 = (locals.var_chi_b - assign92450_e141665);
        let assign92450_e141670: f64 = (0.2 * locals.var_chi_b);
        let assign92450_e141673: f64 = if ((locals.var_chi_a > assign92450_e141666) && (assign92450_e141670 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2146 = assign92450_e141673;
        locals.var_guard2146_rv = 0.0;

        let (assign92460_e141694, assign92460_e141694_d_n0, assign92460_e141694_d_n2, assign92460_e141694_d_n4, assign92460_e141694_d_n5, assign92460_e141694_d_n6, assign92460_e141694_d_n7, assign92460_e141694_d_n8, assign92460_e141694_d_n9, assign92460_e141694_d_n10, assign92460_e141694_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92460_e141688: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign92460_e141691: f64 = (0.2 * locals.var_chi_b);
        let assign92460_e141692: f64 = (assign92460_e141688 + assign92460_e141691);
        (assign92460_e141692, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn13 - locals.var_chi_b_dn13) + (0.2 * locals.var_chi_b_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign92460_e141694;
        locals.var_tmf1_dn0 = assign92460_e141694_d_n0;
        locals.var_tmf1_dn2 = assign92460_e141694_d_n2;
        locals.var_tmf1_dn4 = assign92460_e141694_d_n4;
        locals.var_tmf1_dn5 = assign92460_e141694_d_n5;
        locals.var_tmf1_dn6 = assign92460_e141694_d_n6;
        locals.var_tmf1_dn7 = assign92460_e141694_d_n7;
        locals.var_tmf1_dn8 = assign92460_e141694_d_n8;
        locals.var_tmf1_dn9 = assign92460_e141694_d_n9;
        locals.var_tmf1_dn10 = assign92460_e141694_d_n10;
        locals.var_tmf1_dn13 = assign92460_e141694_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign92470_e141711, assign92470_e141711_d_n0, assign92470_e141711_d_n2, assign92470_e141711_d_n4, assign92470_e141711_d_n5, assign92470_e141711_d_n6, assign92470_e141711_d_n7, assign92470_e141711_d_n8, assign92470_e141711_d_n9, assign92470_e141711_d_n10, assign92470_e141711_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92470_e141709: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign92470_e141709, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign92470_e141711;
        locals.var_x2_dn0 = assign92470_e141711_d_n0;
        locals.var_x2_dn2 = assign92470_e141711_d_n2;
        locals.var_x2_dn4 = assign92470_e141711_d_n4;
        locals.var_x2_dn5 = assign92470_e141711_d_n5;
        locals.var_x2_dn6 = assign92470_e141711_d_n6;
        locals.var_x2_dn7 = assign92470_e141711_d_n7;
        locals.var_x2_dn8 = assign92470_e141711_d_n8;
        locals.var_x2_dn9 = assign92470_e141711_d_n9;
        locals.var_x2_dn10 = assign92470_e141711_d_n10;
        locals.var_x2_dn13 = assign92470_e141711_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign92480_e141732, assign92480_e141732_d_n0, assign92480_e141732_d_n2, assign92480_e141732_d_n4, assign92480_e141732_d_n5, assign92480_e141732_d_n6, assign92480_e141732_d_n7, assign92480_e141732_d_n8, assign92480_e141732_d_n9, assign92480_e141732_d_n10, assign92480_e141732_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92480_e141726: f64 = (0.2 * locals.var_chi_b);
        let assign92480_e141729: f64 = (0.2 * locals.var_chi_b);
        let assign92480_e141730: f64 = (assign92480_e141726 * assign92480_e141729);
        (assign92480_e141730, (((0.2 * locals.var_chi_b_dn0) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn13) * assign92480_e141729) + (assign92480_e141726 * (0.2 * locals.var_chi_b_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign92480_e141732;
        locals.var_xmax2_dn0 = assign92480_e141732_d_n0;
        locals.var_xmax2_dn2 = assign92480_e141732_d_n2;
        locals.var_xmax2_dn4 = assign92480_e141732_d_n4;
        locals.var_xmax2_dn5 = assign92480_e141732_d_n5;
        locals.var_xmax2_dn6 = assign92480_e141732_d_n6;
        locals.var_xmax2_dn7 = assign92480_e141732_d_n7;
        locals.var_xmax2_dn8 = assign92480_e141732_d_n8;
        locals.var_xmax2_dn9 = assign92480_e141732_d_n9;
        locals.var_xmax2_dn10 = assign92480_e141732_d_n10;
        locals.var_xmax2_dn13 = assign92480_e141732_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign92490_e141747, assign92490_e141747_d_n0, assign92490_e141747_d_n2, assign92490_e141747_d_n4, assign92490_e141747_d_n5, assign92490_e141747_d_n6, assign92490_e141747_d_n7, assign92490_e141747_d_n8, assign92490_e141747_d_n9, assign92490_e141747_d_n10, assign92490_e141747_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign92490_e141747;
        locals.var_xp_dn0 = assign92490_e141747_d_n0;
        locals.var_xp_dn2 = assign92490_e141747_d_n2;
        locals.var_xp_dn4 = assign92490_e141747_d_n4;
        locals.var_xp_dn5 = assign92490_e141747_d_n5;
        locals.var_xp_dn6 = assign92490_e141747_d_n6;
        locals.var_xp_dn7 = assign92490_e141747_d_n7;
        locals.var_xp_dn8 = assign92490_e141747_d_n8;
        locals.var_xp_dn9 = assign92490_e141747_d_n9;
        locals.var_xp_dn10 = assign92490_e141747_d_n10;
        locals.var_xp_dn13 = assign92490_e141747_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign92500_e141762, assign92500_e141762_d_n0, assign92500_e141762_d_n2, assign92500_e141762_d_n4, assign92500_e141762_d_n5, assign92500_e141762_d_n6, assign92500_e141762_d_n7, assign92500_e141762_d_n8, assign92500_e141762_d_n9, assign92500_e141762_d_n10, assign92500_e141762_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign92500_e141762;
        locals.var_xmp_dn0 = assign92500_e141762_d_n0;
        locals.var_xmp_dn2 = assign92500_e141762_d_n2;
        locals.var_xmp_dn4 = assign92500_e141762_d_n4;
        locals.var_xmp_dn5 = assign92500_e141762_d_n5;
        locals.var_xmp_dn6 = assign92500_e141762_d_n6;
        locals.var_xmp_dn7 = assign92500_e141762_d_n7;
        locals.var_xmp_dn8 = assign92500_e141762_d_n8;
        locals.var_xmp_dn9 = assign92500_e141762_d_n9;
        locals.var_xmp_dn10 = assign92500_e141762_d_n10;
        locals.var_xmp_dn13 = assign92500_e141762_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign92510_e141777,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign92510_e141777;
        locals.var_m0_rv = 0.0;

        let (assign92520_e141792,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92520_e141792;
        locals.var_mm_rv = 0.0;

        let (assign92530_e141807, assign92530_e141807_d_n0, assign92530_e141807_d_n2, assign92530_e141807_d_n4, assign92530_e141807_d_n5, assign92530_e141807_d_n6, assign92530_e141807_d_n7, assign92530_e141807_d_n8, assign92530_e141807_d_n9, assign92530_e141807_d_n10, assign92530_e141807_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign92530_e141807;
        locals.var_arg_dn0 = assign92530_e141807_d_n0;
        locals.var_arg_dn2 = assign92530_e141807_d_n2;
        locals.var_arg_dn4 = assign92530_e141807_d_n4;
        locals.var_arg_dn5 = assign92530_e141807_d_n5;
        locals.var_arg_dn6 = assign92530_e141807_d_n6;
        locals.var_arg_dn7 = assign92530_e141807_d_n7;
        locals.var_arg_dn8 = assign92530_e141807_d_n8;
        locals.var_arg_dn9 = assign92530_e141807_d_n9;
        locals.var_arg_dn10 = assign92530_e141807_d_n10;
        locals.var_arg_dn13 = assign92530_e141807_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign92540_e141822, assign92540_e141822_d_n0, assign92540_e141822_d_n2, assign92540_e141822_d_n4, assign92540_e141822_d_n5, assign92540_e141822_d_n6, assign92540_e141822_d_n7, assign92540_e141822_d_n8, assign92540_e141822_d_n9, assign92540_e141822_d_n10, assign92540_e141822_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign92540_e141822;
        locals.var_dnm_dn0 = assign92540_e141822_d_n0;
        locals.var_dnm_dn2 = assign92540_e141822_d_n2;
        locals.var_dnm_dn4 = assign92540_e141822_d_n4;
        locals.var_dnm_dn5 = assign92540_e141822_d_n5;
        locals.var_dnm_dn6 = assign92540_e141822_d_n6;
        locals.var_dnm_dn7 = assign92540_e141822_d_n7;
        locals.var_dnm_dn8 = assign92540_e141822_d_n8;
        locals.var_dnm_dn9 = assign92540_e141822_d_n9;
        locals.var_dnm_dn10 = assign92540_e141822_d_n10;
        locals.var_dnm_dn13 = assign92540_e141822_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign92550_e141839, assign92550_e141839_d_n0, assign92550_e141839_d_n2, assign92550_e141839_d_n4, assign92550_e141839_d_n5, assign92550_e141839_d_n6, assign92550_e141839_d_n7, assign92550_e141839_d_n8, assign92550_e141839_d_n9, assign92550_e141839_d_n10, assign92550_e141839_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92550_e141837: f64 = (locals.var_xp * locals.var_x2);
        (assign92550_e141837, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign92550_e141839;
        locals.var_xp_dn0 = assign92550_e141839_d_n0;
        locals.var_xp_dn2 = assign92550_e141839_d_n2;
        locals.var_xp_dn4 = assign92550_e141839_d_n4;
        locals.var_xp_dn5 = assign92550_e141839_d_n5;
        locals.var_xp_dn6 = assign92550_e141839_d_n6;
        locals.var_xp_dn7 = assign92550_e141839_d_n7;
        locals.var_xp_dn8 = assign92550_e141839_d_n8;
        locals.var_xp_dn9 = assign92550_e141839_d_n9;
        locals.var_xp_dn10 = assign92550_e141839_d_n10;
        locals.var_xp_dn13 = assign92550_e141839_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign92560_e141856, assign92560_e141856_d_n0, assign92560_e141856_d_n2, assign92560_e141856_d_n4, assign92560_e141856_d_n5, assign92560_e141856_d_n6, assign92560_e141856_d_n7, assign92560_e141856_d_n8, assign92560_e141856_d_n9, assign92560_e141856_d_n10, assign92560_e141856_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92560_e141854: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign92560_e141854, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign92560_e141856;
        locals.var_xmp_dn0 = assign92560_e141856_d_n0;
        locals.var_xmp_dn2 = assign92560_e141856_d_n2;
        locals.var_xmp_dn4 = assign92560_e141856_d_n4;
        locals.var_xmp_dn5 = assign92560_e141856_d_n5;
        locals.var_xmp_dn6 = assign92560_e141856_d_n6;
        locals.var_xmp_dn7 = assign92560_e141856_d_n7;
        locals.var_xmp_dn8 = assign92560_e141856_d_n8;
        locals.var_xmp_dn9 = assign92560_e141856_d_n9;
        locals.var_xmp_dn10 = assign92560_e141856_d_n10;
        locals.var_xmp_dn13 = assign92560_e141856_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign92570_e141873, assign92570_e141873_d_n0, assign92570_e141873_d_n2, assign92570_e141873_d_n4, assign92570_e141873_d_n5, assign92570_e141873_d_n6, assign92570_e141873_d_n7, assign92570_e141873_d_n8, assign92570_e141873_d_n9, assign92570_e141873_d_n10, assign92570_e141873_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92570_e141871: f64 = (locals.var_xp * locals.var_x2);
        (assign92570_e141871, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign92570_e141873;
        locals.var_xp_dn0 = assign92570_e141873_d_n0;
        locals.var_xp_dn2 = assign92570_e141873_d_n2;
        locals.var_xp_dn4 = assign92570_e141873_d_n4;
        locals.var_xp_dn5 = assign92570_e141873_d_n5;
        locals.var_xp_dn6 = assign92570_e141873_d_n6;
        locals.var_xp_dn7 = assign92570_e141873_d_n7;
        locals.var_xp_dn8 = assign92570_e141873_d_n8;
        locals.var_xp_dn9 = assign92570_e141873_d_n9;
        locals.var_xp_dn10 = assign92570_e141873_d_n10;
        locals.var_xp_dn13 = assign92570_e141873_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign92580_e141890, assign92580_e141890_d_n0, assign92580_e141890_d_n2, assign92580_e141890_d_n4, assign92580_e141890_d_n5, assign92580_e141890_d_n6, assign92580_e141890_d_n7, assign92580_e141890_d_n8, assign92580_e141890_d_n9, assign92580_e141890_d_n10, assign92580_e141890_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92580_e141888: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign92580_e141888, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign92580_e141890;
        locals.var_xmp_dn0 = assign92580_e141890_d_n0;
        locals.var_xmp_dn2 = assign92580_e141890_d_n2;
        locals.var_xmp_dn4 = assign92580_e141890_d_n4;
        locals.var_xmp_dn5 = assign92580_e141890_d_n5;
        locals.var_xmp_dn6 = assign92580_e141890_d_n6;
        locals.var_xmp_dn7 = assign92580_e141890_d_n7;
        locals.var_xmp_dn8 = assign92580_e141890_d_n8;
        locals.var_xmp_dn9 = assign92580_e141890_d_n9;
        locals.var_xmp_dn10 = assign92580_e141890_d_n10;
        locals.var_xmp_dn13 = assign92580_e141890_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign92590_e141907, assign92590_e141907_d_n0, assign92590_e141907_d_n2, assign92590_e141907_d_n4, assign92590_e141907_d_n5, assign92590_e141907_d_n6, assign92590_e141907_d_n7, assign92590_e141907_d_n8, assign92590_e141907_d_n9, assign92590_e141907_d_n10, assign92590_e141907_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92590_e141905: f64 = (locals.var_xp + locals.var_xmp);
        (assign92590_e141905, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign92590_e141907;
        locals.var_arg_dn0 = assign92590_e141907_d_n0;
        locals.var_arg_dn2 = assign92590_e141907_d_n2;
        locals.var_arg_dn4 = assign92590_e141907_d_n4;
        locals.var_arg_dn5 = assign92590_e141907_d_n5;
        locals.var_arg_dn6 = assign92590_e141907_d_n6;
        locals.var_arg_dn7 = assign92590_e141907_d_n7;
        locals.var_arg_dn8 = assign92590_e141907_d_n8;
        locals.var_arg_dn9 = assign92590_e141907_d_n9;
        locals.var_arg_dn10 = assign92590_e141907_d_n10;
        locals.var_arg_dn13 = assign92590_e141907_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign92600_e141922, assign92600_e141922_d_n0, assign92600_e141922_d_n2, assign92600_e141922_d_n4, assign92600_e141922_d_n5, assign92600_e141922_d_n6, assign92600_e141922_d_n7, assign92600_e141922_d_n8, assign92600_e141922_d_n9, assign92600_e141922_d_n10, assign92600_e141922_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign92600_e141922;
        locals.var_dnm_dn0 = assign92600_e141922_d_n0;
        locals.var_dnm_dn2 = assign92600_e141922_d_n2;
        locals.var_dnm_dn4 = assign92600_e141922_d_n4;
        locals.var_dnm_dn5 = assign92600_e141922_d_n5;
        locals.var_dnm_dn6 = assign92600_e141922_d_n6;
        locals.var_dnm_dn7 = assign92600_e141922_d_n7;
        locals.var_dnm_dn8 = assign92600_e141922_d_n8;
        locals.var_dnm_dn9 = assign92600_e141922_d_n9;
        locals.var_dnm_dn10 = assign92600_e141922_d_n10;
        locals.var_dnm_dn13 = assign92600_e141922_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign92610_e141937: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2147 = assign92610_e141937;
        locals.var_guard2147_rv = 0.0;

        let assign92620_e141940: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2148 = assign92620_e141940;
        locals.var_guard2148_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_346(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92630_e141959,) = {
    if ((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92630_e141959;
        locals.var_mm_rv = 0.0;

        let assign92640_e141962: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2149 = assign92640_e141962;
        locals.var_guard2149_rv = 0.0;

        let (assign92650_e141984,) = {
    if (((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 == 0.0)) && (locals.var_guard2149 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92650_e141984;
        locals.var_mm_rv = 0.0;

        let assign92660_e141987: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2150 = assign92660_e141987;
        locals.var_guard2150_rv = 0.0;

        let (assign92670_e142012,) = {
    if ((((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 == 0.0)) && (locals.var_guard2149 == 0.0)) && (locals.var_guard2150 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92670_e142012;
        locals.var_mm_rv = 0.0;

        let assign92680_e142015: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2151 = assign92680_e142015;
        locals.var_guard2151_rv = 0.0;

        let (assign92690_e142043,) = {
    if (((((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_guard2148 == 0.0)) && (locals.var_guard2149 == 0.0)) && (locals.var_guard2150 == 0.0)) && (locals.var_guard2151 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign92690_e142043;
        locals.var_mm_rv = 0.0;

        let (assign92700_e142060,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign92700_e142060;
        locals.var_m0_rv = 0.0;

        let mut assign92710_loop_guard: usize = 0;
        while {
            let assign92710_cond_e142078: f64 = if ((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign92710_cond_e142078 != 0.0
        } {
            assign92710_loop_guard += 1;
            assert!(assign92710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign92710_body0_e142096, assign92710_body0_e142096_d_n0, assign92710_body0_e142096_d_n2, assign92710_body0_e142096_d_n4, assign92710_body0_e142096_d_n5, assign92710_body0_e142096_d_n6, assign92710_body0_e142096_d_n7, assign92710_body0_e142096_d_n8, assign92710_body0_e142096_d_n9, assign92710_body0_e142096_d_n10, assign92710_body0_e142096_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92710_body0_e142094: f64 = (locals.var_dnm).sqrt();
        (assign92710_body0_e142094, (locals.var_dnm_dn0 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn2 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn4 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn5 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn6 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn7 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn8 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn9 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn10 / (2.0 * assign92710_body0_e142094)), (locals.var_dnm_dn13 / (2.0 * assign92710_body0_e142094)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign92710_body0_e142096;
            locals.var_dnm_dn0 = assign92710_body0_e142096_d_n0;
            locals.var_dnm_dn2 = assign92710_body0_e142096_d_n2;
            locals.var_dnm_dn4 = assign92710_body0_e142096_d_n4;
            locals.var_dnm_dn5 = assign92710_body0_e142096_d_n5;
            locals.var_dnm_dn6 = assign92710_body0_e142096_d_n6;
            locals.var_dnm_dn7 = assign92710_body0_e142096_d_n7;
            locals.var_dnm_dn8 = assign92710_body0_e142096_d_n8;
            locals.var_dnm_dn9 = assign92710_body0_e142096_d_n9;
            locals.var_dnm_dn10 = assign92710_body0_e142096_d_n10;
            locals.var_dnm_dn13 = assign92710_body0_e142096_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign92710_body1_e142115,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 != 0.0)) {
        let assign92710_body1_e142113: f64 = (locals.var_m0 + 1.0);
        (assign92710_body1_e142113,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign92710_body1_e142115;
            locals.var_m0_rv = 0.0;
        }

        let (assign92720_e142144, assign92720_e142144_d_n0, assign92720_e142144_d_n2, assign92720_e142144_d_n4, assign92720_e142144_d_n5, assign92720_e142144_d_n6, assign92720_e142144_d_n7, assign92720_e142144_d_n8, assign92720_e142144_d_n9, assign92720_e142144_d_n10, assign92720_e142144_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) && (locals.var_guard2147 == 0.0)) {
        let (assign92720_e142142, assign92720_e142142_d_n0, assign92720_e142142_d_n2, assign92720_e142142_d_n4, assign92720_e142142_d_n5, assign92720_e142142_d_n6, assign92720_e142142_d_n7, assign92720_e142142_d_n8, assign92720_e142142_d_n9, assign92720_e142142_d_n10, assign92720_e142142_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign92720_e142139: f64 = (2.0 * 2.0);
                let assign92720_e142140: f64 = (1.0 / assign92720_e142139);
                let assign92720_e142141: f64 = (locals.var_dnm).powf(assign92720_e142140);
                (assign92720_e142141, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn0)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn2)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn4)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn5)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn6)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn7)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn8)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn9)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn10)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign92720_e142140) as f64).is_finite() && ((assign92720_e142140) as f64).fract() == 0.0 { if assign92720_e142140 == 0.0 { 0.0 } else { (assign92720_e142140 * ((locals.var_dnm).powf(assign92720_e142140 - 1.0) * locals.var_dnm_dn13)) } } else { (assign92720_e142141 * (assign92720_e142140 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign92720_e142142, assign92720_e142142_d_n0, assign92720_e142142_d_n2, assign92720_e142142_d_n4, assign92720_e142142_d_n5, assign92720_e142142_d_n6, assign92720_e142142_d_n7, assign92720_e142142_d_n8, assign92720_e142142_d_n9, assign92720_e142142_d_n10, assign92720_e142142_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign92720_e142144;
        locals.var_dnm_dn0 = assign92720_e142144_d_n0;
        locals.var_dnm_dn2 = assign92720_e142144_d_n2;
        locals.var_dnm_dn4 = assign92720_e142144_d_n4;
        locals.var_dnm_dn5 = assign92720_e142144_d_n5;
        locals.var_dnm_dn6 = assign92720_e142144_d_n6;
        locals.var_dnm_dn7 = assign92720_e142144_d_n7;
        locals.var_dnm_dn8 = assign92720_e142144_d_n8;
        locals.var_dnm_dn9 = assign92720_e142144_d_n9;
        locals.var_dnm_dn10 = assign92720_e142144_d_n10;
        locals.var_dnm_dn13 = assign92720_e142144_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign92730_e142161, assign92730_e142161_d_n0, assign92730_e142161_d_n2, assign92730_e142161_d_n4, assign92730_e142161_d_n5, assign92730_e142161_d_n6, assign92730_e142161_d_n7, assign92730_e142161_d_n8, assign92730_e142161_d_n9, assign92730_e142161_d_n10, assign92730_e142161_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92730_e142159: f64 = (1.0 / locals.var_dnm);
        (assign92730_e142159, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign92730_e142161;
        locals.var_dnm_dn0 = assign92730_e142161_d_n0;
        locals.var_dnm_dn2 = assign92730_e142161_d_n2;
        locals.var_dnm_dn4 = assign92730_e142161_d_n4;
        locals.var_dnm_dn5 = assign92730_e142161_d_n5;
        locals.var_dnm_dn6 = assign92730_e142161_d_n6;
        locals.var_dnm_dn7 = assign92730_e142161_d_n7;
        locals.var_dnm_dn8 = assign92730_e142161_d_n8;
        locals.var_dnm_dn9 = assign92730_e142161_d_n9;
        locals.var_dnm_dn10 = assign92730_e142161_d_n10;
        locals.var_dnm_dn13 = assign92730_e142161_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign92740_e142182, assign92740_e142182_d_n0, assign92740_e142182_d_n2, assign92740_e142182_d_n4, assign92740_e142182_d_n5, assign92740_e142182_d_n6, assign92740_e142182_d_n7, assign92740_e142182_d_n8, assign92740_e142182_d_n9, assign92740_e142182_d_n10, assign92740_e142182_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92740_e142177: f64 = (0.2 * locals.var_chi_b);
        let assign92740_e142178: f64 = (locals.var_tmf1 * assign92740_e142177);
        let assign92740_e142180: f64 = (assign92740_e142178 * locals.var_dnm);
        (assign92740_e142180, ((((locals.var_tmf1_dn0 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign92740_e142177) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn13))) * locals.var_dnm) + (assign92740_e142178 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign92740_e142182;
        locals.var_tmf0_dn0 = assign92740_e142182_d_n0;
        locals.var_tmf0_dn2 = assign92740_e142182_d_n2;
        locals.var_tmf0_dn4 = assign92740_e142182_d_n4;
        locals.var_tmf0_dn5 = assign92740_e142182_d_n5;
        locals.var_tmf0_dn6 = assign92740_e142182_d_n6;
        locals.var_tmf0_dn7 = assign92740_e142182_d_n7;
        locals.var_tmf0_dn8 = assign92740_e142182_d_n8;
        locals.var_tmf0_dn9 = assign92740_e142182_d_n9;
        locals.var_tmf0_dn10 = assign92740_e142182_d_n10;
        locals.var_tmf0_dn13 = assign92740_e142182_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign92750_e142205, assign92750_e142205_d_n0, assign92750_e142205_d_n2, assign92750_e142205_d_n4, assign92750_e142205_d_n5, assign92750_e142205_d_n6, assign92750_e142205_d_n7, assign92750_e142205_d_n8, assign92750_e142205_d_n9, assign92750_e142205_d_n10, assign92750_e142205_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92750_e142197: f64 = (0.2 * locals.var_chi_b);
        let assign92750_e142199: f64 = (assign92750_e142197 * locals.var_xmp);
        let assign92750_e142201: f64 = (assign92750_e142199 * locals.var_dnm);
        let assign92750_e142203: f64 = (assign92750_e142201 / locals.var_arg);
        (assign92750_e142203, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn0)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn2)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn4)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn5)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn6)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn7)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn8)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn9)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn10)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn13) * locals.var_xmp) + (assign92750_e142197 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign92750_e142199 * locals.var_dnm_dn13)) * locals.var_arg) - (assign92750_e142201 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign92750_e142205;
        locals.var_t1_dn0 = assign92750_e142205_d_n0;
        locals.var_t1_dn2 = assign92750_e142205_d_n2;
        locals.var_t1_dn4 = assign92750_e142205_d_n4;
        locals.var_t1_dn5 = assign92750_e142205_d_n5;
        locals.var_t1_dn6 = assign92750_e142205_d_n6;
        locals.var_t1_dn7 = assign92750_e142205_d_n7;
        locals.var_t1_dn8 = assign92750_e142205_d_n8;
        locals.var_t1_dn9 = assign92750_e142205_d_n9;
        locals.var_t1_dn10 = assign92750_e142205_d_n10;
        locals.var_t1_dn13 = assign92750_e142205_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign92760_e142226, assign92760_e142226_d_n0, assign92760_e142226_d_n2, assign92760_e142226_d_n4, assign92760_e142226_d_n5, assign92760_e142226_d_n6, assign92760_e142226_d_n7, assign92760_e142226_d_n8, assign92760_e142226_d_n9, assign92760_e142226_d_n10, assign92760_e142226_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        let assign92760_e142221: f64 = (0.2 * locals.var_chi_b);
        let assign92760_e142222: f64 = (locals.var_chi_b - assign92760_e142221);
        let assign92760_e142224: f64 = (assign92760_e142222 + locals.var_tmf0);
        (assign92760_e142224, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn13 - (0.2 * locals.var_chi_b_dn13)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign92760_e142226;
        locals.var_chi_dn0 = assign92760_e142226_d_n0;
        locals.var_chi_dn2 = assign92760_e142226_d_n2;
        locals.var_chi_dn4 = assign92760_e142226_d_n4;
        locals.var_chi_dn5 = assign92760_e142226_d_n5;
        locals.var_chi_dn6 = assign92760_e142226_d_n6;
        locals.var_chi_dn7 = assign92760_e142226_d_n7;
        locals.var_chi_dn8 = assign92760_e142226_d_n8;
        locals.var_chi_dn9 = assign92760_e142226_d_n9;
        locals.var_chi_dn10 = assign92760_e142226_d_n10;
        locals.var_chi_dn13 = assign92760_e142226_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign92770_e142241, assign92770_e142241_d_n0, assign92770_e142241_d_n2, assign92770_e142241_d_n4, assign92770_e142241_d_n5, assign92770_e142241_d_n6, assign92770_e142241_d_n7, assign92770_e142241_d_n8, assign92770_e142241_d_n9, assign92770_e142241_d_n10, assign92770_e142241_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign92770_e142241;
        locals.var_t1_dn0 = assign92770_e142241_d_n0;
        locals.var_t1_dn2 = assign92770_e142241_d_n2;
        locals.var_t1_dn4 = assign92770_e142241_d_n4;
        locals.var_t1_dn5 = assign92770_e142241_d_n5;
        locals.var_t1_dn6 = assign92770_e142241_d_n6;
        locals.var_t1_dn7 = assign92770_e142241_d_n7;
        locals.var_t1_dn8 = assign92770_e142241_d_n8;
        locals.var_t1_dn9 = assign92770_e142241_d_n9;
        locals.var_t1_dn10 = assign92770_e142241_d_n10;
        locals.var_t1_dn13 = assign92770_e142241_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign92780_e142257, assign92780_e142257_d_n0, assign92780_e142257_d_n2, assign92780_e142257_d_n4, assign92780_e142257_d_n5, assign92780_e142257_d_n6, assign92780_e142257_d_n7, assign92780_e142257_d_n8, assign92780_e142257_d_n9, assign92780_e142257_d_n10, assign92780_e142257_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign92780_e142257;
        locals.var_chi_dn0 = assign92780_e142257_d_n0;
        locals.var_chi_dn2 = assign92780_e142257_d_n2;
        locals.var_chi_dn4 = assign92780_e142257_d_n4;
        locals.var_chi_dn5 = assign92780_e142257_d_n5;
        locals.var_chi_dn6 = assign92780_e142257_d_n6;
        locals.var_chi_dn7 = assign92780_e142257_d_n7;
        locals.var_chi_dn8 = assign92780_e142257_d_n8;
        locals.var_chi_dn9 = assign92780_e142257_d_n9;
        locals.var_chi_dn10 = assign92780_e142257_d_n10;
        locals.var_chi_dn13 = assign92780_e142257_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign92790_e142273, assign92790_e142273_d_n0, assign92790_e142273_d_n2, assign92790_e142273_d_n4, assign92790_e142273_d_n5, assign92790_e142273_d_n6, assign92790_e142273_d_n7, assign92790_e142273_d_n8, assign92790_e142273_d_n9, assign92790_e142273_d_n10, assign92790_e142273_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 != 0.0)) && (locals.var_guard2146 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign92790_e142273;
        locals.var_t1_dn0 = assign92790_e142273_d_n0;
        locals.var_t1_dn2 = assign92790_e142273_d_n2;
        locals.var_t1_dn4 = assign92790_e142273_d_n4;
        locals.var_t1_dn5 = assign92790_e142273_d_n5;
        locals.var_t1_dn6 = assign92790_e142273_d_n6;
        locals.var_t1_dn7 = assign92790_e142273_d_n7;
        locals.var_t1_dn8 = assign92790_e142273_d_n8;
        locals.var_t1_dn9 = assign92790_e142273_d_n9;
        locals.var_t1_dn10 = assign92790_e142273_d_n10;
        locals.var_t1_dn13 = assign92790_e142273_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign92800_e142292, assign92800_e142292_d_n0, assign92800_e142292_d_n2, assign92800_e142292_d_n4, assign92800_e142292_d_n5, assign92800_e142292_d_n6, assign92800_e142292_d_n7, assign92800_e142292_d_n8, assign92800_e142292_d_n9, assign92800_e142292_d_n10, assign92800_e142292_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2143 != 0.0)) && (locals.var_guard2145 == 0.0)) {
        let (assign92800_e142290, assign92800_e142290_d_n0, assign92800_e142290_d_n2, assign92800_e142290_d_n4, assign92800_e142290_d_n5, assign92800_e142290_d_n6, assign92800_e142290_d_n7, assign92800_e142290_d_n8, assign92800_e142290_d_n9, assign92800_e142290_d_n10, assign92800_e142290_d_n13,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            }
        };
        (assign92800_e142290, assign92800_e142290_d_n0, assign92800_e142290_d_n2, assign92800_e142290_d_n4, assign92800_e142290_d_n5, assign92800_e142290_d_n6, assign92800_e142290_d_n7, assign92800_e142290_d_n8, assign92800_e142290_d_n9, assign92800_e142290_d_n10, assign92800_e142290_d_n13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign92800_e142292;
        locals.var_chi_dn0 = assign92800_e142292_d_n0;
        locals.var_chi_dn2 = assign92800_e142292_d_n2;
        locals.var_chi_dn4 = assign92800_e142292_d_n4;
        locals.var_chi_dn5 = assign92800_e142292_d_n5;
        locals.var_chi_dn6 = assign92800_e142292_d_n6;
        locals.var_chi_dn7 = assign92800_e142292_d_n7;
        locals.var_chi_dn8 = assign92800_e142292_d_n8;
        locals.var_chi_dn9 = assign92800_e142292_d_n9;
        locals.var_chi_dn10 = assign92800_e142292_d_n10;
        locals.var_chi_dn13 = assign92800_e142292_d_n13;
        locals.var_chi_rv = 0.0;

        let assign92810_e142295: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2152 = assign92810_e142295;
        locals.var_guard2152_rv = 0.0;

        let (assign92820_e142310, assign92820_e142310_d_n0, assign92820_e142310_d_n2, assign92820_e142310_d_n4, assign92820_e142310_d_n5, assign92820_e142310_d_n6, assign92820_e142310_d_n7, assign92820_e142310_d_n8, assign92820_e142310_d_n9, assign92820_e142310_d_n10, assign92820_e142310_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign92820_e142306: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign92820_e142308: f64 = (assign92820_e142306 - locals.var_vxbgmtcl);
        (assign92820_e142308, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign92820_e142310;
        locals.var_ps0ld_dn0 = assign92820_e142310_d_n0;
        locals.var_ps0ld_dn2 = assign92820_e142310_d_n2;
        locals.var_ps0ld_dn4 = assign92820_e142310_d_n4;
        locals.var_ps0ld_dn5 = assign92820_e142310_d_n5;
        locals.var_ps0ld_dn6 = assign92820_e142310_d_n6;
        locals.var_ps0ld_dn7 = assign92820_e142310_d_n7;
        locals.var_ps0ld_dn8 = assign92820_e142310_d_n8;
        locals.var_ps0ld_dn9 = assign92820_e142310_d_n9;
        locals.var_ps0ld_dn10 = assign92820_e142310_d_n10;
        locals.var_ps0ld_dn13 = assign92820_e142310_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign92830_e142313: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2153 = assign92830_e142313;
        locals.var_guard2153_rv = 0.0;

        let (assign92840_e142328, assign92840_e142328_d_n0, assign92840_e142328_d_n2, assign92840_e142328_d_n4, assign92840_e142328_d_n5, assign92840_e142328_d_n6, assign92840_e142328_d_n7, assign92840_e142328_d_n8, assign92840_e142328_d_n9, assign92840_e142328_d_n10, assign92840_e142328_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2153 != 0.0)) {
        let assign92840_e142326: f64 = (p.p334 - locals.var_wdep_func);
        (assign92840_e142326, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign92840_e142328;
        locals.var_t2_dn0 = assign92840_e142328_d_n0;
        locals.var_t2_dn2 = assign92840_e142328_d_n2;
        locals.var_t2_dn4 = assign92840_e142328_d_n4;
        locals.var_t2_dn5 = assign92840_e142328_d_n5;
        locals.var_t2_dn6 = assign92840_e142328_d_n6;
        locals.var_t2_dn7 = assign92840_e142328_d_n7;
        locals.var_t2_dn8 = assign92840_e142328_d_n8;
        locals.var_t2_dn9 = assign92840_e142328_d_n9;
        locals.var_t2_dn10 = assign92840_e142328_d_n10;
        locals.var_t2_dn13 = assign92840_e142328_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign92850_e142355, assign92850_e142355_d_n0, assign92850_e142355_d_n2, assign92850_e142355_d_n4, assign92850_e142355_d_n5, assign92850_e142355_d_n6, assign92850_e142355_d_n7, assign92850_e142355_d_n8, assign92850_e142355_d_n9, assign92850_e142355_d_n10, assign92850_e142355_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2153 == 0.0)) {
        let assign92850_e142342: f64 = (locals.var_vdsi + p.p137);
        let assign92850_e142345: f64 = (locals.var_vdsi + p.p137);
        let assign92850_e142346: f64 = (assign92850_e142342 * assign92850_e142345);
        let assign92850_e142349: f64 = (4.0 * 0.1);
        let assign92850_e142351: f64 = (assign92850_e142349 * 0.1);
        let assign92850_e142352: f64 = (assign92850_e142346 + assign92850_e142351);
        let assign92850_e142353: f64 = (assign92850_e142352).sqrt();
        (assign92850_e142353, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign92850_e142345) + (assign92850_e142342 * locals.var_vdsi_dn5)) / (2.0 * assign92850_e142353)), 0.0, (((locals.var_vdsi_dn7 * assign92850_e142345) + (assign92850_e142342 * locals.var_vdsi_dn7)) / (2.0 * assign92850_e142353)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign92850_e142355;
        locals.var_tmf2_dn0 = assign92850_e142355_d_n0;
        locals.var_tmf2_dn2 = assign92850_e142355_d_n2;
        locals.var_tmf2_dn4 = assign92850_e142355_d_n4;
        locals.var_tmf2_dn5 = assign92850_e142355_d_n5;
        locals.var_tmf2_dn6 = assign92850_e142355_d_n6;
        locals.var_tmf2_dn7 = assign92850_e142355_d_n7;
        locals.var_tmf2_dn8 = assign92850_e142355_d_n8;
        locals.var_tmf2_dn9 = assign92850_e142355_d_n9;
        locals.var_tmf2_dn10 = assign92850_e142355_d_n10;
        locals.var_tmf2_dn13 = assign92850_e142355_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign92860_e142377, assign92860_e142377_d_n0, assign92860_e142377_d_n2, assign92860_e142377_d_n4, assign92860_e142377_d_n5, assign92860_e142377_d_n6, assign92860_e142377_d_n7, assign92860_e142377_d_n8, assign92860_e142377_d_n9, assign92860_e142377_d_n10, assign92860_e142377_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2153 == 0.0)) {
        let assign92860_e142371: f64 = (locals.var_vdsi + p.p137);
        let assign92860_e142373: f64 = (assign92860_e142371 / locals.var_tmf2);
        let assign92860_e142374: f64 = (1.0 + assign92860_e142373);
        let assign92860_e142375: f64 = (0.5 * assign92860_e142374);
        (assign92860_e142375, (0.5 * (-((assign92860_e142371 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92860_e142371 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92860_e142371 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign92860_e142371 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign92860_e142371 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign92860_e142371 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign92860_e142371 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92860_e142371 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92860_e142371 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign92860_e142371 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign92860_e142377;
        locals.var_t9_dn0 = assign92860_e142377_d_n0;
        locals.var_t9_dn2 = assign92860_e142377_d_n2;
        locals.var_t9_dn4 = assign92860_e142377_d_n4;
        locals.var_t9_dn5 = assign92860_e142377_d_n5;
        locals.var_t9_dn6 = assign92860_e142377_d_n6;
        locals.var_t9_dn7 = assign92860_e142377_d_n7;
        locals.var_t9_dn8 = assign92860_e142377_d_n8;
        locals.var_t9_dn9 = assign92860_e142377_d_n9;
        locals.var_t9_dn10 = assign92860_e142377_d_n10;
        locals.var_t9_dn13 = assign92860_e142377_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign92870_e142397, assign92870_e142397_d_n0, assign92870_e142397_d_n2, assign92870_e142397_d_n4, assign92870_e142397_d_n5, assign92870_e142397_d_n6, assign92870_e142397_d_n7, assign92870_e142397_d_n8, assign92870_e142397_d_n9, assign92870_e142397_d_n10, assign92870_e142397_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2153 == 0.0)) {
        let assign92870_e142392: f64 = (locals.var_vdsi + p.p137);
        let assign92870_e142394: f64 = (assign92870_e142392 + locals.var_tmf2);
        let assign92870_e142395: f64 = (0.5 * assign92870_e142394);
        (assign92870_e142395, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign92870_e142397;
        locals.var_t2_dn0 = assign92870_e142397_d_n0;
        locals.var_t2_dn2 = assign92870_e142397_d_n2;
        locals.var_t2_dn4 = assign92870_e142397_d_n4;
        locals.var_t2_dn5 = assign92870_e142397_d_n5;
        locals.var_t2_dn6 = assign92870_e142397_d_n6;
        locals.var_t2_dn7 = assign92870_e142397_d_n7;
        locals.var_t2_dn8 = assign92870_e142397_d_n8;
        locals.var_t2_dn9 = assign92870_e142397_d_n9;
        locals.var_t2_dn10 = assign92870_e142397_d_n10;
        locals.var_t2_dn13 = assign92870_e142397_d_n13;
        locals.var_t2_rv = 0.0;

        let assign92880_e142400: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2154 = assign92880_e142400;
        locals.var_guard2154_rv = 0.0;

        let (assign92890_e142416, assign92890_e142416_d_n0, assign92890_e142416_d_n2, assign92890_e142416_d_n4, assign92890_e142416_d_n5, assign92890_e142416_d_n6, assign92890_e142416_d_n7, assign92890_e142416_d_n8, assign92890_e142416_d_n9, assign92890_e142416_d_n10, assign92890_e142416_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2153 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign92890_e142416;
        locals.var_t2_dn0 = assign92890_e142416_d_n0;
        locals.var_t2_dn2 = assign92890_e142416_d_n2;
        locals.var_t2_dn4 = assign92890_e142416_d_n4;
        locals.var_t2_dn5 = assign92890_e142416_d_n5;
        locals.var_t2_dn6 = assign92890_e142416_d_n6;
        locals.var_t2_dn7 = assign92890_e142416_d_n7;
        locals.var_t2_dn8 = assign92890_e142416_d_n8;
        locals.var_t2_dn9 = assign92890_e142416_d_n9;
        locals.var_t2_dn10 = assign92890_e142416_d_n10;
        locals.var_t2_dn13 = assign92890_e142416_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign92900_e142432, assign92900_e142432_d_n0, assign92900_e142432_d_n2, assign92900_e142432_d_n4, assign92900_e142432_d_n5, assign92900_e142432_d_n6, assign92900_e142432_d_n7, assign92900_e142432_d_n8, assign92900_e142432_d_n9, assign92900_e142432_d_n10, assign92900_e142432_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2153 == 0.0)) && (locals.var_guard2154 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign92900_e142432;
        locals.var_t9_dn0 = assign92900_e142432_d_n0;
        locals.var_t9_dn2 = assign92900_e142432_d_n2;
        locals.var_t9_dn4 = assign92900_e142432_d_n4;
        locals.var_t9_dn5 = assign92900_e142432_d_n5;
        locals.var_t9_dn6 = assign92900_e142432_d_n6;
        locals.var_t9_dn7 = assign92900_e142432_d_n7;
        locals.var_t9_dn8 = assign92900_e142432_d_n8;
        locals.var_t9_dn9 = assign92900_e142432_d_n9;
        locals.var_t9_dn10 = assign92900_e142432_d_n10;
        locals.var_t9_dn13 = assign92900_e142432_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign92910_e142451, assign92910_e142451_d_n0, assign92910_e142451_d_n2, assign92910_e142451_d_n4, assign92910_e142451_d_n5, assign92910_e142451_d_n6, assign92910_e142451_d_n7, assign92910_e142451_d_n8, assign92910_e142451_d_n9, assign92910_e142451_d_n10, assign92910_e142451_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2153 == 0.0)) {
        let assign92910_e142446: f64 = (locals.var_kjunc * locals.var_t2);
        let assign92910_e142447: f64 = (assign92910_e142446).sqrt();
        let assign92910_e142449: f64 = (assign92910_e142447 * p.p432);
        (assign92910_e142449, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign92910_e142447)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign92910_e142447)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign92910_e142451;
        locals.var_wjunc0_dn0 = assign92910_e142451_d_n0;
        locals.var_wjunc0_dn2 = assign92910_e142451_d_n2;
        locals.var_wjunc0_dn4 = assign92910_e142451_d_n4;
        locals.var_wjunc0_dn5 = assign92910_e142451_d_n5;
        locals.var_wjunc0_dn6 = assign92910_e142451_d_n6;
        locals.var_wjunc0_dn7 = assign92910_e142451_d_n7;
        locals.var_wjunc0_dn8 = assign92910_e142451_d_n8;
        locals.var_wjunc0_dn9 = assign92910_e142451_d_n9;
        locals.var_wjunc0_dn10 = assign92910_e142451_d_n10;
        locals.var_wjunc0_dn13 = assign92910_e142451_d_n13;
        locals.var_wjunc0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_347(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign92920_e142467, assign92920_e142467_d_n0, assign92920_e142467_d_n2, assign92920_e142467_d_n4, assign92920_e142467_d_n5, assign92920_e142467_d_n6, assign92920_e142467_d_n7, assign92920_e142467_d_n8, assign92920_e142467_d_n9, assign92920_e142467_d_n10, assign92920_e142467_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2153 == 0.0)) {
        let assign92920_e142465: f64 = (p.p334 - locals.var_wjunc0);
        (assign92920_e142465, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign92920_e142467;
        locals.var_t2_dn0 = assign92920_e142467_d_n0;
        locals.var_t2_dn2 = assign92920_e142467_d_n2;
        locals.var_t2_dn4 = assign92920_e142467_d_n4;
        locals.var_t2_dn5 = assign92920_e142467_d_n5;
        locals.var_t2_dn6 = assign92920_e142467_d_n6;
        locals.var_t2_dn7 = assign92920_e142467_d_n7;
        locals.var_t2_dn8 = assign92920_e142467_d_n8;
        locals.var_t2_dn9 = assign92920_e142467_d_n9;
        locals.var_t2_dn10 = assign92920_e142467_d_n10;
        locals.var_t2_dn13 = assign92920_e142467_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign92930_e142491, assign92930_e142491_d_n0, assign92930_e142491_d_n2, assign92930_e142491_d_n4, assign92930_e142491_d_n5, assign92930_e142491_d_n6, assign92930_e142491_d_n7, assign92930_e142491_d_n8, assign92930_e142491_d_n9, assign92930_e142491_d_n10, assign92930_e142491_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign92930_e142478: f64 = (locals.var_t2 * locals.var_t2);
        let assign92930_e142482: f64 = (p.p334 * 0.01);
        let assign92930_e142483: f64 = (4.0 * assign92930_e142482);
        let assign92930_e142486: f64 = (p.p334 * 0.01);
        let assign92930_e142487: f64 = (assign92930_e142483 * assign92930_e142486);
        let assign92930_e142488: f64 = (assign92930_e142478 + assign92930_e142487);
        let assign92930_e142489: f64 = (assign92930_e142488).sqrt();
        (assign92930_e142489, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign92930_e142489)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign92930_e142489)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign92930_e142491;
        locals.var_tmf2_dn0 = assign92930_e142491_d_n0;
        locals.var_tmf2_dn2 = assign92930_e142491_d_n2;
        locals.var_tmf2_dn4 = assign92930_e142491_d_n4;
        locals.var_tmf2_dn5 = assign92930_e142491_d_n5;
        locals.var_tmf2_dn6 = assign92930_e142491_d_n6;
        locals.var_tmf2_dn7 = assign92930_e142491_d_n7;
        locals.var_tmf2_dn8 = assign92930_e142491_d_n8;
        locals.var_tmf2_dn9 = assign92930_e142491_d_n9;
        locals.var_tmf2_dn10 = assign92930_e142491_d_n10;
        locals.var_tmf2_dn13 = assign92930_e142491_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign92940_e142508, assign92940_e142508_d_n0, assign92940_e142508_d_n2, assign92940_e142508_d_n4, assign92940_e142508_d_n5, assign92940_e142508_d_n6, assign92940_e142508_d_n7, assign92940_e142508_d_n8, assign92940_e142508_d_n9, assign92940_e142508_d_n10, assign92940_e142508_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign92940_e142504: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign92940_e142505: f64 = (1.0 + assign92940_e142504);
        let assign92940_e142506: f64 = (0.5 * assign92940_e142505);
        (assign92940_e142506, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign92940_e142508;
        locals.var_t9_dn0 = assign92940_e142508_d_n0;
        locals.var_t9_dn2 = assign92940_e142508_d_n2;
        locals.var_t9_dn4 = assign92940_e142508_d_n4;
        locals.var_t9_dn5 = assign92940_e142508_d_n5;
        locals.var_t9_dn6 = assign92940_e142508_d_n6;
        locals.var_t9_dn7 = assign92940_e142508_d_n7;
        locals.var_t9_dn8 = assign92940_e142508_d_n8;
        locals.var_t9_dn9 = assign92940_e142508_d_n9;
        locals.var_t9_dn10 = assign92940_e142508_d_n10;
        locals.var_t9_dn13 = assign92940_e142508_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign92950_e142523, assign92950_e142523_d_n0, assign92950_e142523_d_n2, assign92950_e142523_d_n4, assign92950_e142523_d_n5, assign92950_e142523_d_n6, assign92950_e142523_d_n7, assign92950_e142523_d_n8, assign92950_e142523_d_n9, assign92950_e142523_d_n10, assign92950_e142523_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign92950_e142520: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign92950_e142521: f64 = (0.5 * assign92950_e142520);
        (assign92950_e142521, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign92950_e142523;
        locals.var_t2_dn0 = assign92950_e142523_d_n0;
        locals.var_t2_dn2 = assign92950_e142523_d_n2;
        locals.var_t2_dn4 = assign92950_e142523_d_n4;
        locals.var_t2_dn5 = assign92950_e142523_d_n5;
        locals.var_t2_dn6 = assign92950_e142523_d_n6;
        locals.var_t2_dn7 = assign92950_e142523_d_n7;
        locals.var_t2_dn8 = assign92950_e142523_d_n8;
        locals.var_t2_dn9 = assign92950_e142523_d_n9;
        locals.var_t2_dn10 = assign92950_e142523_d_n10;
        locals.var_t2_dn13 = assign92950_e142523_d_n13;
        locals.var_t2_rv = 0.0;

        let assign92960_e142526: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2155 = assign92960_e142526;
        locals.var_guard2155_rv = 0.0;

        let (assign92970_e142539, assign92970_e142539_d_n0, assign92970_e142539_d_n2, assign92970_e142539_d_n4, assign92970_e142539_d_n5, assign92970_e142539_d_n6, assign92970_e142539_d_n7, assign92970_e142539_d_n8, assign92970_e142539_d_n9, assign92970_e142539_d_n10, assign92970_e142539_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2155 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign92970_e142539;
        locals.var_t2_dn0 = assign92970_e142539_d_n0;
        locals.var_t2_dn2 = assign92970_e142539_d_n2;
        locals.var_t2_dn4 = assign92970_e142539_d_n4;
        locals.var_t2_dn5 = assign92970_e142539_d_n5;
        locals.var_t2_dn6 = assign92970_e142539_d_n6;
        locals.var_t2_dn7 = assign92970_e142539_d_n7;
        locals.var_t2_dn8 = assign92970_e142539_d_n8;
        locals.var_t2_dn9 = assign92970_e142539_d_n9;
        locals.var_t2_dn10 = assign92970_e142539_d_n10;
        locals.var_t2_dn13 = assign92970_e142539_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign92980_e142552, assign92980_e142552_d_n0, assign92980_e142552_d_n2, assign92980_e142552_d_n4, assign92980_e142552_d_n5, assign92980_e142552_d_n6, assign92980_e142552_d_n7, assign92980_e142552_d_n8, assign92980_e142552_d_n9, assign92980_e142552_d_n10, assign92980_e142552_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2155 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign92980_e142552;
        locals.var_t9_dn0 = assign92980_e142552_d_n0;
        locals.var_t9_dn2 = assign92980_e142552_d_n2;
        locals.var_t9_dn4 = assign92980_e142552_d_n4;
        locals.var_t9_dn5 = assign92980_e142552_d_n5;
        locals.var_t9_dn6 = assign92980_e142552_d_n6;
        locals.var_t9_dn7 = assign92980_e142552_d_n7;
        locals.var_t9_dn8 = assign92980_e142552_d_n8;
        locals.var_t9_dn9 = assign92980_e142552_d_n9;
        locals.var_t9_dn10 = assign92980_e142552_d_n10;
        locals.var_t9_dn13 = assign92980_e142552_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign92990_e142563, assign92990_e142563_d_n0, assign92990_e142563_d_n2, assign92990_e142563_d_n4, assign92990_e142563_d_n5, assign92990_e142563_d_n6, assign92990_e142563_d_n7, assign92990_e142563_d_n8, assign92990_e142563_d_n9, assign92990_e142563_d_n10, assign92990_e142563_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign92990_e142563;
        locals.var_ddriftldc_dn0 = assign92990_e142563_d_n0;
        locals.var_ddriftldc_dn2 = assign92990_e142563_d_n2;
        locals.var_ddriftldc_dn4 = assign92990_e142563_d_n4;
        locals.var_ddriftldc_dn5 = assign92990_e142563_d_n5;
        locals.var_ddriftldc_dn6 = assign92990_e142563_d_n6;
        locals.var_ddriftldc_dn7 = assign92990_e142563_d_n7;
        locals.var_ddriftldc_dn8 = assign92990_e142563_d_n8;
        locals.var_ddriftldc_dn9 = assign92990_e142563_d_n9;
        locals.var_ddriftldc_dn10 = assign92990_e142563_d_n10;
        locals.var_ddriftldc_dn13 = assign92990_e142563_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign93000_e142582, assign93000_e142582_d_n0, assign93000_e142582_d_n2, assign93000_e142582_d_n4, assign93000_e142582_d_n5, assign93000_e142582_d_n6, assign93000_e142582_d_n7, assign93000_e142582_d_n8, assign93000_e142582_d_n9, assign93000_e142582_d_n10, assign93000_e142582_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign93000_e142574: f64 = (locals.var_q_nsubld__blk2115 * locals.var_ddriftldc);
        let assign93000_e142576: f64 = (assign93000_e142574 * locals.var_ddriftldc);
        let assign93000_e142578: f64 = (assign93000_e142576 / 2.0);
        let assign93000_e142580: f64 = (assign93000_e142578 / 1.034943e-10);
        (assign93000_e142580, (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign93000_e142574 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign93000_e142582;
        locals.var_dphi_sb_dn0 = assign93000_e142582_d_n0;
        locals.var_dphi_sb_dn2 = assign93000_e142582_d_n2;
        locals.var_dphi_sb_dn4 = assign93000_e142582_d_n4;
        locals.var_dphi_sb_dn5 = assign93000_e142582_d_n5;
        locals.var_dphi_sb_dn6 = assign93000_e142582_d_n6;
        locals.var_dphi_sb_dn7 = assign93000_e142582_d_n7;
        locals.var_dphi_sb_dn8 = assign93000_e142582_d_n8;
        locals.var_dphi_sb_dn9 = assign93000_e142582_d_n9;
        locals.var_dphi_sb_dn10 = assign93000_e142582_d_n10;
        locals.var_dphi_sb_dn13 = assign93000_e142582_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign93010_e142598, assign93010_e142598_d_n0, assign93010_e142598_d_n2, assign93010_e142598_d_n4, assign93010_e142598_d_n5, assign93010_e142598_d_n6, assign93010_e142598_d_n7, assign93010_e142598_d_n8, assign93010_e142598_d_n9, assign93010_e142598_d_n10, assign93010_e142598_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign93010_e142593: f64 = (2.0 * locals.var_beta);
        let assign93010_e142595: f64 = (assign93010_e142593 * locals.var_dphi_sb);
        let assign93010_e142596: f64 = (assign93010_e142595).sqrt();
        (assign93010_e142596, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn0)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn2)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn4)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn5)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn6)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn7)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn8)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn9)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn10)) / (2.0 * assign93010_e142596)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign93010_e142593 * locals.var_dphi_sb_dn13)) / (2.0 * assign93010_e142596)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign93010_e142598;
        locals.var_t0_dn0 = assign93010_e142598_d_n0;
        locals.var_t0_dn2 = assign93010_e142598_d_n2;
        locals.var_t0_dn4 = assign93010_e142598_d_n4;
        locals.var_t0_dn5 = assign93010_e142598_d_n5;
        locals.var_t0_dn6 = assign93010_e142598_d_n6;
        locals.var_t0_dn7 = assign93010_e142598_d_n7;
        locals.var_t0_dn8 = assign93010_e142598_d_n8;
        locals.var_t0_dn9 = assign93010_e142598_d_n9;
        locals.var_t0_dn10 = assign93010_e142598_d_n10;
        locals.var_t0_dn13 = assign93010_e142598_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign93020_e142616, assign93020_e142616_d_n0, assign93020_e142616_d_n2, assign93020_e142616_d_n4, assign93020_e142616_d_n5, assign93020_e142616_d_n6, assign93020_e142616_d_n7, assign93020_e142616_d_n8, assign93020_e142616_d_n9, assign93020_e142616_d_n10, assign93020_e142616_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign93020_e142608: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign93020_e142610: f64 = (-locals.var_t0);
        let assign93020_e142611: f64 = { let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign93020_e142612: f64 = (assign93020_e142608 + assign93020_e142611);
        let assign93020_e142614: f64 = (assign93020_e142612 / 2.0);
        (assign93020_e142614, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign93020_e142610; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign93020_e142616;
        locals.var_t1_dn0 = assign93020_e142616_d_n0;
        locals.var_t1_dn2 = assign93020_e142616_d_n2;
        locals.var_t1_dn4 = assign93020_e142616_d_n4;
        locals.var_t1_dn5 = assign93020_e142616_d_n5;
        locals.var_t1_dn6 = assign93020_e142616_d_n6;
        locals.var_t1_dn7 = assign93020_e142616_d_n7;
        locals.var_t1_dn8 = assign93020_e142616_d_n8;
        locals.var_t1_dn9 = assign93020_e142616_d_n9;
        locals.var_t1_dn10 = assign93020_e142616_d_n10;
        locals.var_t1_dn13 = assign93020_e142616_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign93030_e142630, assign93030_e142630_d_n0, assign93030_e142630_d_n2, assign93030_e142630_d_n4, assign93030_e142630_d_n5, assign93030_e142630_d_n6, assign93030_e142630_d_n7, assign93030_e142630_d_n8, assign93030_e142630_d_n9, assign93030_e142630_d_n10, assign93030_e142630_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign93030_e142626: f64 = (locals.var_t1).ln();
        let assign93030_e142628: f64 = (assign93030_e142626 / locals.var_dphi_sb);
        (assign93030_e142628, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign93030_e142626 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign93030_e142630;
        locals.var_c_sb_dn0 = assign93030_e142630_d_n0;
        locals.var_c_sb_dn2 = assign93030_e142630_d_n2;
        locals.var_c_sb_dn4 = assign93030_e142630_d_n4;
        locals.var_c_sb_dn5 = assign93030_e142630_d_n5;
        locals.var_c_sb_dn6 = assign93030_e142630_d_n6;
        locals.var_c_sb_dn7 = assign93030_e142630_d_n7;
        locals.var_c_sb_dn8 = assign93030_e142630_d_n8;
        locals.var_c_sb_dn9 = assign93030_e142630_d_n9;
        locals.var_c_sb_dn10 = assign93030_e142630_d_n10;
        locals.var_c_sb_dn13 = assign93030_e142630_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign93040_e142643, assign93040_e142643_d_n0, assign93040_e142643_d_n2, assign93040_e142643_d_n4, assign93040_e142643_d_n5, assign93040_e142643_d_n6, assign93040_e142643_d_n7, assign93040_e142643_d_n8, assign93040_e142643_d_n9, assign93040_e142643_d_n10, assign93040_e142643_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign93040_e142641: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign93040_e142641, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
        locals.var_ps0ld_vxb = assign93040_e142643;
        locals.var_ps0ld_vxb_dn0 = assign93040_e142643_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign93040_e142643_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign93040_e142643_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign93040_e142643_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign93040_e142643_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign93040_e142643_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign93040_e142643_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign93040_e142643_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign93040_e142643_d_n10;
        locals.var_ps0ld_vxb_dn13 = assign93040_e142643_d_n13;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign93050_e142658, assign93050_e142658_d_n0, assign93050_e142658_d_n2, assign93050_e142658_d_n4, assign93050_e142658_d_n5, assign93050_e142658_d_n6, assign93050_e142658_d_n7, assign93050_e142658_d_n8, assign93050_e142658_d_n9, assign93050_e142658_d_n10, assign93050_e142658_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign93050_e142655: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign93050_e142656: f64 = (locals.var_c_sb * assign93050_e142655);
        (assign93050_e142656, ((locals.var_c_sb_dn0 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign93050_e142655) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign93050_e142658;
        locals.var_ty_dn0 = assign93050_e142658_d_n0;
        locals.var_ty_dn2 = assign93050_e142658_d_n2;
        locals.var_ty_dn4 = assign93050_e142658_d_n4;
        locals.var_ty_dn5 = assign93050_e142658_d_n5;
        locals.var_ty_dn6 = assign93050_e142658_d_n6;
        locals.var_ty_dn7 = assign93050_e142658_d_n7;
        locals.var_ty_dn8 = assign93050_e142658_d_n8;
        locals.var_ty_dn9 = assign93050_e142658_d_n9;
        locals.var_ty_dn10 = assign93050_e142658_d_n10;
        locals.var_ty_dn13 = assign93050_e142658_d_n13;
        locals.var_ty_rv = 0.0;

        let assign93060_e142661: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard2156 = assign93060_e142661;
        locals.var_guard2156_rv = 0.0;

        let (assign93070_e142675, assign93070_e142675_d_n0, assign93070_e142675_d_n2, assign93070_e142675_d_n4, assign93070_e142675_d_n5, assign93070_e142675_d_n6, assign93070_e142675_d_n7, assign93070_e142675_d_n8, assign93070_e142675_d_n9, assign93070_e142675_d_n10, assign93070_e142675_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93070_e142673: f64 = (locals.var_ty).exp();
        (assign93070_e142673, (assign93070_e142673 * locals.var_ty_dn0), (assign93070_e142673 * locals.var_ty_dn2), (assign93070_e142673 * locals.var_ty_dn4), (assign93070_e142673 * locals.var_ty_dn5), (assign93070_e142673 * locals.var_ty_dn6), (assign93070_e142673 * locals.var_ty_dn7), (assign93070_e142673 * locals.var_ty_dn8), (assign93070_e142673 * locals.var_ty_dn9), (assign93070_e142673 * locals.var_ty_dn10), (assign93070_e142673 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign93070_e142675;
        locals.var_t1_dn0 = assign93070_e142675_d_n0;
        locals.var_t1_dn2 = assign93070_e142675_d_n2;
        locals.var_t1_dn4 = assign93070_e142675_d_n4;
        locals.var_t1_dn5 = assign93070_e142675_d_n5;
        locals.var_t1_dn6 = assign93070_e142675_d_n6;
        locals.var_t1_dn7 = assign93070_e142675_d_n7;
        locals.var_t1_dn8 = assign93070_e142675_d_n8;
        locals.var_t1_dn9 = assign93070_e142675_d_n9;
        locals.var_t1_dn10 = assign93070_e142675_d_n10;
        locals.var_t1_dn13 = assign93070_e142675_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign93080_e142692, assign93080_e142692_d_n0, assign93080_e142692_d_n2, assign93080_e142692_d_n4, assign93080_e142692_d_n5, assign93080_e142692_d_n6, assign93080_e142692_d_n7, assign93080_e142692_d_n8, assign93080_e142692_d_n9, assign93080_e142692_d_n10, assign93080_e142692_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93080_e142687: f64 = (-locals.var_c_sb);
        let assign93080_e142689: f64 = (assign93080_e142687 * locals.var_dphi_sb);
        let assign93080_e142690: f64 = (assign93080_e142689).exp();
        (assign93080_e142690, (assign93080_e142690 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn0))), (assign93080_e142690 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn2))), (assign93080_e142690 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn4))), (assign93080_e142690 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn5))), (assign93080_e142690 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn6))), (assign93080_e142690 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn7))), (assign93080_e142690 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn8))), (assign93080_e142690 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn9))), (assign93080_e142690 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn10))), (assign93080_e142690 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign93080_e142687 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign93080_e142692;
        locals.var_t0_dn0 = assign93080_e142692_d_n0;
        locals.var_t0_dn2 = assign93080_e142692_d_n2;
        locals.var_t0_dn4 = assign93080_e142692_d_n4;
        locals.var_t0_dn5 = assign93080_e142692_d_n5;
        locals.var_t0_dn6 = assign93080_e142692_d_n6;
        locals.var_t0_dn7 = assign93080_e142692_d_n7;
        locals.var_t0_dn8 = assign93080_e142692_d_n8;
        locals.var_t0_dn9 = assign93080_e142692_d_n9;
        locals.var_t0_dn10 = assign93080_e142692_d_n10;
        locals.var_t0_dn13 = assign93080_e142692_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign93090_e142707, assign93090_e142707_d_n0, assign93090_e142707_d_n2, assign93090_e142707_d_n4, assign93090_e142707_d_n5, assign93090_e142707_d_n6, assign93090_e142707_d_n7, assign93090_e142707_d_n8, assign93090_e142707_d_n9, assign93090_e142707_d_n10, assign93090_e142707_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93090_e142705: f64 = (locals.var_t1 - locals.var_t0);
        (assign93090_e142705, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign93090_e142707;
        locals.var_t2_dn0 = assign93090_e142707_d_n0;
        locals.var_t2_dn2 = assign93090_e142707_d_n2;
        locals.var_t2_dn4 = assign93090_e142707_d_n4;
        locals.var_t2_dn5 = assign93090_e142707_d_n5;
        locals.var_t2_dn6 = assign93090_e142707_d_n6;
        locals.var_t2_dn7 = assign93090_e142707_d_n7;
        locals.var_t2_dn8 = assign93090_e142707_d_n8;
        locals.var_t2_dn9 = assign93090_e142707_d_n9;
        locals.var_t2_dn10 = assign93090_e142707_d_n10;
        locals.var_t2_dn13 = assign93090_e142707_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign93100_e142725, assign93100_e142725_d_n0, assign93100_e142725_d_n2, assign93100_e142725_d_n4, assign93100_e142725_d_n5, assign93100_e142725_d_n6, assign93100_e142725_d_n7, assign93100_e142725_d_n8, assign93100_e142725_d_n9, assign93100_e142725_d_n10, assign93100_e142725_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2156 != 0.0)) {
        let assign93100_e142720: f64 = (1.0 + locals.var_t2);
        let assign93100_e142721: f64 = (assign93100_e142720).ln();
        let assign93100_e142723: f64 = (assign93100_e142721 / locals.var_c_sb);
        (assign93100_e142723, ((((locals.var_t2_dn0 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign93100_e142720) * locals.var_c_sb) - (assign93100_e142721 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign93100_e142725;
        locals.var_phi_b_dn0 = assign93100_e142725_d_n0;
        locals.var_phi_b_dn2 = assign93100_e142725_d_n2;
        locals.var_phi_b_dn4 = assign93100_e142725_d_n4;
        locals.var_phi_b_dn5 = assign93100_e142725_d_n5;
        locals.var_phi_b_dn6 = assign93100_e142725_d_n6;
        locals.var_phi_b_dn7 = assign93100_e142725_d_n7;
        locals.var_phi_b_dn8 = assign93100_e142725_d_n8;
        locals.var_phi_b_dn9 = assign93100_e142725_d_n9;
        locals.var_phi_b_dn10 = assign93100_e142725_d_n10;
        locals.var_phi_b_dn13 = assign93100_e142725_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign93110_e142741, assign93110_e142741_d_n0, assign93110_e142741_d_n2, assign93110_e142741_d_n4, assign93110_e142741_d_n5, assign93110_e142741_d_n6, assign93110_e142741_d_n7, assign93110_e142741_d_n8, assign93110_e142741_d_n9, assign93110_e142741_d_n10, assign93110_e142741_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2156 == 0.0)) {
        let assign93110_e142739: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign93110_e142739, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign93110_e142741;
        locals.var_phi_b_dn0 = assign93110_e142741_d_n0;
        locals.var_phi_b_dn2 = assign93110_e142741_d_n2;
        locals.var_phi_b_dn4 = assign93110_e142741_d_n4;
        locals.var_phi_b_dn5 = assign93110_e142741_d_n5;
        locals.var_phi_b_dn6 = assign93110_e142741_d_n6;
        locals.var_phi_b_dn7 = assign93110_e142741_d_n7;
        locals.var_phi_b_dn8 = assign93110_e142741_d_n8;
        locals.var_phi_b_dn9 = assign93110_e142741_d_n9;
        locals.var_phi_b_dn10 = assign93110_e142741_d_n10;
        locals.var_phi_b_dn13 = assign93110_e142741_d_n13;
        locals.var_phi_b_rv = 0.0;

        let (assign93120_e142754, assign93120_e142754_d_n0, assign93120_e142754_d_n2, assign93120_e142754_d_n4, assign93120_e142754_d_n5, assign93120_e142754_d_n6, assign93120_e142754_d_n7, assign93120_e142754_d_n8, assign93120_e142754_d_n9, assign93120_e142754_d_n10, assign93120_e142754_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) {
        let assign93120_e142752: f64 = (locals.var_beta * locals.var_phi_b);
        (assign93120_e142752, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
        locals.var_chib = assign93120_e142754;
        locals.var_chib_dn0 = assign93120_e142754_d_n0;
        locals.var_chib_dn2 = assign93120_e142754_d_n2;
        locals.var_chib_dn4 = assign93120_e142754_d_n4;
        locals.var_chib_dn5 = assign93120_e142754_d_n5;
        locals.var_chib_dn6 = assign93120_e142754_d_n6;
        locals.var_chib_dn7 = assign93120_e142754_d_n7;
        locals.var_chib_dn8 = assign93120_e142754_d_n8;
        locals.var_chib_dn9 = assign93120_e142754_d_n9;
        locals.var_chib_dn10 = assign93120_e142754_d_n10;
        locals.var_chib_dn13 = assign93120_e142754_d_n13;
        locals.var_chib_rv = 0.0;

        let assign93130_e142758: f64 = (locals.var_chi / 100.0);
        let assign93130_e142763: f64 = if ((locals.var_chib > assign93130_e142758) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2157 = assign93130_e142763;
        locals.var_guard2157_rv = 0.0;

        let (assign93140_e142778,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2157 != 0.0)) {
        let assign93140_e142776: f64 = (locals.var_flg_fd_mode__blk2121 + 1.0);
        (assign93140_e142776,)
    } else {
        (locals.var_flg_fd_mode__blk2121,)
    }
};
        locals.var_flg_fd_mode__blk2121 = assign93140_e142778;
        locals.var_flg_fd_mode__blk2121_rv = 0.0;

        let (assign93150_e142791, assign93150_e142791_d_n0, assign93150_e142791_d_n2, assign93150_e142791_d_n4, assign93150_e142791_d_n5, assign93150_e142791_d_n6, assign93150_e142791_d_n7, assign93150_e142791_d_n8, assign93150_e142791_d_n9, assign93150_e142791_d_n10, assign93150_e142791_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2152 != 0.0)) && (locals.var_guard2157 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign93150_e142791;
        locals.var_chi_dn0 = assign93150_e142791_d_n0;
        locals.var_chi_dn2 = assign93150_e142791_d_n2;
        locals.var_chi_dn4 = assign93150_e142791_d_n4;
        locals.var_chi_dn5 = assign93150_e142791_d_n5;
        locals.var_chi_dn6 = assign93150_e142791_d_n6;
        locals.var_chi_dn7 = assign93150_e142791_d_n7;
        locals.var_chi_dn8 = assign93150_e142791_d_n8;
        locals.var_chi_dn9 = assign93150_e142791_d_n9;
        locals.var_chi_dn10 = assign93150_e142791_d_n10;
        locals.var_chi_dn13 = assign93150_e142791_d_n13;
        locals.var_chi_rv = 0.0;

        let (assign93160_e142804, assign93160_e142804_d_n0, assign93160_e142804_d_n2, assign93160_e142804_d_n4, assign93160_e142804_d_n5, assign93160_e142804_d_n6, assign93160_e142804_d_n7, assign93160_e142804_d_n8, assign93160_e142804_d_n9, assign93160_e142804_d_n10, assign93160_e142804_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) {
        let assign93160_e142800: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign93160_e142802: f64 = (assign93160_e142800 - locals.var_vxbgmtcl);
        (assign93160_e142802, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign93160_e142804;
        locals.var_ps0ld_dn0 = assign93160_e142804_d_n0;
        locals.var_ps0ld_dn2 = assign93160_e142804_d_n2;
        locals.var_ps0ld_dn4 = assign93160_e142804_d_n4;
        locals.var_ps0ld_dn5 = assign93160_e142804_d_n5;
        locals.var_ps0ld_dn6 = assign93160_e142804_d_n6;
        locals.var_ps0ld_dn7 = assign93160_e142804_d_n7;
        locals.var_ps0ld_dn8 = assign93160_e142804_d_n8;
        locals.var_ps0ld_dn9 = assign93160_e142804_d_n9;
        locals.var_ps0ld_dn10 = assign93160_e142804_d_n10;
        locals.var_ps0ld_dn13 = assign93160_e142804_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign93170_e142806: f64 = (locals.var_chi).abs();
        let assign93170_e142808: f64 = if assign93170_e142806 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard2158 = assign93170_e142808;
        locals.var_guard2158_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_348(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign93180_e142825, assign93180_e142825_d_n0, assign93180_e142825_d_n2, assign93180_e142825_d_n4, assign93180_e142825_d_n5, assign93180_e142825_d_n6, assign93180_e142825_d_n7, assign93180_e142825_d_n8, assign93180_e142825_d_n9, assign93180_e142825_d_n10, assign93180_e142825_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2158 != 0.0)) {
        let assign93180_e142819: f64 = (locals.var_chi - 1.0);
        let assign93180_e142821: f64 = (-locals.var_chi);
        let assign93180_e142822: f64 = (assign93180_e142821).exp();
        let assign93180_e142823: f64 = (assign93180_e142819 + assign93180_e142822);
        (assign93180_e142823, (locals.var_chi_dn0 + (assign93180_e142822 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign93180_e142822 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign93180_e142822 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign93180_e142822 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign93180_e142822 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign93180_e142822 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign93180_e142822 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign93180_e142822 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign93180_e142822 * (-locals.var_chi_dn10))), (locals.var_chi_dn13 + (assign93180_e142822 * (-locals.var_chi_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign93180_e142825;
        locals.var_t1_dn0 = assign93180_e142825_d_n0;
        locals.var_t1_dn2 = assign93180_e142825_d_n2;
        locals.var_t1_dn4 = assign93180_e142825_d_n4;
        locals.var_t1_dn5 = assign93180_e142825_d_n5;
        locals.var_t1_dn6 = assign93180_e142825_d_n6;
        locals.var_t1_dn7 = assign93180_e142825_d_n7;
        locals.var_t1_dn8 = assign93180_e142825_d_n8;
        locals.var_t1_dn9 = assign93180_e142825_d_n9;
        locals.var_t1_dn10 = assign93180_e142825_d_n10;
        locals.var_t1_dn13 = assign93180_e142825_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign93190_e142837, assign93190_e142837_d_n0, assign93190_e142837_d_n2, assign93190_e142837_d_n4, assign93190_e142837_d_n5, assign93190_e142837_d_n6, assign93190_e142837_d_n7, assign93190_e142837_d_n8, assign93190_e142837_d_n9, assign93190_e142837_d_n10, assign93190_e142837_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2158 != 0.0)) {
        let assign93190_e142835: f64 = (locals.var_t1).sqrt();
        (assign93190_e142835, (locals.var_t1_dn0 / (2.0 * assign93190_e142835)), (locals.var_t1_dn2 / (2.0 * assign93190_e142835)), (locals.var_t1_dn4 / (2.0 * assign93190_e142835)), (locals.var_t1_dn5 / (2.0 * assign93190_e142835)), (locals.var_t1_dn6 / (2.0 * assign93190_e142835)), (locals.var_t1_dn7 / (2.0 * assign93190_e142835)), (locals.var_t1_dn8 / (2.0 * assign93190_e142835)), (locals.var_t1_dn9 / (2.0 * assign93190_e142835)), (locals.var_t1_dn10 / (2.0 * assign93190_e142835)), (locals.var_t1_dn13 / (2.0 * assign93190_e142835)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign93190_e142837;
        locals.var_t2_dn0 = assign93190_e142837_d_n0;
        locals.var_t2_dn2 = assign93190_e142837_d_n2;
        locals.var_t2_dn4 = assign93190_e142837_d_n4;
        locals.var_t2_dn5 = assign93190_e142837_d_n5;
        locals.var_t2_dn6 = assign93190_e142837_d_n6;
        locals.var_t2_dn7 = assign93190_e142837_d_n7;
        locals.var_t2_dn8 = assign93190_e142837_d_n8;
        locals.var_t2_dn9 = assign93190_e142837_d_n9;
        locals.var_t2_dn10 = assign93190_e142837_d_n10;
        locals.var_t2_dn13 = assign93190_e142837_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign93210_e142872, assign93210_e142872_d_n0, assign93210_e142872_d_n2, assign93210_e142872_d_n4, assign93210_e142872_d_n5, assign93210_e142872_d_n6, assign93210_e142872_d_n7, assign93210_e142872_d_n8, assign93210_e142872_d_n9, assign93210_e142872_d_n10, assign93210_e142872_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2158 == 0.0)) {
        let assign93210_e142863: f64 = (0.7071067811865475 * locals.var_chi);
        let assign93210_e142867: f64 = (locals.var_chi * 0.3333333333333333);
        let assign93210_e142868: f64 = (1.0 - assign93210_e142867);
        let assign93210_e142869: f64 = (assign93210_e142868).sqrt();
        let assign93210_e142870: f64 = (assign93210_e142863 * assign93210_e142869);
        (assign93210_e142870, (((0.7071067811865475 * locals.var_chi_dn0) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))), (((0.7071067811865475 * locals.var_chi_dn13) * assign93210_e142869) + (assign93210_e142863 * ((-(locals.var_chi_dn13 * 0.3333333333333333)) / (2.0 * assign93210_e142869)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign93210_e142872;
        locals.var_t2_dn0 = assign93210_e142872_d_n0;
        locals.var_t2_dn2 = assign93210_e142872_d_n2;
        locals.var_t2_dn4 = assign93210_e142872_d_n4;
        locals.var_t2_dn5 = assign93210_e142872_d_n5;
        locals.var_t2_dn6 = assign93210_e142872_d_n6;
        locals.var_t2_dn7 = assign93210_e142872_d_n7;
        locals.var_t2_dn8 = assign93210_e142872_d_n8;
        locals.var_t2_dn9 = assign93210_e142872_d_n9;
        locals.var_t2_dn10 = assign93210_e142872_d_n10;
        locals.var_t2_dn13 = assign93210_e142872_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign93220_e142883, assign93220_e142883_d_n0, assign93220_e142883_d_n2, assign93220_e142883_d_n4, assign93220_e142883_d_n5, assign93220_e142883_d_n6, assign93220_e142883_d_n7, assign93220_e142883_d_n8, assign93220_e142883_d_n9, assign93220_e142883_d_n10, assign93220_e142883_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) {
        let assign93220_e142881: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign93220_e142881, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign93220_e142883;
        locals.var_qbuld_dn0 = assign93220_e142883_d_n0;
        locals.var_qbuld_dn2 = assign93220_e142883_d_n2;
        locals.var_qbuld_dn4 = assign93220_e142883_d_n4;
        locals.var_qbuld_dn5 = assign93220_e142883_d_n5;
        locals.var_qbuld_dn6 = assign93220_e142883_d_n6;
        locals.var_qbuld_dn7 = assign93220_e142883_d_n7;
        locals.var_qbuld_dn8 = assign93220_e142883_d_n8;
        locals.var_qbuld_dn9 = assign93220_e142883_d_n9;
        locals.var_qbuld_dn10 = assign93220_e142883_d_n10;
        locals.var_qbuld_dn13 = assign93220_e142883_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign93230_e142896, assign93230_e142896_d_n0, assign93230_e142896_d_n2, assign93230_e142896_d_n4, assign93230_e142896_d_n5, assign93230_e142896_d_n6, assign93230_e142896_d_n7, assign93230_e142896_d_n8, assign93230_e142896_d_n9, assign93230_e142896_d_n10, assign93230_e142896_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) {
        let assign93230_e142893: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign93230_e142894: f64 = (locals.var_cox0_func * assign93230_e142893);
        (assign93230_e142894, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (-locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn13)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign93230_e142896;
        locals.var_qsuld_dn0 = assign93230_e142896_d_n0;
        locals.var_qsuld_dn2 = assign93230_e142896_d_n2;
        locals.var_qsuld_dn4 = assign93230_e142896_d_n4;
        locals.var_qsuld_dn5 = assign93230_e142896_d_n5;
        locals.var_qsuld_dn6 = assign93230_e142896_d_n6;
        locals.var_qsuld_dn7 = assign93230_e142896_d_n7;
        locals.var_qsuld_dn8 = assign93230_e142896_d_n8;
        locals.var_qsuld_dn9 = assign93230_e142896_d_n9;
        locals.var_qsuld_dn10 = assign93230_e142896_d_n10;
        locals.var_qsuld_dn13 = assign93230_e142896_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign93240_e142907, assign93240_e142907_d_n0, assign93240_e142907_d_n2, assign93240_e142907_d_n4, assign93240_e142907_d_n5, assign93240_e142907_d_n6, assign93240_e142907_d_n7, assign93240_e142907_d_n8, assign93240_e142907_d_n9, assign93240_e142907_d_n10, assign93240_e142907_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) {
        let assign93240_e142905: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk2115);
        (assign93240_e142905, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk2115), (locals.var_qbuld_dn13 / locals.var_q_nsubld__blk2115),)
    } else {
        (locals.var_wdld0__blk2159, locals.var_wdld0__blk2159_dn0, locals.var_wdld0__blk2159_dn2, locals.var_wdld0__blk2159_dn4, locals.var_wdld0__blk2159_dn5, locals.var_wdld0__blk2159_dn6, locals.var_wdld0__blk2159_dn7, locals.var_wdld0__blk2159_dn8, locals.var_wdld0__blk2159_dn9, locals.var_wdld0__blk2159_dn10, locals.var_wdld0__blk2159_dn13,)
    }
};
        locals.var_wdld0__blk2159 = assign93240_e142907;
        locals.var_wdld0__blk2159_dn0 = assign93240_e142907_d_n0;
        locals.var_wdld0__blk2159_dn2 = assign93240_e142907_d_n2;
        locals.var_wdld0__blk2159_dn4 = assign93240_e142907_d_n4;
        locals.var_wdld0__blk2159_dn5 = assign93240_e142907_d_n5;
        locals.var_wdld0__blk2159_dn6 = assign93240_e142907_d_n6;
        locals.var_wdld0__blk2159_dn7 = assign93240_e142907_d_n7;
        locals.var_wdld0__blk2159_dn8 = assign93240_e142907_d_n8;
        locals.var_wdld0__blk2159_dn9 = assign93240_e142907_d_n9;
        locals.var_wdld0__blk2159_dn10 = assign93240_e142907_d_n10;
        locals.var_wdld0__blk2159_dn13 = assign93240_e142907_d_n13;
        locals.var_wdld0__blk2159_rv = 0.0;

        let assign93250_e142910: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2161 = assign93250_e142910;
        locals.var_guard2161_rv = 0.0;

        let assign93260_e142915: f64 = (locals.var_ddriftldc * 0.1);
        let assign93260_e142916: f64 = (locals.var_ddriftldc - assign93260_e142915);
        let assign93260_e142920: f64 = (locals.var_ddriftldc * 0.1);
        let assign93260_e142923: f64 = if ((locals.var_wdld0__blk2159 > assign93260_e142916) && (assign93260_e142920 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2162 = assign93260_e142923;
        locals.var_guard2162_rv = 0.0;

        let (assign93270_e142942, assign93270_e142942_d_n0, assign93270_e142942_d_n2, assign93270_e142942_d_n4, assign93270_e142942_d_n5, assign93270_e142942_d_n6, assign93270_e142942_d_n7, assign93270_e142942_d_n8, assign93270_e142942_d_n9, assign93270_e142942_d_n10, assign93270_e142942_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93270_e142936: f64 = (locals.var_wdld0__blk2159 - locals.var_ddriftldc);
        let assign93270_e142939: f64 = (locals.var_ddriftldc * 0.1);
        let assign93270_e142940: f64 = (assign93270_e142936 + assign93270_e142939);
        (assign93270_e142940, ((locals.var_wdld0__blk2159_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk2159_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk2159_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk2159_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk2159_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk2159_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk2159_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk2159_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk2159_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk2159_dn13 - locals.var_ddriftldc_dn13) + (locals.var_ddriftldc_dn13 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign93270_e142942;
        locals.var_tmf1_dn0 = assign93270_e142942_d_n0;
        locals.var_tmf1_dn2 = assign93270_e142942_d_n2;
        locals.var_tmf1_dn4 = assign93270_e142942_d_n4;
        locals.var_tmf1_dn5 = assign93270_e142942_d_n5;
        locals.var_tmf1_dn6 = assign93270_e142942_d_n6;
        locals.var_tmf1_dn7 = assign93270_e142942_d_n7;
        locals.var_tmf1_dn8 = assign93270_e142942_d_n8;
        locals.var_tmf1_dn9 = assign93270_e142942_d_n9;
        locals.var_tmf1_dn10 = assign93270_e142942_d_n10;
        locals.var_tmf1_dn13 = assign93270_e142942_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign93280_e142957, assign93280_e142957_d_n0, assign93280_e142957_d_n2, assign93280_e142957_d_n4, assign93280_e142957_d_n5, assign93280_e142957_d_n6, assign93280_e142957_d_n7, assign93280_e142957_d_n8, assign93280_e142957_d_n9, assign93280_e142957_d_n10, assign93280_e142957_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93280_e142955: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign93280_e142955, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign93280_e142957;
        locals.var_x2_dn0 = assign93280_e142957_d_n0;
        locals.var_x2_dn2 = assign93280_e142957_d_n2;
        locals.var_x2_dn4 = assign93280_e142957_d_n4;
        locals.var_x2_dn5 = assign93280_e142957_d_n5;
        locals.var_x2_dn6 = assign93280_e142957_d_n6;
        locals.var_x2_dn7 = assign93280_e142957_d_n7;
        locals.var_x2_dn8 = assign93280_e142957_d_n8;
        locals.var_x2_dn9 = assign93280_e142957_d_n9;
        locals.var_x2_dn10 = assign93280_e142957_d_n10;
        locals.var_x2_dn13 = assign93280_e142957_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign93290_e142976, assign93290_e142976_d_n0, assign93290_e142976_d_n2, assign93290_e142976_d_n4, assign93290_e142976_d_n5, assign93290_e142976_d_n6, assign93290_e142976_d_n7, assign93290_e142976_d_n8, assign93290_e142976_d_n9, assign93290_e142976_d_n10, assign93290_e142976_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93290_e142970: f64 = (locals.var_ddriftldc * 0.1);
        let assign93290_e142973: f64 = (locals.var_ddriftldc * 0.1);
        let assign93290_e142974: f64 = (assign93290_e142970 * assign93290_e142973);
        (assign93290_e142974, (((locals.var_ddriftldc_dn0 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn13 * 0.1) * assign93290_e142973) + (assign93290_e142970 * (locals.var_ddriftldc_dn13 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign93290_e142976;
        locals.var_xmax2_dn0 = assign93290_e142976_d_n0;
        locals.var_xmax2_dn2 = assign93290_e142976_d_n2;
        locals.var_xmax2_dn4 = assign93290_e142976_d_n4;
        locals.var_xmax2_dn5 = assign93290_e142976_d_n5;
        locals.var_xmax2_dn6 = assign93290_e142976_d_n6;
        locals.var_xmax2_dn7 = assign93290_e142976_d_n7;
        locals.var_xmax2_dn8 = assign93290_e142976_d_n8;
        locals.var_xmax2_dn9 = assign93290_e142976_d_n9;
        locals.var_xmax2_dn10 = assign93290_e142976_d_n10;
        locals.var_xmax2_dn13 = assign93290_e142976_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign93300_e142989, assign93300_e142989_d_n0, assign93300_e142989_d_n2, assign93300_e142989_d_n4, assign93300_e142989_d_n5, assign93300_e142989_d_n6, assign93300_e142989_d_n7, assign93300_e142989_d_n8, assign93300_e142989_d_n9, assign93300_e142989_d_n10, assign93300_e142989_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign93300_e142989;
        locals.var_xp_dn0 = assign93300_e142989_d_n0;
        locals.var_xp_dn2 = assign93300_e142989_d_n2;
        locals.var_xp_dn4 = assign93300_e142989_d_n4;
        locals.var_xp_dn5 = assign93300_e142989_d_n5;
        locals.var_xp_dn6 = assign93300_e142989_d_n6;
        locals.var_xp_dn7 = assign93300_e142989_d_n7;
        locals.var_xp_dn8 = assign93300_e142989_d_n8;
        locals.var_xp_dn9 = assign93300_e142989_d_n9;
        locals.var_xp_dn10 = assign93300_e142989_d_n10;
        locals.var_xp_dn13 = assign93300_e142989_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign93310_e143002, assign93310_e143002_d_n0, assign93310_e143002_d_n2, assign93310_e143002_d_n4, assign93310_e143002_d_n5, assign93310_e143002_d_n6, assign93310_e143002_d_n7, assign93310_e143002_d_n8, assign93310_e143002_d_n9, assign93310_e143002_d_n10, assign93310_e143002_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign93310_e143002;
        locals.var_xmp_dn0 = assign93310_e143002_d_n0;
        locals.var_xmp_dn2 = assign93310_e143002_d_n2;
        locals.var_xmp_dn4 = assign93310_e143002_d_n4;
        locals.var_xmp_dn5 = assign93310_e143002_d_n5;
        locals.var_xmp_dn6 = assign93310_e143002_d_n6;
        locals.var_xmp_dn7 = assign93310_e143002_d_n7;
        locals.var_xmp_dn8 = assign93310_e143002_d_n8;
        locals.var_xmp_dn9 = assign93310_e143002_d_n9;
        locals.var_xmp_dn10 = assign93310_e143002_d_n10;
        locals.var_xmp_dn13 = assign93310_e143002_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign93320_e143015,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign93320_e143015;
        locals.var_m0_rv = 0.0;

        let (assign93330_e143028,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93330_e143028;
        locals.var_mm_rv = 0.0;

        let (assign93340_e143041, assign93340_e143041_d_n0, assign93340_e143041_d_n2, assign93340_e143041_d_n4, assign93340_e143041_d_n5, assign93340_e143041_d_n6, assign93340_e143041_d_n7, assign93340_e143041_d_n8, assign93340_e143041_d_n9, assign93340_e143041_d_n10, assign93340_e143041_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign93340_e143041;
        locals.var_arg_dn0 = assign93340_e143041_d_n0;
        locals.var_arg_dn2 = assign93340_e143041_d_n2;
        locals.var_arg_dn4 = assign93340_e143041_d_n4;
        locals.var_arg_dn5 = assign93340_e143041_d_n5;
        locals.var_arg_dn6 = assign93340_e143041_d_n6;
        locals.var_arg_dn7 = assign93340_e143041_d_n7;
        locals.var_arg_dn8 = assign93340_e143041_d_n8;
        locals.var_arg_dn9 = assign93340_e143041_d_n9;
        locals.var_arg_dn10 = assign93340_e143041_d_n10;
        locals.var_arg_dn13 = assign93340_e143041_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign93350_e143054, assign93350_e143054_d_n0, assign93350_e143054_d_n2, assign93350_e143054_d_n4, assign93350_e143054_d_n5, assign93350_e143054_d_n6, assign93350_e143054_d_n7, assign93350_e143054_d_n8, assign93350_e143054_d_n9, assign93350_e143054_d_n10, assign93350_e143054_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign93350_e143054;
        locals.var_dnm_dn0 = assign93350_e143054_d_n0;
        locals.var_dnm_dn2 = assign93350_e143054_d_n2;
        locals.var_dnm_dn4 = assign93350_e143054_d_n4;
        locals.var_dnm_dn5 = assign93350_e143054_d_n5;
        locals.var_dnm_dn6 = assign93350_e143054_d_n6;
        locals.var_dnm_dn7 = assign93350_e143054_d_n7;
        locals.var_dnm_dn8 = assign93350_e143054_d_n8;
        locals.var_dnm_dn9 = assign93350_e143054_d_n9;
        locals.var_dnm_dn10 = assign93350_e143054_d_n10;
        locals.var_dnm_dn13 = assign93350_e143054_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign93360_e143069, assign93360_e143069_d_n0, assign93360_e143069_d_n2, assign93360_e143069_d_n4, assign93360_e143069_d_n5, assign93360_e143069_d_n6, assign93360_e143069_d_n7, assign93360_e143069_d_n8, assign93360_e143069_d_n9, assign93360_e143069_d_n10, assign93360_e143069_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93360_e143067: f64 = (locals.var_xp * locals.var_x2);
        (assign93360_e143067, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign93360_e143069;
        locals.var_xp_dn0 = assign93360_e143069_d_n0;
        locals.var_xp_dn2 = assign93360_e143069_d_n2;
        locals.var_xp_dn4 = assign93360_e143069_d_n4;
        locals.var_xp_dn5 = assign93360_e143069_d_n5;
        locals.var_xp_dn6 = assign93360_e143069_d_n6;
        locals.var_xp_dn7 = assign93360_e143069_d_n7;
        locals.var_xp_dn8 = assign93360_e143069_d_n8;
        locals.var_xp_dn9 = assign93360_e143069_d_n9;
        locals.var_xp_dn10 = assign93360_e143069_d_n10;
        locals.var_xp_dn13 = assign93360_e143069_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign93370_e143084, assign93370_e143084_d_n0, assign93370_e143084_d_n2, assign93370_e143084_d_n4, assign93370_e143084_d_n5, assign93370_e143084_d_n6, assign93370_e143084_d_n7, assign93370_e143084_d_n8, assign93370_e143084_d_n9, assign93370_e143084_d_n10, assign93370_e143084_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93370_e143082: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign93370_e143082, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign93370_e143084;
        locals.var_xmp_dn0 = assign93370_e143084_d_n0;
        locals.var_xmp_dn2 = assign93370_e143084_d_n2;
        locals.var_xmp_dn4 = assign93370_e143084_d_n4;
        locals.var_xmp_dn5 = assign93370_e143084_d_n5;
        locals.var_xmp_dn6 = assign93370_e143084_d_n6;
        locals.var_xmp_dn7 = assign93370_e143084_d_n7;
        locals.var_xmp_dn8 = assign93370_e143084_d_n8;
        locals.var_xmp_dn9 = assign93370_e143084_d_n9;
        locals.var_xmp_dn10 = assign93370_e143084_d_n10;
        locals.var_xmp_dn13 = assign93370_e143084_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign93380_e143099, assign93380_e143099_d_n0, assign93380_e143099_d_n2, assign93380_e143099_d_n4, assign93380_e143099_d_n5, assign93380_e143099_d_n6, assign93380_e143099_d_n7, assign93380_e143099_d_n8, assign93380_e143099_d_n9, assign93380_e143099_d_n10, assign93380_e143099_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93380_e143097: f64 = (locals.var_xp * locals.var_x2);
        (assign93380_e143097, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign93380_e143099;
        locals.var_xp_dn0 = assign93380_e143099_d_n0;
        locals.var_xp_dn2 = assign93380_e143099_d_n2;
        locals.var_xp_dn4 = assign93380_e143099_d_n4;
        locals.var_xp_dn5 = assign93380_e143099_d_n5;
        locals.var_xp_dn6 = assign93380_e143099_d_n6;
        locals.var_xp_dn7 = assign93380_e143099_d_n7;
        locals.var_xp_dn8 = assign93380_e143099_d_n8;
        locals.var_xp_dn9 = assign93380_e143099_d_n9;
        locals.var_xp_dn10 = assign93380_e143099_d_n10;
        locals.var_xp_dn13 = assign93380_e143099_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign93390_e143114, assign93390_e143114_d_n0, assign93390_e143114_d_n2, assign93390_e143114_d_n4, assign93390_e143114_d_n5, assign93390_e143114_d_n6, assign93390_e143114_d_n7, assign93390_e143114_d_n8, assign93390_e143114_d_n9, assign93390_e143114_d_n10, assign93390_e143114_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93390_e143112: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign93390_e143112, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign93390_e143114;
        locals.var_xmp_dn0 = assign93390_e143114_d_n0;
        locals.var_xmp_dn2 = assign93390_e143114_d_n2;
        locals.var_xmp_dn4 = assign93390_e143114_d_n4;
        locals.var_xmp_dn5 = assign93390_e143114_d_n5;
        locals.var_xmp_dn6 = assign93390_e143114_d_n6;
        locals.var_xmp_dn7 = assign93390_e143114_d_n7;
        locals.var_xmp_dn8 = assign93390_e143114_d_n8;
        locals.var_xmp_dn9 = assign93390_e143114_d_n9;
        locals.var_xmp_dn10 = assign93390_e143114_d_n10;
        locals.var_xmp_dn13 = assign93390_e143114_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign93400_e143129, assign93400_e143129_d_n0, assign93400_e143129_d_n2, assign93400_e143129_d_n4, assign93400_e143129_d_n5, assign93400_e143129_d_n6, assign93400_e143129_d_n7, assign93400_e143129_d_n8, assign93400_e143129_d_n9, assign93400_e143129_d_n10, assign93400_e143129_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93400_e143127: f64 = (locals.var_xp + locals.var_xmp);
        (assign93400_e143127, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign93400_e143129;
        locals.var_arg_dn0 = assign93400_e143129_d_n0;
        locals.var_arg_dn2 = assign93400_e143129_d_n2;
        locals.var_arg_dn4 = assign93400_e143129_d_n4;
        locals.var_arg_dn5 = assign93400_e143129_d_n5;
        locals.var_arg_dn6 = assign93400_e143129_d_n6;
        locals.var_arg_dn7 = assign93400_e143129_d_n7;
        locals.var_arg_dn8 = assign93400_e143129_d_n8;
        locals.var_arg_dn9 = assign93400_e143129_d_n9;
        locals.var_arg_dn10 = assign93400_e143129_d_n10;
        locals.var_arg_dn13 = assign93400_e143129_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign93410_e143142, assign93410_e143142_d_n0, assign93410_e143142_d_n2, assign93410_e143142_d_n4, assign93410_e143142_d_n5, assign93410_e143142_d_n6, assign93410_e143142_d_n7, assign93410_e143142_d_n8, assign93410_e143142_d_n9, assign93410_e143142_d_n10, assign93410_e143142_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign93410_e143142;
        locals.var_dnm_dn0 = assign93410_e143142_d_n0;
        locals.var_dnm_dn2 = assign93410_e143142_d_n2;
        locals.var_dnm_dn4 = assign93410_e143142_d_n4;
        locals.var_dnm_dn5 = assign93410_e143142_d_n5;
        locals.var_dnm_dn6 = assign93410_e143142_d_n6;
        locals.var_dnm_dn7 = assign93410_e143142_d_n7;
        locals.var_dnm_dn8 = assign93410_e143142_d_n8;
        locals.var_dnm_dn9 = assign93410_e143142_d_n9;
        locals.var_dnm_dn10 = assign93410_e143142_d_n10;
        locals.var_dnm_dn13 = assign93410_e143142_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign93420_e143157: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2163 = assign93420_e143157;
        locals.var_guard2163_rv = 0.0;

        let assign93430_e143160: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2164 = assign93430_e143160;
        locals.var_guard2164_rv = 0.0;

        let (assign93440_e143177,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93440_e143177;
        locals.var_mm_rv = 0.0;

        let assign93450_e143180: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2165 = assign93450_e143180;
        locals.var_guard2165_rv = 0.0;

        let (assign93460_e143200,) = {
    if ((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 == 0.0)) && (locals.var_guard2165 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93460_e143200;
        locals.var_mm_rv = 0.0;

        let assign93470_e143203: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2166 = assign93470_e143203;
        locals.var_guard2166_rv = 0.0;

        let (assign93480_e143226,) = {
    if (((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 == 0.0)) && (locals.var_guard2165 == 0.0)) && (locals.var_guard2166 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93480_e143226;
        locals.var_mm_rv = 0.0;

        let assign93490_e143229: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2167 = assign93490_e143229;
        locals.var_guard2167_rv = 0.0;

        let (assign93500_e143255,) = {
    if ((((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_guard2164 == 0.0)) && (locals.var_guard2165 == 0.0)) && (locals.var_guard2166 == 0.0)) && (locals.var_guard2167 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93500_e143255;
        locals.var_mm_rv = 0.0;

        let (assign93510_e143270,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign93510_e143270;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_349(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign93520_loop_guard: usize = 0;
        while {
            let assign93520_cond_e143286: f64 = if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign93520_cond_e143286 != 0.0
        } {
            assign93520_loop_guard += 1;
            assert!(assign93520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign93520_body0_e143302, assign93520_body0_e143302_d_n0, assign93520_body0_e143302_d_n2, assign93520_body0_e143302_d_n4, assign93520_body0_e143302_d_n5, assign93520_body0_e143302_d_n6, assign93520_body0_e143302_d_n7, assign93520_body0_e143302_d_n8, assign93520_body0_e143302_d_n9, assign93520_body0_e143302_d_n10, assign93520_body0_e143302_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 != 0.0)) {
        let assign93520_body0_e143300: f64 = (locals.var_dnm).sqrt();
        (assign93520_body0_e143300, (locals.var_dnm_dn0 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn2 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn4 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn5 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn6 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn7 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn8 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn9 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn10 / (2.0 * assign93520_body0_e143300)), (locals.var_dnm_dn13 / (2.0 * assign93520_body0_e143300)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign93520_body0_e143302;
            locals.var_dnm_dn0 = assign93520_body0_e143302_d_n0;
            locals.var_dnm_dn2 = assign93520_body0_e143302_d_n2;
            locals.var_dnm_dn4 = assign93520_body0_e143302_d_n4;
            locals.var_dnm_dn5 = assign93520_body0_e143302_d_n5;
            locals.var_dnm_dn6 = assign93520_body0_e143302_d_n6;
            locals.var_dnm_dn7 = assign93520_body0_e143302_d_n7;
            locals.var_dnm_dn8 = assign93520_body0_e143302_d_n8;
            locals.var_dnm_dn9 = assign93520_body0_e143302_d_n9;
            locals.var_dnm_dn10 = assign93520_body0_e143302_d_n10;
            locals.var_dnm_dn13 = assign93520_body0_e143302_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign93520_body1_e143319,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 != 0.0)) {
        let assign93520_body1_e143317: f64 = (locals.var_m0 + 1.0);
        (assign93520_body1_e143317,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign93520_body1_e143319;
            locals.var_m0_rv = 0.0;
        }

        let (assign93530_e143346, assign93530_e143346_d_n0, assign93530_e143346_d_n2, assign93530_e143346_d_n4, assign93530_e143346_d_n5, assign93530_e143346_d_n6, assign93530_e143346_d_n7, assign93530_e143346_d_n8, assign93530_e143346_d_n9, assign93530_e143346_d_n10, assign93530_e143346_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) && (locals.var_guard2163 == 0.0)) {
        let (assign93530_e143344, assign93530_e143344_d_n0, assign93530_e143344_d_n2, assign93530_e143344_d_n4, assign93530_e143344_d_n5, assign93530_e143344_d_n6, assign93530_e143344_d_n7, assign93530_e143344_d_n8, assign93530_e143344_d_n9, assign93530_e143344_d_n10, assign93530_e143344_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign93530_e143341: f64 = (2.0 * 2.0);
                let assign93530_e143342: f64 = (1.0 / assign93530_e143341);
                let assign93530_e143343: f64 = (locals.var_dnm).powf(assign93530_e143342);
                (assign93530_e143343, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn0)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn2)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn4)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn5)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn6)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn7)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn8)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn9)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn10)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93530_e143342) as f64).is_finite() && ((assign93530_e143342) as f64).fract() == 0.0 { if assign93530_e143342 == 0.0 { 0.0 } else { (assign93530_e143342 * ((locals.var_dnm).powf(assign93530_e143342 - 1.0) * locals.var_dnm_dn13)) } } else { (assign93530_e143343 * (assign93530_e143342 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign93530_e143344, assign93530_e143344_d_n0, assign93530_e143344_d_n2, assign93530_e143344_d_n4, assign93530_e143344_d_n5, assign93530_e143344_d_n6, assign93530_e143344_d_n7, assign93530_e143344_d_n8, assign93530_e143344_d_n9, assign93530_e143344_d_n10, assign93530_e143344_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign93530_e143346;
        locals.var_dnm_dn0 = assign93530_e143346_d_n0;
        locals.var_dnm_dn2 = assign93530_e143346_d_n2;
        locals.var_dnm_dn4 = assign93530_e143346_d_n4;
        locals.var_dnm_dn5 = assign93530_e143346_d_n5;
        locals.var_dnm_dn6 = assign93530_e143346_d_n6;
        locals.var_dnm_dn7 = assign93530_e143346_d_n7;
        locals.var_dnm_dn8 = assign93530_e143346_d_n8;
        locals.var_dnm_dn9 = assign93530_e143346_d_n9;
        locals.var_dnm_dn10 = assign93530_e143346_d_n10;
        locals.var_dnm_dn13 = assign93530_e143346_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign93540_e143361, assign93540_e143361_d_n0, assign93540_e143361_d_n2, assign93540_e143361_d_n4, assign93540_e143361_d_n5, assign93540_e143361_d_n6, assign93540_e143361_d_n7, assign93540_e143361_d_n8, assign93540_e143361_d_n9, assign93540_e143361_d_n10, assign93540_e143361_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93540_e143359: f64 = (1.0 / locals.var_dnm);
        (assign93540_e143359, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign93540_e143361;
        locals.var_dnm_dn0 = assign93540_e143361_d_n0;
        locals.var_dnm_dn2 = assign93540_e143361_d_n2;
        locals.var_dnm_dn4 = assign93540_e143361_d_n4;
        locals.var_dnm_dn5 = assign93540_e143361_d_n5;
        locals.var_dnm_dn6 = assign93540_e143361_d_n6;
        locals.var_dnm_dn7 = assign93540_e143361_d_n7;
        locals.var_dnm_dn8 = assign93540_e143361_d_n8;
        locals.var_dnm_dn9 = assign93540_e143361_d_n9;
        locals.var_dnm_dn10 = assign93540_e143361_d_n10;
        locals.var_dnm_dn13 = assign93540_e143361_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign93550_e143380, assign93550_e143380_d_n0, assign93550_e143380_d_n2, assign93550_e143380_d_n4, assign93550_e143380_d_n5, assign93550_e143380_d_n6, assign93550_e143380_d_n7, assign93550_e143380_d_n8, assign93550_e143380_d_n9, assign93550_e143380_d_n10, assign93550_e143380_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93550_e143375: f64 = (locals.var_ddriftldc * 0.1);
        let assign93550_e143376: f64 = (locals.var_tmf1 * assign93550_e143375);
        let assign93550_e143378: f64 = (assign93550_e143376 * locals.var_dnm);
        (assign93550_e143378, ((((locals.var_tmf1_dn0 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign93550_e143375) + (locals.var_tmf1 * (locals.var_ddriftldc_dn13 * 0.1))) * locals.var_dnm) + (assign93550_e143376 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign93550_e143380;
        locals.var_tmf0_dn0 = assign93550_e143380_d_n0;
        locals.var_tmf0_dn2 = assign93550_e143380_d_n2;
        locals.var_tmf0_dn4 = assign93550_e143380_d_n4;
        locals.var_tmf0_dn5 = assign93550_e143380_d_n5;
        locals.var_tmf0_dn6 = assign93550_e143380_d_n6;
        locals.var_tmf0_dn7 = assign93550_e143380_d_n7;
        locals.var_tmf0_dn8 = assign93550_e143380_d_n8;
        locals.var_tmf0_dn9 = assign93550_e143380_d_n9;
        locals.var_tmf0_dn10 = assign93550_e143380_d_n10;
        locals.var_tmf0_dn13 = assign93550_e143380_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign93560_e143401, assign93560_e143401_d_n0, assign93560_e143401_d_n2, assign93560_e143401_d_n4, assign93560_e143401_d_n5, assign93560_e143401_d_n6, assign93560_e143401_d_n7, assign93560_e143401_d_n8, assign93560_e143401_d_n9, assign93560_e143401_d_n10, assign93560_e143401_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93560_e143393: f64 = (locals.var_ddriftldc * 0.1);
        let assign93560_e143395: f64 = (assign93560_e143393 * locals.var_xmp);
        let assign93560_e143397: f64 = (assign93560_e143395 * locals.var_dnm);
        let assign93560_e143399: f64 = (assign93560_e143397 / locals.var_arg);
        (assign93560_e143399, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn0)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn2)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn4)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn5)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn6)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn7)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn8)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn9)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn10)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn13 * 0.1) * locals.var_xmp) + (assign93560_e143393 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign93560_e143395 * locals.var_dnm_dn13)) * locals.var_arg) - (assign93560_e143397 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign93560_e143401;
        locals.var_t0_dn0 = assign93560_e143401_d_n0;
        locals.var_t0_dn2 = assign93560_e143401_d_n2;
        locals.var_t0_dn4 = assign93560_e143401_d_n4;
        locals.var_t0_dn5 = assign93560_e143401_d_n5;
        locals.var_t0_dn6 = assign93560_e143401_d_n6;
        locals.var_t0_dn7 = assign93560_e143401_d_n7;
        locals.var_t0_dn8 = assign93560_e143401_d_n8;
        locals.var_t0_dn9 = assign93560_e143401_d_n9;
        locals.var_t0_dn10 = assign93560_e143401_d_n10;
        locals.var_t0_dn13 = assign93560_e143401_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign93570_e143420, assign93570_e143420_d_n0, assign93570_e143420_d_n2, assign93570_e143420_d_n4, assign93570_e143420_d_n5, assign93570_e143420_d_n6, assign93570_e143420_d_n7, assign93570_e143420_d_n8, assign93570_e143420_d_n9, assign93570_e143420_d_n10, assign93570_e143420_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        let assign93570_e143415: f64 = (locals.var_ddriftldc * 0.1);
        let assign93570_e143416: f64 = (locals.var_ddriftldc - assign93570_e143415);
        let assign93570_e143418: f64 = (assign93570_e143416 + locals.var_tmf0);
        (assign93570_e143418, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn13 - (locals.var_ddriftldc_dn13 * 0.1)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign93570_e143420;
        locals.var_t1_dn0 = assign93570_e143420_d_n0;
        locals.var_t1_dn2 = assign93570_e143420_d_n2;
        locals.var_t1_dn4 = assign93570_e143420_d_n4;
        locals.var_t1_dn5 = assign93570_e143420_d_n5;
        locals.var_t1_dn6 = assign93570_e143420_d_n6;
        locals.var_t1_dn7 = assign93570_e143420_d_n7;
        locals.var_t1_dn8 = assign93570_e143420_d_n8;
        locals.var_t1_dn9 = assign93570_e143420_d_n9;
        locals.var_t1_dn10 = assign93570_e143420_d_n10;
        locals.var_t1_dn13 = assign93570_e143420_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign93580_e143433, assign93580_e143433_d_n0, assign93580_e143433_d_n2, assign93580_e143433_d_n4, assign93580_e143433_d_n5, assign93580_e143433_d_n6, assign93580_e143433_d_n7, assign93580_e143433_d_n8, assign93580_e143433_d_n9, assign93580_e143433_d_n10, assign93580_e143433_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign93580_e143433;
        locals.var_t0_dn0 = assign93580_e143433_d_n0;
        locals.var_t0_dn2 = assign93580_e143433_d_n2;
        locals.var_t0_dn4 = assign93580_e143433_d_n4;
        locals.var_t0_dn5 = assign93580_e143433_d_n5;
        locals.var_t0_dn6 = assign93580_e143433_d_n6;
        locals.var_t0_dn7 = assign93580_e143433_d_n7;
        locals.var_t0_dn8 = assign93580_e143433_d_n8;
        locals.var_t0_dn9 = assign93580_e143433_d_n9;
        locals.var_t0_dn10 = assign93580_e143433_d_n10;
        locals.var_t0_dn13 = assign93580_e143433_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign93590_e143447, assign93590_e143447_d_n0, assign93590_e143447_d_n2, assign93590_e143447_d_n4, assign93590_e143447_d_n5, assign93590_e143447_d_n6, assign93590_e143447_d_n7, assign93590_e143447_d_n8, assign93590_e143447_d_n9, assign93590_e143447_d_n10, assign93590_e143447_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 == 0.0)) {
        (locals.var_wdld0__blk2159, locals.var_wdld0__blk2159_dn0, locals.var_wdld0__blk2159_dn2, locals.var_wdld0__blk2159_dn4, locals.var_wdld0__blk2159_dn5, locals.var_wdld0__blk2159_dn6, locals.var_wdld0__blk2159_dn7, locals.var_wdld0__blk2159_dn8, locals.var_wdld0__blk2159_dn9, locals.var_wdld0__blk2159_dn10, locals.var_wdld0__blk2159_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign93590_e143447;
        locals.var_t1_dn0 = assign93590_e143447_d_n0;
        locals.var_t1_dn2 = assign93590_e143447_d_n2;
        locals.var_t1_dn4 = assign93590_e143447_d_n4;
        locals.var_t1_dn5 = assign93590_e143447_d_n5;
        locals.var_t1_dn6 = assign93590_e143447_d_n6;
        locals.var_t1_dn7 = assign93590_e143447_d_n7;
        locals.var_t1_dn8 = assign93590_e143447_d_n8;
        locals.var_t1_dn9 = assign93590_e143447_d_n9;
        locals.var_t1_dn10 = assign93590_e143447_d_n10;
        locals.var_t1_dn13 = assign93590_e143447_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign93600_e143461, assign93600_e143461_d_n0, assign93600_e143461_d_n2, assign93600_e143461_d_n4, assign93600_e143461_d_n5, assign93600_e143461_d_n6, assign93600_e143461_d_n7, assign93600_e143461_d_n8, assign93600_e143461_d_n9, assign93600_e143461_d_n10, assign93600_e143461_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2162 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign93600_e143461;
        locals.var_t0_dn0 = assign93600_e143461_d_n0;
        locals.var_t0_dn2 = assign93600_e143461_d_n2;
        locals.var_t0_dn4 = assign93600_e143461_d_n4;
        locals.var_t0_dn5 = assign93600_e143461_d_n5;
        locals.var_t0_dn6 = assign93600_e143461_d_n6;
        locals.var_t0_dn7 = assign93600_e143461_d_n7;
        locals.var_t0_dn8 = assign93600_e143461_d_n8;
        locals.var_t0_dn9 = assign93600_e143461_d_n9;
        locals.var_t0_dn10 = assign93600_e143461_d_n10;
        locals.var_t0_dn13 = assign93600_e143461_d_n13;
        locals.var_t0_rv = 0.0;

        let assign93610_e143464: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2168 = assign93610_e143464;
        locals.var_guard2168_rv = 0.0;

        let (assign93620_e143479,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 != 0.0)) && (locals.var_guard2168 != 0.0)) {
        let assign93620_e143477: f64 = (locals.var_flg_fd_mode__blk2121 + 2.0);
        (assign93620_e143477,)
    } else {
        (locals.var_flg_fd_mode__blk2121,)
    }
};
        locals.var_flg_fd_mode__blk2121 = assign93620_e143479;
        locals.var_flg_fd_mode__blk2121_rv = 0.0;

        let (assign93630_e143496, assign93630_e143496_d_n0, assign93630_e143496_d_n2, assign93630_e143496_d_n4, assign93630_e143496_d_n5, assign93630_e143496_d_n6, assign93630_e143496_d_n7, assign93630_e143496_d_n8, assign93630_e143496_d_n9, assign93630_e143496_d_n10, assign93630_e143496_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 == 0.0)) {
        let (assign93630_e143494, assign93630_e143494_d_n0, assign93630_e143494_d_n2, assign93630_e143494_d_n4, assign93630_e143494_d_n5, assign93630_e143494_d_n6, assign93630_e143494_d_n7, assign93630_e143494_d_n8, assign93630_e143494_d_n9, assign93630_e143494_d_n10, assign93630_e143494_d_n13,) = {
            if (locals.var_wdld0__blk2159 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk2159, locals.var_wdld0__blk2159_dn0, locals.var_wdld0__blk2159_dn2, locals.var_wdld0__blk2159_dn4, locals.var_wdld0__blk2159_dn5, locals.var_wdld0__blk2159_dn6, locals.var_wdld0__blk2159_dn7, locals.var_wdld0__blk2159_dn8, locals.var_wdld0__blk2159_dn9, locals.var_wdld0__blk2159_dn10, locals.var_wdld0__blk2159_dn13,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
            }
        };
        (assign93630_e143494, assign93630_e143494_d_n0, assign93630_e143494_d_n2, assign93630_e143494_d_n4, assign93630_e143494_d_n5, assign93630_e143494_d_n6, assign93630_e143494_d_n7, assign93630_e143494_d_n8, assign93630_e143494_d_n9, assign93630_e143494_d_n10, assign93630_e143494_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign93630_e143496;
        locals.var_t1_dn0 = assign93630_e143496_d_n0;
        locals.var_t1_dn2 = assign93630_e143496_d_n2;
        locals.var_t1_dn4 = assign93630_e143496_d_n4;
        locals.var_t1_dn5 = assign93630_e143496_d_n5;
        locals.var_t1_dn6 = assign93630_e143496_d_n6;
        locals.var_t1_dn7 = assign93630_e143496_d_n7;
        locals.var_t1_dn8 = assign93630_e143496_d_n8;
        locals.var_t1_dn9 = assign93630_e143496_d_n9;
        locals.var_t1_dn10 = assign93630_e143496_d_n10;
        locals.var_t1_dn13 = assign93630_e143496_d_n13;
        locals.var_t1_rv = 0.0;

        let assign93640_e143499: f64 = if locals.var_wdld0__blk2159 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard2169 = assign93640_e143499;
        locals.var_guard2169_rv = 0.0;

        let (assign93650_e143515,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2161 == 0.0)) && (locals.var_guard2169 != 0.0)) {
        let assign93650_e143513: f64 = (locals.var_flg_fd_mode__blk2121 + 2.0);
        (assign93650_e143513,)
    } else {
        (locals.var_flg_fd_mode__blk2121,)
    }
};
        locals.var_flg_fd_mode__blk2121 = assign93650_e143515;
        locals.var_flg_fd_mode__blk2121_rv = 0.0;

        let assign93660_e143518: f64 = if locals.var_flg_fd_mode__blk2121 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2170 = assign93660_e143518;
        locals.var_guard2170_rv = 0.0;

        let (assign93670_e143529, assign93670_e143529_d_n0, assign93670_e143529_d_n2, assign93670_e143529_d_n4, assign93670_e143529_d_n5, assign93670_e143529_d_n6, assign93670_e143529_d_n7, assign93670_e143529_d_n8, assign93670_e143529_d_n9, assign93670_e143529_d_n10, assign93670_e143529_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_bef1__blk2160, locals.var_ps0ld_bef1__blk2160_dn0, locals.var_ps0ld_bef1__blk2160_dn2, locals.var_ps0ld_bef1__blk2160_dn4, locals.var_ps0ld_bef1__blk2160_dn5, locals.var_ps0ld_bef1__blk2160_dn6, locals.var_ps0ld_bef1__blk2160_dn7, locals.var_ps0ld_bef1__blk2160_dn8, locals.var_ps0ld_bef1__blk2160_dn9, locals.var_ps0ld_bef1__blk2160_dn10, locals.var_ps0ld_bef1__blk2160_dn13,)
    }
};
        locals.var_ps0ld_bef1__blk2160 = assign93670_e143529;
        locals.var_ps0ld_bef1__blk2160_dn0 = assign93670_e143529_d_n0;
        locals.var_ps0ld_bef1__blk2160_dn2 = assign93670_e143529_d_n2;
        locals.var_ps0ld_bef1__blk2160_dn4 = assign93670_e143529_d_n4;
        locals.var_ps0ld_bef1__blk2160_dn5 = assign93670_e143529_d_n5;
        locals.var_ps0ld_bef1__blk2160_dn6 = assign93670_e143529_d_n6;
        locals.var_ps0ld_bef1__blk2160_dn7 = assign93670_e143529_d_n7;
        locals.var_ps0ld_bef1__blk2160_dn8 = assign93670_e143529_d_n8;
        locals.var_ps0ld_bef1__blk2160_dn9 = assign93670_e143529_d_n9;
        locals.var_ps0ld_bef1__blk2160_dn10 = assign93670_e143529_d_n10;
        locals.var_ps0ld_bef1__blk2160_dn13 = assign93670_e143529_d_n13;
        locals.var_ps0ld_bef1__blk2160_rv = 0.0;

        let (assign93680_e143542, assign93680_e143542_d_n0, assign93680_e143542_d_n2, assign93680_e143542_d_n4, assign93680_e143542_d_n5, assign93680_e143542_d_n6, assign93680_e143542_d_n7, assign93680_e143542_d_n8, assign93680_e143542_d_n9, assign93680_e143542_d_n10, assign93680_e143542_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) {
        let assign93680_e143540: f64 = (locals.var_t1 * locals.var_q_nsubld__blk2115);
        (assign93680_e143540, (locals.var_t1_dn0 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn2 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn4 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn5 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn6 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn7 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn8 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn9 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn10 * locals.var_q_nsubld__blk2115), (locals.var_t1_dn13 * locals.var_q_nsubld__blk2115),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign93680_e143542;
        locals.var_qbuld_dn0 = assign93680_e143542_d_n0;
        locals.var_qbuld_dn2 = assign93680_e143542_d_n2;
        locals.var_qbuld_dn4 = assign93680_e143542_d_n4;
        locals.var_qbuld_dn5 = assign93680_e143542_d_n5;
        locals.var_qbuld_dn6 = assign93680_e143542_d_n6;
        locals.var_qbuld_dn7 = assign93680_e143542_d_n7;
        locals.var_qbuld_dn8 = assign93680_e143542_d_n8;
        locals.var_qbuld_dn9 = assign93680_e143542_d_n9;
        locals.var_qbuld_dn10 = assign93680_e143542_d_n10;
        locals.var_qbuld_dn13 = assign93680_e143542_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign93690_e143557, assign93690_e143557_d_n0, assign93690_e143557_d_n2, assign93690_e143557_d_n4, assign93690_e143557_d_n5, assign93690_e143557_d_n6, assign93690_e143557_d_n7, assign93690_e143557_d_n8, assign93690_e143557_d_n9, assign93690_e143557_d_n10, assign93690_e143557_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) {
        let assign93690_e143554: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign93690_e143555: f64 = (locals.var_vgpld - assign93690_e143554);
        (assign93690_e143555, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (locals.var_vgpld_dn6 - (locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (-(locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn13 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign93690_e143557;
        locals.var_ps0ld_dn0 = assign93690_e143557_d_n0;
        locals.var_ps0ld_dn2 = assign93690_e143557_d_n2;
        locals.var_ps0ld_dn4 = assign93690_e143557_d_n4;
        locals.var_ps0ld_dn5 = assign93690_e143557_d_n5;
        locals.var_ps0ld_dn6 = assign93690_e143557_d_n6;
        locals.var_ps0ld_dn7 = assign93690_e143557_d_n7;
        locals.var_ps0ld_dn8 = assign93690_e143557_d_n8;
        locals.var_ps0ld_dn9 = assign93690_e143557_d_n9;
        locals.var_ps0ld_dn10 = assign93690_e143557_d_n10;
        locals.var_ps0ld_dn13 = assign93690_e143557_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let assign93700_e143560: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2171 = assign93700_e143560;
        locals.var_guard2171_rv = 0.0;

        let assign93710_e143564: f64 = (locals.var_ps0ld_bef1__blk2160 - 0.1);
        let assign93710_e143569: f64 = if ((locals.var_ps0ld > assign93710_e143564) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2172 = assign93710_e143569;
        locals.var_guard2172_rv = 0.0;

        let (assign93720_e143588, assign93720_e143588_d_n0, assign93720_e143588_d_n2, assign93720_e143588_d_n4, assign93720_e143588_d_n5, assign93720_e143588_d_n6, assign93720_e143588_d_n7, assign93720_e143588_d_n8, assign93720_e143588_d_n9, assign93720_e143588_d_n10, assign93720_e143588_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93720_e143584: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk2160);
        let assign93720_e143586: f64 = (assign93720_e143584 + 0.1);
        (assign93720_e143586, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk2160_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk2160_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk2160_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk2160_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk2160_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk2160_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk2160_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk2160_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk2160_dn10), (locals.var_ps0ld_dn13 - locals.var_ps0ld_bef1__blk2160_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign93720_e143588;
        locals.var_tmf1_dn0 = assign93720_e143588_d_n0;
        locals.var_tmf1_dn2 = assign93720_e143588_d_n2;
        locals.var_tmf1_dn4 = assign93720_e143588_d_n4;
        locals.var_tmf1_dn5 = assign93720_e143588_d_n5;
        locals.var_tmf1_dn6 = assign93720_e143588_d_n6;
        locals.var_tmf1_dn7 = assign93720_e143588_d_n7;
        locals.var_tmf1_dn8 = assign93720_e143588_d_n8;
        locals.var_tmf1_dn9 = assign93720_e143588_d_n9;
        locals.var_tmf1_dn10 = assign93720_e143588_d_n10;
        locals.var_tmf1_dn13 = assign93720_e143588_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign93730_e143605, assign93730_e143605_d_n0, assign93730_e143605_d_n2, assign93730_e143605_d_n4, assign93730_e143605_d_n5, assign93730_e143605_d_n6, assign93730_e143605_d_n7, assign93730_e143605_d_n8, assign93730_e143605_d_n9, assign93730_e143605_d_n10, assign93730_e143605_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93730_e143603: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign93730_e143603, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign93730_e143605;
        locals.var_x2_dn0 = assign93730_e143605_d_n0;
        locals.var_x2_dn2 = assign93730_e143605_d_n2;
        locals.var_x2_dn4 = assign93730_e143605_d_n4;
        locals.var_x2_dn5 = assign93730_e143605_d_n5;
        locals.var_x2_dn6 = assign93730_e143605_d_n6;
        locals.var_x2_dn7 = assign93730_e143605_d_n7;
        locals.var_x2_dn8 = assign93730_e143605_d_n8;
        locals.var_x2_dn9 = assign93730_e143605_d_n9;
        locals.var_x2_dn10 = assign93730_e143605_d_n10;
        locals.var_x2_dn13 = assign93730_e143605_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign93740_e143622, assign93740_e143622_d_n0, assign93740_e143622_d_n2, assign93740_e143622_d_n4, assign93740_e143622_d_n5, assign93740_e143622_d_n6, assign93740_e143622_d_n7, assign93740_e143622_d_n8, assign93740_e143622_d_n9, assign93740_e143622_d_n10, assign93740_e143622_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93740_e143620: f64 = (0.1 * 0.1);
        (assign93740_e143620, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign93740_e143622;
        locals.var_xmax2_dn0 = assign93740_e143622_d_n0;
        locals.var_xmax2_dn2 = assign93740_e143622_d_n2;
        locals.var_xmax2_dn4 = assign93740_e143622_d_n4;
        locals.var_xmax2_dn5 = assign93740_e143622_d_n5;
        locals.var_xmax2_dn6 = assign93740_e143622_d_n6;
        locals.var_xmax2_dn7 = assign93740_e143622_d_n7;
        locals.var_xmax2_dn8 = assign93740_e143622_d_n8;
        locals.var_xmax2_dn9 = assign93740_e143622_d_n9;
        locals.var_xmax2_dn10 = assign93740_e143622_d_n10;
        locals.var_xmax2_dn13 = assign93740_e143622_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign93750_e143637, assign93750_e143637_d_n0, assign93750_e143637_d_n2, assign93750_e143637_d_n4, assign93750_e143637_d_n5, assign93750_e143637_d_n6, assign93750_e143637_d_n7, assign93750_e143637_d_n8, assign93750_e143637_d_n9, assign93750_e143637_d_n10, assign93750_e143637_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign93750_e143637;
        locals.var_xp_dn0 = assign93750_e143637_d_n0;
        locals.var_xp_dn2 = assign93750_e143637_d_n2;
        locals.var_xp_dn4 = assign93750_e143637_d_n4;
        locals.var_xp_dn5 = assign93750_e143637_d_n5;
        locals.var_xp_dn6 = assign93750_e143637_d_n6;
        locals.var_xp_dn7 = assign93750_e143637_d_n7;
        locals.var_xp_dn8 = assign93750_e143637_d_n8;
        locals.var_xp_dn9 = assign93750_e143637_d_n9;
        locals.var_xp_dn10 = assign93750_e143637_d_n10;
        locals.var_xp_dn13 = assign93750_e143637_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign93760_e143652, assign93760_e143652_d_n0, assign93760_e143652_d_n2, assign93760_e143652_d_n4, assign93760_e143652_d_n5, assign93760_e143652_d_n6, assign93760_e143652_d_n7, assign93760_e143652_d_n8, assign93760_e143652_d_n9, assign93760_e143652_d_n10, assign93760_e143652_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign93760_e143652;
        locals.var_xmp_dn0 = assign93760_e143652_d_n0;
        locals.var_xmp_dn2 = assign93760_e143652_d_n2;
        locals.var_xmp_dn4 = assign93760_e143652_d_n4;
        locals.var_xmp_dn5 = assign93760_e143652_d_n5;
        locals.var_xmp_dn6 = assign93760_e143652_d_n6;
        locals.var_xmp_dn7 = assign93760_e143652_d_n7;
        locals.var_xmp_dn8 = assign93760_e143652_d_n8;
        locals.var_xmp_dn9 = assign93760_e143652_d_n9;
        locals.var_xmp_dn10 = assign93760_e143652_d_n10;
        locals.var_xmp_dn13 = assign93760_e143652_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign93770_e143667,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign93770_e143667;
        locals.var_m0_rv = 0.0;

        let (assign93780_e143682,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93780_e143682;
        locals.var_mm_rv = 0.0;

        let (assign93790_e143697, assign93790_e143697_d_n0, assign93790_e143697_d_n2, assign93790_e143697_d_n4, assign93790_e143697_d_n5, assign93790_e143697_d_n6, assign93790_e143697_d_n7, assign93790_e143697_d_n8, assign93790_e143697_d_n9, assign93790_e143697_d_n10, assign93790_e143697_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign93790_e143697;
        locals.var_arg_dn0 = assign93790_e143697_d_n0;
        locals.var_arg_dn2 = assign93790_e143697_d_n2;
        locals.var_arg_dn4 = assign93790_e143697_d_n4;
        locals.var_arg_dn5 = assign93790_e143697_d_n5;
        locals.var_arg_dn6 = assign93790_e143697_d_n6;
        locals.var_arg_dn7 = assign93790_e143697_d_n7;
        locals.var_arg_dn8 = assign93790_e143697_d_n8;
        locals.var_arg_dn9 = assign93790_e143697_d_n9;
        locals.var_arg_dn10 = assign93790_e143697_d_n10;
        locals.var_arg_dn13 = assign93790_e143697_d_n13;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_350(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign93800_e143712, assign93800_e143712_d_n0, assign93800_e143712_d_n2, assign93800_e143712_d_n4, assign93800_e143712_d_n5, assign93800_e143712_d_n6, assign93800_e143712_d_n7, assign93800_e143712_d_n8, assign93800_e143712_d_n9, assign93800_e143712_d_n10, assign93800_e143712_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign93800_e143712;
        locals.var_dnm_dn0 = assign93800_e143712_d_n0;
        locals.var_dnm_dn2 = assign93800_e143712_d_n2;
        locals.var_dnm_dn4 = assign93800_e143712_d_n4;
        locals.var_dnm_dn5 = assign93800_e143712_d_n5;
        locals.var_dnm_dn6 = assign93800_e143712_d_n6;
        locals.var_dnm_dn7 = assign93800_e143712_d_n7;
        locals.var_dnm_dn8 = assign93800_e143712_d_n8;
        locals.var_dnm_dn9 = assign93800_e143712_d_n9;
        locals.var_dnm_dn10 = assign93800_e143712_d_n10;
        locals.var_dnm_dn13 = assign93800_e143712_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign93810_e143729, assign93810_e143729_d_n0, assign93810_e143729_d_n2, assign93810_e143729_d_n4, assign93810_e143729_d_n5, assign93810_e143729_d_n6, assign93810_e143729_d_n7, assign93810_e143729_d_n8, assign93810_e143729_d_n9, assign93810_e143729_d_n10, assign93810_e143729_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93810_e143727: f64 = (locals.var_xp * locals.var_x2);
        (assign93810_e143727, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign93810_e143729;
        locals.var_xp_dn0 = assign93810_e143729_d_n0;
        locals.var_xp_dn2 = assign93810_e143729_d_n2;
        locals.var_xp_dn4 = assign93810_e143729_d_n4;
        locals.var_xp_dn5 = assign93810_e143729_d_n5;
        locals.var_xp_dn6 = assign93810_e143729_d_n6;
        locals.var_xp_dn7 = assign93810_e143729_d_n7;
        locals.var_xp_dn8 = assign93810_e143729_d_n8;
        locals.var_xp_dn9 = assign93810_e143729_d_n9;
        locals.var_xp_dn10 = assign93810_e143729_d_n10;
        locals.var_xp_dn13 = assign93810_e143729_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign93820_e143746, assign93820_e143746_d_n0, assign93820_e143746_d_n2, assign93820_e143746_d_n4, assign93820_e143746_d_n5, assign93820_e143746_d_n6, assign93820_e143746_d_n7, assign93820_e143746_d_n8, assign93820_e143746_d_n9, assign93820_e143746_d_n10, assign93820_e143746_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93820_e143744: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign93820_e143744, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign93820_e143746;
        locals.var_xmp_dn0 = assign93820_e143746_d_n0;
        locals.var_xmp_dn2 = assign93820_e143746_d_n2;
        locals.var_xmp_dn4 = assign93820_e143746_d_n4;
        locals.var_xmp_dn5 = assign93820_e143746_d_n5;
        locals.var_xmp_dn6 = assign93820_e143746_d_n6;
        locals.var_xmp_dn7 = assign93820_e143746_d_n7;
        locals.var_xmp_dn8 = assign93820_e143746_d_n8;
        locals.var_xmp_dn9 = assign93820_e143746_d_n9;
        locals.var_xmp_dn10 = assign93820_e143746_d_n10;
        locals.var_xmp_dn13 = assign93820_e143746_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign93830_e143763, assign93830_e143763_d_n0, assign93830_e143763_d_n2, assign93830_e143763_d_n4, assign93830_e143763_d_n5, assign93830_e143763_d_n6, assign93830_e143763_d_n7, assign93830_e143763_d_n8, assign93830_e143763_d_n9, assign93830_e143763_d_n10, assign93830_e143763_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93830_e143761: f64 = (locals.var_xp * locals.var_x2);
        (assign93830_e143761, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign93830_e143763;
        locals.var_xp_dn0 = assign93830_e143763_d_n0;
        locals.var_xp_dn2 = assign93830_e143763_d_n2;
        locals.var_xp_dn4 = assign93830_e143763_d_n4;
        locals.var_xp_dn5 = assign93830_e143763_d_n5;
        locals.var_xp_dn6 = assign93830_e143763_d_n6;
        locals.var_xp_dn7 = assign93830_e143763_d_n7;
        locals.var_xp_dn8 = assign93830_e143763_d_n8;
        locals.var_xp_dn9 = assign93830_e143763_d_n9;
        locals.var_xp_dn10 = assign93830_e143763_d_n10;
        locals.var_xp_dn13 = assign93830_e143763_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign93840_e143780, assign93840_e143780_d_n0, assign93840_e143780_d_n2, assign93840_e143780_d_n4, assign93840_e143780_d_n5, assign93840_e143780_d_n6, assign93840_e143780_d_n7, assign93840_e143780_d_n8, assign93840_e143780_d_n9, assign93840_e143780_d_n10, assign93840_e143780_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93840_e143778: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign93840_e143778, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign93840_e143780;
        locals.var_xmp_dn0 = assign93840_e143780_d_n0;
        locals.var_xmp_dn2 = assign93840_e143780_d_n2;
        locals.var_xmp_dn4 = assign93840_e143780_d_n4;
        locals.var_xmp_dn5 = assign93840_e143780_d_n5;
        locals.var_xmp_dn6 = assign93840_e143780_d_n6;
        locals.var_xmp_dn7 = assign93840_e143780_d_n7;
        locals.var_xmp_dn8 = assign93840_e143780_d_n8;
        locals.var_xmp_dn9 = assign93840_e143780_d_n9;
        locals.var_xmp_dn10 = assign93840_e143780_d_n10;
        locals.var_xmp_dn13 = assign93840_e143780_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign93850_e143797, assign93850_e143797_d_n0, assign93850_e143797_d_n2, assign93850_e143797_d_n4, assign93850_e143797_d_n5, assign93850_e143797_d_n6, assign93850_e143797_d_n7, assign93850_e143797_d_n8, assign93850_e143797_d_n9, assign93850_e143797_d_n10, assign93850_e143797_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93850_e143795: f64 = (locals.var_xp + locals.var_xmp);
        (assign93850_e143795, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign93850_e143797;
        locals.var_arg_dn0 = assign93850_e143797_d_n0;
        locals.var_arg_dn2 = assign93850_e143797_d_n2;
        locals.var_arg_dn4 = assign93850_e143797_d_n4;
        locals.var_arg_dn5 = assign93850_e143797_d_n5;
        locals.var_arg_dn6 = assign93850_e143797_d_n6;
        locals.var_arg_dn7 = assign93850_e143797_d_n7;
        locals.var_arg_dn8 = assign93850_e143797_d_n8;
        locals.var_arg_dn9 = assign93850_e143797_d_n9;
        locals.var_arg_dn10 = assign93850_e143797_d_n10;
        locals.var_arg_dn13 = assign93850_e143797_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign93860_e143812, assign93860_e143812_d_n0, assign93860_e143812_d_n2, assign93860_e143812_d_n4, assign93860_e143812_d_n5, assign93860_e143812_d_n6, assign93860_e143812_d_n7, assign93860_e143812_d_n8, assign93860_e143812_d_n9, assign93860_e143812_d_n10, assign93860_e143812_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign93860_e143812;
        locals.var_dnm_dn0 = assign93860_e143812_d_n0;
        locals.var_dnm_dn2 = assign93860_e143812_d_n2;
        locals.var_dnm_dn4 = assign93860_e143812_d_n4;
        locals.var_dnm_dn5 = assign93860_e143812_d_n5;
        locals.var_dnm_dn6 = assign93860_e143812_d_n6;
        locals.var_dnm_dn7 = assign93860_e143812_d_n7;
        locals.var_dnm_dn8 = assign93860_e143812_d_n8;
        locals.var_dnm_dn9 = assign93860_e143812_d_n9;
        locals.var_dnm_dn10 = assign93860_e143812_d_n10;
        locals.var_dnm_dn13 = assign93860_e143812_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign93870_e143827: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2173 = assign93870_e143827;
        locals.var_guard2173_rv = 0.0;

        let assign93880_e143830: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2174 = assign93880_e143830;
        locals.var_guard2174_rv = 0.0;

        let (assign93890_e143849,) = {
    if ((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 != 0.0)) && (locals.var_guard2174 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93890_e143849;
        locals.var_mm_rv = 0.0;

        let assign93900_e143852: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2175 = assign93900_e143852;
        locals.var_guard2175_rv = 0.0;

        let (assign93910_e143874,) = {
    if (((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 != 0.0)) && (locals.var_guard2174 == 0.0)) && (locals.var_guard2175 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93910_e143874;
        locals.var_mm_rv = 0.0;

        let assign93920_e143877: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2176 = assign93920_e143877;
        locals.var_guard2176_rv = 0.0;

        let (assign93930_e143902,) = {
    if ((((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 != 0.0)) && (locals.var_guard2174 == 0.0)) && (locals.var_guard2175 == 0.0)) && (locals.var_guard2176 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93930_e143902;
        locals.var_mm_rv = 0.0;

        let assign93940_e143905: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2177 = assign93940_e143905;
        locals.var_guard2177_rv = 0.0;

        let (assign93950_e143933,) = {
    if (((((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 != 0.0)) && (locals.var_guard2174 == 0.0)) && (locals.var_guard2175 == 0.0)) && (locals.var_guard2176 == 0.0)) && (locals.var_guard2177 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign93950_e143933;
        locals.var_mm_rv = 0.0;

        let (assign93960_e143950,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign93960_e143950;
        locals.var_m0_rv = 0.0;

        let mut assign93970_loop_guard: usize = 0;
        while {
            let assign93970_cond_e143968: f64 = if ((((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign93970_cond_e143968 != 0.0
        } {
            assign93970_loop_guard += 1;
            assert!(assign93970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign93970_body0_e143986, assign93970_body0_e143986_d_n0, assign93970_body0_e143986_d_n2, assign93970_body0_e143986_d_n4, assign93970_body0_e143986_d_n5, assign93970_body0_e143986_d_n6, assign93970_body0_e143986_d_n7, assign93970_body0_e143986_d_n8, assign93970_body0_e143986_d_n9, assign93970_body0_e143986_d_n10, assign93970_body0_e143986_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 != 0.0)) {
        let assign93970_body0_e143984: f64 = (locals.var_dnm).sqrt();
        (assign93970_body0_e143984, (locals.var_dnm_dn0 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn2 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn4 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn5 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn6 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn7 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn8 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn9 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn10 / (2.0 * assign93970_body0_e143984)), (locals.var_dnm_dn13 / (2.0 * assign93970_body0_e143984)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign93970_body0_e143986;
            locals.var_dnm_dn0 = assign93970_body0_e143986_d_n0;
            locals.var_dnm_dn2 = assign93970_body0_e143986_d_n2;
            locals.var_dnm_dn4 = assign93970_body0_e143986_d_n4;
            locals.var_dnm_dn5 = assign93970_body0_e143986_d_n5;
            locals.var_dnm_dn6 = assign93970_body0_e143986_d_n6;
            locals.var_dnm_dn7 = assign93970_body0_e143986_d_n7;
            locals.var_dnm_dn8 = assign93970_body0_e143986_d_n8;
            locals.var_dnm_dn9 = assign93970_body0_e143986_d_n9;
            locals.var_dnm_dn10 = assign93970_body0_e143986_d_n10;
            locals.var_dnm_dn13 = assign93970_body0_e143986_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign93970_body1_e144005,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 != 0.0)) {
        let assign93970_body1_e144003: f64 = (locals.var_m0 + 1.0);
        (assign93970_body1_e144003,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign93970_body1_e144005;
            locals.var_m0_rv = 0.0;
        }

        let (assign93980_e144034, assign93980_e144034_d_n0, assign93980_e144034_d_n2, assign93980_e144034_d_n4, assign93980_e144034_d_n5, assign93980_e144034_d_n6, assign93980_e144034_d_n7, assign93980_e144034_d_n8, assign93980_e144034_d_n9, assign93980_e144034_d_n10, assign93980_e144034_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) && (locals.var_guard2173 == 0.0)) {
        let (assign93980_e144032, assign93980_e144032_d_n0, assign93980_e144032_d_n2, assign93980_e144032_d_n4, assign93980_e144032_d_n5, assign93980_e144032_d_n6, assign93980_e144032_d_n7, assign93980_e144032_d_n8, assign93980_e144032_d_n9, assign93980_e144032_d_n10, assign93980_e144032_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign93980_e144029: f64 = (2.0 * 2.0);
                let assign93980_e144030: f64 = (1.0 / assign93980_e144029);
                let assign93980_e144031: f64 = (locals.var_dnm).powf(assign93980_e144030);
                (assign93980_e144031, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn0)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn2)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn4)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn5)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn6)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn7)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn8)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn9)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn10)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign93980_e144030) as f64).is_finite() && ((assign93980_e144030) as f64).fract() == 0.0 { if assign93980_e144030 == 0.0 { 0.0 } else { (assign93980_e144030 * ((locals.var_dnm).powf(assign93980_e144030 - 1.0) * locals.var_dnm_dn13)) } } else { (assign93980_e144031 * (assign93980_e144030 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign93980_e144032, assign93980_e144032_d_n0, assign93980_e144032_d_n2, assign93980_e144032_d_n4, assign93980_e144032_d_n5, assign93980_e144032_d_n6, assign93980_e144032_d_n7, assign93980_e144032_d_n8, assign93980_e144032_d_n9, assign93980_e144032_d_n10, assign93980_e144032_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign93980_e144034;
        locals.var_dnm_dn0 = assign93980_e144034_d_n0;
        locals.var_dnm_dn2 = assign93980_e144034_d_n2;
        locals.var_dnm_dn4 = assign93980_e144034_d_n4;
        locals.var_dnm_dn5 = assign93980_e144034_d_n5;
        locals.var_dnm_dn6 = assign93980_e144034_d_n6;
        locals.var_dnm_dn7 = assign93980_e144034_d_n7;
        locals.var_dnm_dn8 = assign93980_e144034_d_n8;
        locals.var_dnm_dn9 = assign93980_e144034_d_n9;
        locals.var_dnm_dn10 = assign93980_e144034_d_n10;
        locals.var_dnm_dn13 = assign93980_e144034_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign93990_e144051, assign93990_e144051_d_n0, assign93990_e144051_d_n2, assign93990_e144051_d_n4, assign93990_e144051_d_n5, assign93990_e144051_d_n6, assign93990_e144051_d_n7, assign93990_e144051_d_n8, assign93990_e144051_d_n9, assign93990_e144051_d_n10, assign93990_e144051_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign93990_e144049: f64 = (1.0 / locals.var_dnm);
        (assign93990_e144049, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign93990_e144051;
        locals.var_dnm_dn0 = assign93990_e144051_d_n0;
        locals.var_dnm_dn2 = assign93990_e144051_d_n2;
        locals.var_dnm_dn4 = assign93990_e144051_d_n4;
        locals.var_dnm_dn5 = assign93990_e144051_d_n5;
        locals.var_dnm_dn6 = assign93990_e144051_d_n6;
        locals.var_dnm_dn7 = assign93990_e144051_d_n7;
        locals.var_dnm_dn8 = assign93990_e144051_d_n8;
        locals.var_dnm_dn9 = assign93990_e144051_d_n9;
        locals.var_dnm_dn10 = assign93990_e144051_d_n10;
        locals.var_dnm_dn13 = assign93990_e144051_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign94000_e144070, assign94000_e144070_d_n0, assign94000_e144070_d_n2, assign94000_e144070_d_n4, assign94000_e144070_d_n5, assign94000_e144070_d_n6, assign94000_e144070_d_n7, assign94000_e144070_d_n8, assign94000_e144070_d_n9, assign94000_e144070_d_n10, assign94000_e144070_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign94000_e144066: f64 = (locals.var_tmf1 * 0.1);
        let assign94000_e144068: f64 = (assign94000_e144066 * locals.var_dnm);
        (assign94000_e144068, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign94000_e144066 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign94000_e144070;
        locals.var_tmf0_dn0 = assign94000_e144070_d_n0;
        locals.var_tmf0_dn2 = assign94000_e144070_d_n2;
        locals.var_tmf0_dn4 = assign94000_e144070_d_n4;
        locals.var_tmf0_dn5 = assign94000_e144070_d_n5;
        locals.var_tmf0_dn6 = assign94000_e144070_d_n6;
        locals.var_tmf0_dn7 = assign94000_e144070_d_n7;
        locals.var_tmf0_dn8 = assign94000_e144070_d_n8;
        locals.var_tmf0_dn9 = assign94000_e144070_d_n9;
        locals.var_tmf0_dn10 = assign94000_e144070_d_n10;
        locals.var_tmf0_dn13 = assign94000_e144070_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign94010_e144091, assign94010_e144091_d_n0, assign94010_e144091_d_n2, assign94010_e144091_d_n4, assign94010_e144091_d_n5, assign94010_e144091_d_n6, assign94010_e144091_d_n7, assign94010_e144091_d_n8, assign94010_e144091_d_n9, assign94010_e144091_d_n10, assign94010_e144091_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign94010_e144085: f64 = (0.1 * locals.var_xmp);
        let assign94010_e144087: f64 = (assign94010_e144085 * locals.var_dnm);
        let assign94010_e144089: f64 = (assign94010_e144087 / locals.var_arg);
        (assign94010_e144089, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn0)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn2)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn4)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn5)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn6)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn7)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn8)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn9)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn10)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign94010_e144085 * locals.var_dnm_dn13)) * locals.var_arg) - (assign94010_e144087 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign94010_e144091;
        locals.var_t0_dn0 = assign94010_e144091_d_n0;
        locals.var_t0_dn2 = assign94010_e144091_d_n2;
        locals.var_t0_dn4 = assign94010_e144091_d_n4;
        locals.var_t0_dn5 = assign94010_e144091_d_n5;
        locals.var_t0_dn6 = assign94010_e144091_d_n6;
        locals.var_t0_dn7 = assign94010_e144091_d_n7;
        locals.var_t0_dn8 = assign94010_e144091_d_n8;
        locals.var_t0_dn9 = assign94010_e144091_d_n9;
        locals.var_t0_dn10 = assign94010_e144091_d_n10;
        locals.var_t0_dn13 = assign94010_e144091_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign94020_e144110, assign94020_e144110_d_n0, assign94020_e144110_d_n2, assign94020_e144110_d_n4, assign94020_e144110_d_n5, assign94020_e144110_d_n6, assign94020_e144110_d_n7, assign94020_e144110_d_n8, assign94020_e144110_d_n9, assign94020_e144110_d_n10, assign94020_e144110_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        let assign94020_e144106: f64 = (locals.var_ps0ld_bef1__blk2160 - 0.1);
        let assign94020_e144108: f64 = (assign94020_e144106 + locals.var_tmf0);
        (assign94020_e144108, (locals.var_ps0ld_bef1__blk2160_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk2160_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk2160_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk2160_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk2160_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk2160_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk2160_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk2160_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk2160_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk2160_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign94020_e144110;
        locals.var_ps0ld_dn0 = assign94020_e144110_d_n0;
        locals.var_ps0ld_dn2 = assign94020_e144110_d_n2;
        locals.var_ps0ld_dn4 = assign94020_e144110_d_n4;
        locals.var_ps0ld_dn5 = assign94020_e144110_d_n5;
        locals.var_ps0ld_dn6 = assign94020_e144110_d_n6;
        locals.var_ps0ld_dn7 = assign94020_e144110_d_n7;
        locals.var_ps0ld_dn8 = assign94020_e144110_d_n8;
        locals.var_ps0ld_dn9 = assign94020_e144110_d_n9;
        locals.var_ps0ld_dn10 = assign94020_e144110_d_n10;
        locals.var_ps0ld_dn13 = assign94020_e144110_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign94030_e144125, assign94030_e144125_d_n0, assign94030_e144125_d_n2, assign94030_e144125_d_n4, assign94030_e144125_d_n5, assign94030_e144125_d_n6, assign94030_e144125_d_n7, assign94030_e144125_d_n8, assign94030_e144125_d_n9, assign94030_e144125_d_n10, assign94030_e144125_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign94030_e144125;
        locals.var_t0_dn0 = assign94030_e144125_d_n0;
        locals.var_t0_dn2 = assign94030_e144125_d_n2;
        locals.var_t0_dn4 = assign94030_e144125_d_n4;
        locals.var_t0_dn5 = assign94030_e144125_d_n5;
        locals.var_t0_dn6 = assign94030_e144125_d_n6;
        locals.var_t0_dn7 = assign94030_e144125_d_n7;
        locals.var_t0_dn8 = assign94030_e144125_d_n8;
        locals.var_t0_dn9 = assign94030_e144125_d_n9;
        locals.var_t0_dn10 = assign94030_e144125_d_n10;
        locals.var_t0_dn13 = assign94030_e144125_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign94040_e144141, assign94040_e144141_d_n0, assign94040_e144141_d_n2, assign94040_e144141_d_n4, assign94040_e144141_d_n5, assign94040_e144141_d_n6, assign94040_e144141_d_n7, assign94040_e144141_d_n8, assign94040_e144141_d_n9, assign94040_e144141_d_n10, assign94040_e144141_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign94040_e144141;
        locals.var_ps0ld_dn0 = assign94040_e144141_d_n0;
        locals.var_ps0ld_dn2 = assign94040_e144141_d_n2;
        locals.var_ps0ld_dn4 = assign94040_e144141_d_n4;
        locals.var_ps0ld_dn5 = assign94040_e144141_d_n5;
        locals.var_ps0ld_dn6 = assign94040_e144141_d_n6;
        locals.var_ps0ld_dn7 = assign94040_e144141_d_n7;
        locals.var_ps0ld_dn8 = assign94040_e144141_d_n8;
        locals.var_ps0ld_dn9 = assign94040_e144141_d_n9;
        locals.var_ps0ld_dn10 = assign94040_e144141_d_n10;
        locals.var_ps0ld_dn13 = assign94040_e144141_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign94050_e144157, assign94050_e144157_d_n0, assign94050_e144157_d_n2, assign94050_e144157_d_n4, assign94050_e144157_d_n5, assign94050_e144157_d_n6, assign94050_e144157_d_n7, assign94050_e144157_d_n8, assign94050_e144157_d_n9, assign94050_e144157_d_n10, assign94050_e144157_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2172 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign94050_e144157;
        locals.var_t0_dn0 = assign94050_e144157_d_n0;
        locals.var_t0_dn2 = assign94050_e144157_d_n2;
        locals.var_t0_dn4 = assign94050_e144157_d_n4;
        locals.var_t0_dn5 = assign94050_e144157_d_n5;
        locals.var_t0_dn6 = assign94050_e144157_d_n6;
        locals.var_t0_dn7 = assign94050_e144157_d_n7;
        locals.var_t0_dn8 = assign94050_e144157_d_n8;
        locals.var_t0_dn9 = assign94050_e144157_d_n9;
        locals.var_t0_dn10 = assign94050_e144157_d_n10;
        locals.var_t0_dn13 = assign94050_e144157_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign94060_e144176, assign94060_e144176_d_n0, assign94060_e144176_d_n2, assign94060_e144176_d_n4, assign94060_e144176_d_n5, assign94060_e144176_d_n6, assign94060_e144176_d_n7, assign94060_e144176_d_n8, assign94060_e144176_d_n9, assign94060_e144176_d_n10, assign94060_e144176_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2170 != 0.0)) && (locals.var_guard2171 == 0.0)) {
        let (assign94060_e144174, assign94060_e144174_d_n0, assign94060_e144174_d_n2, assign94060_e144174_d_n4, assign94060_e144174_d_n5, assign94060_e144174_d_n6, assign94060_e144174_d_n7, assign94060_e144174_d_n8, assign94060_e144174_d_n9, assign94060_e144174_d_n10, assign94060_e144174_d_n13,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk2160) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
            } else {
                (locals.var_ps0ld_bef1__blk2160, locals.var_ps0ld_bef1__blk2160_dn0, locals.var_ps0ld_bef1__blk2160_dn2, locals.var_ps0ld_bef1__blk2160_dn4, locals.var_ps0ld_bef1__blk2160_dn5, locals.var_ps0ld_bef1__blk2160_dn6, locals.var_ps0ld_bef1__blk2160_dn7, locals.var_ps0ld_bef1__blk2160_dn8, locals.var_ps0ld_bef1__blk2160_dn9, locals.var_ps0ld_bef1__blk2160_dn10, locals.var_ps0ld_bef1__blk2160_dn13,)
            }
        };
        (assign94060_e144174, assign94060_e144174_d_n0, assign94060_e144174_d_n2, assign94060_e144174_d_n4, assign94060_e144174_d_n5, assign94060_e144174_d_n6, assign94060_e144174_d_n7, assign94060_e144174_d_n8, assign94060_e144174_d_n9, assign94060_e144174_d_n10, assign94060_e144174_d_n13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign94060_e144176;
        locals.var_ps0ld_dn0 = assign94060_e144176_d_n0;
        locals.var_ps0ld_dn2 = assign94060_e144176_d_n2;
        locals.var_ps0ld_dn4 = assign94060_e144176_d_n4;
        locals.var_ps0ld_dn5 = assign94060_e144176_d_n5;
        locals.var_ps0ld_dn6 = assign94060_e144176_d_n6;
        locals.var_ps0ld_dn7 = assign94060_e144176_d_n7;
        locals.var_ps0ld_dn8 = assign94060_e144176_d_n8;
        locals.var_ps0ld_dn9 = assign94060_e144176_d_n9;
        locals.var_ps0ld_dn10 = assign94060_e144176_d_n10;
        locals.var_ps0ld_dn13 = assign94060_e144176_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign94070_e144185, assign94070_e144185_d_n0, assign94070_e144185_d_n2, assign94070_e144185_d_n4, assign94070_e144185_d_n5, assign94070_e144185_d_n6, assign94070_e144185_d_n7, assign94070_e144185_d_n8, assign94070_e144185_d_n9, assign94070_e144185_d_n10, assign94070_e144185_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk2122, locals.var_ps0ld_ini__blk2122_dn0, locals.var_ps0ld_ini__blk2122_dn2, locals.var_ps0ld_ini__blk2122_dn4, locals.var_ps0ld_ini__blk2122_dn5, locals.var_ps0ld_ini__blk2122_dn6, locals.var_ps0ld_ini__blk2122_dn7, locals.var_ps0ld_ini__blk2122_dn8, locals.var_ps0ld_ini__blk2122_dn9, locals.var_ps0ld_ini__blk2122_dn10, locals.var_ps0ld_ini__blk2122_dn13,)
    }
};
        locals.var_ps0ld_ini__blk2122 = assign94070_e144185;
        locals.var_ps0ld_ini__blk2122_dn0 = assign94070_e144185_d_n0;
        locals.var_ps0ld_ini__blk2122_dn2 = assign94070_e144185_d_n2;
        locals.var_ps0ld_ini__blk2122_dn4 = assign94070_e144185_d_n4;
        locals.var_ps0ld_ini__blk2122_dn5 = assign94070_e144185_d_n5;
        locals.var_ps0ld_ini__blk2122_dn6 = assign94070_e144185_d_n6;
        locals.var_ps0ld_ini__blk2122_dn7 = assign94070_e144185_d_n7;
        locals.var_ps0ld_ini__blk2122_dn8 = assign94070_e144185_d_n8;
        locals.var_ps0ld_ini__blk2122_dn9 = assign94070_e144185_d_n9;
        locals.var_ps0ld_ini__blk2122_dn10 = assign94070_e144185_d_n10;
        locals.var_ps0ld_ini__blk2122_dn13 = assign94070_e144185_d_n13;
        locals.var_ps0ld_ini__blk2122_rv = 0.0;

        let assign94080_e144188: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2178 = assign94080_e144188;
        locals.var_guard2178_rv = 0.0;

        let (assign94090_e144199,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign94090_e144199;
        locals.var_flg_conv_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_351(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94100_e144217, assign94100_e144217_d_n0, assign94100_e144217_d_n2, assign94100_e144217_d_n4, assign94100_e144217_d_n5, assign94100_e144217_d_n6, assign94100_e144217_d_n7, assign94100_e144217_d_n8, assign94100_e144217_d_n9, assign94100_e144217_d_n10, assign94100_e144217_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94100_e144211: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2115);
        let assign94100_e144213: f64 = (assign94100_e144211 * locals.var_beta_inv);
        let assign94100_e144214: f64 = (2.0 * assign94100_e144213);
        let assign94100_e144215: f64 = (assign94100_e144214).sqrt();
        (assign94100_e144215, ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn0)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn2)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn4)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn5)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn6)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn7)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn8)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn9)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn10)) / (2.0 * assign94100_e144215)), ((2.0 * (assign94100_e144211 * locals.var_beta_inv_dn13)) / (2.0 * assign94100_e144215)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign94100_e144217;
        locals.var_c_w_ld_dn0 = assign94100_e144217_d_n0;
        locals.var_c_w_ld_dn2 = assign94100_e144217_d_n2;
        locals.var_c_w_ld_dn4 = assign94100_e144217_d_n4;
        locals.var_c_w_ld_dn5 = assign94100_e144217_d_n5;
        locals.var_c_w_ld_dn6 = assign94100_e144217_d_n6;
        locals.var_c_w_ld_dn7 = assign94100_e144217_d_n7;
        locals.var_c_w_ld_dn8 = assign94100_e144217_d_n8;
        locals.var_c_w_ld_dn9 = assign94100_e144217_d_n9;
        locals.var_c_w_ld_dn10 = assign94100_e144217_d_n10;
        locals.var_c_w_ld_dn13 = assign94100_e144217_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign94110_e144220: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2179 = assign94110_e144220;
        locals.var_guard2179_rv = 0.0;

        let (assign94120_e144235, assign94120_e144235_d_n0, assign94120_e144235_d_n2, assign94120_e144235_d_n4, assign94120_e144235_d_n5, assign94120_e144235_d_n6, assign94120_e144235_d_n7, assign94120_e144235_d_n8, assign94120_e144235_d_n9, assign94120_e144235_d_n10, assign94120_e144235_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2179 != 0.0)) {
        let assign94120_e144233: f64 = (p.p334 - locals.var_wdep_func);
        (assign94120_e144233, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94120_e144235;
        locals.var_t2_dn0 = assign94120_e144235_d_n0;
        locals.var_t2_dn2 = assign94120_e144235_d_n2;
        locals.var_t2_dn4 = assign94120_e144235_d_n4;
        locals.var_t2_dn5 = assign94120_e144235_d_n5;
        locals.var_t2_dn6 = assign94120_e144235_d_n6;
        locals.var_t2_dn7 = assign94120_e144235_d_n7;
        locals.var_t2_dn8 = assign94120_e144235_d_n8;
        locals.var_t2_dn9 = assign94120_e144235_d_n9;
        locals.var_t2_dn10 = assign94120_e144235_d_n10;
        locals.var_t2_dn13 = assign94120_e144235_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94130_e144262, assign94130_e144262_d_n0, assign94130_e144262_d_n2, assign94130_e144262_d_n4, assign94130_e144262_d_n5, assign94130_e144262_d_n6, assign94130_e144262_d_n7, assign94130_e144262_d_n8, assign94130_e144262_d_n9, assign94130_e144262_d_n10, assign94130_e144262_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2179 == 0.0)) {
        let assign94130_e144249: f64 = (locals.var_vdsi + p.p137);
        let assign94130_e144252: f64 = (locals.var_vdsi + p.p137);
        let assign94130_e144253: f64 = (assign94130_e144249 * assign94130_e144252);
        let assign94130_e144256: f64 = (4.0 * 0.1);
        let assign94130_e144258: f64 = (assign94130_e144256 * 0.1);
        let assign94130_e144259: f64 = (assign94130_e144253 + assign94130_e144258);
        let assign94130_e144260: f64 = (assign94130_e144259).sqrt();
        (assign94130_e144260, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign94130_e144252) + (assign94130_e144249 * locals.var_vdsi_dn5)) / (2.0 * assign94130_e144260)), 0.0, (((locals.var_vdsi_dn7 * assign94130_e144252) + (assign94130_e144249 * locals.var_vdsi_dn7)) / (2.0 * assign94130_e144260)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94130_e144262;
        locals.var_tmf2_dn0 = assign94130_e144262_d_n0;
        locals.var_tmf2_dn2 = assign94130_e144262_d_n2;
        locals.var_tmf2_dn4 = assign94130_e144262_d_n4;
        locals.var_tmf2_dn5 = assign94130_e144262_d_n5;
        locals.var_tmf2_dn6 = assign94130_e144262_d_n6;
        locals.var_tmf2_dn7 = assign94130_e144262_d_n7;
        locals.var_tmf2_dn8 = assign94130_e144262_d_n8;
        locals.var_tmf2_dn9 = assign94130_e144262_d_n9;
        locals.var_tmf2_dn10 = assign94130_e144262_d_n10;
        locals.var_tmf2_dn13 = assign94130_e144262_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign94140_e144284, assign94140_e144284_d_n0, assign94140_e144284_d_n2, assign94140_e144284_d_n4, assign94140_e144284_d_n5, assign94140_e144284_d_n6, assign94140_e144284_d_n7, assign94140_e144284_d_n8, assign94140_e144284_d_n9, assign94140_e144284_d_n10, assign94140_e144284_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2179 == 0.0)) {
        let assign94140_e144278: f64 = (locals.var_vdsi + p.p137);
        let assign94140_e144280: f64 = (assign94140_e144278 / locals.var_tmf2);
        let assign94140_e144281: f64 = (1.0 + assign94140_e144280);
        let assign94140_e144282: f64 = (0.5 * assign94140_e144281);
        (assign94140_e144282, (0.5 * (-((assign94140_e144278 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94140_e144278 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94140_e144278 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign94140_e144278 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign94140_e144278 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign94140_e144278 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign94140_e144278 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94140_e144278 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94140_e144278 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94140_e144278 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94140_e144284;
        locals.var_t9_dn0 = assign94140_e144284_d_n0;
        locals.var_t9_dn2 = assign94140_e144284_d_n2;
        locals.var_t9_dn4 = assign94140_e144284_d_n4;
        locals.var_t9_dn5 = assign94140_e144284_d_n5;
        locals.var_t9_dn6 = assign94140_e144284_d_n6;
        locals.var_t9_dn7 = assign94140_e144284_d_n7;
        locals.var_t9_dn8 = assign94140_e144284_d_n8;
        locals.var_t9_dn9 = assign94140_e144284_d_n9;
        locals.var_t9_dn10 = assign94140_e144284_d_n10;
        locals.var_t9_dn13 = assign94140_e144284_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94150_e144304, assign94150_e144304_d_n0, assign94150_e144304_d_n2, assign94150_e144304_d_n4, assign94150_e144304_d_n5, assign94150_e144304_d_n6, assign94150_e144304_d_n7, assign94150_e144304_d_n8, assign94150_e144304_d_n9, assign94150_e144304_d_n10, assign94150_e144304_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2179 == 0.0)) {
        let assign94150_e144299: f64 = (locals.var_vdsi + p.p137);
        let assign94150_e144301: f64 = (assign94150_e144299 + locals.var_tmf2);
        let assign94150_e144302: f64 = (0.5 * assign94150_e144301);
        (assign94150_e144302, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94150_e144304;
        locals.var_t2_dn0 = assign94150_e144304_d_n0;
        locals.var_t2_dn2 = assign94150_e144304_d_n2;
        locals.var_t2_dn4 = assign94150_e144304_d_n4;
        locals.var_t2_dn5 = assign94150_e144304_d_n5;
        locals.var_t2_dn6 = assign94150_e144304_d_n6;
        locals.var_t2_dn7 = assign94150_e144304_d_n7;
        locals.var_t2_dn8 = assign94150_e144304_d_n8;
        locals.var_t2_dn9 = assign94150_e144304_d_n9;
        locals.var_t2_dn10 = assign94150_e144304_d_n10;
        locals.var_t2_dn13 = assign94150_e144304_d_n13;
        locals.var_t2_rv = 0.0;

        let assign94160_e144307: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2180 = assign94160_e144307;
        locals.var_guard2180_rv = 0.0;

        let (assign94170_e144323, assign94170_e144323_d_n0, assign94170_e144323_d_n2, assign94170_e144323_d_n4, assign94170_e144323_d_n5, assign94170_e144323_d_n6, assign94170_e144323_d_n7, assign94170_e144323_d_n8, assign94170_e144323_d_n9, assign94170_e144323_d_n10, assign94170_e144323_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2179 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94170_e144323;
        locals.var_t2_dn0 = assign94170_e144323_d_n0;
        locals.var_t2_dn2 = assign94170_e144323_d_n2;
        locals.var_t2_dn4 = assign94170_e144323_d_n4;
        locals.var_t2_dn5 = assign94170_e144323_d_n5;
        locals.var_t2_dn6 = assign94170_e144323_d_n6;
        locals.var_t2_dn7 = assign94170_e144323_d_n7;
        locals.var_t2_dn8 = assign94170_e144323_d_n8;
        locals.var_t2_dn9 = assign94170_e144323_d_n9;
        locals.var_t2_dn10 = assign94170_e144323_d_n10;
        locals.var_t2_dn13 = assign94170_e144323_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94180_e144339, assign94180_e144339_d_n0, assign94180_e144339_d_n2, assign94180_e144339_d_n4, assign94180_e144339_d_n5, assign94180_e144339_d_n6, assign94180_e144339_d_n7, assign94180_e144339_d_n8, assign94180_e144339_d_n9, assign94180_e144339_d_n10, assign94180_e144339_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2179 == 0.0)) && (locals.var_guard2180 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94180_e144339;
        locals.var_t9_dn0 = assign94180_e144339_d_n0;
        locals.var_t9_dn2 = assign94180_e144339_d_n2;
        locals.var_t9_dn4 = assign94180_e144339_d_n4;
        locals.var_t9_dn5 = assign94180_e144339_d_n5;
        locals.var_t9_dn6 = assign94180_e144339_d_n6;
        locals.var_t9_dn7 = assign94180_e144339_d_n7;
        locals.var_t9_dn8 = assign94180_e144339_d_n8;
        locals.var_t9_dn9 = assign94180_e144339_d_n9;
        locals.var_t9_dn10 = assign94180_e144339_d_n10;
        locals.var_t9_dn13 = assign94180_e144339_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94190_e144358, assign94190_e144358_d_n0, assign94190_e144358_d_n2, assign94190_e144358_d_n4, assign94190_e144358_d_n5, assign94190_e144358_d_n6, assign94190_e144358_d_n7, assign94190_e144358_d_n8, assign94190_e144358_d_n9, assign94190_e144358_d_n10, assign94190_e144358_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2179 == 0.0)) {
        let assign94190_e144353: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94190_e144354: f64 = (assign94190_e144353).sqrt();
        let assign94190_e144356: f64 = (assign94190_e144354 * p.p432);
        (assign94190_e144356, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94190_e144354)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign94190_e144354)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign94190_e144358;
        locals.var_wjunc0_dn0 = assign94190_e144358_d_n0;
        locals.var_wjunc0_dn2 = assign94190_e144358_d_n2;
        locals.var_wjunc0_dn4 = assign94190_e144358_d_n4;
        locals.var_wjunc0_dn5 = assign94190_e144358_d_n5;
        locals.var_wjunc0_dn6 = assign94190_e144358_d_n6;
        locals.var_wjunc0_dn7 = assign94190_e144358_d_n7;
        locals.var_wjunc0_dn8 = assign94190_e144358_d_n8;
        locals.var_wjunc0_dn9 = assign94190_e144358_d_n9;
        locals.var_wjunc0_dn10 = assign94190_e144358_d_n10;
        locals.var_wjunc0_dn13 = assign94190_e144358_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign94200_e144374, assign94200_e144374_d_n0, assign94200_e144374_d_n2, assign94200_e144374_d_n4, assign94200_e144374_d_n5, assign94200_e144374_d_n6, assign94200_e144374_d_n7, assign94200_e144374_d_n8, assign94200_e144374_d_n9, assign94200_e144374_d_n10, assign94200_e144374_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2179 == 0.0)) {
        let assign94200_e144372: f64 = (p.p334 - locals.var_wjunc0);
        (assign94200_e144372, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94200_e144374;
        locals.var_t2_dn0 = assign94200_e144374_d_n0;
        locals.var_t2_dn2 = assign94200_e144374_d_n2;
        locals.var_t2_dn4 = assign94200_e144374_d_n4;
        locals.var_t2_dn5 = assign94200_e144374_d_n5;
        locals.var_t2_dn6 = assign94200_e144374_d_n6;
        locals.var_t2_dn7 = assign94200_e144374_d_n7;
        locals.var_t2_dn8 = assign94200_e144374_d_n8;
        locals.var_t2_dn9 = assign94200_e144374_d_n9;
        locals.var_t2_dn10 = assign94200_e144374_d_n10;
        locals.var_t2_dn13 = assign94200_e144374_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94210_e144398, assign94210_e144398_d_n0, assign94210_e144398_d_n2, assign94210_e144398_d_n4, assign94210_e144398_d_n5, assign94210_e144398_d_n6, assign94210_e144398_d_n7, assign94210_e144398_d_n8, assign94210_e144398_d_n9, assign94210_e144398_d_n10, assign94210_e144398_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94210_e144385: f64 = (locals.var_t2 * locals.var_t2);
        let assign94210_e144389: f64 = (p.p334 * 0.01);
        let assign94210_e144390: f64 = (4.0 * assign94210_e144389);
        let assign94210_e144393: f64 = (p.p334 * 0.01);
        let assign94210_e144394: f64 = (assign94210_e144390 * assign94210_e144393);
        let assign94210_e144395: f64 = (assign94210_e144385 + assign94210_e144394);
        let assign94210_e144396: f64 = (assign94210_e144395).sqrt();
        (assign94210_e144396, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign94210_e144396)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign94210_e144396)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94210_e144398;
        locals.var_tmf2_dn0 = assign94210_e144398_d_n0;
        locals.var_tmf2_dn2 = assign94210_e144398_d_n2;
        locals.var_tmf2_dn4 = assign94210_e144398_d_n4;
        locals.var_tmf2_dn5 = assign94210_e144398_d_n5;
        locals.var_tmf2_dn6 = assign94210_e144398_d_n6;
        locals.var_tmf2_dn7 = assign94210_e144398_d_n7;
        locals.var_tmf2_dn8 = assign94210_e144398_d_n8;
        locals.var_tmf2_dn9 = assign94210_e144398_d_n9;
        locals.var_tmf2_dn10 = assign94210_e144398_d_n10;
        locals.var_tmf2_dn13 = assign94210_e144398_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign94220_e144415, assign94220_e144415_d_n0, assign94220_e144415_d_n2, assign94220_e144415_d_n4, assign94220_e144415_d_n5, assign94220_e144415_d_n6, assign94220_e144415_d_n7, assign94220_e144415_d_n8, assign94220_e144415_d_n9, assign94220_e144415_d_n10, assign94220_e144415_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94220_e144411: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign94220_e144412: f64 = (1.0 + assign94220_e144411);
        let assign94220_e144413: f64 = (0.5 * assign94220_e144412);
        (assign94220_e144413, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94220_e144415;
        locals.var_t9_dn0 = assign94220_e144415_d_n0;
        locals.var_t9_dn2 = assign94220_e144415_d_n2;
        locals.var_t9_dn4 = assign94220_e144415_d_n4;
        locals.var_t9_dn5 = assign94220_e144415_d_n5;
        locals.var_t9_dn6 = assign94220_e144415_d_n6;
        locals.var_t9_dn7 = assign94220_e144415_d_n7;
        locals.var_t9_dn8 = assign94220_e144415_d_n8;
        locals.var_t9_dn9 = assign94220_e144415_d_n9;
        locals.var_t9_dn10 = assign94220_e144415_d_n10;
        locals.var_t9_dn13 = assign94220_e144415_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94230_e144430, assign94230_e144430_d_n0, assign94230_e144430_d_n2, assign94230_e144430_d_n4, assign94230_e144430_d_n5, assign94230_e144430_d_n6, assign94230_e144430_d_n7, assign94230_e144430_d_n8, assign94230_e144430_d_n9, assign94230_e144430_d_n10, assign94230_e144430_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94230_e144427: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign94230_e144428: f64 = (0.5 * assign94230_e144427);
        (assign94230_e144428, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94230_e144430;
        locals.var_t2_dn0 = assign94230_e144430_d_n0;
        locals.var_t2_dn2 = assign94230_e144430_d_n2;
        locals.var_t2_dn4 = assign94230_e144430_d_n4;
        locals.var_t2_dn5 = assign94230_e144430_d_n5;
        locals.var_t2_dn6 = assign94230_e144430_d_n6;
        locals.var_t2_dn7 = assign94230_e144430_d_n7;
        locals.var_t2_dn8 = assign94230_e144430_d_n8;
        locals.var_t2_dn9 = assign94230_e144430_d_n9;
        locals.var_t2_dn10 = assign94230_e144430_d_n10;
        locals.var_t2_dn13 = assign94230_e144430_d_n13;
        locals.var_t2_rv = 0.0;

        let assign94240_e144433: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2181 = assign94240_e144433;
        locals.var_guard2181_rv = 0.0;

        let (assign94250_e144446, assign94250_e144446_d_n0, assign94250_e144446_d_n2, assign94250_e144446_d_n4, assign94250_e144446_d_n5, assign94250_e144446_d_n6, assign94250_e144446_d_n7, assign94250_e144446_d_n8, assign94250_e144446_d_n9, assign94250_e144446_d_n10, assign94250_e144446_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2181 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94250_e144446;
        locals.var_t2_dn0 = assign94250_e144446_d_n0;
        locals.var_t2_dn2 = assign94250_e144446_d_n2;
        locals.var_t2_dn4 = assign94250_e144446_d_n4;
        locals.var_t2_dn5 = assign94250_e144446_d_n5;
        locals.var_t2_dn6 = assign94250_e144446_d_n6;
        locals.var_t2_dn7 = assign94250_e144446_d_n7;
        locals.var_t2_dn8 = assign94250_e144446_d_n8;
        locals.var_t2_dn9 = assign94250_e144446_d_n9;
        locals.var_t2_dn10 = assign94250_e144446_d_n10;
        locals.var_t2_dn13 = assign94250_e144446_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94260_e144459, assign94260_e144459_d_n0, assign94260_e144459_d_n2, assign94260_e144459_d_n4, assign94260_e144459_d_n5, assign94260_e144459_d_n6, assign94260_e144459_d_n7, assign94260_e144459_d_n8, assign94260_e144459_d_n9, assign94260_e144459_d_n10, assign94260_e144459_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2181 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94260_e144459;
        locals.var_t9_dn0 = assign94260_e144459_d_n0;
        locals.var_t9_dn2 = assign94260_e144459_d_n2;
        locals.var_t9_dn4 = assign94260_e144459_d_n4;
        locals.var_t9_dn5 = assign94260_e144459_d_n5;
        locals.var_t9_dn6 = assign94260_e144459_d_n6;
        locals.var_t9_dn7 = assign94260_e144459_d_n7;
        locals.var_t9_dn8 = assign94260_e144459_d_n8;
        locals.var_t9_dn9 = assign94260_e144459_d_n9;
        locals.var_t9_dn10 = assign94260_e144459_d_n10;
        locals.var_t9_dn13 = assign94260_e144459_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94270_e144470, assign94270_e144470_d_n0, assign94270_e144470_d_n2, assign94270_e144470_d_n4, assign94270_e144470_d_n5, assign94270_e144470_d_n6, assign94270_e144470_d_n7, assign94270_e144470_d_n8, assign94270_e144470_d_n9, assign94270_e144470_d_n10, assign94270_e144470_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign94270_e144470;
        locals.var_ddriftldc_dn0 = assign94270_e144470_d_n0;
        locals.var_ddriftldc_dn2 = assign94270_e144470_d_n2;
        locals.var_ddriftldc_dn4 = assign94270_e144470_d_n4;
        locals.var_ddriftldc_dn5 = assign94270_e144470_d_n5;
        locals.var_ddriftldc_dn6 = assign94270_e144470_d_n6;
        locals.var_ddriftldc_dn7 = assign94270_e144470_d_n7;
        locals.var_ddriftldc_dn8 = assign94270_e144470_d_n8;
        locals.var_ddriftldc_dn9 = assign94270_e144470_d_n9;
        locals.var_ddriftldc_dn10 = assign94270_e144470_d_n10;
        locals.var_ddriftldc_dn13 = assign94270_e144470_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign94280_e144489, assign94280_e144489_d_n0, assign94280_e144489_d_n2, assign94280_e144489_d_n4, assign94280_e144489_d_n5, assign94280_e144489_d_n6, assign94280_e144489_d_n7, assign94280_e144489_d_n8, assign94280_e144489_d_n9, assign94280_e144489_d_n10, assign94280_e144489_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94280_e144481: f64 = (locals.var_q_nsubld__blk2115 * locals.var_ddriftldc);
        let assign94280_e144483: f64 = (assign94280_e144481 * locals.var_ddriftldc);
        let assign94280_e144485: f64 = (assign94280_e144483 / 2.0);
        let assign94280_e144487: f64 = (assign94280_e144485 / 1.034943e-10);
        (assign94280_e144487, (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign94280_e144481 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign94280_e144489;
        locals.var_dphi_sb_dn0 = assign94280_e144489_d_n0;
        locals.var_dphi_sb_dn2 = assign94280_e144489_d_n2;
        locals.var_dphi_sb_dn4 = assign94280_e144489_d_n4;
        locals.var_dphi_sb_dn5 = assign94280_e144489_d_n5;
        locals.var_dphi_sb_dn6 = assign94280_e144489_d_n6;
        locals.var_dphi_sb_dn7 = assign94280_e144489_d_n7;
        locals.var_dphi_sb_dn8 = assign94280_e144489_d_n8;
        locals.var_dphi_sb_dn9 = assign94280_e144489_d_n9;
        locals.var_dphi_sb_dn10 = assign94280_e144489_d_n10;
        locals.var_dphi_sb_dn13 = assign94280_e144489_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign94290_e144505, assign94290_e144505_d_n0, assign94290_e144505_d_n2, assign94290_e144505_d_n4, assign94290_e144505_d_n5, assign94290_e144505_d_n6, assign94290_e144505_d_n7, assign94290_e144505_d_n8, assign94290_e144505_d_n9, assign94290_e144505_d_n10, assign94290_e144505_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94290_e144500: f64 = (2.0 * locals.var_beta);
        let assign94290_e144502: f64 = (assign94290_e144500 * locals.var_dphi_sb);
        let assign94290_e144503: f64 = (assign94290_e144502).sqrt();
        (assign94290_e144503, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn0)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn2)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn4)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn5)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn6)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn7)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn8)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn9)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn10)) / (2.0 * assign94290_e144503)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign94290_e144500 * locals.var_dphi_sb_dn13)) / (2.0 * assign94290_e144503)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign94290_e144505;
        locals.var_t0_dn0 = assign94290_e144505_d_n0;
        locals.var_t0_dn2 = assign94290_e144505_d_n2;
        locals.var_t0_dn4 = assign94290_e144505_d_n4;
        locals.var_t0_dn5 = assign94290_e144505_d_n5;
        locals.var_t0_dn6 = assign94290_e144505_d_n6;
        locals.var_t0_dn7 = assign94290_e144505_d_n7;
        locals.var_t0_dn8 = assign94290_e144505_d_n8;
        locals.var_t0_dn9 = assign94290_e144505_d_n9;
        locals.var_t0_dn10 = assign94290_e144505_d_n10;
        locals.var_t0_dn13 = assign94290_e144505_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign94300_e144523, assign94300_e144523_d_n0, assign94300_e144523_d_n2, assign94300_e144523_d_n4, assign94300_e144523_d_n5, assign94300_e144523_d_n6, assign94300_e144523_d_n7, assign94300_e144523_d_n8, assign94300_e144523_d_n9, assign94300_e144523_d_n10, assign94300_e144523_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94300_e144515: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94300_e144517: f64 = (-locals.var_t0);
        let assign94300_e144518: f64 = { let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94300_e144519: f64 = (assign94300_e144515 + assign94300_e144518);
        let assign94300_e144521: f64 = (assign94300_e144519 / 2.0);
        (assign94300_e144521, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign94300_e144517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign94300_e144523;
        locals.var_t1_dn0 = assign94300_e144523_d_n0;
        locals.var_t1_dn2 = assign94300_e144523_d_n2;
        locals.var_t1_dn4 = assign94300_e144523_d_n4;
        locals.var_t1_dn5 = assign94300_e144523_d_n5;
        locals.var_t1_dn6 = assign94300_e144523_d_n6;
        locals.var_t1_dn7 = assign94300_e144523_d_n7;
        locals.var_t1_dn8 = assign94300_e144523_d_n8;
        locals.var_t1_dn9 = assign94300_e144523_d_n9;
        locals.var_t1_dn10 = assign94300_e144523_d_n10;
        locals.var_t1_dn13 = assign94300_e144523_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign94310_e144537, assign94310_e144537_d_n0, assign94310_e144537_d_n2, assign94310_e144537_d_n4, assign94310_e144537_d_n5, assign94310_e144537_d_n6, assign94310_e144537_d_n7, assign94310_e144537_d_n8, assign94310_e144537_d_n9, assign94310_e144537_d_n10, assign94310_e144537_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94310_e144533: f64 = (locals.var_t1).ln();
        let assign94310_e144535: f64 = (assign94310_e144533 / locals.var_dphi_sb);
        (assign94310_e144535, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign94310_e144533 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign94310_e144537;
        locals.var_c_sb_dn0 = assign94310_e144537_d_n0;
        locals.var_c_sb_dn2 = assign94310_e144537_d_n2;
        locals.var_c_sb_dn4 = assign94310_e144537_d_n4;
        locals.var_c_sb_dn5 = assign94310_e144537_d_n5;
        locals.var_c_sb_dn6 = assign94310_e144537_d_n6;
        locals.var_c_sb_dn7 = assign94310_e144537_d_n7;
        locals.var_c_sb_dn8 = assign94310_e144537_d_n8;
        locals.var_c_sb_dn9 = assign94310_e144537_d_n9;
        locals.var_c_sb_dn10 = assign94310_e144537_d_n10;
        locals.var_c_sb_dn13 = assign94310_e144537_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign94320_e144548,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign94320_e144548;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_352(
        locals: &mut StampLocals,
    ) {
        let mut assign94330_loop_guard: usize = 0;
        while {
            let assign94330_cond_e144560: f64 = (locals.var_lp_s0_max + 1.0);
            let assign94330_cond_e144562: f64 = if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_lp_s0 <= assign94330_cond_e144560)) { 1.0 } else { 0.0 };
            assign94330_cond_e144562 != 0.0
        } {
            assign94330_loop_guard += 1;
            assert!(assign94330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign94330_body3_e144604, assign94330_body3_e144604_d_n0, assign94330_body3_e144604_d_n2, assign94330_body3_e144604_d_n4, assign94330_body3_e144604_d_n5, assign94330_body3_e144604_d_n6, assign94330_body3_e144604_d_n7, assign94330_body3_e144604_d_n8, assign94330_body3_e144604_d_n9, assign94330_body3_e144604_d_n10, assign94330_body3_e144604_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94330_body3_e144602: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign94330_body3_e144602, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign94330_body3_e144604;
            locals.var_ps0ld_vxb_dn0 = assign94330_body3_e144604_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign94330_body3_e144604_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign94330_body3_e144604_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign94330_body3_e144604_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign94330_body3_e144604_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign94330_body3_e144604_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign94330_body3_e144604_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign94330_body3_e144604_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign94330_body3_e144604_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign94330_body3_e144604_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign94330_body4_e144617, assign94330_body4_e144617_d_n0, assign94330_body4_e144617_d_n2, assign94330_body4_e144617_d_n4, assign94330_body4_e144617_d_n5, assign94330_body4_e144617_d_n6, assign94330_body4_e144617_d_n7, assign94330_body4_e144617_d_n8, assign94330_body4_e144617_d_n9, assign94330_body4_e144617_d_n10, assign94330_body4_e144617_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94330_body4_e144615: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign94330_body4_e144615, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign94330_body4_e144617;
            locals.var_chi_dn0 = assign94330_body4_e144617_d_n0;
            locals.var_chi_dn2 = assign94330_body4_e144617_d_n2;
            locals.var_chi_dn4 = assign94330_body4_e144617_d_n4;
            locals.var_chi_dn5 = assign94330_body4_e144617_d_n5;
            locals.var_chi_dn6 = assign94330_body4_e144617_d_n6;
            locals.var_chi_dn7 = assign94330_body4_e144617_d_n7;
            locals.var_chi_dn8 = assign94330_body4_e144617_d_n8;
            locals.var_chi_dn9 = assign94330_body4_e144617_d_n9;
            locals.var_chi_dn10 = assign94330_body4_e144617_d_n10;
            locals.var_chi_dn13 = assign94330_body4_e144617_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign94330_body5_e144632, assign94330_body5_e144632_d_n0, assign94330_body5_e144632_d_n2, assign94330_body5_e144632_d_n4, assign94330_body5_e144632_d_n5, assign94330_body5_e144632_d_n6, assign94330_body5_e144632_d_n7, assign94330_body5_e144632_d_n8, assign94330_body5_e144632_d_n9, assign94330_body5_e144632_d_n10, assign94330_body5_e144632_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94330_body5_e144629: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign94330_body5_e144630: f64 = (locals.var_c_sb * assign94330_body5_e144629);
        (assign94330_body5_e144630, ((locals.var_c_sb_dn0 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign94330_body5_e144629) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign94330_body5_e144632;
            locals.var_ty_dn0 = assign94330_body5_e144632_d_n0;
            locals.var_ty_dn2 = assign94330_body5_e144632_d_n2;
            locals.var_ty_dn4 = assign94330_body5_e144632_d_n4;
            locals.var_ty_dn5 = assign94330_body5_e144632_d_n5;
            locals.var_ty_dn6 = assign94330_body5_e144632_d_n6;
            locals.var_ty_dn7 = assign94330_body5_e144632_d_n7;
            locals.var_ty_dn8 = assign94330_body5_e144632_d_n8;
            locals.var_ty_dn9 = assign94330_body5_e144632_d_n9;
            locals.var_ty_dn10 = assign94330_body5_e144632_d_n10;
            locals.var_ty_dn13 = assign94330_body5_e144632_d_n13;
            locals.var_ty_rv = 0.0;
            let assign94330_body6_e144635: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2183 = assign94330_body6_e144635;
            locals.var_guard2183_rv = 0.0;
            let (assign94330_body7_e144649, assign94330_body7_e144649_d_n0, assign94330_body7_e144649_d_n2, assign94330_body7_e144649_d_n4, assign94330_body7_e144649_d_n5, assign94330_body7_e144649_d_n6, assign94330_body7_e144649_d_n7, assign94330_body7_e144649_d_n8, assign94330_body7_e144649_d_n9, assign94330_body7_e144649_d_n10, assign94330_body7_e144649_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2183 != 0.0)) {
        let assign94330_body7_e144647: f64 = (locals.var_ty).exp();
        (assign94330_body7_e144647, (assign94330_body7_e144647 * locals.var_ty_dn0), (assign94330_body7_e144647 * locals.var_ty_dn2), (assign94330_body7_e144647 * locals.var_ty_dn4), (assign94330_body7_e144647 * locals.var_ty_dn5), (assign94330_body7_e144647 * locals.var_ty_dn6), (assign94330_body7_e144647 * locals.var_ty_dn7), (assign94330_body7_e144647 * locals.var_ty_dn8), (assign94330_body7_e144647 * locals.var_ty_dn9), (assign94330_body7_e144647 * locals.var_ty_dn10), (assign94330_body7_e144647 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94330_body7_e144649;
            locals.var_t1_dn0 = assign94330_body7_e144649_d_n0;
            locals.var_t1_dn2 = assign94330_body7_e144649_d_n2;
            locals.var_t1_dn4 = assign94330_body7_e144649_d_n4;
            locals.var_t1_dn5 = assign94330_body7_e144649_d_n5;
            locals.var_t1_dn6 = assign94330_body7_e144649_d_n6;
            locals.var_t1_dn7 = assign94330_body7_e144649_d_n7;
            locals.var_t1_dn8 = assign94330_body7_e144649_d_n8;
            locals.var_t1_dn9 = assign94330_body7_e144649_d_n9;
            locals.var_t1_dn10 = assign94330_body7_e144649_d_n10;
            locals.var_t1_dn13 = assign94330_body7_e144649_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94330_body8_e144666, assign94330_body8_e144666_d_n0, assign94330_body8_e144666_d_n2, assign94330_body8_e144666_d_n4, assign94330_body8_e144666_d_n5, assign94330_body8_e144666_d_n6, assign94330_body8_e144666_d_n7, assign94330_body8_e144666_d_n8, assign94330_body8_e144666_d_n9, assign94330_body8_e144666_d_n10, assign94330_body8_e144666_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2183 != 0.0)) {
        let assign94330_body8_e144661: f64 = (-locals.var_c_sb);
        let assign94330_body8_e144663: f64 = (assign94330_body8_e144661 * locals.var_dphi_sb);
        let assign94330_body8_e144664: f64 = (assign94330_body8_e144663).exp();
        (assign94330_body8_e144664, (assign94330_body8_e144664 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn0))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn2))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn4))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn5))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn6))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn7))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn8))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn9))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn10))), (assign94330_body8_e144664 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign94330_body8_e144661 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94330_body8_e144666;
            locals.var_t0_dn0 = assign94330_body8_e144666_d_n0;
            locals.var_t0_dn2 = assign94330_body8_e144666_d_n2;
            locals.var_t0_dn4 = assign94330_body8_e144666_d_n4;
            locals.var_t0_dn5 = assign94330_body8_e144666_d_n5;
            locals.var_t0_dn6 = assign94330_body8_e144666_d_n6;
            locals.var_t0_dn7 = assign94330_body8_e144666_d_n7;
            locals.var_t0_dn8 = assign94330_body8_e144666_d_n8;
            locals.var_t0_dn9 = assign94330_body8_e144666_d_n9;
            locals.var_t0_dn10 = assign94330_body8_e144666_d_n10;
            locals.var_t0_dn13 = assign94330_body8_e144666_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94330_body9_e144681, assign94330_body9_e144681_d_n0, assign94330_body9_e144681_d_n2, assign94330_body9_e144681_d_n4, assign94330_body9_e144681_d_n5, assign94330_body9_e144681_d_n6, assign94330_body9_e144681_d_n7, assign94330_body9_e144681_d_n8, assign94330_body9_e144681_d_n9, assign94330_body9_e144681_d_n10, assign94330_body9_e144681_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2183 != 0.0)) {
        let assign94330_body9_e144679: f64 = (locals.var_t1 - locals.var_t0);
        (assign94330_body9_e144679, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign94330_body9_e144681;
            locals.var_t2_dn0 = assign94330_body9_e144681_d_n0;
            locals.var_t2_dn2 = assign94330_body9_e144681_d_n2;
            locals.var_t2_dn4 = assign94330_body9_e144681_d_n4;
            locals.var_t2_dn5 = assign94330_body9_e144681_d_n5;
            locals.var_t2_dn6 = assign94330_body9_e144681_d_n6;
            locals.var_t2_dn7 = assign94330_body9_e144681_d_n7;
            locals.var_t2_dn8 = assign94330_body9_e144681_d_n8;
            locals.var_t2_dn9 = assign94330_body9_e144681_d_n9;
            locals.var_t2_dn10 = assign94330_body9_e144681_d_n10;
            locals.var_t2_dn13 = assign94330_body9_e144681_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign94330_body10_e144699, assign94330_body10_e144699_d_n0, assign94330_body10_e144699_d_n2, assign94330_body10_e144699_d_n4, assign94330_body10_e144699_d_n5, assign94330_body10_e144699_d_n6, assign94330_body10_e144699_d_n7, assign94330_body10_e144699_d_n8, assign94330_body10_e144699_d_n9, assign94330_body10_e144699_d_n10, assign94330_body10_e144699_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2183 != 0.0)) {
        let assign94330_body10_e144694: f64 = (1.0 + locals.var_t2);
        let assign94330_body10_e144695: f64 = (assign94330_body10_e144694).ln();
        let assign94330_body10_e144697: f64 = (assign94330_body10_e144695 / locals.var_c_sb);
        (assign94330_body10_e144697, ((((locals.var_t2_dn0 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign94330_body10_e144694) * locals.var_c_sb) - (assign94330_body10_e144695 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign94330_body10_e144699;
            locals.var_phi_b_dn0 = assign94330_body10_e144699_d_n0;
            locals.var_phi_b_dn2 = assign94330_body10_e144699_d_n2;
            locals.var_phi_b_dn4 = assign94330_body10_e144699_d_n4;
            locals.var_phi_b_dn5 = assign94330_body10_e144699_d_n5;
            locals.var_phi_b_dn6 = assign94330_body10_e144699_d_n6;
            locals.var_phi_b_dn7 = assign94330_body10_e144699_d_n7;
            locals.var_phi_b_dn8 = assign94330_body10_e144699_d_n8;
            locals.var_phi_b_dn9 = assign94330_body10_e144699_d_n9;
            locals.var_phi_b_dn10 = assign94330_body10_e144699_d_n10;
            locals.var_phi_b_dn13 = assign94330_body10_e144699_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign94330_body11_e144716, assign94330_body11_e144716_d_n0, assign94330_body11_e144716_d_n2, assign94330_body11_e144716_d_n4, assign94330_body11_e144716_d_n5, assign94330_body11_e144716_d_n6, assign94330_body11_e144716_d_n7, assign94330_body11_e144716_d_n8, assign94330_body11_e144716_d_n9, assign94330_body11_e144716_d_n10, assign94330_body11_e144716_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2183 != 0.0)) {
        let assign94330_body11_e144713: f64 = (1.0 + locals.var_t2);
        let assign94330_body11_e144714: f64 = (locals.var_t1 / assign94330_body11_e144713);
        (assign94330_body11_e144714, (((locals.var_t1_dn0 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn0)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn2 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn2)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn4 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn4)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn5 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn5)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn6 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn6)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn7 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn7)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn8 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn8)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn9 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn9)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn10 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn10)) / (assign94330_body11_e144713 * assign94330_body11_e144713)), (((locals.var_t1_dn13 * assign94330_body11_e144713) - (locals.var_t1 * locals.var_t2_dn13)) / (assign94330_body11_e144713 * assign94330_body11_e144713)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign94330_body11_e144716;
            locals.var_phi_b_dpss_dn0 = assign94330_body11_e144716_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94330_body11_e144716_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94330_body11_e144716_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94330_body11_e144716_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94330_body11_e144716_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94330_body11_e144716_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94330_body11_e144716_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94330_body11_e144716_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94330_body11_e144716_d_n10;
            locals.var_phi_b_dpss_dn13 = assign94330_body11_e144716_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94330_body13_e144748, assign94330_body13_e144748_d_n0, assign94330_body13_e144748_d_n2, assign94330_body13_e144748_d_n4, assign94330_body13_e144748_d_n5, assign94330_body13_e144748_d_n6, assign94330_body13_e144748_d_n7, assign94330_body13_e144748_d_n8, assign94330_body13_e144748_d_n9, assign94330_body13_e144748_d_n10, assign94330_body13_e144748_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2183 == 0.0)) {
        let assign94330_body13_e144746: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign94330_body13_e144746, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign94330_body13_e144748;
            locals.var_phi_b_dn0 = assign94330_body13_e144748_d_n0;
            locals.var_phi_b_dn2 = assign94330_body13_e144748_d_n2;
            locals.var_phi_b_dn4 = assign94330_body13_e144748_d_n4;
            locals.var_phi_b_dn5 = assign94330_body13_e144748_d_n5;
            locals.var_phi_b_dn6 = assign94330_body13_e144748_d_n6;
            locals.var_phi_b_dn7 = assign94330_body13_e144748_d_n7;
            locals.var_phi_b_dn8 = assign94330_body13_e144748_d_n8;
            locals.var_phi_b_dn9 = assign94330_body13_e144748_d_n9;
            locals.var_phi_b_dn10 = assign94330_body13_e144748_d_n10;
            locals.var_phi_b_dn13 = assign94330_body13_e144748_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign94330_body14_e144762, assign94330_body14_e144762_d_n0, assign94330_body14_e144762_d_n2, assign94330_body14_e144762_d_n4, assign94330_body14_e144762_d_n5, assign94330_body14_e144762_d_n6, assign94330_body14_e144762_d_n7, assign94330_body14_e144762_d_n8, assign94330_body14_e144762_d_n9, assign94330_body14_e144762_d_n10, assign94330_body14_e144762_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2183 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign94330_body14_e144762;
            locals.var_phi_b_dpss_dn0 = assign94330_body14_e144762_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94330_body14_e144762_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94330_body14_e144762_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94330_body14_e144762_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94330_body14_e144762_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94330_body14_e144762_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94330_body14_e144762_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94330_body14_e144762_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94330_body14_e144762_d_n10;
            locals.var_phi_b_dpss_dn13 = assign94330_body14_e144762_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94330_body15_e144775, assign94330_body15_e144775_d_n0, assign94330_body15_e144775_d_n2, assign94330_body15_e144775_d_n4, assign94330_body15_e144775_d_n5, assign94330_body15_e144775_d_n6, assign94330_body15_e144775_d_n7, assign94330_body15_e144775_d_n8, assign94330_body15_e144775_d_n9, assign94330_body15_e144775_d_n10, assign94330_body15_e144775_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94330_body15_e144773: f64 = (locals.var_beta * locals.var_phi_b);
        (assign94330_body15_e144773, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign94330_body15_e144775;
            locals.var_chib_dn0 = assign94330_body15_e144775_d_n0;
            locals.var_chib_dn2 = assign94330_body15_e144775_d_n2;
            locals.var_chib_dn4 = assign94330_body15_e144775_d_n4;
            locals.var_chib_dn5 = assign94330_body15_e144775_d_n5;
            locals.var_chib_dn6 = assign94330_body15_e144775_d_n6;
            locals.var_chib_dn7 = assign94330_body15_e144775_d_n7;
            locals.var_chib_dn8 = assign94330_body15_e144775_d_n8;
            locals.var_chib_dn9 = assign94330_body15_e144775_d_n9;
            locals.var_chib_dn10 = assign94330_body15_e144775_d_n10;
            locals.var_chib_dn13 = assign94330_body15_e144775_d_n13;
            locals.var_chib_rv = 0.0;
            let assign94330_body16_e144778: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2184 = assign94330_body16_e144778;
            locals.var_guard2184_rv = 0.0;
            let (assign94330_body18_e144807, assign94330_body18_e144807_d_n0, assign94330_body18_e144807_d_n2, assign94330_body18_e144807_d_n4, assign94330_body18_e144807_d_n5, assign94330_body18_e144807_d_n6, assign94330_body18_e144807_d_n7, assign94330_body18_e144807_d_n8, assign94330_body18_e144807_d_n9, assign94330_body18_e144807_d_n10, assign94330_body18_e144807_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 != 0.0)) {
        let assign94330_body18_e144805: f64 = (-0.7071067811865475);
        (assign94330_body18_e144805, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94330_body18_e144807;
            locals.var_t0_dn0 = assign94330_body18_e144807_d_n0;
            locals.var_t0_dn2 = assign94330_body18_e144807_d_n2;
            locals.var_t0_dn4 = assign94330_body18_e144807_d_n4;
            locals.var_t0_dn5 = assign94330_body18_e144807_d_n5;
            locals.var_t0_dn6 = assign94330_body18_e144807_d_n6;
            locals.var_t0_dn7 = assign94330_body18_e144807_d_n7;
            locals.var_t0_dn8 = assign94330_body18_e144807_d_n8;
            locals.var_t0_dn9 = assign94330_body18_e144807_d_n9;
            locals.var_t0_dn10 = assign94330_body18_e144807_d_n10;
            locals.var_t0_dn13 = assign94330_body18_e144807_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94330_body19_e144822, assign94330_body19_e144822_d_n0, assign94330_body19_e144822_d_n2, assign94330_body19_e144822_d_n4, assign94330_body19_e144822_d_n5, assign94330_body19_e144822_d_n6, assign94330_body19_e144822_d_n7, assign94330_body19_e144822_d_n8, assign94330_body19_e144822_d_n9, assign94330_body19_e144822_d_n10, assign94330_body19_e144822_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 != 0.0)) {
        let assign94330_body19_e144820: f64 = (locals.var_chi * locals.var_t0);
        (assign94330_body19_e144820, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign94330_body19_e144822;
            locals.var_fb_dn0 = assign94330_body19_e144822_d_n0;
            locals.var_fb_dn2 = assign94330_body19_e144822_d_n2;
            locals.var_fb_dn4 = assign94330_body19_e144822_d_n4;
            locals.var_fb_dn5 = assign94330_body19_e144822_d_n5;
            locals.var_fb_dn6 = assign94330_body19_e144822_d_n6;
            locals.var_fb_dn7 = assign94330_body19_e144822_d_n7;
            locals.var_fb_dn8 = assign94330_body19_e144822_d_n8;
            locals.var_fb_dn9 = assign94330_body19_e144822_d_n9;
            locals.var_fb_dn10 = assign94330_body19_e144822_d_n10;
            locals.var_fb_dn13 = assign94330_body19_e144822_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign94330_body20_e144837, assign94330_body20_e144837_d_n0, assign94330_body20_e144837_d_n2, assign94330_body20_e144837_d_n4, assign94330_body20_e144837_d_n5, assign94330_body20_e144837_d_n6, assign94330_body20_e144837_d_n7, assign94330_body20_e144837_d_n8, assign94330_body20_e144837_d_n9, assign94330_body20_e144837_d_n10, assign94330_body20_e144837_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 != 0.0)) {
        let assign94330_body20_e144835: f64 = (locals.var_beta * locals.var_t0);
        (assign94330_body20_e144835, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign94330_body20_e144837;
            locals.var_fb_dpss_dn0 = assign94330_body20_e144837_d_n0;
            locals.var_fb_dpss_dn2 = assign94330_body20_e144837_d_n2;
            locals.var_fb_dpss_dn4 = assign94330_body20_e144837_d_n4;
            locals.var_fb_dpss_dn5 = assign94330_body20_e144837_d_n5;
            locals.var_fb_dpss_dn6 = assign94330_body20_e144837_d_n6;
            locals.var_fb_dpss_dn7 = assign94330_body20_e144837_d_n7;
            locals.var_fb_dpss_dn8 = assign94330_body20_e144837_d_n8;
            locals.var_fb_dpss_dn9 = assign94330_body20_e144837_d_n9;
            locals.var_fb_dpss_dn10 = assign94330_body20_e144837_d_n10;
            locals.var_fb_dpss_dn13 = assign94330_body20_e144837_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign94330_body21_e144840: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2185 = assign94330_body21_e144840;
            locals.var_guard2185_rv = 0.0;
            let (assign94330_body23_e144896, assign94330_body23_e144896_d_n0, assign94330_body23_e144896_d_n2, assign94330_body23_e144896_d_n4, assign94330_body23_e144896_d_n5, assign94330_body23_e144896_d_n6, assign94330_body23_e144896_d_n7, assign94330_body23_e144896_d_n8, assign94330_body23_e144896_d_n9, assign94330_body23_e144896_d_n10, assign94330_body23_e144896_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94330_body23_e144874: f64 = (locals.var_chi * locals.var_chi);
        let assign94330_body23_e144876: f64 = (assign94330_body23_e144874 / 2.0);
        let assign94330_body23_e144880: f64 = (locals.var_chi / 3.0);
        let assign94330_body23_e144884: f64 = (locals.var_chi / 4.0);
        let assign94330_body23_e144888: f64 = (locals.var_chi / 5.0);
        let assign94330_body23_e144889: f64 = (1.0 - assign94330_body23_e144888);
        let assign94330_body23_e144890: f64 = (assign94330_body23_e144884 * assign94330_body23_e144889);
        let assign94330_body23_e144891: f64 = (1.0 - assign94330_body23_e144890);
        let assign94330_body23_e144892: f64 = (assign94330_body23_e144880 * assign94330_body23_e144891);
        let assign94330_body23_e144893: f64 = (1.0 - assign94330_body23_e144892);
        let assign94330_body23_e144894: f64 = (assign94330_body23_e144876 * assign94330_body23_e144893);
        (assign94330_body23_e144894, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn0 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn0 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn2 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn2 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn4 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn4 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn5 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn5 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn6 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn6 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn7 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn7 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn8 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn8 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn9 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn9 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn10 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn10 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign94330_body23_e144893) + (assign94330_body23_e144876 * (-(((locals.var_chi_dn13 / 3.0) * assign94330_body23_e144891) + (assign94330_body23_e144880 * (-(((locals.var_chi_dn13 / 4.0) * assign94330_body23_e144889) + (assign94330_body23_e144884 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94330_body23_e144896;
            locals.var_t0_dn0 = assign94330_body23_e144896_d_n0;
            locals.var_t0_dn2 = assign94330_body23_e144896_d_n2;
            locals.var_t0_dn4 = assign94330_body23_e144896_d_n4;
            locals.var_t0_dn5 = assign94330_body23_e144896_d_n5;
            locals.var_t0_dn6 = assign94330_body23_e144896_d_n6;
            locals.var_t0_dn7 = assign94330_body23_e144896_d_n7;
            locals.var_t0_dn8 = assign94330_body23_e144896_d_n8;
            locals.var_t0_dn9 = assign94330_body23_e144896_d_n9;
            locals.var_t0_dn10 = assign94330_body23_e144896_d_n10;
            locals.var_t0_dn13 = assign94330_body23_e144896_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94330_body24_e144930, assign94330_body24_e144930_d_n0, assign94330_body24_e144930_d_n2, assign94330_body24_e144930_d_n4, assign94330_body24_e144930_d_n5, assign94330_body24_e144930_d_n6, assign94330_body24_e144930_d_n7, assign94330_body24_e144930_d_n8, assign94330_body24_e144930_d_n9, assign94330_body24_e144930_d_n10, assign94330_body24_e144930_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94330_body24_e144914: f64 = (locals.var_chi / 2.0);
        let assign94330_body24_e144918: f64 = (locals.var_chi / 3.0);
        let assign94330_body24_e144922: f64 = (locals.var_chi / 4.0);
        let assign94330_body24_e144923: f64 = (1.0 - assign94330_body24_e144922);
        let assign94330_body24_e144924: f64 = (assign94330_body24_e144918 * assign94330_body24_e144923);
        let assign94330_body24_e144925: f64 = (1.0 - assign94330_body24_e144924);
        let assign94330_body24_e144926: f64 = (assign94330_body24_e144914 * assign94330_body24_e144925);
        let assign94330_body24_e144927: f64 = (1.0 - assign94330_body24_e144926);
        let assign94330_body24_e144928: f64 = (locals.var_chi * assign94330_body24_e144927);
        (assign94330_body24_e144928, ((locals.var_chi_dn0 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn0 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn2 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn4 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn5 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn6 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn7 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn8 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn9 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn10 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign94330_body24_e144927) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign94330_body24_e144925) + (assign94330_body24_e144914 * (-(((locals.var_chi_dn13 / 3.0) * assign94330_body24_e144923) + (assign94330_body24_e144918 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94330_body24_e144930;
            locals.var_t1_dn0 = assign94330_body24_e144930_d_n0;
            locals.var_t1_dn2 = assign94330_body24_e144930_d_n2;
            locals.var_t1_dn4 = assign94330_body24_e144930_d_n4;
            locals.var_t1_dn5 = assign94330_body24_e144930_d_n5;
            locals.var_t1_dn6 = assign94330_body24_e144930_d_n6;
            locals.var_t1_dn7 = assign94330_body24_e144930_d_n7;
            locals.var_t1_dn8 = assign94330_body24_e144930_d_n8;
            locals.var_t1_dn9 = assign94330_body24_e144930_d_n9;
            locals.var_t1_dn10 = assign94330_body24_e144930_d_n10;
            locals.var_t1_dn13 = assign94330_body24_e144930_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94330_body25_e144968, assign94330_body25_e144968_d_n0, assign94330_body25_e144968_d_n2, assign94330_body25_e144968_d_n4, assign94330_body25_e144968_d_n5, assign94330_body25_e144968_d_n6, assign94330_body25_e144968_d_n7, assign94330_body25_e144968_d_n8, assign94330_body25_e144968_d_n9, assign94330_body25_e144968_d_n10, assign94330_body25_e144968_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94330_body25_e144946: f64 = (locals.var_chib * locals.var_chib);
        let assign94330_body25_e144948: f64 = (assign94330_body25_e144946 / 2.0);
        let assign94330_body25_e144952: f64 = (locals.var_chib / 3.0);
        let assign94330_body25_e144956: f64 = (locals.var_chib / 4.0);
        let assign94330_body25_e144960: f64 = (locals.var_chib / 5.0);
        let assign94330_body25_e144961: f64 = (1.0 - assign94330_body25_e144960);
        let assign94330_body25_e144962: f64 = (assign94330_body25_e144956 * assign94330_body25_e144961);
        let assign94330_body25_e144963: f64 = (1.0 - assign94330_body25_e144962);
        let assign94330_body25_e144964: f64 = (assign94330_body25_e144952 * assign94330_body25_e144963);
        let assign94330_body25_e144965: f64 = (1.0 - assign94330_body25_e144964);
        let assign94330_body25_e144966: f64 = (assign94330_body25_e144948 * assign94330_body25_e144965);
        (assign94330_body25_e144966, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn0 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn0 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn2 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn2 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn4 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn4 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn5 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn5 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn6 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn6 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn7 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn7 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn8 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn8 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn9 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn9 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn10 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn10 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign94330_body25_e144965) + (assign94330_body25_e144948 * (-(((locals.var_chib_dn13 / 3.0) * assign94330_body25_e144963) + (assign94330_body25_e144952 * (-(((locals.var_chib_dn13 / 4.0) * assign94330_body25_e144961) + (assign94330_body25_e144956 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign94330_body25_e144968;
            locals.var_t2_dn0 = assign94330_body25_e144968_d_n0;
            locals.var_t2_dn2 = assign94330_body25_e144968_d_n2;
            locals.var_t2_dn4 = assign94330_body25_e144968_d_n4;
            locals.var_t2_dn5 = assign94330_body25_e144968_d_n5;
            locals.var_t2_dn6 = assign94330_body25_e144968_d_n6;
            locals.var_t2_dn7 = assign94330_body25_e144968_d_n7;
            locals.var_t2_dn8 = assign94330_body25_e144968_d_n8;
            locals.var_t2_dn9 = assign94330_body25_e144968_d_n9;
            locals.var_t2_dn10 = assign94330_body25_e144968_d_n10;
            locals.var_t2_dn13 = assign94330_body25_e144968_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign94330_body26_e145002, assign94330_body26_e145002_d_n0, assign94330_body26_e145002_d_n2, assign94330_body26_e145002_d_n4, assign94330_body26_e145002_d_n5, assign94330_body26_e145002_d_n6, assign94330_body26_e145002_d_n7, assign94330_body26_e145002_d_n8, assign94330_body26_e145002_d_n9, assign94330_body26_e145002_d_n10, assign94330_body26_e145002_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94330_body26_e144986: f64 = (locals.var_chib / 2.0);
        let assign94330_body26_e144990: f64 = (locals.var_chib / 3.0);
        let assign94330_body26_e144994: f64 = (locals.var_chib / 4.0);
        let assign94330_body26_e144995: f64 = (1.0 - assign94330_body26_e144994);
        let assign94330_body26_e144996: f64 = (assign94330_body26_e144990 * assign94330_body26_e144995);
        let assign94330_body26_e144997: f64 = (1.0 - assign94330_body26_e144996);
        let assign94330_body26_e144998: f64 = (assign94330_body26_e144986 * assign94330_body26_e144997);
        let assign94330_body26_e144999: f64 = (1.0 - assign94330_body26_e144998);
        let assign94330_body26_e145000: f64 = (locals.var_chib * assign94330_body26_e144999);
        (assign94330_body26_e145000, ((locals.var_chib_dn0 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn0 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn2 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn4 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn5 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn6 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn7 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn8 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn9 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn10 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign94330_body26_e144999) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign94330_body26_e144997) + (assign94330_body26_e144986 * (-(((locals.var_chib_dn13 / 3.0) * assign94330_body26_e144995) + (assign94330_body26_e144990 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign94330_body26_e145002;
            locals.var_t3_dn0 = assign94330_body26_e145002_d_n0;
            locals.var_t3_dn2 = assign94330_body26_e145002_d_n2;
            locals.var_t3_dn4 = assign94330_body26_e145002_d_n4;
            locals.var_t3_dn5 = assign94330_body26_e145002_d_n5;
            locals.var_t3_dn6 = assign94330_body26_e145002_d_n6;
            locals.var_t3_dn7 = assign94330_body26_e145002_d_n7;
            locals.var_t3_dn8 = assign94330_body26_e145002_d_n8;
            locals.var_t3_dn9 = assign94330_body26_e145002_d_n9;
            locals.var_t3_dn10 = assign94330_body26_e145002_d_n10;
            locals.var_t3_dn13 = assign94330_body26_e145002_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign94330_body27_e145020, assign94330_body27_e145020_d_n0, assign94330_body27_e145020_d_n2, assign94330_body27_e145020_d_n4, assign94330_body27_e145020_d_n5, assign94330_body27_e145020_d_n6, assign94330_body27_e145020_d_n7, assign94330_body27_e145020_d_n8, assign94330_body27_e145020_d_n9, assign94330_body27_e145020_d_n10, assign94330_body27_e145020_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) {
        let assign94330_body27_e145018: f64 = (locals.var_t0 - locals.var_t2);
        (assign94330_body27_e145018, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign94330_body27_e145020;
            locals.var_t4_dn0 = assign94330_body27_e145020_d_n0;
            locals.var_t4_dn2 = assign94330_body27_e145020_d_n2;
            locals.var_t4_dn4 = assign94330_body27_e145020_d_n4;
            locals.var_t4_dn5 = assign94330_body27_e145020_d_n5;
            locals.var_t4_dn6 = assign94330_body27_e145020_d_n6;
            locals.var_t4_dn7 = assign94330_body27_e145020_d_n7;
            locals.var_t4_dn8 = assign94330_body27_e145020_d_n8;
            locals.var_t4_dn9 = assign94330_body27_e145020_d_n9;
            locals.var_t4_dn10 = assign94330_body27_e145020_d_n10;
            locals.var_t4_dn13 = assign94330_body27_e145020_d_n13;
            locals.var_t4_rv = 0.0;
            let assign94330_body28_e145023: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2186 = assign94330_body28_e145023;
            locals.var_guard2186_rv = 0.0;
            let (assign94330_body29_e145042, assign94330_body29_e145042_d_n0, assign94330_body29_e145042_d_n2, assign94330_body29_e145042_d_n4, assign94330_body29_e145042_d_n5, assign94330_body29_e145042_d_n6, assign94330_body29_e145042_d_n7, assign94330_body29_e145042_d_n8, assign94330_body29_e145042_d_n9, assign94330_body29_e145042_d_n10, assign94330_body29_e145042_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) && (locals.var_guard2186 != 0.0)) {
        let assign94330_body29_e145040: f64 = (locals.var_t4).sqrt();
        (assign94330_body29_e145040, (locals.var_t4_dn0 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn2 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn4 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn5 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn6 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn7 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn8 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn9 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn10 / (2.0 * assign94330_body29_e145040)), (locals.var_t4_dn13 / (2.0 * assign94330_body29_e145040)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign94330_body29_e145042;
            locals.var_fb_dn0 = assign94330_body29_e145042_d_n0;
            locals.var_fb_dn2 = assign94330_body29_e145042_d_n2;
            locals.var_fb_dn4 = assign94330_body29_e145042_d_n4;
            locals.var_fb_dn5 = assign94330_body29_e145042_d_n5;
            locals.var_fb_dn6 = assign94330_body29_e145042_d_n6;
            locals.var_fb_dn7 = assign94330_body29_e145042_d_n7;
            locals.var_fb_dn8 = assign94330_body29_e145042_d_n8;
            locals.var_fb_dn9 = assign94330_body29_e145042_d_n9;
            locals.var_fb_dn10 = assign94330_body29_e145042_d_n10;
            locals.var_fb_dn13 = assign94330_body29_e145042_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign94330_body30_e145070, assign94330_body30_e145070_d_n0, assign94330_body30_e145070_d_n2, assign94330_body30_e145070_d_n4, assign94330_body30_e145070_d_n5, assign94330_body30_e145070_d_n6, assign94330_body30_e145070_d_n7, assign94330_body30_e145070_d_n8, assign94330_body30_e145070_d_n9, assign94330_body30_e145070_d_n10, assign94330_body30_e145070_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) && (locals.var_guard2186 != 0.0)) {
        let assign94330_body30_e145060: f64 = (locals.var_beta * 0.5);
        let assign94330_body30_e145064: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign94330_body30_e145065: f64 = (locals.var_t1 - assign94330_body30_e145064);
        let assign94330_body30_e145066: f64 = (assign94330_body30_e145060 * assign94330_body30_e145065);
        let assign94330_body30_e145068: f64 = (assign94330_body30_e145066 / locals.var_fb);
        (assign94330_body30_e145068, ((((((locals.var_beta_dn0 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign94330_body30_e145065) + (assign94330_body30_e145060 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))) * locals.var_fb) - (assign94330_body30_e145066 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign94330_body30_e145070;
            locals.var_fb_dpss_dn0 = assign94330_body30_e145070_d_n0;
            locals.var_fb_dpss_dn2 = assign94330_body30_e145070_d_n2;
            locals.var_fb_dpss_dn4 = assign94330_body30_e145070_d_n4;
            locals.var_fb_dpss_dn5 = assign94330_body30_e145070_d_n5;
            locals.var_fb_dpss_dn6 = assign94330_body30_e145070_d_n6;
            locals.var_fb_dpss_dn7 = assign94330_body30_e145070_d_n7;
            locals.var_fb_dpss_dn8 = assign94330_body30_e145070_d_n8;
            locals.var_fb_dpss_dn9 = assign94330_body30_e145070_d_n9;
            locals.var_fb_dpss_dn10 = assign94330_body30_e145070_d_n10;
            locals.var_fb_dpss_dn13 = assign94330_body30_e145070_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94330_body32_e145110, assign94330_body32_e145110_d_n0, assign94330_body32_e145110_d_n2, assign94330_body32_e145110_d_n4, assign94330_body32_e145110_d_n5, assign94330_body32_e145110_d_n6, assign94330_body32_e145110_d_n7, assign94330_body32_e145110_d_n8, assign94330_body32_e145110_d_n9, assign94330_body32_e145110_d_n10, assign94330_body32_e145110_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) && (locals.var_guard2186 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign94330_body32_e145110;
            locals.var_fb_dn0 = assign94330_body32_e145110_d_n0;
            locals.var_fb_dn2 = assign94330_body32_e145110_d_n2;
            locals.var_fb_dn4 = assign94330_body32_e145110_d_n4;
            locals.var_fb_dn5 = assign94330_body32_e145110_d_n5;
            locals.var_fb_dn6 = assign94330_body32_e145110_d_n6;
            locals.var_fb_dn7 = assign94330_body32_e145110_d_n7;
            locals.var_fb_dn8 = assign94330_body32_e145110_d_n8;
            locals.var_fb_dn9 = assign94330_body32_e145110_d_n9;
            locals.var_fb_dn10 = assign94330_body32_e145110_d_n10;
            locals.var_fb_dn13 = assign94330_body32_e145110_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign94330_body33_e145129, assign94330_body33_e145129_d_n0, assign94330_body33_e145129_d_n2, assign94330_body33_e145129_d_n4, assign94330_body33_e145129_d_n5, assign94330_body33_e145129_d_n6, assign94330_body33_e145129_d_n7, assign94330_body33_e145129_d_n8, assign94330_body33_e145129_d_n9, assign94330_body33_e145129_d_n10, assign94330_body33_e145129_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 != 0.0)) && (locals.var_guard2186 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign94330_body33_e145129;
            locals.var_fb_dpss_dn0 = assign94330_body33_e145129_d_n0;
            locals.var_fb_dpss_dn2 = assign94330_body33_e145129_d_n2;
            locals.var_fb_dpss_dn4 = assign94330_body33_e145129_d_n4;
            locals.var_fb_dpss_dn5 = assign94330_body33_e145129_d_n5;
            locals.var_fb_dpss_dn6 = assign94330_body33_e145129_d_n6;
            locals.var_fb_dpss_dn7 = assign94330_body33_e145129_d_n7;
            locals.var_fb_dpss_dn8 = assign94330_body33_e145129_d_n8;
            locals.var_fb_dpss_dn9 = assign94330_body33_e145129_d_n9;
            locals.var_fb_dpss_dn10 = assign94330_body33_e145129_d_n10;
            locals.var_fb_dpss_dn13 = assign94330_body33_e145129_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94330_body34_e145148, assign94330_body34_e145148_d_n0, assign94330_body34_e145148_d_n2, assign94330_body34_e145148_d_n4, assign94330_body34_e145148_d_n5, assign94330_body34_e145148_d_n6, assign94330_body34_e145148_d_n7, assign94330_body34_e145148_d_n8, assign94330_body34_e145148_d_n9, assign94330_body34_e145148_d_n10, assign94330_body34_e145148_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 == 0.0)) {
        let assign94330_body34_e145145: f64 = (-locals.var_chi);
        let assign94330_body34_e145146: f64 = (assign94330_body34_e145145).exp();
        (assign94330_body34_e145146, (assign94330_body34_e145146 * (-locals.var_chi_dn0)), (assign94330_body34_e145146 * (-locals.var_chi_dn2)), (assign94330_body34_e145146 * (-locals.var_chi_dn4)), (assign94330_body34_e145146 * (-locals.var_chi_dn5)), (assign94330_body34_e145146 * (-locals.var_chi_dn6)), (assign94330_body34_e145146 * (-locals.var_chi_dn7)), (assign94330_body34_e145146 * (-locals.var_chi_dn8)), (assign94330_body34_e145146 * (-locals.var_chi_dn9)), (assign94330_body34_e145146 * (-locals.var_chi_dn10)), (assign94330_body34_e145146 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94330_body34_e145148;
            locals.var_t0_dn0 = assign94330_body34_e145148_d_n0;
            locals.var_t0_dn2 = assign94330_body34_e145148_d_n2;
            locals.var_t0_dn4 = assign94330_body34_e145148_d_n4;
            locals.var_t0_dn5 = assign94330_body34_e145148_d_n5;
            locals.var_t0_dn6 = assign94330_body34_e145148_d_n6;
            locals.var_t0_dn7 = assign94330_body34_e145148_d_n7;
            locals.var_t0_dn8 = assign94330_body34_e145148_d_n8;
            locals.var_t0_dn9 = assign94330_body34_e145148_d_n9;
            locals.var_t0_dn10 = assign94330_body34_e145148_d_n10;
            locals.var_t0_dn13 = assign94330_body34_e145148_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94330_body35_e145167, assign94330_body35_e145167_d_n0, assign94330_body35_e145167_d_n2, assign94330_body35_e145167_d_n4, assign94330_body35_e145167_d_n5, assign94330_body35_e145167_d_n6, assign94330_body35_e145167_d_n7, assign94330_body35_e145167_d_n8, assign94330_body35_e145167_d_n9, assign94330_body35_e145167_d_n10, assign94330_body35_e145167_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 == 0.0)) {
        let assign94330_body35_e145164: f64 = (-locals.var_chib);
        let assign94330_body35_e145165: f64 = (assign94330_body35_e145164).exp();
        (assign94330_body35_e145165, (assign94330_body35_e145165 * (-locals.var_chib_dn0)), (assign94330_body35_e145165 * (-locals.var_chib_dn2)), (assign94330_body35_e145165 * (-locals.var_chib_dn4)), (assign94330_body35_e145165 * (-locals.var_chib_dn5)), (assign94330_body35_e145165 * (-locals.var_chib_dn6)), (assign94330_body35_e145165 * (-locals.var_chib_dn7)), (assign94330_body35_e145165 * (-locals.var_chib_dn8)), (assign94330_body35_e145165 * (-locals.var_chib_dn9)), (assign94330_body35_e145165 * (-locals.var_chib_dn10)), (assign94330_body35_e145165 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94330_body35_e145167;
            locals.var_t1_dn0 = assign94330_body35_e145167_d_n0;
            locals.var_t1_dn2 = assign94330_body35_e145167_d_n2;
            locals.var_t1_dn4 = assign94330_body35_e145167_d_n4;
            locals.var_t1_dn5 = assign94330_body35_e145167_d_n5;
            locals.var_t1_dn6 = assign94330_body35_e145167_d_n6;
            locals.var_t1_dn7 = assign94330_body35_e145167_d_n7;
            locals.var_t1_dn8 = assign94330_body35_e145167_d_n8;
            locals.var_t1_dn9 = assign94330_body35_e145167_d_n9;
            locals.var_t1_dn10 = assign94330_body35_e145167_d_n10;
            locals.var_t1_dn13 = assign94330_body35_e145167_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94330_body36_e145190, assign94330_body36_e145190_d_n0, assign94330_body36_e145190_d_n2, assign94330_body36_e145190_d_n4, assign94330_body36_e145190_d_n5, assign94330_body36_e145190_d_n6, assign94330_body36_e145190_d_n7, assign94330_body36_e145190_d_n8, assign94330_body36_e145190_d_n9, assign94330_body36_e145190_d_n10, assign94330_body36_e145190_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 == 0.0)) {
        let assign94330_body36_e145184: f64 = (locals.var_chi - locals.var_chib);
        let assign94330_body36_e145187: f64 = (locals.var_t0 - locals.var_t1);
        let assign94330_body36_e145188: f64 = (assign94330_body36_e145184 + assign94330_body36_e145187);
        (assign94330_body36_e145188, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign94330_body36_e145190;
            locals.var_t4_dn0 = assign94330_body36_e145190_d_n0;
            locals.var_t4_dn2 = assign94330_body36_e145190_d_n2;
            locals.var_t4_dn4 = assign94330_body36_e145190_d_n4;
            locals.var_t4_dn5 = assign94330_body36_e145190_d_n5;
            locals.var_t4_dn6 = assign94330_body36_e145190_d_n6;
            locals.var_t4_dn7 = assign94330_body36_e145190_d_n7;
            locals.var_t4_dn8 = assign94330_body36_e145190_d_n8;
            locals.var_t4_dn9 = assign94330_body36_e145190_d_n9;
            locals.var_t4_dn10 = assign94330_body36_e145190_d_n10;
            locals.var_t4_dn13 = assign94330_body36_e145190_d_n13;
            locals.var_t4_rv = 0.0;
            let assign94330_body37_e145193: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2187 = assign94330_body37_e145193;
            locals.var_guard2187_rv = 0.0;
            let (assign94330_body38_e145213, assign94330_body38_e145213_d_n0, assign94330_body38_e145213_d_n2, assign94330_body38_e145213_d_n4, assign94330_body38_e145213_d_n5, assign94330_body38_e145213_d_n6, assign94330_body38_e145213_d_n7, assign94330_body38_e145213_d_n8, assign94330_body38_e145213_d_n9, assign94330_body38_e145213_d_n10, assign94330_body38_e145213_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 == 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94330_body38_e145211: f64 = (locals.var_t4).sqrt();
        (assign94330_body38_e145211, (locals.var_t4_dn0 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn2 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn4 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn5 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn6 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn7 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn8 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn9 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn10 / (2.0 * assign94330_body38_e145211)), (locals.var_t4_dn13 / (2.0 * assign94330_body38_e145211)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign94330_body38_e145213;
            locals.var_fb_dn0 = assign94330_body38_e145213_d_n0;
            locals.var_fb_dn2 = assign94330_body38_e145213_d_n2;
            locals.var_fb_dn4 = assign94330_body38_e145213_d_n4;
            locals.var_fb_dn5 = assign94330_body38_e145213_d_n5;
            locals.var_fb_dn6 = assign94330_body38_e145213_d_n6;
            locals.var_fb_dn7 = assign94330_body38_e145213_d_n7;
            locals.var_fb_dn8 = assign94330_body38_e145213_d_n8;
            locals.var_fb_dn9 = assign94330_body38_e145213_d_n9;
            locals.var_fb_dn10 = assign94330_body38_e145213_d_n10;
            locals.var_fb_dn13 = assign94330_body38_e145213_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign94330_body39_e145246, assign94330_body39_e145246_d_n0, assign94330_body39_e145246_d_n2, assign94330_body39_e145246_d_n4, assign94330_body39_e145246_d_n5, assign94330_body39_e145246_d_n6, assign94330_body39_e145246_d_n7, assign94330_body39_e145246_d_n8, assign94330_body39_e145246_d_n9, assign94330_body39_e145246_d_n10, assign94330_body39_e145246_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 == 0.0)) && (locals.var_guard2187 != 0.0)) {
        let assign94330_body39_e145232: f64 = (locals.var_beta * 0.5);
        let assign94330_body39_e145235: f64 = (1.0 - locals.var_t0);
        let assign94330_body39_e145239: f64 = (1.0 - locals.var_t1);
        let assign94330_body39_e145240: f64 = (locals.var_phi_b_dpss * assign94330_body39_e145239);
        let assign94330_body39_e145241: f64 = (assign94330_body39_e145235 - assign94330_body39_e145240);
        let assign94330_body39_e145242: f64 = (assign94330_body39_e145232 * assign94330_body39_e145241);
        let assign94330_body39_e145244: f64 = (assign94330_body39_e145242 / locals.var_fb);
        (assign94330_body39_e145244, ((((((locals.var_beta_dn0 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign94330_body39_e145241) + (assign94330_body39_e145232 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign94330_body39_e145239) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign94330_body39_e145242 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign94330_body39_e145246;
            locals.var_fb_dpss_dn0 = assign94330_body39_e145246_d_n0;
            locals.var_fb_dpss_dn2 = assign94330_body39_e145246_d_n2;
            locals.var_fb_dpss_dn4 = assign94330_body39_e145246_d_n4;
            locals.var_fb_dpss_dn5 = assign94330_body39_e145246_d_n5;
            locals.var_fb_dpss_dn6 = assign94330_body39_e145246_d_n6;
            locals.var_fb_dpss_dn7 = assign94330_body39_e145246_d_n7;
            locals.var_fb_dpss_dn8 = assign94330_body39_e145246_d_n8;
            locals.var_fb_dpss_dn9 = assign94330_body39_e145246_d_n9;
            locals.var_fb_dpss_dn10 = assign94330_body39_e145246_d_n10;
            locals.var_fb_dpss_dn13 = assign94330_body39_e145246_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let (assign94330_body41_e145288, assign94330_body41_e145288_d_n0, assign94330_body41_e145288_d_n2, assign94330_body41_e145288_d_n4, assign94330_body41_e145288_d_n5, assign94330_body41_e145288_d_n6, assign94330_body41_e145288_d_n7, assign94330_body41_e145288_d_n8, assign94330_body41_e145288_d_n9, assign94330_body41_e145288_d_n10, assign94330_body41_e145288_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 == 0.0)) && (locals.var_guard2187 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign94330_body41_e145288;
            locals.var_fb_dn0 = assign94330_body41_e145288_d_n0;
            locals.var_fb_dn2 = assign94330_body41_e145288_d_n2;
            locals.var_fb_dn4 = assign94330_body41_e145288_d_n4;
            locals.var_fb_dn5 = assign94330_body41_e145288_d_n5;
            locals.var_fb_dn6 = assign94330_body41_e145288_d_n6;
            locals.var_fb_dn7 = assign94330_body41_e145288_d_n7;
            locals.var_fb_dn8 = assign94330_body41_e145288_d_n8;
            locals.var_fb_dn9 = assign94330_body41_e145288_d_n9;
            locals.var_fb_dn10 = assign94330_body41_e145288_d_n10;
            locals.var_fb_dn13 = assign94330_body41_e145288_d_n13;
            locals.var_fb_rv = 0.0;
            let (assign94330_body42_e145308, assign94330_body42_e145308_d_n0, assign94330_body42_e145308_d_n2, assign94330_body42_e145308_d_n4, assign94330_body42_e145308_d_n5, assign94330_body42_e145308_d_n6, assign94330_body42_e145308_d_n7, assign94330_body42_e145308_d_n8, assign94330_body42_e145308_d_n9, assign94330_body42_e145308_d_n10, assign94330_body42_e145308_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2184 == 0.0)) && (locals.var_guard2185 == 0.0)) && (locals.var_guard2187 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign94330_body42_e145308;
            locals.var_fb_dpss_dn0 = assign94330_body42_e145308_d_n0;
            locals.var_fb_dpss_dn2 = assign94330_body42_e145308_d_n2;
            locals.var_fb_dpss_dn4 = assign94330_body42_e145308_d_n4;
            locals.var_fb_dpss_dn5 = assign94330_body42_e145308_d_n5;
            locals.var_fb_dpss_dn6 = assign94330_body42_e145308_d_n6;
            locals.var_fb_dpss_dn7 = assign94330_body42_e145308_d_n7;
            locals.var_fb_dpss_dn8 = assign94330_body42_e145308_d_n8;
            locals.var_fb_dpss_dn9 = assign94330_body42_e145308_d_n9;
            locals.var_fb_dpss_dn10 = assign94330_body42_e145308_d_n10;
            locals.var_fb_dpss_dn13 = assign94330_body42_e145308_d_n13;
            locals.var_fb_dpss_rv = 0.0;
            let assign94330_body43_e145311: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2188 = assign94330_body43_e145311;
            locals.var_guard2188_rv = 0.0;
            let (assign94330_body45_e145339, assign94330_body45_e145339_d_n0, assign94330_body45_e145339_d_n2, assign94330_body45_e145339_d_n4, assign94330_body45_e145339_d_n5, assign94330_body45_e145339_d_n6, assign94330_body45_e145339_d_n7, assign94330_body45_e145339_d_n8, assign94330_body45_e145339_d_n9, assign94330_body45_e145339_d_n10, assign94330_body45_e145339_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94330_body45_e145339;
            locals.var_fs01_dn0 = assign94330_body45_e145339_d_n0;
            locals.var_fs01_dn2 = assign94330_body45_e145339_d_n2;
            locals.var_fs01_dn4 = assign94330_body45_e145339_d_n4;
            locals.var_fs01_dn5 = assign94330_body45_e145339_d_n5;
            locals.var_fs01_dn6 = assign94330_body45_e145339_d_n6;
            locals.var_fs01_dn7 = assign94330_body45_e145339_d_n7;
            locals.var_fs01_dn8 = assign94330_body45_e145339_d_n8;
            locals.var_fs01_dn9 = assign94330_body45_e145339_d_n9;
            locals.var_fs01_dn10 = assign94330_body45_e145339_d_n10;
            locals.var_fs01_dn13 = assign94330_body45_e145339_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign94330_body46_e145352, assign94330_body46_e145352_d_n0, assign94330_body46_e145352_d_n2, assign94330_body46_e145352_d_n4, assign94330_body46_e145352_d_n5, assign94330_body46_e145352_d_n6, assign94330_body46_e145352_d_n7, assign94330_body46_e145352_d_n8, assign94330_body46_e145352_d_n9, assign94330_body46_e145352_d_n10, assign94330_body46_e145352_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94330_body46_e145352;
            locals.var_fs01_dps0_dn0 = assign94330_body46_e145352_d_n0;
            locals.var_fs01_dps0_dn2 = assign94330_body46_e145352_d_n2;
            locals.var_fs01_dps0_dn4 = assign94330_body46_e145352_d_n4;
            locals.var_fs01_dps0_dn5 = assign94330_body46_e145352_d_n5;
            locals.var_fs01_dps0_dn6 = assign94330_body46_e145352_d_n6;
            locals.var_fs01_dps0_dn7 = assign94330_body46_e145352_d_n7;
            locals.var_fs01_dps0_dn8 = assign94330_body46_e145352_d_n8;
            locals.var_fs01_dps0_dn9 = assign94330_body46_e145352_d_n9;
            locals.var_fs01_dps0_dn10 = assign94330_body46_e145352_d_n10;
            locals.var_fs01_dps0_dn13 = assign94330_body46_e145352_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94330_body47_e145366, assign94330_body47_e145366_d_n0, assign94330_body47_e145366_d_n2, assign94330_body47_e145366_d_n4, assign94330_body47_e145366_d_n5, assign94330_body47_e145366_d_n6, assign94330_body47_e145366_d_n7, assign94330_body47_e145366_d_n8, assign94330_body47_e145366_d_n9, assign94330_body47_e145366_d_n10, assign94330_body47_e145366_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        let assign94330_body47_e145364: f64 = (-locals.var_fb);
        (assign94330_body47_e145364, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94330_body47_e145366;
            locals.var_fs02_dn0 = assign94330_body47_e145366_d_n0;
            locals.var_fs02_dn2 = assign94330_body47_e145366_d_n2;
            locals.var_fs02_dn4 = assign94330_body47_e145366_d_n4;
            locals.var_fs02_dn5 = assign94330_body47_e145366_d_n5;
            locals.var_fs02_dn6 = assign94330_body47_e145366_d_n6;
            locals.var_fs02_dn7 = assign94330_body47_e145366_d_n7;
            locals.var_fs02_dn8 = assign94330_body47_e145366_d_n8;
            locals.var_fs02_dn9 = assign94330_body47_e145366_d_n9;
            locals.var_fs02_dn10 = assign94330_body47_e145366_d_n10;
            locals.var_fs02_dn13 = assign94330_body47_e145366_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign94330_body48_e145380, assign94330_body48_e145380_d_n0, assign94330_body48_e145380_d_n2, assign94330_body48_e145380_d_n4, assign94330_body48_e145380_d_n5, assign94330_body48_e145380_d_n6, assign94330_body48_e145380_d_n7, assign94330_body48_e145380_d_n8, assign94330_body48_e145380_d_n9, assign94330_body48_e145380_d_n10, assign94330_body48_e145380_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 != 0.0)) {
        let assign94330_body48_e145378: f64 = (-locals.var_fb_dpss);
        (assign94330_body48_e145378, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94330_body48_e145380;
            locals.var_fs02_dps0_dn0 = assign94330_body48_e145380_d_n0;
            locals.var_fs02_dps0_dn2 = assign94330_body48_e145380_d_n2;
            locals.var_fs02_dps0_dn4 = assign94330_body48_e145380_d_n4;
            locals.var_fs02_dps0_dn5 = assign94330_body48_e145380_d_n5;
            locals.var_fs02_dps0_dn6 = assign94330_body48_e145380_d_n6;
            locals.var_fs02_dps0_dn7 = assign94330_body48_e145380_d_n7;
            locals.var_fs02_dps0_dn8 = assign94330_body48_e145380_d_n8;
            locals.var_fs02_dps0_dn9 = assign94330_body48_e145380_d_n9;
            locals.var_fs02_dps0_dn10 = assign94330_body48_e145380_d_n10;
            locals.var_fs02_dps0_dn13 = assign94330_body48_e145380_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign94330_body49_e145383: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2189 = assign94330_body49_e145383;
            locals.var_guard2189_rv = 0.0;
            let assign94330_body50_e145386: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2190 = assign94330_body50_e145386;
            locals.var_guard2190_rv = 0.0;
            let (assign94330_body51_e145426, assign94330_body51_e145426_d_n0, assign94330_body51_e145426_d_n2, assign94330_body51_e145426_d_n4, assign94330_body51_e145426_d_n5, assign94330_body51_e145426_d_n6, assign94330_body51_e145426_d_n7, assign94330_body51_e145426_d_n8, assign94330_body51_e145426_d_n9, assign94330_body51_e145426_d_n10, assign94330_body51_e145426_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        let assign94330_body51_e145404: f64 = (locals.var_chi * locals.var_chi);
        let assign94330_body51_e145406: f64 = (assign94330_body51_e145404 / 2.0);
        let assign94330_body51_e145410: f64 = (locals.var_chi / 3.0);
        let assign94330_body51_e145414: f64 = (locals.var_chi / 4.0);
        let assign94330_body51_e145418: f64 = (locals.var_chi / 5.0);
        let assign94330_body51_e145419: f64 = (1.0 + assign94330_body51_e145418);
        let assign94330_body51_e145420: f64 = (assign94330_body51_e145414 * assign94330_body51_e145419);
        let assign94330_body51_e145421: f64 = (1.0 + assign94330_body51_e145420);
        let assign94330_body51_e145422: f64 = (assign94330_body51_e145410 * assign94330_body51_e145421);
        let assign94330_body51_e145423: f64 = (1.0 + assign94330_body51_e145422);
        let assign94330_body51_e145424: f64 = (assign94330_body51_e145406 * assign94330_body51_e145423);
        (assign94330_body51_e145424, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn0 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn0 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn2 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn2 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn4 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn4 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn5 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn5 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn6 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn6 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn7 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn7 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn8 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn8 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn9 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn9 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn10 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn10 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign94330_body51_e145423) + (assign94330_body51_e145406 * (((locals.var_chi_dn13 / 3.0) * assign94330_body51_e145421) + (assign94330_body51_e145410 * (((locals.var_chi_dn13 / 4.0) * assign94330_body51_e145419) + (assign94330_body51_e145414 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94330_body51_e145426;
            locals.var_t0_dn0 = assign94330_body51_e145426_d_n0;
            locals.var_t0_dn2 = assign94330_body51_e145426_d_n2;
            locals.var_t0_dn4 = assign94330_body51_e145426_d_n4;
            locals.var_t0_dn5 = assign94330_body51_e145426_d_n5;
            locals.var_t0_dn6 = assign94330_body51_e145426_d_n6;
            locals.var_t0_dn7 = assign94330_body51_e145426_d_n7;
            locals.var_t0_dn8 = assign94330_body51_e145426_d_n8;
            locals.var_t0_dn9 = assign94330_body51_e145426_d_n9;
            locals.var_t0_dn10 = assign94330_body51_e145426_d_n10;
            locals.var_t0_dn13 = assign94330_body51_e145426_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94330_body52_e145462, assign94330_body52_e145462_d_n0, assign94330_body52_e145462_d_n2, assign94330_body52_e145462_d_n4, assign94330_body52_e145462_d_n5, assign94330_body52_e145462_d_n6, assign94330_body52_e145462_d_n7, assign94330_body52_e145462_d_n8, assign94330_body52_e145462_d_n9, assign94330_body52_e145462_d_n10, assign94330_body52_e145462_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        let assign94330_body52_e145446: f64 = (locals.var_chi / 2.0);
        let assign94330_body52_e145450: f64 = (locals.var_chi / 3.0);
        let assign94330_body52_e145454: f64 = (locals.var_chi / 4.0);
        let assign94330_body52_e145455: f64 = (1.0 + assign94330_body52_e145454);
        let assign94330_body52_e145456: f64 = (assign94330_body52_e145450 * assign94330_body52_e145455);
        let assign94330_body52_e145457: f64 = (1.0 + assign94330_body52_e145456);
        let assign94330_body52_e145458: f64 = (assign94330_body52_e145446 * assign94330_body52_e145457);
        let assign94330_body52_e145459: f64 = (1.0 + assign94330_body52_e145458);
        let assign94330_body52_e145460: f64 = (locals.var_chi * assign94330_body52_e145459);
        (assign94330_body52_e145460, ((locals.var_chi_dn0 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn0 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn2 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn4 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn5 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn6 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn7 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn8 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn9 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn10 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign94330_body52_e145459) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign94330_body52_e145457) + (assign94330_body52_e145446 * (((locals.var_chi_dn13 / 3.0) * assign94330_body52_e145455) + (assign94330_body52_e145450 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94330_body52_e145462;
            locals.var_t1_dn0 = assign94330_body52_e145462_d_n0;
            locals.var_t1_dn2 = assign94330_body52_e145462_d_n2;
            locals.var_t1_dn4 = assign94330_body52_e145462_d_n4;
            locals.var_t1_dn5 = assign94330_body52_e145462_d_n5;
            locals.var_t1_dn6 = assign94330_body52_e145462_d_n6;
            locals.var_t1_dn7 = assign94330_body52_e145462_d_n7;
            locals.var_t1_dn8 = assign94330_body52_e145462_d_n8;
            locals.var_t1_dn9 = assign94330_body52_e145462_d_n9;
            locals.var_t1_dn10 = assign94330_body52_e145462_d_n10;
            locals.var_t1_dn13 = assign94330_body52_e145462_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94330_body53_e145482, assign94330_body53_e145482_d_n0, assign94330_body53_e145482_d_n2, assign94330_body53_e145482_d_n4, assign94330_body53_e145482_d_n5, assign94330_body53_e145482_d_n6, assign94330_body53_e145482_d_n7, assign94330_body53_e145482_d_n8, assign94330_body53_e145482_d_n9, assign94330_body53_e145482_d_n10, assign94330_body53_e145482_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        let assign94330_body53_e145480: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign94330_body53_e145480, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94330_body53_e145482;
            locals.var_fs01_dn0 = assign94330_body53_e145482_d_n0;
            locals.var_fs01_dn2 = assign94330_body53_e145482_d_n2;
            locals.var_fs01_dn4 = assign94330_body53_e145482_d_n4;
            locals.var_fs01_dn5 = assign94330_body53_e145482_d_n5;
            locals.var_fs01_dn6 = assign94330_body53_e145482_d_n6;
            locals.var_fs01_dn7 = assign94330_body53_e145482_d_n7;
            locals.var_fs01_dn8 = assign94330_body53_e145482_d_n8;
            locals.var_fs01_dn9 = assign94330_body53_e145482_d_n9;
            locals.var_fs01_dn10 = assign94330_body53_e145482_d_n10;
            locals.var_fs01_dn13 = assign94330_body53_e145482_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign94330_body54_e145504, assign94330_body54_e145504_d_n0, assign94330_body54_e145504_d_n2, assign94330_body54_e145504_d_n4, assign94330_body54_e145504_d_n5, assign94330_body54_e145504_d_n6, assign94330_body54_e145504_d_n7, assign94330_body54_e145504_d_n8, assign94330_body54_e145504_d_n9, assign94330_body54_e145504_d_n10, assign94330_body54_e145504_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 != 0.0)) {
        let assign94330_body54_e145500: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign94330_body54_e145502: f64 = (assign94330_body54_e145500 * locals.var_beta);
        (assign94330_body54_e145502, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign94330_body54_e145500 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94330_body54_e145504;
            locals.var_fs01_dps0_dn0 = assign94330_body54_e145504_d_n0;
            locals.var_fs01_dps0_dn2 = assign94330_body54_e145504_d_n2;
            locals.var_fs01_dps0_dn4 = assign94330_body54_e145504_d_n4;
            locals.var_fs01_dps0_dn5 = assign94330_body54_e145504_d_n5;
            locals.var_fs01_dps0_dn6 = assign94330_body54_e145504_d_n6;
            locals.var_fs01_dps0_dn7 = assign94330_body54_e145504_d_n7;
            locals.var_fs01_dps0_dn8 = assign94330_body54_e145504_d_n8;
            locals.var_fs01_dps0_dn9 = assign94330_body54_e145504_d_n9;
            locals.var_fs01_dps0_dn10 = assign94330_body54_e145504_d_n10;
            locals.var_fs01_dps0_dn13 = assign94330_body54_e145504_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94330_body55_e145524, assign94330_body55_e145524_d_n0, assign94330_body55_e145524_d_n2, assign94330_body55_e145524_d_n4, assign94330_body55_e145524_d_n5, assign94330_body55_e145524_d_n6, assign94330_body55_e145524_d_n7, assign94330_body55_e145524_d_n8, assign94330_body55_e145524_d_n9, assign94330_body55_e145524_d_n10, assign94330_body55_e145524_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 == 0.0)) {
        let assign94330_body55_e145522: f64 = (locals.var_chi).exp();
        (assign94330_body55_e145522, (assign94330_body55_e145522 * locals.var_chi_dn0), (assign94330_body55_e145522 * locals.var_chi_dn2), (assign94330_body55_e145522 * locals.var_chi_dn4), (assign94330_body55_e145522 * locals.var_chi_dn5), (assign94330_body55_e145522 * locals.var_chi_dn6), (assign94330_body55_e145522 * locals.var_chi_dn7), (assign94330_body55_e145522 * locals.var_chi_dn8), (assign94330_body55_e145522 * locals.var_chi_dn9), (assign94330_body55_e145522 * locals.var_chi_dn10), (assign94330_body55_e145522 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign94330_body55_e145524;
            locals.var_exp_chi_dn0 = assign94330_body55_e145524_d_n0;
            locals.var_exp_chi_dn2 = assign94330_body55_e145524_d_n2;
            locals.var_exp_chi_dn4 = assign94330_body55_e145524_d_n4;
            locals.var_exp_chi_dn5 = assign94330_body55_e145524_d_n5;
            locals.var_exp_chi_dn6 = assign94330_body55_e145524_d_n6;
            locals.var_exp_chi_dn7 = assign94330_body55_e145524_d_n7;
            locals.var_exp_chi_dn8 = assign94330_body55_e145524_d_n8;
            locals.var_exp_chi_dn9 = assign94330_body55_e145524_d_n9;
            locals.var_exp_chi_dn10 = assign94330_body55_e145524_d_n10;
            locals.var_exp_chi_dn13 = assign94330_body55_e145524_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign94330_body56_e145545, assign94330_body56_e145545_d_n0, assign94330_body56_e145545_d_n2, assign94330_body56_e145545_d_n4, assign94330_body56_e145545_d_n5, assign94330_body56_e145545_d_n6, assign94330_body56_e145545_d_n7, assign94330_body56_e145545_d_n8, assign94330_body56_e145545_d_n9, assign94330_body56_e145545_d_n10, assign94330_body56_e145545_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 == 0.0)) {
        let assign94330_body56_e145543: f64 = (locals.var_exp_chi - 1.0);
        (assign94330_body56_e145543, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94330_body56_e145545;
            locals.var_t1_dn0 = assign94330_body56_e145545_d_n0;
            locals.var_t1_dn2 = assign94330_body56_e145545_d_n2;
            locals.var_t1_dn4 = assign94330_body56_e145545_d_n4;
            locals.var_t1_dn5 = assign94330_body56_e145545_d_n5;
            locals.var_t1_dn6 = assign94330_body56_e145545_d_n6;
            locals.var_t1_dn7 = assign94330_body56_e145545_d_n7;
            locals.var_t1_dn8 = assign94330_body56_e145545_d_n8;
            locals.var_t1_dn9 = assign94330_body56_e145545_d_n9;
            locals.var_t1_dn10 = assign94330_body56_e145545_d_n10;
            locals.var_t1_dn13 = assign94330_body56_e145545_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94330_body57_e145568, assign94330_body57_e145568_d_n0, assign94330_body57_e145568_d_n2, assign94330_body57_e145568_d_n4, assign94330_body57_e145568_d_n5, assign94330_body57_e145568_d_n6, assign94330_body57_e145568_d_n7, assign94330_body57_e145568_d_n8, assign94330_body57_e145568_d_n9, assign94330_body57_e145568_d_n10, assign94330_body57_e145568_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 == 0.0)) {
        let assign94330_body57_e145565: f64 = (locals.var_t1 - locals.var_chi);
        let assign94330_body57_e145566: f64 = (locals.var_cfs1 * assign94330_body57_e145565);
        (assign94330_body57_e145566, ((locals.var_cfs1_dn0 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign94330_body57_e145565) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94330_body57_e145568;
            locals.var_fs01_dn0 = assign94330_body57_e145568_d_n0;
            locals.var_fs01_dn2 = assign94330_body57_e145568_d_n2;
            locals.var_fs01_dn4 = assign94330_body57_e145568_d_n4;
            locals.var_fs01_dn5 = assign94330_body57_e145568_d_n5;
            locals.var_fs01_dn6 = assign94330_body57_e145568_d_n6;
            locals.var_fs01_dn7 = assign94330_body57_e145568_d_n7;
            locals.var_fs01_dn8 = assign94330_body57_e145568_d_n8;
            locals.var_fs01_dn9 = assign94330_body57_e145568_d_n9;
            locals.var_fs01_dn10 = assign94330_body57_e145568_d_n10;
            locals.var_fs01_dn13 = assign94330_body57_e145568_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign94330_body58_e145591, assign94330_body58_e145591_d_n0, assign94330_body58_e145591_d_n2, assign94330_body58_e145591_d_n4, assign94330_body58_e145591_d_n5, assign94330_body58_e145591_d_n6, assign94330_body58_e145591_d_n7, assign94330_body58_e145591_d_n8, assign94330_body58_e145591_d_n9, assign94330_body58_e145591_d_n10, assign94330_body58_e145591_d_n13,) = {
    if (((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 != 0.0)) && (locals.var_guard2190 == 0.0)) {
        let assign94330_body58_e145587: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign94330_body58_e145589: f64 = (assign94330_body58_e145587 * locals.var_t1);
        (assign94330_body58_e145589, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign94330_body58_e145587 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94330_body58_e145591;
            locals.var_fs01_dps0_dn0 = assign94330_body58_e145591_d_n0;
            locals.var_fs01_dps0_dn2 = assign94330_body58_e145591_d_n2;
            locals.var_fs01_dps0_dn4 = assign94330_body58_e145591_d_n4;
            locals.var_fs01_dps0_dn5 = assign94330_body58_e145591_d_n5;
            locals.var_fs01_dps0_dn6 = assign94330_body58_e145591_d_n6;
            locals.var_fs01_dps0_dn7 = assign94330_body58_e145591_d_n7;
            locals.var_fs01_dps0_dn8 = assign94330_body58_e145591_d_n8;
            locals.var_fs01_dps0_dn9 = assign94330_body58_e145591_d_n9;
            locals.var_fs01_dps0_dn10 = assign94330_body58_e145591_d_n10;
            locals.var_fs01_dps0_dn13 = assign94330_body58_e145591_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94330_body60_e145630, assign94330_body60_e145630_d_n0, assign94330_body60_e145630_d_n2, assign94330_body60_e145630_d_n4, assign94330_body60_e145630_d_n5, assign94330_body60_e145630_d_n6, assign94330_body60_e145630_d_n7, assign94330_body60_e145630_d_n8, assign94330_body60_e145630_d_n9, assign94330_body60_e145630_d_n10, assign94330_body60_e145630_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) {
        let assign94330_body60_e145627: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign94330_body60_e145628: f64 = (assign94330_body60_e145627).exp();
        (assign94330_body60_e145628, (assign94330_body60_e145628 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign94330_body60_e145628 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign94330_body60_e145628 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign94330_body60_e145628 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign94330_body60_e145628 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign94330_body60_e145628 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign94330_body60_e145628 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign94330_body60_e145628 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign94330_body60_e145628 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign94330_body60_e145628 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign94330_body60_e145630;
            locals.var_exp_bps0_dn0 = assign94330_body60_e145630_d_n0;
            locals.var_exp_bps0_dn2 = assign94330_body60_e145630_d_n2;
            locals.var_exp_bps0_dn4 = assign94330_body60_e145630_d_n4;
            locals.var_exp_bps0_dn5 = assign94330_body60_e145630_d_n5;
            locals.var_exp_bps0_dn6 = assign94330_body60_e145630_d_n6;
            locals.var_exp_bps0_dn7 = assign94330_body60_e145630_d_n7;
            locals.var_exp_bps0_dn8 = assign94330_body60_e145630_d_n8;
            locals.var_exp_bps0_dn9 = assign94330_body60_e145630_d_n9;
            locals.var_exp_bps0_dn10 = assign94330_body60_e145630_d_n10;
            locals.var_exp_bps0_dn13 = assign94330_body60_e145630_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign94330_body61_e145655, assign94330_body61_e145655_d_n0, assign94330_body61_e145655_d_n2, assign94330_body61_e145655_d_n4, assign94330_body61_e145655_d_n5, assign94330_body61_e145655_d_n6, assign94330_body61_e145655_d_n7, assign94330_body61_e145655_d_n8, assign94330_body61_e145655_d_n9, assign94330_body61_e145655_d_n10, assign94330_body61_e145655_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) {
        let assign94330_body61_e145650: f64 = (locals.var_chi + 1.0);
        let assign94330_body61_e145651: f64 = (locals.var_exp_bvbs * assign94330_body61_e145650);
        let assign94330_body61_e145652: f64 = (locals.var_exp_bps0 - assign94330_body61_e145651);
        let assign94330_body61_e145653: f64 = (locals.var_cnst1over * assign94330_body61_e145652);
        (assign94330_body61_e145653, ((locals.var_cnst1over_dn0 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign94330_body61_e145652) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign94330_body61_e145650) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94330_body61_e145655;
            locals.var_fs01_dn0 = assign94330_body61_e145655_d_n0;
            locals.var_fs01_dn2 = assign94330_body61_e145655_d_n2;
            locals.var_fs01_dn4 = assign94330_body61_e145655_d_n4;
            locals.var_fs01_dn5 = assign94330_body61_e145655_d_n5;
            locals.var_fs01_dn6 = assign94330_body61_e145655_d_n6;
            locals.var_fs01_dn7 = assign94330_body61_e145655_d_n7;
            locals.var_fs01_dn8 = assign94330_body61_e145655_d_n8;
            locals.var_fs01_dn9 = assign94330_body61_e145655_d_n9;
            locals.var_fs01_dn10 = assign94330_body61_e145655_d_n10;
            locals.var_fs01_dn13 = assign94330_body61_e145655_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign94330_body62_e145678, assign94330_body62_e145678_d_n0, assign94330_body62_e145678_d_n2, assign94330_body62_e145678_d_n4, assign94330_body62_e145678_d_n5, assign94330_body62_e145678_d_n6, assign94330_body62_e145678_d_n7, assign94330_body62_e145678_d_n8, assign94330_body62_e145678_d_n9, assign94330_body62_e145678_d_n10, assign94330_body62_e145678_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2189 == 0.0)) {
        let assign94330_body62_e145672: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign94330_body62_e145675: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign94330_body62_e145676: f64 = (assign94330_body62_e145672 * assign94330_body62_e145675);
        (assign94330_body62_e145676, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign94330_body62_e145675) + (assign94330_body62_e145672 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94330_body62_e145678;
            locals.var_fs01_dps0_dn0 = assign94330_body62_e145678_d_n0;
            locals.var_fs01_dps0_dn2 = assign94330_body62_e145678_d_n2;
            locals.var_fs01_dps0_dn4 = assign94330_body62_e145678_d_n4;
            locals.var_fs01_dps0_dn5 = assign94330_body62_e145678_d_n5;
            locals.var_fs01_dps0_dn6 = assign94330_body62_e145678_d_n6;
            locals.var_fs01_dps0_dn7 = assign94330_body62_e145678_d_n7;
            locals.var_fs01_dps0_dn8 = assign94330_body62_e145678_d_n8;
            locals.var_fs01_dps0_dn9 = assign94330_body62_e145678_d_n9;
            locals.var_fs01_dps0_dn10 = assign94330_body62_e145678_d_n10;
            locals.var_fs01_dps0_dn13 = assign94330_body62_e145678_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94330_body63_e145681: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2191 = assign94330_body63_e145681;
            locals.var_guard2191_rv = 0.0;
            let (assign94330_body64_e145702, assign94330_body64_e145702_d_n0, assign94330_body64_e145702_d_n2, assign94330_body64_e145702_d_n4, assign94330_body64_e145702_d_n5, assign94330_body64_e145702_d_n6, assign94330_body64_e145702_d_n7, assign94330_body64_e145702_d_n8, assign94330_body64_e145702_d_n9, assign94330_body64_e145702_d_n10, assign94330_body64_e145702_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2191 != 0.0)) {
        let assign94330_body64_e145697: f64 = (locals.var_fb * locals.var_fb);
        let assign94330_body64_e145699: f64 = (assign94330_body64_e145697 + locals.var_fs01);
        let assign94330_body64_e145700: f64 = (assign94330_body64_e145699).sqrt();
        (assign94330_body64_e145700, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign94330_body64_e145700)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign94330_body64_e145700)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94330_body64_e145702;
            locals.var_fs02_dn0 = assign94330_body64_e145702_d_n0;
            locals.var_fs02_dn2 = assign94330_body64_e145702_d_n2;
            locals.var_fs02_dn4 = assign94330_body64_e145702_d_n4;
            locals.var_fs02_dn5 = assign94330_body64_e145702_d_n5;
            locals.var_fs02_dn6 = assign94330_body64_e145702_d_n6;
            locals.var_fs02_dn7 = assign94330_body64_e145702_d_n7;
            locals.var_fs02_dn8 = assign94330_body64_e145702_d_n8;
            locals.var_fs02_dn9 = assign94330_body64_e145702_d_n9;
            locals.var_fs02_dn10 = assign94330_body64_e145702_d_n10;
            locals.var_fs02_dn13 = assign94330_body64_e145702_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign94330_body65_e145728, assign94330_body65_e145728_d_n0, assign94330_body65_e145728_d_n2, assign94330_body65_e145728_d_n4, assign94330_body65_e145728_d_n5, assign94330_body65_e145728_d_n6, assign94330_body65_e145728_d_n7, assign94330_body65_e145728_d_n8, assign94330_body65_e145728_d_n9, assign94330_body65_e145728_d_n10, assign94330_body65_e145728_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2191 != 0.0)) {
        let assign94330_body65_e145719: f64 = (2.0 * locals.var_fb_dpss);
        let assign94330_body65_e145721: f64 = (assign94330_body65_e145719 * locals.var_fb);
        let assign94330_body65_e145723: f64 = (assign94330_body65_e145721 + locals.var_fs01_dps0);
        let assign94330_body65_e145724: f64 = (0.5 * assign94330_body65_e145723);
        let assign94330_body65_e145726: f64 = (assign94330_body65_e145724 / locals.var_fs02);
        (assign94330_body65_e145726, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn13) * locals.var_fb) + (assign94330_body65_e145719 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign94330_body65_e145724 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94330_body65_e145728;
            locals.var_fs02_dps0_dn0 = assign94330_body65_e145728_d_n0;
            locals.var_fs02_dps0_dn2 = assign94330_body65_e145728_d_n2;
            locals.var_fs02_dps0_dn4 = assign94330_body65_e145728_d_n4;
            locals.var_fs02_dps0_dn5 = assign94330_body65_e145728_d_n5;
            locals.var_fs02_dps0_dn6 = assign94330_body65_e145728_d_n6;
            locals.var_fs02_dps0_dn7 = assign94330_body65_e145728_d_n7;
            locals.var_fs02_dps0_dn8 = assign94330_body65_e145728_d_n8;
            locals.var_fs02_dps0_dn9 = assign94330_body65_e145728_d_n9;
            locals.var_fs02_dps0_dn10 = assign94330_body65_e145728_d_n10;
            locals.var_fs02_dps0_dn13 = assign94330_body65_e145728_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94330_body67_e145764, assign94330_body67_e145764_d_n0, assign94330_body67_e145764_d_n2, assign94330_body67_e145764_d_n4, assign94330_body67_e145764_d_n5, assign94330_body67_e145764_d_n6, assign94330_body67_e145764_d_n7, assign94330_body67_e145764_d_n8, assign94330_body67_e145764_d_n9, assign94330_body67_e145764_d_n10, assign94330_body67_e145764_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2191 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94330_body67_e145764;
            locals.var_fs02_dn0 = assign94330_body67_e145764_d_n0;
            locals.var_fs02_dn2 = assign94330_body67_e145764_d_n2;
            locals.var_fs02_dn4 = assign94330_body67_e145764_d_n4;
            locals.var_fs02_dn5 = assign94330_body67_e145764_d_n5;
            locals.var_fs02_dn6 = assign94330_body67_e145764_d_n6;
            locals.var_fs02_dn7 = assign94330_body67_e145764_d_n7;
            locals.var_fs02_dn8 = assign94330_body67_e145764_d_n8;
            locals.var_fs02_dn9 = assign94330_body67_e145764_d_n9;
            locals.var_fs02_dn10 = assign94330_body67_e145764_d_n10;
            locals.var_fs02_dn13 = assign94330_body67_e145764_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign94330_body68_e145781, assign94330_body68_e145781_d_n0, assign94330_body68_e145781_d_n2, assign94330_body68_e145781_d_n4, assign94330_body68_e145781_d_n5, assign94330_body68_e145781_d_n6, assign94330_body68_e145781_d_n7, assign94330_body68_e145781_d_n8, assign94330_body68_e145781_d_n9, assign94330_body68_e145781_d_n10, assign94330_body68_e145781_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2188 == 0.0)) && (locals.var_guard2191 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94330_body68_e145781;
            locals.var_fs02_dps0_dn0 = assign94330_body68_e145781_d_n0;
            locals.var_fs02_dps0_dn2 = assign94330_body68_e145781_d_n2;
            locals.var_fs02_dps0_dn4 = assign94330_body68_e145781_d_n4;
            locals.var_fs02_dps0_dn5 = assign94330_body68_e145781_d_n5;
            locals.var_fs02_dps0_dn6 = assign94330_body68_e145781_d_n6;
            locals.var_fs02_dps0_dn7 = assign94330_body68_e145781_d_n7;
            locals.var_fs02_dps0_dn8 = assign94330_body68_e145781_d_n8;
            locals.var_fs02_dps0_dn9 = assign94330_body68_e145781_d_n9;
            locals.var_fs02_dps0_dn10 = assign94330_body68_e145781_d_n10;
            locals.var_fs02_dps0_dn13 = assign94330_body68_e145781_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94330_body69_e145799, assign94330_body69_e145799_d_n0, assign94330_body69_e145799_d_n2, assign94330_body69_e145799_d_n4, assign94330_body69_e145799_d_n5, assign94330_body69_e145799_d_n6, assign94330_body69_e145799_d_n7, assign94330_body69_e145799_d_n8, assign94330_body69_e145799_d_n9, assign94330_body69_e145799_d_n10, assign94330_body69_e145799_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94330_body69_e145791: f64 = (-locals.var_vgpld);
        let assign94330_body69_e145793: f64 = (assign94330_body69_e145791 + locals.var_ps0ld);
        let assign94330_body69_e145796: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign94330_body69_e145797: f64 = (assign94330_body69_e145793 + assign94330_body69_e145796);
        (assign94330_body69_e145797, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign94330_body69_e145799;
            locals.var_fs0_dn0 = assign94330_body69_e145799_d_n0;
            locals.var_fs0_dn2 = assign94330_body69_e145799_d_n2;
            locals.var_fs0_dn4 = assign94330_body69_e145799_d_n4;
            locals.var_fs0_dn5 = assign94330_body69_e145799_d_n5;
            locals.var_fs0_dn6 = assign94330_body69_e145799_d_n6;
            locals.var_fs0_dn7 = assign94330_body69_e145799_d_n7;
            locals.var_fs0_dn8 = assign94330_body69_e145799_d_n8;
            locals.var_fs0_dn9 = assign94330_body69_e145799_d_n9;
            locals.var_fs0_dn10 = assign94330_body69_e145799_d_n10;
            locals.var_fs0_dn13 = assign94330_body69_e145799_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign94330_body70_e145814, assign94330_body70_e145814_d_n0, assign94330_body70_e145814_d_n2, assign94330_body70_e145814_d_n4, assign94330_body70_e145814_d_n5, assign94330_body70_e145814_d_n6, assign94330_body70_e145814_d_n7, assign94330_body70_e145814_d_n8, assign94330_body70_e145814_d_n9, assign94330_body70_e145814_d_n10, assign94330_body70_e145814_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94330_body70_e145811: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign94330_body70_e145812: f64 = (1.0 + assign94330_body70_e145811);
        (assign94330_body70_e145812, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign94330_body70_e145814;
            locals.var_fs0_dps0_dn0 = assign94330_body70_e145814_d_n0;
            locals.var_fs0_dps0_dn2 = assign94330_body70_e145814_d_n2;
            locals.var_fs0_dps0_dn4 = assign94330_body70_e145814_d_n4;
            locals.var_fs0_dps0_dn5 = assign94330_body70_e145814_d_n5;
            locals.var_fs0_dps0_dn6 = assign94330_body70_e145814_d_n6;
            locals.var_fs0_dps0_dn7 = assign94330_body70_e145814_d_n7;
            locals.var_fs0_dps0_dn8 = assign94330_body70_e145814_d_n8;
            locals.var_fs0_dps0_dn9 = assign94330_body70_e145814_d_n9;
            locals.var_fs0_dps0_dn10 = assign94330_body70_e145814_d_n10;
            locals.var_fs0_dps0_dn13 = assign94330_body70_e145814_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign94330_body71_e145817: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard2192 = assign94330_body71_e145817;
            locals.var_guard2192_rv = 0.0;
            let (assign94330_body72_e145832,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2192 != 0.0)) {
        let assign94330_body72_e145830: f64 = (locals.var_lp_s0_max + 1.0);
        (assign94330_body72_e145830,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94330_body72_e145832;
            locals.var_lp_s0_rv = 0.0;
            let (assign94330_body73_e145849, assign94330_body73_e145849_d_n0, assign94330_body73_e145849_d_n2, assign94330_body73_e145849_d_n4, assign94330_body73_e145849_d_n5, assign94330_body73_e145849_d_n6, assign94330_body73_e145849_d_n7, assign94330_body73_e145849_d_n8, assign94330_body73_e145849_d_n9, assign94330_body73_e145849_d_n10, assign94330_body73_e145849_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2192 == 0.0)) {
        let assign94330_body73_e145845: f64 = (-locals.var_fs0);
        let assign94330_body73_e145847: f64 = (assign94330_body73_e145845 / locals.var_fs0_dps0);
        (assign94330_body73_e145847, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign94330_body73_e145845 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign94330_body73_e145849;
            locals.var_dps0_dn0 = assign94330_body73_e145849_d_n0;
            locals.var_dps0_dn2 = assign94330_body73_e145849_d_n2;
            locals.var_dps0_dn4 = assign94330_body73_e145849_d_n4;
            locals.var_dps0_dn5 = assign94330_body73_e145849_d_n5;
            locals.var_dps0_dn6 = assign94330_body73_e145849_d_n6;
            locals.var_dps0_dn7 = assign94330_body73_e145849_d_n7;
            locals.var_dps0_dn8 = assign94330_body73_e145849_d_n8;
            locals.var_dps0_dn9 = assign94330_body73_e145849_d_n9;
            locals.var_dps0_dn10 = assign94330_body73_e145849_d_n10;
            locals.var_dps0_dn13 = assign94330_body73_e145849_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign94330_body74_e145876, assign94330_body74_e145876_d_n0, assign94330_body74_e145876_d_n2, assign94330_body74_e145876_d_n4, assign94330_body74_e145876_d_n5, assign94330_body74_e145876_d_n6, assign94330_body74_e145876_d_n7, assign94330_body74_e145876_d_n8, assign94330_body74_e145876_d_n9, assign94330_body74_e145876_d_n10, assign94330_body74_e145876_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2192 == 0.0)) {
        let assign94330_body74_e145863: f64 = (0.5 * 0.1);
        let assign94330_body74_e145867: f64 = (locals.var_ps0ld).abs();
        let (assign94330_body74_e145872, assign94330_body74_e145872_d_n0, assign94330_body74_e145872_d_n2, assign94330_body74_e145872_d_n4, assign94330_body74_e145872_d_n5, assign94330_body74_e145872_d_n6, assign94330_body74_e145872_d_n7, assign94330_body74_e145872_d_n8, assign94330_body74_e145872_d_n9, assign94330_body74_e145872_d_n10, assign94330_body74_e145872_d_n13,) = {
            if (1.0 >= assign94330_body74_e145867) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign94330_body74_e145871: f64 = (locals.var_ps0ld).abs();
                (assign94330_body74_e145871, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign94330_body74_e145873: f64 = (1.0 + assign94330_body74_e145872);
        let assign94330_body74_e145874: f64 = (assign94330_body74_e145863 * assign94330_body74_e145873);
        (assign94330_body74_e145874, (assign94330_body74_e145863 * assign94330_body74_e145872_d_n0), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n2), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n4), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n5), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n6), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n7), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n8), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n9), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n10), (assign94330_body74_e145863 * assign94330_body74_e145872_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign94330_body74_e145876;
            locals.var_dplim_dn0 = assign94330_body74_e145876_d_n0;
            locals.var_dplim_dn2 = assign94330_body74_e145876_d_n2;
            locals.var_dplim_dn4 = assign94330_body74_e145876_d_n4;
            locals.var_dplim_dn5 = assign94330_body74_e145876_d_n5;
            locals.var_dplim_dn6 = assign94330_body74_e145876_d_n6;
            locals.var_dplim_dn7 = assign94330_body74_e145876_d_n7;
            locals.var_dplim_dn8 = assign94330_body74_e145876_d_n8;
            locals.var_dplim_dn9 = assign94330_body74_e145876_d_n9;
            locals.var_dplim_dn10 = assign94330_body74_e145876_d_n10;
            locals.var_dplim_dn13 = assign94330_body74_e145876_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign94330_body75_e145878: f64 = (locals.var_dps0).abs();
            let assign94330_body75_e145880: f64 = if assign94330_body75_e145878 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2193 = assign94330_body75_e145880;
            locals.var_guard2193_rv = 0.0;
            let (assign94330_body76_e145904, assign94330_body76_e145904_d_n0, assign94330_body76_e145904_d_n2, assign94330_body76_e145904_d_n4, assign94330_body76_e145904_d_n5, assign94330_body76_e145904_d_n6, assign94330_body76_e145904_d_n7, assign94330_body76_e145904_d_n8, assign94330_body76_e145904_d_n9, assign94330_body76_e145904_d_n10, assign94330_body76_e145904_d_n13,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2193 != 0.0)) {
        let (assign94330_body76_e145901,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign94330_body76_e145900: f64 = (-1.0);
                (assign94330_body76_e145900,)
            }
        };
        let assign94330_body76_e145902: f64 = (locals.var_dplim * assign94330_body76_e145901);
        (assign94330_body76_e145902, (locals.var_dplim_dn0 * assign94330_body76_e145901), (locals.var_dplim_dn2 * assign94330_body76_e145901), (locals.var_dplim_dn4 * assign94330_body76_e145901), (locals.var_dplim_dn5 * assign94330_body76_e145901), (locals.var_dplim_dn6 * assign94330_body76_e145901), (locals.var_dplim_dn7 * assign94330_body76_e145901), (locals.var_dplim_dn8 * assign94330_body76_e145901), (locals.var_dplim_dn9 * assign94330_body76_e145901), (locals.var_dplim_dn10 * assign94330_body76_e145901), (locals.var_dplim_dn13 * assign94330_body76_e145901),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign94330_body76_e145904;
            locals.var_dps0_dn0 = assign94330_body76_e145904_d_n0;
            locals.var_dps0_dn2 = assign94330_body76_e145904_d_n2;
            locals.var_dps0_dn4 = assign94330_body76_e145904_d_n4;
            locals.var_dps0_dn5 = assign94330_body76_e145904_d_n5;
            locals.var_dps0_dn6 = assign94330_body76_e145904_d_n6;
            locals.var_dps0_dn7 = assign94330_body76_e145904_d_n7;
            locals.var_dps0_dn8 = assign94330_body76_e145904_d_n8;
            locals.var_dps0_dn9 = assign94330_body76_e145904_d_n9;
            locals.var_dps0_dn10 = assign94330_body76_e145904_d_n10;
            locals.var_dps0_dn13 = assign94330_body76_e145904_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign94330_body77_e145920, assign94330_body77_e145920_d_n0, assign94330_body77_e145920_d_n2, assign94330_body77_e145920_d_n4, assign94330_body77_e145920_d_n5, assign94330_body77_e145920_d_n6, assign94330_body77_e145920_d_n7, assign94330_body77_e145920_d_n8, assign94330_body77_e145920_d_n9, assign94330_body77_e145920_d_n10, assign94330_body77_e145920_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2192 == 0.0)) {
        let assign94330_body77_e145918: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign94330_body77_e145918, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign94330_body77_e145920;
            locals.var_ps0ld_dn0 = assign94330_body77_e145920_d_n0;
            locals.var_ps0ld_dn2 = assign94330_body77_e145920_d_n2;
            locals.var_ps0ld_dn4 = assign94330_body77_e145920_d_n4;
            locals.var_ps0ld_dn5 = assign94330_body77_e145920_d_n5;
            locals.var_ps0ld_dn6 = assign94330_body77_e145920_d_n6;
            locals.var_ps0ld_dn7 = assign94330_body77_e145920_d_n7;
            locals.var_ps0ld_dn8 = assign94330_body77_e145920_d_n8;
            locals.var_ps0ld_dn9 = assign94330_body77_e145920_d_n9;
            locals.var_ps0ld_dn10 = assign94330_body77_e145920_d_n10;
            locals.var_ps0ld_dn13 = assign94330_body77_e145920_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign94330_body78_e145922: f64 = (locals.var_dps0).abs();
            let assign94330_body78_e145926: f64 = (locals.var_fs0).abs();
            let assign94330_body78_e145929: f64 = if ((assign94330_body78_e145922 <= 1e-12) && (assign94330_body78_e145926 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2194 = assign94330_body78_e145929;
            locals.var_guard2194_rv = 0.0;
            let (assign94330_body79_e145945,) = {
    if ((((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) && (locals.var_guard2192 == 0.0)) && (locals.var_guard2194 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign94330_body79_e145945;
            locals.var_flg_conv_rv = 0.0;
            let (assign94330_body80_e145958,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94330_body80_e145956: f64 = (locals.var_lp_s0 + 1.0);
        (assign94330_body80_e145956,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94330_body80_e145958;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_353(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94350_e145974, assign94350_e145974_d_n0, assign94350_e145974_d_n2, assign94350_e145974_d_n4, assign94350_e145974_d_n5, assign94350_e145974_d_n6, assign94350_e145974_d_n7, assign94350_e145974_d_n8, assign94350_e145974_d_n9, assign94350_e145974_d_n10, assign94350_e145974_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94350_e145972: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign94350_e145972, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk2113, locals.var_wdld__blk2113_dn0, locals.var_wdld__blk2113_dn2, locals.var_wdld__blk2113_dn4, locals.var_wdld__blk2113_dn5, locals.var_wdld__blk2113_dn6, locals.var_wdld__blk2113_dn7, locals.var_wdld__blk2113_dn8, locals.var_wdld__blk2113_dn9, locals.var_wdld__blk2113_dn10, locals.var_wdld__blk2113_dn13,)
    }
};
        locals.var_wdld__blk2113 = assign94350_e145974;
        locals.var_wdld__blk2113_dn0 = assign94350_e145974_d_n0;
        locals.var_wdld__blk2113_dn2 = assign94350_e145974_d_n2;
        locals.var_wdld__blk2113_dn4 = assign94350_e145974_d_n4;
        locals.var_wdld__blk2113_dn5 = assign94350_e145974_d_n5;
        locals.var_wdld__blk2113_dn6 = assign94350_e145974_d_n6;
        locals.var_wdld__blk2113_dn7 = assign94350_e145974_d_n7;
        locals.var_wdld__blk2113_dn8 = assign94350_e145974_d_n8;
        locals.var_wdld__blk2113_dn9 = assign94350_e145974_d_n9;
        locals.var_wdld__blk2113_dn10 = assign94350_e145974_d_n10;
        locals.var_wdld__blk2113_dn13 = assign94350_e145974_d_n13;
        locals.var_wdld__blk2113_rv = 0.0;

        let (assign94360_e145987, assign94360_e145987_d_n0, assign94360_e145987_d_n2, assign94360_e145987_d_n4, assign94360_e145987_d_n5, assign94360_e145987_d_n6, assign94360_e145987_d_n7, assign94360_e145987_d_n8, assign94360_e145987_d_n9, assign94360_e145987_d_n10, assign94360_e145987_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94360_e145985: f64 = (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113);
        (assign94360_e145985, (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn0), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn2), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn4), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn5), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn6), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn7), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn8), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn9), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn10), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn13),)
    } else {
        (locals.var_q_dep_ld__blk2114, locals.var_q_dep_ld__blk2114_dn0, locals.var_q_dep_ld__blk2114_dn2, locals.var_q_dep_ld__blk2114_dn4, locals.var_q_dep_ld__blk2114_dn5, locals.var_q_dep_ld__blk2114_dn6, locals.var_q_dep_ld__blk2114_dn7, locals.var_q_dep_ld__blk2114_dn8, locals.var_q_dep_ld__blk2114_dn9, locals.var_q_dep_ld__blk2114_dn10, locals.var_q_dep_ld__blk2114_dn13,)
    }
};
        locals.var_q_dep_ld__blk2114 = assign94360_e145987;
        locals.var_q_dep_ld__blk2114_dn0 = assign94360_e145987_d_n0;
        locals.var_q_dep_ld__blk2114_dn2 = assign94360_e145987_d_n2;
        locals.var_q_dep_ld__blk2114_dn4 = assign94360_e145987_d_n4;
        locals.var_q_dep_ld__blk2114_dn5 = assign94360_e145987_d_n5;
        locals.var_q_dep_ld__blk2114_dn6 = assign94360_e145987_d_n6;
        locals.var_q_dep_ld__blk2114_dn7 = assign94360_e145987_d_n7;
        locals.var_q_dep_ld__blk2114_dn8 = assign94360_e145987_d_n8;
        locals.var_q_dep_ld__blk2114_dn9 = assign94360_e145987_d_n9;
        locals.var_q_dep_ld__blk2114_dn10 = assign94360_e145987_d_n10;
        locals.var_q_dep_ld__blk2114_dn13 = assign94360_e145987_d_n13;
        locals.var_q_dep_ld__blk2114_rv = 0.0;

        let (assign94370_e146004, assign94370_e146004_d_n0, assign94370_e146004_d_n2, assign94370_e146004_d_n4, assign94370_e146004_d_n5, assign94370_e146004_d_n6, assign94370_e146004_d_n7, assign94370_e146004_d_n8, assign94370_e146004_d_n9, assign94370_e146004_d_n10, assign94370_e146004_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94370_e145998: f64 = (locals.var_q_dep_ld__blk2114 / locals.var_cnst0over_func);
        let assign94370_e146001: f64 = (10.0 * 2.220446049250313e-16);
        let assign94370_e146002: f64 = (assign94370_e145998 + assign94370_e146001);
        (assign94370_e146002, (((locals.var_q_dep_ld__blk2114_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign94370_e146004;
        locals.var_xi0p12_dn0 = assign94370_e146004_d_n0;
        locals.var_xi0p12_dn2 = assign94370_e146004_d_n2;
        locals.var_xi0p12_dn4 = assign94370_e146004_d_n4;
        locals.var_xi0p12_dn5 = assign94370_e146004_d_n5;
        locals.var_xi0p12_dn6 = assign94370_e146004_d_n6;
        locals.var_xi0p12_dn7 = assign94370_e146004_d_n7;
        locals.var_xi0p12_dn8 = assign94370_e146004_d_n8;
        locals.var_xi0p12_dn9 = assign94370_e146004_d_n9;
        locals.var_xi0p12_dn10 = assign94370_e146004_d_n10;
        locals.var_xi0p12_dn13 = assign94370_e146004_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign94380_e146017, assign94380_e146017_d_n0, assign94380_e146017_d_n2, assign94380_e146017_d_n4, assign94380_e146017_d_n5, assign94380_e146017_d_n6, assign94380_e146017_d_n7, assign94380_e146017_d_n8, assign94380_e146017_d_n9, assign94380_e146017_d_n10, assign94380_e146017_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94380_e146015: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign94380_e146015, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign94380_e146017;
        locals.var_qbuld_dn0 = assign94380_e146017_d_n0;
        locals.var_qbuld_dn2 = assign94380_e146017_d_n2;
        locals.var_qbuld_dn4 = assign94380_e146017_d_n4;
        locals.var_qbuld_dn5 = assign94380_e146017_d_n5;
        locals.var_qbuld_dn6 = assign94380_e146017_d_n6;
        locals.var_qbuld_dn7 = assign94380_e146017_d_n7;
        locals.var_qbuld_dn8 = assign94380_e146017_d_n8;
        locals.var_qbuld_dn9 = assign94380_e146017_d_n9;
        locals.var_qbuld_dn10 = assign94380_e146017_d_n10;
        locals.var_qbuld_dn13 = assign94380_e146017_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign94390_e146032, assign94390_e146032_d_n0, assign94390_e146032_d_n2, assign94390_e146032_d_n4, assign94390_e146032_d_n5, assign94390_e146032_d_n6, assign94390_e146032_d_n7, assign94390_e146032_d_n8, assign94390_e146032_d_n9, assign94390_e146032_d_n10, assign94390_e146032_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94390_e146029: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign94390_e146030: f64 = (1.0 / assign94390_e146029);
        (assign94390_e146030, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign94390_e146029 * assign94390_e146029))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign94390_e146029 * assign94390_e146029))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign94390_e146032;
        locals.var_t1_dn0 = assign94390_e146032_d_n0;
        locals.var_t1_dn2 = assign94390_e146032_d_n2;
        locals.var_t1_dn4 = assign94390_e146032_d_n4;
        locals.var_t1_dn5 = assign94390_e146032_d_n5;
        locals.var_t1_dn6 = assign94390_e146032_d_n6;
        locals.var_t1_dn7 = assign94390_e146032_d_n7;
        locals.var_t1_dn8 = assign94390_e146032_d_n8;
        locals.var_t1_dn9 = assign94390_e146032_d_n9;
        locals.var_t1_dn10 = assign94390_e146032_d_n10;
        locals.var_t1_dn13 = assign94390_e146032_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign94400_e146047, assign94400_e146047_d_n0, assign94400_e146047_d_n2, assign94400_e146047_d_n4, assign94400_e146047_d_n5, assign94400_e146047_d_n6, assign94400_e146047_d_n7, assign94400_e146047_d_n8, assign94400_e146047_d_n9, assign94400_e146047_d_n10, assign94400_e146047_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94400_e146043: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign94400_e146045: f64 = (assign94400_e146043 * locals.var_t1);
        (assign94400_e146045, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign94400_e146043 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign94400_e146047;
        locals.var_qiuld_dn0 = assign94400_e146047_d_n0;
        locals.var_qiuld_dn2 = assign94400_e146047_d_n2;
        locals.var_qiuld_dn4 = assign94400_e146047_d_n4;
        locals.var_qiuld_dn5 = assign94400_e146047_d_n5;
        locals.var_qiuld_dn6 = assign94400_e146047_d_n6;
        locals.var_qiuld_dn7 = assign94400_e146047_d_n7;
        locals.var_qiuld_dn8 = assign94400_e146047_d_n8;
        locals.var_qiuld_dn9 = assign94400_e146047_d_n9;
        locals.var_qiuld_dn10 = assign94400_e146047_d_n10;
        locals.var_qiuld_dn13 = assign94400_e146047_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign94410_e146060, assign94410_e146060_d_n0, assign94410_e146060_d_n2, assign94410_e146060_d_n4, assign94410_e146060_d_n5, assign94410_e146060_d_n6, assign94410_e146060_d_n7, assign94410_e146060_d_n8, assign94410_e146060_d_n9, assign94410_e146060_d_n10, assign94410_e146060_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2139 == 0.0)) && (locals.var_guard2178 != 0.0)) {
        let assign94410_e146058: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign94410_e146058, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign94410_e146060;
        locals.var_qsuld_dn0 = assign94410_e146060_d_n0;
        locals.var_qsuld_dn2 = assign94410_e146060_d_n2;
        locals.var_qsuld_dn4 = assign94410_e146060_d_n4;
        locals.var_qsuld_dn5 = assign94410_e146060_d_n5;
        locals.var_qsuld_dn6 = assign94410_e146060_d_n6;
        locals.var_qsuld_dn7 = assign94410_e146060_d_n7;
        locals.var_qsuld_dn8 = assign94410_e146060_d_n8;
        locals.var_qsuld_dn9 = assign94410_e146060_d_n9;
        locals.var_qsuld_dn10 = assign94410_e146060_d_n10;
        locals.var_qsuld_dn13 = assign94410_e146060_d_n13;
        locals.var_qsuld_rv = 0.0;

        let assign94420_e146063: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2196 = assign94420_e146063;
        locals.var_guard2196_rv = 0.0;

        let (assign94430_e146075, assign94430_e146075_d_n0, assign94430_e146075_d_n2, assign94430_e146075_d_n4, assign94430_e146075_d_n5, assign94430_e146075_d_n6, assign94430_e146075_d_n7, assign94430_e146075_d_n8, assign94430_e146075_d_n9, assign94430_e146075_d_n10, assign94430_e146075_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94430_e146071: f64 = (-locals.var_vxbgmtcl);
        let assign94430_e146072: f64 = (locals.var_beta * assign94430_e146071);
        let assign94430_e146073: f64 = (assign94430_e146072).exp();
        (assign94430_e146073, (assign94430_e146073 * ((locals.var_beta_dn0 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign94430_e146073 * ((locals.var_beta_dn2 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign94430_e146073 * ((locals.var_beta_dn4 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign94430_e146073 * ((locals.var_beta_dn5 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign94430_e146073 * ((locals.var_beta_dn6 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign94430_e146073 * ((locals.var_beta_dn7 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign94430_e146073 * ((locals.var_beta_dn8 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign94430_e146073 * ((locals.var_beta_dn9 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign94430_e146073 * ((locals.var_beta_dn10 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign94430_e146073 * ((locals.var_beta_dn13 * assign94430_e146071) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign94430_e146075;
        locals.var_exp_bvbs_dn0 = assign94430_e146075_d_n0;
        locals.var_exp_bvbs_dn2 = assign94430_e146075_d_n2;
        locals.var_exp_bvbs_dn4 = assign94430_e146075_d_n4;
        locals.var_exp_bvbs_dn5 = assign94430_e146075_d_n5;
        locals.var_exp_bvbs_dn6 = assign94430_e146075_d_n6;
        locals.var_exp_bvbs_dn7 = assign94430_e146075_d_n7;
        locals.var_exp_bvbs_dn8 = assign94430_e146075_d_n8;
        locals.var_exp_bvbs_dn9 = assign94430_e146075_d_n9;
        locals.var_exp_bvbs_dn10 = assign94430_e146075_d_n10;
        locals.var_exp_bvbs_dn13 = assign94430_e146075_d_n13;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign94440_e146085, assign94440_e146085_d_n0, assign94440_e146085_d_n2, assign94440_e146085_d_n4, assign94440_e146085_d_n5, assign94440_e146085_d_n6, assign94440_e146085_d_n7, assign94440_e146085_d_n8, assign94440_e146085_d_n9, assign94440_e146085_d_n10, assign94440_e146085_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94440_e146083: f64 = (locals.var_nin / locals.var_nover_func);
        (assign94440_e146083, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign94440_e146085;
        locals.var_t0_dn0 = assign94440_e146085_d_n0;
        locals.var_t0_dn2 = assign94440_e146085_d_n2;
        locals.var_t0_dn4 = assign94440_e146085_d_n4;
        locals.var_t0_dn5 = assign94440_e146085_d_n5;
        locals.var_t0_dn6 = assign94440_e146085_d_n6;
        locals.var_t0_dn7 = assign94440_e146085_d_n7;
        locals.var_t0_dn8 = assign94440_e146085_d_n8;
        locals.var_t0_dn9 = assign94440_e146085_d_n9;
        locals.var_t0_dn10 = assign94440_e146085_d_n10;
        locals.var_t0_dn13 = assign94440_e146085_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign94450_e146095, assign94450_e146095_d_n0, assign94450_e146095_d_n2, assign94450_e146095_d_n4, assign94450_e146095_d_n5, assign94450_e146095_d_n6, assign94450_e146095_d_n7, assign94450_e146095_d_n8, assign94450_e146095_d_n9, assign94450_e146095_d_n10, assign94450_e146095_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94450_e146093: f64 = (locals.var_t0 * locals.var_t0);
        (assign94450_e146093, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign94450_e146095;
        locals.var_cnst1over_dn0 = assign94450_e146095_d_n0;
        locals.var_cnst1over_dn2 = assign94450_e146095_d_n2;
        locals.var_cnst1over_dn4 = assign94450_e146095_d_n4;
        locals.var_cnst1over_dn5 = assign94450_e146095_d_n5;
        locals.var_cnst1over_dn6 = assign94450_e146095_d_n6;
        locals.var_cnst1over_dn7 = assign94450_e146095_d_n7;
        locals.var_cnst1over_dn8 = assign94450_e146095_d_n8;
        locals.var_cnst1over_dn9 = assign94450_e146095_d_n9;
        locals.var_cnst1over_dn10 = assign94450_e146095_d_n10;
        locals.var_cnst1over_dn13 = assign94450_e146095_d_n13;
        locals.var_cnst1over_rv = 0.0;

        let (assign94460_e146105, assign94460_e146105_d_n0, assign94460_e146105_d_n2, assign94460_e146105_d_n4, assign94460_e146105_d_n5, assign94460_e146105_d_n6, assign94460_e146105_d_n7, assign94460_e146105_d_n8, assign94460_e146105_d_n9, assign94460_e146105_d_n10, assign94460_e146105_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94460_e146103: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign94460_e146103, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign94460_e146105;
        locals.var_cfs1_dn0 = assign94460_e146105_d_n0;
        locals.var_cfs1_dn2 = assign94460_e146105_d_n2;
        locals.var_cfs1_dn4 = assign94460_e146105_d_n4;
        locals.var_cfs1_dn5 = assign94460_e146105_d_n5;
        locals.var_cfs1_dn6 = assign94460_e146105_d_n6;
        locals.var_cfs1_dn7 = assign94460_e146105_d_n7;
        locals.var_cfs1_dn8 = assign94460_e146105_d_n8;
        locals.var_cfs1_dn9 = assign94460_e146105_d_n9;
        locals.var_cfs1_dn10 = assign94460_e146105_d_n10;
        locals.var_cfs1_dn13 = assign94460_e146105_d_n13;
        locals.var_cfs1_rv = 0.0;

        let (assign94470_e146113, assign94470_e146113_d_n0, assign94470_e146113_d_n2, assign94470_e146113_d_n4, assign94470_e146113_d_n5, assign94470_e146113_d_n6, assign94470_e146113_d_n7, assign94470_e146113_d_n8, assign94470_e146113_d_n9, assign94470_e146113_d_n10, assign94470_e146113_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        (locals.var_ps0ld_ini__blk2122, locals.var_ps0ld_ini__blk2122_dn0, locals.var_ps0ld_ini__blk2122_dn2, locals.var_ps0ld_ini__blk2122_dn4, locals.var_ps0ld_ini__blk2122_dn5, locals.var_ps0ld_ini__blk2122_dn6, locals.var_ps0ld_ini__blk2122_dn7, locals.var_ps0ld_ini__blk2122_dn8, locals.var_ps0ld_ini__blk2122_dn9, locals.var_ps0ld_ini__blk2122_dn10, locals.var_ps0ld_ini__blk2122_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign94470_e146113;
        locals.var_ps0ld_dn0 = assign94470_e146113_d_n0;
        locals.var_ps0ld_dn2 = assign94470_e146113_d_n2;
        locals.var_ps0ld_dn4 = assign94470_e146113_d_n4;
        locals.var_ps0ld_dn5 = assign94470_e146113_d_n5;
        locals.var_ps0ld_dn6 = assign94470_e146113_d_n6;
        locals.var_ps0ld_dn7 = assign94470_e146113_d_n7;
        locals.var_ps0ld_dn8 = assign94470_e146113_d_n8;
        locals.var_ps0ld_dn9 = assign94470_e146113_d_n9;
        locals.var_ps0ld_dn10 = assign94470_e146113_d_n10;
        locals.var_ps0ld_dn13 = assign94470_e146113_d_n13;
        locals.var_ps0ld_rv = 0.0;

        let (assign94480_e146121,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign94480_e146121;
        locals.var_flg_conv_rv = 0.0;

        let (assign94490_e146136, assign94490_e146136_d_n0, assign94490_e146136_d_n2, assign94490_e146136_d_n4, assign94490_e146136_d_n5, assign94490_e146136_d_n6, assign94490_e146136_d_n7, assign94490_e146136_d_n8, assign94490_e146136_d_n9, assign94490_e146136_d_n10, assign94490_e146136_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94490_e146130: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2115);
        let assign94490_e146132: f64 = (assign94490_e146130 * locals.var_beta_inv);
        let assign94490_e146133: f64 = (2.0 * assign94490_e146132);
        let assign94490_e146134: f64 = (assign94490_e146133).sqrt();
        (assign94490_e146134, ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn0)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn2)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn4)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn5)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn6)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn7)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn8)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn9)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn10)) / (2.0 * assign94490_e146134)), ((2.0 * (assign94490_e146130 * locals.var_beta_inv_dn13)) / (2.0 * assign94490_e146134)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign94490_e146136;
        locals.var_c_w_ld_dn0 = assign94490_e146136_d_n0;
        locals.var_c_w_ld_dn2 = assign94490_e146136_d_n2;
        locals.var_c_w_ld_dn4 = assign94490_e146136_d_n4;
        locals.var_c_w_ld_dn5 = assign94490_e146136_d_n5;
        locals.var_c_w_ld_dn6 = assign94490_e146136_d_n6;
        locals.var_c_w_ld_dn7 = assign94490_e146136_d_n7;
        locals.var_c_w_ld_dn8 = assign94490_e146136_d_n8;
        locals.var_c_w_ld_dn9 = assign94490_e146136_d_n9;
        locals.var_c_w_ld_dn10 = assign94490_e146136_d_n10;
        locals.var_c_w_ld_dn13 = assign94490_e146136_d_n13;
        locals.var_c_w_ld_rv = 0.0;

        let assign94500_e146139: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2197 = assign94500_e146139;
        locals.var_guard2197_rv = 0.0;

        let (assign94510_e146151, assign94510_e146151_d_n0, assign94510_e146151_d_n2, assign94510_e146151_d_n4, assign94510_e146151_d_n5, assign94510_e146151_d_n6, assign94510_e146151_d_n7, assign94510_e146151_d_n8, assign94510_e146151_d_n9, assign94510_e146151_d_n10, assign94510_e146151_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2197 != 0.0)) {
        let assign94510_e146149: f64 = (p.p334 - locals.var_wdep_func);
        (assign94510_e146149, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94510_e146151;
        locals.var_t2_dn0 = assign94510_e146151_d_n0;
        locals.var_t2_dn2 = assign94510_e146151_d_n2;
        locals.var_t2_dn4 = assign94510_e146151_d_n4;
        locals.var_t2_dn5 = assign94510_e146151_d_n5;
        locals.var_t2_dn6 = assign94510_e146151_d_n6;
        locals.var_t2_dn7 = assign94510_e146151_d_n7;
        locals.var_t2_dn8 = assign94510_e146151_d_n8;
        locals.var_t2_dn9 = assign94510_e146151_d_n9;
        locals.var_t2_dn10 = assign94510_e146151_d_n10;
        locals.var_t2_dn13 = assign94510_e146151_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94520_e146175, assign94520_e146175_d_n0, assign94520_e146175_d_n2, assign94520_e146175_d_n4, assign94520_e146175_d_n5, assign94520_e146175_d_n6, assign94520_e146175_d_n7, assign94520_e146175_d_n8, assign94520_e146175_d_n9, assign94520_e146175_d_n10, assign94520_e146175_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2197 == 0.0)) {
        let assign94520_e146162: f64 = (locals.var_vdsi + p.p137);
        let assign94520_e146165: f64 = (locals.var_vdsi + p.p137);
        let assign94520_e146166: f64 = (assign94520_e146162 * assign94520_e146165);
        let assign94520_e146169: f64 = (4.0 * 0.1);
        let assign94520_e146171: f64 = (assign94520_e146169 * 0.1);
        let assign94520_e146172: f64 = (assign94520_e146166 + assign94520_e146171);
        let assign94520_e146173: f64 = (assign94520_e146172).sqrt();
        (assign94520_e146173, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign94520_e146165) + (assign94520_e146162 * locals.var_vdsi_dn5)) / (2.0 * assign94520_e146173)), 0.0, (((locals.var_vdsi_dn7 * assign94520_e146165) + (assign94520_e146162 * locals.var_vdsi_dn7)) / (2.0 * assign94520_e146173)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94520_e146175;
        locals.var_tmf2_dn0 = assign94520_e146175_d_n0;
        locals.var_tmf2_dn2 = assign94520_e146175_d_n2;
        locals.var_tmf2_dn4 = assign94520_e146175_d_n4;
        locals.var_tmf2_dn5 = assign94520_e146175_d_n5;
        locals.var_tmf2_dn6 = assign94520_e146175_d_n6;
        locals.var_tmf2_dn7 = assign94520_e146175_d_n7;
        locals.var_tmf2_dn8 = assign94520_e146175_d_n8;
        locals.var_tmf2_dn9 = assign94520_e146175_d_n9;
        locals.var_tmf2_dn10 = assign94520_e146175_d_n10;
        locals.var_tmf2_dn13 = assign94520_e146175_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign94530_e146194, assign94530_e146194_d_n0, assign94530_e146194_d_n2, assign94530_e146194_d_n4, assign94530_e146194_d_n5, assign94530_e146194_d_n6, assign94530_e146194_d_n7, assign94530_e146194_d_n8, assign94530_e146194_d_n9, assign94530_e146194_d_n10, assign94530_e146194_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2197 == 0.0)) {
        let assign94530_e146188: f64 = (locals.var_vdsi + p.p137);
        let assign94530_e146190: f64 = (assign94530_e146188 / locals.var_tmf2);
        let assign94530_e146191: f64 = (1.0 + assign94530_e146190);
        let assign94530_e146192: f64 = (0.5 * assign94530_e146191);
        (assign94530_e146192, (0.5 * (-((assign94530_e146188 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94530_e146188 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94530_e146188 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign94530_e146188 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign94530_e146188 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign94530_e146188 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign94530_e146188 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94530_e146188 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94530_e146188 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign94530_e146188 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94530_e146194;
        locals.var_t9_dn0 = assign94530_e146194_d_n0;
        locals.var_t9_dn2 = assign94530_e146194_d_n2;
        locals.var_t9_dn4 = assign94530_e146194_d_n4;
        locals.var_t9_dn5 = assign94530_e146194_d_n5;
        locals.var_t9_dn6 = assign94530_e146194_d_n6;
        locals.var_t9_dn7 = assign94530_e146194_d_n7;
        locals.var_t9_dn8 = assign94530_e146194_d_n8;
        locals.var_t9_dn9 = assign94530_e146194_d_n9;
        locals.var_t9_dn10 = assign94530_e146194_d_n10;
        locals.var_t9_dn13 = assign94530_e146194_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94540_e146211, assign94540_e146211_d_n0, assign94540_e146211_d_n2, assign94540_e146211_d_n4, assign94540_e146211_d_n5, assign94540_e146211_d_n6, assign94540_e146211_d_n7, assign94540_e146211_d_n8, assign94540_e146211_d_n9, assign94540_e146211_d_n10, assign94540_e146211_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2197 == 0.0)) {
        let assign94540_e146206: f64 = (locals.var_vdsi + p.p137);
        let assign94540_e146208: f64 = (assign94540_e146206 + locals.var_tmf2);
        let assign94540_e146209: f64 = (0.5 * assign94540_e146208);
        (assign94540_e146209, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94540_e146211;
        locals.var_t2_dn0 = assign94540_e146211_d_n0;
        locals.var_t2_dn2 = assign94540_e146211_d_n2;
        locals.var_t2_dn4 = assign94540_e146211_d_n4;
        locals.var_t2_dn5 = assign94540_e146211_d_n5;
        locals.var_t2_dn6 = assign94540_e146211_d_n6;
        locals.var_t2_dn7 = assign94540_e146211_d_n7;
        locals.var_t2_dn8 = assign94540_e146211_d_n8;
        locals.var_t2_dn9 = assign94540_e146211_d_n9;
        locals.var_t2_dn10 = assign94540_e146211_d_n10;
        locals.var_t2_dn13 = assign94540_e146211_d_n13;
        locals.var_t2_rv = 0.0;

        let assign94550_e146214: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2198 = assign94550_e146214;
        locals.var_guard2198_rv = 0.0;

        let (assign94560_e146227, assign94560_e146227_d_n0, assign94560_e146227_d_n2, assign94560_e146227_d_n4, assign94560_e146227_d_n5, assign94560_e146227_d_n6, assign94560_e146227_d_n7, assign94560_e146227_d_n8, assign94560_e146227_d_n9, assign94560_e146227_d_n10, assign94560_e146227_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2197 == 0.0)) && (locals.var_guard2198 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94560_e146227;
        locals.var_t2_dn0 = assign94560_e146227_d_n0;
        locals.var_t2_dn2 = assign94560_e146227_d_n2;
        locals.var_t2_dn4 = assign94560_e146227_d_n4;
        locals.var_t2_dn5 = assign94560_e146227_d_n5;
        locals.var_t2_dn6 = assign94560_e146227_d_n6;
        locals.var_t2_dn7 = assign94560_e146227_d_n7;
        locals.var_t2_dn8 = assign94560_e146227_d_n8;
        locals.var_t2_dn9 = assign94560_e146227_d_n9;
        locals.var_t2_dn10 = assign94560_e146227_d_n10;
        locals.var_t2_dn13 = assign94560_e146227_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94570_e146240, assign94570_e146240_d_n0, assign94570_e146240_d_n2, assign94570_e146240_d_n4, assign94570_e146240_d_n5, assign94570_e146240_d_n6, assign94570_e146240_d_n7, assign94570_e146240_d_n8, assign94570_e146240_d_n9, assign94570_e146240_d_n10, assign94570_e146240_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2197 == 0.0)) && (locals.var_guard2198 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94570_e146240;
        locals.var_t9_dn0 = assign94570_e146240_d_n0;
        locals.var_t9_dn2 = assign94570_e146240_d_n2;
        locals.var_t9_dn4 = assign94570_e146240_d_n4;
        locals.var_t9_dn5 = assign94570_e146240_d_n5;
        locals.var_t9_dn6 = assign94570_e146240_d_n6;
        locals.var_t9_dn7 = assign94570_e146240_d_n7;
        locals.var_t9_dn8 = assign94570_e146240_d_n8;
        locals.var_t9_dn9 = assign94570_e146240_d_n9;
        locals.var_t9_dn10 = assign94570_e146240_d_n10;
        locals.var_t9_dn13 = assign94570_e146240_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94580_e146256, assign94580_e146256_d_n0, assign94580_e146256_d_n2, assign94580_e146256_d_n4, assign94580_e146256_d_n5, assign94580_e146256_d_n6, assign94580_e146256_d_n7, assign94580_e146256_d_n8, assign94580_e146256_d_n9, assign94580_e146256_d_n10, assign94580_e146256_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2197 == 0.0)) {
        let assign94580_e146251: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94580_e146252: f64 = (assign94580_e146251).sqrt();
        let assign94580_e146254: f64 = (assign94580_e146252 * p.p432);
        (assign94580_e146254, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94580_e146252)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign94580_e146252)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign94580_e146256;
        locals.var_wjunc0_dn0 = assign94580_e146256_d_n0;
        locals.var_wjunc0_dn2 = assign94580_e146256_d_n2;
        locals.var_wjunc0_dn4 = assign94580_e146256_d_n4;
        locals.var_wjunc0_dn5 = assign94580_e146256_d_n5;
        locals.var_wjunc0_dn6 = assign94580_e146256_d_n6;
        locals.var_wjunc0_dn7 = assign94580_e146256_d_n7;
        locals.var_wjunc0_dn8 = assign94580_e146256_d_n8;
        locals.var_wjunc0_dn9 = assign94580_e146256_d_n9;
        locals.var_wjunc0_dn10 = assign94580_e146256_d_n10;
        locals.var_wjunc0_dn13 = assign94580_e146256_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign94590_e146269, assign94590_e146269_d_n0, assign94590_e146269_d_n2, assign94590_e146269_d_n4, assign94590_e146269_d_n5, assign94590_e146269_d_n6, assign94590_e146269_d_n7, assign94590_e146269_d_n8, assign94590_e146269_d_n9, assign94590_e146269_d_n10, assign94590_e146269_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2197 == 0.0)) {
        let assign94590_e146267: f64 = (p.p334 - locals.var_wjunc0);
        (assign94590_e146267, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94590_e146269;
        locals.var_t2_dn0 = assign94590_e146269_d_n0;
        locals.var_t2_dn2 = assign94590_e146269_d_n2;
        locals.var_t2_dn4 = assign94590_e146269_d_n4;
        locals.var_t2_dn5 = assign94590_e146269_d_n5;
        locals.var_t2_dn6 = assign94590_e146269_d_n6;
        locals.var_t2_dn7 = assign94590_e146269_d_n7;
        locals.var_t2_dn8 = assign94590_e146269_d_n8;
        locals.var_t2_dn9 = assign94590_e146269_d_n9;
        locals.var_t2_dn10 = assign94590_e146269_d_n10;
        locals.var_t2_dn13 = assign94590_e146269_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94600_e146290, assign94600_e146290_d_n0, assign94600_e146290_d_n2, assign94600_e146290_d_n4, assign94600_e146290_d_n5, assign94600_e146290_d_n6, assign94600_e146290_d_n7, assign94600_e146290_d_n8, assign94600_e146290_d_n9, assign94600_e146290_d_n10, assign94600_e146290_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94600_e146277: f64 = (locals.var_t2 * locals.var_t2);
        let assign94600_e146281: f64 = (p.p334 * 0.01);
        let assign94600_e146282: f64 = (4.0 * assign94600_e146281);
        let assign94600_e146285: f64 = (p.p334 * 0.01);
        let assign94600_e146286: f64 = (assign94600_e146282 * assign94600_e146285);
        let assign94600_e146287: f64 = (assign94600_e146277 + assign94600_e146286);
        let assign94600_e146288: f64 = (assign94600_e146287).sqrt();
        (assign94600_e146288, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign94600_e146288)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign94600_e146288)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94600_e146290;
        locals.var_tmf2_dn0 = assign94600_e146290_d_n0;
        locals.var_tmf2_dn2 = assign94600_e146290_d_n2;
        locals.var_tmf2_dn4 = assign94600_e146290_d_n4;
        locals.var_tmf2_dn5 = assign94600_e146290_d_n5;
        locals.var_tmf2_dn6 = assign94600_e146290_d_n6;
        locals.var_tmf2_dn7 = assign94600_e146290_d_n7;
        locals.var_tmf2_dn8 = assign94600_e146290_d_n8;
        locals.var_tmf2_dn9 = assign94600_e146290_d_n9;
        locals.var_tmf2_dn10 = assign94600_e146290_d_n10;
        locals.var_tmf2_dn13 = assign94600_e146290_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_354(
        locals: &mut StampLocals,
    ) {
        let (assign94610_e146304, assign94610_e146304_d_n0, assign94610_e146304_d_n2, assign94610_e146304_d_n4, assign94610_e146304_d_n5, assign94610_e146304_d_n6, assign94610_e146304_d_n7, assign94610_e146304_d_n8, assign94610_e146304_d_n9, assign94610_e146304_d_n10, assign94610_e146304_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94610_e146300: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign94610_e146301: f64 = (1.0 + assign94610_e146300);
        let assign94610_e146302: f64 = (0.5 * assign94610_e146301);
        (assign94610_e146302, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94610_e146304;
        locals.var_t9_dn0 = assign94610_e146304_d_n0;
        locals.var_t9_dn2 = assign94610_e146304_d_n2;
        locals.var_t9_dn4 = assign94610_e146304_d_n4;
        locals.var_t9_dn5 = assign94610_e146304_d_n5;
        locals.var_t9_dn6 = assign94610_e146304_d_n6;
        locals.var_t9_dn7 = assign94610_e146304_d_n7;
        locals.var_t9_dn8 = assign94610_e146304_d_n8;
        locals.var_t9_dn9 = assign94610_e146304_d_n9;
        locals.var_t9_dn10 = assign94610_e146304_d_n10;
        locals.var_t9_dn13 = assign94610_e146304_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94620_e146316, assign94620_e146316_d_n0, assign94620_e146316_d_n2, assign94620_e146316_d_n4, assign94620_e146316_d_n5, assign94620_e146316_d_n6, assign94620_e146316_d_n7, assign94620_e146316_d_n8, assign94620_e146316_d_n9, assign94620_e146316_d_n10, assign94620_e146316_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94620_e146313: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign94620_e146314: f64 = (0.5 * assign94620_e146313);
        (assign94620_e146314, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94620_e146316;
        locals.var_t2_dn0 = assign94620_e146316_d_n0;
        locals.var_t2_dn2 = assign94620_e146316_d_n2;
        locals.var_t2_dn4 = assign94620_e146316_d_n4;
        locals.var_t2_dn5 = assign94620_e146316_d_n5;
        locals.var_t2_dn6 = assign94620_e146316_d_n6;
        locals.var_t2_dn7 = assign94620_e146316_d_n7;
        locals.var_t2_dn8 = assign94620_e146316_d_n8;
        locals.var_t2_dn9 = assign94620_e146316_d_n9;
        locals.var_t2_dn10 = assign94620_e146316_d_n10;
        locals.var_t2_dn13 = assign94620_e146316_d_n13;
        locals.var_t2_rv = 0.0;

        let assign94630_e146319: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2199 = assign94630_e146319;
        locals.var_guard2199_rv = 0.0;

        let (assign94640_e146329, assign94640_e146329_d_n0, assign94640_e146329_d_n2, assign94640_e146329_d_n4, assign94640_e146329_d_n5, assign94640_e146329_d_n6, assign94640_e146329_d_n7, assign94640_e146329_d_n8, assign94640_e146329_d_n9, assign94640_e146329_d_n10, assign94640_e146329_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94640_e146329;
        locals.var_t2_dn0 = assign94640_e146329_d_n0;
        locals.var_t2_dn2 = assign94640_e146329_d_n2;
        locals.var_t2_dn4 = assign94640_e146329_d_n4;
        locals.var_t2_dn5 = assign94640_e146329_d_n5;
        locals.var_t2_dn6 = assign94640_e146329_d_n6;
        locals.var_t2_dn7 = assign94640_e146329_d_n7;
        locals.var_t2_dn8 = assign94640_e146329_d_n8;
        locals.var_t2_dn9 = assign94640_e146329_d_n9;
        locals.var_t2_dn10 = assign94640_e146329_d_n10;
        locals.var_t2_dn13 = assign94640_e146329_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94650_e146339, assign94650_e146339_d_n0, assign94650_e146339_d_n2, assign94650_e146339_d_n4, assign94650_e146339_d_n5, assign94650_e146339_d_n6, assign94650_e146339_d_n7, assign94650_e146339_d_n8, assign94650_e146339_d_n9, assign94650_e146339_d_n10, assign94650_e146339_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2199 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94650_e146339;
        locals.var_t9_dn0 = assign94650_e146339_d_n0;
        locals.var_t9_dn2 = assign94650_e146339_d_n2;
        locals.var_t9_dn4 = assign94650_e146339_d_n4;
        locals.var_t9_dn5 = assign94650_e146339_d_n5;
        locals.var_t9_dn6 = assign94650_e146339_d_n6;
        locals.var_t9_dn7 = assign94650_e146339_d_n7;
        locals.var_t9_dn8 = assign94650_e146339_d_n8;
        locals.var_t9_dn9 = assign94650_e146339_d_n9;
        locals.var_t9_dn10 = assign94650_e146339_d_n10;
        locals.var_t9_dn13 = assign94650_e146339_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94660_e146347, assign94660_e146347_d_n0, assign94660_e146347_d_n2, assign94660_e146347_d_n4, assign94660_e146347_d_n5, assign94660_e146347_d_n6, assign94660_e146347_d_n7, assign94660_e146347_d_n8, assign94660_e146347_d_n9, assign94660_e146347_d_n10, assign94660_e146347_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign94660_e146347;
        locals.var_ddriftldc_dn0 = assign94660_e146347_d_n0;
        locals.var_ddriftldc_dn2 = assign94660_e146347_d_n2;
        locals.var_ddriftldc_dn4 = assign94660_e146347_d_n4;
        locals.var_ddriftldc_dn5 = assign94660_e146347_d_n5;
        locals.var_ddriftldc_dn6 = assign94660_e146347_d_n6;
        locals.var_ddriftldc_dn7 = assign94660_e146347_d_n7;
        locals.var_ddriftldc_dn8 = assign94660_e146347_d_n8;
        locals.var_ddriftldc_dn9 = assign94660_e146347_d_n9;
        locals.var_ddriftldc_dn10 = assign94660_e146347_d_n10;
        locals.var_ddriftldc_dn13 = assign94660_e146347_d_n13;
        locals.var_ddriftldc_rv = 0.0;

        let (assign94670_e146363, assign94670_e146363_d_n0, assign94670_e146363_d_n2, assign94670_e146363_d_n4, assign94670_e146363_d_n5, assign94670_e146363_d_n6, assign94670_e146363_d_n7, assign94670_e146363_d_n8, assign94670_e146363_d_n9, assign94670_e146363_d_n10, assign94670_e146363_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94670_e146355: f64 = (locals.var_q_nsubld__blk2115 * locals.var_ddriftldc);
        let assign94670_e146357: f64 = (assign94670_e146355 * locals.var_ddriftldc);
        let assign94670_e146359: f64 = (assign94670_e146357 / 2.0);
        let assign94670_e146361: f64 = (assign94670_e146359 / 1.034943e-10);
        (assign94670_e146361, (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2115 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign94670_e146355 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign94670_e146363;
        locals.var_dphi_sb_dn0 = assign94670_e146363_d_n0;
        locals.var_dphi_sb_dn2 = assign94670_e146363_d_n2;
        locals.var_dphi_sb_dn4 = assign94670_e146363_d_n4;
        locals.var_dphi_sb_dn5 = assign94670_e146363_d_n5;
        locals.var_dphi_sb_dn6 = assign94670_e146363_d_n6;
        locals.var_dphi_sb_dn7 = assign94670_e146363_d_n7;
        locals.var_dphi_sb_dn8 = assign94670_e146363_d_n8;
        locals.var_dphi_sb_dn9 = assign94670_e146363_d_n9;
        locals.var_dphi_sb_dn10 = assign94670_e146363_d_n10;
        locals.var_dphi_sb_dn13 = assign94670_e146363_d_n13;
        locals.var_dphi_sb_rv = 0.0;

        let (assign94680_e146376, assign94680_e146376_d_n0, assign94680_e146376_d_n2, assign94680_e146376_d_n4, assign94680_e146376_d_n5, assign94680_e146376_d_n6, assign94680_e146376_d_n7, assign94680_e146376_d_n8, assign94680_e146376_d_n9, assign94680_e146376_d_n10, assign94680_e146376_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94680_e146371: f64 = (2.0 * locals.var_beta);
        let assign94680_e146373: f64 = (assign94680_e146371 * locals.var_dphi_sb);
        let assign94680_e146374: f64 = (assign94680_e146373).sqrt();
        (assign94680_e146374, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn0)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn2)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn4)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn5)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn6)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn7)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn8)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn9)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn10)) / (2.0 * assign94680_e146374)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign94680_e146371 * locals.var_dphi_sb_dn13)) / (2.0 * assign94680_e146374)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign94680_e146376;
        locals.var_t0_dn0 = assign94680_e146376_d_n0;
        locals.var_t0_dn2 = assign94680_e146376_d_n2;
        locals.var_t0_dn4 = assign94680_e146376_d_n4;
        locals.var_t0_dn5 = assign94680_e146376_d_n5;
        locals.var_t0_dn6 = assign94680_e146376_d_n6;
        locals.var_t0_dn7 = assign94680_e146376_d_n7;
        locals.var_t0_dn8 = assign94680_e146376_d_n8;
        locals.var_t0_dn9 = assign94680_e146376_d_n9;
        locals.var_t0_dn10 = assign94680_e146376_d_n10;
        locals.var_t0_dn13 = assign94680_e146376_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign94690_e146391, assign94690_e146391_d_n0, assign94690_e146391_d_n2, assign94690_e146391_d_n4, assign94690_e146391_d_n5, assign94690_e146391_d_n6, assign94690_e146391_d_n7, assign94690_e146391_d_n8, assign94690_e146391_d_n9, assign94690_e146391_d_n10, assign94690_e146391_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94690_e146383: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94690_e146385: f64 = (-locals.var_t0);
        let assign94690_e146386: f64 = { let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign94690_e146387: f64 = (assign94690_e146383 + assign94690_e146386);
        let assign94690_e146389: f64 = (assign94690_e146387 / 2.0);
        (assign94690_e146389, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign94690_e146385; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign94690_e146391;
        locals.var_t1_dn0 = assign94690_e146391_d_n0;
        locals.var_t1_dn2 = assign94690_e146391_d_n2;
        locals.var_t1_dn4 = assign94690_e146391_d_n4;
        locals.var_t1_dn5 = assign94690_e146391_d_n5;
        locals.var_t1_dn6 = assign94690_e146391_d_n6;
        locals.var_t1_dn7 = assign94690_e146391_d_n7;
        locals.var_t1_dn8 = assign94690_e146391_d_n8;
        locals.var_t1_dn9 = assign94690_e146391_d_n9;
        locals.var_t1_dn10 = assign94690_e146391_d_n10;
        locals.var_t1_dn13 = assign94690_e146391_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign94700_e146402, assign94700_e146402_d_n0, assign94700_e146402_d_n2, assign94700_e146402_d_n4, assign94700_e146402_d_n5, assign94700_e146402_d_n6, assign94700_e146402_d_n7, assign94700_e146402_d_n8, assign94700_e146402_d_n9, assign94700_e146402_d_n10, assign94700_e146402_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94700_e146398: f64 = (locals.var_t1).ln();
        let assign94700_e146400: f64 = (assign94700_e146398 / locals.var_dphi_sb);
        (assign94700_e146400, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign94700_e146398 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign94700_e146402;
        locals.var_c_sb_dn0 = assign94700_e146402_d_n0;
        locals.var_c_sb_dn2 = assign94700_e146402_d_n2;
        locals.var_c_sb_dn4 = assign94700_e146402_d_n4;
        locals.var_c_sb_dn5 = assign94700_e146402_d_n5;
        locals.var_c_sb_dn6 = assign94700_e146402_d_n6;
        locals.var_c_sb_dn7 = assign94700_e146402_d_n7;
        locals.var_c_sb_dn8 = assign94700_e146402_d_n8;
        locals.var_c_sb_dn9 = assign94700_e146402_d_n9;
        locals.var_c_sb_dn10 = assign94700_e146402_d_n10;
        locals.var_c_sb_dn13 = assign94700_e146402_d_n13;
        locals.var_c_sb_rv = 0.0;

        let (assign94710_e146410,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign94710_e146410;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_355(
        locals: &mut StampLocals,
    ) {
        let mut assign94720_loop_guard: usize = 0;
        while {
            let assign94720_cond_e146419: f64 = (locals.var_lp_s0_max + 1.0);
            let assign94720_cond_e146421: f64 = if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_lp_s0 <= assign94720_cond_e146419)) { 1.0 } else { 0.0 };
            assign94720_cond_e146421 != 0.0
        } {
            assign94720_loop_guard += 1;
            assert!(assign94720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign94720_body3_e146454, assign94720_body3_e146454_d_n0, assign94720_body3_e146454_d_n2, assign94720_body3_e146454_d_n4, assign94720_body3_e146454_d_n5, assign94720_body3_e146454_d_n6, assign94720_body3_e146454_d_n7, assign94720_body3_e146454_d_n8, assign94720_body3_e146454_d_n9, assign94720_body3_e146454_d_n10, assign94720_body3_e146454_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body3_e146452: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign94720_body3_e146452, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign94720_body3_e146454;
            locals.var_ps0ld_vxb_dn0 = assign94720_body3_e146454_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign94720_body3_e146454_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign94720_body3_e146454_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign94720_body3_e146454_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign94720_body3_e146454_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign94720_body3_e146454_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign94720_body3_e146454_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign94720_body3_e146454_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign94720_body3_e146454_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign94720_body3_e146454_d_n13;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign94720_body4_e146464, assign94720_body4_e146464_d_n0, assign94720_body4_e146464_d_n2, assign94720_body4_e146464_d_n4, assign94720_body4_e146464_d_n5, assign94720_body4_e146464_d_n6, assign94720_body4_e146464_d_n7, assign94720_body4_e146464_d_n8, assign94720_body4_e146464_d_n9, assign94720_body4_e146464_d_n10, assign94720_body4_e146464_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body4_e146462: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign94720_body4_e146462, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign94720_body4_e146464;
            locals.var_chi_dn0 = assign94720_body4_e146464_d_n0;
            locals.var_chi_dn2 = assign94720_body4_e146464_d_n2;
            locals.var_chi_dn4 = assign94720_body4_e146464_d_n4;
            locals.var_chi_dn5 = assign94720_body4_e146464_d_n5;
            locals.var_chi_dn6 = assign94720_body4_e146464_d_n6;
            locals.var_chi_dn7 = assign94720_body4_e146464_d_n7;
            locals.var_chi_dn8 = assign94720_body4_e146464_d_n8;
            locals.var_chi_dn9 = assign94720_body4_e146464_d_n9;
            locals.var_chi_dn10 = assign94720_body4_e146464_d_n10;
            locals.var_chi_dn13 = assign94720_body4_e146464_d_n13;
            locals.var_chi_rv = 0.0;
            let (assign94720_body5_e146476, assign94720_body5_e146476_d_n0, assign94720_body5_e146476_d_n2, assign94720_body5_e146476_d_n4, assign94720_body5_e146476_d_n5, assign94720_body5_e146476_d_n6, assign94720_body5_e146476_d_n7, assign94720_body5_e146476_d_n8, assign94720_body5_e146476_d_n9, assign94720_body5_e146476_d_n10, assign94720_body5_e146476_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body5_e146473: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign94720_body5_e146474: f64 = (locals.var_c_sb * assign94720_body5_e146473);
        (assign94720_body5_e146474, ((locals.var_c_sb_dn0 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign94720_body5_e146473) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign94720_body5_e146476;
            locals.var_ty_dn0 = assign94720_body5_e146476_d_n0;
            locals.var_ty_dn2 = assign94720_body5_e146476_d_n2;
            locals.var_ty_dn4 = assign94720_body5_e146476_d_n4;
            locals.var_ty_dn5 = assign94720_body5_e146476_d_n5;
            locals.var_ty_dn6 = assign94720_body5_e146476_d_n6;
            locals.var_ty_dn7 = assign94720_body5_e146476_d_n7;
            locals.var_ty_dn8 = assign94720_body5_e146476_d_n8;
            locals.var_ty_dn9 = assign94720_body5_e146476_d_n9;
            locals.var_ty_dn10 = assign94720_body5_e146476_d_n10;
            locals.var_ty_dn13 = assign94720_body5_e146476_d_n13;
            locals.var_ty_rv = 0.0;
            let assign94720_body6_e146479: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2201 = assign94720_body6_e146479;
            locals.var_guard2201_rv = 0.0;
            let (assign94720_body7_e146490, assign94720_body7_e146490_d_n0, assign94720_body7_e146490_d_n2, assign94720_body7_e146490_d_n4, assign94720_body7_e146490_d_n5, assign94720_body7_e146490_d_n6, assign94720_body7_e146490_d_n7, assign94720_body7_e146490_d_n8, assign94720_body7_e146490_d_n9, assign94720_body7_e146490_d_n10, assign94720_body7_e146490_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body7_e146488: f64 = (locals.var_ty).exp();
        (assign94720_body7_e146488, (assign94720_body7_e146488 * locals.var_ty_dn0), (assign94720_body7_e146488 * locals.var_ty_dn2), (assign94720_body7_e146488 * locals.var_ty_dn4), (assign94720_body7_e146488 * locals.var_ty_dn5), (assign94720_body7_e146488 * locals.var_ty_dn6), (assign94720_body7_e146488 * locals.var_ty_dn7), (assign94720_body7_e146488 * locals.var_ty_dn8), (assign94720_body7_e146488 * locals.var_ty_dn9), (assign94720_body7_e146488 * locals.var_ty_dn10), (assign94720_body7_e146488 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body7_e146490;
            locals.var_t1_dn0 = assign94720_body7_e146490_d_n0;
            locals.var_t1_dn2 = assign94720_body7_e146490_d_n2;
            locals.var_t1_dn4 = assign94720_body7_e146490_d_n4;
            locals.var_t1_dn5 = assign94720_body7_e146490_d_n5;
            locals.var_t1_dn6 = assign94720_body7_e146490_d_n6;
            locals.var_t1_dn7 = assign94720_body7_e146490_d_n7;
            locals.var_t1_dn8 = assign94720_body7_e146490_d_n8;
            locals.var_t1_dn9 = assign94720_body7_e146490_d_n9;
            locals.var_t1_dn10 = assign94720_body7_e146490_d_n10;
            locals.var_t1_dn13 = assign94720_body7_e146490_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94720_body8_e146504, assign94720_body8_e146504_d_n0, assign94720_body8_e146504_d_n2, assign94720_body8_e146504_d_n4, assign94720_body8_e146504_d_n5, assign94720_body8_e146504_d_n6, assign94720_body8_e146504_d_n7, assign94720_body8_e146504_d_n8, assign94720_body8_e146504_d_n9, assign94720_body8_e146504_d_n10, assign94720_body8_e146504_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body8_e146499: f64 = (-locals.var_c_sb);
        let assign94720_body8_e146501: f64 = (assign94720_body8_e146499 * locals.var_dphi_sb);
        let assign94720_body8_e146502: f64 = (assign94720_body8_e146501).exp();
        (assign94720_body8_e146502, (assign94720_body8_e146502 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn0))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn2))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn4))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn5))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn6))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn7))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn8))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn9))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn10))), (assign94720_body8_e146502 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign94720_body8_e146499 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94720_body8_e146504;
            locals.var_t0_dn0 = assign94720_body8_e146504_d_n0;
            locals.var_t0_dn2 = assign94720_body8_e146504_d_n2;
            locals.var_t0_dn4 = assign94720_body8_e146504_d_n4;
            locals.var_t0_dn5 = assign94720_body8_e146504_d_n5;
            locals.var_t0_dn6 = assign94720_body8_e146504_d_n6;
            locals.var_t0_dn7 = assign94720_body8_e146504_d_n7;
            locals.var_t0_dn8 = assign94720_body8_e146504_d_n8;
            locals.var_t0_dn9 = assign94720_body8_e146504_d_n9;
            locals.var_t0_dn10 = assign94720_body8_e146504_d_n10;
            locals.var_t0_dn13 = assign94720_body8_e146504_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94720_body9_e146516, assign94720_body9_e146516_d_n0, assign94720_body9_e146516_d_n2, assign94720_body9_e146516_d_n4, assign94720_body9_e146516_d_n5, assign94720_body9_e146516_d_n6, assign94720_body9_e146516_d_n7, assign94720_body9_e146516_d_n8, assign94720_body9_e146516_d_n9, assign94720_body9_e146516_d_n10, assign94720_body9_e146516_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body9_e146514: f64 = (locals.var_t1 - locals.var_t0);
        (assign94720_body9_e146514, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign94720_body9_e146516;
            locals.var_t2_dn0 = assign94720_body9_e146516_d_n0;
            locals.var_t2_dn2 = assign94720_body9_e146516_d_n2;
            locals.var_t2_dn4 = assign94720_body9_e146516_d_n4;
            locals.var_t2_dn5 = assign94720_body9_e146516_d_n5;
            locals.var_t2_dn6 = assign94720_body9_e146516_d_n6;
            locals.var_t2_dn7 = assign94720_body9_e146516_d_n7;
            locals.var_t2_dn8 = assign94720_body9_e146516_d_n8;
            locals.var_t2_dn9 = assign94720_body9_e146516_d_n9;
            locals.var_t2_dn10 = assign94720_body9_e146516_d_n10;
            locals.var_t2_dn13 = assign94720_body9_e146516_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign94720_body10_e146531, assign94720_body10_e146531_d_n0, assign94720_body10_e146531_d_n2, assign94720_body10_e146531_d_n4, assign94720_body10_e146531_d_n5, assign94720_body10_e146531_d_n6, assign94720_body10_e146531_d_n7, assign94720_body10_e146531_d_n8, assign94720_body10_e146531_d_n9, assign94720_body10_e146531_d_n10, assign94720_body10_e146531_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body10_e146526: f64 = (1.0 + locals.var_t2);
        let assign94720_body10_e146527: f64 = (assign94720_body10_e146526).ln();
        let assign94720_body10_e146529: f64 = (assign94720_body10_e146527 / locals.var_c_sb);
        (assign94720_body10_e146529, ((((locals.var_t2_dn0 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign94720_body10_e146526) * locals.var_c_sb) - (assign94720_body10_e146527 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign94720_body10_e146531;
            locals.var_phi_b_dn0 = assign94720_body10_e146531_d_n0;
            locals.var_phi_b_dn2 = assign94720_body10_e146531_d_n2;
            locals.var_phi_b_dn4 = assign94720_body10_e146531_d_n4;
            locals.var_phi_b_dn5 = assign94720_body10_e146531_d_n5;
            locals.var_phi_b_dn6 = assign94720_body10_e146531_d_n6;
            locals.var_phi_b_dn7 = assign94720_body10_e146531_d_n7;
            locals.var_phi_b_dn8 = assign94720_body10_e146531_d_n8;
            locals.var_phi_b_dn9 = assign94720_body10_e146531_d_n9;
            locals.var_phi_b_dn10 = assign94720_body10_e146531_d_n10;
            locals.var_phi_b_dn13 = assign94720_body10_e146531_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign94720_body11_e146545, assign94720_body11_e146545_d_n0, assign94720_body11_e146545_d_n2, assign94720_body11_e146545_d_n4, assign94720_body11_e146545_d_n5, assign94720_body11_e146545_d_n6, assign94720_body11_e146545_d_n7, assign94720_body11_e146545_d_n8, assign94720_body11_e146545_d_n9, assign94720_body11_e146545_d_n10, assign94720_body11_e146545_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 != 0.0)) {
        let assign94720_body11_e146542: f64 = (1.0 + locals.var_t2);
        let assign94720_body11_e146543: f64 = (locals.var_t1 / assign94720_body11_e146542);
        (assign94720_body11_e146543, (((locals.var_t1_dn0 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn0)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn2 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn2)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn4 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn4)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn5 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn5)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn6 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn6)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn7 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn7)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn8 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn8)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn9 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn9)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn10 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn10)) / (assign94720_body11_e146542 * assign94720_body11_e146542)), (((locals.var_t1_dn13 * assign94720_body11_e146542) - (locals.var_t1 * locals.var_t2_dn13)) / (assign94720_body11_e146542 * assign94720_body11_e146542)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign94720_body11_e146545;
            locals.var_phi_b_dpss_dn0 = assign94720_body11_e146545_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94720_body11_e146545_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94720_body11_e146545_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94720_body11_e146545_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94720_body11_e146545_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94720_body11_e146545_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94720_body11_e146545_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94720_body11_e146545_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94720_body11_e146545_d_n10;
            locals.var_phi_b_dpss_dn13 = assign94720_body11_e146545_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94720_body12_e146558, assign94720_body12_e146558_d_n0, assign94720_body12_e146558_d_n2, assign94720_body12_e146558_d_n4, assign94720_body12_e146558_d_n5, assign94720_body12_e146558_d_n6, assign94720_body12_e146558_d_n7, assign94720_body12_e146558_d_n8, assign94720_body12_e146558_d_n9, assign94720_body12_e146558_d_n10, assign94720_body12_e146558_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        let assign94720_body12_e146556: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign94720_body12_e146556, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign94720_body12_e146558;
            locals.var_phi_b_dn0 = assign94720_body12_e146558_d_n0;
            locals.var_phi_b_dn2 = assign94720_body12_e146558_d_n2;
            locals.var_phi_b_dn4 = assign94720_body12_e146558_d_n4;
            locals.var_phi_b_dn5 = assign94720_body12_e146558_d_n5;
            locals.var_phi_b_dn6 = assign94720_body12_e146558_d_n6;
            locals.var_phi_b_dn7 = assign94720_body12_e146558_d_n7;
            locals.var_phi_b_dn8 = assign94720_body12_e146558_d_n8;
            locals.var_phi_b_dn9 = assign94720_body12_e146558_d_n9;
            locals.var_phi_b_dn10 = assign94720_body12_e146558_d_n10;
            locals.var_phi_b_dn13 = assign94720_body12_e146558_d_n13;
            locals.var_phi_b_rv = 0.0;
            let (assign94720_body13_e146569, assign94720_body13_e146569_d_n0, assign94720_body13_e146569_d_n2, assign94720_body13_e146569_d_n4, assign94720_body13_e146569_d_n5, assign94720_body13_e146569_d_n6, assign94720_body13_e146569_d_n7, assign94720_body13_e146569_d_n8, assign94720_body13_e146569_d_n9, assign94720_body13_e146569_d_n10, assign94720_body13_e146569_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2201 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign94720_body13_e146569;
            locals.var_phi_b_dpss_dn0 = assign94720_body13_e146569_d_n0;
            locals.var_phi_b_dpss_dn2 = assign94720_body13_e146569_d_n2;
            locals.var_phi_b_dpss_dn4 = assign94720_body13_e146569_d_n4;
            locals.var_phi_b_dpss_dn5 = assign94720_body13_e146569_d_n5;
            locals.var_phi_b_dpss_dn6 = assign94720_body13_e146569_d_n6;
            locals.var_phi_b_dpss_dn7 = assign94720_body13_e146569_d_n7;
            locals.var_phi_b_dpss_dn8 = assign94720_body13_e146569_d_n8;
            locals.var_phi_b_dpss_dn9 = assign94720_body13_e146569_d_n9;
            locals.var_phi_b_dpss_dn10 = assign94720_body13_e146569_d_n10;
            locals.var_phi_b_dpss_dn13 = assign94720_body13_e146569_d_n13;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign94720_body14_e146579, assign94720_body14_e146579_d_n0, assign94720_body14_e146579_d_n2, assign94720_body14_e146579_d_n4, assign94720_body14_e146579_d_n5, assign94720_body14_e146579_d_n6, assign94720_body14_e146579_d_n7, assign94720_body14_e146579_d_n8, assign94720_body14_e146579_d_n9, assign94720_body14_e146579_d_n10, assign94720_body14_e146579_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body14_e146577: f64 = (locals.var_beta * locals.var_phi_b);
        (assign94720_body14_e146577, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign94720_body14_e146579;
            locals.var_chib_dn0 = assign94720_body14_e146579_d_n0;
            locals.var_chib_dn2 = assign94720_body14_e146579_d_n2;
            locals.var_chib_dn4 = assign94720_body14_e146579_d_n4;
            locals.var_chib_dn5 = assign94720_body14_e146579_d_n5;
            locals.var_chib_dn6 = assign94720_body14_e146579_d_n6;
            locals.var_chib_dn7 = assign94720_body14_e146579_d_n7;
            locals.var_chib_dn8 = assign94720_body14_e146579_d_n8;
            locals.var_chib_dn9 = assign94720_body14_e146579_d_n9;
            locals.var_chib_dn10 = assign94720_body14_e146579_d_n10;
            locals.var_chib_dn13 = assign94720_body14_e146579_d_n13;
            locals.var_chib_rv = 0.0;
            let assign94720_body15_e146581: f64 = (locals.var_chi).abs();
            let assign94720_body15_e146583: f64 = if assign94720_body15_e146581 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2202 = assign94720_body15_e146583;
            locals.var_guard2202_rv = 0.0;
            let (assign94720_body17_e146633, assign94720_body17_e146633_d_n0, assign94720_body17_e146633_d_n2, assign94720_body17_e146633_d_n4, assign94720_body17_e146633_d_n5, assign94720_body17_e146633_d_n6, assign94720_body17_e146633_d_n7, assign94720_body17_e146633_d_n8, assign94720_body17_e146633_d_n9, assign94720_body17_e146633_d_n10, assign94720_body17_e146633_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body17_e146611: f64 = (locals.var_chi * locals.var_chi);
        let assign94720_body17_e146613: f64 = (assign94720_body17_e146611 / 2.0);
        let assign94720_body17_e146617: f64 = (locals.var_chi / 3.0);
        let assign94720_body17_e146621: f64 = (locals.var_chi / 4.0);
        let assign94720_body17_e146625: f64 = (locals.var_chi / 5.0);
        let assign94720_body17_e146626: f64 = (1.0 - assign94720_body17_e146625);
        let assign94720_body17_e146627: f64 = (assign94720_body17_e146621 * assign94720_body17_e146626);
        let assign94720_body17_e146628: f64 = (1.0 - assign94720_body17_e146627);
        let assign94720_body17_e146629: f64 = (assign94720_body17_e146617 * assign94720_body17_e146628);
        let assign94720_body17_e146630: f64 = (1.0 - assign94720_body17_e146629);
        let assign94720_body17_e146631: f64 = (assign94720_body17_e146613 * assign94720_body17_e146630);
        (assign94720_body17_e146631, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn0 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn0 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn2 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn2 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn4 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn4 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn5 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn5 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn6 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn6 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn7 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn7 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn8 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn8 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn9 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn9 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn10 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn10 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign94720_body17_e146630) + (assign94720_body17_e146613 * (-(((locals.var_chi_dn13 / 3.0) * assign94720_body17_e146628) + (assign94720_body17_e146617 * (-(((locals.var_chi_dn13 / 4.0) * assign94720_body17_e146626) + (assign94720_body17_e146621 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94720_body17_e146633;
            locals.var_t0_dn0 = assign94720_body17_e146633_d_n0;
            locals.var_t0_dn2 = assign94720_body17_e146633_d_n2;
            locals.var_t0_dn4 = assign94720_body17_e146633_d_n4;
            locals.var_t0_dn5 = assign94720_body17_e146633_d_n5;
            locals.var_t0_dn6 = assign94720_body17_e146633_d_n6;
            locals.var_t0_dn7 = assign94720_body17_e146633_d_n7;
            locals.var_t0_dn8 = assign94720_body17_e146633_d_n8;
            locals.var_t0_dn9 = assign94720_body17_e146633_d_n9;
            locals.var_t0_dn10 = assign94720_body17_e146633_d_n10;
            locals.var_t0_dn13 = assign94720_body17_e146633_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94720_body18_e146661, assign94720_body18_e146661_d_n0, assign94720_body18_e146661_d_n2, assign94720_body18_e146661_d_n4, assign94720_body18_e146661_d_n5, assign94720_body18_e146661_d_n6, assign94720_body18_e146661_d_n7, assign94720_body18_e146661_d_n8, assign94720_body18_e146661_d_n9, assign94720_body18_e146661_d_n10, assign94720_body18_e146661_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body18_e146645: f64 = (locals.var_chi / 2.0);
        let assign94720_body18_e146649: f64 = (locals.var_chi / 3.0);
        let assign94720_body18_e146653: f64 = (locals.var_chi / 4.0);
        let assign94720_body18_e146654: f64 = (1.0 - assign94720_body18_e146653);
        let assign94720_body18_e146655: f64 = (assign94720_body18_e146649 * assign94720_body18_e146654);
        let assign94720_body18_e146656: f64 = (1.0 - assign94720_body18_e146655);
        let assign94720_body18_e146657: f64 = (assign94720_body18_e146645 * assign94720_body18_e146656);
        let assign94720_body18_e146658: f64 = (1.0 - assign94720_body18_e146657);
        let assign94720_body18_e146659: f64 = (locals.var_chi * assign94720_body18_e146658);
        (assign94720_body18_e146659, ((locals.var_chi_dn0 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn0 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn2 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn4 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn5 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn6 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn7 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn8 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn9 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn10 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign94720_body18_e146658) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign94720_body18_e146656) + (assign94720_body18_e146645 * (-(((locals.var_chi_dn13 / 3.0) * assign94720_body18_e146654) + (assign94720_body18_e146649 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body18_e146661;
            locals.var_t1_dn0 = assign94720_body18_e146661_d_n0;
            locals.var_t1_dn2 = assign94720_body18_e146661_d_n2;
            locals.var_t1_dn4 = assign94720_body18_e146661_d_n4;
            locals.var_t1_dn5 = assign94720_body18_e146661_d_n5;
            locals.var_t1_dn6 = assign94720_body18_e146661_d_n6;
            locals.var_t1_dn7 = assign94720_body18_e146661_d_n7;
            locals.var_t1_dn8 = assign94720_body18_e146661_d_n8;
            locals.var_t1_dn9 = assign94720_body18_e146661_d_n9;
            locals.var_t1_dn10 = assign94720_body18_e146661_d_n10;
            locals.var_t1_dn13 = assign94720_body18_e146661_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94720_body19_e146693, assign94720_body19_e146693_d_n0, assign94720_body19_e146693_d_n2, assign94720_body19_e146693_d_n4, assign94720_body19_e146693_d_n5, assign94720_body19_e146693_d_n6, assign94720_body19_e146693_d_n7, assign94720_body19_e146693_d_n8, assign94720_body19_e146693_d_n9, assign94720_body19_e146693_d_n10, assign94720_body19_e146693_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body19_e146671: f64 = (locals.var_chib * locals.var_chib);
        let assign94720_body19_e146673: f64 = (assign94720_body19_e146671 / 2.0);
        let assign94720_body19_e146677: f64 = (locals.var_chib / 3.0);
        let assign94720_body19_e146681: f64 = (locals.var_chib / 4.0);
        let assign94720_body19_e146685: f64 = (locals.var_chib / 5.0);
        let assign94720_body19_e146686: f64 = (1.0 - assign94720_body19_e146685);
        let assign94720_body19_e146687: f64 = (assign94720_body19_e146681 * assign94720_body19_e146686);
        let assign94720_body19_e146688: f64 = (1.0 - assign94720_body19_e146687);
        let assign94720_body19_e146689: f64 = (assign94720_body19_e146677 * assign94720_body19_e146688);
        let assign94720_body19_e146690: f64 = (1.0 - assign94720_body19_e146689);
        let assign94720_body19_e146691: f64 = (assign94720_body19_e146673 * assign94720_body19_e146690);
        (assign94720_body19_e146691, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn0 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn0 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn2 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn2 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn4 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn4 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn5 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn5 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn6 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn6 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn7 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn7 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn8 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn8 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn9 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn9 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn10 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn10 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign94720_body19_e146690) + (assign94720_body19_e146673 * (-(((locals.var_chib_dn13 / 3.0) * assign94720_body19_e146688) + (assign94720_body19_e146677 * (-(((locals.var_chib_dn13 / 4.0) * assign94720_body19_e146686) + (assign94720_body19_e146681 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign94720_body19_e146693;
            locals.var_t2_dn0 = assign94720_body19_e146693_d_n0;
            locals.var_t2_dn2 = assign94720_body19_e146693_d_n2;
            locals.var_t2_dn4 = assign94720_body19_e146693_d_n4;
            locals.var_t2_dn5 = assign94720_body19_e146693_d_n5;
            locals.var_t2_dn6 = assign94720_body19_e146693_d_n6;
            locals.var_t2_dn7 = assign94720_body19_e146693_d_n7;
            locals.var_t2_dn8 = assign94720_body19_e146693_d_n8;
            locals.var_t2_dn9 = assign94720_body19_e146693_d_n9;
            locals.var_t2_dn10 = assign94720_body19_e146693_d_n10;
            locals.var_t2_dn13 = assign94720_body19_e146693_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign94720_body20_e146721, assign94720_body20_e146721_d_n0, assign94720_body20_e146721_d_n2, assign94720_body20_e146721_d_n4, assign94720_body20_e146721_d_n5, assign94720_body20_e146721_d_n6, assign94720_body20_e146721_d_n7, assign94720_body20_e146721_d_n8, assign94720_body20_e146721_d_n9, assign94720_body20_e146721_d_n10, assign94720_body20_e146721_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body20_e146705: f64 = (locals.var_chib / 2.0);
        let assign94720_body20_e146709: f64 = (locals.var_chib / 3.0);
        let assign94720_body20_e146713: f64 = (locals.var_chib / 4.0);
        let assign94720_body20_e146714: f64 = (1.0 - assign94720_body20_e146713);
        let assign94720_body20_e146715: f64 = (assign94720_body20_e146709 * assign94720_body20_e146714);
        let assign94720_body20_e146716: f64 = (1.0 - assign94720_body20_e146715);
        let assign94720_body20_e146717: f64 = (assign94720_body20_e146705 * assign94720_body20_e146716);
        let assign94720_body20_e146718: f64 = (1.0 - assign94720_body20_e146717);
        let assign94720_body20_e146719: f64 = (locals.var_chib * assign94720_body20_e146718);
        (assign94720_body20_e146719, ((locals.var_chib_dn0 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn0 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn2 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn4 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn5 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn6 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn7 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn8 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn9 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn10 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign94720_body20_e146718) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign94720_body20_e146716) + (assign94720_body20_e146705 * (-(((locals.var_chib_dn13 / 3.0) * assign94720_body20_e146714) + (assign94720_body20_e146709 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign94720_body20_e146721;
            locals.var_t3_dn0 = assign94720_body20_e146721_d_n0;
            locals.var_t3_dn2 = assign94720_body20_e146721_d_n2;
            locals.var_t3_dn4 = assign94720_body20_e146721_d_n4;
            locals.var_t3_dn5 = assign94720_body20_e146721_d_n5;
            locals.var_t3_dn6 = assign94720_body20_e146721_d_n6;
            locals.var_t3_dn7 = assign94720_body20_e146721_d_n7;
            locals.var_t3_dn8 = assign94720_body20_e146721_d_n8;
            locals.var_t3_dn9 = assign94720_body20_e146721_d_n9;
            locals.var_t3_dn10 = assign94720_body20_e146721_d_n10;
            locals.var_t3_dn13 = assign94720_body20_e146721_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign94720_body21_e146733, assign94720_body21_e146733_d_n0, assign94720_body21_e146733_d_n2, assign94720_body21_e146733_d_n4, assign94720_body21_e146733_d_n5, assign94720_body21_e146733_d_n6, assign94720_body21_e146733_d_n7, assign94720_body21_e146733_d_n8, assign94720_body21_e146733_d_n9, assign94720_body21_e146733_d_n10, assign94720_body21_e146733_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body21_e146731: f64 = (locals.var_t0 - locals.var_t2);
        (assign94720_body21_e146731, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq__blk2123, locals.var_fbsq__blk2123_dn0, locals.var_fbsq__blk2123_dn2, locals.var_fbsq__blk2123_dn4, locals.var_fbsq__blk2123_dn5, locals.var_fbsq__blk2123_dn6, locals.var_fbsq__blk2123_dn7, locals.var_fbsq__blk2123_dn8, locals.var_fbsq__blk2123_dn9, locals.var_fbsq__blk2123_dn10, locals.var_fbsq__blk2123_dn13,)
    }
};
            locals.var_fbsq__blk2123 = assign94720_body21_e146733;
            locals.var_fbsq__blk2123_dn0 = assign94720_body21_e146733_d_n0;
            locals.var_fbsq__blk2123_dn2 = assign94720_body21_e146733_d_n2;
            locals.var_fbsq__blk2123_dn4 = assign94720_body21_e146733_d_n4;
            locals.var_fbsq__blk2123_dn5 = assign94720_body21_e146733_d_n5;
            locals.var_fbsq__blk2123_dn6 = assign94720_body21_e146733_d_n6;
            locals.var_fbsq__blk2123_dn7 = assign94720_body21_e146733_d_n7;
            locals.var_fbsq__blk2123_dn8 = assign94720_body21_e146733_d_n8;
            locals.var_fbsq__blk2123_dn9 = assign94720_body21_e146733_d_n9;
            locals.var_fbsq__blk2123_dn10 = assign94720_body21_e146733_d_n10;
            locals.var_fbsq__blk2123_dn13 = assign94720_body21_e146733_d_n13;
            locals.var_fbsq__blk2123_rv = 0.0;
            let (assign94720_body22_e146749, assign94720_body22_e146749_d_n0, assign94720_body22_e146749_d_n2, assign94720_body22_e146749_d_n4, assign94720_body22_e146749_d_n5, assign94720_body22_e146749_d_n6, assign94720_body22_e146749_d_n7, assign94720_body22_e146749_d_n8, assign94720_body22_e146749_d_n9, assign94720_body22_e146749_d_n10, assign94720_body22_e146749_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 != 0.0)) {
        let assign94720_body22_e146745: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign94720_body22_e146746: f64 = (locals.var_t1 - assign94720_body22_e146745);
        let assign94720_body22_e146747: f64 = (locals.var_beta * assign94720_body22_e146746);
        (assign94720_body22_e146747, ((locals.var_beta_dn0 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign94720_body22_e146746) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss__blk2124, locals.var_fbsq_dpss__blk2124_dn0, locals.var_fbsq_dpss__blk2124_dn2, locals.var_fbsq_dpss__blk2124_dn4, locals.var_fbsq_dpss__blk2124_dn5, locals.var_fbsq_dpss__blk2124_dn6, locals.var_fbsq_dpss__blk2124_dn7, locals.var_fbsq_dpss__blk2124_dn8, locals.var_fbsq_dpss__blk2124_dn9, locals.var_fbsq_dpss__blk2124_dn10, locals.var_fbsq_dpss__blk2124_dn13,)
    }
};
            locals.var_fbsq_dpss__blk2124 = assign94720_body22_e146749;
            locals.var_fbsq_dpss__blk2124_dn0 = assign94720_body22_e146749_d_n0;
            locals.var_fbsq_dpss__blk2124_dn2 = assign94720_body22_e146749_d_n2;
            locals.var_fbsq_dpss__blk2124_dn4 = assign94720_body22_e146749_d_n4;
            locals.var_fbsq_dpss__blk2124_dn5 = assign94720_body22_e146749_d_n5;
            locals.var_fbsq_dpss__blk2124_dn6 = assign94720_body22_e146749_d_n6;
            locals.var_fbsq_dpss__blk2124_dn7 = assign94720_body22_e146749_d_n7;
            locals.var_fbsq_dpss__blk2124_dn8 = assign94720_body22_e146749_d_n8;
            locals.var_fbsq_dpss__blk2124_dn9 = assign94720_body22_e146749_d_n9;
            locals.var_fbsq_dpss__blk2124_dn10 = assign94720_body22_e146749_d_n10;
            locals.var_fbsq_dpss__blk2124_dn13 = assign94720_body22_e146749_d_n13;
            locals.var_fbsq_dpss__blk2124_rv = 0.0;
            let (assign94720_body24_e146781, assign94720_body24_e146781_d_n0, assign94720_body24_e146781_d_n2, assign94720_body24_e146781_d_n4, assign94720_body24_e146781_d_n5, assign94720_body24_e146781_d_n6, assign94720_body24_e146781_d_n7, assign94720_body24_e146781_d_n8, assign94720_body24_e146781_d_n9, assign94720_body24_e146781_d_n10, assign94720_body24_e146781_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 == 0.0)) {
        let assign94720_body24_e146778: f64 = (-locals.var_chi);
        let assign94720_body24_e146779: f64 = (assign94720_body24_e146778).exp();
        (assign94720_body24_e146779, (assign94720_body24_e146779 * (-locals.var_chi_dn0)), (assign94720_body24_e146779 * (-locals.var_chi_dn2)), (assign94720_body24_e146779 * (-locals.var_chi_dn4)), (assign94720_body24_e146779 * (-locals.var_chi_dn5)), (assign94720_body24_e146779 * (-locals.var_chi_dn6)), (assign94720_body24_e146779 * (-locals.var_chi_dn7)), (assign94720_body24_e146779 * (-locals.var_chi_dn8)), (assign94720_body24_e146779 * (-locals.var_chi_dn9)), (assign94720_body24_e146779 * (-locals.var_chi_dn10)), (assign94720_body24_e146779 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94720_body24_e146781;
            locals.var_t0_dn0 = assign94720_body24_e146781_d_n0;
            locals.var_t0_dn2 = assign94720_body24_e146781_d_n2;
            locals.var_t0_dn4 = assign94720_body24_e146781_d_n4;
            locals.var_t0_dn5 = assign94720_body24_e146781_d_n5;
            locals.var_t0_dn6 = assign94720_body24_e146781_d_n6;
            locals.var_t0_dn7 = assign94720_body24_e146781_d_n7;
            locals.var_t0_dn8 = assign94720_body24_e146781_d_n8;
            locals.var_t0_dn9 = assign94720_body24_e146781_d_n9;
            locals.var_t0_dn10 = assign94720_body24_e146781_d_n10;
            locals.var_t0_dn13 = assign94720_body24_e146781_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94720_body25_e146794, assign94720_body25_e146794_d_n0, assign94720_body25_e146794_d_n2, assign94720_body25_e146794_d_n4, assign94720_body25_e146794_d_n5, assign94720_body25_e146794_d_n6, assign94720_body25_e146794_d_n7, assign94720_body25_e146794_d_n8, assign94720_body25_e146794_d_n9, assign94720_body25_e146794_d_n10, assign94720_body25_e146794_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 == 0.0)) {
        let assign94720_body25_e146791: f64 = (-locals.var_chib);
        let assign94720_body25_e146792: f64 = (assign94720_body25_e146791).exp();
        (assign94720_body25_e146792, (assign94720_body25_e146792 * (-locals.var_chib_dn0)), (assign94720_body25_e146792 * (-locals.var_chib_dn2)), (assign94720_body25_e146792 * (-locals.var_chib_dn4)), (assign94720_body25_e146792 * (-locals.var_chib_dn5)), (assign94720_body25_e146792 * (-locals.var_chib_dn6)), (assign94720_body25_e146792 * (-locals.var_chib_dn7)), (assign94720_body25_e146792 * (-locals.var_chib_dn8)), (assign94720_body25_e146792 * (-locals.var_chib_dn9)), (assign94720_body25_e146792 * (-locals.var_chib_dn10)), (assign94720_body25_e146792 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body25_e146794;
            locals.var_t1_dn0 = assign94720_body25_e146794_d_n0;
            locals.var_t1_dn2 = assign94720_body25_e146794_d_n2;
            locals.var_t1_dn4 = assign94720_body25_e146794_d_n4;
            locals.var_t1_dn5 = assign94720_body25_e146794_d_n5;
            locals.var_t1_dn6 = assign94720_body25_e146794_d_n6;
            locals.var_t1_dn7 = assign94720_body25_e146794_d_n7;
            locals.var_t1_dn8 = assign94720_body25_e146794_d_n8;
            locals.var_t1_dn9 = assign94720_body25_e146794_d_n9;
            locals.var_t1_dn10 = assign94720_body25_e146794_d_n10;
            locals.var_t1_dn13 = assign94720_body25_e146794_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94720_body26_e146811, assign94720_body26_e146811_d_n0, assign94720_body26_e146811_d_n2, assign94720_body26_e146811_d_n4, assign94720_body26_e146811_d_n5, assign94720_body26_e146811_d_n6, assign94720_body26_e146811_d_n7, assign94720_body26_e146811_d_n8, assign94720_body26_e146811_d_n9, assign94720_body26_e146811_d_n10, assign94720_body26_e146811_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 == 0.0)) {
        let assign94720_body26_e146805: f64 = (locals.var_chi - locals.var_chib);
        let assign94720_body26_e146808: f64 = (locals.var_t0 - locals.var_t1);
        let assign94720_body26_e146809: f64 = (assign94720_body26_e146805 + assign94720_body26_e146808);
        (assign94720_body26_e146809, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq__blk2123, locals.var_fbsq__blk2123_dn0, locals.var_fbsq__blk2123_dn2, locals.var_fbsq__blk2123_dn4, locals.var_fbsq__blk2123_dn5, locals.var_fbsq__blk2123_dn6, locals.var_fbsq__blk2123_dn7, locals.var_fbsq__blk2123_dn8, locals.var_fbsq__blk2123_dn9, locals.var_fbsq__blk2123_dn10, locals.var_fbsq__blk2123_dn13,)
    }
};
            locals.var_fbsq__blk2123 = assign94720_body26_e146811;
            locals.var_fbsq__blk2123_dn0 = assign94720_body26_e146811_d_n0;
            locals.var_fbsq__blk2123_dn2 = assign94720_body26_e146811_d_n2;
            locals.var_fbsq__blk2123_dn4 = assign94720_body26_e146811_d_n4;
            locals.var_fbsq__blk2123_dn5 = assign94720_body26_e146811_d_n5;
            locals.var_fbsq__blk2123_dn6 = assign94720_body26_e146811_d_n6;
            locals.var_fbsq__blk2123_dn7 = assign94720_body26_e146811_d_n7;
            locals.var_fbsq__blk2123_dn8 = assign94720_body26_e146811_d_n8;
            locals.var_fbsq__blk2123_dn9 = assign94720_body26_e146811_d_n9;
            locals.var_fbsq__blk2123_dn10 = assign94720_body26_e146811_d_n10;
            locals.var_fbsq__blk2123_dn13 = assign94720_body26_e146811_d_n13;
            locals.var_fbsq__blk2123_rv = 0.0;
            let (assign94720_body27_e146832, assign94720_body27_e146832_d_n0, assign94720_body27_e146832_d_n2, assign94720_body27_e146832_d_n4, assign94720_body27_e146832_d_n5, assign94720_body27_e146832_d_n6, assign94720_body27_e146832_d_n7, assign94720_body27_e146832_d_n8, assign94720_body27_e146832_d_n9, assign94720_body27_e146832_d_n10, assign94720_body27_e146832_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2202 == 0.0)) {
        let assign94720_body27_e146823: f64 = (1.0 - locals.var_t0);
        let assign94720_body27_e146827: f64 = (1.0 - locals.var_t1);
        let assign94720_body27_e146828: f64 = (locals.var_phi_b_dpss * assign94720_body27_e146827);
        let assign94720_body27_e146829: f64 = (assign94720_body27_e146823 - assign94720_body27_e146828);
        let assign94720_body27_e146830: f64 = (locals.var_beta * assign94720_body27_e146829);
        (assign94720_body27_e146830, ((locals.var_beta_dn0 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign94720_body27_e146829) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign94720_body27_e146827) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss__blk2124, locals.var_fbsq_dpss__blk2124_dn0, locals.var_fbsq_dpss__blk2124_dn2, locals.var_fbsq_dpss__blk2124_dn4, locals.var_fbsq_dpss__blk2124_dn5, locals.var_fbsq_dpss__blk2124_dn6, locals.var_fbsq_dpss__blk2124_dn7, locals.var_fbsq_dpss__blk2124_dn8, locals.var_fbsq_dpss__blk2124_dn9, locals.var_fbsq_dpss__blk2124_dn10, locals.var_fbsq_dpss__blk2124_dn13,)
    }
};
            locals.var_fbsq_dpss__blk2124 = assign94720_body27_e146832;
            locals.var_fbsq_dpss__blk2124_dn0 = assign94720_body27_e146832_d_n0;
            locals.var_fbsq_dpss__blk2124_dn2 = assign94720_body27_e146832_d_n2;
            locals.var_fbsq_dpss__blk2124_dn4 = assign94720_body27_e146832_d_n4;
            locals.var_fbsq_dpss__blk2124_dn5 = assign94720_body27_e146832_d_n5;
            locals.var_fbsq_dpss__blk2124_dn6 = assign94720_body27_e146832_d_n6;
            locals.var_fbsq_dpss__blk2124_dn7 = assign94720_body27_e146832_d_n7;
            locals.var_fbsq_dpss__blk2124_dn8 = assign94720_body27_e146832_d_n8;
            locals.var_fbsq_dpss__blk2124_dn9 = assign94720_body27_e146832_d_n9;
            locals.var_fbsq_dpss__blk2124_dn10 = assign94720_body27_e146832_d_n10;
            locals.var_fbsq_dpss__blk2124_dn13 = assign94720_body27_e146832_d_n13;
            locals.var_fbsq_dpss__blk2124_rv = 0.0;
            let assign94720_body28_e146834: f64 = (locals.var_chi).abs();
            let assign94720_body28_e146836: f64 = if assign94720_body28_e146834 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2203 = assign94720_body28_e146836;
            locals.var_guard2203_rv = 0.0;
            let (assign94720_body29_e146868, assign94720_body29_e146868_d_n0, assign94720_body29_e146868_d_n2, assign94720_body29_e146868_d_n4, assign94720_body29_e146868_d_n5, assign94720_body29_e146868_d_n6, assign94720_body29_e146868_d_n7, assign94720_body29_e146868_d_n8, assign94720_body29_e146868_d_n9, assign94720_body29_e146868_d_n10, assign94720_body29_e146868_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94720_body29_e146846: f64 = (locals.var_chi * locals.var_chi);
        let assign94720_body29_e146848: f64 = (assign94720_body29_e146846 / 2.0);
        let assign94720_body29_e146852: f64 = (locals.var_chi / 3.0);
        let assign94720_body29_e146856: f64 = (locals.var_chi / 4.0);
        let assign94720_body29_e146860: f64 = (locals.var_chi / 5.0);
        let assign94720_body29_e146861: f64 = (1.0 + assign94720_body29_e146860);
        let assign94720_body29_e146862: f64 = (assign94720_body29_e146856 * assign94720_body29_e146861);
        let assign94720_body29_e146863: f64 = (1.0 + assign94720_body29_e146862);
        let assign94720_body29_e146864: f64 = (assign94720_body29_e146852 * assign94720_body29_e146863);
        let assign94720_body29_e146865: f64 = (1.0 + assign94720_body29_e146864);
        let assign94720_body29_e146866: f64 = (assign94720_body29_e146848 * assign94720_body29_e146865);
        (assign94720_body29_e146866, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn0 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn0 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn2 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn2 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn4 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn4 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn5 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn5 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn6 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn6 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn7 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn7 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn8 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn8 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn9 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn9 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn10 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn10 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign94720_body29_e146865) + (assign94720_body29_e146848 * (((locals.var_chi_dn13 / 3.0) * assign94720_body29_e146863) + (assign94720_body29_e146852 * (((locals.var_chi_dn13 / 4.0) * assign94720_body29_e146861) + (assign94720_body29_e146856 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign94720_body29_e146868;
            locals.var_t0_dn0 = assign94720_body29_e146868_d_n0;
            locals.var_t0_dn2 = assign94720_body29_e146868_d_n2;
            locals.var_t0_dn4 = assign94720_body29_e146868_d_n4;
            locals.var_t0_dn5 = assign94720_body29_e146868_d_n5;
            locals.var_t0_dn6 = assign94720_body29_e146868_d_n6;
            locals.var_t0_dn7 = assign94720_body29_e146868_d_n7;
            locals.var_t0_dn8 = assign94720_body29_e146868_d_n8;
            locals.var_t0_dn9 = assign94720_body29_e146868_d_n9;
            locals.var_t0_dn10 = assign94720_body29_e146868_d_n10;
            locals.var_t0_dn13 = assign94720_body29_e146868_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign94720_body30_e146896, assign94720_body30_e146896_d_n0, assign94720_body30_e146896_d_n2, assign94720_body30_e146896_d_n4, assign94720_body30_e146896_d_n5, assign94720_body30_e146896_d_n6, assign94720_body30_e146896_d_n7, assign94720_body30_e146896_d_n8, assign94720_body30_e146896_d_n9, assign94720_body30_e146896_d_n10, assign94720_body30_e146896_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94720_body30_e146880: f64 = (locals.var_chi / 2.0);
        let assign94720_body30_e146884: f64 = (locals.var_chi / 3.0);
        let assign94720_body30_e146888: f64 = (locals.var_chi / 4.0);
        let assign94720_body30_e146889: f64 = (1.0 + assign94720_body30_e146888);
        let assign94720_body30_e146890: f64 = (assign94720_body30_e146884 * assign94720_body30_e146889);
        let assign94720_body30_e146891: f64 = (1.0 + assign94720_body30_e146890);
        let assign94720_body30_e146892: f64 = (assign94720_body30_e146880 * assign94720_body30_e146891);
        let assign94720_body30_e146893: f64 = (1.0 + assign94720_body30_e146892);
        let assign94720_body30_e146894: f64 = (locals.var_chi * assign94720_body30_e146893);
        (assign94720_body30_e146894, ((locals.var_chi_dn0 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn0 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn2 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn4 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn5 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn6 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn7 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn8 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn9 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn10 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign94720_body30_e146893) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign94720_body30_e146891) + (assign94720_body30_e146880 * (((locals.var_chi_dn13 / 3.0) * assign94720_body30_e146889) + (assign94720_body30_e146884 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body30_e146896;
            locals.var_t1_dn0 = assign94720_body30_e146896_d_n0;
            locals.var_t1_dn2 = assign94720_body30_e146896_d_n2;
            locals.var_t1_dn4 = assign94720_body30_e146896_d_n4;
            locals.var_t1_dn5 = assign94720_body30_e146896_d_n5;
            locals.var_t1_dn6 = assign94720_body30_e146896_d_n6;
            locals.var_t1_dn7 = assign94720_body30_e146896_d_n7;
            locals.var_t1_dn8 = assign94720_body30_e146896_d_n8;
            locals.var_t1_dn9 = assign94720_body30_e146896_d_n9;
            locals.var_t1_dn10 = assign94720_body30_e146896_d_n10;
            locals.var_t1_dn13 = assign94720_body30_e146896_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94720_body31_e146908, assign94720_body31_e146908_d_n0, assign94720_body31_e146908_d_n2, assign94720_body31_e146908_d_n4, assign94720_body31_e146908_d_n5, assign94720_body31_e146908_d_n6, assign94720_body31_e146908_d_n7, assign94720_body31_e146908_d_n8, assign94720_body31_e146908_d_n9, assign94720_body31_e146908_d_n10, assign94720_body31_e146908_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94720_body31_e146906: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign94720_body31_e146906, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94720_body31_e146908;
            locals.var_fs01_dn0 = assign94720_body31_e146908_d_n0;
            locals.var_fs01_dn2 = assign94720_body31_e146908_d_n2;
            locals.var_fs01_dn4 = assign94720_body31_e146908_d_n4;
            locals.var_fs01_dn5 = assign94720_body31_e146908_d_n5;
            locals.var_fs01_dn6 = assign94720_body31_e146908_d_n6;
            locals.var_fs01_dn7 = assign94720_body31_e146908_d_n7;
            locals.var_fs01_dn8 = assign94720_body31_e146908_d_n8;
            locals.var_fs01_dn9 = assign94720_body31_e146908_d_n9;
            locals.var_fs01_dn10 = assign94720_body31_e146908_d_n10;
            locals.var_fs01_dn13 = assign94720_body31_e146908_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign94720_body32_e146922, assign94720_body32_e146922_d_n0, assign94720_body32_e146922_d_n2, assign94720_body32_e146922_d_n4, assign94720_body32_e146922_d_n5, assign94720_body32_e146922_d_n6, assign94720_body32_e146922_d_n7, assign94720_body32_e146922_d_n8, assign94720_body32_e146922_d_n9, assign94720_body32_e146922_d_n10, assign94720_body32_e146922_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 != 0.0)) {
        let assign94720_body32_e146918: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign94720_body32_e146920: f64 = (assign94720_body32_e146918 * locals.var_beta);
        (assign94720_body32_e146920, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign94720_body32_e146918 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94720_body32_e146922;
            locals.var_fs01_dps0_dn0 = assign94720_body32_e146922_d_n0;
            locals.var_fs01_dps0_dn2 = assign94720_body32_e146922_d_n2;
            locals.var_fs01_dps0_dn4 = assign94720_body32_e146922_d_n4;
            locals.var_fs01_dps0_dn5 = assign94720_body32_e146922_d_n5;
            locals.var_fs01_dps0_dn6 = assign94720_body32_e146922_d_n6;
            locals.var_fs01_dps0_dn7 = assign94720_body32_e146922_d_n7;
            locals.var_fs01_dps0_dn8 = assign94720_body32_e146922_d_n8;
            locals.var_fs01_dps0_dn9 = assign94720_body32_e146922_d_n9;
            locals.var_fs01_dps0_dn10 = assign94720_body32_e146922_d_n10;
            locals.var_fs01_dps0_dn13 = assign94720_body32_e146922_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94720_body33_e146924: f64 = (locals.var_chi).abs();
            let assign94720_body33_e146926: f64 = if assign94720_body33_e146924 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2204 = assign94720_body33_e146926;
            locals.var_guard2204_rv = 0.0;
            let (assign94720_body35_e146961, assign94720_body35_e146961_d_n0, assign94720_body35_e146961_d_n2, assign94720_body35_e146961_d_n4, assign94720_body35_e146961_d_n5, assign94720_body35_e146961_d_n6, assign94720_body35_e146961_d_n7, assign94720_body35_e146961_d_n8, assign94720_body35_e146961_d_n9, assign94720_body35_e146961_d_n10, assign94720_body35_e146961_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94720_body35_e146959: f64 = (locals.var_chi).exp();
        (assign94720_body35_e146959, (assign94720_body35_e146959 * locals.var_chi_dn0), (assign94720_body35_e146959 * locals.var_chi_dn2), (assign94720_body35_e146959 * locals.var_chi_dn4), (assign94720_body35_e146959 * locals.var_chi_dn5), (assign94720_body35_e146959 * locals.var_chi_dn6), (assign94720_body35_e146959 * locals.var_chi_dn7), (assign94720_body35_e146959 * locals.var_chi_dn8), (assign94720_body35_e146959 * locals.var_chi_dn9), (assign94720_body35_e146959 * locals.var_chi_dn10), (assign94720_body35_e146959 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign94720_body35_e146961;
            locals.var_exp_chi_dn0 = assign94720_body35_e146961_d_n0;
            locals.var_exp_chi_dn2 = assign94720_body35_e146961_d_n2;
            locals.var_exp_chi_dn4 = assign94720_body35_e146961_d_n4;
            locals.var_exp_chi_dn5 = assign94720_body35_e146961_d_n5;
            locals.var_exp_chi_dn6 = assign94720_body35_e146961_d_n6;
            locals.var_exp_chi_dn7 = assign94720_body35_e146961_d_n7;
            locals.var_exp_chi_dn8 = assign94720_body35_e146961_d_n8;
            locals.var_exp_chi_dn9 = assign94720_body35_e146961_d_n9;
            locals.var_exp_chi_dn10 = assign94720_body35_e146961_d_n10;
            locals.var_exp_chi_dn13 = assign94720_body35_e146961_d_n13;
            locals.var_exp_chi_rv = 0.0;
            let (assign94720_body36_e146976, assign94720_body36_e146976_d_n0, assign94720_body36_e146976_d_n2, assign94720_body36_e146976_d_n4, assign94720_body36_e146976_d_n5, assign94720_body36_e146976_d_n6, assign94720_body36_e146976_d_n7, assign94720_body36_e146976_d_n8, assign94720_body36_e146976_d_n9, assign94720_body36_e146976_d_n10, assign94720_body36_e146976_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94720_body36_e146974: f64 = (locals.var_exp_chi - 1.0);
        (assign94720_body36_e146974, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign94720_body36_e146976;
            locals.var_t1_dn0 = assign94720_body36_e146976_d_n0;
            locals.var_t1_dn2 = assign94720_body36_e146976_d_n2;
            locals.var_t1_dn4 = assign94720_body36_e146976_d_n4;
            locals.var_t1_dn5 = assign94720_body36_e146976_d_n5;
            locals.var_t1_dn6 = assign94720_body36_e146976_d_n6;
            locals.var_t1_dn7 = assign94720_body36_e146976_d_n7;
            locals.var_t1_dn8 = assign94720_body36_e146976_d_n8;
            locals.var_t1_dn9 = assign94720_body36_e146976_d_n9;
            locals.var_t1_dn10 = assign94720_body36_e146976_d_n10;
            locals.var_t1_dn13 = assign94720_body36_e146976_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign94720_body37_e146993, assign94720_body37_e146993_d_n0, assign94720_body37_e146993_d_n2, assign94720_body37_e146993_d_n4, assign94720_body37_e146993_d_n5, assign94720_body37_e146993_d_n6, assign94720_body37_e146993_d_n7, assign94720_body37_e146993_d_n8, assign94720_body37_e146993_d_n9, assign94720_body37_e146993_d_n10, assign94720_body37_e146993_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94720_body37_e146990: f64 = (locals.var_t1 - locals.var_chi);
        let assign94720_body37_e146991: f64 = (locals.var_cfs1 * assign94720_body37_e146990);
        (assign94720_body37_e146991, ((locals.var_cfs1_dn0 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign94720_body37_e146990) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94720_body37_e146993;
            locals.var_fs01_dn0 = assign94720_body37_e146993_d_n0;
            locals.var_fs01_dn2 = assign94720_body37_e146993_d_n2;
            locals.var_fs01_dn4 = assign94720_body37_e146993_d_n4;
            locals.var_fs01_dn5 = assign94720_body37_e146993_d_n5;
            locals.var_fs01_dn6 = assign94720_body37_e146993_d_n6;
            locals.var_fs01_dn7 = assign94720_body37_e146993_d_n7;
            locals.var_fs01_dn8 = assign94720_body37_e146993_d_n8;
            locals.var_fs01_dn9 = assign94720_body37_e146993_d_n9;
            locals.var_fs01_dn10 = assign94720_body37_e146993_d_n10;
            locals.var_fs01_dn13 = assign94720_body37_e146993_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign94720_body38_e147010, assign94720_body38_e147010_d_n0, assign94720_body38_e147010_d_n2, assign94720_body38_e147010_d_n4, assign94720_body38_e147010_d_n5, assign94720_body38_e147010_d_n6, assign94720_body38_e147010_d_n7, assign94720_body38_e147010_d_n8, assign94720_body38_e147010_d_n9, assign94720_body38_e147010_d_n10, assign94720_body38_e147010_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 != 0.0)) {
        let assign94720_body38_e147006: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign94720_body38_e147008: f64 = (assign94720_body38_e147006 * locals.var_t1);
        (assign94720_body38_e147008, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign94720_body38_e147006 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94720_body38_e147010;
            locals.var_fs01_dps0_dn0 = assign94720_body38_e147010_d_n0;
            locals.var_fs01_dps0_dn2 = assign94720_body38_e147010_d_n2;
            locals.var_fs01_dps0_dn4 = assign94720_body38_e147010_d_n4;
            locals.var_fs01_dps0_dn5 = assign94720_body38_e147010_d_n5;
            locals.var_fs01_dps0_dn6 = assign94720_body38_e147010_d_n6;
            locals.var_fs01_dps0_dn7 = assign94720_body38_e147010_d_n7;
            locals.var_fs01_dps0_dn8 = assign94720_body38_e147010_d_n8;
            locals.var_fs01_dps0_dn9 = assign94720_body38_e147010_d_n9;
            locals.var_fs01_dps0_dn10 = assign94720_body38_e147010_d_n10;
            locals.var_fs01_dps0_dn13 = assign94720_body38_e147010_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign94720_body40_e147049, assign94720_body40_e147049_d_n0, assign94720_body40_e147049_d_n2, assign94720_body40_e147049_d_n4, assign94720_body40_e147049_d_n5, assign94720_body40_e147049_d_n6, assign94720_body40_e147049_d_n7, assign94720_body40_e147049_d_n8, assign94720_body40_e147049_d_n9, assign94720_body40_e147049_d_n10, assign94720_body40_e147049_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94720_body40_e147046: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign94720_body40_e147047: f64 = (assign94720_body40_e147046).exp();
        (assign94720_body40_e147047, (assign94720_body40_e147047 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign94720_body40_e147047 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign94720_body40_e147047 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign94720_body40_e147047 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign94720_body40_e147047 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign94720_body40_e147047 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign94720_body40_e147047 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign94720_body40_e147047 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign94720_body40_e147047 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign94720_body40_e147047 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign94720_body40_e147049;
            locals.var_exp_bps0_dn0 = assign94720_body40_e147049_d_n0;
            locals.var_exp_bps0_dn2 = assign94720_body40_e147049_d_n2;
            locals.var_exp_bps0_dn4 = assign94720_body40_e147049_d_n4;
            locals.var_exp_bps0_dn5 = assign94720_body40_e147049_d_n5;
            locals.var_exp_bps0_dn6 = assign94720_body40_e147049_d_n6;
            locals.var_exp_bps0_dn7 = assign94720_body40_e147049_d_n7;
            locals.var_exp_bps0_dn8 = assign94720_body40_e147049_d_n8;
            locals.var_exp_bps0_dn9 = assign94720_body40_e147049_d_n9;
            locals.var_exp_bps0_dn10 = assign94720_body40_e147049_d_n10;
            locals.var_exp_bps0_dn13 = assign94720_body40_e147049_d_n13;
            locals.var_exp_bps0_rv = 0.0;
            let (assign94720_body41_e147071, assign94720_body41_e147071_d_n0, assign94720_body41_e147071_d_n2, assign94720_body41_e147071_d_n4, assign94720_body41_e147071_d_n5, assign94720_body41_e147071_d_n6, assign94720_body41_e147071_d_n7, assign94720_body41_e147071_d_n8, assign94720_body41_e147071_d_n9, assign94720_body41_e147071_d_n10, assign94720_body41_e147071_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94720_body41_e147066: f64 = (locals.var_chi + 1.0);
        let assign94720_body41_e147067: f64 = (locals.var_exp_bvbs * assign94720_body41_e147066);
        let assign94720_body41_e147068: f64 = (locals.var_exp_bps0 - assign94720_body41_e147067);
        let assign94720_body41_e147069: f64 = (locals.var_cnst1over * assign94720_body41_e147068);
        (assign94720_body41_e147069, ((locals.var_cnst1over_dn0 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign94720_body41_e147068) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign94720_body41_e147066) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign94720_body41_e147071;
            locals.var_fs01_dn0 = assign94720_body41_e147071_d_n0;
            locals.var_fs01_dn2 = assign94720_body41_e147071_d_n2;
            locals.var_fs01_dn4 = assign94720_body41_e147071_d_n4;
            locals.var_fs01_dn5 = assign94720_body41_e147071_d_n5;
            locals.var_fs01_dn6 = assign94720_body41_e147071_d_n6;
            locals.var_fs01_dn7 = assign94720_body41_e147071_d_n7;
            locals.var_fs01_dn8 = assign94720_body41_e147071_d_n8;
            locals.var_fs01_dn9 = assign94720_body41_e147071_d_n9;
            locals.var_fs01_dn10 = assign94720_body41_e147071_d_n10;
            locals.var_fs01_dn13 = assign94720_body41_e147071_d_n13;
            locals.var_fs01_rv = 0.0;
            let (assign94720_body42_e147091, assign94720_body42_e147091_d_n0, assign94720_body42_e147091_d_n2, assign94720_body42_e147091_d_n4, assign94720_body42_e147091_d_n5, assign94720_body42_e147091_d_n6, assign94720_body42_e147091_d_n7, assign94720_body42_e147091_d_n8, assign94720_body42_e147091_d_n9, assign94720_body42_e147091_d_n10, assign94720_body42_e147091_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2203 == 0.0)) && (locals.var_guard2204 == 0.0)) {
        let assign94720_body42_e147085: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign94720_body42_e147088: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign94720_body42_e147089: f64 = (assign94720_body42_e147085 * assign94720_body42_e147088);
        (assign94720_body42_e147089, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign94720_body42_e147088) + (assign94720_body42_e147085 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign94720_body42_e147091;
            locals.var_fs01_dps0_dn0 = assign94720_body42_e147091_d_n0;
            locals.var_fs01_dps0_dn2 = assign94720_body42_e147091_d_n2;
            locals.var_fs01_dps0_dn4 = assign94720_body42_e147091_d_n4;
            locals.var_fs01_dps0_dn5 = assign94720_body42_e147091_d_n5;
            locals.var_fs01_dps0_dn6 = assign94720_body42_e147091_d_n6;
            locals.var_fs01_dps0_dn7 = assign94720_body42_e147091_d_n7;
            locals.var_fs01_dps0_dn8 = assign94720_body42_e147091_d_n8;
            locals.var_fs01_dps0_dn9 = assign94720_body42_e147091_d_n9;
            locals.var_fs01_dps0_dn10 = assign94720_body42_e147091_d_n10;
            locals.var_fs01_dps0_dn13 = assign94720_body42_e147091_d_n13;
            locals.var_fs01_dps0_rv = 0.0;
            let assign94720_body43_e147094: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2205 = assign94720_body43_e147094;
            locals.var_guard2205_rv = 0.0;
            let (assign94720_body44_e147107, assign94720_body44_e147107_d_n0, assign94720_body44_e147107_d_n2, assign94720_body44_e147107_d_n4, assign94720_body44_e147107_d_n5, assign94720_body44_e147107_d_n6, assign94720_body44_e147107_d_n7, assign94720_body44_e147107_d_n8, assign94720_body44_e147107_d_n9, assign94720_body44_e147107_d_n10, assign94720_body44_e147107_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94720_body44_e147104: f64 = (locals.var_fbsq__blk2123 + locals.var_fs01);
        let assign94720_body44_e147105: f64 = (assign94720_body44_e147104).sqrt();
        (assign94720_body44_e147105, ((locals.var_fbsq__blk2123_dn0 + locals.var_fs01_dn0) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn2 + locals.var_fs01_dn2) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn4 + locals.var_fs01_dn4) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn5 + locals.var_fs01_dn5) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn6 + locals.var_fs01_dn6) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn7 + locals.var_fs01_dn7) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn8 + locals.var_fs01_dn8) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn9 + locals.var_fs01_dn9) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn10 + locals.var_fs01_dn10) / (2.0 * assign94720_body44_e147105)), ((locals.var_fbsq__blk2123_dn13 + locals.var_fs01_dn13) / (2.0 * assign94720_body44_e147105)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94720_body44_e147107;
            locals.var_fs02_dn0 = assign94720_body44_e147107_d_n0;
            locals.var_fs02_dn2 = assign94720_body44_e147107_d_n2;
            locals.var_fs02_dn4 = assign94720_body44_e147107_d_n4;
            locals.var_fs02_dn5 = assign94720_body44_e147107_d_n5;
            locals.var_fs02_dn6 = assign94720_body44_e147107_d_n6;
            locals.var_fs02_dn7 = assign94720_body44_e147107_d_n7;
            locals.var_fs02_dn8 = assign94720_body44_e147107_d_n8;
            locals.var_fs02_dn9 = assign94720_body44_e147107_d_n9;
            locals.var_fs02_dn10 = assign94720_body44_e147107_d_n10;
            locals.var_fs02_dn13 = assign94720_body44_e147107_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign94720_body45_e147123, assign94720_body45_e147123_d_n0, assign94720_body45_e147123_d_n2, assign94720_body45_e147123_d_n4, assign94720_body45_e147123_d_n5, assign94720_body45_e147123_d_n6, assign94720_body45_e147123_d_n7, assign94720_body45_e147123_d_n8, assign94720_body45_e147123_d_n9, assign94720_body45_e147123_d_n10, assign94720_body45_e147123_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 != 0.0)) {
        let assign94720_body45_e147118: f64 = (locals.var_fbsq_dpss__blk2124 + locals.var_fs01_dps0);
        let assign94720_body45_e147119: f64 = (0.5 * assign94720_body45_e147118);
        let assign94720_body45_e147121: f64 = (assign94720_body45_e147119 / locals.var_fs02);
        (assign94720_body45_e147121, ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2124_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign94720_body45_e147119 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94720_body45_e147123;
            locals.var_fs02_dps0_dn0 = assign94720_body45_e147123_d_n0;
            locals.var_fs02_dps0_dn2 = assign94720_body45_e147123_d_n2;
            locals.var_fs02_dps0_dn4 = assign94720_body45_e147123_d_n4;
            locals.var_fs02_dps0_dn5 = assign94720_body45_e147123_d_n5;
            locals.var_fs02_dps0_dn6 = assign94720_body45_e147123_d_n6;
            locals.var_fs02_dps0_dn7 = assign94720_body45_e147123_d_n7;
            locals.var_fs02_dps0_dn8 = assign94720_body45_e147123_d_n8;
            locals.var_fs02_dps0_dn9 = assign94720_body45_e147123_d_n9;
            locals.var_fs02_dps0_dn10 = assign94720_body45_e147123_d_n10;
            locals.var_fs02_dps0_dn13 = assign94720_body45_e147123_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let assign94720_body46_e147126: f64 = if locals.var_fbsq__blk2123 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2206 = assign94720_body46_e147126;
            locals.var_guard2206_rv = 0.0;
            let (assign94720_body47_e147140, assign94720_body47_e147140_d_n0, assign94720_body47_e147140_d_n2, assign94720_body47_e147140_d_n4, assign94720_body47_e147140_d_n5, assign94720_body47_e147140_d_n6, assign94720_body47_e147140_d_n7, assign94720_body47_e147140_d_n8, assign94720_body47_e147140_d_n9, assign94720_body47_e147140_d_n10, assign94720_body47_e147140_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94720_body47_e147138: f64 = (locals.var_fbsq__blk2123).sqrt();
        (assign94720_body47_e147138, (locals.var_fbsq__blk2123_dn0 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn2 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn4 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn5 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn6 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn7 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn8 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn9 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn10 / (2.0 * assign94720_body47_e147138)), (locals.var_fbsq__blk2123_dn13 / (2.0 * assign94720_body47_e147138)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94720_body47_e147140;
            locals.var_fs02_dn0 = assign94720_body47_e147140_d_n0;
            locals.var_fs02_dn2 = assign94720_body47_e147140_d_n2;
            locals.var_fs02_dn4 = assign94720_body47_e147140_d_n4;
            locals.var_fs02_dn5 = assign94720_body47_e147140_d_n5;
            locals.var_fs02_dn6 = assign94720_body47_e147140_d_n6;
            locals.var_fs02_dn7 = assign94720_body47_e147140_d_n7;
            locals.var_fs02_dn8 = assign94720_body47_e147140_d_n8;
            locals.var_fs02_dn9 = assign94720_body47_e147140_d_n9;
            locals.var_fs02_dn10 = assign94720_body47_e147140_d_n10;
            locals.var_fs02_dn13 = assign94720_body47_e147140_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign94720_body48_e147157, assign94720_body48_e147157_d_n0, assign94720_body48_e147157_d_n2, assign94720_body48_e147157_d_n4, assign94720_body48_e147157_d_n5, assign94720_body48_e147157_d_n6, assign94720_body48_e147157_d_n7, assign94720_body48_e147157_d_n8, assign94720_body48_e147157_d_n9, assign94720_body48_e147157_d_n10, assign94720_body48_e147157_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 != 0.0)) {
        let assign94720_body48_e147153: f64 = (0.5 * locals.var_fbsq_dpss__blk2124);
        let assign94720_body48_e147155: f64 = (assign94720_body48_e147153 / locals.var_fs02);
        (assign94720_body48_e147155, ((((0.5 * locals.var_fbsq_dpss__blk2124_dn0) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn2) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn4) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn5) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn6) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn7) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn8) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn9) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn10) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2124_dn13) * locals.var_fs02) - (assign94720_body48_e147153 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94720_body48_e147157;
            locals.var_fs02_dps0_dn0 = assign94720_body48_e147157_d_n0;
            locals.var_fs02_dps0_dn2 = assign94720_body48_e147157_d_n2;
            locals.var_fs02_dps0_dn4 = assign94720_body48_e147157_d_n4;
            locals.var_fs02_dps0_dn5 = assign94720_body48_e147157_d_n5;
            locals.var_fs02_dps0_dn6 = assign94720_body48_e147157_d_n6;
            locals.var_fs02_dps0_dn7 = assign94720_body48_e147157_d_n7;
            locals.var_fs02_dps0_dn8 = assign94720_body48_e147157_d_n8;
            locals.var_fs02_dps0_dn9 = assign94720_body48_e147157_d_n9;
            locals.var_fs02_dps0_dn10 = assign94720_body48_e147157_d_n10;
            locals.var_fs02_dps0_dn13 = assign94720_body48_e147157_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94720_body49_e147171, assign94720_body49_e147171_d_n0, assign94720_body49_e147171_d_n2, assign94720_body49_e147171_d_n4, assign94720_body49_e147171_d_n5, assign94720_body49_e147171_d_n6, assign94720_body49_e147171_d_n7, assign94720_body49_e147171_d_n8, assign94720_body49_e147171_d_n9, assign94720_body49_e147171_d_n10, assign94720_body49_e147171_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94720_body49_e147171;
            locals.var_fs02_dn0 = assign94720_body49_e147171_d_n0;
            locals.var_fs02_dn2 = assign94720_body49_e147171_d_n2;
            locals.var_fs02_dn4 = assign94720_body49_e147171_d_n4;
            locals.var_fs02_dn5 = assign94720_body49_e147171_d_n5;
            locals.var_fs02_dn6 = assign94720_body49_e147171_d_n6;
            locals.var_fs02_dn7 = assign94720_body49_e147171_d_n7;
            locals.var_fs02_dn8 = assign94720_body49_e147171_d_n8;
            locals.var_fs02_dn9 = assign94720_body49_e147171_d_n9;
            locals.var_fs02_dn10 = assign94720_body49_e147171_d_n10;
            locals.var_fs02_dn13 = assign94720_body49_e147171_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign94720_body50_e147185, assign94720_body50_e147185_d_n0, assign94720_body50_e147185_d_n2, assign94720_body50_e147185_d_n4, assign94720_body50_e147185_d_n5, assign94720_body50_e147185_d_n6, assign94720_body50_e147185_d_n7, assign94720_body50_e147185_d_n8, assign94720_body50_e147185_d_n9, assign94720_body50_e147185_d_n10, assign94720_body50_e147185_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2205 == 0.0)) && (locals.var_guard2206 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94720_body50_e147185;
            locals.var_fs02_dps0_dn0 = assign94720_body50_e147185_d_n0;
            locals.var_fs02_dps0_dn2 = assign94720_body50_e147185_d_n2;
            locals.var_fs02_dps0_dn4 = assign94720_body50_e147185_d_n4;
            locals.var_fs02_dps0_dn5 = assign94720_body50_e147185_d_n5;
            locals.var_fs02_dps0_dn6 = assign94720_body50_e147185_d_n6;
            locals.var_fs02_dps0_dn7 = assign94720_body50_e147185_d_n7;
            locals.var_fs02_dps0_dn8 = assign94720_body50_e147185_d_n8;
            locals.var_fs02_dps0_dn9 = assign94720_body50_e147185_d_n9;
            locals.var_fs02_dps0_dn10 = assign94720_body50_e147185_d_n10;
            locals.var_fs02_dps0_dn13 = assign94720_body50_e147185_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94720_body51_e147201, assign94720_body51_e147201_d_n0, assign94720_body51_e147201_d_n2, assign94720_body51_e147201_d_n4, assign94720_body51_e147201_d_n5, assign94720_body51_e147201_d_n6, assign94720_body51_e147201_d_n7, assign94720_body51_e147201_d_n8, assign94720_body51_e147201_d_n9, assign94720_body51_e147201_d_n10, assign94720_body51_e147201_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let (assign94720_body51_e147197,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign94720_body51_e147196: f64 = (-1.0);
                (assign94720_body51_e147196,)
            }
        };
        let assign94720_body51_e147199: f64 = (assign94720_body51_e147197 * locals.var_fs02);
        (assign94720_body51_e147199, (assign94720_body51_e147197 * locals.var_fs02_dn0), (assign94720_body51_e147197 * locals.var_fs02_dn2), (assign94720_body51_e147197 * locals.var_fs02_dn4), (assign94720_body51_e147197 * locals.var_fs02_dn5), (assign94720_body51_e147197 * locals.var_fs02_dn6), (assign94720_body51_e147197 * locals.var_fs02_dn7), (assign94720_body51_e147197 * locals.var_fs02_dn8), (assign94720_body51_e147197 * locals.var_fs02_dn9), (assign94720_body51_e147197 * locals.var_fs02_dn10), (assign94720_body51_e147197 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign94720_body51_e147201;
            locals.var_fs02_dn0 = assign94720_body51_e147201_d_n0;
            locals.var_fs02_dn2 = assign94720_body51_e147201_d_n2;
            locals.var_fs02_dn4 = assign94720_body51_e147201_d_n4;
            locals.var_fs02_dn5 = assign94720_body51_e147201_d_n5;
            locals.var_fs02_dn6 = assign94720_body51_e147201_d_n6;
            locals.var_fs02_dn7 = assign94720_body51_e147201_d_n7;
            locals.var_fs02_dn8 = assign94720_body51_e147201_d_n8;
            locals.var_fs02_dn9 = assign94720_body51_e147201_d_n9;
            locals.var_fs02_dn10 = assign94720_body51_e147201_d_n10;
            locals.var_fs02_dn13 = assign94720_body51_e147201_d_n13;
            locals.var_fs02_rv = 0.0;
            let (assign94720_body52_e147217, assign94720_body52_e147217_d_n0, assign94720_body52_e147217_d_n2, assign94720_body52_e147217_d_n4, assign94720_body52_e147217_d_n5, assign94720_body52_e147217_d_n6, assign94720_body52_e147217_d_n7, assign94720_body52_e147217_d_n8, assign94720_body52_e147217_d_n9, assign94720_body52_e147217_d_n10, assign94720_body52_e147217_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let (assign94720_body52_e147213,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign94720_body52_e147212: f64 = (-1.0);
                (assign94720_body52_e147212,)
            }
        };
        let assign94720_body52_e147215: f64 = (assign94720_body52_e147213 * locals.var_fs02_dps0);
        (assign94720_body52_e147215, (assign94720_body52_e147213 * locals.var_fs02_dps0_dn0), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn2), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn4), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn5), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn6), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn7), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn8), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn9), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn10), (assign94720_body52_e147213 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign94720_body52_e147217;
            locals.var_fs02_dps0_dn0 = assign94720_body52_e147217_d_n0;
            locals.var_fs02_dps0_dn2 = assign94720_body52_e147217_d_n2;
            locals.var_fs02_dps0_dn4 = assign94720_body52_e147217_d_n4;
            locals.var_fs02_dps0_dn5 = assign94720_body52_e147217_d_n5;
            locals.var_fs02_dps0_dn6 = assign94720_body52_e147217_d_n6;
            locals.var_fs02_dps0_dn7 = assign94720_body52_e147217_d_n7;
            locals.var_fs02_dps0_dn8 = assign94720_body52_e147217_d_n8;
            locals.var_fs02_dps0_dn9 = assign94720_body52_e147217_d_n9;
            locals.var_fs02_dps0_dn10 = assign94720_body52_e147217_d_n10;
            locals.var_fs02_dps0_dn13 = assign94720_body52_e147217_d_n13;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign94720_body53_e147232, assign94720_body53_e147232_d_n0, assign94720_body53_e147232_d_n2, assign94720_body53_e147232_d_n4, assign94720_body53_e147232_d_n5, assign94720_body53_e147232_d_n6, assign94720_body53_e147232_d_n7, assign94720_body53_e147232_d_n8, assign94720_body53_e147232_d_n9, assign94720_body53_e147232_d_n10, assign94720_body53_e147232_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body53_e147224: f64 = (-locals.var_vgpld);
        let assign94720_body53_e147226: f64 = (assign94720_body53_e147224 + locals.var_ps0ld);
        let assign94720_body53_e147229: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign94720_body53_e147230: f64 = (assign94720_body53_e147226 + assign94720_body53_e147229);
        (assign94720_body53_e147230, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign94720_body53_e147232;
            locals.var_fs0_dn0 = assign94720_body53_e147232_d_n0;
            locals.var_fs0_dn2 = assign94720_body53_e147232_d_n2;
            locals.var_fs0_dn4 = assign94720_body53_e147232_d_n4;
            locals.var_fs0_dn5 = assign94720_body53_e147232_d_n5;
            locals.var_fs0_dn6 = assign94720_body53_e147232_d_n6;
            locals.var_fs0_dn7 = assign94720_body53_e147232_d_n7;
            locals.var_fs0_dn8 = assign94720_body53_e147232_d_n8;
            locals.var_fs0_dn9 = assign94720_body53_e147232_d_n9;
            locals.var_fs0_dn10 = assign94720_body53_e147232_d_n10;
            locals.var_fs0_dn13 = assign94720_body53_e147232_d_n13;
            locals.var_fs0_rv = 0.0;
            let (assign94720_body54_e147244, assign94720_body54_e147244_d_n0, assign94720_body54_e147244_d_n2, assign94720_body54_e147244_d_n4, assign94720_body54_e147244_d_n5, assign94720_body54_e147244_d_n6, assign94720_body54_e147244_d_n7, assign94720_body54_e147244_d_n8, assign94720_body54_e147244_d_n9, assign94720_body54_e147244_d_n10, assign94720_body54_e147244_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body54_e147241: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign94720_body54_e147242: f64 = (1.0 + assign94720_body54_e147241);
        (assign94720_body54_e147242, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign94720_body54_e147244;
            locals.var_fs0_dps0_dn0 = assign94720_body54_e147244_d_n0;
            locals.var_fs0_dps0_dn2 = assign94720_body54_e147244_d_n2;
            locals.var_fs0_dps0_dn4 = assign94720_body54_e147244_d_n4;
            locals.var_fs0_dps0_dn5 = assign94720_body54_e147244_d_n5;
            locals.var_fs0_dps0_dn6 = assign94720_body54_e147244_d_n6;
            locals.var_fs0_dps0_dn7 = assign94720_body54_e147244_d_n7;
            locals.var_fs0_dps0_dn8 = assign94720_body54_e147244_d_n8;
            locals.var_fs0_dps0_dn9 = assign94720_body54_e147244_d_n9;
            locals.var_fs0_dps0_dn10 = assign94720_body54_e147244_d_n10;
            locals.var_fs0_dps0_dn13 = assign94720_body54_e147244_d_n13;
            locals.var_fs0_dps0_rv = 0.0;
            let assign94720_body55_e147247: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2207 = assign94720_body55_e147247;
            locals.var_guard2207_rv = 0.0;
            let (assign94720_body56_e147259,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 != 0.0)) {
        let assign94720_body56_e147257: f64 = (locals.var_lp_s0_max + 1.0);
        (assign94720_body56_e147257,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94720_body56_e147259;
            locals.var_lp_s0_rv = 0.0;
            let (assign94720_body57_e147273, assign94720_body57_e147273_d_n0, assign94720_body57_e147273_d_n2, assign94720_body57_e147273_d_n4, assign94720_body57_e147273_d_n5, assign94720_body57_e147273_d_n6, assign94720_body57_e147273_d_n7, assign94720_body57_e147273_d_n8, assign94720_body57_e147273_d_n9, assign94720_body57_e147273_d_n10, assign94720_body57_e147273_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) {
        let assign94720_body57_e147269: f64 = (-locals.var_fs0);
        let assign94720_body57_e147271: f64 = (assign94720_body57_e147269 / locals.var_fs0_dps0);
        (assign94720_body57_e147271, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign94720_body57_e147269 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign94720_body57_e147273;
            locals.var_dps0_dn0 = assign94720_body57_e147273_d_n0;
            locals.var_dps0_dn2 = assign94720_body57_e147273_d_n2;
            locals.var_dps0_dn4 = assign94720_body57_e147273_d_n4;
            locals.var_dps0_dn5 = assign94720_body57_e147273_d_n5;
            locals.var_dps0_dn6 = assign94720_body57_e147273_d_n6;
            locals.var_dps0_dn7 = assign94720_body57_e147273_d_n7;
            locals.var_dps0_dn8 = assign94720_body57_e147273_d_n8;
            locals.var_dps0_dn9 = assign94720_body57_e147273_d_n9;
            locals.var_dps0_dn10 = assign94720_body57_e147273_d_n10;
            locals.var_dps0_dn13 = assign94720_body57_e147273_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign94720_body58_e147297, assign94720_body58_e147297_d_n0, assign94720_body58_e147297_d_n2, assign94720_body58_e147297_d_n4, assign94720_body58_e147297_d_n5, assign94720_body58_e147297_d_n6, assign94720_body58_e147297_d_n7, assign94720_body58_e147297_d_n8, assign94720_body58_e147297_d_n9, assign94720_body58_e147297_d_n10, assign94720_body58_e147297_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) {
        let assign94720_body58_e147284: f64 = (0.5 * 0.1);
        let assign94720_body58_e147288: f64 = (locals.var_ps0ld).abs();
        let (assign94720_body58_e147293, assign94720_body58_e147293_d_n0, assign94720_body58_e147293_d_n2, assign94720_body58_e147293_d_n4, assign94720_body58_e147293_d_n5, assign94720_body58_e147293_d_n6, assign94720_body58_e147293_d_n7, assign94720_body58_e147293_d_n8, assign94720_body58_e147293_d_n9, assign94720_body58_e147293_d_n10, assign94720_body58_e147293_d_n13,) = {
            if (1.0 >= assign94720_body58_e147288) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign94720_body58_e147292: f64 = (locals.var_ps0ld).abs();
                (assign94720_body58_e147292, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign94720_body58_e147294: f64 = (1.0 + assign94720_body58_e147293);
        let assign94720_body58_e147295: f64 = (assign94720_body58_e147284 * assign94720_body58_e147294);
        (assign94720_body58_e147295, (assign94720_body58_e147284 * assign94720_body58_e147293_d_n0), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n2), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n4), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n5), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n6), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n7), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n8), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n9), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n10), (assign94720_body58_e147284 * assign94720_body58_e147293_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign94720_body58_e147297;
            locals.var_dplim_dn0 = assign94720_body58_e147297_d_n0;
            locals.var_dplim_dn2 = assign94720_body58_e147297_d_n2;
            locals.var_dplim_dn4 = assign94720_body58_e147297_d_n4;
            locals.var_dplim_dn5 = assign94720_body58_e147297_d_n5;
            locals.var_dplim_dn6 = assign94720_body58_e147297_d_n6;
            locals.var_dplim_dn7 = assign94720_body58_e147297_d_n7;
            locals.var_dplim_dn8 = assign94720_body58_e147297_d_n8;
            locals.var_dplim_dn9 = assign94720_body58_e147297_d_n9;
            locals.var_dplim_dn10 = assign94720_body58_e147297_d_n10;
            locals.var_dplim_dn13 = assign94720_body58_e147297_d_n13;
            locals.var_dplim_rv = 0.0;
            let assign94720_body59_e147299: f64 = (locals.var_dps0).abs();
            let assign94720_body59_e147301: f64 = if assign94720_body59_e147299 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2208 = assign94720_body59_e147301;
            locals.var_guard2208_rv = 0.0;
            let (assign94720_body60_e147322, assign94720_body60_e147322_d_n0, assign94720_body60_e147322_d_n2, assign94720_body60_e147322_d_n4, assign94720_body60_e147322_d_n5, assign94720_body60_e147322_d_n6, assign94720_body60_e147322_d_n7, assign94720_body60_e147322_d_n8, assign94720_body60_e147322_d_n9, assign94720_body60_e147322_d_n10, assign94720_body60_e147322_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2208 != 0.0)) {
        let (assign94720_body60_e147319,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign94720_body60_e147318: f64 = (-1.0);
                (assign94720_body60_e147318,)
            }
        };
        let assign94720_body60_e147320: f64 = (locals.var_dplim * assign94720_body60_e147319);
        (assign94720_body60_e147320, (locals.var_dplim_dn0 * assign94720_body60_e147319), (locals.var_dplim_dn2 * assign94720_body60_e147319), (locals.var_dplim_dn4 * assign94720_body60_e147319), (locals.var_dplim_dn5 * assign94720_body60_e147319), (locals.var_dplim_dn6 * assign94720_body60_e147319), (locals.var_dplim_dn7 * assign94720_body60_e147319), (locals.var_dplim_dn8 * assign94720_body60_e147319), (locals.var_dplim_dn9 * assign94720_body60_e147319), (locals.var_dplim_dn10 * assign94720_body60_e147319), (locals.var_dplim_dn13 * assign94720_body60_e147319),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign94720_body60_e147322;
            locals.var_dps0_dn0 = assign94720_body60_e147322_d_n0;
            locals.var_dps0_dn2 = assign94720_body60_e147322_d_n2;
            locals.var_dps0_dn4 = assign94720_body60_e147322_d_n4;
            locals.var_dps0_dn5 = assign94720_body60_e147322_d_n5;
            locals.var_dps0_dn6 = assign94720_body60_e147322_d_n6;
            locals.var_dps0_dn7 = assign94720_body60_e147322_d_n7;
            locals.var_dps0_dn8 = assign94720_body60_e147322_d_n8;
            locals.var_dps0_dn9 = assign94720_body60_e147322_d_n9;
            locals.var_dps0_dn10 = assign94720_body60_e147322_d_n10;
            locals.var_dps0_dn13 = assign94720_body60_e147322_d_n13;
            locals.var_dps0_rv = 0.0;
            let (assign94720_body61_e147335, assign94720_body61_e147335_d_n0, assign94720_body61_e147335_d_n2, assign94720_body61_e147335_d_n4, assign94720_body61_e147335_d_n5, assign94720_body61_e147335_d_n6, assign94720_body61_e147335_d_n7, assign94720_body61_e147335_d_n8, assign94720_body61_e147335_d_n9, assign94720_body61_e147335_d_n10, assign94720_body61_e147335_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) {
        let assign94720_body61_e147333: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign94720_body61_e147333, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign94720_body61_e147335;
            locals.var_ps0ld_dn0 = assign94720_body61_e147335_d_n0;
            locals.var_ps0ld_dn2 = assign94720_body61_e147335_d_n2;
            locals.var_ps0ld_dn4 = assign94720_body61_e147335_d_n4;
            locals.var_ps0ld_dn5 = assign94720_body61_e147335_d_n5;
            locals.var_ps0ld_dn6 = assign94720_body61_e147335_d_n6;
            locals.var_ps0ld_dn7 = assign94720_body61_e147335_d_n7;
            locals.var_ps0ld_dn8 = assign94720_body61_e147335_d_n8;
            locals.var_ps0ld_dn9 = assign94720_body61_e147335_d_n9;
            locals.var_ps0ld_dn10 = assign94720_body61_e147335_d_n10;
            locals.var_ps0ld_dn13 = assign94720_body61_e147335_d_n13;
            locals.var_ps0ld_rv = 0.0;
            let assign94720_body62_e147337: f64 = (locals.var_dps0).abs();
            let assign94720_body62_e147341: f64 = (locals.var_fs0).abs();
            let assign94720_body62_e147344: f64 = if ((assign94720_body62_e147337 <= 1e-12) && (assign94720_body62_e147341 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2209 = assign94720_body62_e147344;
            locals.var_guard2209_rv = 0.0;
            let (assign94720_body63_e147359,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) && (locals.var_guard2207 == 0.0)) && (locals.var_guard2209 != 0.0)) {
        let assign94720_body63_e147357: f64 = (locals.var_flg_conv + 2.0);
        (assign94720_body63_e147357,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign94720_body63_e147359;
            locals.var_flg_conv_rv = 0.0;
            let (assign94720_body64_e147369,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94720_body64_e147367: f64 = (locals.var_lp_s0 + 1.0);
        (assign94720_body64_e147367,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign94720_body64_e147369;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_356(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94740_e147394, assign94740_e147394_d_n0, assign94740_e147394_d_n2, assign94740_e147394_d_n4, assign94740_e147394_d_n5, assign94740_e147394_d_n6, assign94740_e147394_d_n7, assign94740_e147394_d_n8, assign94740_e147394_d_n9, assign94740_e147394_d_n10, assign94740_e147394_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let (assign94740_e147392, assign94740_e147392_d_n0, assign94740_e147392_d_n2, assign94740_e147392_d_n4, assign94740_e147392_d_n5, assign94740_e147392_d_n6, assign94740_e147392_d_n7, assign94740_e147392_d_n8, assign94740_e147392_d_n9, assign94740_e147392_d_n10, assign94740_e147392_d_n13,) = {
            if (locals.var_fbsq__blk2123 >= 0.0) {
                let (assign94740_e147387,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign94740_e147386: f64 = (-1.0);
                        (assign94740_e147386,)
                    }
                };
                let assign94740_e147389: f64 = (locals.var_fbsq__blk2123).sqrt();
                let assign94740_e147390: f64 = (assign94740_e147387 * assign94740_e147389);
                (assign94740_e147390, (assign94740_e147387 * (locals.var_fbsq__blk2123_dn0 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn2 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn4 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn5 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn6 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn7 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn8 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn9 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn10 / (2.0 * assign94740_e147389))), (assign94740_e147387 * (locals.var_fbsq__blk2123_dn13 / (2.0 * assign94740_e147389))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign94740_e147392, assign94740_e147392_d_n0, assign94740_e147392_d_n2, assign94740_e147392_d_n4, assign94740_e147392_d_n5, assign94740_e147392_d_n6, assign94740_e147392_d_n7, assign94740_e147392_d_n8, assign94740_e147392_d_n9, assign94740_e147392_d_n10, assign94740_e147392_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign94740_e147394;
        locals.var_fb_dn0 = assign94740_e147394_d_n0;
        locals.var_fb_dn2 = assign94740_e147394_d_n2;
        locals.var_fb_dn4 = assign94740_e147394_d_n4;
        locals.var_fb_dn5 = assign94740_e147394_d_n5;
        locals.var_fb_dn6 = assign94740_e147394_d_n6;
        locals.var_fb_dn7 = assign94740_e147394_d_n7;
        locals.var_fb_dn8 = assign94740_e147394_d_n8;
        locals.var_fb_dn9 = assign94740_e147394_d_n9;
        locals.var_fb_dn10 = assign94740_e147394_d_n10;
        locals.var_fb_dn13 = assign94740_e147394_d_n13;
        locals.var_fb_rv = 0.0;

        let (assign94750_e147404, assign94750_e147404_d_n0, assign94750_e147404_d_n2, assign94750_e147404_d_n4, assign94750_e147404_d_n5, assign94750_e147404_d_n6, assign94750_e147404_d_n7, assign94750_e147404_d_n8, assign94750_e147404_d_n9, assign94750_e147404_d_n10, assign94750_e147404_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94750_e147402: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign94750_e147402, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk2113, locals.var_wdld__blk2113_dn0, locals.var_wdld__blk2113_dn2, locals.var_wdld__blk2113_dn4, locals.var_wdld__blk2113_dn5, locals.var_wdld__blk2113_dn6, locals.var_wdld__blk2113_dn7, locals.var_wdld__blk2113_dn8, locals.var_wdld__blk2113_dn9, locals.var_wdld__blk2113_dn10, locals.var_wdld__blk2113_dn13,)
    }
};
        locals.var_wdld__blk2113 = assign94750_e147404;
        locals.var_wdld__blk2113_dn0 = assign94750_e147404_d_n0;
        locals.var_wdld__blk2113_dn2 = assign94750_e147404_d_n2;
        locals.var_wdld__blk2113_dn4 = assign94750_e147404_d_n4;
        locals.var_wdld__blk2113_dn5 = assign94750_e147404_d_n5;
        locals.var_wdld__blk2113_dn6 = assign94750_e147404_d_n6;
        locals.var_wdld__blk2113_dn7 = assign94750_e147404_d_n7;
        locals.var_wdld__blk2113_dn8 = assign94750_e147404_d_n8;
        locals.var_wdld__blk2113_dn9 = assign94750_e147404_d_n9;
        locals.var_wdld__blk2113_dn10 = assign94750_e147404_d_n10;
        locals.var_wdld__blk2113_dn13 = assign94750_e147404_d_n13;
        locals.var_wdld__blk2113_rv = 0.0;

        let (assign94760_e147414, assign94760_e147414_d_n0, assign94760_e147414_d_n2, assign94760_e147414_d_n4, assign94760_e147414_d_n5, assign94760_e147414_d_n6, assign94760_e147414_d_n7, assign94760_e147414_d_n8, assign94760_e147414_d_n9, assign94760_e147414_d_n10, assign94760_e147414_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94760_e147412: f64 = (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113);
        (assign94760_e147412, (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn0), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn2), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn4), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn5), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn6), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn7), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn8), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn9), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn10), (locals.var_q_nsubld__blk2115 * locals.var_wdld__blk2113_dn13),)
    } else {
        (locals.var_q_dep_ld__blk2114, locals.var_q_dep_ld__blk2114_dn0, locals.var_q_dep_ld__blk2114_dn2, locals.var_q_dep_ld__blk2114_dn4, locals.var_q_dep_ld__blk2114_dn5, locals.var_q_dep_ld__blk2114_dn6, locals.var_q_dep_ld__blk2114_dn7, locals.var_q_dep_ld__blk2114_dn8, locals.var_q_dep_ld__blk2114_dn9, locals.var_q_dep_ld__blk2114_dn10, locals.var_q_dep_ld__blk2114_dn13,)
    }
};
        locals.var_q_dep_ld__blk2114 = assign94760_e147414;
        locals.var_q_dep_ld__blk2114_dn0 = assign94760_e147414_d_n0;
        locals.var_q_dep_ld__blk2114_dn2 = assign94760_e147414_d_n2;
        locals.var_q_dep_ld__blk2114_dn4 = assign94760_e147414_d_n4;
        locals.var_q_dep_ld__blk2114_dn5 = assign94760_e147414_d_n5;
        locals.var_q_dep_ld__blk2114_dn6 = assign94760_e147414_d_n6;
        locals.var_q_dep_ld__blk2114_dn7 = assign94760_e147414_d_n7;
        locals.var_q_dep_ld__blk2114_dn8 = assign94760_e147414_d_n8;
        locals.var_q_dep_ld__blk2114_dn9 = assign94760_e147414_d_n9;
        locals.var_q_dep_ld__blk2114_dn10 = assign94760_e147414_d_n10;
        locals.var_q_dep_ld__blk2114_dn13 = assign94760_e147414_d_n13;
        locals.var_q_dep_ld__blk2114_rv = 0.0;

        let (assign94770_e147428, assign94770_e147428_d_n0, assign94770_e147428_d_n2, assign94770_e147428_d_n4, assign94770_e147428_d_n5, assign94770_e147428_d_n6, assign94770_e147428_d_n7, assign94770_e147428_d_n8, assign94770_e147428_d_n9, assign94770_e147428_d_n10, assign94770_e147428_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94770_e147422: f64 = (locals.var_q_dep_ld__blk2114 / locals.var_cnst0over_func);
        let assign94770_e147425: f64 = (10.0 * 2.220446049250313e-16);
        let assign94770_e147426: f64 = (assign94770_e147422 + assign94770_e147425);
        (assign94770_e147426, (((locals.var_q_dep_ld__blk2114_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2114_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2114 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign94770_e147428;
        locals.var_xi0p12_dn0 = assign94770_e147428_d_n0;
        locals.var_xi0p12_dn2 = assign94770_e147428_d_n2;
        locals.var_xi0p12_dn4 = assign94770_e147428_d_n4;
        locals.var_xi0p12_dn5 = assign94770_e147428_d_n5;
        locals.var_xi0p12_dn6 = assign94770_e147428_d_n6;
        locals.var_xi0p12_dn7 = assign94770_e147428_d_n7;
        locals.var_xi0p12_dn8 = assign94770_e147428_d_n8;
        locals.var_xi0p12_dn9 = assign94770_e147428_d_n9;
        locals.var_xi0p12_dn10 = assign94770_e147428_d_n10;
        locals.var_xi0p12_dn13 = assign94770_e147428_d_n13;
        locals.var_xi0p12_rv = 0.0;

        let (assign94780_e147438, assign94780_e147438_d_n0, assign94780_e147438_d_n2, assign94780_e147438_d_n4, assign94780_e147438_d_n5, assign94780_e147438_d_n6, assign94780_e147438_d_n7, assign94780_e147438_d_n8, assign94780_e147438_d_n9, assign94780_e147438_d_n10, assign94780_e147438_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94780_e147436: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign94780_e147436, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign94780_e147438;
        locals.var_qbuld_dn0 = assign94780_e147438_d_n0;
        locals.var_qbuld_dn2 = assign94780_e147438_d_n2;
        locals.var_qbuld_dn4 = assign94780_e147438_d_n4;
        locals.var_qbuld_dn5 = assign94780_e147438_d_n5;
        locals.var_qbuld_dn6 = assign94780_e147438_d_n6;
        locals.var_qbuld_dn7 = assign94780_e147438_d_n7;
        locals.var_qbuld_dn8 = assign94780_e147438_d_n8;
        locals.var_qbuld_dn9 = assign94780_e147438_d_n9;
        locals.var_qbuld_dn10 = assign94780_e147438_d_n10;
        locals.var_qbuld_dn13 = assign94780_e147438_d_n13;
        locals.var_qbuld_rv = 0.0;

        let (assign94790_e147450, assign94790_e147450_d_n0, assign94790_e147450_d_n2, assign94790_e147450_d_n4, assign94790_e147450_d_n5, assign94790_e147450_d_n6, assign94790_e147450_d_n7, assign94790_e147450_d_n8, assign94790_e147450_d_n9, assign94790_e147450_d_n10, assign94790_e147450_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94790_e147447: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign94790_e147448: f64 = (1.0 / assign94790_e147447);
        (assign94790_e147448, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign94790_e147447 * assign94790_e147447))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign94790_e147447 * assign94790_e147447))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign94790_e147450;
        locals.var_t1_dn0 = assign94790_e147450_d_n0;
        locals.var_t1_dn2 = assign94790_e147450_d_n2;
        locals.var_t1_dn4 = assign94790_e147450_d_n4;
        locals.var_t1_dn5 = assign94790_e147450_d_n5;
        locals.var_t1_dn6 = assign94790_e147450_d_n6;
        locals.var_t1_dn7 = assign94790_e147450_d_n7;
        locals.var_t1_dn8 = assign94790_e147450_d_n8;
        locals.var_t1_dn9 = assign94790_e147450_d_n9;
        locals.var_t1_dn10 = assign94790_e147450_d_n10;
        locals.var_t1_dn13 = assign94790_e147450_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign94800_e147462, assign94800_e147462_d_n0, assign94800_e147462_d_n2, assign94800_e147462_d_n4, assign94800_e147462_d_n5, assign94800_e147462_d_n6, assign94800_e147462_d_n7, assign94800_e147462_d_n8, assign94800_e147462_d_n9, assign94800_e147462_d_n10, assign94800_e147462_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94800_e147458: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign94800_e147460: f64 = (assign94800_e147458 * locals.var_t1);
        (assign94800_e147460, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign94800_e147458 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign94800_e147462;
        locals.var_qiuld_dn0 = assign94800_e147462_d_n0;
        locals.var_qiuld_dn2 = assign94800_e147462_d_n2;
        locals.var_qiuld_dn4 = assign94800_e147462_d_n4;
        locals.var_qiuld_dn5 = assign94800_e147462_d_n5;
        locals.var_qiuld_dn6 = assign94800_e147462_d_n6;
        locals.var_qiuld_dn7 = assign94800_e147462_d_n7;
        locals.var_qiuld_dn8 = assign94800_e147462_d_n8;
        locals.var_qiuld_dn9 = assign94800_e147462_d_n9;
        locals.var_qiuld_dn10 = assign94800_e147462_d_n10;
        locals.var_qiuld_dn13 = assign94800_e147462_d_n13;
        locals.var_qiuld_rv = 0.0;

        let (assign94810_e147472, assign94810_e147472_d_n0, assign94810_e147472_d_n2, assign94810_e147472_d_n4, assign94810_e147472_d_n5, assign94810_e147472_d_n6, assign94810_e147472_d_n7, assign94810_e147472_d_n8, assign94810_e147472_d_n9, assign94810_e147472_d_n10, assign94810_e147472_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2196 != 0.0)) {
        let assign94810_e147470: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign94810_e147470, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign94810_e147472;
        locals.var_qsuld_dn0 = assign94810_e147472_d_n0;
        locals.var_qsuld_dn2 = assign94810_e147472_d_n2;
        locals.var_qsuld_dn4 = assign94810_e147472_d_n4;
        locals.var_qsuld_dn5 = assign94810_e147472_d_n5;
        locals.var_qsuld_dn6 = assign94810_e147472_d_n6;
        locals.var_qsuld_dn7 = assign94810_e147472_d_n7;
        locals.var_qsuld_dn8 = assign94810_e147472_d_n8;
        locals.var_qsuld_dn9 = assign94810_e147472_d_n9;
        locals.var_qsuld_dn10 = assign94810_e147472_d_n10;
        locals.var_qsuld_dn13 = assign94810_e147472_d_n13;
        locals.var_qsuld_rv = 0.0;

        let (assign94820_e147480, assign94820_e147480_d_n0, assign94820_e147480_d_n2, assign94820_e147480_d_n4, assign94820_e147480_d_n5, assign94820_e147480_d_n6, assign94820_e147480_d_n7, assign94820_e147480_d_n8, assign94820_e147480_d_n9, assign94820_e147480_d_n10, assign94820_e147480_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        let assign94820_e147478: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign94820_e147478, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign94820_e147480;
        locals.var_qiuld_dn0 = assign94820_e147480_d_n0;
        locals.var_qiuld_dn2 = assign94820_e147480_d_n2;
        locals.var_qiuld_dn4 = assign94820_e147480_d_n4;
        locals.var_qiuld_dn5 = assign94820_e147480_d_n5;
        locals.var_qiuld_dn6 = assign94820_e147480_d_n6;
        locals.var_qiuld_dn7 = assign94820_e147480_d_n7;
        locals.var_qiuld_dn8 = assign94820_e147480_d_n8;
        locals.var_qiuld_dn9 = assign94820_e147480_d_n9;
        locals.var_qiuld_dn10 = assign94820_e147480_d_n10;
        locals.var_qiuld_dn13 = assign94820_e147480_d_n13;
        locals.var_qiuld_rv = 0.0;

        let assign94830_e147483: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2211 = assign94830_e147483;
        locals.var_guard2211_rv = 0.0;

        let (assign94840_e147492, assign94840_e147492_d_n0, assign94840_e147492_d_n2, assign94840_e147492_d_n4, assign94840_e147492_d_n5, assign94840_e147492_d_n6, assign94840_e147492_d_n7, assign94840_e147492_d_n8, assign94840_e147492_d_n9, assign94840_e147492_d_n10, assign94840_e147492_d_n13,) = {
    if (((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) {
        let assign94840_e147490: f64 = (-locals.var_lover_func);
        (assign94840_e147490, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign94840_e147492;
        locals.var_lover_func_dn0 = assign94840_e147492_d_n0;
        locals.var_lover_func_dn2 = assign94840_e147492_d_n2;
        locals.var_lover_func_dn4 = assign94840_e147492_d_n4;
        locals.var_lover_func_dn5 = assign94840_e147492_d_n5;
        locals.var_lover_func_dn6 = assign94840_e147492_d_n6;
        locals.var_lover_func_dn7 = assign94840_e147492_d_n7;
        locals.var_lover_func_dn8 = assign94840_e147492_d_n8;
        locals.var_lover_func_dn9 = assign94840_e147492_d_n9;
        locals.var_lover_func_dn10 = assign94840_e147492_d_n10;
        locals.var_lover_func_dn13 = assign94840_e147492_d_n13;
        locals.var_lover_func_rv = 0.0;

        let assign94850_e147495: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2212 = assign94850_e147495;
        locals.var_guard2212_rv = 0.0;

        let assign94860_e147498: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2213 = assign94860_e147498;
        locals.var_guard2213_rv = 0.0;

        let (assign94870_e147511, assign94870_e147511_d_n0, assign94870_e147511_d_n2, assign94870_e147511_d_n4, assign94870_e147511_d_n5, assign94870_e147511_d_n6, assign94870_e147511_d_n7, assign94870_e147511_d_n8, assign94870_e147511_d_n9, assign94870_e147511_d_n10, assign94870_e147511_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) && (locals.var_guard2213 != 0.0)) {
        let assign94870_e147509: f64 = (-locals.var_ps0ld);
        (assign94870_e147509, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx__blk2116, locals.var_vx__blk2116_dn0, locals.var_vx__blk2116_dn2, locals.var_vx__blk2116_dn4, locals.var_vx__blk2116_dn5, locals.var_vx__blk2116_dn6, locals.var_vx__blk2116_dn7, locals.var_vx__blk2116_dn8, locals.var_vx__blk2116_dn9, locals.var_vx__blk2116_dn10, locals.var_vx__blk2116_dn13,)
    }
};
        locals.var_vx__blk2116 = assign94870_e147511;
        locals.var_vx__blk2116_dn0 = assign94870_e147511_d_n0;
        locals.var_vx__blk2116_dn2 = assign94870_e147511_d_n2;
        locals.var_vx__blk2116_dn4 = assign94870_e147511_d_n4;
        locals.var_vx__blk2116_dn5 = assign94870_e147511_d_n5;
        locals.var_vx__blk2116_dn6 = assign94870_e147511_d_n6;
        locals.var_vx__blk2116_dn7 = assign94870_e147511_d_n7;
        locals.var_vx__blk2116_dn8 = assign94870_e147511_d_n8;
        locals.var_vx__blk2116_dn9 = assign94870_e147511_d_n9;
        locals.var_vx__blk2116_dn10 = assign94870_e147511_d_n10;
        locals.var_vx__blk2116_dn13 = assign94870_e147511_d_n13;
        locals.var_vx__blk2116_rv = 0.0;

        let (assign94880_e147524, assign94880_e147524_d_n0, assign94880_e147524_d_n2, assign94880_e147524_d_n4, assign94880_e147524_d_n5, assign94880_e147524_d_n6, assign94880_e147524_d_n7, assign94880_e147524_d_n8, assign94880_e147524_d_n9, assign94880_e147524_d_n10, assign94880_e147524_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) && (locals.var_guard2213 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx__blk2116, locals.var_vx__blk2116_dn0, locals.var_vx__blk2116_dn2, locals.var_vx__blk2116_dn4, locals.var_vx__blk2116_dn5, locals.var_vx__blk2116_dn6, locals.var_vx__blk2116_dn7, locals.var_vx__blk2116_dn8, locals.var_vx__blk2116_dn9, locals.var_vx__blk2116_dn10, locals.var_vx__blk2116_dn13,)
    }
};
        locals.var_vx__blk2116 = assign94880_e147524;
        locals.var_vx__blk2116_dn0 = assign94880_e147524_d_n0;
        locals.var_vx__blk2116_dn2 = assign94880_e147524_d_n2;
        locals.var_vx__blk2116_dn4 = assign94880_e147524_d_n4;
        locals.var_vx__blk2116_dn5 = assign94880_e147524_d_n5;
        locals.var_vx__blk2116_dn6 = assign94880_e147524_d_n6;
        locals.var_vx__blk2116_dn7 = assign94880_e147524_d_n7;
        locals.var_vx__blk2116_dn8 = assign94880_e147524_d_n8;
        locals.var_vx__blk2116_dn9 = assign94880_e147524_d_n9;
        locals.var_vx__blk2116_dn10 = assign94880_e147524_d_n10;
        locals.var_vx__blk2116_dn13 = assign94880_e147524_d_n13;
        locals.var_vx__blk2116_rv = 0.0;

        let (assign94890_e147547, assign94890_e147547_d_n0, assign94890_e147547_d_n2, assign94890_e147547_d_n4, assign94890_e147547_d_n5, assign94890_e147547_d_n6, assign94890_e147547_d_n7, assign94890_e147547_d_n8, assign94890_e147547_d_n9, assign94890_e147547_d_n10, assign94890_e147547_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94890_e147534: f64 = (locals.var_vx__blk2116 + p.p137);
        let assign94890_e147537: f64 = (locals.var_vx__blk2116 + p.p137);
        let assign94890_e147538: f64 = (assign94890_e147534 * assign94890_e147537);
        let assign94890_e147541: f64 = (4.0 * 0.1);
        let assign94890_e147543: f64 = (assign94890_e147541 * 0.1);
        let assign94890_e147544: f64 = (assign94890_e147538 + assign94890_e147543);
        let assign94890_e147545: f64 = (assign94890_e147544).sqrt();
        (assign94890_e147545, (((locals.var_vx__blk2116_dn0 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn0)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn2 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn2)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn4 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn4)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn5 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn5)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn6 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn6)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn7 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn7)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn8 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn8)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn9 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn9)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn10 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn10)) / (2.0 * assign94890_e147545)), (((locals.var_vx__blk2116_dn13 * assign94890_e147537) + (assign94890_e147534 * locals.var_vx__blk2116_dn13)) / (2.0 * assign94890_e147545)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94890_e147547;
        locals.var_tmf2_dn0 = assign94890_e147547_d_n0;
        locals.var_tmf2_dn2 = assign94890_e147547_d_n2;
        locals.var_tmf2_dn4 = assign94890_e147547_d_n4;
        locals.var_tmf2_dn5 = assign94890_e147547_d_n5;
        locals.var_tmf2_dn6 = assign94890_e147547_d_n6;
        locals.var_tmf2_dn7 = assign94890_e147547_d_n7;
        locals.var_tmf2_dn8 = assign94890_e147547_d_n8;
        locals.var_tmf2_dn9 = assign94890_e147547_d_n9;
        locals.var_tmf2_dn10 = assign94890_e147547_d_n10;
        locals.var_tmf2_dn13 = assign94890_e147547_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign94900_e147565, assign94900_e147565_d_n0, assign94900_e147565_d_n2, assign94900_e147565_d_n4, assign94900_e147565_d_n5, assign94900_e147565_d_n6, assign94900_e147565_d_n7, assign94900_e147565_d_n8, assign94900_e147565_d_n9, assign94900_e147565_d_n10, assign94900_e147565_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94900_e147559: f64 = (locals.var_vx__blk2116 + p.p137);
        let assign94900_e147561: f64 = (assign94900_e147559 / locals.var_tmf2);
        let assign94900_e147562: f64 = (1.0 + assign94900_e147561);
        let assign94900_e147563: f64 = (0.5 * assign94900_e147562);
        (assign94900_e147563, (0.5 * (((locals.var_vx__blk2116_dn0 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn2 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn4 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn5 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn6 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn7 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn8 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn9 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn10 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2116_dn13 * locals.var_tmf2) - (assign94900_e147559 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94900_e147565;
        locals.var_t9_dn0 = assign94900_e147565_d_n0;
        locals.var_t9_dn2 = assign94900_e147565_d_n2;
        locals.var_t9_dn4 = assign94900_e147565_d_n4;
        locals.var_t9_dn5 = assign94900_e147565_d_n5;
        locals.var_t9_dn6 = assign94900_e147565_d_n6;
        locals.var_t9_dn7 = assign94900_e147565_d_n7;
        locals.var_t9_dn8 = assign94900_e147565_d_n8;
        locals.var_t9_dn9 = assign94900_e147565_d_n9;
        locals.var_t9_dn10 = assign94900_e147565_d_n10;
        locals.var_t9_dn13 = assign94900_e147565_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94910_e147581, assign94910_e147581_d_n0, assign94910_e147581_d_n2, assign94910_e147581_d_n4, assign94910_e147581_d_n5, assign94910_e147581_d_n6, assign94910_e147581_d_n7, assign94910_e147581_d_n8, assign94910_e147581_d_n9, assign94910_e147581_d_n10, assign94910_e147581_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94910_e147576: f64 = (locals.var_vx__blk2116 + p.p137);
        let assign94910_e147578: f64 = (assign94910_e147576 + locals.var_tmf2);
        let assign94910_e147579: f64 = (0.5 * assign94910_e147578);
        (assign94910_e147579, (0.5 * (locals.var_vx__blk2116_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk2116_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk2116_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk2116_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk2116_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk2116_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk2116_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk2116_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk2116_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk2116_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94910_e147581;
        locals.var_t2_dn0 = assign94910_e147581_d_n0;
        locals.var_t2_dn2 = assign94910_e147581_d_n2;
        locals.var_t2_dn4 = assign94910_e147581_d_n4;
        locals.var_t2_dn5 = assign94910_e147581_d_n5;
        locals.var_t2_dn6 = assign94910_e147581_d_n6;
        locals.var_t2_dn7 = assign94910_e147581_d_n7;
        locals.var_t2_dn8 = assign94910_e147581_d_n8;
        locals.var_t2_dn9 = assign94910_e147581_d_n9;
        locals.var_t2_dn10 = assign94910_e147581_d_n10;
        locals.var_t2_dn13 = assign94910_e147581_d_n13;
        locals.var_t2_rv = 0.0;

        let assign94920_e147584: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2214 = assign94920_e147584;
        locals.var_guard2214_rv = 0.0;

        let (assign94930_e147596, assign94930_e147596_d_n0, assign94930_e147596_d_n2, assign94930_e147596_d_n4, assign94930_e147596_d_n5, assign94930_e147596_d_n6, assign94930_e147596_d_n7, assign94930_e147596_d_n8, assign94930_e147596_d_n9, assign94930_e147596_d_n10, assign94930_e147596_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign94930_e147596;
        locals.var_t2_dn0 = assign94930_e147596_d_n0;
        locals.var_t2_dn2 = assign94930_e147596_d_n2;
        locals.var_t2_dn4 = assign94930_e147596_d_n4;
        locals.var_t2_dn5 = assign94930_e147596_d_n5;
        locals.var_t2_dn6 = assign94930_e147596_d_n6;
        locals.var_t2_dn7 = assign94930_e147596_d_n7;
        locals.var_t2_dn8 = assign94930_e147596_d_n8;
        locals.var_t2_dn9 = assign94930_e147596_d_n9;
        locals.var_t2_dn10 = assign94930_e147596_d_n10;
        locals.var_t2_dn13 = assign94930_e147596_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign94940_e147608, assign94940_e147608_d_n0, assign94940_e147608_d_n2, assign94940_e147608_d_n4, assign94940_e147608_d_n5, assign94940_e147608_d_n6, assign94940_e147608_d_n7, assign94940_e147608_d_n8, assign94940_e147608_d_n9, assign94940_e147608_d_n10, assign94940_e147608_d_n13,) = {
    if (((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) && (locals.var_guard2214 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign94940_e147608;
        locals.var_t9_dn0 = assign94940_e147608_d_n0;
        locals.var_t9_dn2 = assign94940_e147608_d_n2;
        locals.var_t9_dn4 = assign94940_e147608_d_n4;
        locals.var_t9_dn5 = assign94940_e147608_d_n5;
        locals.var_t9_dn6 = assign94940_e147608_d_n6;
        locals.var_t9_dn7 = assign94940_e147608_d_n7;
        locals.var_t9_dn8 = assign94940_e147608_d_n8;
        locals.var_t9_dn9 = assign94940_e147608_d_n9;
        locals.var_t9_dn10 = assign94940_e147608_d_n10;
        locals.var_t9_dn13 = assign94940_e147608_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign94950_e147623, assign94950_e147623_d_n0, assign94950_e147623_d_n2, assign94950_e147623_d_n4, assign94950_e147623_d_n5, assign94950_e147623_d_n6, assign94950_e147623_d_n7, assign94950_e147623_d_n8, assign94950_e147623_d_n9, assign94950_e147623_d_n10, assign94950_e147623_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94950_e147618: f64 = (locals.var_kjunc * locals.var_t2);
        let assign94950_e147619: f64 = (assign94950_e147618).sqrt();
        let assign94950_e147621: f64 = (assign94950_e147619 * p.p432);
        (assign94950_e147621, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign94950_e147619)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign94950_e147619)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign94950_e147623;
        locals.var_wjunc0_dn0 = assign94950_e147623_d_n0;
        locals.var_wjunc0_dn2 = assign94950_e147623_d_n2;
        locals.var_wjunc0_dn4 = assign94950_e147623_d_n4;
        locals.var_wjunc0_dn5 = assign94950_e147623_d_n5;
        locals.var_wjunc0_dn6 = assign94950_e147623_d_n6;
        locals.var_wjunc0_dn7 = assign94950_e147623_d_n7;
        locals.var_wjunc0_dn8 = assign94950_e147623_d_n8;
        locals.var_wjunc0_dn9 = assign94950_e147623_d_n9;
        locals.var_wjunc0_dn10 = assign94950_e147623_d_n10;
        locals.var_wjunc0_dn13 = assign94950_e147623_d_n13;
        locals.var_wjunc0_rv = 0.0;

        let (assign94960_e147639, assign94960_e147639_d_n0, assign94960_e147639_d_n2, assign94960_e147639_d_n4, assign94960_e147639_d_n5, assign94960_e147639_d_n6, assign94960_e147639_d_n7, assign94960_e147639_d_n8, assign94960_e147639_d_n9, assign94960_e147639_d_n10, assign94960_e147639_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94960_e147633: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign94960_e147636: f64 = (0.1 * locals.var_lover_func);
        let assign94960_e147637: f64 = (assign94960_e147633 - assign94960_e147636);
        (assign94960_e147637, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign94960_e147639;
        locals.var_tmf1_dn0 = assign94960_e147639_d_n0;
        locals.var_tmf1_dn2 = assign94960_e147639_d_n2;
        locals.var_tmf1_dn4 = assign94960_e147639_d_n4;
        locals.var_tmf1_dn5 = assign94960_e147639_d_n5;
        locals.var_tmf1_dn6 = assign94960_e147639_d_n6;
        locals.var_tmf1_dn7 = assign94960_e147639_d_n7;
        locals.var_tmf1_dn8 = assign94960_e147639_d_n8;
        locals.var_tmf1_dn9 = assign94960_e147639_d_n9;
        locals.var_tmf1_dn10 = assign94960_e147639_d_n10;
        locals.var_tmf1_dn13 = assign94960_e147639_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign94970_e147655, assign94970_e147655_d_n0, assign94970_e147655_d_n2, assign94970_e147655_d_n4, assign94970_e147655_d_n5, assign94970_e147655_d_n6, assign94970_e147655_d_n7, assign94970_e147655_d_n8, assign94970_e147655_d_n9, assign94970_e147655_d_n10, assign94970_e147655_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94970_e147649: f64 = (4.0 * locals.var_lover_func);
        let assign94970_e147652: f64 = (0.1 * locals.var_lover_func);
        let assign94970_e147653: f64 = (assign94970_e147649 * assign94970_e147652);
        (assign94970_e147653, (((4.0 * locals.var_lover_func_dn0) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign94970_e147652) + (assign94970_e147649 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94970_e147655;
        locals.var_tmf2_dn0 = assign94970_e147655_d_n0;
        locals.var_tmf2_dn2 = assign94970_e147655_d_n2;
        locals.var_tmf2_dn4 = assign94970_e147655_d_n4;
        locals.var_tmf2_dn5 = assign94970_e147655_d_n5;
        locals.var_tmf2_dn6 = assign94970_e147655_d_n6;
        locals.var_tmf2_dn7 = assign94970_e147655_d_n7;
        locals.var_tmf2_dn8 = assign94970_e147655_d_n8;
        locals.var_tmf2_dn9 = assign94970_e147655_d_n9;
        locals.var_tmf2_dn10 = assign94970_e147655_d_n10;
        locals.var_tmf2_dn13 = assign94970_e147655_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign94980_e147671, assign94980_e147671_d_n0, assign94980_e147671_d_n2, assign94980_e147671_d_n4, assign94980_e147671_d_n5, assign94980_e147671_d_n6, assign94980_e147671_d_n7, assign94980_e147671_d_n8, assign94980_e147671_d_n9, assign94980_e147671_d_n10, assign94980_e147671_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let (assign94980_e147669, assign94980_e147669_d_n0, assign94980_e147669_d_n2, assign94980_e147669_d_n4, assign94980_e147669_d_n5, assign94980_e147669_d_n6, assign94980_e147669_d_n7, assign94980_e147669_d_n8, assign94980_e147669_d_n9, assign94980_e147669_d_n10, assign94980_e147669_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign94980_e147668: f64 = (-locals.var_tmf2);
                (assign94980_e147668, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign94980_e147669, assign94980_e147669_d_n0, assign94980_e147669_d_n2, assign94980_e147669_d_n4, assign94980_e147669_d_n5, assign94980_e147669_d_n6, assign94980_e147669_d_n7, assign94980_e147669_d_n8, assign94980_e147669_d_n9, assign94980_e147669_d_n10, assign94980_e147669_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94980_e147671;
        locals.var_tmf2_dn0 = assign94980_e147671_d_n0;
        locals.var_tmf2_dn2 = assign94980_e147671_d_n2;
        locals.var_tmf2_dn4 = assign94980_e147671_d_n4;
        locals.var_tmf2_dn5 = assign94980_e147671_d_n5;
        locals.var_tmf2_dn6 = assign94980_e147671_d_n6;
        locals.var_tmf2_dn7 = assign94980_e147671_d_n7;
        locals.var_tmf2_dn8 = assign94980_e147671_d_n8;
        locals.var_tmf2_dn9 = assign94980_e147671_d_n9;
        locals.var_tmf2_dn10 = assign94980_e147671_d_n10;
        locals.var_tmf2_dn13 = assign94980_e147671_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_357(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign94990_e147686, assign94990_e147686_d_n0, assign94990_e147686_d_n2, assign94990_e147686_d_n4, assign94990_e147686_d_n5, assign94990_e147686_d_n6, assign94990_e147686_d_n7, assign94990_e147686_d_n8, assign94990_e147686_d_n9, assign94990_e147686_d_n10, assign94990_e147686_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign94990_e147681: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign94990_e147683: f64 = (assign94990_e147681 + locals.var_tmf2);
        let assign94990_e147684: f64 = (assign94990_e147683).sqrt();
        (assign94990_e147684, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign94990_e147684)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign94990_e147684)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign94990_e147686;
        locals.var_tmf2_dn0 = assign94990_e147686_d_n0;
        locals.var_tmf2_dn2 = assign94990_e147686_d_n2;
        locals.var_tmf2_dn4 = assign94990_e147686_d_n4;
        locals.var_tmf2_dn5 = assign94990_e147686_d_n5;
        locals.var_tmf2_dn6 = assign94990_e147686_d_n6;
        locals.var_tmf2_dn7 = assign94990_e147686_d_n7;
        locals.var_tmf2_dn8 = assign94990_e147686_d_n8;
        locals.var_tmf2_dn9 = assign94990_e147686_d_n9;
        locals.var_tmf2_dn10 = assign94990_e147686_d_n10;
        locals.var_tmf2_dn13 = assign94990_e147686_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign95000_e147702, assign95000_e147702_d_n0, assign95000_e147702_d_n2, assign95000_e147702_d_n4, assign95000_e147702_d_n5, assign95000_e147702_d_n6, assign95000_e147702_d_n7, assign95000_e147702_d_n8, assign95000_e147702_d_n9, assign95000_e147702_d_n10, assign95000_e147702_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign95000_e147698: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign95000_e147699: f64 = (1.0 + assign95000_e147698);
        let assign95000_e147700: f64 = (0.5 * assign95000_e147699);
        (assign95000_e147700, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95000_e147702;
        locals.var_t0_dn0 = assign95000_e147702_d_n0;
        locals.var_t0_dn2 = assign95000_e147702_d_n2;
        locals.var_t0_dn4 = assign95000_e147702_d_n4;
        locals.var_t0_dn5 = assign95000_e147702_d_n5;
        locals.var_t0_dn6 = assign95000_e147702_d_n6;
        locals.var_t0_dn7 = assign95000_e147702_d_n7;
        locals.var_t0_dn8 = assign95000_e147702_d_n8;
        locals.var_t0_dn9 = assign95000_e147702_d_n9;
        locals.var_t0_dn10 = assign95000_e147702_d_n10;
        locals.var_t0_dn13 = assign95000_e147702_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign95010_e147718, assign95010_e147718_d_n0, assign95010_e147718_d_n2, assign95010_e147718_d_n4, assign95010_e147718_d_n5, assign95010_e147718_d_n6, assign95010_e147718_d_n7, assign95010_e147718_d_n8, assign95010_e147718_d_n9, assign95010_e147718_d_n10, assign95010_e147718_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign95010_e147714: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign95010_e147715: f64 = (0.5 * assign95010_e147714);
        let assign95010_e147716: f64 = (locals.var_lover_func - assign95010_e147715);
        (assign95010_e147716, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn13,)
    }
};
        locals.var_wjuncld = assign95010_e147718;
        locals.var_wjuncld_dn0 = assign95010_e147718_d_n0;
        locals.var_wjuncld_dn2 = assign95010_e147718_d_n2;
        locals.var_wjuncld_dn4 = assign95010_e147718_d_n4;
        locals.var_wjuncld_dn5 = assign95010_e147718_d_n5;
        locals.var_wjuncld_dn6 = assign95010_e147718_d_n6;
        locals.var_wjuncld_dn7 = assign95010_e147718_d_n7;
        locals.var_wjuncld_dn8 = assign95010_e147718_d_n8;
        locals.var_wjuncld_dn9 = assign95010_e147718_d_n9;
        locals.var_wjuncld_dn10 = assign95010_e147718_d_n10;
        locals.var_wjuncld_dn13 = assign95010_e147718_d_n13;
        locals.var_wjuncld_rv = 0.0;

        let (assign95020_e147730, assign95020_e147730_d_n0, assign95020_e147730_d_n2, assign95020_e147730_d_n4, assign95020_e147730_d_n5, assign95020_e147730_d_n6, assign95020_e147730_d_n7, assign95020_e147730_d_n8, assign95020_e147730_d_n9, assign95020_e147730_d_n10, assign95020_e147730_d_n13,) = {
    if ((((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) && (locals.var_guard2211 != 0.0)) && (locals.var_guard2212 != 0.0)) {
        let assign95020_e147728: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign95020_e147728, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn13 - locals.var_wjuncld_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign95020_e147730;
        locals.var_lover_func_dn0 = assign95020_e147730_d_n0;
        locals.var_lover_func_dn2 = assign95020_e147730_d_n2;
        locals.var_lover_func_dn4 = assign95020_e147730_d_n4;
        locals.var_lover_func_dn5 = assign95020_e147730_d_n5;
        locals.var_lover_func_dn6 = assign95020_e147730_d_n6;
        locals.var_lover_func_dn7 = assign95020_e147730_d_n7;
        locals.var_lover_func_dn8 = assign95020_e147730_d_n8;
        locals.var_lover_func_dn9 = assign95020_e147730_d_n9;
        locals.var_lover_func_dn10 = assign95020_e147730_d_n10;
        locals.var_lover_func_dn13 = assign95020_e147730_d_n13;
        locals.var_lover_func_rv = 0.0;

        let (assign95030_e147736, assign95030_e147736_d_n0, assign95030_e147736_d_n2, assign95030_e147736_d_n4, assign95030_e147736_d_n5, assign95030_e147736_d_n6, assign95030_e147736_d_n7, assign95030_e147736_d_n8, assign95030_e147736_d_n9, assign95030_e147736_d_n10, assign95030_e147736_d_n13,) = {
    if ((locals.var_guard2111 != 0.0) && (locals.var_guard2112 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn13,)
    }
};
        locals.var_rd_qbuld = assign95030_e147736;
        locals.var_rd_qbuld_dn0 = assign95030_e147736_d_n0;
        locals.var_rd_qbuld_dn2 = assign95030_e147736_d_n2;
        locals.var_rd_qbuld_dn4 = assign95030_e147736_d_n4;
        locals.var_rd_qbuld_dn5 = assign95030_e147736_d_n5;
        locals.var_rd_qbuld_dn6 = assign95030_e147736_d_n6;
        locals.var_rd_qbuld_dn7 = assign95030_e147736_d_n7;
        locals.var_rd_qbuld_dn8 = assign95030_e147736_d_n8;
        locals.var_rd_qbuld_dn9 = assign95030_e147736_d_n9;
        locals.var_rd_qbuld_dn10 = assign95030_e147736_d_n10;
        locals.var_rd_qbuld_dn13 = assign95030_e147736_d_n13;
        locals.var_rd_qbuld_rv = 0.0;

        let assign95040_e147747: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2215 = assign95040_e147747;
        locals.var_guard2215_rv = 0.0;

        let (assign95050_e147751,) = {
    if (locals.var_guard2215 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign95050_e147751;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign95060_e147755,) = {
    if (locals.var_guard2215 != 0.0) {
        (locals.var_mks_ovslp,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign95060_e147755;
        locals.var_cov_slp_rv = 0.0;

        let (assign95070_e147759,) = {
    if (locals.var_guard2215 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign95070_e147759;
        locals.var_cov_mag_rv = 0.0;

        let (assign95080_e147765, assign95080_e147765_d_n0, assign95080_e147765_d_n2, assign95080_e147765_d_n4, assign95080_e147765_d_n5, assign95080_e147765_d_n6, assign95080_e147765_d_n7, assign95080_e147765_d_n8, assign95080_e147765_d_n9, assign95080_e147765_d_n10, assign95080_e147765_d_n13,) = {
    if (locals.var_guard2215 != 0.0) {
        let assign95080_e147763: f64 = (locals.var_cox0 * locals.var_weffcv_nf);
        (assign95080_e147763, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign95080_e147765;
        locals.var_t1_dn0 = assign95080_e147765_d_n0;
        locals.var_t1_dn2 = assign95080_e147765_d_n2;
        locals.var_t1_dn4 = assign95080_e147765_d_n4;
        locals.var_t1_dn5 = assign95080_e147765_d_n5;
        locals.var_t1_dn6 = assign95080_e147765_d_n6;
        locals.var_t1_dn7 = assign95080_e147765_d_n7;
        locals.var_t1_dn8 = assign95080_e147765_d_n8;
        locals.var_t1_dn9 = assign95080_e147765_d_n9;
        locals.var_t1_dn10 = assign95080_e147765_d_n10;
        locals.var_t1_dn13 = assign95080_e147765_d_n13;
        locals.var_t1_rv = 0.0;

        let assign95090_e147768: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2216 = assign95090_e147768;
        locals.var_guard2216_rv = 0.0;

        let (assign95100_e147780, assign95100_e147780_d_n0, assign95100_e147780_d_n2, assign95100_e147780_d_n4, assign95100_e147780_d_n5, assign95100_e147780_d_n6, assign95100_e147780_d_n7, assign95100_e147780_d_n8, assign95100_e147780_d_n9, assign95100_e147780_d_n10, assign95100_e147780_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 != 0.0)) {
        let assign95100_e147774: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95100_e147777: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95100_e147778: f64 = (assign95100_e147774 * assign95100_e147777);
        (assign95100_e147778, ((locals.var_cov_slp * locals.var_t1_dn0) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn2) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn4) * assign95100_e147777), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95100_e147777) + (assign95100_e147774 * locals.var_vgs_dn5)), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95100_e147777) + (assign95100_e147774 * locals.var_vgs_dn6)), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95100_e147777) + (assign95100_e147774 * locals.var_vgs_dn7)), ((locals.var_cov_slp * locals.var_t1_dn8) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn9) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn10) * assign95100_e147777), ((locals.var_cov_slp * locals.var_t1_dn13) * assign95100_e147777),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign95100_e147780;
        locals.var_t4_dn0 = assign95100_e147780_d_n0;
        locals.var_t4_dn2 = assign95100_e147780_d_n2;
        locals.var_t4_dn4 = assign95100_e147780_d_n4;
        locals.var_t4_dn5 = assign95100_e147780_d_n5;
        locals.var_t4_dn6 = assign95100_e147780_d_n6;
        locals.var_t4_dn7 = assign95100_e147780_d_n7;
        locals.var_t4_dn8 = assign95100_e147780_d_n8;
        locals.var_t4_dn9 = assign95100_e147780_d_n9;
        locals.var_t4_dn10 = assign95100_e147780_d_n10;
        locals.var_t4_dn13 = assign95100_e147780_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign95110_e147788, assign95110_e147788_d_n0, assign95110_e147788_d_n2, assign95110_e147788_d_n4, assign95110_e147788_d_n5, assign95110_e147788_d_n6, assign95110_e147788_d_n7, assign95110_e147788_d_n8, assign95110_e147788_d_n9, assign95110_e147788_d_n10, assign95110_e147788_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 != 0.0)) {
        let assign95110_e147786: f64 = (p.p66 * locals.var_t1);
        (assign95110_e147786, (p.p66 * locals.var_t1_dn0), (p.p66 * locals.var_t1_dn2), (p.p66 * locals.var_t1_dn4), (p.p66 * locals.var_t1_dn5), (p.p66 * locals.var_t1_dn6), (p.p66 * locals.var_t1_dn7), (p.p66 * locals.var_t1_dn8), (p.p66 * locals.var_t1_dn9), (p.p66 * locals.var_t1_dn10), (p.p66 * locals.var_t1_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign95110_e147788;
        locals.var_t5_dn0 = assign95110_e147788_d_n0;
        locals.var_t5_dn2 = assign95110_e147788_d_n2;
        locals.var_t5_dn4 = assign95110_e147788_d_n4;
        locals.var_t5_dn5 = assign95110_e147788_d_n5;
        locals.var_t5_dn6 = assign95110_e147788_d_n6;
        locals.var_t5_dn7 = assign95110_e147788_d_n7;
        locals.var_t5_dn8 = assign95110_e147788_d_n8;
        locals.var_t5_dn9 = assign95110_e147788_d_n9;
        locals.var_t5_dn10 = assign95110_e147788_d_n10;
        locals.var_t5_dn13 = assign95110_e147788_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign95120_e147796, assign95120_e147796_d_n0, assign95120_e147796_d_n2, assign95120_e147796_d_n4, assign95120_e147796_d_n5, assign95120_e147796_d_n6, assign95120_e147796_d_n7, assign95120_e147796_d_n8, assign95120_e147796_d_n9, assign95120_e147796_d_n10, assign95120_e147796_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 != 0.0)) {
        let assign95120_e147794: f64 = (1.2 - locals.var_ps0);
        (assign95120_e147794, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign95120_e147796;
        locals.var_t9_dn0 = assign95120_e147796_d_n0;
        locals.var_t9_dn2 = assign95120_e147796_d_n2;
        locals.var_t9_dn4 = assign95120_e147796_d_n4;
        locals.var_t9_dn5 = assign95120_e147796_d_n5;
        locals.var_t9_dn6 = assign95120_e147796_d_n6;
        locals.var_t9_dn7 = assign95120_e147796_d_n7;
        locals.var_t9_dn8 = assign95120_e147796_d_n8;
        locals.var_t9_dn9 = assign95120_e147796_d_n9;
        locals.var_t9_dn10 = assign95120_e147796_d_n10;
        locals.var_t9_dn13 = assign95120_e147796_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign95130_e147808, assign95130_e147808_d_n0, assign95130_e147808_d_n2, assign95130_e147808_d_n4, assign95130_e147808_d_n5, assign95130_e147808_d_n6, assign95130_e147808_d_n7, assign95130_e147808_d_n8, assign95130_e147808_d_n9, assign95130_e147808_d_n10, assign95130_e147808_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 != 0.0)) {
        let assign95130_e147802: f64 = (locals.var_vgs * locals.var_t5);
        let assign95130_e147805: f64 = (locals.var_t4 * locals.var_t9);
        let assign95130_e147806: f64 = (assign95130_e147802 - assign95130_e147805);
        (assign95130_e147806, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), (((locals.var_vgs_dn5 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((locals.var_vgs * locals.var_t5_dn8) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn13) - ((locals.var_t4_dn13 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn13))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn8, locals.var_qgos_dn9, locals.var_qgos_dn10, locals.var_qgos_dn13,)
    }
};
        locals.var_qgos = assign95130_e147808;
        locals.var_qgos_dn0 = assign95130_e147808_d_n0;
        locals.var_qgos_dn2 = assign95130_e147808_d_n2;
        locals.var_qgos_dn4 = assign95130_e147808_d_n4;
        locals.var_qgos_dn5 = assign95130_e147808_d_n5;
        locals.var_qgos_dn6 = assign95130_e147808_d_n6;
        locals.var_qgos_dn7 = assign95130_e147808_d_n7;
        locals.var_qgos_dn8 = assign95130_e147808_d_n8;
        locals.var_qgos_dn9 = assign95130_e147808_d_n9;
        locals.var_qgos_dn10 = assign95130_e147808_d_n10;
        locals.var_qgos_dn13 = assign95130_e147808_d_n13;
        locals.var_qgos_rv = 0.0;

        let (assign95140_e147823, assign95140_e147823_d_n0, assign95140_e147823_d_n2, assign95140_e147823_d_n4, assign95140_e147823_d_n5, assign95140_e147823_d_n6, assign95140_e147823_d_n7, assign95140_e147823_d_n8, assign95140_e147823_d_n9, assign95140_e147823_d_n10, assign95140_e147823_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 == 0.0)) {
        let assign95140_e147815: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95140_e147818: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95140_e147820: f64 = (assign95140_e147818 - locals.var_vds);
        let assign95140_e147821: f64 = (assign95140_e147815 * assign95140_e147820);
        (assign95140_e147821, (((locals.var_cov_slp * locals.var_t1_dn0) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1_dn2) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1_dn4) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn4))), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95140_e147820) + (assign95140_e147815 * (locals.var_vgs_dn5 - locals.var_vds_dn5))), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95140_e147820) + (assign95140_e147815 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95140_e147820) + (assign95140_e147815 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn8))), (((locals.var_cov_slp * locals.var_t1_dn9) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn9))), (((locals.var_cov_slp * locals.var_t1_dn10) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1_dn13) * assign95140_e147820) + (assign95140_e147815 * (-locals.var_vds_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign95140_e147823;
        locals.var_t4_dn0 = assign95140_e147823_d_n0;
        locals.var_t4_dn2 = assign95140_e147823_d_n2;
        locals.var_t4_dn4 = assign95140_e147823_d_n4;
        locals.var_t4_dn5 = assign95140_e147823_d_n5;
        locals.var_t4_dn6 = assign95140_e147823_d_n6;
        locals.var_t4_dn7 = assign95140_e147823_d_n7;
        locals.var_t4_dn8 = assign95140_e147823_d_n8;
        locals.var_t4_dn9 = assign95140_e147823_d_n9;
        locals.var_t4_dn10 = assign95140_e147823_d_n10;
        locals.var_t4_dn13 = assign95140_e147823_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign95150_e147832, assign95150_e147832_d_n0, assign95150_e147832_d_n2, assign95150_e147832_d_n4, assign95150_e147832_d_n5, assign95150_e147832_d_n6, assign95150_e147832_d_n7, assign95150_e147832_d_n8, assign95150_e147832_d_n9, assign95150_e147832_d_n10, assign95150_e147832_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 == 0.0)) {
        let assign95150_e147830: f64 = (p.p66 * locals.var_t1);
        (assign95150_e147830, (p.p66 * locals.var_t1_dn0), (p.p66 * locals.var_t1_dn2), (p.p66 * locals.var_t1_dn4), (p.p66 * locals.var_t1_dn5), (p.p66 * locals.var_t1_dn6), (p.p66 * locals.var_t1_dn7), (p.p66 * locals.var_t1_dn8), (p.p66 * locals.var_t1_dn9), (p.p66 * locals.var_t1_dn10), (p.p66 * locals.var_t1_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign95150_e147832;
        locals.var_t5_dn0 = assign95150_e147832_d_n0;
        locals.var_t5_dn2 = assign95150_e147832_d_n2;
        locals.var_t5_dn4 = assign95150_e147832_d_n4;
        locals.var_t5_dn5 = assign95150_e147832_d_n5;
        locals.var_t5_dn6 = assign95150_e147832_d_n6;
        locals.var_t5_dn7 = assign95150_e147832_d_n7;
        locals.var_t5_dn8 = assign95150_e147832_d_n8;
        locals.var_t5_dn9 = assign95150_e147832_d_n9;
        locals.var_t5_dn10 = assign95150_e147832_d_n10;
        locals.var_t5_dn13 = assign95150_e147832_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign95160_e147843, assign95160_e147843_d_n0, assign95160_e147843_d_n2, assign95160_e147843_d_n4, assign95160_e147843_d_n5, assign95160_e147843_d_n6, assign95160_e147843_d_n7, assign95160_e147843_d_n8, assign95160_e147843_d_n9, assign95160_e147843_d_n10, assign95160_e147843_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 == 0.0)) {
        let assign95160_e147839: f64 = (1.2 + locals.var_vds);
        let assign95160_e147841: f64 = (assign95160_e147839 - locals.var_psl);
        (assign95160_e147841, (locals.var_vds_dn0 - locals.var_psl_dn0), (locals.var_vds_dn2 - locals.var_psl_dn2), (locals.var_vds_dn4 - locals.var_psl_dn4), (locals.var_vds_dn5 - locals.var_psl_dn5), (locals.var_vds_dn6 - locals.var_psl_dn6), (locals.var_vds_dn7 - locals.var_psl_dn7), (locals.var_vds_dn8 - locals.var_psl_dn8), (locals.var_vds_dn9 - locals.var_psl_dn9), (locals.var_vds_dn10 - locals.var_psl_dn10), (locals.var_vds_dn13 - locals.var_psl_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign95160_e147843;
        locals.var_t9_dn0 = assign95160_e147843_d_n0;
        locals.var_t9_dn2 = assign95160_e147843_d_n2;
        locals.var_t9_dn4 = assign95160_e147843_d_n4;
        locals.var_t9_dn5 = assign95160_e147843_d_n5;
        locals.var_t9_dn6 = assign95160_e147843_d_n6;
        locals.var_t9_dn7 = assign95160_e147843_d_n7;
        locals.var_t9_dn8 = assign95160_e147843_d_n8;
        locals.var_t9_dn9 = assign95160_e147843_d_n9;
        locals.var_t9_dn10 = assign95160_e147843_d_n10;
        locals.var_t9_dn13 = assign95160_e147843_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign95170_e147858, assign95170_e147858_d_n0, assign95170_e147858_d_n2, assign95170_e147858_d_n4, assign95170_e147858_d_n5, assign95170_e147858_d_n6, assign95170_e147858_d_n7, assign95170_e147858_d_n8, assign95170_e147858_d_n9, assign95170_e147858_d_n10, assign95170_e147858_d_n13,) = {
    if ((locals.var_guard2215 != 0.0) && (locals.var_guard2216 == 0.0)) {
        let assign95170_e147850: f64 = (locals.var_vgs - locals.var_vds);
        let assign95170_e147852: f64 = (assign95170_e147850 * locals.var_t5);
        let assign95170_e147855: f64 = (locals.var_t4 * locals.var_t9);
        let assign95170_e147856: f64 = (assign95170_e147852 - assign95170_e147855);
        (assign95170_e147856, ((((-locals.var_vds_dn0) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn0)) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn2)) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((((-locals.var_vds_dn4) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn4)) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((((locals.var_vgs_dn5 - locals.var_vds_dn5) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((((-locals.var_vds_dn8) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((((-locals.var_vds_dn9) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn9)) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((((-locals.var_vds_dn10) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn10)) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((((-locals.var_vds_dn13) * locals.var_t5) + (assign95170_e147850 * locals.var_t5_dn13)) - ((locals.var_t4_dn13 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn13))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn8, locals.var_qgos_dn9, locals.var_qgos_dn10, locals.var_qgos_dn13,)
    }
};
        locals.var_qgos = assign95170_e147858;
        locals.var_qgos_dn0 = assign95170_e147858_d_n0;
        locals.var_qgos_dn2 = assign95170_e147858_d_n2;
        locals.var_qgos_dn4 = assign95170_e147858_d_n4;
        locals.var_qgos_dn5 = assign95170_e147858_d_n5;
        locals.var_qgos_dn6 = assign95170_e147858_d_n6;
        locals.var_qgos_dn7 = assign95170_e147858_d_n7;
        locals.var_qgos_dn8 = assign95170_e147858_d_n8;
        locals.var_qgos_dn9 = assign95170_e147858_d_n9;
        locals.var_qgos_dn10 = assign95170_e147858_d_n10;
        locals.var_qgos_dn13 = assign95170_e147858_d_n13;
        locals.var_qgos_rv = 0.0;

        let assign95180_e147869: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2217 = assign95180_e147869;
        locals.var_guard2217_rv = 0.0;

        let (assign95190_e147873,) = {
    if (locals.var_guard2217 != 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign95190_e147873;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign95200_e147877,) = {
    if (locals.var_guard2217 != 0.0) {
        (locals.var_mks_ovslp,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign95200_e147877;
        locals.var_cov_slp_rv = 0.0;

        let (assign95210_e147881,) = {
    if (locals.var_guard2217 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign95210_e147881;
        locals.var_cov_mag_rv = 0.0;

        let (assign95220_e147887, assign95220_e147887_d_n0, assign95220_e147887_d_n2, assign95220_e147887_d_n4, assign95220_e147887_d_n5, assign95220_e147887_d_n6, assign95220_e147887_d_n7, assign95220_e147887_d_n8, assign95220_e147887_d_n9, assign95220_e147887_d_n10, assign95220_e147887_d_n13,) = {
    if (locals.var_guard2217 != 0.0) {
        let assign95220_e147885: f64 = (locals.var_coxb0 * locals.var_weffcv_nf);
        (assign95220_e147885, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign95220_e147887;
        locals.var_t1_dn0 = assign95220_e147887_d_n0;
        locals.var_t1_dn2 = assign95220_e147887_d_n2;
        locals.var_t1_dn4 = assign95220_e147887_d_n4;
        locals.var_t1_dn5 = assign95220_e147887_d_n5;
        locals.var_t1_dn6 = assign95220_e147887_d_n6;
        locals.var_t1_dn7 = assign95220_e147887_d_n7;
        locals.var_t1_dn8 = assign95220_e147887_d_n8;
        locals.var_t1_dn9 = assign95220_e147887_d_n9;
        locals.var_t1_dn10 = assign95220_e147887_d_n10;
        locals.var_t1_dn13 = assign95220_e147887_d_n13;
        locals.var_t1_rv = 0.0;

        let assign95230_e147890: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2218 = assign95230_e147890;
        locals.var_guard2218_rv = 0.0;

        let (assign95240_e147904, assign95240_e147904_d_n0, assign95240_e147904_d_n2, assign95240_e147904_d_n4, assign95240_e147904_d_n5, assign95240_e147904_d_n6, assign95240_e147904_d_n7, assign95240_e147904_d_n8, assign95240_e147904_d_n9, assign95240_e147904_d_n10, assign95240_e147904_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95240_e147896: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95240_e147899: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95240_e147901: f64 = (assign95240_e147899 - locals.var_vds);
        let assign95240_e147902: f64 = (assign95240_e147896 * assign95240_e147901);
        (assign95240_e147902, (((locals.var_cov_slp * locals.var_t1_dn0) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1_dn2) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1_dn4) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn4))), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95240_e147901) + (assign95240_e147896 * (locals.var_vgs_dn5 - locals.var_vds_dn5))), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95240_e147901) + (assign95240_e147896 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95240_e147901) + (assign95240_e147896 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1_dn8) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn8))), (((locals.var_cov_slp * locals.var_t1_dn9) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn9))), (((locals.var_cov_slp * locals.var_t1_dn10) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1_dn13) * assign95240_e147901) + (assign95240_e147896 * (-locals.var_vds_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign95240_e147904;
        locals.var_t4_dn0 = assign95240_e147904_d_n0;
        locals.var_t4_dn2 = assign95240_e147904_d_n2;
        locals.var_t4_dn4 = assign95240_e147904_d_n4;
        locals.var_t4_dn5 = assign95240_e147904_d_n5;
        locals.var_t4_dn6 = assign95240_e147904_d_n6;
        locals.var_t4_dn7 = assign95240_e147904_d_n7;
        locals.var_t4_dn8 = assign95240_e147904_d_n8;
        locals.var_t4_dn9 = assign95240_e147904_d_n9;
        locals.var_t4_dn10 = assign95240_e147904_d_n10;
        locals.var_t4_dn13 = assign95240_e147904_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign95250_e147912, assign95250_e147912_d_n0, assign95250_e147912_d_n2, assign95250_e147912_d_n4, assign95250_e147912_d_n5, assign95250_e147912_d_n6, assign95250_e147912_d_n7, assign95250_e147912_d_n8, assign95250_e147912_d_n9, assign95250_e147912_d_n10, assign95250_e147912_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95250_e147910: f64 = (p.p63 * locals.var_t1);
        (assign95250_e147910, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign95250_e147912;
        locals.var_t5_dn0 = assign95250_e147912_d_n0;
        locals.var_t5_dn2 = assign95250_e147912_d_n2;
        locals.var_t5_dn4 = assign95250_e147912_d_n4;
        locals.var_t5_dn5 = assign95250_e147912_d_n5;
        locals.var_t5_dn6 = assign95250_e147912_d_n6;
        locals.var_t5_dn7 = assign95250_e147912_d_n7;
        locals.var_t5_dn8 = assign95250_e147912_d_n8;
        locals.var_t5_dn9 = assign95250_e147912_d_n9;
        locals.var_t5_dn10 = assign95250_e147912_d_n10;
        locals.var_t5_dn13 = assign95250_e147912_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign95260_e147922, assign95260_e147922_d_n0, assign95260_e147922_d_n2, assign95260_e147922_d_n4, assign95260_e147922_d_n5, assign95260_e147922_d_n6, assign95260_e147922_d_n7, assign95260_e147922_d_n8, assign95260_e147922_d_n9, assign95260_e147922_d_n10, assign95260_e147922_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95260_e147918: f64 = (1.2 + locals.var_vds);
        let assign95260_e147920: f64 = (assign95260_e147918 - locals.var_psl);
        (assign95260_e147920, (locals.var_vds_dn0 - locals.var_psl_dn0), (locals.var_vds_dn2 - locals.var_psl_dn2), (locals.var_vds_dn4 - locals.var_psl_dn4), (locals.var_vds_dn5 - locals.var_psl_dn5), (locals.var_vds_dn6 - locals.var_psl_dn6), (locals.var_vds_dn7 - locals.var_psl_dn7), (locals.var_vds_dn8 - locals.var_psl_dn8), (locals.var_vds_dn9 - locals.var_psl_dn9), (locals.var_vds_dn10 - locals.var_psl_dn10), (locals.var_vds_dn13 - locals.var_psl_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign95260_e147922;
        locals.var_t9_dn0 = assign95260_e147922_d_n0;
        locals.var_t9_dn2 = assign95260_e147922_d_n2;
        locals.var_t9_dn4 = assign95260_e147922_d_n4;
        locals.var_t9_dn5 = assign95260_e147922_d_n5;
        locals.var_t9_dn6 = assign95260_e147922_d_n6;
        locals.var_t9_dn7 = assign95260_e147922_d_n7;
        locals.var_t9_dn8 = assign95260_e147922_d_n8;
        locals.var_t9_dn9 = assign95260_e147922_d_n9;
        locals.var_t9_dn10 = assign95260_e147922_d_n10;
        locals.var_t9_dn13 = assign95260_e147922_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign95270_e147936, assign95270_e147936_d_n0, assign95270_e147936_d_n2, assign95270_e147936_d_n4, assign95270_e147936_d_n5, assign95270_e147936_d_n6, assign95270_e147936_d_n7, assign95270_e147936_d_n8, assign95270_e147936_d_n9, assign95270_e147936_d_n10, assign95270_e147936_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 != 0.0)) {
        let assign95270_e147928: f64 = (locals.var_vgs - locals.var_vds);
        let assign95270_e147930: f64 = (assign95270_e147928 * locals.var_t5);
        let assign95270_e147933: f64 = (locals.var_t4 * locals.var_t9);
        let assign95270_e147934: f64 = (assign95270_e147930 - assign95270_e147933);
        (assign95270_e147934, ((((-locals.var_vds_dn0) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn0)) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn2)) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((((-locals.var_vds_dn4) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn4)) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), ((((locals.var_vgs_dn5 - locals.var_vds_dn5) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((((-locals.var_vds_dn8) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn8)) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((((-locals.var_vds_dn9) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn9)) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((((-locals.var_vds_dn10) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn10)) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((((-locals.var_vds_dn13) * locals.var_t5) + (assign95270_e147928 * locals.var_t5_dn13)) - ((locals.var_t4_dn13 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn13))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn13,)
    }
};
        locals.var_qgod = assign95270_e147936;
        locals.var_qgod_dn0 = assign95270_e147936_d_n0;
        locals.var_qgod_dn2 = assign95270_e147936_d_n2;
        locals.var_qgod_dn4 = assign95270_e147936_d_n4;
        locals.var_qgod_dn5 = assign95270_e147936_d_n5;
        locals.var_qgod_dn6 = assign95270_e147936_d_n6;
        locals.var_qgod_dn7 = assign95270_e147936_d_n7;
        locals.var_qgod_dn8 = assign95270_e147936_d_n8;
        locals.var_qgod_dn9 = assign95270_e147936_d_n9;
        locals.var_qgod_dn10 = assign95270_e147936_d_n10;
        locals.var_qgod_dn13 = assign95270_e147936_d_n13;
        locals.var_qgod_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_358(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95280_e147949, assign95280_e147949_d_n0, assign95280_e147949_d_n2, assign95280_e147949_d_n4, assign95280_e147949_d_n5, assign95280_e147949_d_n6, assign95280_e147949_d_n7, assign95280_e147949_d_n8, assign95280_e147949_d_n9, assign95280_e147949_d_n10, assign95280_e147949_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95280_e147943: f64 = (locals.var_cov_slp * locals.var_t1);
        let assign95280_e147946: f64 = (locals.var_cov_mag + locals.var_vgs);
        let assign95280_e147947: f64 = (assign95280_e147943 * assign95280_e147946);
        (assign95280_e147947, ((locals.var_cov_slp * locals.var_t1_dn0) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn2) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn4) * assign95280_e147946), (((locals.var_cov_slp * locals.var_t1_dn5) * assign95280_e147946) + (assign95280_e147943 * locals.var_vgs_dn5)), (((locals.var_cov_slp * locals.var_t1_dn6) * assign95280_e147946) + (assign95280_e147943 * locals.var_vgs_dn6)), (((locals.var_cov_slp * locals.var_t1_dn7) * assign95280_e147946) + (assign95280_e147943 * locals.var_vgs_dn7)), ((locals.var_cov_slp * locals.var_t1_dn8) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn9) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn10) * assign95280_e147946), ((locals.var_cov_slp * locals.var_t1_dn13) * assign95280_e147946),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign95280_e147949;
        locals.var_t4_dn0 = assign95280_e147949_d_n0;
        locals.var_t4_dn2 = assign95280_e147949_d_n2;
        locals.var_t4_dn4 = assign95280_e147949_d_n4;
        locals.var_t4_dn5 = assign95280_e147949_d_n5;
        locals.var_t4_dn6 = assign95280_e147949_d_n6;
        locals.var_t4_dn7 = assign95280_e147949_d_n7;
        locals.var_t4_dn8 = assign95280_e147949_d_n8;
        locals.var_t4_dn9 = assign95280_e147949_d_n9;
        locals.var_t4_dn10 = assign95280_e147949_d_n10;
        locals.var_t4_dn13 = assign95280_e147949_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign95290_e147958, assign95290_e147958_d_n0, assign95290_e147958_d_n2, assign95290_e147958_d_n4, assign95290_e147958_d_n5, assign95290_e147958_d_n6, assign95290_e147958_d_n7, assign95290_e147958_d_n8, assign95290_e147958_d_n9, assign95290_e147958_d_n10, assign95290_e147958_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95290_e147956: f64 = (p.p63 * locals.var_t1);
        (assign95290_e147956, (p.p63 * locals.var_t1_dn0), (p.p63 * locals.var_t1_dn2), (p.p63 * locals.var_t1_dn4), (p.p63 * locals.var_t1_dn5), (p.p63 * locals.var_t1_dn6), (p.p63 * locals.var_t1_dn7), (p.p63 * locals.var_t1_dn8), (p.p63 * locals.var_t1_dn9), (p.p63 * locals.var_t1_dn10), (p.p63 * locals.var_t1_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign95290_e147958;
        locals.var_t5_dn0 = assign95290_e147958_d_n0;
        locals.var_t5_dn2 = assign95290_e147958_d_n2;
        locals.var_t5_dn4 = assign95290_e147958_d_n4;
        locals.var_t5_dn5 = assign95290_e147958_d_n5;
        locals.var_t5_dn6 = assign95290_e147958_d_n6;
        locals.var_t5_dn7 = assign95290_e147958_d_n7;
        locals.var_t5_dn8 = assign95290_e147958_d_n8;
        locals.var_t5_dn9 = assign95290_e147958_d_n9;
        locals.var_t5_dn10 = assign95290_e147958_d_n10;
        locals.var_t5_dn13 = assign95290_e147958_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign95300_e147967, assign95300_e147967_d_n0, assign95300_e147967_d_n2, assign95300_e147967_d_n4, assign95300_e147967_d_n5, assign95300_e147967_d_n6, assign95300_e147967_d_n7, assign95300_e147967_d_n8, assign95300_e147967_d_n9, assign95300_e147967_d_n10, assign95300_e147967_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95300_e147965: f64 = (1.2 - locals.var_ps0);
        (assign95300_e147965, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn4), (-locals.var_ps0_dn5), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn8), (-locals.var_ps0_dn9), (-locals.var_ps0_dn10), (-locals.var_ps0_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign95300_e147967;
        locals.var_t9_dn0 = assign95300_e147967_d_n0;
        locals.var_t9_dn2 = assign95300_e147967_d_n2;
        locals.var_t9_dn4 = assign95300_e147967_d_n4;
        locals.var_t9_dn5 = assign95300_e147967_d_n5;
        locals.var_t9_dn6 = assign95300_e147967_d_n6;
        locals.var_t9_dn7 = assign95300_e147967_d_n7;
        locals.var_t9_dn8 = assign95300_e147967_d_n8;
        locals.var_t9_dn9 = assign95300_e147967_d_n9;
        locals.var_t9_dn10 = assign95300_e147967_d_n10;
        locals.var_t9_dn13 = assign95300_e147967_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign95310_e147980, assign95310_e147980_d_n0, assign95310_e147980_d_n2, assign95310_e147980_d_n4, assign95310_e147980_d_n5, assign95310_e147980_d_n6, assign95310_e147980_d_n7, assign95310_e147980_d_n8, assign95310_e147980_d_n9, assign95310_e147980_d_n10, assign95310_e147980_d_n13,) = {
    if ((locals.var_guard2217 != 0.0) && (locals.var_guard2218 == 0.0)) {
        let assign95310_e147974: f64 = (locals.var_vgs * locals.var_t5);
        let assign95310_e147977: f64 = (locals.var_t4 * locals.var_t9);
        let assign95310_e147978: f64 = (assign95310_e147974 - assign95310_e147977);
        (assign95310_e147978, ((locals.var_vgs * locals.var_t5_dn0) - ((locals.var_t4_dn0 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn0))), ((locals.var_vgs * locals.var_t5_dn2) - ((locals.var_t4_dn2 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn2))), ((locals.var_vgs * locals.var_t5_dn4) - ((locals.var_t4_dn4 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn4))), (((locals.var_vgs_dn5 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn5)) - ((locals.var_t4_dn5 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn5))), (((locals.var_vgs_dn6 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn6)) - ((locals.var_t4_dn6 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn6))), (((locals.var_vgs_dn7 * locals.var_t5) + (locals.var_vgs * locals.var_t5_dn7)) - ((locals.var_t4_dn7 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn7))), ((locals.var_vgs * locals.var_t5_dn8) - ((locals.var_t4_dn8 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn8))), ((locals.var_vgs * locals.var_t5_dn9) - ((locals.var_t4_dn9 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn9))), ((locals.var_vgs * locals.var_t5_dn10) - ((locals.var_t4_dn10 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn10))), ((locals.var_vgs * locals.var_t5_dn13) - ((locals.var_t4_dn13 * locals.var_t9) + (locals.var_t4 * locals.var_t9_dn13))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn8, locals.var_qgod_dn9, locals.var_qgod_dn10, locals.var_qgod_dn13,)
    }
};
        locals.var_qgod = assign95310_e147980;
        locals.var_qgod_dn0 = assign95310_e147980_d_n0;
        locals.var_qgod_dn2 = assign95310_e147980_d_n2;
        locals.var_qgod_dn4 = assign95310_e147980_d_n4;
        locals.var_qgod_dn5 = assign95310_e147980_d_n5;
        locals.var_qgod_dn6 = assign95310_e147980_d_n6;
        locals.var_qgod_dn7 = assign95310_e147980_d_n7;
        locals.var_qgod_dn8 = assign95310_e147980_d_n8;
        locals.var_qgod_dn9 = assign95310_e147980_d_n9;
        locals.var_qgod_dn10 = assign95310_e147980_d_n10;
        locals.var_qgod_dn13 = assign95310_e147980_d_n13;
        locals.var_qgod_rv = 0.0;

        let (assign95320_e147987,) = {
    if (locals.var_cgso_given != 0.0) {
        let assign95320_e147984: f64 = (-locals.var_weffcv_nf);
        let assign95320_e147985: f64 = (locals.var_uc_cgso * assign95320_e147984);
        (assign95320_e147985,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95320_e147987;
        locals.var_cgsoe_rv = 0.0;

        let assign95330_e147990: f64 = if locals.var_flg_coovlps == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2219 = assign95330_e147990;
        locals.var_guard2219_rv = 0.0;

        let (assign95340_e148002,) = {
    if ((locals.var_cgso_given == 0.0) && (locals.var_guard2219 != 0.0)) {
        let assign95340_e147996: f64 = (-locals.var_cox0);
        let assign95340_e147998: f64 = (assign95340_e147996 * p.p66);
        let assign95340_e148000: f64 = (assign95340_e147998 * locals.var_weffcv_nf);
        (assign95340_e148000,)
    } else {
        (locals.var_cgsoe,)
    }
};
        locals.var_cgsoe = assign95340_e148002;
        locals.var_cgsoe_rv = 0.0;

        let assign95350_e148004: f64 = (-locals.var_cgsoe);
        let assign95350_e148006: f64 = (assign95350_e148004 * locals.var_vgsei);
        locals.var_qgso = assign95350_e148006;
        locals.var_qgso_dn2 = (assign95350_e148004 * locals.var_vgsei_dn2);
        locals.var_qgso_dn6 = (assign95350_e148004 * locals.var_vgsei_dn6);
        locals.var_qgso_rv = 0.0;

        let (assign95360_e148013,) = {
    if (locals.var_cgdo_given != 0.0) {
        let assign95360_e148010: f64 = (-locals.var_weffcv_nf);
        let assign95360_e148011: f64 = (locals.var_uc_cgdo * assign95360_e148010);
        (assign95360_e148011,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95360_e148013;
        locals.var_cgdoe_rv = 0.0;

        let assign95370_e148016: f64 = if locals.var_flg_coovlp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2220 = assign95370_e148016;
        locals.var_guard2220_rv = 0.0;

        let (assign95380_e148028,) = {
    if ((locals.var_cgdo_given == 0.0) && (locals.var_guard2220 != 0.0)) {
        let assign95380_e148022: f64 = (-locals.var_coxb0);
        let assign95380_e148024: f64 = (assign95380_e148022 * p.p63);
        let assign95380_e148026: f64 = (assign95380_e148024 * locals.var_weffcv_nf);
        (assign95380_e148026,)
    } else {
        (locals.var_cgdoe,)
    }
};
        locals.var_cgdoe = assign95380_e148028;
        locals.var_cgdoe_rv = 0.0;

        let assign95390_e148030: f64 = (-locals.var_cgdoe);
        let assign95390_e148033: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign95390_e148034: f64 = (assign95390_e148030 * assign95390_e148033);
        locals.var_qgdo = assign95390_e148034;
        locals.var_qgdo_dn0 = (assign95390_e148030 * (-locals.var_vdsei_dn0));
        locals.var_qgdo_dn2 = (assign95390_e148030 * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qgdo_dn6 = (assign95390_e148030 * locals.var_vgsei_dn6);
        locals.var_qgdo_rv = 0.0;

        let assign95400_e148037: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2221 = assign95400_e148037;
        locals.var_guard2221_rv = 0.0;

        let (assign95410_e148045, assign95410_e148045_d_n0, assign95410_e148045_d_n2, assign95410_e148045_d_n4, assign95410_e148045_d_n5, assign95410_e148045_d_n6, assign95410_e148045_d_n7, assign95410_e148045_d_n8, assign95410_e148045_d_n9, assign95410_e148045_d_n10, assign95410_e148045_d_n13,) = {
    if (locals.var_guard2221 != 0.0) {
        let assign95410_e148042: f64 = (locals.var_vds - locals.var_pds);
        let assign95410_e148043: f64 = (p.p431 * assign95410_e148042);
        (assign95410_e148043, (p.p431 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (p.p431 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (p.p431 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (p.p431 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (p.p431 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (p.p431 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (p.p431 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (p.p431 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (p.p431 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (p.p431 * (locals.var_vds_dn13 - locals.var_pds_dn13)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn13,)
    }
};
        locals.var_qodad = assign95410_e148045;
        locals.var_qodad_dn0 = assign95410_e148045_d_n0;
        locals.var_qodad_dn2 = assign95410_e148045_d_n2;
        locals.var_qodad_dn4 = assign95410_e148045_d_n4;
        locals.var_qodad_dn5 = assign95410_e148045_d_n5;
        locals.var_qodad_dn6 = assign95410_e148045_d_n6;
        locals.var_qodad_dn7 = assign95410_e148045_d_n7;
        locals.var_qodad_dn8 = assign95410_e148045_d_n8;
        locals.var_qodad_dn9 = assign95410_e148045_d_n9;
        locals.var_qodad_dn10 = assign95410_e148045_d_n10;
        locals.var_qodad_dn13 = assign95410_e148045_d_n13;
        locals.var_qodad_rv = 0.0;

        let (assign95420_e148051, assign95420_e148051_d_n0, assign95420_e148051_d_n2, assign95420_e148051_d_n4, assign95420_e148051_d_n5, assign95420_e148051_d_n6, assign95420_e148051_d_n7, assign95420_e148051_d_n8, assign95420_e148051_d_n9, assign95420_e148051_d_n10, assign95420_e148051_d_n13,) = {
    if (locals.var_guard2221 != 0.0) {
        let assign95420_e148049: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95420_e148049, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn13 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn13)),)
    } else {
        (locals.var_qovd_add, locals.var_qovd_add_dn0, locals.var_qovd_add_dn2, locals.var_qovd_add_dn4, locals.var_qovd_add_dn5, locals.var_qovd_add_dn6, locals.var_qovd_add_dn7, locals.var_qovd_add_dn8, locals.var_qovd_add_dn9, locals.var_qovd_add_dn10, locals.var_qovd_add_dn13,)
    }
};
        locals.var_qovd_add = assign95420_e148051;
        locals.var_qovd_add_dn0 = assign95420_e148051_d_n0;
        locals.var_qovd_add_dn2 = assign95420_e148051_d_n2;
        locals.var_qovd_add_dn4 = assign95420_e148051_d_n4;
        locals.var_qovd_add_dn5 = assign95420_e148051_d_n5;
        locals.var_qovd_add_dn6 = assign95420_e148051_d_n6;
        locals.var_qovd_add_dn7 = assign95420_e148051_d_n7;
        locals.var_qovd_add_dn8 = assign95420_e148051_d_n8;
        locals.var_qovd_add_dn9 = assign95420_e148051_d_n9;
        locals.var_qovd_add_dn10 = assign95420_e148051_d_n10;
        locals.var_qovd_add_dn13 = assign95420_e148051_d_n13;
        locals.var_qovd_add_rv = 0.0;

        let (assign95430_e148057, assign95430_e148057_d_n0, assign95430_e148057_d_n2, assign95430_e148057_d_n4, assign95430_e148057_d_n5, assign95430_e148057_d_n6, assign95430_e148057_d_n7, assign95430_e148057_d_n8, assign95430_e148057_d_n9, assign95430_e148057_d_n10, assign95430_e148057_d_n13,) = {
    if (locals.var_guard2221 != 0.0) {
        let assign95430_e148055: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95430_e148055, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn13 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn13)),)
    } else {
        (locals.var_qbdld_add, locals.var_qbdld_add_dn0, locals.var_qbdld_add_dn2, locals.var_qbdld_add_dn4, locals.var_qbdld_add_dn5, locals.var_qbdld_add_dn6, locals.var_qbdld_add_dn7, locals.var_qbdld_add_dn8, locals.var_qbdld_add_dn9, locals.var_qbdld_add_dn10, locals.var_qbdld_add_dn13,)
    }
};
        locals.var_qbdld_add = assign95430_e148057;
        locals.var_qbdld_add_dn0 = assign95430_e148057_d_n0;
        locals.var_qbdld_add_dn2 = assign95430_e148057_d_n2;
        locals.var_qbdld_add_dn4 = assign95430_e148057_d_n4;
        locals.var_qbdld_add_dn5 = assign95430_e148057_d_n5;
        locals.var_qbdld_add_dn6 = assign95430_e148057_d_n6;
        locals.var_qbdld_add_dn7 = assign95430_e148057_d_n7;
        locals.var_qbdld_add_dn8 = assign95430_e148057_d_n8;
        locals.var_qbdld_add_dn9 = assign95430_e148057_d_n9;
        locals.var_qbdld_add_dn10 = assign95430_e148057_d_n10;
        locals.var_qbdld_add_dn13 = assign95430_e148057_d_n13;
        locals.var_qbdld_add_rv = 0.0;

        let (assign95440_e148067, assign95440_e148067_d_n0, assign95440_e148067_d_n2, assign95440_e148067_d_n4, assign95440_e148067_d_n5, assign95440_e148067_d_n6, assign95440_e148067_d_n7, assign95440_e148067_d_n8, assign95440_e148067_d_n9, assign95440_e148067_d_n10, assign95440_e148067_d_n13,) = {
    if (locals.var_guard2221 == 0.0) {
        let assign95440_e148061: f64 = (-p.p431);
        let assign95440_e148064: f64 = (locals.var_vds - locals.var_pds);
        let assign95440_e148065: f64 = (assign95440_e148061 * assign95440_e148064);
        (assign95440_e148065, (assign95440_e148061 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (assign95440_e148061 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (assign95440_e148061 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (assign95440_e148061 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (assign95440_e148061 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (assign95440_e148061 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (assign95440_e148061 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (assign95440_e148061 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (assign95440_e148061 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (assign95440_e148061 * (locals.var_vds_dn13 - locals.var_pds_dn13)),)
    } else {
        (locals.var_qodad, locals.var_qodad_dn0, locals.var_qodad_dn2, locals.var_qodad_dn4, locals.var_qodad_dn5, locals.var_qodad_dn6, locals.var_qodad_dn7, locals.var_qodad_dn8, locals.var_qodad_dn9, locals.var_qodad_dn10, locals.var_qodad_dn13,)
    }
};
        locals.var_qodad = assign95440_e148067;
        locals.var_qodad_dn0 = assign95440_e148067_d_n0;
        locals.var_qodad_dn2 = assign95440_e148067_d_n2;
        locals.var_qodad_dn4 = assign95440_e148067_d_n4;
        locals.var_qodad_dn5 = assign95440_e148067_d_n5;
        locals.var_qodad_dn6 = assign95440_e148067_d_n6;
        locals.var_qodad_dn7 = assign95440_e148067_d_n7;
        locals.var_qodad_dn8 = assign95440_e148067_d_n8;
        locals.var_qodad_dn9 = assign95440_e148067_d_n9;
        locals.var_qodad_dn10 = assign95440_e148067_d_n10;
        locals.var_qodad_dn13 = assign95440_e148067_d_n13;
        locals.var_qodad_rv = 0.0;

        let (assign95450_e148074, assign95450_e148074_d_n0, assign95450_e148074_d_n2, assign95450_e148074_d_n4, assign95450_e148074_d_n5, assign95450_e148074_d_n6, assign95450_e148074_d_n7, assign95450_e148074_d_n8, assign95450_e148074_d_n9, assign95450_e148074_d_n10, assign95450_e148074_d_n13,) = {
    if (locals.var_guard2221 == 0.0) {
        let assign95450_e148072: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95450_e148072, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn13 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn13)),)
    } else {
        (locals.var_qovs_add, locals.var_qovs_add_dn0, locals.var_qovs_add_dn2, locals.var_qovs_add_dn4, locals.var_qovs_add_dn5, locals.var_qovs_add_dn6, locals.var_qovs_add_dn7, locals.var_qovs_add_dn8, locals.var_qovs_add_dn9, locals.var_qovs_add_dn10, locals.var_qovs_add_dn13,)
    }
};
        locals.var_qovs_add = assign95450_e148074;
        locals.var_qovs_add_dn0 = assign95450_e148074_d_n0;
        locals.var_qovs_add_dn2 = assign95450_e148074_d_n2;
        locals.var_qovs_add_dn4 = assign95450_e148074_d_n4;
        locals.var_qovs_add_dn5 = assign95450_e148074_d_n5;
        locals.var_qovs_add_dn6 = assign95450_e148074_d_n6;
        locals.var_qovs_add_dn7 = assign95450_e148074_d_n7;
        locals.var_qovs_add_dn8 = assign95450_e148074_d_n8;
        locals.var_qovs_add_dn9 = assign95450_e148074_d_n9;
        locals.var_qovs_add_dn10 = assign95450_e148074_d_n10;
        locals.var_qovs_add_dn13 = assign95450_e148074_d_n13;
        locals.var_qovs_add_rv = 0.0;

        let (assign95460_e148081, assign95460_e148081_d_n0, assign95460_e148081_d_n2, assign95460_e148081_d_n4, assign95460_e148081_d_n5, assign95460_e148081_d_n6, assign95460_e148081_d_n7, assign95460_e148081_d_n8, assign95460_e148081_d_n9, assign95460_e148081_d_n10, assign95460_e148081_d_n13,) = {
    if (locals.var_guard2221 == 0.0) {
        let assign95460_e148079: f64 = (locals.var_t4 * locals.var_qodad);
        (assign95460_e148079, ((locals.var_t4_dn0 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn0)), ((locals.var_t4_dn2 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn2)), ((locals.var_t4_dn4 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn4)), ((locals.var_t4_dn5 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn5)), ((locals.var_t4_dn6 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn6)), ((locals.var_t4_dn7 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn7)), ((locals.var_t4_dn8 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn8)), ((locals.var_t4_dn9 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn9)), ((locals.var_t4_dn10 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn10)), ((locals.var_t4_dn13 * locals.var_qodad) + (locals.var_t4 * locals.var_qodad_dn13)),)
    } else {
        (locals.var_qbsld_add, locals.var_qbsld_add_dn0, locals.var_qbsld_add_dn2, locals.var_qbsld_add_dn4, locals.var_qbsld_add_dn5, locals.var_qbsld_add_dn6, locals.var_qbsld_add_dn7, locals.var_qbsld_add_dn8, locals.var_qbsld_add_dn9, locals.var_qbsld_add_dn10, locals.var_qbsld_add_dn13,)
    }
};
        locals.var_qbsld_add = assign95460_e148081;
        locals.var_qbsld_add_dn0 = assign95460_e148081_d_n0;
        locals.var_qbsld_add_dn2 = assign95460_e148081_d_n2;
        locals.var_qbsld_add_dn4 = assign95460_e148081_d_n4;
        locals.var_qbsld_add_dn5 = assign95460_e148081_d_n5;
        locals.var_qbsld_add_dn6 = assign95460_e148081_d_n6;
        locals.var_qbsld_add_dn7 = assign95460_e148081_d_n7;
        locals.var_qbsld_add_dn8 = assign95460_e148081_d_n8;
        locals.var_qbsld_add_dn9 = assign95460_e148081_d_n9;
        locals.var_qbsld_add_dn10 = assign95460_e148081_d_n10;
        locals.var_qbsld_add_dn13 = assign95460_e148081_d_n13;
        locals.var_qbsld_add_rv = 0.0;

        let assign95470_e148083: f64 = (-locals.var_uc_cgbo);
        let assign95470_e148085: f64 = (assign95470_e148083 * locals.var_lgate);
        locals.var_cgbo_loc = assign95470_e148085;
        locals.var_cgbo_loc_rv = 0.0;

        let assign95480_e148087: f64 = (-locals.var_cgbo_loc);
        let assign95480_e148090: f64 = (locals.var_vgsi - locals.var_vbsi);
        let assign95480_e148091: f64 = (assign95480_e148087 * assign95480_e148090);
        locals.var_qgbo = assign95480_e148091;
        locals.var_qgbo_dn6 = (assign95480_e148087 * locals.var_vgsi_dn6);
        locals.var_qgbo_dn7 = (assign95480_e148087 * (locals.var_vgsi_dn7 - locals.var_vbsi_dn7));
        locals.var_qgbo_dn8 = (assign95480_e148087 * (-locals.var_vbsi_dn8));
        locals.var_qgbo_rv = 0.0;

        locals.var_aclm = locals.var_uc_clm1;
        locals.var_aclm_rv = 0.0;

        let assign95500_e148095: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2222 = assign95500_e148095;
        locals.var_guard2222_rv = 0.0;

        let (assign95510_e148109, assign95510_e148109_d_n0, assign95510_e148109_d_n2, assign95510_e148109_d_n4, assign95510_e148109_d_n5, assign95510_e148109_d_n6, assign95510_e148109_d_n7, assign95510_e148109_d_n8, assign95510_e148109_d_n9, assign95510_e148109_d_n10, assign95510_e148109_d_n13,) = {
    if (locals.var_guard2222 != 0.0) {
        let assign95510_e148100: f64 = (locals.var_vds + locals.var_ps0);
        let assign95510_e148101: f64 = (locals.var_aclm * assign95510_e148100);
        let assign95510_e148104: f64 = (1.0 - locals.var_aclm);
        let assign95510_e148106: f64 = (assign95510_e148104 * locals.var_psl);
        let assign95510_e148107: f64 = (assign95510_e148101 + assign95510_e148106);
        (assign95510_e148107, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign95510_e148104 * locals.var_psl_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign95510_e148104 * locals.var_psl_dn2)), ((locals.var_aclm * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + (assign95510_e148104 * locals.var_psl_dn4)), ((locals.var_aclm * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + (assign95510_e148104 * locals.var_psl_dn5)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign95510_e148104 * locals.var_psl_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign95510_e148104 * locals.var_psl_dn7)), ((locals.var_aclm * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + (assign95510_e148104 * locals.var_psl_dn8)), ((locals.var_aclm * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + (assign95510_e148104 * locals.var_psl_dn9)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign95510_e148104 * locals.var_psl_dn10)), ((locals.var_aclm * (locals.var_vds_dn13 + locals.var_ps0_dn13)) + (assign95510_e148104 * locals.var_psl_dn13)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign95510_e148109;
        locals.var_psdl_dn0 = assign95510_e148109_d_n0;
        locals.var_psdl_dn2 = assign95510_e148109_d_n2;
        locals.var_psdl_dn4 = assign95510_e148109_d_n4;
        locals.var_psdl_dn5 = assign95510_e148109_d_n5;
        locals.var_psdl_dn6 = assign95510_e148109_d_n6;
        locals.var_psdl_dn7 = assign95510_e148109_d_n7;
        locals.var_psdl_dn8 = assign95510_e148109_d_n8;
        locals.var_psdl_dn9 = assign95510_e148109_d_n9;
        locals.var_psdl_dn10 = assign95510_e148109_d_n10;
        locals.var_psdl_dn13 = assign95510_e148109_d_n13;
        locals.var_psdl_rv = 0.0;

        let assign95520_e148113: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95520_e148116: f64 = (10.0 * 2.220446049250313e-16);
        let assign95520_e148117: f64 = (assign95520_e148113 - assign95520_e148116);
        let assign95520_e148120: f64 = (10.0 * 2.220446049250313e-16);
        let assign95520_e148121: f64 = (assign95520_e148117 - assign95520_e148120);
        let assign95520_e148125: f64 = (10.0 * 2.220446049250313e-16);
        let assign95520_e148128: f64 = if ((locals.var_psdl > assign95520_e148121) && (assign95520_e148125 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2223 = assign95520_e148128;
        locals.var_guard2223_rv = 0.0;

        let (assign95530_e148146, assign95530_e148146_d_n0, assign95530_e148146_d_n2, assign95530_e148146_d_n4, assign95530_e148146_d_n5, assign95530_e148146_d_n6, assign95530_e148146_d_n7, assign95530_e148146_d_n8, assign95530_e148146_d_n9, assign95530_e148146_d_n10, assign95530_e148146_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95530_e148135: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95530_e148138: f64 = (10.0 * 2.220446049250313e-16);
        let assign95530_e148139: f64 = (assign95530_e148135 - assign95530_e148138);
        let assign95530_e148140: f64 = (locals.var_psdl - assign95530_e148139);
        let assign95530_e148143: f64 = (10.0 * 2.220446049250313e-16);
        let assign95530_e148144: f64 = (assign95530_e148140 + assign95530_e148143);
        (assign95530_e148144, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn13 - (locals.var_ps0_dn13 + locals.var_vds_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign95530_e148146;
        locals.var_tmf1_dn0 = assign95530_e148146_d_n0;
        locals.var_tmf1_dn2 = assign95530_e148146_d_n2;
        locals.var_tmf1_dn4 = assign95530_e148146_d_n4;
        locals.var_tmf1_dn5 = assign95530_e148146_d_n5;
        locals.var_tmf1_dn6 = assign95530_e148146_d_n6;
        locals.var_tmf1_dn7 = assign95530_e148146_d_n7;
        locals.var_tmf1_dn8 = assign95530_e148146_d_n8;
        locals.var_tmf1_dn9 = assign95530_e148146_d_n9;
        locals.var_tmf1_dn10 = assign95530_e148146_d_n10;
        locals.var_tmf1_dn13 = assign95530_e148146_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign95540_e148154, assign95540_e148154_d_n0, assign95540_e148154_d_n2, assign95540_e148154_d_n4, assign95540_e148154_d_n5, assign95540_e148154_d_n6, assign95540_e148154_d_n7, assign95540_e148154_d_n8, assign95540_e148154_d_n9, assign95540_e148154_d_n10, assign95540_e148154_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95540_e148152: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign95540_e148152, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign95540_e148154;
        locals.var_x2_dn0 = assign95540_e148154_d_n0;
        locals.var_x2_dn2 = assign95540_e148154_d_n2;
        locals.var_x2_dn4 = assign95540_e148154_d_n4;
        locals.var_x2_dn5 = assign95540_e148154_d_n5;
        locals.var_x2_dn6 = assign95540_e148154_d_n6;
        locals.var_x2_dn7 = assign95540_e148154_d_n7;
        locals.var_x2_dn8 = assign95540_e148154_d_n8;
        locals.var_x2_dn9 = assign95540_e148154_d_n9;
        locals.var_x2_dn10 = assign95540_e148154_d_n10;
        locals.var_x2_dn13 = assign95540_e148154_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign95550_e148166, assign95550_e148166_d_n0, assign95550_e148166_d_n2, assign95550_e148166_d_n4, assign95550_e148166_d_n5, assign95550_e148166_d_n6, assign95550_e148166_d_n7, assign95550_e148166_d_n8, assign95550_e148166_d_n9, assign95550_e148166_d_n10, assign95550_e148166_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95550_e148160: f64 = (10.0 * 2.220446049250313e-16);
        let assign95550_e148163: f64 = (10.0 * 2.220446049250313e-16);
        let assign95550_e148164: f64 = (assign95550_e148160 * assign95550_e148163);
        (assign95550_e148164, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign95550_e148166;
        locals.var_xmax2_dn0 = assign95550_e148166_d_n0;
        locals.var_xmax2_dn2 = assign95550_e148166_d_n2;
        locals.var_xmax2_dn4 = assign95550_e148166_d_n4;
        locals.var_xmax2_dn5 = assign95550_e148166_d_n5;
        locals.var_xmax2_dn6 = assign95550_e148166_d_n6;
        locals.var_xmax2_dn7 = assign95550_e148166_d_n7;
        locals.var_xmax2_dn8 = assign95550_e148166_d_n8;
        locals.var_xmax2_dn9 = assign95550_e148166_d_n9;
        locals.var_xmax2_dn10 = assign95550_e148166_d_n10;
        locals.var_xmax2_dn13 = assign95550_e148166_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign95560_e148172, assign95560_e148172_d_n0, assign95560_e148172_d_n2, assign95560_e148172_d_n4, assign95560_e148172_d_n5, assign95560_e148172_d_n6, assign95560_e148172_d_n7, assign95560_e148172_d_n8, assign95560_e148172_d_n9, assign95560_e148172_d_n10, assign95560_e148172_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign95560_e148172;
        locals.var_xp_dn0 = assign95560_e148172_d_n0;
        locals.var_xp_dn2 = assign95560_e148172_d_n2;
        locals.var_xp_dn4 = assign95560_e148172_d_n4;
        locals.var_xp_dn5 = assign95560_e148172_d_n5;
        locals.var_xp_dn6 = assign95560_e148172_d_n6;
        locals.var_xp_dn7 = assign95560_e148172_d_n7;
        locals.var_xp_dn8 = assign95560_e148172_d_n8;
        locals.var_xp_dn9 = assign95560_e148172_d_n9;
        locals.var_xp_dn10 = assign95560_e148172_d_n10;
        locals.var_xp_dn13 = assign95560_e148172_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign95570_e148178, assign95570_e148178_d_n0, assign95570_e148178_d_n2, assign95570_e148178_d_n4, assign95570_e148178_d_n5, assign95570_e148178_d_n6, assign95570_e148178_d_n7, assign95570_e148178_d_n8, assign95570_e148178_d_n9, assign95570_e148178_d_n10, assign95570_e148178_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign95570_e148178;
        locals.var_xmp_dn0 = assign95570_e148178_d_n0;
        locals.var_xmp_dn2 = assign95570_e148178_d_n2;
        locals.var_xmp_dn4 = assign95570_e148178_d_n4;
        locals.var_xmp_dn5 = assign95570_e148178_d_n5;
        locals.var_xmp_dn6 = assign95570_e148178_d_n6;
        locals.var_xmp_dn7 = assign95570_e148178_d_n7;
        locals.var_xmp_dn8 = assign95570_e148178_d_n8;
        locals.var_xmp_dn9 = assign95570_e148178_d_n9;
        locals.var_xmp_dn10 = assign95570_e148178_d_n10;
        locals.var_xmp_dn13 = assign95570_e148178_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign95580_e148184,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95580_e148184;
        locals.var_m0_rv = 0.0;

        let (assign95590_e148190,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95590_e148190;
        locals.var_mm_rv = 0.0;

        let (assign95600_e148196, assign95600_e148196_d_n0, assign95600_e148196_d_n2, assign95600_e148196_d_n4, assign95600_e148196_d_n5, assign95600_e148196_d_n6, assign95600_e148196_d_n7, assign95600_e148196_d_n8, assign95600_e148196_d_n9, assign95600_e148196_d_n10, assign95600_e148196_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign95600_e148196;
        locals.var_arg_dn0 = assign95600_e148196_d_n0;
        locals.var_arg_dn2 = assign95600_e148196_d_n2;
        locals.var_arg_dn4 = assign95600_e148196_d_n4;
        locals.var_arg_dn5 = assign95600_e148196_d_n5;
        locals.var_arg_dn6 = assign95600_e148196_d_n6;
        locals.var_arg_dn7 = assign95600_e148196_d_n7;
        locals.var_arg_dn8 = assign95600_e148196_d_n8;
        locals.var_arg_dn9 = assign95600_e148196_d_n9;
        locals.var_arg_dn10 = assign95600_e148196_d_n10;
        locals.var_arg_dn13 = assign95600_e148196_d_n13;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_359(
        locals: &mut StampLocals,
    ) {
        let (assign95610_e148202, assign95610_e148202_d_n0, assign95610_e148202_d_n2, assign95610_e148202_d_n4, assign95610_e148202_d_n5, assign95610_e148202_d_n6, assign95610_e148202_d_n7, assign95610_e148202_d_n8, assign95610_e148202_d_n9, assign95610_e148202_d_n10, assign95610_e148202_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95610_e148202;
        locals.var_dnm_dn0 = assign95610_e148202_d_n0;
        locals.var_dnm_dn2 = assign95610_e148202_d_n2;
        locals.var_dnm_dn4 = assign95610_e148202_d_n4;
        locals.var_dnm_dn5 = assign95610_e148202_d_n5;
        locals.var_dnm_dn6 = assign95610_e148202_d_n6;
        locals.var_dnm_dn7 = assign95610_e148202_d_n7;
        locals.var_dnm_dn8 = assign95610_e148202_d_n8;
        locals.var_dnm_dn9 = assign95610_e148202_d_n9;
        locals.var_dnm_dn10 = assign95610_e148202_d_n10;
        locals.var_dnm_dn13 = assign95610_e148202_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign95620_e148210, assign95620_e148210_d_n0, assign95620_e148210_d_n2, assign95620_e148210_d_n4, assign95620_e148210_d_n5, assign95620_e148210_d_n6, assign95620_e148210_d_n7, assign95620_e148210_d_n8, assign95620_e148210_d_n9, assign95620_e148210_d_n10, assign95620_e148210_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95620_e148208: f64 = (locals.var_xp * locals.var_x2);
        (assign95620_e148208, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign95620_e148210;
        locals.var_xp_dn0 = assign95620_e148210_d_n0;
        locals.var_xp_dn2 = assign95620_e148210_d_n2;
        locals.var_xp_dn4 = assign95620_e148210_d_n4;
        locals.var_xp_dn5 = assign95620_e148210_d_n5;
        locals.var_xp_dn6 = assign95620_e148210_d_n6;
        locals.var_xp_dn7 = assign95620_e148210_d_n7;
        locals.var_xp_dn8 = assign95620_e148210_d_n8;
        locals.var_xp_dn9 = assign95620_e148210_d_n9;
        locals.var_xp_dn10 = assign95620_e148210_d_n10;
        locals.var_xp_dn13 = assign95620_e148210_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign95630_e148218, assign95630_e148218_d_n0, assign95630_e148218_d_n2, assign95630_e148218_d_n4, assign95630_e148218_d_n5, assign95630_e148218_d_n6, assign95630_e148218_d_n7, assign95630_e148218_d_n8, assign95630_e148218_d_n9, assign95630_e148218_d_n10, assign95630_e148218_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95630_e148216: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95630_e148216, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign95630_e148218;
        locals.var_xmp_dn0 = assign95630_e148218_d_n0;
        locals.var_xmp_dn2 = assign95630_e148218_d_n2;
        locals.var_xmp_dn4 = assign95630_e148218_d_n4;
        locals.var_xmp_dn5 = assign95630_e148218_d_n5;
        locals.var_xmp_dn6 = assign95630_e148218_d_n6;
        locals.var_xmp_dn7 = assign95630_e148218_d_n7;
        locals.var_xmp_dn8 = assign95630_e148218_d_n8;
        locals.var_xmp_dn9 = assign95630_e148218_d_n9;
        locals.var_xmp_dn10 = assign95630_e148218_d_n10;
        locals.var_xmp_dn13 = assign95630_e148218_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign95640_e148226, assign95640_e148226_d_n0, assign95640_e148226_d_n2, assign95640_e148226_d_n4, assign95640_e148226_d_n5, assign95640_e148226_d_n6, assign95640_e148226_d_n7, assign95640_e148226_d_n8, assign95640_e148226_d_n9, assign95640_e148226_d_n10, assign95640_e148226_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95640_e148224: f64 = (locals.var_xp * locals.var_x2);
        (assign95640_e148224, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign95640_e148226;
        locals.var_xp_dn0 = assign95640_e148226_d_n0;
        locals.var_xp_dn2 = assign95640_e148226_d_n2;
        locals.var_xp_dn4 = assign95640_e148226_d_n4;
        locals.var_xp_dn5 = assign95640_e148226_d_n5;
        locals.var_xp_dn6 = assign95640_e148226_d_n6;
        locals.var_xp_dn7 = assign95640_e148226_d_n7;
        locals.var_xp_dn8 = assign95640_e148226_d_n8;
        locals.var_xp_dn9 = assign95640_e148226_d_n9;
        locals.var_xp_dn10 = assign95640_e148226_d_n10;
        locals.var_xp_dn13 = assign95640_e148226_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign95650_e148234, assign95650_e148234_d_n0, assign95650_e148234_d_n2, assign95650_e148234_d_n4, assign95650_e148234_d_n5, assign95650_e148234_d_n6, assign95650_e148234_d_n7, assign95650_e148234_d_n8, assign95650_e148234_d_n9, assign95650_e148234_d_n10, assign95650_e148234_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95650_e148232: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95650_e148232, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign95650_e148234;
        locals.var_xmp_dn0 = assign95650_e148234_d_n0;
        locals.var_xmp_dn2 = assign95650_e148234_d_n2;
        locals.var_xmp_dn4 = assign95650_e148234_d_n4;
        locals.var_xmp_dn5 = assign95650_e148234_d_n5;
        locals.var_xmp_dn6 = assign95650_e148234_d_n6;
        locals.var_xmp_dn7 = assign95650_e148234_d_n7;
        locals.var_xmp_dn8 = assign95650_e148234_d_n8;
        locals.var_xmp_dn9 = assign95650_e148234_d_n9;
        locals.var_xmp_dn10 = assign95650_e148234_d_n10;
        locals.var_xmp_dn13 = assign95650_e148234_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign95660_e148242, assign95660_e148242_d_n0, assign95660_e148242_d_n2, assign95660_e148242_d_n4, assign95660_e148242_d_n5, assign95660_e148242_d_n6, assign95660_e148242_d_n7, assign95660_e148242_d_n8, assign95660_e148242_d_n9, assign95660_e148242_d_n10, assign95660_e148242_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95660_e148240: f64 = (locals.var_xp + locals.var_xmp);
        (assign95660_e148240, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign95660_e148242;
        locals.var_arg_dn0 = assign95660_e148242_d_n0;
        locals.var_arg_dn2 = assign95660_e148242_d_n2;
        locals.var_arg_dn4 = assign95660_e148242_d_n4;
        locals.var_arg_dn5 = assign95660_e148242_d_n5;
        locals.var_arg_dn6 = assign95660_e148242_d_n6;
        locals.var_arg_dn7 = assign95660_e148242_d_n7;
        locals.var_arg_dn8 = assign95660_e148242_d_n8;
        locals.var_arg_dn9 = assign95660_e148242_d_n9;
        locals.var_arg_dn10 = assign95660_e148242_d_n10;
        locals.var_arg_dn13 = assign95660_e148242_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign95670_e148248, assign95670_e148248_d_n0, assign95670_e148248_d_n2, assign95670_e148248_d_n4, assign95670_e148248_d_n5, assign95670_e148248_d_n6, assign95670_e148248_d_n7, assign95670_e148248_d_n8, assign95670_e148248_d_n9, assign95670_e148248_d_n10, assign95670_e148248_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95670_e148248;
        locals.var_dnm_dn0 = assign95670_e148248_d_n0;
        locals.var_dnm_dn2 = assign95670_e148248_d_n2;
        locals.var_dnm_dn4 = assign95670_e148248_d_n4;
        locals.var_dnm_dn5 = assign95670_e148248_d_n5;
        locals.var_dnm_dn6 = assign95670_e148248_d_n6;
        locals.var_dnm_dn7 = assign95670_e148248_d_n7;
        locals.var_dnm_dn8 = assign95670_e148248_d_n8;
        locals.var_dnm_dn9 = assign95670_e148248_d_n9;
        locals.var_dnm_dn10 = assign95670_e148248_d_n10;
        locals.var_dnm_dn13 = assign95670_e148248_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign95680_e148263: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2224 = assign95680_e148263;
        locals.var_guard2224_rv = 0.0;

        let assign95690_e148266: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2225 = assign95690_e148266;
        locals.var_guard2225_rv = 0.0;

        let (assign95700_e148276,) = {
    if ((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95700_e148276;
        locals.var_mm_rv = 0.0;

        let assign95710_e148279: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2226 = assign95710_e148279;
        locals.var_guard2226_rv = 0.0;

        let (assign95720_e148292,) = {
    if (((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95720_e148292;
        locals.var_mm_rv = 0.0;

        let assign95730_e148295: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2227 = assign95730_e148295;
        locals.var_guard2227_rv = 0.0;

        let (assign95740_e148311,) = {
    if ((((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 == 0.0)) && (locals.var_guard2227 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95740_e148311;
        locals.var_mm_rv = 0.0;

        let assign95750_e148314: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2228 = assign95750_e148314;
        locals.var_guard2228_rv = 0.0;

        let (assign95760_e148333,) = {
    if (((((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 == 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95760_e148333;
        locals.var_mm_rv = 0.0;

        let (assign95770_e148341,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95770_e148341;
        locals.var_m0_rv = 0.0;

        let mut assign95780_loop_guard: usize = 0;
        while {
            let assign95780_cond_e148350: f64 = if ((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign95780_cond_e148350 != 0.0
        } {
            assign95780_loop_guard += 1;
            assert!(assign95780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign95780_body0_e148359, assign95780_body0_e148359_d_n0, assign95780_body0_e148359_d_n2, assign95780_body0_e148359_d_n4, assign95780_body0_e148359_d_n5, assign95780_body0_e148359_d_n6, assign95780_body0_e148359_d_n7, assign95780_body0_e148359_d_n8, assign95780_body0_e148359_d_n9, assign95780_body0_e148359_d_n10, assign95780_body0_e148359_d_n13,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        let assign95780_body0_e148357: f64 = (locals.var_dnm).sqrt();
        (assign95780_body0_e148357, (locals.var_dnm_dn0 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn2 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn4 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn5 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn6 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn7 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn8 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn9 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn10 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn13 / (2.0 * assign95780_body0_e148357)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign95780_body0_e148359;
            locals.var_dnm_dn0 = assign95780_body0_e148359_d_n0;
            locals.var_dnm_dn2 = assign95780_body0_e148359_d_n2;
            locals.var_dnm_dn4 = assign95780_body0_e148359_d_n4;
            locals.var_dnm_dn5 = assign95780_body0_e148359_d_n5;
            locals.var_dnm_dn6 = assign95780_body0_e148359_d_n6;
            locals.var_dnm_dn7 = assign95780_body0_e148359_d_n7;
            locals.var_dnm_dn8 = assign95780_body0_e148359_d_n8;
            locals.var_dnm_dn9 = assign95780_body0_e148359_d_n9;
            locals.var_dnm_dn10 = assign95780_body0_e148359_d_n10;
            locals.var_dnm_dn13 = assign95780_body0_e148359_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign95780_body1_e148369,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        let assign95780_body1_e148367: f64 = (locals.var_m0 + 1.0);
        (assign95780_body1_e148367,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign95780_body1_e148369;
            locals.var_m0_rv = 0.0;
        }

        let (assign95790_e148389, assign95790_e148389_d_n0, assign95790_e148389_d_n2, assign95790_e148389_d_n4, assign95790_e148389_d_n5, assign95790_e148389_d_n6, assign95790_e148389_d_n7, assign95790_e148389_d_n8, assign95790_e148389_d_n9, assign95790_e148389_d_n10, assign95790_e148389_d_n13,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 == 0.0)) {
        let (assign95790_e148387, assign95790_e148387_d_n0, assign95790_e148387_d_n2, assign95790_e148387_d_n4, assign95790_e148387_d_n5, assign95790_e148387_d_n6, assign95790_e148387_d_n7, assign95790_e148387_d_n8, assign95790_e148387_d_n9, assign95790_e148387_d_n10, assign95790_e148387_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign95790_e148384: f64 = (2.0 * 2.0);
                let assign95790_e148385: f64 = (1.0 / assign95790_e148384);
                let assign95790_e148386: f64 = (locals.var_dnm).powf(assign95790_e148385);
                (assign95790_e148386, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn0)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn2)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn4)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn5)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn6)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn7)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn8)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn9)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn10)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn13)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign95790_e148387, assign95790_e148387_d_n0, assign95790_e148387_d_n2, assign95790_e148387_d_n4, assign95790_e148387_d_n5, assign95790_e148387_d_n6, assign95790_e148387_d_n7, assign95790_e148387_d_n8, assign95790_e148387_d_n9, assign95790_e148387_d_n10, assign95790_e148387_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95790_e148389;
        locals.var_dnm_dn0 = assign95790_e148389_d_n0;
        locals.var_dnm_dn2 = assign95790_e148389_d_n2;
        locals.var_dnm_dn4 = assign95790_e148389_d_n4;
        locals.var_dnm_dn5 = assign95790_e148389_d_n5;
        locals.var_dnm_dn6 = assign95790_e148389_d_n6;
        locals.var_dnm_dn7 = assign95790_e148389_d_n7;
        locals.var_dnm_dn8 = assign95790_e148389_d_n8;
        locals.var_dnm_dn9 = assign95790_e148389_d_n9;
        locals.var_dnm_dn10 = assign95790_e148389_d_n10;
        locals.var_dnm_dn13 = assign95790_e148389_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign95800_e148397, assign95800_e148397_d_n0, assign95800_e148397_d_n2, assign95800_e148397_d_n4, assign95800_e148397_d_n5, assign95800_e148397_d_n6, assign95800_e148397_d_n7, assign95800_e148397_d_n8, assign95800_e148397_d_n9, assign95800_e148397_d_n10, assign95800_e148397_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95800_e148395: f64 = (1.0 / locals.var_dnm);
        (assign95800_e148395, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95800_e148397;
        locals.var_dnm_dn0 = assign95800_e148397_d_n0;
        locals.var_dnm_dn2 = assign95800_e148397_d_n2;
        locals.var_dnm_dn4 = assign95800_e148397_d_n4;
        locals.var_dnm_dn5 = assign95800_e148397_d_n5;
        locals.var_dnm_dn6 = assign95800_e148397_d_n6;
        locals.var_dnm_dn7 = assign95800_e148397_d_n7;
        locals.var_dnm_dn8 = assign95800_e148397_d_n8;
        locals.var_dnm_dn9 = assign95800_e148397_d_n9;
        locals.var_dnm_dn10 = assign95800_e148397_d_n10;
        locals.var_dnm_dn13 = assign95800_e148397_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign95810_e148409, assign95810_e148409_d_n0, assign95810_e148409_d_n2, assign95810_e148409_d_n4, assign95810_e148409_d_n5, assign95810_e148409_d_n6, assign95810_e148409_d_n7, assign95810_e148409_d_n8, assign95810_e148409_d_n9, assign95810_e148409_d_n10, assign95810_e148409_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95810_e148404: f64 = (10.0 * 2.220446049250313e-16);
        let assign95810_e148405: f64 = (locals.var_tmf1 * assign95810_e148404);
        let assign95810_e148407: f64 = (assign95810_e148405 * locals.var_dnm);
        (assign95810_e148407, (((locals.var_tmf1_dn0 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign95810_e148409;
        locals.var_tmf0_dn0 = assign95810_e148409_d_n0;
        locals.var_tmf0_dn2 = assign95810_e148409_d_n2;
        locals.var_tmf0_dn4 = assign95810_e148409_d_n4;
        locals.var_tmf0_dn5 = assign95810_e148409_d_n5;
        locals.var_tmf0_dn6 = assign95810_e148409_d_n6;
        locals.var_tmf0_dn7 = assign95810_e148409_d_n7;
        locals.var_tmf0_dn8 = assign95810_e148409_d_n8;
        locals.var_tmf0_dn9 = assign95810_e148409_d_n9;
        locals.var_tmf0_dn10 = assign95810_e148409_d_n10;
        locals.var_tmf0_dn13 = assign95810_e148409_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign95820_e148423, assign95820_e148423_d_n0, assign95820_e148423_d_n2, assign95820_e148423_d_n4, assign95820_e148423_d_n5, assign95820_e148423_d_n6, assign95820_e148423_d_n7, assign95820_e148423_d_n8, assign95820_e148423_d_n9, assign95820_e148423_d_n10, assign95820_e148423_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95820_e148415: f64 = (10.0 * 2.220446049250313e-16);
        let assign95820_e148417: f64 = (assign95820_e148415 * locals.var_xmp);
        let assign95820_e148419: f64 = (assign95820_e148417 * locals.var_dnm);
        let assign95820_e148421: f64 = (assign95820_e148419 / locals.var_arg);
        (assign95820_e148421, ((((((assign95820_e148415 * locals.var_xmp_dn0) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn0)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn2) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn2)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn4) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn4)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn5) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn5)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn6) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn6)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn7) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn7)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn8) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn8)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn9) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn9)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn10) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn10)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn13) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn13)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95820_e148423;
        locals.var_t0_dn0 = assign95820_e148423_d_n0;
        locals.var_t0_dn2 = assign95820_e148423_d_n2;
        locals.var_t0_dn4 = assign95820_e148423_d_n4;
        locals.var_t0_dn5 = assign95820_e148423_d_n5;
        locals.var_t0_dn6 = assign95820_e148423_d_n6;
        locals.var_t0_dn7 = assign95820_e148423_d_n7;
        locals.var_t0_dn8 = assign95820_e148423_d_n8;
        locals.var_t0_dn9 = assign95820_e148423_d_n9;
        locals.var_t0_dn10 = assign95820_e148423_d_n10;
        locals.var_t0_dn13 = assign95820_e148423_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign95830_e148441, assign95830_e148441_d_n0, assign95830_e148441_d_n2, assign95830_e148441_d_n4, assign95830_e148441_d_n5, assign95830_e148441_d_n6, assign95830_e148441_d_n7, assign95830_e148441_d_n8, assign95830_e148441_d_n9, assign95830_e148441_d_n10, assign95830_e148441_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95830_e148429: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95830_e148432: f64 = (10.0 * 2.220446049250313e-16);
        let assign95830_e148433: f64 = (assign95830_e148429 - assign95830_e148432);
        let assign95830_e148436: f64 = (10.0 * 2.220446049250313e-16);
        let assign95830_e148437: f64 = (assign95830_e148433 - assign95830_e148436);
        let assign95830_e148439: f64 = (assign95830_e148437 + locals.var_tmf0);
        (assign95830_e148439, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn13 + locals.var_vds_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign95830_e148441;
        locals.var_psdl_dn0 = assign95830_e148441_d_n0;
        locals.var_psdl_dn2 = assign95830_e148441_d_n2;
        locals.var_psdl_dn4 = assign95830_e148441_d_n4;
        locals.var_psdl_dn5 = assign95830_e148441_d_n5;
        locals.var_psdl_dn6 = assign95830_e148441_d_n6;
        locals.var_psdl_dn7 = assign95830_e148441_d_n7;
        locals.var_psdl_dn8 = assign95830_e148441_d_n8;
        locals.var_psdl_dn9 = assign95830_e148441_d_n9;
        locals.var_psdl_dn10 = assign95830_e148441_d_n10;
        locals.var_psdl_dn13 = assign95830_e148441_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign95840_e148447, assign95840_e148447_d_n0, assign95840_e148447_d_n2, assign95840_e148447_d_n4, assign95840_e148447_d_n5, assign95840_e148447_d_n6, assign95840_e148447_d_n7, assign95840_e148447_d_n8, assign95840_e148447_d_n9, assign95840_e148447_d_n10, assign95840_e148447_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95840_e148447;
        locals.var_t0_dn0 = assign95840_e148447_d_n0;
        locals.var_t0_dn2 = assign95840_e148447_d_n2;
        locals.var_t0_dn4 = assign95840_e148447_d_n4;
        locals.var_t0_dn5 = assign95840_e148447_d_n5;
        locals.var_t0_dn6 = assign95840_e148447_d_n6;
        locals.var_t0_dn7 = assign95840_e148447_d_n7;
        locals.var_t0_dn8 = assign95840_e148447_d_n8;
        locals.var_t0_dn9 = assign95840_e148447_d_n9;
        locals.var_t0_dn10 = assign95840_e148447_d_n10;
        locals.var_t0_dn13 = assign95840_e148447_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign95850_e148454, assign95850_e148454_d_n0, assign95850_e148454_d_n2, assign95850_e148454_d_n4, assign95850_e148454_d_n5, assign95850_e148454_d_n6, assign95850_e148454_d_n7, assign95850_e148454_d_n8, assign95850_e148454_d_n9, assign95850_e148454_d_n10, assign95850_e148454_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign95850_e148454;
        locals.var_psdl_dn0 = assign95850_e148454_d_n0;
        locals.var_psdl_dn2 = assign95850_e148454_d_n2;
        locals.var_psdl_dn4 = assign95850_e148454_d_n4;
        locals.var_psdl_dn5 = assign95850_e148454_d_n5;
        locals.var_psdl_dn6 = assign95850_e148454_d_n6;
        locals.var_psdl_dn7 = assign95850_e148454_d_n7;
        locals.var_psdl_dn8 = assign95850_e148454_d_n8;
        locals.var_psdl_dn9 = assign95850_e148454_d_n9;
        locals.var_psdl_dn10 = assign95850_e148454_d_n10;
        locals.var_psdl_dn13 = assign95850_e148454_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign95860_e148461, assign95860_e148461_d_n0, assign95860_e148461_d_n2, assign95860_e148461_d_n4, assign95860_e148461_d_n5, assign95860_e148461_d_n6, assign95860_e148461_d_n7, assign95860_e148461_d_n8, assign95860_e148461_d_n9, assign95860_e148461_d_n10, assign95860_e148461_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95860_e148461;
        locals.var_t0_dn0 = assign95860_e148461_d_n0;
        locals.var_t0_dn2 = assign95860_e148461_d_n2;
        locals.var_t0_dn4 = assign95860_e148461_d_n4;
        locals.var_t0_dn5 = assign95860_e148461_d_n5;
        locals.var_t0_dn6 = assign95860_e148461_d_n6;
        locals.var_t0_dn7 = assign95860_e148461_d_n7;
        locals.var_t0_dn8 = assign95860_e148461_d_n8;
        locals.var_t0_dn9 = assign95860_e148461_d_n9;
        locals.var_t0_dn10 = assign95860_e148461_d_n10;
        locals.var_t0_dn13 = assign95860_e148461_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign95870_e148467, assign95870_e148467_d_n0, assign95870_e148467_d_n2, assign95870_e148467_d_n4, assign95870_e148467_d_n5, assign95870_e148467_d_n6, assign95870_e148467_d_n7, assign95870_e148467_d_n8, assign95870_e148467_d_n9, assign95870_e148467_d_n10, assign95870_e148467_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_flg_qy != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95870_e148467;
        locals.var_ec_dn0 = assign95870_e148467_d_n0;
        locals.var_ec_dn2 = assign95870_e148467_d_n2;
        locals.var_ec_dn4 = assign95870_e148467_d_n4;
        locals.var_ec_dn5 = assign95870_e148467_d_n5;
        locals.var_ec_dn6 = assign95870_e148467_d_n6;
        locals.var_ec_dn7 = assign95870_e148467_d_n7;
        locals.var_ec_dn8 = assign95870_e148467_d_n8;
        locals.var_ec_dn9 = assign95870_e148467_d_n9;
        locals.var_ec_dn10 = assign95870_e148467_d_n10;
        locals.var_ec_dn13 = assign95870_e148467_d_n13;
        locals.var_ec_rv = 0.0;

        let assign95880_e148474: f64 = if ((locals.var_idd < 1e-15) || (locals.var_vdseff < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard2229 = assign95880_e148474;
        locals.var_guard2229_rv = 0.0;

        let (assign95890_e148483, assign95890_e148483_d_n0, assign95890_e148483_d_n2, assign95890_e148483_d_n4, assign95890_e148483_d_n5, assign95890_e148483_d_n6, assign95890_e148483_d_n7, assign95890_e148483_d_n8, assign95890_e148483_d_n9, assign95890_e148483_d_n10, assign95890_e148483_d_n13,) = {
    if (((locals.var_guard2222 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2229 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95890_e148483;
        locals.var_ec_dn0 = assign95890_e148483_d_n0;
        locals.var_ec_dn2 = assign95890_e148483_d_n2;
        locals.var_ec_dn4 = assign95890_e148483_d_n4;
        locals.var_ec_dn5 = assign95890_e148483_d_n5;
        locals.var_ec_dn6 = assign95890_e148483_d_n6;
        locals.var_ec_dn7 = assign95890_e148483_d_n7;
        locals.var_ec_dn8 = assign95890_e148483_d_n8;
        locals.var_ec_dn9 = assign95890_e148483_d_n9;
        locals.var_ec_dn10 = assign95890_e148483_d_n10;
        locals.var_ec_dn13 = assign95890_e148483_d_n13;
        locals.var_ec_rv = 0.0;

        let (assign95900_e148499, assign95900_e148499_d_n0, assign95900_e148499_d_n2, assign95900_e148499_d_n4, assign95900_e148499_d_n5, assign95900_e148499_d_n6, assign95900_e148499_d_n7, assign95900_e148499_d_n8, assign95900_e148499_d_n9, assign95900_e148499_d_n10, assign95900_e148499_d_n13,) = {
    if (((locals.var_guard2222 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign95900_e148493: f64 = (locals.var_idd / locals.var_qn0);
        let assign95900_e148495: f64 = (assign95900_e148493 * locals.var_beta_inv);
        let assign95900_e148497: f64 = (assign95900_e148495 / locals.var_leff);
        (assign95900_e148497, ((((((locals.var_idd_dn0 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn0)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn0)) / locals.var_leff), ((((((locals.var_idd_dn2 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn2)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn2)) / locals.var_leff), ((((((locals.var_idd_dn4 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn4)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn4)) / locals.var_leff), ((((((locals.var_idd_dn5 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn5)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn5)) / locals.var_leff), ((((((locals.var_idd_dn6 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn6)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn6)) / locals.var_leff), ((((((locals.var_idd_dn7 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn7)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn7)) / locals.var_leff), ((((((locals.var_idd_dn8 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn8)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn8)) / locals.var_leff), ((((((locals.var_idd_dn9 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn9)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn9)) / locals.var_leff), ((((((locals.var_idd_dn10 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn10)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn10)) / locals.var_leff), ((((((locals.var_idd_dn13 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn13)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn13)) / locals.var_leff),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95900_e148499;
        locals.var_ec_dn0 = assign95900_e148499_d_n0;
        locals.var_ec_dn2 = assign95900_e148499_d_n2;
        locals.var_ec_dn4 = assign95900_e148499_d_n4;
        locals.var_ec_dn5 = assign95900_e148499_d_n5;
        locals.var_ec_dn6 = assign95900_e148499_d_n6;
        locals.var_ec_dn7 = assign95900_e148499_d_n7;
        locals.var_ec_dn8 = assign95900_e148499_d_n8;
        locals.var_ec_dn9 = assign95900_e148499_d_n9;
        locals.var_ec_dn10 = assign95900_e148499_d_n10;
        locals.var_ec_dn13 = assign95900_e148499_d_n13;
        locals.var_ec_rv = 0.0;

        let assign95910_e148502: f64 = if locals.var_flg_qy == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign95910_e148502;
        locals.var_guard2230_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_360(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95920_e148506, assign95920_e148506_d_n0, assign95920_e148506_d_n2, assign95920_e148506_d_n4, assign95920_e148506_d_n5, assign95920_e148506_d_n6, assign95920_e148506_d_n7, assign95920_e148506_d_n8, assign95920_e148506_d_n9, assign95920_e148506_d_n10, assign95920_e148506_d_n13,) = {
    if (locals.var_guard2230 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95920_e148506;
        locals.var_qy_dn0 = assign95920_e148506_d_n0;
        locals.var_qy_dn2 = assign95920_e148506_d_n2;
        locals.var_qy_dn4 = assign95920_e148506_d_n4;
        locals.var_qy_dn5 = assign95920_e148506_d_n5;
        locals.var_qy_dn6 = assign95920_e148506_d_n6;
        locals.var_qy_dn7 = assign95920_e148506_d_n7;
        locals.var_qy_dn8 = assign95920_e148506_d_n8;
        locals.var_qy_dn9 = assign95920_e148506_d_n9;
        locals.var_qy_dn10 = assign95920_e148506_d_n10;
        locals.var_qy_dn13 = assign95920_e148506_d_n13;
        locals.var_qy_rv = 0.0;

        let (assign95930_e148517, assign95930_e148517_d_n0, assign95930_e148517_d_n2, assign95930_e148517_d_n4, assign95930_e148517_d_n5, assign95930_e148517_d_n6, assign95930_e148517_d_n7, assign95930_e148517_d_n8, assign95930_e148517_d_n9, assign95930_e148517_d_n10, assign95930_e148517_d_n13,) = {
    if (locals.var_guard2230 == 0.0) {
        let assign95930_e148511: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign95930_e148513: f64 = (assign95930_e148511 * locals.var_wdpl);
        let assign95930_e148515: f64 = (assign95930_e148513 * 1.3);
        (assign95930_e148515, ((assign95930_e148511 * locals.var_wdpl_dn0) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn2) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn4) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn5) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn6) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn7) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn8) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn9) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn10) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn13) * 1.3),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign95930_e148517;
        locals.var_t2_dn0 = assign95930_e148517_d_n0;
        locals.var_t2_dn2 = assign95930_e148517_d_n2;
        locals.var_t2_dn4 = assign95930_e148517_d_n4;
        locals.var_t2_dn5 = assign95930_e148517_d_n5;
        locals.var_t2_dn6 = assign95930_e148517_d_n6;
        locals.var_t2_dn7 = assign95930_e148517_d_n7;
        locals.var_t2_dn8 = assign95930_e148517_d_n8;
        locals.var_t2_dn9 = assign95930_e148517_d_n9;
        locals.var_t2_dn10 = assign95930_e148517_d_n10;
        locals.var_t2_dn13 = assign95930_e148517_d_n13;
        locals.var_t2_rv = 0.0;

        let assign95940_e148520: f64 = if p.p133 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign95940_e148520;
        locals.var_guard2231_rv = 0.0;

        let (assign95950_e148531, assign95950_e148531_d_n0, assign95950_e148531_d_n2, assign95950_e148531_d_n4, assign95950_e148531_d_n5, assign95950_e148531_d_n6, assign95950_e148531_d_n7, assign95950_e148531_d_n8, assign95950_e148531_d_n9, assign95950_e148531_d_n10, assign95950_e148531_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95950_e148527: f64 = (locals.var_ec * locals.var_leff);
        let assign95950_e148529: f64 = (assign95950_e148527 + locals.var_ps0);
        (assign95950_e148529, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn4 * locals.var_leff) + locals.var_ps0_dn4), ((locals.var_ec_dn5 * locals.var_leff) + locals.var_ps0_dn5), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn8 * locals.var_leff) + locals.var_ps0_dn8), ((locals.var_ec_dn9 * locals.var_leff) + locals.var_ps0_dn9), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn13 * locals.var_leff) + locals.var_ps0_dn13),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn8, locals.var_pslk_dn9, locals.var_pslk_dn10, locals.var_pslk_dn13,)
    }
};
        locals.var_pslk = assign95950_e148531;
        locals.var_pslk_dn0 = assign95950_e148531_d_n0;
        locals.var_pslk_dn2 = assign95950_e148531_d_n2;
        locals.var_pslk_dn4 = assign95950_e148531_d_n4;
        locals.var_pslk_dn5 = assign95950_e148531_d_n5;
        locals.var_pslk_dn6 = assign95950_e148531_d_n6;
        locals.var_pslk_dn7 = assign95950_e148531_d_n7;
        locals.var_pslk_dn8 = assign95950_e148531_d_n8;
        locals.var_pslk_dn9 = assign95950_e148531_d_n9;
        locals.var_pslk_dn10 = assign95950_e148531_d_n10;
        locals.var_pslk_dn13 = assign95950_e148531_d_n13;
        locals.var_pslk_rv = 0.0;

        let (assign95960_e148548, assign95960_e148548_d_n0, assign95960_e148548_d_n2, assign95960_e148548_d_n4, assign95960_e148548_d_n5, assign95960_e148548_d_n6, assign95960_e148548_d_n7, assign95960_e148548_d_n8, assign95960_e148548_d_n9, assign95960_e148548_d_n10, assign95960_e148548_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95960_e148539: f64 = (locals.var_vdsz__blk439 + locals.var_ps0);
        let assign95960_e148540: f64 = (locals.var_aclm * assign95960_e148539);
        let assign95960_e148543: f64 = (1.0 - locals.var_aclm);
        let assign95960_e148545: f64 = (assign95960_e148543 * locals.var_pslk);
        let assign95960_e148546: f64 = (assign95960_e148540 + assign95960_e148545);
        (assign95960_e148546, ((locals.var_aclm * (locals.var_vdsz__blk439_dn0 + locals.var_ps0_dn0)) + (assign95960_e148543 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn2 + locals.var_ps0_dn2)) + (assign95960_e148543 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn4 + locals.var_ps0_dn4)) + (assign95960_e148543 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn5 + locals.var_ps0_dn5)) + (assign95960_e148543 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn6 + locals.var_ps0_dn6)) + (assign95960_e148543 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn7 + locals.var_ps0_dn7)) + (assign95960_e148543 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn8 + locals.var_ps0_dn8)) + (assign95960_e148543 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn9 + locals.var_ps0_dn9)) + (assign95960_e148543 * locals.var_pslk_dn9)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn10 + locals.var_ps0_dn10)) + (assign95960_e148543 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn13 + locals.var_ps0_dn13)) + (assign95960_e148543 * locals.var_pslk_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign95960_e148548;
        locals.var_t1_dn0 = assign95960_e148548_d_n0;
        locals.var_t1_dn2 = assign95960_e148548_d_n2;
        locals.var_t1_dn4 = assign95960_e148548_d_n4;
        locals.var_t1_dn5 = assign95960_e148548_d_n5;
        locals.var_t1_dn6 = assign95960_e148548_d_n6;
        locals.var_t1_dn7 = assign95960_e148548_d_n7;
        locals.var_t1_dn8 = assign95960_e148548_d_n8;
        locals.var_t1_dn9 = assign95960_e148548_d_n9;
        locals.var_t1_dn10 = assign95960_e148548_d_n10;
        locals.var_t1_dn13 = assign95960_e148548_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign95970_e148564, assign95970_e148564_d_n0, assign95970_e148564_d_n2, assign95970_e148564_d_n4, assign95970_e148564_d_n5, assign95970_e148564_d_n6, assign95970_e148564_d_n7, assign95970_e148564_d_n8, assign95970_e148564_d_n9, assign95970_e148564_d_n10, assign95970_e148564_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95970_e148555: f64 = (locals.var_ps0 + locals.var_vdsz__blk439);
        let assign95970_e148557: f64 = (assign95970_e148555 - locals.var_t1);
        let assign95970_e148559: f64 = (assign95970_e148557 / p.p133);
        let assign95970_e148560: f64 = (-assign95970_e148559);
        let assign95970_e148562: f64 = (assign95970_e148560 * locals.var_t2);
        (assign95970_e148562, (((-(((locals.var_ps0_dn0 + locals.var_vdsz__blk439_dn0) - locals.var_t1_dn0) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn0)), (((-(((locals.var_ps0_dn2 + locals.var_vdsz__blk439_dn2) - locals.var_t1_dn2) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn2)), (((-(((locals.var_ps0_dn4 + locals.var_vdsz__blk439_dn4) - locals.var_t1_dn4) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn4)), (((-(((locals.var_ps0_dn5 + locals.var_vdsz__blk439_dn5) - locals.var_t1_dn5) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn5)), (((-(((locals.var_ps0_dn6 + locals.var_vdsz__blk439_dn6) - locals.var_t1_dn6) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn6)), (((-(((locals.var_ps0_dn7 + locals.var_vdsz__blk439_dn7) - locals.var_t1_dn7) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn7)), (((-(((locals.var_ps0_dn8 + locals.var_vdsz__blk439_dn8) - locals.var_t1_dn8) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn8)), (((-(((locals.var_ps0_dn9 + locals.var_vdsz__blk439_dn9) - locals.var_t1_dn9) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn9)), (((-(((locals.var_ps0_dn10 + locals.var_vdsz__blk439_dn10) - locals.var_t1_dn10) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn10)), (((-(((locals.var_ps0_dn13 + locals.var_vdsz__blk439_dn13) - locals.var_t1_dn13) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn13)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95970_e148564;
        locals.var_qy_dn0 = assign95970_e148564_d_n0;
        locals.var_qy_dn2 = assign95970_e148564_d_n2;
        locals.var_qy_dn4 = assign95970_e148564_d_n4;
        locals.var_qy_dn5 = assign95970_e148564_d_n5;
        locals.var_qy_dn6 = assign95970_e148564_d_n6;
        locals.var_qy_dn7 = assign95970_e148564_d_n7;
        locals.var_qy_dn8 = assign95970_e148564_d_n8;
        locals.var_qy_dn9 = assign95970_e148564_d_n9;
        locals.var_qy_dn10 = assign95970_e148564_d_n10;
        locals.var_qy_dn13 = assign95970_e148564_d_n13;
        locals.var_qy_rv = 0.0;

        let assign95980_e148567: f64 = if p.p134 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign95980_e148567;
        locals.var_guard2232_rv = 0.0;

        let (assign95990_e148578, assign95990_e148578_d_n0, assign95990_e148578_d_n2, assign95990_e148578_d_n4, assign95990_e148578_d_n5, assign95990_e148578_d_n6, assign95990_e148578_d_n7, assign95990_e148578_d_n8, assign95990_e148578_d_n9, assign95990_e148578_d_n10, assign95990_e148578_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2232 != 0.0)) {
        let assign95990_e148575: f64 = (locals.var_cqyb0 * locals.var_vbs);
        let assign95990_e148576: f64 = (locals.var_qy + assign95990_e148575);
        (assign95990_e148576, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, (locals.var_qy_dn5 + (locals.var_cqyb0 * locals.var_vbs_dn5)), locals.var_qy_dn6, (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbs_dn7)), (locals.var_qy_dn8 + (locals.var_cqyb0 * locals.var_vbs_dn8)), locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95990_e148578;
        locals.var_qy_dn0 = assign95990_e148578_d_n0;
        locals.var_qy_dn2 = assign95990_e148578_d_n2;
        locals.var_qy_dn4 = assign95990_e148578_d_n4;
        locals.var_qy_dn5 = assign95990_e148578_d_n5;
        locals.var_qy_dn6 = assign95990_e148578_d_n6;
        locals.var_qy_dn7 = assign95990_e148578_d_n7;
        locals.var_qy_dn8 = assign95990_e148578_d_n8;
        locals.var_qy_dn9 = assign95990_e148578_d_n9;
        locals.var_qy_dn10 = assign95990_e148578_d_n10;
        locals.var_qy_dn13 = assign95990_e148578_d_n13;
        locals.var_qy_rv = 0.0;

        locals.var_cfd = locals.var_cfrng;
        locals.var_cfd_rv = 0.0;

        locals.var_cfs = locals.var_cfrng;
        locals.var_cfs_rv = 0.0;

        let assign96020_e148584: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign96020_e148585: f64 = (locals.var_cfd * assign96020_e148584);
        locals.var_qfd = assign96020_e148585;
        locals.var_qfd_dn0 = (locals.var_cfd * (-locals.var_vdsei_dn0));
        locals.var_qfd_dn2 = (locals.var_cfd * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qfd_dn6 = (locals.var_cfd * locals.var_vgsei_dn6);
        locals.var_qfd_rv = 0.0;

        let assign96030_e148588: f64 = (locals.var_cfs * locals.var_vgsei);
        locals.var_qfs = assign96030_e148588;
        locals.var_qfs_dn2 = (locals.var_cfs * locals.var_vgsei_dn2);
        locals.var_qfs_dn6 = (locals.var_cfs * locals.var_vgsei_dn6);
        locals.var_qfs_rv = 0.0;

        let assign96040_e148595: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign96040_e148595;
        locals.var_guard2233_rv = 0.0;

        let (assign96050_e148601, assign96050_e148601_d_n0, assign96050_e148601_d_n2, assign96050_e148601_d_n4, assign96050_e148601_d_n5, assign96050_e148601_d_n6, assign96050_e148601_d_n7, assign96050_e148601_d_n8, assign96050_e148601_d_n9, assign96050_e148601_d_n10, assign96050_e148601_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96050_e148599: f64 = (locals.var_tratio * locals.var_tratio);
        (assign96050_e148599, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn13 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign96050_e148601;
        locals.var_t0_dn0 = assign96050_e148601_d_n0;
        locals.var_t0_dn2 = assign96050_e148601_d_n2;
        locals.var_t0_dn4 = assign96050_e148601_d_n4;
        locals.var_t0_dn5 = assign96050_e148601_d_n5;
        locals.var_t0_dn6 = assign96050_e148601_d_n6;
        locals.var_t0_dn7 = assign96050_e148601_d_n7;
        locals.var_t0_dn8 = assign96050_e148601_d_n8;
        locals.var_t0_dn9 = assign96050_e148601_d_n9;
        locals.var_t0_dn10 = assign96050_e148601_d_n10;
        locals.var_t0_dn13 = assign96050_e148601_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign96060_e148620, assign96060_e148620_d_n0, assign96060_e148620_d_n2, assign96060_e148620_d_n4, assign96060_e148620_d_n5, assign96060_e148620_d_n6, assign96060_e148620_d_n7, assign96060_e148620_d_n8, assign96060_e148620_d_n9, assign96060_e148620_d_n10, assign96060_e148620_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96060_e148606: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96060_e148609: f64 = (locals.var_eg * locals.var_beta);
        let assign96060_e148610: f64 = (assign96060_e148606 - assign96060_e148609);
        let assign96060_e148613: f64 = (p.p499 * locals.var_log_tratio);
        let assign96060_e148614: f64 = (assign96060_e148610 + assign96060_e148613);
        let assign96060_e148616: f64 = (assign96060_e148614 / locals.var_uc_njd);
        let assign96060_e148617: f64 = (assign96060_e148616).exp();
        let assign96060_e148618: f64 = (locals.var_uc_js0d * assign96060_e148617);
        (assign96060_e148618, (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign96060_e148620;
        locals.var_js_dn0 = assign96060_e148620_d_n0;
        locals.var_js_dn2 = assign96060_e148620_d_n2;
        locals.var_js_dn4 = assign96060_e148620_d_n4;
        locals.var_js_dn5 = assign96060_e148620_d_n5;
        locals.var_js_dn6 = assign96060_e148620_d_n6;
        locals.var_js_dn7 = assign96060_e148620_d_n7;
        locals.var_js_dn8 = assign96060_e148620_d_n8;
        locals.var_js_dn9 = assign96060_e148620_d_n9;
        locals.var_js_dn10 = assign96060_e148620_d_n10;
        locals.var_js_dn13 = assign96060_e148620_d_n13;
        locals.var_js_rv = 0.0;

        let (assign96070_e148639, assign96070_e148639_d_n0, assign96070_e148639_d_n2, assign96070_e148639_d_n4, assign96070_e148639_d_n5, assign96070_e148639_d_n6, assign96070_e148639_d_n7, assign96070_e148639_d_n8, assign96070_e148639_d_n9, assign96070_e148639_d_n10, assign96070_e148639_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96070_e148625: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96070_e148628: f64 = (locals.var_eg * locals.var_beta);
        let assign96070_e148629: f64 = (assign96070_e148625 - assign96070_e148628);
        let assign96070_e148632: f64 = (p.p499 * locals.var_log_tratio);
        let assign96070_e148633: f64 = (assign96070_e148629 + assign96070_e148632);
        let assign96070_e148635: f64 = (assign96070_e148633 / p.p497);
        let assign96070_e148636: f64 = (assign96070_e148635).exp();
        let assign96070_e148637: f64 = (locals.var_uc_js0swd * assign96070_e148636);
        (assign96070_e148637, (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign96070_e148639;
        locals.var_jssw_dn0 = assign96070_e148639_d_n0;
        locals.var_jssw_dn2 = assign96070_e148639_d_n2;
        locals.var_jssw_dn4 = assign96070_e148639_d_n4;
        locals.var_jssw_dn5 = assign96070_e148639_d_n5;
        locals.var_jssw_dn6 = assign96070_e148639_d_n6;
        locals.var_jssw_dn7 = assign96070_e148639_d_n7;
        locals.var_jssw_dn8 = assign96070_e148639_d_n8;
        locals.var_jssw_dn9 = assign96070_e148639_d_n9;
        locals.var_jssw_dn10 = assign96070_e148639_d_n10;
        locals.var_jssw_dn13 = assign96070_e148639_d_n13;
        locals.var_jssw_rv = 0.0;

        let (assign96080_e148658, assign96080_e148658_d_n0, assign96080_e148658_d_n2, assign96080_e148658_d_n4, assign96080_e148658_d_n5, assign96080_e148658_d_n6, assign96080_e148658_d_n7, assign96080_e148658_d_n8, assign96080_e148658_d_n9, assign96080_e148658_d_n10, assign96080_e148658_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96080_e148644: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96080_e148647: f64 = (locals.var_eg * locals.var_beta);
        let assign96080_e148648: f64 = (assign96080_e148644 - assign96080_e148647);
        let assign96080_e148651: f64 = (p.p499 * locals.var_log_tratio);
        let assign96080_e148652: f64 = (assign96080_e148648 + assign96080_e148651);
        let assign96080_e148654: f64 = (assign96080_e148652 / p.p498);
        let assign96080_e148655: f64 = (assign96080_e148654).exp();
        let assign96080_e148656: f64 = (p.p495 * assign96080_e148655);
        (assign96080_e148656, (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign96080_e148658;
        locals.var_jsswg_dn0 = assign96080_e148658_d_n0;
        locals.var_jsswg_dn2 = assign96080_e148658_d_n2;
        locals.var_jsswg_dn4 = assign96080_e148658_d_n4;
        locals.var_jsswg_dn5 = assign96080_e148658_d_n5;
        locals.var_jsswg_dn6 = assign96080_e148658_d_n6;
        locals.var_jsswg_dn7 = assign96080_e148658_d_n7;
        locals.var_jsswg_dn8 = assign96080_e148658_d_n8;
        locals.var_jsswg_dn9 = assign96080_e148658_d_n9;
        locals.var_jsswg_dn10 = assign96080_e148658_d_n10;
        locals.var_jsswg_dn13 = assign96080_e148658_d_n13;
        locals.var_jsswg_rv = 0.0;

        let (assign96090_e148677, assign96090_e148677_d_n0, assign96090_e148677_d_n2, assign96090_e148677_d_n4, assign96090_e148677_d_n5, assign96090_e148677_d_n6, assign96090_e148677_d_n7, assign96090_e148677_d_n8, assign96090_e148677_d_n9, assign96090_e148677_d_n10, assign96090_e148677_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96090_e148663: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96090_e148666: f64 = (locals.var_eg * locals.var_beta);
        let assign96090_e148667: f64 = (assign96090_e148663 - assign96090_e148666);
        let assign96090_e148670: f64 = (p.p509 * locals.var_log_tratio);
        let assign96090_e148671: f64 = (assign96090_e148667 + assign96090_e148670);
        let assign96090_e148673: f64 = (assign96090_e148671 / locals.var_uc_njd);
        let assign96090_e148674: f64 = (assign96090_e148673).exp();
        let assign96090_e148675: f64 = (locals.var_uc_js0d * assign96090_e148674);
        (assign96090_e148675, (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign96090_e148677;
        locals.var_js2_dn0 = assign96090_e148677_d_n0;
        locals.var_js2_dn2 = assign96090_e148677_d_n2;
        locals.var_js2_dn4 = assign96090_e148677_d_n4;
        locals.var_js2_dn5 = assign96090_e148677_d_n5;
        locals.var_js2_dn6 = assign96090_e148677_d_n6;
        locals.var_js2_dn7 = assign96090_e148677_d_n7;
        locals.var_js2_dn8 = assign96090_e148677_d_n8;
        locals.var_js2_dn9 = assign96090_e148677_d_n9;
        locals.var_js2_dn10 = assign96090_e148677_d_n10;
        locals.var_js2_dn13 = assign96090_e148677_d_n13;
        locals.var_js2_rv = 0.0;

        let (assign96100_e148696, assign96100_e148696_d_n0, assign96100_e148696_d_n2, assign96100_e148696_d_n4, assign96100_e148696_d_n5, assign96100_e148696_d_n6, assign96100_e148696_d_n7, assign96100_e148696_d_n8, assign96100_e148696_d_n9, assign96100_e148696_d_n10, assign96100_e148696_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96100_e148682: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96100_e148685: f64 = (locals.var_eg * locals.var_beta);
        let assign96100_e148686: f64 = (assign96100_e148682 - assign96100_e148685);
        let assign96100_e148689: f64 = (p.p509 * locals.var_log_tratio);
        let assign96100_e148690: f64 = (assign96100_e148686 + assign96100_e148689);
        let assign96100_e148692: f64 = (assign96100_e148690 / p.p497);
        let assign96100_e148693: f64 = (assign96100_e148692).exp();
        let assign96100_e148694: f64 = (locals.var_uc_js0swd * assign96100_e148693);
        (assign96100_e148694, (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign96100_e148696;
        locals.var_jssw2_dn0 = assign96100_e148696_d_n0;
        locals.var_jssw2_dn2 = assign96100_e148696_d_n2;
        locals.var_jssw2_dn4 = assign96100_e148696_d_n4;
        locals.var_jssw2_dn5 = assign96100_e148696_d_n5;
        locals.var_jssw2_dn6 = assign96100_e148696_d_n6;
        locals.var_jssw2_dn7 = assign96100_e148696_d_n7;
        locals.var_jssw2_dn8 = assign96100_e148696_d_n8;
        locals.var_jssw2_dn9 = assign96100_e148696_d_n9;
        locals.var_jssw2_dn10 = assign96100_e148696_d_n10;
        locals.var_jssw2_dn13 = assign96100_e148696_d_n13;
        locals.var_jssw2_rv = 0.0;

        let (assign96110_e148715, assign96110_e148715_d_n0, assign96110_e148715_d_n2, assign96110_e148715_d_n4, assign96110_e148715_d_n5, assign96110_e148715_d_n6, assign96110_e148715_d_n7, assign96110_e148715_d_n8, assign96110_e148715_d_n9, assign96110_e148715_d_n10, assign96110_e148715_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96110_e148701: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96110_e148704: f64 = (locals.var_eg * locals.var_beta);
        let assign96110_e148705: f64 = (assign96110_e148701 - assign96110_e148704);
        let assign96110_e148708: f64 = (p.p509 * locals.var_log_tratio);
        let assign96110_e148709: f64 = (assign96110_e148705 + assign96110_e148708);
        let assign96110_e148711: f64 = (assign96110_e148709 / p.p498);
        let assign96110_e148712: f64 = (assign96110_e148711).exp();
        let assign96110_e148713: f64 = (p.p495 * assign96110_e148712);
        (assign96110_e148713, (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign96110_e148715;
        locals.var_jsswg2_dn0 = assign96110_e148715_d_n0;
        locals.var_jsswg2_dn2 = assign96110_e148715_d_n2;
        locals.var_jsswg2_dn4 = assign96110_e148715_d_n4;
        locals.var_jsswg2_dn5 = assign96110_e148715_d_n5;
        locals.var_jsswg2_dn6 = assign96110_e148715_d_n6;
        locals.var_jsswg2_dn7 = assign96110_e148715_d_n7;
        locals.var_jsswg2_dn8 = assign96110_e148715_d_n8;
        locals.var_jsswg2_dn9 = assign96110_e148715_d_n9;
        locals.var_jsswg2_dn10 = assign96110_e148715_d_n10;
        locals.var_jsswg2_dn13 = assign96110_e148715_d_n13;
        locals.var_jsswg2_rv = 0.0;

        let assign96120_e148718: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign96120_e148718;
        locals.var_guard2234_rv = 0.0;

        let assign96130_e148721: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign96130_e148721;
        locals.var_guard2235_rv = 0.0;

        let (assign96140_e148731, assign96140_e148731_d_n0, assign96140_e148731_d_n2, assign96140_e148731_d_n4, assign96140_e148731_d_n5, assign96140_e148731_d_n6, assign96140_e148731_d_n7, assign96140_e148731_d_n8, assign96140_e148731_d_n9, assign96140_e148731_d_n10, assign96140_e148731_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96140_e148729: f64 = (p.p13 * locals.var_js);
        (assign96140_e148729, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign96140_e148731;
        locals.var_isbd_btm_dn0 = assign96140_e148731_d_n0;
        locals.var_isbd_btm_dn2 = assign96140_e148731_d_n2;
        locals.var_isbd_btm_dn4 = assign96140_e148731_d_n4;
        locals.var_isbd_btm_dn5 = assign96140_e148731_d_n5;
        locals.var_isbd_btm_dn6 = assign96140_e148731_d_n6;
        locals.var_isbd_btm_dn7 = assign96140_e148731_d_n7;
        locals.var_isbd_btm_dn8 = assign96140_e148731_d_n8;
        locals.var_isbd_btm_dn9 = assign96140_e148731_d_n9;
        locals.var_isbd_btm_dn10 = assign96140_e148731_d_n10;
        locals.var_isbd_btm_dn13 = assign96140_e148731_d_n13;
        locals.var_isbd_btm_rv = 0.0;

        let (assign96150_e148741, assign96150_e148741_d_n0, assign96150_e148741_d_n2, assign96150_e148741_d_n4, assign96150_e148741_d_n5, assign96150_e148741_d_n6, assign96150_e148741_d_n7, assign96150_e148741_d_n8, assign96150_e148741_d_n9, assign96150_e148741_d_n10, assign96150_e148741_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96150_e148739: f64 = (p.p13 * locals.var_js2);
        (assign96150_e148739, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign96150_e148741;
        locals.var_isbd2_btm_dn0 = assign96150_e148741_d_n0;
        locals.var_isbd2_btm_dn2 = assign96150_e148741_d_n2;
        locals.var_isbd2_btm_dn4 = assign96150_e148741_d_n4;
        locals.var_isbd2_btm_dn5 = assign96150_e148741_d_n5;
        locals.var_isbd2_btm_dn6 = assign96150_e148741_d_n6;
        locals.var_isbd2_btm_dn7 = assign96150_e148741_d_n7;
        locals.var_isbd2_btm_dn8 = assign96150_e148741_d_n8;
        locals.var_isbd2_btm_dn9 = assign96150_e148741_d_n9;
        locals.var_isbd2_btm_dn10 = assign96150_e148741_d_n10;
        locals.var_isbd2_btm_dn13 = assign96150_e148741_d_n13;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96160_e148753, assign96160_e148753_d_n0, assign96160_e148753_d_n2, assign96160_e148753_d_n4, assign96160_e148753_d_n5, assign96160_e148753_d_n6, assign96160_e148753_d_n7, assign96160_e148753_d_n8, assign96160_e148753_d_n9, assign96160_e148753_d_n10, assign96160_e148753_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96160_e148749: f64 = (p.p15 - locals.var_weff_nf);
        let assign96160_e148751: f64 = (assign96160_e148749 * locals.var_jssw);
        (assign96160_e148751, (assign96160_e148749 * locals.var_jssw_dn0), (assign96160_e148749 * locals.var_jssw_dn2), (assign96160_e148749 * locals.var_jssw_dn4), (assign96160_e148749 * locals.var_jssw_dn5), (assign96160_e148749 * locals.var_jssw_dn6), (assign96160_e148749 * locals.var_jssw_dn7), (assign96160_e148749 * locals.var_jssw_dn8), (assign96160_e148749 * locals.var_jssw_dn9), (assign96160_e148749 * locals.var_jssw_dn10), (assign96160_e148749 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign96160_e148753;
        locals.var_isbd_sws_dn0 = assign96160_e148753_d_n0;
        locals.var_isbd_sws_dn2 = assign96160_e148753_d_n2;
        locals.var_isbd_sws_dn4 = assign96160_e148753_d_n4;
        locals.var_isbd_sws_dn5 = assign96160_e148753_d_n5;
        locals.var_isbd_sws_dn6 = assign96160_e148753_d_n6;
        locals.var_isbd_sws_dn7 = assign96160_e148753_d_n7;
        locals.var_isbd_sws_dn8 = assign96160_e148753_d_n8;
        locals.var_isbd_sws_dn9 = assign96160_e148753_d_n9;
        locals.var_isbd_sws_dn10 = assign96160_e148753_d_n10;
        locals.var_isbd_sws_dn13 = assign96160_e148753_d_n13;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96170_e148765, assign96170_e148765_d_n0, assign96170_e148765_d_n2, assign96170_e148765_d_n4, assign96170_e148765_d_n5, assign96170_e148765_d_n6, assign96170_e148765_d_n7, assign96170_e148765_d_n8, assign96170_e148765_d_n9, assign96170_e148765_d_n10, assign96170_e148765_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96170_e148761: f64 = (p.p15 - locals.var_weff_nf);
        let assign96170_e148763: f64 = (assign96170_e148761 * locals.var_jssw2);
        (assign96170_e148763, (assign96170_e148761 * locals.var_jssw2_dn0), (assign96170_e148761 * locals.var_jssw2_dn2), (assign96170_e148761 * locals.var_jssw2_dn4), (assign96170_e148761 * locals.var_jssw2_dn5), (assign96170_e148761 * locals.var_jssw2_dn6), (assign96170_e148761 * locals.var_jssw2_dn7), (assign96170_e148761 * locals.var_jssw2_dn8), (assign96170_e148761 * locals.var_jssw2_dn9), (assign96170_e148761 * locals.var_jssw2_dn10), (assign96170_e148761 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign96170_e148765;
        locals.var_isbd2_sws_dn0 = assign96170_e148765_d_n0;
        locals.var_isbd2_sws_dn2 = assign96170_e148765_d_n2;
        locals.var_isbd2_sws_dn4 = assign96170_e148765_d_n4;
        locals.var_isbd2_sws_dn5 = assign96170_e148765_d_n5;
        locals.var_isbd2_sws_dn6 = assign96170_e148765_d_n6;
        locals.var_isbd2_sws_dn7 = assign96170_e148765_d_n7;
        locals.var_isbd2_sws_dn8 = assign96170_e148765_d_n8;
        locals.var_isbd2_sws_dn9 = assign96170_e148765_d_n9;
        locals.var_isbd2_sws_dn10 = assign96170_e148765_d_n10;
        locals.var_isbd2_sws_dn13 = assign96170_e148765_d_n13;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96180_e148775, assign96180_e148775_d_n0, assign96180_e148775_d_n2, assign96180_e148775_d_n4, assign96180_e148775_d_n5, assign96180_e148775_d_n6, assign96180_e148775_d_n7, assign96180_e148775_d_n8, assign96180_e148775_d_n9, assign96180_e148775_d_n10, assign96180_e148775_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96180_e148773: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96180_e148773, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign96180_e148775;
        locals.var_isbd_swg_dn0 = assign96180_e148775_d_n0;
        locals.var_isbd_swg_dn2 = assign96180_e148775_d_n2;
        locals.var_isbd_swg_dn4 = assign96180_e148775_d_n4;
        locals.var_isbd_swg_dn5 = assign96180_e148775_d_n5;
        locals.var_isbd_swg_dn6 = assign96180_e148775_d_n6;
        locals.var_isbd_swg_dn7 = assign96180_e148775_d_n7;
        locals.var_isbd_swg_dn8 = assign96180_e148775_d_n8;
        locals.var_isbd_swg_dn9 = assign96180_e148775_d_n9;
        locals.var_isbd_swg_dn10 = assign96180_e148775_d_n10;
        locals.var_isbd_swg_dn13 = assign96180_e148775_d_n13;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96190_e148785, assign96190_e148785_d_n0, assign96190_e148785_d_n2, assign96190_e148785_d_n4, assign96190_e148785_d_n5, assign96190_e148785_d_n6, assign96190_e148785_d_n7, assign96190_e148785_d_n8, assign96190_e148785_d_n9, assign96190_e148785_d_n10, assign96190_e148785_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96190_e148783: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96190_e148783, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign96190_e148785;
        locals.var_isbd2_swg_dn0 = assign96190_e148785_d_n0;
        locals.var_isbd2_swg_dn2 = assign96190_e148785_d_n2;
        locals.var_isbd2_swg_dn4 = assign96190_e148785_d_n4;
        locals.var_isbd2_swg_dn5 = assign96190_e148785_d_n5;
        locals.var_isbd2_swg_dn6 = assign96190_e148785_d_n6;
        locals.var_isbd2_swg_dn7 = assign96190_e148785_d_n7;
        locals.var_isbd2_swg_dn8 = assign96190_e148785_d_n8;
        locals.var_isbd2_swg_dn9 = assign96190_e148785_d_n9;
        locals.var_isbd2_swg_dn10 = assign96190_e148785_d_n10;
        locals.var_isbd2_swg_dn13 = assign96190_e148785_d_n13;
        locals.var_isbd2_swg_rv = 0.0;

    }
}
