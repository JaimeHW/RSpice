#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_183(
        locals: &mut StampLocals,
    ) {
        let (assign52520_e79884, assign52520_e79884_d_n0, assign52520_e79884_d_n2, assign52520_e79884_d_n4, assign52520_e79884_d_n5, assign52520_e79884_d_n6, assign52520_e79884_d_n7, assign52520_e79884_d_n8, assign52520_e79884_d_n9, assign52520_e79884_d_n10, assign52520_e79884_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign52520_e79884;
        locals.var_xmp_dn0 = assign52520_e79884_d_n0;
        locals.var_xmp_dn2 = assign52520_e79884_d_n2;
        locals.var_xmp_dn4 = assign52520_e79884_d_n4;
        locals.var_xmp_dn5 = assign52520_e79884_d_n5;
        locals.var_xmp_dn6 = assign52520_e79884_d_n6;
        locals.var_xmp_dn7 = assign52520_e79884_d_n7;
        locals.var_xmp_dn8 = assign52520_e79884_d_n8;
        locals.var_xmp_dn9 = assign52520_e79884_d_n9;
        locals.var_xmp_dn10 = assign52520_e79884_d_n10;
        locals.var_xmp_dn13 = assign52520_e79884_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign52530_e79903,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52530_e79903;
        locals.var_m0_rv = 0.0;

        let (assign52540_e79922,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52540_e79922;
        locals.var_mm_rv = 0.0;

        let (assign52550_e79941, assign52550_e79941_d_n0, assign52550_e79941_d_n2, assign52550_e79941_d_n4, assign52550_e79941_d_n5, assign52550_e79941_d_n6, assign52550_e79941_d_n7, assign52550_e79941_d_n8, assign52550_e79941_d_n9, assign52550_e79941_d_n10, assign52550_e79941_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign52550_e79941;
        locals.var_arg_dn0 = assign52550_e79941_d_n0;
        locals.var_arg_dn2 = assign52550_e79941_d_n2;
        locals.var_arg_dn4 = assign52550_e79941_d_n4;
        locals.var_arg_dn5 = assign52550_e79941_d_n5;
        locals.var_arg_dn6 = assign52550_e79941_d_n6;
        locals.var_arg_dn7 = assign52550_e79941_d_n7;
        locals.var_arg_dn8 = assign52550_e79941_d_n8;
        locals.var_arg_dn9 = assign52550_e79941_d_n9;
        locals.var_arg_dn10 = assign52550_e79941_d_n10;
        locals.var_arg_dn13 = assign52550_e79941_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign52560_e79960, assign52560_e79960_d_n0, assign52560_e79960_d_n2, assign52560_e79960_d_n4, assign52560_e79960_d_n5, assign52560_e79960_d_n6, assign52560_e79960_d_n7, assign52560_e79960_d_n8, assign52560_e79960_d_n9, assign52560_e79960_d_n10, assign52560_e79960_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52560_e79960;
        locals.var_dnm_dn0 = assign52560_e79960_d_n0;
        locals.var_dnm_dn2 = assign52560_e79960_d_n2;
        locals.var_dnm_dn4 = assign52560_e79960_d_n4;
        locals.var_dnm_dn5 = assign52560_e79960_d_n5;
        locals.var_dnm_dn6 = assign52560_e79960_d_n6;
        locals.var_dnm_dn7 = assign52560_e79960_d_n7;
        locals.var_dnm_dn8 = assign52560_e79960_d_n8;
        locals.var_dnm_dn9 = assign52560_e79960_d_n9;
        locals.var_dnm_dn10 = assign52560_e79960_d_n10;
        locals.var_dnm_dn13 = assign52560_e79960_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign52570_e79981, assign52570_e79981_d_n0, assign52570_e79981_d_n2, assign52570_e79981_d_n4, assign52570_e79981_d_n5, assign52570_e79981_d_n6, assign52570_e79981_d_n7, assign52570_e79981_d_n8, assign52570_e79981_d_n9, assign52570_e79981_d_n10, assign52570_e79981_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52570_e79979: f64 = (locals.var_xp * locals.var_x2);
        (assign52570_e79979, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign52570_e79981;
        locals.var_xp_dn0 = assign52570_e79981_d_n0;
        locals.var_xp_dn2 = assign52570_e79981_d_n2;
        locals.var_xp_dn4 = assign52570_e79981_d_n4;
        locals.var_xp_dn5 = assign52570_e79981_d_n5;
        locals.var_xp_dn6 = assign52570_e79981_d_n6;
        locals.var_xp_dn7 = assign52570_e79981_d_n7;
        locals.var_xp_dn8 = assign52570_e79981_d_n8;
        locals.var_xp_dn9 = assign52570_e79981_d_n9;
        locals.var_xp_dn10 = assign52570_e79981_d_n10;
        locals.var_xp_dn13 = assign52570_e79981_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign52580_e80002, assign52580_e80002_d_n0, assign52580_e80002_d_n2, assign52580_e80002_d_n4, assign52580_e80002_d_n5, assign52580_e80002_d_n6, assign52580_e80002_d_n7, assign52580_e80002_d_n8, assign52580_e80002_d_n9, assign52580_e80002_d_n10, assign52580_e80002_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52580_e80000: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52580_e80000, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign52580_e80002;
        locals.var_xmp_dn0 = assign52580_e80002_d_n0;
        locals.var_xmp_dn2 = assign52580_e80002_d_n2;
        locals.var_xmp_dn4 = assign52580_e80002_d_n4;
        locals.var_xmp_dn5 = assign52580_e80002_d_n5;
        locals.var_xmp_dn6 = assign52580_e80002_d_n6;
        locals.var_xmp_dn7 = assign52580_e80002_d_n7;
        locals.var_xmp_dn8 = assign52580_e80002_d_n8;
        locals.var_xmp_dn9 = assign52580_e80002_d_n9;
        locals.var_xmp_dn10 = assign52580_e80002_d_n10;
        locals.var_xmp_dn13 = assign52580_e80002_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign52590_e80023, assign52590_e80023_d_n0, assign52590_e80023_d_n2, assign52590_e80023_d_n4, assign52590_e80023_d_n5, assign52590_e80023_d_n6, assign52590_e80023_d_n7, assign52590_e80023_d_n8, assign52590_e80023_d_n9, assign52590_e80023_d_n10, assign52590_e80023_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52590_e80021: f64 = (locals.var_xp * locals.var_x2);
        (assign52590_e80021, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign52590_e80023;
        locals.var_xp_dn0 = assign52590_e80023_d_n0;
        locals.var_xp_dn2 = assign52590_e80023_d_n2;
        locals.var_xp_dn4 = assign52590_e80023_d_n4;
        locals.var_xp_dn5 = assign52590_e80023_d_n5;
        locals.var_xp_dn6 = assign52590_e80023_d_n6;
        locals.var_xp_dn7 = assign52590_e80023_d_n7;
        locals.var_xp_dn8 = assign52590_e80023_d_n8;
        locals.var_xp_dn9 = assign52590_e80023_d_n9;
        locals.var_xp_dn10 = assign52590_e80023_d_n10;
        locals.var_xp_dn13 = assign52590_e80023_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign52600_e80044, assign52600_e80044_d_n0, assign52600_e80044_d_n2, assign52600_e80044_d_n4, assign52600_e80044_d_n5, assign52600_e80044_d_n6, assign52600_e80044_d_n7, assign52600_e80044_d_n8, assign52600_e80044_d_n9, assign52600_e80044_d_n10, assign52600_e80044_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52600_e80042: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52600_e80042, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign52600_e80044;
        locals.var_xmp_dn0 = assign52600_e80044_d_n0;
        locals.var_xmp_dn2 = assign52600_e80044_d_n2;
        locals.var_xmp_dn4 = assign52600_e80044_d_n4;
        locals.var_xmp_dn5 = assign52600_e80044_d_n5;
        locals.var_xmp_dn6 = assign52600_e80044_d_n6;
        locals.var_xmp_dn7 = assign52600_e80044_d_n7;
        locals.var_xmp_dn8 = assign52600_e80044_d_n8;
        locals.var_xmp_dn9 = assign52600_e80044_d_n9;
        locals.var_xmp_dn10 = assign52600_e80044_d_n10;
        locals.var_xmp_dn13 = assign52600_e80044_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign52610_e80065, assign52610_e80065_d_n0, assign52610_e80065_d_n2, assign52610_e80065_d_n4, assign52610_e80065_d_n5, assign52610_e80065_d_n6, assign52610_e80065_d_n7, assign52610_e80065_d_n8, assign52610_e80065_d_n9, assign52610_e80065_d_n10, assign52610_e80065_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52610_e80063: f64 = (locals.var_xp + locals.var_xmp);
        (assign52610_e80063, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign52610_e80065;
        locals.var_arg_dn0 = assign52610_e80065_d_n0;
        locals.var_arg_dn2 = assign52610_e80065_d_n2;
        locals.var_arg_dn4 = assign52610_e80065_d_n4;
        locals.var_arg_dn5 = assign52610_e80065_d_n5;
        locals.var_arg_dn6 = assign52610_e80065_d_n6;
        locals.var_arg_dn7 = assign52610_e80065_d_n7;
        locals.var_arg_dn8 = assign52610_e80065_d_n8;
        locals.var_arg_dn9 = assign52610_e80065_d_n9;
        locals.var_arg_dn10 = assign52610_e80065_d_n10;
        locals.var_arg_dn13 = assign52610_e80065_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign52620_e80084, assign52620_e80084_d_n0, assign52620_e80084_d_n2, assign52620_e80084_d_n4, assign52620_e80084_d_n5, assign52620_e80084_d_n6, assign52620_e80084_d_n7, assign52620_e80084_d_n8, assign52620_e80084_d_n9, assign52620_e80084_d_n10, assign52620_e80084_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52620_e80084;
        locals.var_dnm_dn0 = assign52620_e80084_d_n0;
        locals.var_dnm_dn2 = assign52620_e80084_d_n2;
        locals.var_dnm_dn4 = assign52620_e80084_d_n4;
        locals.var_dnm_dn5 = assign52620_e80084_d_n5;
        locals.var_dnm_dn6 = assign52620_e80084_d_n6;
        locals.var_dnm_dn7 = assign52620_e80084_d_n7;
        locals.var_dnm_dn8 = assign52620_e80084_d_n8;
        locals.var_dnm_dn9 = assign52620_e80084_d_n9;
        locals.var_dnm_dn10 = assign52620_e80084_d_n10;
        locals.var_dnm_dn13 = assign52620_e80084_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign52630_e80099: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1337 = assign52630_e80099;
        locals.var_guard1337_rv = 0.0;

        let assign52640_e80102: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1338 = assign52640_e80102;
        locals.var_guard1338_rv = 0.0;

        let (assign52650_e80125,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_guard1338 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52650_e80125;
        locals.var_mm_rv = 0.0;

        let assign52660_e80128: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1339 = assign52660_e80128;
        locals.var_guard1339_rv = 0.0;

        let (assign52670_e80154,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_guard1338 == 0.0)) && (locals.var_guard1339 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52670_e80154;
        locals.var_mm_rv = 0.0;

        let assign52680_e80157: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1340 = assign52680_e80157;
        locals.var_guard1340_rv = 0.0;

        let (assign52690_e80186,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_guard1338 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52690_e80186;
        locals.var_mm_rv = 0.0;

        let assign52700_e80189: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1341 = assign52700_e80189;
        locals.var_guard1341_rv = 0.0;

        let (assign52710_e80221,) = {
    if ((((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_guard1338 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 == 0.0)) && (locals.var_guard1341 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52710_e80221;
        locals.var_mm_rv = 0.0;

        let (assign52720_e80242,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52720_e80242;
        locals.var_m0_rv = 0.0;

        let mut assign52730_loop_guard: usize = 0;
        while {
            let assign52730_cond_e80264: f64 = if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign52730_cond_e80264 != 0.0
        } {
            assign52730_loop_guard += 1;
            assert!(assign52730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52730_body0_e80286, assign52730_body0_e80286_d_n0, assign52730_body0_e80286_d_n2, assign52730_body0_e80286_d_n4, assign52730_body0_e80286_d_n5, assign52730_body0_e80286_d_n6, assign52730_body0_e80286_d_n7, assign52730_body0_e80286_d_n8, assign52730_body0_e80286_d_n9, assign52730_body0_e80286_d_n10, assign52730_body0_e80286_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) {
        let assign52730_body0_e80284: f64 = (locals.var_dnm).sqrt();
        (assign52730_body0_e80284, (locals.var_dnm_dn0 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn2 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn4 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn5 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn6 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn7 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn8 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn9 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn10 / (2.0 * assign52730_body0_e80284)), (locals.var_dnm_dn13 / (2.0 * assign52730_body0_e80284)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign52730_body0_e80286;
            locals.var_dnm_dn0 = assign52730_body0_e80286_d_n0;
            locals.var_dnm_dn2 = assign52730_body0_e80286_d_n2;
            locals.var_dnm_dn4 = assign52730_body0_e80286_d_n4;
            locals.var_dnm_dn5 = assign52730_body0_e80286_d_n5;
            locals.var_dnm_dn6 = assign52730_body0_e80286_d_n6;
            locals.var_dnm_dn7 = assign52730_body0_e80286_d_n7;
            locals.var_dnm_dn8 = assign52730_body0_e80286_d_n8;
            locals.var_dnm_dn9 = assign52730_body0_e80286_d_n9;
            locals.var_dnm_dn10 = assign52730_body0_e80286_d_n10;
            locals.var_dnm_dn13 = assign52730_body0_e80286_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign52730_body1_e80309,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 != 0.0)) {
        let assign52730_body1_e80307: f64 = (locals.var_m0 + 1.0);
        (assign52730_body1_e80307,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign52730_body1_e80309;
            locals.var_m0_rv = 0.0;
        }

        let (assign52740_e80342, assign52740_e80342_d_n0, assign52740_e80342_d_n2, assign52740_e80342_d_n4, assign52740_e80342_d_n5, assign52740_e80342_d_n6, assign52740_e80342_d_n7, assign52740_e80342_d_n8, assign52740_e80342_d_n9, assign52740_e80342_d_n10, assign52740_e80342_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) && (locals.var_guard1337 == 0.0)) {
        let (assign52740_e80340, assign52740_e80340_d_n0, assign52740_e80340_d_n2, assign52740_e80340_d_n4, assign52740_e80340_d_n5, assign52740_e80340_d_n6, assign52740_e80340_d_n7, assign52740_e80340_d_n8, assign52740_e80340_d_n9, assign52740_e80340_d_n10, assign52740_e80340_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52740_e80337: f64 = (2.0 * 2.0);
                let assign52740_e80338: f64 = (1.0 / assign52740_e80337);
                let assign52740_e80339: f64 = (locals.var_dnm).powf(assign52740_e80338);
                (assign52740_e80339, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn0)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn2)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn4)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn5)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn6)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn7)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn8)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn9)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn10)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52740_e80338) as f64).is_finite() && ((assign52740_e80338) as f64).fract() == 0.0 { if assign52740_e80338 == 0.0 { 0.0 } else { (assign52740_e80338 * ((locals.var_dnm).powf(assign52740_e80338 - 1.0) * locals.var_dnm_dn13)) } } else { (assign52740_e80339 * (assign52740_e80338 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign52740_e80340, assign52740_e80340_d_n0, assign52740_e80340_d_n2, assign52740_e80340_d_n4, assign52740_e80340_d_n5, assign52740_e80340_d_n6, assign52740_e80340_d_n7, assign52740_e80340_d_n8, assign52740_e80340_d_n9, assign52740_e80340_d_n10, assign52740_e80340_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52740_e80342;
        locals.var_dnm_dn0 = assign52740_e80342_d_n0;
        locals.var_dnm_dn2 = assign52740_e80342_d_n2;
        locals.var_dnm_dn4 = assign52740_e80342_d_n4;
        locals.var_dnm_dn5 = assign52740_e80342_d_n5;
        locals.var_dnm_dn6 = assign52740_e80342_d_n6;
        locals.var_dnm_dn7 = assign52740_e80342_d_n7;
        locals.var_dnm_dn8 = assign52740_e80342_d_n8;
        locals.var_dnm_dn9 = assign52740_e80342_d_n9;
        locals.var_dnm_dn10 = assign52740_e80342_d_n10;
        locals.var_dnm_dn13 = assign52740_e80342_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign52750_e80363, assign52750_e80363_d_n0, assign52750_e80363_d_n2, assign52750_e80363_d_n4, assign52750_e80363_d_n5, assign52750_e80363_d_n6, assign52750_e80363_d_n7, assign52750_e80363_d_n8, assign52750_e80363_d_n9, assign52750_e80363_d_n10, assign52750_e80363_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52750_e80361: f64 = (1.0 / locals.var_dnm);
        (assign52750_e80361, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52750_e80363;
        locals.var_dnm_dn0 = assign52750_e80363_d_n0;
        locals.var_dnm_dn2 = assign52750_e80363_d_n2;
        locals.var_dnm_dn4 = assign52750_e80363_d_n4;
        locals.var_dnm_dn5 = assign52750_e80363_d_n5;
        locals.var_dnm_dn6 = assign52750_e80363_d_n6;
        locals.var_dnm_dn7 = assign52750_e80363_d_n7;
        locals.var_dnm_dn8 = assign52750_e80363_d_n8;
        locals.var_dnm_dn9 = assign52750_e80363_d_n9;
        locals.var_dnm_dn10 = assign52750_e80363_d_n10;
        locals.var_dnm_dn13 = assign52750_e80363_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign52760_e80386, assign52760_e80386_d_n0, assign52760_e80386_d_n2, assign52760_e80386_d_n4, assign52760_e80386_d_n5, assign52760_e80386_d_n6, assign52760_e80386_d_n7, assign52760_e80386_d_n8, assign52760_e80386_d_n9, assign52760_e80386_d_n10, assign52760_e80386_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52760_e80382: f64 = (locals.var_tmf1 * 0.2);
        let assign52760_e80384: f64 = (assign52760_e80382 * locals.var_dnm);
        (assign52760_e80384, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.2) * locals.var_dnm) + (assign52760_e80382 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign52760_e80386;
        locals.var_tmf0_dn0 = assign52760_e80386_d_n0;
        locals.var_tmf0_dn2 = assign52760_e80386_d_n2;
        locals.var_tmf0_dn4 = assign52760_e80386_d_n4;
        locals.var_tmf0_dn5 = assign52760_e80386_d_n5;
        locals.var_tmf0_dn6 = assign52760_e80386_d_n6;
        locals.var_tmf0_dn7 = assign52760_e80386_d_n7;
        locals.var_tmf0_dn8 = assign52760_e80386_d_n8;
        locals.var_tmf0_dn9 = assign52760_e80386_d_n9;
        locals.var_tmf0_dn10 = assign52760_e80386_d_n10;
        locals.var_tmf0_dn13 = assign52760_e80386_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign52770_e80411, assign52770_e80411_d_n0, assign52770_e80411_d_n2, assign52770_e80411_d_n4, assign52770_e80411_d_n5, assign52770_e80411_d_n6, assign52770_e80411_d_n7, assign52770_e80411_d_n8, assign52770_e80411_d_n9, assign52770_e80411_d_n10, assign52770_e80411_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52770_e80405: f64 = (0.2 * locals.var_xmp);
        let assign52770_e80407: f64 = (assign52770_e80405 * locals.var_dnm);
        let assign52770_e80409: f64 = (assign52770_e80407 / locals.var_arg);
        (assign52770_e80409, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn0)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn2)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn4)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn5)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn6)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn7)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn8)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn9)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn10)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn13) * locals.var_dnm) + (assign52770_e80405 * locals.var_dnm_dn13)) * locals.var_arg) - (assign52770_e80407 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign52770_e80411;
        locals.var_t0_dn0 = assign52770_e80411_d_n0;
        locals.var_t0_dn2 = assign52770_e80411_d_n2;
        locals.var_t0_dn4 = assign52770_e80411_d_n4;
        locals.var_t0_dn5 = assign52770_e80411_d_n5;
        locals.var_t0_dn6 = assign52770_e80411_d_n6;
        locals.var_t0_dn7 = assign52770_e80411_d_n7;
        locals.var_t0_dn8 = assign52770_e80411_d_n8;
        locals.var_t0_dn9 = assign52770_e80411_d_n9;
        locals.var_t0_dn10 = assign52770_e80411_d_n10;
        locals.var_t0_dn13 = assign52770_e80411_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign52780_e80434, assign52780_e80434_d_n0, assign52780_e80434_d_n2, assign52780_e80434_d_n4, assign52780_e80434_d_n5, assign52780_e80434_d_n6, assign52780_e80434_d_n7, assign52780_e80434_d_n8, assign52780_e80434_d_n9, assign52780_e80434_d_n10, assign52780_e80434_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        let assign52780_e80430: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52780_e80432: f64 = (assign52780_e80430 - locals.var_tmf0);
        (assign52780_e80432, (locals.var_ps0dep0_dn0 - locals.var_tmf0_dn0), (locals.var_ps0dep0_dn2 - locals.var_tmf0_dn2), (locals.var_ps0dep0_dn4 - locals.var_tmf0_dn4), (locals.var_ps0dep0_dn5 - locals.var_tmf0_dn5), (locals.var_ps0dep0_dn6 - locals.var_tmf0_dn6), (locals.var_ps0dep0_dn7 - locals.var_tmf0_dn7), (locals.var_ps0dep0_dn8 - locals.var_tmf0_dn8), (locals.var_ps0dep0_dn9 - locals.var_tmf0_dn9), (locals.var_ps0dep0_dn10 - locals.var_tmf0_dn10), (locals.var_ps0dep0_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign52780_e80434;
        locals.var_ps0dep_dn0 = assign52780_e80434_d_n0;
        locals.var_ps0dep_dn2 = assign52780_e80434_d_n2;
        locals.var_ps0dep_dn4 = assign52780_e80434_d_n4;
        locals.var_ps0dep_dn5 = assign52780_e80434_d_n5;
        locals.var_ps0dep_dn6 = assign52780_e80434_d_n6;
        locals.var_ps0dep_dn7 = assign52780_e80434_d_n7;
        locals.var_ps0dep_dn8 = assign52780_e80434_d_n8;
        locals.var_ps0dep_dn9 = assign52780_e80434_d_n9;
        locals.var_ps0dep_dn10 = assign52780_e80434_d_n10;
        locals.var_ps0dep_dn13 = assign52780_e80434_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign52790_e80453, assign52790_e80453_d_n0, assign52790_e80453_d_n2, assign52790_e80453_d_n4, assign52790_e80453_d_n5, assign52790_e80453_d_n6, assign52790_e80453_d_n7, assign52790_e80453_d_n8, assign52790_e80453_d_n9, assign52790_e80453_d_n10, assign52790_e80453_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign52790_e80453;
        locals.var_t0_dn0 = assign52790_e80453_d_n0;
        locals.var_t0_dn2 = assign52790_e80453_d_n2;
        locals.var_t0_dn4 = assign52790_e80453_d_n4;
        locals.var_t0_dn5 = assign52790_e80453_d_n5;
        locals.var_t0_dn6 = assign52790_e80453_d_n6;
        locals.var_t0_dn7 = assign52790_e80453_d_n7;
        locals.var_t0_dn8 = assign52790_e80453_d_n8;
        locals.var_t0_dn9 = assign52790_e80453_d_n9;
        locals.var_t0_dn10 = assign52790_e80453_d_n10;
        locals.var_t0_dn13 = assign52790_e80453_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign52800_e80473, assign52800_e80473_d_n0, assign52800_e80473_d_n2, assign52800_e80473_d_n4, assign52800_e80473_d_n5, assign52800_e80473_d_n6, assign52800_e80473_d_n7, assign52800_e80473_d_n8, assign52800_e80473_d_n9, assign52800_e80473_d_n10, assign52800_e80473_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign52800_e80473;
        locals.var_ps0dep_dn0 = assign52800_e80473_d_n0;
        locals.var_ps0dep_dn2 = assign52800_e80473_d_n2;
        locals.var_ps0dep_dn4 = assign52800_e80473_d_n4;
        locals.var_ps0dep_dn5 = assign52800_e80473_d_n5;
        locals.var_ps0dep_dn6 = assign52800_e80473_d_n6;
        locals.var_ps0dep_dn7 = assign52800_e80473_d_n7;
        locals.var_ps0dep_dn8 = assign52800_e80473_d_n8;
        locals.var_ps0dep_dn9 = assign52800_e80473_d_n9;
        locals.var_ps0dep_dn10 = assign52800_e80473_d_n10;
        locals.var_ps0dep_dn13 = assign52800_e80473_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign52810_e80493, assign52810_e80493_d_n0, assign52810_e80493_d_n2, assign52810_e80493_d_n4, assign52810_e80493_d_n5, assign52810_e80493_d_n6, assign52810_e80493_d_n7, assign52810_e80493_d_n8, assign52810_e80493_d_n9, assign52810_e80493_d_n10, assign52810_e80493_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign52810_e80493;
        locals.var_t0_dn0 = assign52810_e80493_d_n0;
        locals.var_t0_dn2 = assign52810_e80493_d_n2;
        locals.var_t0_dn4 = assign52810_e80493_d_n4;
        locals.var_t0_dn5 = assign52810_e80493_d_n5;
        locals.var_t0_dn6 = assign52810_e80493_d_n6;
        locals.var_t0_dn7 = assign52810_e80493_d_n7;
        locals.var_t0_dn8 = assign52810_e80493_d_n8;
        locals.var_t0_dn9 = assign52810_e80493_d_n9;
        locals.var_t0_dn10 = assign52810_e80493_d_n10;
        locals.var_t0_dn13 = assign52810_e80493_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign52820_e80507, assign52820_e80507_d_n0, assign52820_e80507_d_n2, assign52820_e80507_d_n4, assign52820_e80507_d_n5, assign52820_e80507_d_n6, assign52820_e80507_d_n7, assign52820_e80507_d_n8, assign52820_e80507_d_n9, assign52820_e80507_d_n10, assign52820_e80507_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    }
};
        locals.var_ps0_res = assign52820_e80507;
        locals.var_ps0_res_dn0 = assign52820_e80507_d_n0;
        locals.var_ps0_res_dn2 = assign52820_e80507_d_n2;
        locals.var_ps0_res_dn4 = assign52820_e80507_d_n4;
        locals.var_ps0_res_dn5 = assign52820_e80507_d_n5;
        locals.var_ps0_res_dn6 = assign52820_e80507_d_n6;
        locals.var_ps0_res_dn7 = assign52820_e80507_d_n7;
        locals.var_ps0_res_dn8 = assign52820_e80507_d_n8;
        locals.var_ps0_res_dn9 = assign52820_e80507_d_n9;
        locals.var_ps0_res_dn10 = assign52820_e80507_d_n10;
        locals.var_ps0_res_dn13 = assign52820_e80507_d_n13;
        locals.var_ps0_res_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_184(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52830_e80526,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let (assign52830_e80524,) = {
            if (1e-6 >= p.p407) {
                (1e-6,)
            } else {
                (p.p407,)
            }
        };
        (assign52830_e80524,)
    } else {
        (locals.var_vgpdep_dlt__blk1142,)
    }
};
        locals.var_vgpdep_dlt__blk1142 = assign52830_e80526;
        locals.var_vgpdep_dlt__blk1142_rv = 0.0;

        let assign52840_e80530: f64 = (-locals.var_vgpdep_dlt__blk1142);
        let assign52840_e80535: f64 = if ((locals.var_ps0_res > assign52840_e80530) && (locals.var_vgpdep_dlt__blk1142 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1342 = assign52840_e80535;
        locals.var_guard1342_rv = 0.0;

        let (assign52850_e80555, assign52850_e80555_d_n0, assign52850_e80555_d_n2, assign52850_e80555_d_n4, assign52850_e80555_d_n5, assign52850_e80555_d_n6, assign52850_e80555_d_n7, assign52850_e80555_d_n8, assign52850_e80555_d_n9, assign52850_e80555_d_n10, assign52850_e80555_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52850_e80551: f64 = locals.var_ps0_res;
        let assign52850_e80553: f64 = (assign52850_e80551 + locals.var_vgpdep_dlt__blk1142);
        (assign52850_e80553, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign52850_e80555;
        locals.var_tmf1_dn0 = assign52850_e80555_d_n0;
        locals.var_tmf1_dn2 = assign52850_e80555_d_n2;
        locals.var_tmf1_dn4 = assign52850_e80555_d_n4;
        locals.var_tmf1_dn5 = assign52850_e80555_d_n5;
        locals.var_tmf1_dn6 = assign52850_e80555_d_n6;
        locals.var_tmf1_dn7 = assign52850_e80555_d_n7;
        locals.var_tmf1_dn8 = assign52850_e80555_d_n8;
        locals.var_tmf1_dn9 = assign52850_e80555_d_n9;
        locals.var_tmf1_dn10 = assign52850_e80555_d_n10;
        locals.var_tmf1_dn13 = assign52850_e80555_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign52860_e80573, assign52860_e80573_d_n0, assign52860_e80573_d_n2, assign52860_e80573_d_n4, assign52860_e80573_d_n5, assign52860_e80573_d_n6, assign52860_e80573_d_n7, assign52860_e80573_d_n8, assign52860_e80573_d_n9, assign52860_e80573_d_n10, assign52860_e80573_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52860_e80571: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign52860_e80571, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign52860_e80573;
        locals.var_x2_dn0 = assign52860_e80573_d_n0;
        locals.var_x2_dn2 = assign52860_e80573_d_n2;
        locals.var_x2_dn4 = assign52860_e80573_d_n4;
        locals.var_x2_dn5 = assign52860_e80573_d_n5;
        locals.var_x2_dn6 = assign52860_e80573_d_n6;
        locals.var_x2_dn7 = assign52860_e80573_d_n7;
        locals.var_x2_dn8 = assign52860_e80573_d_n8;
        locals.var_x2_dn9 = assign52860_e80573_d_n9;
        locals.var_x2_dn10 = assign52860_e80573_d_n10;
        locals.var_x2_dn13 = assign52860_e80573_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign52870_e80591, assign52870_e80591_d_n0, assign52870_e80591_d_n2, assign52870_e80591_d_n4, assign52870_e80591_d_n5, assign52870_e80591_d_n6, assign52870_e80591_d_n7, assign52870_e80591_d_n8, assign52870_e80591_d_n9, assign52870_e80591_d_n10, assign52870_e80591_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52870_e80589: f64 = (locals.var_vgpdep_dlt__blk1142 * locals.var_vgpdep_dlt__blk1142);
        (assign52870_e80589, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign52870_e80591;
        locals.var_xmax2_dn0 = assign52870_e80591_d_n0;
        locals.var_xmax2_dn2 = assign52870_e80591_d_n2;
        locals.var_xmax2_dn4 = assign52870_e80591_d_n4;
        locals.var_xmax2_dn5 = assign52870_e80591_d_n5;
        locals.var_xmax2_dn6 = assign52870_e80591_d_n6;
        locals.var_xmax2_dn7 = assign52870_e80591_d_n7;
        locals.var_xmax2_dn8 = assign52870_e80591_d_n8;
        locals.var_xmax2_dn9 = assign52870_e80591_d_n9;
        locals.var_xmax2_dn10 = assign52870_e80591_d_n10;
        locals.var_xmax2_dn13 = assign52870_e80591_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign52880_e80607, assign52880_e80607_d_n0, assign52880_e80607_d_n2, assign52880_e80607_d_n4, assign52880_e80607_d_n5, assign52880_e80607_d_n6, assign52880_e80607_d_n7, assign52880_e80607_d_n8, assign52880_e80607_d_n9, assign52880_e80607_d_n10, assign52880_e80607_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign52880_e80607;
        locals.var_xp_dn0 = assign52880_e80607_d_n0;
        locals.var_xp_dn2 = assign52880_e80607_d_n2;
        locals.var_xp_dn4 = assign52880_e80607_d_n4;
        locals.var_xp_dn5 = assign52880_e80607_d_n5;
        locals.var_xp_dn6 = assign52880_e80607_d_n6;
        locals.var_xp_dn7 = assign52880_e80607_d_n7;
        locals.var_xp_dn8 = assign52880_e80607_d_n8;
        locals.var_xp_dn9 = assign52880_e80607_d_n9;
        locals.var_xp_dn10 = assign52880_e80607_d_n10;
        locals.var_xp_dn13 = assign52880_e80607_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign52890_e80623, assign52890_e80623_d_n0, assign52890_e80623_d_n2, assign52890_e80623_d_n4, assign52890_e80623_d_n5, assign52890_e80623_d_n6, assign52890_e80623_d_n7, assign52890_e80623_d_n8, assign52890_e80623_d_n9, assign52890_e80623_d_n10, assign52890_e80623_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign52890_e80623;
        locals.var_xmp_dn0 = assign52890_e80623_d_n0;
        locals.var_xmp_dn2 = assign52890_e80623_d_n2;
        locals.var_xmp_dn4 = assign52890_e80623_d_n4;
        locals.var_xmp_dn5 = assign52890_e80623_d_n5;
        locals.var_xmp_dn6 = assign52890_e80623_d_n6;
        locals.var_xmp_dn7 = assign52890_e80623_d_n7;
        locals.var_xmp_dn8 = assign52890_e80623_d_n8;
        locals.var_xmp_dn9 = assign52890_e80623_d_n9;
        locals.var_xmp_dn10 = assign52890_e80623_d_n10;
        locals.var_xmp_dn13 = assign52890_e80623_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign52900_e80639,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52900_e80639;
        locals.var_m0_rv = 0.0;

        let (assign52910_e80655,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52910_e80655;
        locals.var_mm_rv = 0.0;

        let (assign52920_e80671, assign52920_e80671_d_n0, assign52920_e80671_d_n2, assign52920_e80671_d_n4, assign52920_e80671_d_n5, assign52920_e80671_d_n6, assign52920_e80671_d_n7, assign52920_e80671_d_n8, assign52920_e80671_d_n9, assign52920_e80671_d_n10, assign52920_e80671_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign52920_e80671;
        locals.var_arg_dn0 = assign52920_e80671_d_n0;
        locals.var_arg_dn2 = assign52920_e80671_d_n2;
        locals.var_arg_dn4 = assign52920_e80671_d_n4;
        locals.var_arg_dn5 = assign52920_e80671_d_n5;
        locals.var_arg_dn6 = assign52920_e80671_d_n6;
        locals.var_arg_dn7 = assign52920_e80671_d_n7;
        locals.var_arg_dn8 = assign52920_e80671_d_n8;
        locals.var_arg_dn9 = assign52920_e80671_d_n9;
        locals.var_arg_dn10 = assign52920_e80671_d_n10;
        locals.var_arg_dn13 = assign52920_e80671_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign52930_e80687, assign52930_e80687_d_n0, assign52930_e80687_d_n2, assign52930_e80687_d_n4, assign52930_e80687_d_n5, assign52930_e80687_d_n6, assign52930_e80687_d_n7, assign52930_e80687_d_n8, assign52930_e80687_d_n9, assign52930_e80687_d_n10, assign52930_e80687_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52930_e80687;
        locals.var_dnm_dn0 = assign52930_e80687_d_n0;
        locals.var_dnm_dn2 = assign52930_e80687_d_n2;
        locals.var_dnm_dn4 = assign52930_e80687_d_n4;
        locals.var_dnm_dn5 = assign52930_e80687_d_n5;
        locals.var_dnm_dn6 = assign52930_e80687_d_n6;
        locals.var_dnm_dn7 = assign52930_e80687_d_n7;
        locals.var_dnm_dn8 = assign52930_e80687_d_n8;
        locals.var_dnm_dn9 = assign52930_e80687_d_n9;
        locals.var_dnm_dn10 = assign52930_e80687_d_n10;
        locals.var_dnm_dn13 = assign52930_e80687_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign52940_e80703,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52940_e80703;
        locals.var_m0_rv = 0.0;

        let mut assign52950_loop_guard: usize = 0;
        while {
            let assign52950_cond_e80720: f64 = if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw__blk1143)) { 1.0 } else { 0.0 };
            assign52950_cond_e80720 != 0.0
        } {
            assign52950_loop_guard += 1;
            assert!(assign52950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52950_body0_e80738, assign52950_body0_e80738_d_n0, assign52950_body0_e80738_d_n2, assign52950_body0_e80738_d_n4, assign52950_body0_e80738_d_n5, assign52950_body0_e80738_d_n6, assign52950_body0_e80738_d_n7, assign52950_body0_e80738_d_n8, assign52950_body0_e80738_d_n9, assign52950_body0_e80738_d_n10, assign52950_body0_e80738_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52950_body0_e80736: f64 = (locals.var_xp * locals.var_x2);
        (assign52950_body0_e80736, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign52950_body0_e80738;
            locals.var_xp_dn0 = assign52950_body0_e80738_d_n0;
            locals.var_xp_dn2 = assign52950_body0_e80738_d_n2;
            locals.var_xp_dn4 = assign52950_body0_e80738_d_n4;
            locals.var_xp_dn5 = assign52950_body0_e80738_d_n5;
            locals.var_xp_dn6 = assign52950_body0_e80738_d_n6;
            locals.var_xp_dn7 = assign52950_body0_e80738_d_n7;
            locals.var_xp_dn8 = assign52950_body0_e80738_d_n8;
            locals.var_xp_dn9 = assign52950_body0_e80738_d_n9;
            locals.var_xp_dn10 = assign52950_body0_e80738_d_n10;
            locals.var_xp_dn13 = assign52950_body0_e80738_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign52950_body1_e80756, assign52950_body1_e80756_d_n0, assign52950_body1_e80756_d_n2, assign52950_body1_e80756_d_n4, assign52950_body1_e80756_d_n5, assign52950_body1_e80756_d_n6, assign52950_body1_e80756_d_n7, assign52950_body1_e80756_d_n8, assign52950_body1_e80756_d_n9, assign52950_body1_e80756_d_n10, assign52950_body1_e80756_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52950_body1_e80754: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52950_body1_e80754, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign52950_body1_e80756;
            locals.var_xmp_dn0 = assign52950_body1_e80756_d_n0;
            locals.var_xmp_dn2 = assign52950_body1_e80756_d_n2;
            locals.var_xmp_dn4 = assign52950_body1_e80756_d_n4;
            locals.var_xmp_dn5 = assign52950_body1_e80756_d_n5;
            locals.var_xmp_dn6 = assign52950_body1_e80756_d_n6;
            locals.var_xmp_dn7 = assign52950_body1_e80756_d_n7;
            locals.var_xmp_dn8 = assign52950_body1_e80756_d_n8;
            locals.var_xmp_dn9 = assign52950_body1_e80756_d_n9;
            locals.var_xmp_dn10 = assign52950_body1_e80756_d_n10;
            locals.var_xmp_dn13 = assign52950_body1_e80756_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign52950_body2_e80774,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52950_body2_e80772: f64 = (locals.var_m0 + 1.0);
        (assign52950_body2_e80772,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign52950_body2_e80774;
            locals.var_m0_rv = 0.0;
        }

        let (assign52960_e80792, assign52960_e80792_d_n0, assign52960_e80792_d_n2, assign52960_e80792_d_n4, assign52960_e80792_d_n5, assign52960_e80792_d_n6, assign52960_e80792_d_n7, assign52960_e80792_d_n8, assign52960_e80792_d_n9, assign52960_e80792_d_n10, assign52960_e80792_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign52960_e80790: f64 = (locals.var_xp + locals.var_xmp);
        (assign52960_e80790, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign52960_e80792;
        locals.var_arg_dn0 = assign52960_e80792_d_n0;
        locals.var_arg_dn2 = assign52960_e80792_d_n2;
        locals.var_arg_dn4 = assign52960_e80792_d_n4;
        locals.var_arg_dn5 = assign52960_e80792_d_n5;
        locals.var_arg_dn6 = assign52960_e80792_d_n6;
        locals.var_arg_dn7 = assign52960_e80792_d_n7;
        locals.var_arg_dn8 = assign52960_e80792_d_n8;
        locals.var_arg_dn9 = assign52960_e80792_d_n9;
        locals.var_arg_dn10 = assign52960_e80792_d_n10;
        locals.var_arg_dn13 = assign52960_e80792_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign52970_e80808, assign52970_e80808_d_n0, assign52970_e80808_d_n2, assign52970_e80808_d_n4, assign52970_e80808_d_n5, assign52970_e80808_d_n6, assign52970_e80808_d_n7, assign52970_e80808_d_n8, assign52970_e80808_d_n9, assign52970_e80808_d_n10, assign52970_e80808_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign52970_e80808;
        locals.var_dnm_dn0 = assign52970_e80808_d_n0;
        locals.var_dnm_dn2 = assign52970_e80808_d_n2;
        locals.var_dnm_dn4 = assign52970_e80808_d_n4;
        locals.var_dnm_dn5 = assign52970_e80808_d_n5;
        locals.var_dnm_dn6 = assign52970_e80808_d_n6;
        locals.var_dnm_dn7 = assign52970_e80808_d_n7;
        locals.var_dnm_dn8 = assign52970_e80808_d_n8;
        locals.var_dnm_dn9 = assign52970_e80808_d_n9;
        locals.var_dnm_dn10 = assign52970_e80808_d_n10;
        locals.var_dnm_dn13 = assign52970_e80808_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign52980_e80823: f64 = if ((((locals.var_vgpdep_pw__blk1143 == 1.0) || (locals.var_vgpdep_pw__blk1143 == 2.0)) || (locals.var_vgpdep_pw__blk1143 == 4.0)) || (locals.var_vgpdep_pw__blk1143 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1343 = assign52980_e80823;
        locals.var_guard1343_rv = 0.0;

        let assign52990_e80826: f64 = if locals.var_vgpdep_pw__blk1143 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1344 = assign52990_e80826;
        locals.var_guard1344_rv = 0.0;

        let (assign53000_e80846,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_guard1344 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53000_e80846;
        locals.var_mm_rv = 0.0;

        let assign53010_e80849: f64 = if locals.var_vgpdep_pw__blk1143 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1345 = assign53010_e80849;
        locals.var_guard1345_rv = 0.0;

        let (assign53020_e80872,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_guard1344 == 0.0)) && (locals.var_guard1345 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53020_e80872;
        locals.var_mm_rv = 0.0;

        let assign53030_e80875: f64 = if locals.var_vgpdep_pw__blk1143 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1346 = assign53030_e80875;
        locals.var_guard1346_rv = 0.0;

        let (assign53040_e80901,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_guard1344 == 0.0)) && (locals.var_guard1345 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53040_e80901;
        locals.var_mm_rv = 0.0;

        let assign53050_e80904: f64 = if locals.var_vgpdep_pw__blk1143 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1347 = assign53050_e80904;
        locals.var_guard1347_rv = 0.0;

        let (assign53060_e80933,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_guard1344 == 0.0)) && (locals.var_guard1345 == 0.0)) && (locals.var_guard1346 == 0.0)) && (locals.var_guard1347 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53060_e80933;
        locals.var_mm_rv = 0.0;

        let (assign53070_e80951,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53070_e80951;
        locals.var_m0_rv = 0.0;

        let mut assign53080_loop_guard: usize = 0;
        while {
            let assign53080_cond_e80970: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign53080_cond_e80970 != 0.0
        } {
            assign53080_loop_guard += 1;
            assert!(assign53080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53080_body0_e80989, assign53080_body0_e80989_d_n0, assign53080_body0_e80989_d_n2, assign53080_body0_e80989_d_n4, assign53080_body0_e80989_d_n5, assign53080_body0_e80989_d_n6, assign53080_body0_e80989_d_n7, assign53080_body0_e80989_d_n8, assign53080_body0_e80989_d_n9, assign53080_body0_e80989_d_n10, assign53080_body0_e80989_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) {
        let assign53080_body0_e80987: f64 = (locals.var_dnm).sqrt();
        (assign53080_body0_e80987, (locals.var_dnm_dn0 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn2 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn4 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn5 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn6 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn7 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn8 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn9 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn10 / (2.0 * assign53080_body0_e80987)), (locals.var_dnm_dn13 / (2.0 * assign53080_body0_e80987)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign53080_body0_e80989;
            locals.var_dnm_dn0 = assign53080_body0_e80989_d_n0;
            locals.var_dnm_dn2 = assign53080_body0_e80989_d_n2;
            locals.var_dnm_dn4 = assign53080_body0_e80989_d_n4;
            locals.var_dnm_dn5 = assign53080_body0_e80989_d_n5;
            locals.var_dnm_dn6 = assign53080_body0_e80989_d_n6;
            locals.var_dnm_dn7 = assign53080_body0_e80989_d_n7;
            locals.var_dnm_dn8 = assign53080_body0_e80989_d_n8;
            locals.var_dnm_dn9 = assign53080_body0_e80989_d_n9;
            locals.var_dnm_dn10 = assign53080_body0_e80989_d_n10;
            locals.var_dnm_dn13 = assign53080_body0_e80989_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign53080_body1_e81009,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 != 0.0)) {
        let assign53080_body1_e81007: f64 = (locals.var_m0 + 1.0);
        (assign53080_body1_e81007,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign53080_body1_e81009;
            locals.var_m0_rv = 0.0;
        }

        let (assign53090_e81039, assign53090_e81039_d_n0, assign53090_e81039_d_n2, assign53090_e81039_d_n4, assign53090_e81039_d_n5, assign53090_e81039_d_n6, assign53090_e81039_d_n7, assign53090_e81039_d_n8, assign53090_e81039_d_n9, assign53090_e81039_d_n10, assign53090_e81039_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) && (locals.var_guard1343 == 0.0)) {
        let (assign53090_e81037, assign53090_e81037_d_n0, assign53090_e81037_d_n2, assign53090_e81037_d_n4, assign53090_e81037_d_n5, assign53090_e81037_d_n6, assign53090_e81037_d_n7, assign53090_e81037_d_n8, assign53090_e81037_d_n9, assign53090_e81037_d_n10, assign53090_e81037_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign53090_e81034: f64 = (2.0 * locals.var_vgpdep_pw__blk1143);
                let assign53090_e81035: f64 = (1.0 / assign53090_e81034);
                let assign53090_e81036: f64 = (locals.var_dnm).powf(assign53090_e81035);
                (assign53090_e81036, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn0)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn2)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn4)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn5)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn6)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn7)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn8)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn9)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn10)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53090_e81035) as f64).is_finite() && ((assign53090_e81035) as f64).fract() == 0.0 { if assign53090_e81035 == 0.0 { 0.0 } else { (assign53090_e81035 * ((locals.var_dnm).powf(assign53090_e81035 - 1.0) * locals.var_dnm_dn13)) } } else { (assign53090_e81036 * (assign53090_e81035 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign53090_e81037, assign53090_e81037_d_n0, assign53090_e81037_d_n2, assign53090_e81037_d_n4, assign53090_e81037_d_n5, assign53090_e81037_d_n6, assign53090_e81037_d_n7, assign53090_e81037_d_n8, assign53090_e81037_d_n9, assign53090_e81037_d_n10, assign53090_e81037_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign53090_e81039;
        locals.var_dnm_dn0 = assign53090_e81039_d_n0;
        locals.var_dnm_dn2 = assign53090_e81039_d_n2;
        locals.var_dnm_dn4 = assign53090_e81039_d_n4;
        locals.var_dnm_dn5 = assign53090_e81039_d_n5;
        locals.var_dnm_dn6 = assign53090_e81039_d_n6;
        locals.var_dnm_dn7 = assign53090_e81039_d_n7;
        locals.var_dnm_dn8 = assign53090_e81039_d_n8;
        locals.var_dnm_dn9 = assign53090_e81039_d_n9;
        locals.var_dnm_dn10 = assign53090_e81039_d_n10;
        locals.var_dnm_dn13 = assign53090_e81039_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign53100_e81057, assign53100_e81057_d_n0, assign53100_e81057_d_n2, assign53100_e81057_d_n4, assign53100_e81057_d_n5, assign53100_e81057_d_n6, assign53100_e81057_d_n7, assign53100_e81057_d_n8, assign53100_e81057_d_n9, assign53100_e81057_d_n10, assign53100_e81057_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign53100_e81055: f64 = (1.0 / locals.var_dnm);
        (assign53100_e81055, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign53100_e81057;
        locals.var_dnm_dn0 = assign53100_e81057_d_n0;
        locals.var_dnm_dn2 = assign53100_e81057_d_n2;
        locals.var_dnm_dn4 = assign53100_e81057_d_n4;
        locals.var_dnm_dn5 = assign53100_e81057_d_n5;
        locals.var_dnm_dn6 = assign53100_e81057_d_n6;
        locals.var_dnm_dn7 = assign53100_e81057_d_n7;
        locals.var_dnm_dn8 = assign53100_e81057_d_n8;
        locals.var_dnm_dn9 = assign53100_e81057_d_n9;
        locals.var_dnm_dn10 = assign53100_e81057_d_n10;
        locals.var_dnm_dn13 = assign53100_e81057_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign53110_e81077, assign53110_e81077_d_n0, assign53110_e81077_d_n2, assign53110_e81077_d_n4, assign53110_e81077_d_n5, assign53110_e81077_d_n6, assign53110_e81077_d_n7, assign53110_e81077_d_n8, assign53110_e81077_d_n9, assign53110_e81077_d_n10, assign53110_e81077_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign53110_e81073: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt__blk1142);
        let assign53110_e81075: f64 = (assign53110_e81073 * locals.var_dnm);
        (assign53110_e81075, (((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign53110_e81073 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign53110_e81077;
        locals.var_tmf0_dn0 = assign53110_e81077_d_n0;
        locals.var_tmf0_dn2 = assign53110_e81077_d_n2;
        locals.var_tmf0_dn4 = assign53110_e81077_d_n4;
        locals.var_tmf0_dn5 = assign53110_e81077_d_n5;
        locals.var_tmf0_dn6 = assign53110_e81077_d_n6;
        locals.var_tmf0_dn7 = assign53110_e81077_d_n7;
        locals.var_tmf0_dn8 = assign53110_e81077_d_n8;
        locals.var_tmf0_dn9 = assign53110_e81077_d_n9;
        locals.var_tmf0_dn10 = assign53110_e81077_d_n10;
        locals.var_tmf0_dn13 = assign53110_e81077_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign53120_e81099, assign53120_e81099_d_n0, assign53120_e81099_d_n2, assign53120_e81099_d_n4, assign53120_e81099_d_n5, assign53120_e81099_d_n6, assign53120_e81099_d_n7, assign53120_e81099_d_n8, assign53120_e81099_d_n9, assign53120_e81099_d_n10, assign53120_e81099_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign53120_e81093: f64 = (locals.var_vgpdep_dlt__blk1142 * locals.var_xmp);
        let assign53120_e81095: f64 = (assign53120_e81093 * locals.var_dnm);
        let assign53120_e81097: f64 = (assign53120_e81095 / locals.var_arg);
        (assign53120_e81097, ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn0) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn0)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn2) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn2)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn4) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn4)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn5) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn5)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn6) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn6)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn7) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn7)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn8) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn8)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn9) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn9)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn10) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn10)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn13) * locals.var_dnm) + (assign53120_e81093 * locals.var_dnm_dn13)) * locals.var_arg) - (assign53120_e81095 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53120_e81099;
        locals.var_t0_dn0 = assign53120_e81099_d_n0;
        locals.var_t0_dn2 = assign53120_e81099_d_n2;
        locals.var_t0_dn4 = assign53120_e81099_d_n4;
        locals.var_t0_dn5 = assign53120_e81099_d_n5;
        locals.var_t0_dn6 = assign53120_e81099_d_n6;
        locals.var_t0_dn7 = assign53120_e81099_d_n7;
        locals.var_t0_dn8 = assign53120_e81099_d_n8;
        locals.var_t0_dn9 = assign53120_e81099_d_n9;
        locals.var_t0_dn10 = assign53120_e81099_d_n10;
        locals.var_t0_dn13 = assign53120_e81099_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_185(
        locals: &mut StampLocals,
    ) {
        let (assign53130_e81119, assign53130_e81119_d_n0, assign53130_e81119_d_n2, assign53130_e81119_d_n4, assign53130_e81119_d_n5, assign53130_e81119_d_n6, assign53130_e81119_d_n7, assign53130_e81119_d_n8, assign53130_e81119_d_n9, assign53130_e81119_d_n10, assign53130_e81119_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        let assign53130_e81115: f64 = (-locals.var_vgpdep_dlt__blk1142);
        let assign53130_e81117: f64 = (assign53130_e81115 + locals.var_tmf0);
        (assign53130_e81117, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign53130_e81119;
        locals.var_ps0dep_dn0 = assign53130_e81119_d_n0;
        locals.var_ps0dep_dn2 = assign53130_e81119_d_n2;
        locals.var_ps0dep_dn4 = assign53130_e81119_d_n4;
        locals.var_ps0dep_dn5 = assign53130_e81119_d_n5;
        locals.var_ps0dep_dn6 = assign53130_e81119_d_n6;
        locals.var_ps0dep_dn7 = assign53130_e81119_d_n7;
        locals.var_ps0dep_dn8 = assign53130_e81119_d_n8;
        locals.var_ps0dep_dn9 = assign53130_e81119_d_n9;
        locals.var_ps0dep_dn10 = assign53130_e81119_d_n10;
        locals.var_ps0dep_dn13 = assign53130_e81119_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign53140_e81135, assign53140_e81135_d_n0, assign53140_e81135_d_n2, assign53140_e81135_d_n4, assign53140_e81135_d_n5, assign53140_e81135_d_n6, assign53140_e81135_d_n7, assign53140_e81135_d_n8, assign53140_e81135_d_n9, assign53140_e81135_d_n10, assign53140_e81135_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53140_e81135;
        locals.var_t0_dn0 = assign53140_e81135_d_n0;
        locals.var_t0_dn2 = assign53140_e81135_d_n2;
        locals.var_t0_dn4 = assign53140_e81135_d_n4;
        locals.var_t0_dn5 = assign53140_e81135_d_n5;
        locals.var_t0_dn6 = assign53140_e81135_d_n6;
        locals.var_t0_dn7 = assign53140_e81135_d_n7;
        locals.var_t0_dn8 = assign53140_e81135_d_n8;
        locals.var_t0_dn9 = assign53140_e81135_d_n9;
        locals.var_t0_dn10 = assign53140_e81135_d_n10;
        locals.var_t0_dn13 = assign53140_e81135_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign53150_e81152, assign53150_e81152_d_n0, assign53150_e81152_d_n2, assign53150_e81152_d_n4, assign53150_e81152_d_n5, assign53150_e81152_d_n6, assign53150_e81152_d_n7, assign53150_e81152_d_n8, assign53150_e81152_d_n9, assign53150_e81152_d_n10, assign53150_e81152_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 == 0.0)) {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign53150_e81152;
        locals.var_ps0dep_dn0 = assign53150_e81152_d_n0;
        locals.var_ps0dep_dn2 = assign53150_e81152_d_n2;
        locals.var_ps0dep_dn4 = assign53150_e81152_d_n4;
        locals.var_ps0dep_dn5 = assign53150_e81152_d_n5;
        locals.var_ps0dep_dn6 = assign53150_e81152_d_n6;
        locals.var_ps0dep_dn7 = assign53150_e81152_d_n7;
        locals.var_ps0dep_dn8 = assign53150_e81152_d_n8;
        locals.var_ps0dep_dn9 = assign53150_e81152_d_n9;
        locals.var_ps0dep_dn10 = assign53150_e81152_d_n10;
        locals.var_ps0dep_dn13 = assign53150_e81152_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign53160_e81169, assign53160_e81169_d_n0, assign53160_e81169_d_n2, assign53160_e81169_d_n4, assign53160_e81169_d_n5, assign53160_e81169_d_n6, assign53160_e81169_d_n7, assign53160_e81169_d_n8, assign53160_e81169_d_n9, assign53160_e81169_d_n10, assign53160_e81169_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1342 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53160_e81169;
        locals.var_t0_dn0 = assign53160_e81169_d_n0;
        locals.var_t0_dn2 = assign53160_e81169_d_n2;
        locals.var_t0_dn4 = assign53160_e81169_d_n4;
        locals.var_t0_dn5 = assign53160_e81169_d_n5;
        locals.var_t0_dn6 = assign53160_e81169_d_n6;
        locals.var_t0_dn7 = assign53160_e81169_d_n7;
        locals.var_t0_dn8 = assign53160_e81169_d_n8;
        locals.var_t0_dn9 = assign53160_e81169_d_n9;
        locals.var_t0_dn10 = assign53160_e81169_d_n10;
        locals.var_t0_dn13 = assign53160_e81169_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign53170_e81184, assign53170_e81184_d_n0, assign53170_e81184_d_n2, assign53170_e81184_d_n4, assign53170_e81184_d_n5, assign53170_e81184_d_n6, assign53170_e81184_d_n7, assign53170_e81184_d_n8, assign53170_e81184_d_n9, assign53170_e81184_d_n10, assign53170_e81184_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign53170_e81182: f64 = (-locals.var_ps0dep);
        (assign53170_e81182, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign53170_e81184;
        locals.var_ps0dep_dn0 = assign53170_e81184_d_n0;
        locals.var_ps0dep_dn2 = assign53170_e81184_d_n2;
        locals.var_ps0dep_dn4 = assign53170_e81184_d_n4;
        locals.var_ps0dep_dn5 = assign53170_e81184_d_n5;
        locals.var_ps0dep_dn6 = assign53170_e81184_d_n6;
        locals.var_ps0dep_dn7 = assign53170_e81184_d_n7;
        locals.var_ps0dep_dn8 = assign53170_e81184_d_n8;
        locals.var_ps0dep_dn9 = assign53170_e81184_d_n9;
        locals.var_ps0dep_dn10 = assign53170_e81184_d_n10;
        locals.var_ps0dep_dn13 = assign53170_e81184_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign53180_e81206, assign53180_e81206_d_n0, assign53180_e81206_d_n2, assign53180_e81206_d_n4, assign53180_e81206_d_n5, assign53180_e81206_d_n6, assign53180_e81206_d_n7, assign53180_e81206_d_n8, assign53180_e81206_d_n9, assign53180_e81206_d_n10, assign53180_e81206_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign53180_e81198: f64 = (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148);
        let assign53180_e81200: f64 = (assign53180_e81198 * locals.var_tnp__blk1148);
        let assign53180_e81202: f64 = (assign53180_e81200 / 2.0);
        let assign53180_e81204: f64 = (assign53180_e81202 / 1.034943e-10);
        (assign53180_e81204, ((((((locals.var_q_ndepm__blk1133_dn0 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn0)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn2 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn2)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn4 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn4)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn5 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn5)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn6 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn6)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn7 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn7)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn8 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn8)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn9 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn9)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn10 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn10)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn13 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn13)) * locals.var_tnp__blk1148) + (assign53180_e81198 * locals.var_tnp__blk1148_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1320, locals.var_dphi_sb__blk1320_dn0, locals.var_dphi_sb__blk1320_dn2, locals.var_dphi_sb__blk1320_dn4, locals.var_dphi_sb__blk1320_dn5, locals.var_dphi_sb__blk1320_dn6, locals.var_dphi_sb__blk1320_dn7, locals.var_dphi_sb__blk1320_dn8, locals.var_dphi_sb__blk1320_dn9, locals.var_dphi_sb__blk1320_dn10, locals.var_dphi_sb__blk1320_dn13,)
    }
};
        locals.var_dphi_sb__blk1320 = assign53180_e81206;
        locals.var_dphi_sb__blk1320_dn0 = assign53180_e81206_d_n0;
        locals.var_dphi_sb__blk1320_dn2 = assign53180_e81206_d_n2;
        locals.var_dphi_sb__blk1320_dn4 = assign53180_e81206_d_n4;
        locals.var_dphi_sb__blk1320_dn5 = assign53180_e81206_d_n5;
        locals.var_dphi_sb__blk1320_dn6 = assign53180_e81206_d_n6;
        locals.var_dphi_sb__blk1320_dn7 = assign53180_e81206_d_n7;
        locals.var_dphi_sb__blk1320_dn8 = assign53180_e81206_d_n8;
        locals.var_dphi_sb__blk1320_dn9 = assign53180_e81206_d_n9;
        locals.var_dphi_sb__blk1320_dn10 = assign53180_e81206_d_n10;
        locals.var_dphi_sb__blk1320_dn13 = assign53180_e81206_d_n13;
        locals.var_dphi_sb__blk1320_rv = 0.0;

        let (assign53190_e81227, assign53190_e81227_d_n0, assign53190_e81227_d_n2, assign53190_e81227_d_n4, assign53190_e81227_d_n5, assign53190_e81227_d_n6, assign53190_e81227_d_n7, assign53190_e81227_d_n8, assign53190_e81227_d_n9, assign53190_e81227_d_n10, assign53190_e81227_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign53190_e81221: f64 = (2.0 * locals.var_beta);
        let assign53190_e81223: f64 = (assign53190_e81221 * locals.var_dphi_sb__blk1320);
        let assign53190_e81224: f64 = (assign53190_e81223).sqrt();
        let assign53190_e81225: f64 = (locals.var_wdepsubsl * assign53190_e81224);
        (assign53190_e81225, ((locals.var_wdepsubsl_dn0 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn0)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn2 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn2)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn4 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn4)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn5 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn5)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn6 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn6)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn7 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn7)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn8 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn8)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn9 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn9)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn10 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn10)) / (2.0 * assign53190_e81224)))), ((locals.var_wdepsubsl_dn13 * assign53190_e81224) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb__blk1320) + (assign53190_e81221 * locals.var_dphi_sb__blk1320_dn13)) / (2.0 * assign53190_e81224)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53190_e81227;
        locals.var_t0_dn0 = assign53190_e81227_d_n0;
        locals.var_t0_dn2 = assign53190_e81227_d_n2;
        locals.var_t0_dn4 = assign53190_e81227_d_n4;
        locals.var_t0_dn5 = assign53190_e81227_d_n5;
        locals.var_t0_dn6 = assign53190_e81227_d_n6;
        locals.var_t0_dn7 = assign53190_e81227_d_n7;
        locals.var_t0_dn8 = assign53190_e81227_d_n8;
        locals.var_t0_dn9 = assign53190_e81227_d_n9;
        locals.var_t0_dn10 = assign53190_e81227_d_n10;
        locals.var_t0_dn13 = assign53190_e81227_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign53200_e81248, assign53200_e81248_d_n0, assign53200_e81248_d_n2, assign53200_e81248_d_n4, assign53200_e81248_d_n5, assign53200_e81248_d_n6, assign53200_e81248_d_n7, assign53200_e81248_d_n8, assign53200_e81248_d_n9, assign53200_e81248_d_n10, assign53200_e81248_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign53200_e81240: f64 = (locals.var_t0).exp();
        let assign53200_e81242: f64 = (-locals.var_t0);
        let assign53200_e81243: f64 = (assign53200_e81242).exp();
        let assign53200_e81244: f64 = (assign53200_e81240 + assign53200_e81243);
        let assign53200_e81246: f64 = (assign53200_e81244 / 2.0);
        (assign53200_e81246, (((assign53200_e81240 * locals.var_t0_dn0) + (assign53200_e81243 * (-locals.var_t0_dn0))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn2) + (assign53200_e81243 * (-locals.var_t0_dn2))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn4) + (assign53200_e81243 * (-locals.var_t0_dn4))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn5) + (assign53200_e81243 * (-locals.var_t0_dn5))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn6) + (assign53200_e81243 * (-locals.var_t0_dn6))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn7) + (assign53200_e81243 * (-locals.var_t0_dn7))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn8) + (assign53200_e81243 * (-locals.var_t0_dn8))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn9) + (assign53200_e81243 * (-locals.var_t0_dn9))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn10) + (assign53200_e81243 * (-locals.var_t0_dn10))) / 2.0), (((assign53200_e81240 * locals.var_t0_dn13) + (assign53200_e81243 * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign53200_e81248;
        locals.var_t1_dn0 = assign53200_e81248_d_n0;
        locals.var_t1_dn2 = assign53200_e81248_d_n2;
        locals.var_t1_dn4 = assign53200_e81248_d_n4;
        locals.var_t1_dn5 = assign53200_e81248_d_n5;
        locals.var_t1_dn6 = assign53200_e81248_d_n6;
        locals.var_t1_dn7 = assign53200_e81248_d_n7;
        locals.var_t1_dn8 = assign53200_e81248_d_n8;
        locals.var_t1_dn9 = assign53200_e81248_d_n9;
        locals.var_t1_dn10 = assign53200_e81248_d_n10;
        locals.var_t1_dn13 = assign53200_e81248_d_n13;
        locals.var_t1_rv = 0.0;

        let assign53210_e81250: f64 = (locals.var_t0).abs();
        let assign53210_e81252: f64 = if assign53210_e81250 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1348 = assign53210_e81252;
        locals.var_guard1348_rv = 0.0;

        let (assign53220_e81271, assign53220_e81271_d_n0, assign53220_e81271_d_n2, assign53220_e81271_d_n4, assign53220_e81271_d_n5, assign53220_e81271_d_n6, assign53220_e81271_d_n7, assign53220_e81271_d_n8, assign53220_e81271_d_n9, assign53220_e81271_d_n10, assign53220_e81271_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1348 != 0.0)) {
        let assign53220_e81267: f64 = (locals.var_t1).ln();
        let assign53220_e81269: f64 = (assign53220_e81267 / locals.var_dphi_sb__blk1320);
        (assign53220_e81269, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn0)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn2)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn4)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn5)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn6)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn7)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn8)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn9)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn10)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign53220_e81267 * locals.var_dphi_sb__blk1320_dn13)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)),)
    } else {
        (locals.var_c_sb__blk1321, locals.var_c_sb__blk1321_dn0, locals.var_c_sb__blk1321_dn2, locals.var_c_sb__blk1321_dn4, locals.var_c_sb__blk1321_dn5, locals.var_c_sb__blk1321_dn6, locals.var_c_sb__blk1321_dn7, locals.var_c_sb__blk1321_dn8, locals.var_c_sb__blk1321_dn9, locals.var_c_sb__blk1321_dn10, locals.var_c_sb__blk1321_dn13,)
    }
};
        locals.var_c_sb__blk1321 = assign53220_e81271;
        locals.var_c_sb__blk1321_dn0 = assign53220_e81271_d_n0;
        locals.var_c_sb__blk1321_dn2 = assign53220_e81271_d_n2;
        locals.var_c_sb__blk1321_dn4 = assign53220_e81271_d_n4;
        locals.var_c_sb__blk1321_dn5 = assign53220_e81271_d_n5;
        locals.var_c_sb__blk1321_dn6 = assign53220_e81271_d_n6;
        locals.var_c_sb__blk1321_dn7 = assign53220_e81271_d_n7;
        locals.var_c_sb__blk1321_dn8 = assign53220_e81271_d_n8;
        locals.var_c_sb__blk1321_dn9 = assign53220_e81271_d_n9;
        locals.var_c_sb__blk1321_dn10 = assign53220_e81271_d_n10;
        locals.var_c_sb__blk1321_dn13 = assign53220_e81271_d_n13;
        locals.var_c_sb__blk1321_rv = 0.0;

        let (assign53230_e81300, assign53230_e81300_d_n0, assign53230_e81300_d_n2, assign53230_e81300_d_n4, assign53230_e81300_d_n5, assign53230_e81300_d_n6, assign53230_e81300_d_n7, assign53230_e81300_d_n8, assign53230_e81300_d_n9, assign53230_e81300_d_n10, assign53230_e81300_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1348 == 0.0)) {
        let assign53230_e81288: f64 = (locals.var_wdepsubsl * locals.var_wdepsubsl);
        let assign53230_e81290: f64 = (assign53230_e81288 * locals.var_beta);
        let assign53230_e81294: f64 = (0.1666666666666667 * locals.var_t0);
        let assign53230_e81296: f64 = (assign53230_e81294 * locals.var_t0);
        let assign53230_e81297: f64 = (1.0 - assign53230_e81296);
        let assign53230_e81298: f64 = (assign53230_e81290 * assign53230_e81297);
        (assign53230_e81298, ((((((locals.var_wdepsubsl_dn0 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn0)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn0)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn0) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn0))))), ((((((locals.var_wdepsubsl_dn2 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn2)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn2)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn2) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn2))))), ((((((locals.var_wdepsubsl_dn4 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn4)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn4)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn4) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn4))))), ((((((locals.var_wdepsubsl_dn5 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn5)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn5)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn5) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn5))))), ((((((locals.var_wdepsubsl_dn6 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn6)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn6)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn6) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn6))))), ((((((locals.var_wdepsubsl_dn7 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn7)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn7)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn7) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn7))))), ((((((locals.var_wdepsubsl_dn8 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn8)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn8)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn8) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn8))))), ((((((locals.var_wdepsubsl_dn9 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn9)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn9)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn9) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn9))))), ((((((locals.var_wdepsubsl_dn10 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn10)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn10)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn10) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn10))))), ((((((locals.var_wdepsubsl_dn13 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn13)) * locals.var_beta) + (assign53230_e81288 * locals.var_beta_dn13)) * assign53230_e81297) + (assign53230_e81290 * (-(((0.1666666666666667 * locals.var_t0_dn13) * locals.var_t0) + (assign53230_e81294 * locals.var_t0_dn13))))),)
    } else {
        (locals.var_c_sb__blk1321, locals.var_c_sb__blk1321_dn0, locals.var_c_sb__blk1321_dn2, locals.var_c_sb__blk1321_dn4, locals.var_c_sb__blk1321_dn5, locals.var_c_sb__blk1321_dn6, locals.var_c_sb__blk1321_dn7, locals.var_c_sb__blk1321_dn8, locals.var_c_sb__blk1321_dn9, locals.var_c_sb__blk1321_dn10, locals.var_c_sb__blk1321_dn13,)
    }
};
        locals.var_c_sb__blk1321 = assign53230_e81300;
        locals.var_c_sb__blk1321_dn0 = assign53230_e81300_d_n0;
        locals.var_c_sb__blk1321_dn2 = assign53230_e81300_d_n2;
        locals.var_c_sb__blk1321_dn4 = assign53230_e81300_d_n4;
        locals.var_c_sb__blk1321_dn5 = assign53230_e81300_d_n5;
        locals.var_c_sb__blk1321_dn6 = assign53230_e81300_d_n6;
        locals.var_c_sb__blk1321_dn7 = assign53230_e81300_d_n7;
        locals.var_c_sb__blk1321_dn8 = assign53230_e81300_d_n8;
        locals.var_c_sb__blk1321_dn9 = assign53230_e81300_d_n9;
        locals.var_c_sb__blk1321_dn10 = assign53230_e81300_d_n10;
        locals.var_c_sb__blk1321_dn13 = assign53230_e81300_d_n13;
        locals.var_c_sb__blk1321_rv = 0.0;

        let (assign53240_e81316, assign53240_e81316_d_n0, assign53240_e81316_d_n2, assign53240_e81316_d_n4, assign53240_e81316_d_n5, assign53240_e81316_d_n6, assign53240_e81316_d_n7, assign53240_e81316_d_n8, assign53240_e81316_d_n9, assign53240_e81316_d_n10, assign53240_e81316_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign53240_e81314: f64 = (locals.var_c_sb__blk1321 * locals.var_ps0dep);
        (assign53240_e81314, ((locals.var_c_sb__blk1321_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1321_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1321_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1321_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1321_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1321_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1321_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1321_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1321_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1321_dn13 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign53240_e81316;
        locals.var_tx_dn0 = assign53240_e81316_d_n0;
        locals.var_tx_dn2 = assign53240_e81316_d_n2;
        locals.var_tx_dn4 = assign53240_e81316_d_n4;
        locals.var_tx_dn5 = assign53240_e81316_d_n5;
        locals.var_tx_dn6 = assign53240_e81316_d_n6;
        locals.var_tx_dn7 = assign53240_e81316_d_n7;
        locals.var_tx_dn8 = assign53240_e81316_d_n8;
        locals.var_tx_dn9 = assign53240_e81316_d_n9;
        locals.var_tx_dn10 = assign53240_e81316_d_n10;
        locals.var_tx_dn13 = assign53240_e81316_d_n13;
        locals.var_tx_rv = 0.0;

        let assign53250_e81319: f64 = if locals.var_tx > 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1349 = assign53250_e81319;
        locals.var_guard1349_rv = 0.0;

        let (assign53260_e81337, assign53260_e81337_d_n0, assign53260_e81337_d_n2, assign53260_e81337_d_n4, assign53260_e81337_d_n5, assign53260_e81337_d_n6, assign53260_e81337_d_n7, assign53260_e81337_d_n8, assign53260_e81337_d_n9, assign53260_e81337_d_n10, assign53260_e81337_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 != 0.0)) {
        let assign53260_e81335: f64 = (locals.var_ps0dep - locals.var_dphi_sb__blk1320);
        (assign53260_e81335, (locals.var_ps0dep_dn0 - locals.var_dphi_sb__blk1320_dn0), (locals.var_ps0dep_dn2 - locals.var_dphi_sb__blk1320_dn2), (locals.var_ps0dep_dn4 - locals.var_dphi_sb__blk1320_dn4), (locals.var_ps0dep_dn5 - locals.var_dphi_sb__blk1320_dn5), (locals.var_ps0dep_dn6 - locals.var_dphi_sb__blk1320_dn6), (locals.var_ps0dep_dn7 - locals.var_dphi_sb__blk1320_dn7), (locals.var_ps0dep_dn8 - locals.var_dphi_sb__blk1320_dn8), (locals.var_ps0dep_dn9 - locals.var_dphi_sb__blk1320_dn9), (locals.var_ps0dep_dn10 - locals.var_dphi_sb__blk1320_dn10), (locals.var_ps0dep_dn13 - locals.var_dphi_sb__blk1320_dn13),)
    } else {
        (locals.var_pb0dep__blk1165, locals.var_pb0dep__blk1165_dn0, locals.var_pb0dep__blk1165_dn2, locals.var_pb0dep__blk1165_dn4, locals.var_pb0dep__blk1165_dn5, locals.var_pb0dep__blk1165_dn6, locals.var_pb0dep__blk1165_dn7, locals.var_pb0dep__blk1165_dn8, locals.var_pb0dep__blk1165_dn9, locals.var_pb0dep__blk1165_dn10, locals.var_pb0dep__blk1165_dn13,)
    }
};
        locals.var_pb0dep__blk1165 = assign53260_e81337;
        locals.var_pb0dep__blk1165_dn0 = assign53260_e81337_d_n0;
        locals.var_pb0dep__blk1165_dn2 = assign53260_e81337_d_n2;
        locals.var_pb0dep__blk1165_dn4 = assign53260_e81337_d_n4;
        locals.var_pb0dep__blk1165_dn5 = assign53260_e81337_d_n5;
        locals.var_pb0dep__blk1165_dn6 = assign53260_e81337_d_n6;
        locals.var_pb0dep__blk1165_dn7 = assign53260_e81337_d_n7;
        locals.var_pb0dep__blk1165_dn8 = assign53260_e81337_d_n8;
        locals.var_pb0dep__blk1165_dn9 = assign53260_e81337_d_n9;
        locals.var_pb0dep__blk1165_dn10 = assign53260_e81337_d_n10;
        locals.var_pb0dep__blk1165_dn13 = assign53260_e81337_d_n13;
        locals.var_pb0dep__blk1165_rv = 0.0;

        let (assign53270_e81358, assign53270_e81358_d_n0, assign53270_e81358_d_n2, assign53270_e81358_d_n4, assign53270_e81358_d_n5, assign53270_e81358_d_n6, assign53270_e81358_d_n7, assign53270_e81358_d_n8, assign53270_e81358_d_n9, assign53270_e81358_d_n10, assign53270_e81358_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) {
        let assign53270_e81353: f64 = (-locals.var_c_sb__blk1321);
        let assign53270_e81355: f64 = (assign53270_e81353 * locals.var_dphi_sb__blk1320);
        let assign53270_e81356: f64 = (assign53270_e81355).exp();
        (assign53270_e81356, (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn0) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn0))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn2) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn2))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn4) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn4))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn5) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn5))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn6) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn6))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn7) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn7))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn8) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn8))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn9) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn9))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn10) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn10))), (assign53270_e81356 * (((-locals.var_c_sb__blk1321_dn13) * locals.var_dphi_sb__blk1320) + (assign53270_e81353 * locals.var_dphi_sb__blk1320_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53270_e81358;
        locals.var_t0_dn0 = assign53270_e81358_d_n0;
        locals.var_t0_dn2 = assign53270_e81358_d_n2;
        locals.var_t0_dn4 = assign53270_e81358_d_n4;
        locals.var_t0_dn5 = assign53270_e81358_d_n5;
        locals.var_t0_dn6 = assign53270_e81358_d_n6;
        locals.var_t0_dn7 = assign53270_e81358_d_n7;
        locals.var_t0_dn8 = assign53270_e81358_d_n8;
        locals.var_t0_dn9 = assign53270_e81358_d_n9;
        locals.var_t0_dn10 = assign53270_e81358_d_n10;
        locals.var_t0_dn13 = assign53270_e81358_d_n13;
        locals.var_t0_rv = 0.0;

        let assign53280_e81360: f64 = (locals.var_tx).abs();
        let assign53280_e81362: f64 = if assign53280_e81360 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1350 = assign53280_e81362;
        locals.var_guard1350_rv = 0.0;

        let assign53290_e81365: f64 = if locals.var_tx >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1351 = assign53290_e81365;
        locals.var_guard1351_rv = 0.0;

        let (assign53300_e81392, assign53300_e81392_d_n0, assign53300_e81392_d_n2, assign53300_e81392_d_n4, assign53300_e81392_d_n5, assign53300_e81392_d_n6, assign53300_e81392_d_n7, assign53300_e81392_d_n8, assign53300_e81392_d_n9, assign53300_e81392_d_n10, assign53300_e81392_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 != 0.0)) {
        let assign53300_e81387: f64 = (1.0 + locals.var_tx);
        let assign53300_e81389: f64 = (assign53300_e81387 - 500.0);
        let assign53300_e81390: f64 = (1.403592217853e217 * assign53300_e81389);
        (assign53300_e81390, (1.403592217853e217 * locals.var_tx_dn0), (1.403592217853e217 * locals.var_tx_dn2), (1.403592217853e217 * locals.var_tx_dn4), (1.403592217853e217 * locals.var_tx_dn5), (1.403592217853e217 * locals.var_tx_dn6), (1.403592217853e217 * locals.var_tx_dn7), (1.403592217853e217 * locals.var_tx_dn8), (1.403592217853e217 * locals.var_tx_dn9), (1.403592217853e217 * locals.var_tx_dn10), (1.403592217853e217 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign53300_e81392;
        locals.var_t1_dn0 = assign53300_e81392_d_n0;
        locals.var_t1_dn2 = assign53300_e81392_d_n2;
        locals.var_t1_dn4 = assign53300_e81392_d_n4;
        locals.var_t1_dn5 = assign53300_e81392_d_n5;
        locals.var_t1_dn6 = assign53300_e81392_d_n6;
        locals.var_t1_dn7 = assign53300_e81392_d_n7;
        locals.var_t1_dn8 = assign53300_e81392_d_n8;
        locals.var_t1_dn9 = assign53300_e81392_d_n9;
        locals.var_t1_dn10 = assign53300_e81392_d_n10;
        locals.var_t1_dn13 = assign53300_e81392_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign53310_e81413, assign53310_e81413_d_n0, assign53310_e81413_d_n2, assign53310_e81413_d_n4, assign53310_e81413_d_n5, assign53310_e81413_d_n6, assign53310_e81413_d_n7, assign53310_e81413_d_n8, assign53310_e81413_d_n9, assign53310_e81413_d_n10, assign53310_e81413_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign53310_e81413;
        locals.var_t3_dn0 = assign53310_e81413_d_n0;
        locals.var_t3_dn2 = assign53310_e81413_d_n2;
        locals.var_t3_dn4 = assign53310_e81413_d_n4;
        locals.var_t3_dn5 = assign53310_e81413_d_n5;
        locals.var_t3_dn6 = assign53310_e81413_d_n6;
        locals.var_t3_dn7 = assign53310_e81413_d_n7;
        locals.var_t3_dn8 = assign53310_e81413_d_n8;
        locals.var_t3_dn9 = assign53310_e81413_d_n9;
        locals.var_t3_dn10 = assign53310_e81413_d_n10;
        locals.var_t3_dn13 = assign53310_e81413_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign53320_e81435, assign53320_e81435_d_n0, assign53320_e81435_d_n2, assign53320_e81435_d_n4, assign53320_e81435_d_n5, assign53320_e81435_d_n6, assign53320_e81435_d_n7, assign53320_e81435_d_n8, assign53320_e81435_d_n9, assign53320_e81435_d_n10, assign53320_e81435_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 == 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign53320_e81435;
        locals.var_tmf1_dn0 = assign53320_e81435_d_n0;
        locals.var_tmf1_dn2 = assign53320_e81435_d_n2;
        locals.var_tmf1_dn4 = assign53320_e81435_d_n4;
        locals.var_tmf1_dn5 = assign53320_e81435_d_n5;
        locals.var_tmf1_dn6 = assign53320_e81435_d_n6;
        locals.var_tmf1_dn7 = assign53320_e81435_d_n7;
        locals.var_tmf1_dn8 = assign53320_e81435_d_n8;
        locals.var_tmf1_dn9 = assign53320_e81435_d_n9;
        locals.var_tmf1_dn10 = assign53320_e81435_d_n10;
        locals.var_tmf1_dn13 = assign53320_e81435_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign53330_e81457, assign53330_e81457_d_n0, assign53330_e81457_d_n2, assign53330_e81457_d_n4, assign53330_e81457_d_n5, assign53330_e81457_d_n6, assign53330_e81457_d_n7, assign53330_e81457_d_n8, assign53330_e81457_d_n9, assign53330_e81457_d_n10, assign53330_e81457_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign53330_e81457;
        locals.var_t1_dn0 = assign53330_e81457_d_n0;
        locals.var_t1_dn2 = assign53330_e81457_d_n2;
        locals.var_t1_dn4 = assign53330_e81457_d_n4;
        locals.var_t1_dn5 = assign53330_e81457_d_n5;
        locals.var_t1_dn6 = assign53330_e81457_d_n6;
        locals.var_t1_dn7 = assign53330_e81457_d_n7;
        locals.var_t1_dn8 = assign53330_e81457_d_n8;
        locals.var_t1_dn9 = assign53330_e81457_d_n9;
        locals.var_t1_dn10 = assign53330_e81457_d_n10;
        locals.var_t1_dn13 = assign53330_e81457_d_n13;
        locals.var_t1_rv = 0.0;

        let mut assign53340_loop_guard: usize = 0;
        while {
            let assign53340_cond_e81480: f64 = if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign53340_cond_e81480 != 0.0
        } {
            assign53340_loop_guard += 1;
            assert!(assign53340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53340_body0_e81504, assign53340_body0_e81504_d_n0, assign53340_body0_e81504_d_n2, assign53340_body0_e81504_d_n4, assign53340_body0_e81504_d_n5, assign53340_body0_e81504_d_n6, assign53340_body0_e81504_d_n7, assign53340_body0_e81504_d_n8, assign53340_body0_e81504_d_n9, assign53340_body0_e81504_d_n10, assign53340_body0_e81504_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 == 0.0)) {
        let assign53340_body0_e81502: f64 = (locals.var_t1 * 1.14200738981568e26);
        (assign53340_body0_e81502, (locals.var_t1_dn0 * 1.14200738981568e26), (locals.var_t1_dn2 * 1.14200738981568e26), (locals.var_t1_dn4 * 1.14200738981568e26), (locals.var_t1_dn5 * 1.14200738981568e26), (locals.var_t1_dn6 * 1.14200738981568e26), (locals.var_t1_dn7 * 1.14200738981568e26), (locals.var_t1_dn8 * 1.14200738981568e26), (locals.var_t1_dn9 * 1.14200738981568e26), (locals.var_t1_dn10 * 1.14200738981568e26), (locals.var_t1_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign53340_body0_e81504;
            locals.var_t1_dn0 = assign53340_body0_e81504_d_n0;
            locals.var_t1_dn2 = assign53340_body0_e81504_d_n2;
            locals.var_t1_dn4 = assign53340_body0_e81504_d_n4;
            locals.var_t1_dn5 = assign53340_body0_e81504_d_n5;
            locals.var_t1_dn6 = assign53340_body0_e81504_d_n6;
            locals.var_t1_dn7 = assign53340_body0_e81504_d_n7;
            locals.var_t1_dn8 = assign53340_body0_e81504_d_n8;
            locals.var_t1_dn9 = assign53340_body0_e81504_d_n9;
            locals.var_t1_dn10 = assign53340_body0_e81504_d_n10;
            locals.var_t1_dn13 = assign53340_body0_e81504_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign53340_body1_e81528, assign53340_body1_e81528_d_n0, assign53340_body1_e81528_d_n2, assign53340_body1_e81528_d_n4, assign53340_body1_e81528_d_n5, assign53340_body1_e81528_d_n6, assign53340_body1_e81528_d_n7, assign53340_body1_e81528_d_n8, assign53340_body1_e81528_d_n9, assign53340_body1_e81528_d_n10, assign53340_body1_e81528_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 == 0.0)) {
        let assign53340_body1_e81526: f64 = (locals.var_tmf1 - 60.0);
        (assign53340_body1_e81526, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign53340_body1_e81528;
            locals.var_tmf1_dn0 = assign53340_body1_e81528_d_n0;
            locals.var_tmf1_dn2 = assign53340_body1_e81528_d_n2;
            locals.var_tmf1_dn4 = assign53340_body1_e81528_d_n4;
            locals.var_tmf1_dn5 = assign53340_body1_e81528_d_n5;
            locals.var_tmf1_dn6 = assign53340_body1_e81528_d_n6;
            locals.var_tmf1_dn7 = assign53340_body1_e81528_d_n7;
            locals.var_tmf1_dn8 = assign53340_body1_e81528_d_n8;
            locals.var_tmf1_dn9 = assign53340_body1_e81528_d_n9;
            locals.var_tmf1_dn10 = assign53340_body1_e81528_d_n10;
            locals.var_tmf1_dn13 = assign53340_body1_e81528_d_n13;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign53350_e81553, assign53350_e81553_d_n0, assign53350_e81553_d_n2, assign53350_e81553_d_n4, assign53350_e81553_d_n5, assign53350_e81553_d_n6, assign53350_e81553_d_n7, assign53350_e81553_d_n8, assign53350_e81553_d_n9, assign53350_e81553_d_n10, assign53350_e81553_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 == 0.0)) {
        let assign53350_e81550: f64 = (locals.var_tmf1).exp();
        let assign53350_e81551: f64 = (locals.var_t1 * assign53350_e81550);
        (assign53350_e81551, ((locals.var_t1_dn0 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn0))), ((locals.var_t1_dn2 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn2))), ((locals.var_t1_dn4 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn4))), ((locals.var_t1_dn5 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn5))), ((locals.var_t1_dn6 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn6))), ((locals.var_t1_dn7 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn7))), ((locals.var_t1_dn8 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn8))), ((locals.var_t1_dn9 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn9))), ((locals.var_t1_dn10 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn10))), ((locals.var_t1_dn13 * assign53350_e81550) + (locals.var_t1 * (assign53350_e81550 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign53350_e81553;
        locals.var_t1_dn0 = assign53350_e81553_d_n0;
        locals.var_t1_dn2 = assign53350_e81553_d_n2;
        locals.var_t1_dn4 = assign53350_e81553_d_n4;
        locals.var_t1_dn5 = assign53350_e81553_d_n5;
        locals.var_t1_dn6 = assign53350_e81553_d_n6;
        locals.var_t1_dn7 = assign53350_e81553_d_n7;
        locals.var_t1_dn8 = assign53350_e81553_d_n8;
        locals.var_t1_dn9 = assign53350_e81553_d_n9;
        locals.var_t1_dn10 = assign53350_e81553_d_n10;
        locals.var_t1_dn13 = assign53350_e81553_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign53360_e81575, assign53360_e81575_d_n0, assign53360_e81575_d_n2, assign53360_e81575_d_n4, assign53360_e81575_d_n5, assign53360_e81575_d_n6, assign53360_e81575_d_n7, assign53360_e81575_d_n8, assign53360_e81575_d_n9, assign53360_e81575_d_n10, assign53360_e81575_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign53360_e81575;
        locals.var_t3_dn0 = assign53360_e81575_d_n0;
        locals.var_t3_dn2 = assign53360_e81575_d_n2;
        locals.var_t3_dn4 = assign53360_e81575_d_n4;
        locals.var_t3_dn5 = assign53360_e81575_d_n5;
        locals.var_t3_dn6 = assign53360_e81575_d_n6;
        locals.var_t3_dn7 = assign53360_e81575_d_n7;
        locals.var_t3_dn8 = assign53360_e81575_d_n8;
        locals.var_t3_dn9 = assign53360_e81575_d_n9;
        locals.var_t3_dn10 = assign53360_e81575_d_n10;
        locals.var_t3_dn13 = assign53360_e81575_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign53370_e81596, assign53370_e81596_d_n0, assign53370_e81596_d_n2, assign53370_e81596_d_n4, assign53370_e81596_d_n5, assign53370_e81596_d_n6, assign53370_e81596_d_n7, assign53370_e81596_d_n8, assign53370_e81596_d_n9, assign53370_e81596_d_n10, assign53370_e81596_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) {
        let assign53370_e81594: f64 = (locals.var_t1 * locals.var_t0);
        (assign53370_e81594, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn13 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign53370_e81596;
        locals.var_t1_dn0 = assign53370_e81596_d_n0;
        locals.var_t1_dn2 = assign53370_e81596_d_n2;
        locals.var_t1_dn4 = assign53370_e81596_d_n4;
        locals.var_t1_dn5 = assign53370_e81596_d_n5;
        locals.var_t1_dn6 = assign53370_e81596_d_n6;
        locals.var_t1_dn7 = assign53370_e81596_d_n7;
        locals.var_t1_dn8 = assign53370_e81596_d_n8;
        locals.var_t1_dn9 = assign53370_e81596_d_n9;
        locals.var_t1_dn10 = assign53370_e81596_d_n10;
        locals.var_t1_dn13 = assign53370_e81596_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_186(
        locals: &mut StampLocals,
    ) {
        let (assign53380_e81617, assign53380_e81617_d_n0, assign53380_e81617_d_n2, assign53380_e81617_d_n4, assign53380_e81617_d_n5, assign53380_e81617_d_n6, assign53380_e81617_d_n7, assign53380_e81617_d_n8, assign53380_e81617_d_n9, assign53380_e81617_d_n10, assign53380_e81617_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) {
        let assign53380_e81615: f64 = (locals.var_t1 - locals.var_t0);
        (assign53380_e81615, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign53380_e81617;
        locals.var_t2_dn0 = assign53380_e81617_d_n0;
        locals.var_t2_dn2 = assign53380_e81617_d_n2;
        locals.var_t2_dn4 = assign53380_e81617_d_n4;
        locals.var_t2_dn5 = assign53380_e81617_d_n5;
        locals.var_t2_dn6 = assign53380_e81617_d_n6;
        locals.var_t2_dn7 = assign53380_e81617_d_n7;
        locals.var_t2_dn8 = assign53380_e81617_d_n8;
        locals.var_t2_dn9 = assign53380_e81617_d_n9;
        locals.var_t2_dn10 = assign53380_e81617_d_n10;
        locals.var_t2_dn13 = assign53380_e81617_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign53390_e81641, assign53390_e81641_d_n0, assign53390_e81641_d_n2, assign53390_e81641_d_n4, assign53390_e81641_d_n5, assign53390_e81641_d_n6, assign53390_e81641_d_n7, assign53390_e81641_d_n8, assign53390_e81641_d_n9, assign53390_e81641_d_n10, assign53390_e81641_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 == 0.0)) {
        let assign53390_e81637: f64 = (1.0 + locals.var_tx);
        let assign53390_e81639: f64 = (assign53390_e81637 * locals.var_t0);
        (assign53390_e81639, ((locals.var_tx_dn0 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn10)), ((locals.var_tx_dn13 * locals.var_t0) + (assign53390_e81637 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign53390_e81641;
        locals.var_t1_dn0 = assign53390_e81641_d_n0;
        locals.var_t1_dn2 = assign53390_e81641_d_n2;
        locals.var_t1_dn4 = assign53390_e81641_d_n4;
        locals.var_t1_dn5 = assign53390_e81641_d_n5;
        locals.var_t1_dn6 = assign53390_e81641_d_n6;
        locals.var_t1_dn7 = assign53390_e81641_d_n7;
        locals.var_t1_dn8 = assign53390_e81641_d_n8;
        locals.var_t1_dn9 = assign53390_e81641_d_n9;
        locals.var_t1_dn10 = assign53390_e81641_d_n10;
        locals.var_t1_dn13 = assign53390_e81641_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign53400_e81669, assign53400_e81669_d_n0, assign53400_e81669_d_n2, assign53400_e81669_d_n4, assign53400_e81669_d_n5, assign53400_e81669_d_n6, assign53400_e81669_d_n7, assign53400_e81669_d_n8, assign53400_e81669_d_n9, assign53400_e81669_d_n10, assign53400_e81669_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 == 0.0)) {
        let assign53400_e81663: f64 = (locals.var_tx / 2.0);
        let assign53400_e81664: f64 = (1.0 + assign53400_e81663);
        let assign53400_e81665: f64 = (locals.var_tx * assign53400_e81664);
        let assign53400_e81667: f64 = (assign53400_e81665 * locals.var_t0);
        (assign53400_e81667, ((((locals.var_tx_dn0 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn10)), ((((locals.var_tx_dn13 * assign53400_e81664) + (locals.var_tx * (locals.var_tx_dn13 / 2.0))) * locals.var_t0) + (assign53400_e81665 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign53400_e81669;
        locals.var_t2_dn0 = assign53400_e81669_d_n0;
        locals.var_t2_dn2 = assign53400_e81669_d_n2;
        locals.var_t2_dn4 = assign53400_e81669_d_n4;
        locals.var_t2_dn5 = assign53400_e81669_d_n5;
        locals.var_t2_dn6 = assign53400_e81669_d_n6;
        locals.var_t2_dn7 = assign53400_e81669_d_n7;
        locals.var_t2_dn8 = assign53400_e81669_d_n8;
        locals.var_t2_dn9 = assign53400_e81669_d_n9;
        locals.var_t2_dn10 = assign53400_e81669_d_n10;
        locals.var_t2_dn13 = assign53400_e81669_d_n13;
        locals.var_t2_rv = 0.0;

        let assign53410_e81671: f64 = (locals.var_t2).abs();
        let assign53410_e81673: f64 = if assign53410_e81671 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1352 = assign53410_e81673;
        locals.var_guard1352_rv = 0.0;

        let (assign53420_e81697, assign53420_e81697_d_n0, assign53420_e81697_d_n2, assign53420_e81697_d_n4, assign53420_e81697_d_n5, assign53420_e81697_d_n6, assign53420_e81697_d_n7, assign53420_e81697_d_n8, assign53420_e81697_d_n9, assign53420_e81697_d_n10, assign53420_e81697_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1352 != 0.0)) {
        let assign53420_e81692: f64 = (1.0 + locals.var_t2);
        let assign53420_e81693: f64 = (assign53420_e81692).ln();
        let assign53420_e81695: f64 = (assign53420_e81693 / locals.var_c_sb__blk1321);
        (assign53420_e81695, ((((locals.var_t2_dn0 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn0)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn2 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn2)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn4 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn4)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn5 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn5)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn6 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn6)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn7 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn7)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn8 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn8)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn9 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn9)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn10 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn10)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn13 / assign53420_e81692) * locals.var_c_sb__blk1321) - (assign53420_e81693 * locals.var_c_sb__blk1321_dn13)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)),)
    } else {
        (locals.var_pb0dep__blk1165, locals.var_pb0dep__blk1165_dn0, locals.var_pb0dep__blk1165_dn2, locals.var_pb0dep__blk1165_dn4, locals.var_pb0dep__blk1165_dn5, locals.var_pb0dep__blk1165_dn6, locals.var_pb0dep__blk1165_dn7, locals.var_pb0dep__blk1165_dn8, locals.var_pb0dep__blk1165_dn9, locals.var_pb0dep__blk1165_dn10, locals.var_pb0dep__blk1165_dn13,)
    }
};
        locals.var_pb0dep__blk1165 = assign53420_e81697;
        locals.var_pb0dep__blk1165_dn0 = assign53420_e81697_d_n0;
        locals.var_pb0dep__blk1165_dn2 = assign53420_e81697_d_n2;
        locals.var_pb0dep__blk1165_dn4 = assign53420_e81697_d_n4;
        locals.var_pb0dep__blk1165_dn5 = assign53420_e81697_d_n5;
        locals.var_pb0dep__blk1165_dn6 = assign53420_e81697_d_n6;
        locals.var_pb0dep__blk1165_dn7 = assign53420_e81697_d_n7;
        locals.var_pb0dep__blk1165_dn8 = assign53420_e81697_d_n8;
        locals.var_pb0dep__blk1165_dn9 = assign53420_e81697_d_n9;
        locals.var_pb0dep__blk1165_dn10 = assign53420_e81697_d_n10;
        locals.var_pb0dep__blk1165_dn13 = assign53420_e81697_d_n13;
        locals.var_pb0dep__blk1165_rv = 0.0;

        let (assign53430_e81719, assign53430_e81719_d_n0, assign53430_e81719_d_n2, assign53430_e81719_d_n4, assign53430_e81719_d_n5, assign53430_e81719_d_n6, assign53430_e81719_d_n7, assign53430_e81719_d_n8, assign53430_e81719_d_n9, assign53430_e81719_d_n10, assign53430_e81719_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1352 == 0.0)) {
        let assign53430_e81717: f64 = (locals.var_t2 / locals.var_c_sb__blk1321);
        (assign53430_e81717, (((locals.var_t2_dn0 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn0)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn2)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn4)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn5)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn6)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn7)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn8)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn9)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn10)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn13 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn13)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)),)
    } else {
        (locals.var_pb0dep__blk1165, locals.var_pb0dep__blk1165_dn0, locals.var_pb0dep__blk1165_dn2, locals.var_pb0dep__blk1165_dn4, locals.var_pb0dep__blk1165_dn5, locals.var_pb0dep__blk1165_dn6, locals.var_pb0dep__blk1165_dn7, locals.var_pb0dep__blk1165_dn8, locals.var_pb0dep__blk1165_dn9, locals.var_pb0dep__blk1165_dn10, locals.var_pb0dep__blk1165_dn13,)
    }
};
        locals.var_pb0dep__blk1165 = assign53430_e81719;
        locals.var_pb0dep__blk1165_dn0 = assign53430_e81719_d_n0;
        locals.var_pb0dep__blk1165_dn2 = assign53430_e81719_d_n2;
        locals.var_pb0dep__blk1165_dn4 = assign53430_e81719_d_n4;
        locals.var_pb0dep__blk1165_dn5 = assign53430_e81719_d_n5;
        locals.var_pb0dep__blk1165_dn6 = assign53430_e81719_d_n6;
        locals.var_pb0dep__blk1165_dn7 = assign53430_e81719_d_n7;
        locals.var_pb0dep__blk1165_dn8 = assign53430_e81719_d_n8;
        locals.var_pb0dep__blk1165_dn9 = assign53430_e81719_d_n9;
        locals.var_pb0dep__blk1165_dn10 = assign53430_e81719_d_n10;
        locals.var_pb0dep__blk1165_dn13 = assign53430_e81719_d_n13;
        locals.var_pb0dep__blk1165_rv = 0.0;

        let (assign53440_e81735, assign53440_e81735_d_n0, assign53440_e81735_d_n2, assign53440_e81735_d_n4, assign53440_e81735_d_n5, assign53440_e81735_d_n6, assign53440_e81735_d_n7, assign53440_e81735_d_n8, assign53440_e81735_d_n9, assign53440_e81735_d_n10, assign53440_e81735_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign53440_e81733: f64 = (locals.var_ps0dep - locals.var_pb0dep__blk1165);
        (assign53440_e81733, (locals.var_ps0dep_dn0 - locals.var_pb0dep__blk1165_dn0), (locals.var_ps0dep_dn2 - locals.var_pb0dep__blk1165_dn2), (locals.var_ps0dep_dn4 - locals.var_pb0dep__blk1165_dn4), (locals.var_ps0dep_dn5 - locals.var_pb0dep__blk1165_dn5), (locals.var_ps0dep_dn6 - locals.var_pb0dep__blk1165_dn6), (locals.var_ps0dep_dn7 - locals.var_pb0dep__blk1165_dn7), (locals.var_ps0dep_dn8 - locals.var_pb0dep__blk1165_dn8), (locals.var_ps0dep_dn9 - locals.var_pb0dep__blk1165_dn9), (locals.var_ps0dep_dn10 - locals.var_pb0dep__blk1165_dn10), (locals.var_ps0dep_dn13 - locals.var_pb0dep__blk1165_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign53440_e81735;
        locals.var_t2_dn0 = assign53440_e81735_d_n0;
        locals.var_t2_dn2 = assign53440_e81735_d_n2;
        locals.var_t2_dn4 = assign53440_e81735_d_n4;
        locals.var_t2_dn5 = assign53440_e81735_d_n5;
        locals.var_t2_dn6 = assign53440_e81735_d_n6;
        locals.var_t2_dn7 = assign53440_e81735_d_n7;
        locals.var_t2_dn8 = assign53440_e81735_d_n8;
        locals.var_t2_dn9 = assign53440_e81735_d_n9;
        locals.var_t2_dn10 = assign53440_e81735_d_n10;
        locals.var_t2_dn13 = assign53440_e81735_d_n13;
        locals.var_t2_rv = 0.0;

        let assign53450_e81738: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1353 = assign53450_e81738;
        locals.var_guard1353_rv = 0.0;

        let (assign53460_e81767, assign53460_e81767_d_n0, assign53460_e81767_d_n2, assign53460_e81767_d_n4, assign53460_e81767_d_n5, assign53460_e81767_d_n6, assign53460_e81767_d_n7, assign53460_e81767_d_n8, assign53460_e81767_d_n9, assign53460_e81767_d_n10, assign53460_e81767_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1353 != 0.0)) {
        let (assign53460_e81765, assign53460_e81765_d_n0, assign53460_e81765_d_n2, assign53460_e81765_d_n4, assign53460_e81765_d_n5, assign53460_e81765_d_n6, assign53460_e81765_d_n7, assign53460_e81765_d_n8, assign53460_e81765_d_n9, assign53460_e81765_d_n10, assign53460_e81765_d_n13,) = {
            if (locals.var_t2 < 0.0) {
                let assign53460_e81756: f64 = (-locals.var_c_2esipq_ndepm__blk1136);
                let assign53460_e81758: f64 = (assign53460_e81756 * locals.var_t2);
                let assign53460_e81759: f64 = (assign53460_e81758).sqrt();
                let assign53460_e81760: f64 = (-assign53460_e81759);
                (assign53460_e81760, (-((((-locals.var_c_2esipq_ndepm__blk1136_dn0) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn0)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn2) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn2)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn4) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn4)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn5) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn5)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn6) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn6)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn7) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn7)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn8) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn8)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn9) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn9)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn10) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn10)) / (2.0 * assign53460_e81759))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn13) * locals.var_t2) + (assign53460_e81756 * locals.var_t2_dn13)) / (2.0 * assign53460_e81759))),)
            } else {
                let assign53460_e81763: f64 = (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2);
                let assign53460_e81764: f64 = (assign53460_e81763).sqrt();
                (assign53460_e81764, (((locals.var_c_2esipq_ndepm__blk1136_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn0)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn2)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn4)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn5)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn6)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn7)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn8)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn9)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn10)) / (2.0 * assign53460_e81764)), (((locals.var_c_2esipq_ndepm__blk1136_dn13 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn13)) / (2.0 * assign53460_e81764)),)
            }
        };
        (assign53460_e81765, assign53460_e81765_d_n0, assign53460_e81765_d_n2, assign53460_e81765_d_n4, assign53460_e81765_d_n5, assign53460_e81765_d_n6, assign53460_e81765_d_n7, assign53460_e81765_d_n8, assign53460_e81765_d_n9, assign53460_e81765_d_n10, assign53460_e81765_d_n13,)
    } else {
        (locals.var_ws__blk1147, locals.var_ws__blk1147_dn0, locals.var_ws__blk1147_dn2, locals.var_ws__blk1147_dn4, locals.var_ws__blk1147_dn5, locals.var_ws__blk1147_dn6, locals.var_ws__blk1147_dn7, locals.var_ws__blk1147_dn8, locals.var_ws__blk1147_dn9, locals.var_ws__blk1147_dn10, locals.var_ws__blk1147_dn13,)
    }
};
        locals.var_ws__blk1147 = assign53460_e81767;
        locals.var_ws__blk1147_dn0 = assign53460_e81767_d_n0;
        locals.var_ws__blk1147_dn2 = assign53460_e81767_d_n2;
        locals.var_ws__blk1147_dn4 = assign53460_e81767_d_n4;
        locals.var_ws__blk1147_dn5 = assign53460_e81767_d_n5;
        locals.var_ws__blk1147_dn6 = assign53460_e81767_d_n6;
        locals.var_ws__blk1147_dn7 = assign53460_e81767_d_n7;
        locals.var_ws__blk1147_dn8 = assign53460_e81767_d_n8;
        locals.var_ws__blk1147_dn9 = assign53460_e81767_d_n9;
        locals.var_ws__blk1147_dn10 = assign53460_e81767_d_n10;
        locals.var_ws__blk1147_dn13 = assign53460_e81767_d_n13;
        locals.var_ws__blk1147_rv = 0.0;

        let assign53470_e81770: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1354 = assign53470_e81770;
        locals.var_guard1354_rv = 0.0;

        let (assign53480_e81791, assign53480_e81791_d_n0, assign53480_e81791_d_n2, assign53480_e81791_d_n4, assign53480_e81791_d_n5, assign53480_e81791_d_n6, assign53480_e81791_d_n7, assign53480_e81791_d_n8, assign53480_e81791_d_n9, assign53480_e81791_d_n10, assign53480_e81791_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) {
        let assign53480_e81789: f64 = (locals.var_beta * locals.var_t2);
        (assign53480_e81789, ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)), ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)), ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)), ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)), ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)), ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)), ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)), ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)), ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)), ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign53480_e81791;
        locals.var_t3_dn0 = assign53480_e81791_d_n0;
        locals.var_t3_dn2 = assign53480_e81791_d_n2;
        locals.var_t3_dn4 = assign53480_e81791_d_n4;
        locals.var_t3_dn5 = assign53480_e81791_d_n5;
        locals.var_t3_dn6 = assign53480_e81791_d_n6;
        locals.var_t3_dn7 = assign53480_e81791_d_n7;
        locals.var_t3_dn8 = assign53480_e81791_d_n8;
        locals.var_t3_dn9 = assign53480_e81791_d_n9;
        locals.var_t3_dn10 = assign53480_e81791_d_n10;
        locals.var_t3_dn13 = assign53480_e81791_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign53490_e81821, assign53490_e81821_d_n0, assign53490_e81821_d_n2, assign53490_e81821_d_n4, assign53490_e81821_d_n5, assign53490_e81821_d_n6, assign53490_e81821_d_n7, assign53490_e81821_d_n8, assign53490_e81821_d_n9, assign53490_e81821_d_n10, assign53490_e81821_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) {
        let assign53490_e81810: f64 = (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv);
        let assign53490_e81812: f64 = (locals.var_t3).exp();
        let assign53490_e81814: f64 = (assign53490_e81812 - locals.var_t3);
        let assign53490_e81816: f64 = (assign53490_e81814 - 1.0);
        let assign53490_e81817: f64 = (assign53490_e81810 * assign53490_e81816);
        let assign53490_e81818: f64 = (assign53490_e81817).sqrt();
        let assign53490_e81819: f64 = (-assign53490_e81818);
        (assign53490_e81819, (-(((((locals.var_c_2esipq_ndepm__blk1136_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn0)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn2)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn4)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn5)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn6)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn7)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn8)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn9)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn10)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign53490_e81818))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn13 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn13)) * assign53490_e81816) + (assign53490_e81810 * ((assign53490_e81812 * locals.var_t3_dn13) - locals.var_t3_dn13))) / (2.0 * assign53490_e81818))),)
    } else {
        (locals.var_ws__blk1147, locals.var_ws__blk1147_dn0, locals.var_ws__blk1147_dn2, locals.var_ws__blk1147_dn4, locals.var_ws__blk1147_dn5, locals.var_ws__blk1147_dn6, locals.var_ws__blk1147_dn7, locals.var_ws__blk1147_dn8, locals.var_ws__blk1147_dn9, locals.var_ws__blk1147_dn10, locals.var_ws__blk1147_dn13,)
    }
};
        locals.var_ws__blk1147 = assign53490_e81821;
        locals.var_ws__blk1147_dn0 = assign53490_e81821_d_n0;
        locals.var_ws__blk1147_dn2 = assign53490_e81821_d_n2;
        locals.var_ws__blk1147_dn4 = assign53490_e81821_d_n4;
        locals.var_ws__blk1147_dn5 = assign53490_e81821_d_n5;
        locals.var_ws__blk1147_dn6 = assign53490_e81821_d_n6;
        locals.var_ws__blk1147_dn7 = assign53490_e81821_d_n7;
        locals.var_ws__blk1147_dn8 = assign53490_e81821_d_n8;
        locals.var_ws__blk1147_dn9 = assign53490_e81821_d_n9;
        locals.var_ws__blk1147_dn10 = assign53490_e81821_d_n10;
        locals.var_ws__blk1147_dn13 = assign53490_e81821_d_n13;
        locals.var_ws__blk1147_rv = 0.0;

        let (assign53500_e81844, assign53500_e81844_d_n0, assign53500_e81844_d_n2, assign53500_e81844_d_n4, assign53500_e81844_d_n5, assign53500_e81844_d_n6, assign53500_e81844_d_n7, assign53500_e81844_d_n8, assign53500_e81844_d_n9, assign53500_e81844_d_n10, assign53500_e81844_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 == 0.0)) {
        let assign53500_e81840: f64 = (-locals.var_beta);
        let assign53500_e81842: f64 = (assign53500_e81840 * locals.var_t2);
        (assign53500_e81842, (((-locals.var_beta_dn0) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn0)), (((-locals.var_beta_dn2) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn2)), (((-locals.var_beta_dn4) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn4)), (((-locals.var_beta_dn5) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn5)), (((-locals.var_beta_dn6) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn6)), (((-locals.var_beta_dn7) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn7)), (((-locals.var_beta_dn8) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn8)), (((-locals.var_beta_dn9) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn9)), (((-locals.var_beta_dn10) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn10)), (((-locals.var_beta_dn13) * locals.var_t2) + (assign53500_e81840 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign53500_e81844;
        locals.var_t3_dn0 = assign53500_e81844_d_n0;
        locals.var_t3_dn2 = assign53500_e81844_d_n2;
        locals.var_t3_dn4 = assign53500_e81844_d_n4;
        locals.var_t3_dn5 = assign53500_e81844_d_n5;
        locals.var_t3_dn6 = assign53500_e81844_d_n6;
        locals.var_t3_dn7 = assign53500_e81844_d_n7;
        locals.var_t3_dn8 = assign53500_e81844_d_n8;
        locals.var_t3_dn9 = assign53500_e81844_d_n9;
        locals.var_t3_dn10 = assign53500_e81844_d_n10;
        locals.var_t3_dn13 = assign53500_e81844_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign53510_e81874, assign53510_e81874_d_n0, assign53510_e81874_d_n2, assign53510_e81874_d_n4, assign53510_e81874_d_n5, assign53510_e81874_d_n6, assign53510_e81874_d_n7, assign53510_e81874_d_n8, assign53510_e81874_d_n9, assign53510_e81874_d_n10, assign53510_e81874_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 == 0.0)) {
        let assign53510_e81864: f64 = (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv);
        let assign53510_e81866: f64 = (locals.var_t3).exp();
        let assign53510_e81868: f64 = (assign53510_e81866 - locals.var_t3);
        let assign53510_e81870: f64 = (assign53510_e81868 - 1.0);
        let assign53510_e81871: f64 = (assign53510_e81864 * assign53510_e81870);
        let assign53510_e81872: f64 = (assign53510_e81871).sqrt();
        (assign53510_e81872, (((((locals.var_c_2esipq_ndepm__blk1136_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn0)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn2)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn4)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn5)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn6)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn7)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn8)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn9)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn10)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign53510_e81872)), (((((locals.var_c_2esipq_ndepm__blk1136_dn13 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn13)) * assign53510_e81870) + (assign53510_e81864 * ((assign53510_e81866 * locals.var_t3_dn13) - locals.var_t3_dn13))) / (2.0 * assign53510_e81872)),)
    } else {
        (locals.var_ws__blk1147, locals.var_ws__blk1147_dn0, locals.var_ws__blk1147_dn2, locals.var_ws__blk1147_dn4, locals.var_ws__blk1147_dn5, locals.var_ws__blk1147_dn6, locals.var_ws__blk1147_dn7, locals.var_ws__blk1147_dn8, locals.var_ws__blk1147_dn9, locals.var_ws__blk1147_dn10, locals.var_ws__blk1147_dn13,)
    }
};
        locals.var_ws__blk1147 = assign53510_e81874;
        locals.var_ws__blk1147_dn0 = assign53510_e81874_d_n0;
        locals.var_ws__blk1147_dn2 = assign53510_e81874_d_n2;
        locals.var_ws__blk1147_dn4 = assign53510_e81874_d_n4;
        locals.var_ws__blk1147_dn5 = assign53510_e81874_d_n5;
        locals.var_ws__blk1147_dn6 = assign53510_e81874_d_n6;
        locals.var_ws__blk1147_dn7 = assign53510_e81874_d_n7;
        locals.var_ws__blk1147_dn8 = assign53510_e81874_d_n8;
        locals.var_ws__blk1147_dn9 = assign53510_e81874_d_n9;
        locals.var_ws__blk1147_dn10 = assign53510_e81874_d_n10;
        locals.var_ws__blk1147_dn13 = assign53510_e81874_d_n13;
        locals.var_ws__blk1147_rv = 0.0;

        let (assign53520_e81890, assign53520_e81890_d_n0, assign53520_e81890_d_n2, assign53520_e81890_d_n4, assign53520_e81890_d_n5, assign53520_e81890_d_n6, assign53520_e81890_d_n7, assign53520_e81890_d_n8, assign53520_e81890_d_n9, assign53520_e81890_d_n10, assign53520_e81890_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign53520_e81888: f64 = (locals.var_tnp__blk1148 - locals.var_ws__blk1147);
        (assign53520_e81888, (locals.var_tnp__blk1148_dn0 - locals.var_ws__blk1147_dn0), (locals.var_tnp__blk1148_dn2 - locals.var_ws__blk1147_dn2), (locals.var_tnp__blk1148_dn4 - locals.var_ws__blk1147_dn4), (locals.var_tnp__blk1148_dn5 - locals.var_ws__blk1147_dn5), (locals.var_tnp__blk1148_dn6 - locals.var_ws__blk1147_dn6), (locals.var_tnp__blk1148_dn7 - locals.var_ws__blk1147_dn7), (locals.var_tnp__blk1148_dn8 - locals.var_ws__blk1147_dn8), (locals.var_tnp__blk1148_dn9 - locals.var_ws__blk1147_dn9), (locals.var_tnp__blk1148_dn10 - locals.var_ws__blk1147_dn10), (locals.var_tnp__blk1148_dn13 - locals.var_ws__blk1147_dn13),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign53520_e81890;
        locals.var_w_res_dn0 = assign53520_e81890_d_n0;
        locals.var_w_res_dn2 = assign53520_e81890_d_n2;
        locals.var_w_res_dn4 = assign53520_e81890_d_n4;
        locals.var_w_res_dn5 = assign53520_e81890_d_n5;
        locals.var_w_res_dn6 = assign53520_e81890_d_n6;
        locals.var_w_res_dn7 = assign53520_e81890_d_n7;
        locals.var_w_res_dn8 = assign53520_e81890_d_n8;
        locals.var_w_res_dn9 = assign53520_e81890_d_n9;
        locals.var_w_res_dn10 = assign53520_e81890_d_n10;
        locals.var_w_res_dn13 = assign53520_e81890_d_n13;
        locals.var_w_res_rv = 0.0;

        let assign53530_e81894: f64 = 1e-16;
        let assign53530_e81899: f64 = if ((locals.var_w_res < assign53530_e81894) && (1e-16 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1355 = assign53530_e81899;
        locals.var_guard1355_rv = 0.0;

        let (assign53540_e81919, assign53540_e81919_d_n0, assign53540_e81919_d_n2, assign53540_e81919_d_n4, assign53540_e81919_d_n5, assign53540_e81919_d_n6, assign53540_e81919_d_n7, assign53540_e81919_d_n8, assign53540_e81919_d_n9, assign53540_e81919_d_n10, assign53540_e81919_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53540_e81915: f64 = 1e-16;
        let assign53540_e81917: f64 = (assign53540_e81915 - locals.var_w_res);
        (assign53540_e81917, (-locals.var_w_res_dn0), (-locals.var_w_res_dn2), (-locals.var_w_res_dn4), (-locals.var_w_res_dn5), (-locals.var_w_res_dn6), (-locals.var_w_res_dn7), (-locals.var_w_res_dn8), (-locals.var_w_res_dn9), (-locals.var_w_res_dn10), (-locals.var_w_res_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign53540_e81919;
        locals.var_tmf1_dn0 = assign53540_e81919_d_n0;
        locals.var_tmf1_dn2 = assign53540_e81919_d_n2;
        locals.var_tmf1_dn4 = assign53540_e81919_d_n4;
        locals.var_tmf1_dn5 = assign53540_e81919_d_n5;
        locals.var_tmf1_dn6 = assign53540_e81919_d_n6;
        locals.var_tmf1_dn7 = assign53540_e81919_d_n7;
        locals.var_tmf1_dn8 = assign53540_e81919_d_n8;
        locals.var_tmf1_dn9 = assign53540_e81919_d_n9;
        locals.var_tmf1_dn10 = assign53540_e81919_d_n10;
        locals.var_tmf1_dn13 = assign53540_e81919_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign53550_e81937, assign53550_e81937_d_n0, assign53550_e81937_d_n2, assign53550_e81937_d_n4, assign53550_e81937_d_n5, assign53550_e81937_d_n6, assign53550_e81937_d_n7, assign53550_e81937_d_n8, assign53550_e81937_d_n9, assign53550_e81937_d_n10, assign53550_e81937_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53550_e81935: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign53550_e81935, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign53550_e81937;
        locals.var_x2_dn0 = assign53550_e81937_d_n0;
        locals.var_x2_dn2 = assign53550_e81937_d_n2;
        locals.var_x2_dn4 = assign53550_e81937_d_n4;
        locals.var_x2_dn5 = assign53550_e81937_d_n5;
        locals.var_x2_dn6 = assign53550_e81937_d_n6;
        locals.var_x2_dn7 = assign53550_e81937_d_n7;
        locals.var_x2_dn8 = assign53550_e81937_d_n8;
        locals.var_x2_dn9 = assign53550_e81937_d_n9;
        locals.var_x2_dn10 = assign53550_e81937_d_n10;
        locals.var_x2_dn13 = assign53550_e81937_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign53560_e81955, assign53560_e81955_d_n0, assign53560_e81955_d_n2, assign53560_e81955_d_n4, assign53560_e81955_d_n5, assign53560_e81955_d_n6, assign53560_e81955_d_n7, assign53560_e81955_d_n8, assign53560_e81955_d_n9, assign53560_e81955_d_n10, assign53560_e81955_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53560_e81953: f64 = (1e-16 * 1e-16);
        (assign53560_e81953, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign53560_e81955;
        locals.var_xmax2_dn0 = assign53560_e81955_d_n0;
        locals.var_xmax2_dn2 = assign53560_e81955_d_n2;
        locals.var_xmax2_dn4 = assign53560_e81955_d_n4;
        locals.var_xmax2_dn5 = assign53560_e81955_d_n5;
        locals.var_xmax2_dn6 = assign53560_e81955_d_n6;
        locals.var_xmax2_dn7 = assign53560_e81955_d_n7;
        locals.var_xmax2_dn8 = assign53560_e81955_d_n8;
        locals.var_xmax2_dn9 = assign53560_e81955_d_n9;
        locals.var_xmax2_dn10 = assign53560_e81955_d_n10;
        locals.var_xmax2_dn13 = assign53560_e81955_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign53570_e81971, assign53570_e81971_d_n0, assign53570_e81971_d_n2, assign53570_e81971_d_n4, assign53570_e81971_d_n5, assign53570_e81971_d_n6, assign53570_e81971_d_n7, assign53570_e81971_d_n8, assign53570_e81971_d_n9, assign53570_e81971_d_n10, assign53570_e81971_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign53570_e81971;
        locals.var_xp_dn0 = assign53570_e81971_d_n0;
        locals.var_xp_dn2 = assign53570_e81971_d_n2;
        locals.var_xp_dn4 = assign53570_e81971_d_n4;
        locals.var_xp_dn5 = assign53570_e81971_d_n5;
        locals.var_xp_dn6 = assign53570_e81971_d_n6;
        locals.var_xp_dn7 = assign53570_e81971_d_n7;
        locals.var_xp_dn8 = assign53570_e81971_d_n8;
        locals.var_xp_dn9 = assign53570_e81971_d_n9;
        locals.var_xp_dn10 = assign53570_e81971_d_n10;
        locals.var_xp_dn13 = assign53570_e81971_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign53580_e81987, assign53580_e81987_d_n0, assign53580_e81987_d_n2, assign53580_e81987_d_n4, assign53580_e81987_d_n5, assign53580_e81987_d_n6, assign53580_e81987_d_n7, assign53580_e81987_d_n8, assign53580_e81987_d_n9, assign53580_e81987_d_n10, assign53580_e81987_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign53580_e81987;
        locals.var_xmp_dn0 = assign53580_e81987_d_n0;
        locals.var_xmp_dn2 = assign53580_e81987_d_n2;
        locals.var_xmp_dn4 = assign53580_e81987_d_n4;
        locals.var_xmp_dn5 = assign53580_e81987_d_n5;
        locals.var_xmp_dn6 = assign53580_e81987_d_n6;
        locals.var_xmp_dn7 = assign53580_e81987_d_n7;
        locals.var_xmp_dn8 = assign53580_e81987_d_n8;
        locals.var_xmp_dn9 = assign53580_e81987_d_n9;
        locals.var_xmp_dn10 = assign53580_e81987_d_n10;
        locals.var_xmp_dn13 = assign53580_e81987_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign53590_e82003,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53590_e82003;
        locals.var_m0_rv = 0.0;

        let (assign53600_e82019,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53600_e82019;
        locals.var_mm_rv = 0.0;

        let (assign53610_e82035, assign53610_e82035_d_n0, assign53610_e82035_d_n2, assign53610_e82035_d_n4, assign53610_e82035_d_n5, assign53610_e82035_d_n6, assign53610_e82035_d_n7, assign53610_e82035_d_n8, assign53610_e82035_d_n9, assign53610_e82035_d_n10, assign53610_e82035_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign53610_e82035;
        locals.var_arg_dn0 = assign53610_e82035_d_n0;
        locals.var_arg_dn2 = assign53610_e82035_d_n2;
        locals.var_arg_dn4 = assign53610_e82035_d_n4;
        locals.var_arg_dn5 = assign53610_e82035_d_n5;
        locals.var_arg_dn6 = assign53610_e82035_d_n6;
        locals.var_arg_dn7 = assign53610_e82035_d_n7;
        locals.var_arg_dn8 = assign53610_e82035_d_n8;
        locals.var_arg_dn9 = assign53610_e82035_d_n9;
        locals.var_arg_dn10 = assign53610_e82035_d_n10;
        locals.var_arg_dn13 = assign53610_e82035_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign53620_e82051, assign53620_e82051_d_n0, assign53620_e82051_d_n2, assign53620_e82051_d_n4, assign53620_e82051_d_n5, assign53620_e82051_d_n6, assign53620_e82051_d_n7, assign53620_e82051_d_n8, assign53620_e82051_d_n9, assign53620_e82051_d_n10, assign53620_e82051_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign53620_e82051;
        locals.var_dnm_dn0 = assign53620_e82051_d_n0;
        locals.var_dnm_dn2 = assign53620_e82051_d_n2;
        locals.var_dnm_dn4 = assign53620_e82051_d_n4;
        locals.var_dnm_dn5 = assign53620_e82051_d_n5;
        locals.var_dnm_dn6 = assign53620_e82051_d_n6;
        locals.var_dnm_dn7 = assign53620_e82051_d_n7;
        locals.var_dnm_dn8 = assign53620_e82051_d_n8;
        locals.var_dnm_dn9 = assign53620_e82051_d_n9;
        locals.var_dnm_dn10 = assign53620_e82051_d_n10;
        locals.var_dnm_dn13 = assign53620_e82051_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign53630_e82069, assign53630_e82069_d_n0, assign53630_e82069_d_n2, assign53630_e82069_d_n4, assign53630_e82069_d_n5, assign53630_e82069_d_n6, assign53630_e82069_d_n7, assign53630_e82069_d_n8, assign53630_e82069_d_n9, assign53630_e82069_d_n10, assign53630_e82069_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53630_e82067: f64 = (locals.var_xp * locals.var_x2);
        (assign53630_e82067, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign53630_e82069;
        locals.var_xp_dn0 = assign53630_e82069_d_n0;
        locals.var_xp_dn2 = assign53630_e82069_d_n2;
        locals.var_xp_dn4 = assign53630_e82069_d_n4;
        locals.var_xp_dn5 = assign53630_e82069_d_n5;
        locals.var_xp_dn6 = assign53630_e82069_d_n6;
        locals.var_xp_dn7 = assign53630_e82069_d_n7;
        locals.var_xp_dn8 = assign53630_e82069_d_n8;
        locals.var_xp_dn9 = assign53630_e82069_d_n9;
        locals.var_xp_dn10 = assign53630_e82069_d_n10;
        locals.var_xp_dn13 = assign53630_e82069_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign53640_e82087, assign53640_e82087_d_n0, assign53640_e82087_d_n2, assign53640_e82087_d_n4, assign53640_e82087_d_n5, assign53640_e82087_d_n6, assign53640_e82087_d_n7, assign53640_e82087_d_n8, assign53640_e82087_d_n9, assign53640_e82087_d_n10, assign53640_e82087_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53640_e82085: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign53640_e82085, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign53640_e82087;
        locals.var_xmp_dn0 = assign53640_e82087_d_n0;
        locals.var_xmp_dn2 = assign53640_e82087_d_n2;
        locals.var_xmp_dn4 = assign53640_e82087_d_n4;
        locals.var_xmp_dn5 = assign53640_e82087_d_n5;
        locals.var_xmp_dn6 = assign53640_e82087_d_n6;
        locals.var_xmp_dn7 = assign53640_e82087_d_n7;
        locals.var_xmp_dn8 = assign53640_e82087_d_n8;
        locals.var_xmp_dn9 = assign53640_e82087_d_n9;
        locals.var_xmp_dn10 = assign53640_e82087_d_n10;
        locals.var_xmp_dn13 = assign53640_e82087_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_187(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53650_e82105, assign53650_e82105_d_n0, assign53650_e82105_d_n2, assign53650_e82105_d_n4, assign53650_e82105_d_n5, assign53650_e82105_d_n6, assign53650_e82105_d_n7, assign53650_e82105_d_n8, assign53650_e82105_d_n9, assign53650_e82105_d_n10, assign53650_e82105_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53650_e82103: f64 = (locals.var_xp * locals.var_x2);
        (assign53650_e82103, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign53650_e82105;
        locals.var_xp_dn0 = assign53650_e82105_d_n0;
        locals.var_xp_dn2 = assign53650_e82105_d_n2;
        locals.var_xp_dn4 = assign53650_e82105_d_n4;
        locals.var_xp_dn5 = assign53650_e82105_d_n5;
        locals.var_xp_dn6 = assign53650_e82105_d_n6;
        locals.var_xp_dn7 = assign53650_e82105_d_n7;
        locals.var_xp_dn8 = assign53650_e82105_d_n8;
        locals.var_xp_dn9 = assign53650_e82105_d_n9;
        locals.var_xp_dn10 = assign53650_e82105_d_n10;
        locals.var_xp_dn13 = assign53650_e82105_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign53660_e82123, assign53660_e82123_d_n0, assign53660_e82123_d_n2, assign53660_e82123_d_n4, assign53660_e82123_d_n5, assign53660_e82123_d_n6, assign53660_e82123_d_n7, assign53660_e82123_d_n8, assign53660_e82123_d_n9, assign53660_e82123_d_n10, assign53660_e82123_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53660_e82121: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign53660_e82121, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign53660_e82123;
        locals.var_xmp_dn0 = assign53660_e82123_d_n0;
        locals.var_xmp_dn2 = assign53660_e82123_d_n2;
        locals.var_xmp_dn4 = assign53660_e82123_d_n4;
        locals.var_xmp_dn5 = assign53660_e82123_d_n5;
        locals.var_xmp_dn6 = assign53660_e82123_d_n6;
        locals.var_xmp_dn7 = assign53660_e82123_d_n7;
        locals.var_xmp_dn8 = assign53660_e82123_d_n8;
        locals.var_xmp_dn9 = assign53660_e82123_d_n9;
        locals.var_xmp_dn10 = assign53660_e82123_d_n10;
        locals.var_xmp_dn13 = assign53660_e82123_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign53670_e82141, assign53670_e82141_d_n0, assign53670_e82141_d_n2, assign53670_e82141_d_n4, assign53670_e82141_d_n5, assign53670_e82141_d_n6, assign53670_e82141_d_n7, assign53670_e82141_d_n8, assign53670_e82141_d_n9, assign53670_e82141_d_n10, assign53670_e82141_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53670_e82139: f64 = (locals.var_xp + locals.var_xmp);
        (assign53670_e82139, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign53670_e82141;
        locals.var_arg_dn0 = assign53670_e82141_d_n0;
        locals.var_arg_dn2 = assign53670_e82141_d_n2;
        locals.var_arg_dn4 = assign53670_e82141_d_n4;
        locals.var_arg_dn5 = assign53670_e82141_d_n5;
        locals.var_arg_dn6 = assign53670_e82141_d_n6;
        locals.var_arg_dn7 = assign53670_e82141_d_n7;
        locals.var_arg_dn8 = assign53670_e82141_d_n8;
        locals.var_arg_dn9 = assign53670_e82141_d_n9;
        locals.var_arg_dn10 = assign53670_e82141_d_n10;
        locals.var_arg_dn13 = assign53670_e82141_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign53680_e82157, assign53680_e82157_d_n0, assign53680_e82157_d_n2, assign53680_e82157_d_n4, assign53680_e82157_d_n5, assign53680_e82157_d_n6, assign53680_e82157_d_n7, assign53680_e82157_d_n8, assign53680_e82157_d_n9, assign53680_e82157_d_n10, assign53680_e82157_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign53680_e82157;
        locals.var_dnm_dn0 = assign53680_e82157_d_n0;
        locals.var_dnm_dn2 = assign53680_e82157_d_n2;
        locals.var_dnm_dn4 = assign53680_e82157_d_n4;
        locals.var_dnm_dn5 = assign53680_e82157_d_n5;
        locals.var_dnm_dn6 = assign53680_e82157_d_n6;
        locals.var_dnm_dn7 = assign53680_e82157_d_n7;
        locals.var_dnm_dn8 = assign53680_e82157_d_n8;
        locals.var_dnm_dn9 = assign53680_e82157_d_n9;
        locals.var_dnm_dn10 = assign53680_e82157_d_n10;
        locals.var_dnm_dn13 = assign53680_e82157_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign53690_e82172: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1356 = assign53690_e82172;
        locals.var_guard1356_rv = 0.0;

        let assign53700_e82175: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1357 = assign53700_e82175;
        locals.var_guard1357_rv = 0.0;

        let (assign53710_e82195,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 != 0.0)) && (locals.var_guard1357 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53710_e82195;
        locals.var_mm_rv = 0.0;

        let assign53720_e82198: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1358 = assign53720_e82198;
        locals.var_guard1358_rv = 0.0;

        let (assign53730_e82221,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 != 0.0)) && (locals.var_guard1357 == 0.0)) && (locals.var_guard1358 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53730_e82221;
        locals.var_mm_rv = 0.0;

        let assign53740_e82224: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1359 = assign53740_e82224;
        locals.var_guard1359_rv = 0.0;

        let (assign53750_e82250,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 != 0.0)) && (locals.var_guard1357 == 0.0)) && (locals.var_guard1358 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53750_e82250;
        locals.var_mm_rv = 0.0;

        let assign53760_e82253: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1360 = assign53760_e82253;
        locals.var_guard1360_rv = 0.0;

        let (assign53770_e82282,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 != 0.0)) && (locals.var_guard1357 == 0.0)) && (locals.var_guard1358 == 0.0)) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1360 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53770_e82282;
        locals.var_mm_rv = 0.0;

        let (assign53780_e82300,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53780_e82300;
        locals.var_m0_rv = 0.0;

        let mut assign53790_loop_guard: usize = 0;
        while {
            let assign53790_cond_e82319: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign53790_cond_e82319 != 0.0
        } {
            assign53790_loop_guard += 1;
            assert!(assign53790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53790_body0_e82338, assign53790_body0_e82338_d_n0, assign53790_body0_e82338_d_n2, assign53790_body0_e82338_d_n4, assign53790_body0_e82338_d_n5, assign53790_body0_e82338_d_n6, assign53790_body0_e82338_d_n7, assign53790_body0_e82338_d_n8, assign53790_body0_e82338_d_n9, assign53790_body0_e82338_d_n10, assign53790_body0_e82338_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 != 0.0)) {
        let assign53790_body0_e82336: f64 = (locals.var_dnm).sqrt();
        (assign53790_body0_e82336, (locals.var_dnm_dn0 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn2 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn4 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn5 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn6 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn7 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn8 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn9 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn10 / (2.0 * assign53790_body0_e82336)), (locals.var_dnm_dn13 / (2.0 * assign53790_body0_e82336)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign53790_body0_e82338;
            locals.var_dnm_dn0 = assign53790_body0_e82338_d_n0;
            locals.var_dnm_dn2 = assign53790_body0_e82338_d_n2;
            locals.var_dnm_dn4 = assign53790_body0_e82338_d_n4;
            locals.var_dnm_dn5 = assign53790_body0_e82338_d_n5;
            locals.var_dnm_dn6 = assign53790_body0_e82338_d_n6;
            locals.var_dnm_dn7 = assign53790_body0_e82338_d_n7;
            locals.var_dnm_dn8 = assign53790_body0_e82338_d_n8;
            locals.var_dnm_dn9 = assign53790_body0_e82338_d_n9;
            locals.var_dnm_dn10 = assign53790_body0_e82338_d_n10;
            locals.var_dnm_dn13 = assign53790_body0_e82338_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign53790_body1_e82358,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 != 0.0)) {
        let assign53790_body1_e82356: f64 = (locals.var_m0 + 1.0);
        (assign53790_body1_e82356,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign53790_body1_e82358;
            locals.var_m0_rv = 0.0;
        }

        let (assign53800_e82388, assign53800_e82388_d_n0, assign53800_e82388_d_n2, assign53800_e82388_d_n4, assign53800_e82388_d_n5, assign53800_e82388_d_n6, assign53800_e82388_d_n7, assign53800_e82388_d_n8, assign53800_e82388_d_n9, assign53800_e82388_d_n10, assign53800_e82388_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) && (locals.var_guard1356 == 0.0)) {
        let (assign53800_e82386, assign53800_e82386_d_n0, assign53800_e82386_d_n2, assign53800_e82386_d_n4, assign53800_e82386_d_n5, assign53800_e82386_d_n6, assign53800_e82386_d_n7, assign53800_e82386_d_n8, assign53800_e82386_d_n9, assign53800_e82386_d_n10, assign53800_e82386_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign53800_e82383: f64 = (2.0 * 2.0);
                let assign53800_e82384: f64 = (1.0 / assign53800_e82383);
                let assign53800_e82385: f64 = (locals.var_dnm).powf(assign53800_e82384);
                (assign53800_e82385, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn0)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn2)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn4)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn5)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn6)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn7)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn8)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn9)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn10)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53800_e82384) as f64).is_finite() && ((assign53800_e82384) as f64).fract() == 0.0 { if assign53800_e82384 == 0.0 { 0.0 } else { (assign53800_e82384 * ((locals.var_dnm).powf(assign53800_e82384 - 1.0) * locals.var_dnm_dn13)) } } else { (assign53800_e82385 * (assign53800_e82384 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign53800_e82386, assign53800_e82386_d_n0, assign53800_e82386_d_n2, assign53800_e82386_d_n4, assign53800_e82386_d_n5, assign53800_e82386_d_n6, assign53800_e82386_d_n7, assign53800_e82386_d_n8, assign53800_e82386_d_n9, assign53800_e82386_d_n10, assign53800_e82386_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign53800_e82388;
        locals.var_dnm_dn0 = assign53800_e82388_d_n0;
        locals.var_dnm_dn2 = assign53800_e82388_d_n2;
        locals.var_dnm_dn4 = assign53800_e82388_d_n4;
        locals.var_dnm_dn5 = assign53800_e82388_d_n5;
        locals.var_dnm_dn6 = assign53800_e82388_d_n6;
        locals.var_dnm_dn7 = assign53800_e82388_d_n7;
        locals.var_dnm_dn8 = assign53800_e82388_d_n8;
        locals.var_dnm_dn9 = assign53800_e82388_d_n9;
        locals.var_dnm_dn10 = assign53800_e82388_d_n10;
        locals.var_dnm_dn13 = assign53800_e82388_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign53810_e82406, assign53810_e82406_d_n0, assign53810_e82406_d_n2, assign53810_e82406_d_n4, assign53810_e82406_d_n5, assign53810_e82406_d_n6, assign53810_e82406_d_n7, assign53810_e82406_d_n8, assign53810_e82406_d_n9, assign53810_e82406_d_n10, assign53810_e82406_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53810_e82404: f64 = (1.0 / locals.var_dnm);
        (assign53810_e82404, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign53810_e82406;
        locals.var_dnm_dn0 = assign53810_e82406_d_n0;
        locals.var_dnm_dn2 = assign53810_e82406_d_n2;
        locals.var_dnm_dn4 = assign53810_e82406_d_n4;
        locals.var_dnm_dn5 = assign53810_e82406_d_n5;
        locals.var_dnm_dn6 = assign53810_e82406_d_n6;
        locals.var_dnm_dn7 = assign53810_e82406_d_n7;
        locals.var_dnm_dn8 = assign53810_e82406_d_n8;
        locals.var_dnm_dn9 = assign53810_e82406_d_n9;
        locals.var_dnm_dn10 = assign53810_e82406_d_n10;
        locals.var_dnm_dn13 = assign53810_e82406_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign53820_e82426, assign53820_e82426_d_n0, assign53820_e82426_d_n2, assign53820_e82426_d_n4, assign53820_e82426_d_n5, assign53820_e82426_d_n6, assign53820_e82426_d_n7, assign53820_e82426_d_n8, assign53820_e82426_d_n9, assign53820_e82426_d_n10, assign53820_e82426_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53820_e82422: f64 = (locals.var_tmf1 * 1e-16);
        let assign53820_e82424: f64 = (assign53820_e82422 * locals.var_dnm);
        (assign53820_e82424, (((locals.var_tmf1_dn0 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-16) * locals.var_dnm) + (assign53820_e82422 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign53820_e82426;
        locals.var_tmf0_dn0 = assign53820_e82426_d_n0;
        locals.var_tmf0_dn2 = assign53820_e82426_d_n2;
        locals.var_tmf0_dn4 = assign53820_e82426_d_n4;
        locals.var_tmf0_dn5 = assign53820_e82426_d_n5;
        locals.var_tmf0_dn6 = assign53820_e82426_d_n6;
        locals.var_tmf0_dn7 = assign53820_e82426_d_n7;
        locals.var_tmf0_dn8 = assign53820_e82426_d_n8;
        locals.var_tmf0_dn9 = assign53820_e82426_d_n9;
        locals.var_tmf0_dn10 = assign53820_e82426_d_n10;
        locals.var_tmf0_dn13 = assign53820_e82426_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign53830_e82448, assign53830_e82448_d_n0, assign53830_e82448_d_n2, assign53830_e82448_d_n4, assign53830_e82448_d_n5, assign53830_e82448_d_n6, assign53830_e82448_d_n7, assign53830_e82448_d_n8, assign53830_e82448_d_n9, assign53830_e82448_d_n10, assign53830_e82448_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53830_e82442: f64 = (1e-16 * locals.var_xmp);
        let assign53830_e82444: f64 = (assign53830_e82442 * locals.var_dnm);
        let assign53830_e82446: f64 = (assign53830_e82444 / locals.var_arg);
        (assign53830_e82446, ((((((1e-16 * locals.var_xmp_dn0) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn0)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn2) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn2)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn4) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn4)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn5) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn5)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn6) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn6)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn7) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn7)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn8) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn8)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn9) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn9)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn10) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn10)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn13) * locals.var_dnm) + (assign53830_e82442 * locals.var_dnm_dn13)) * locals.var_arg) - (assign53830_e82444 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53830_e82448;
        locals.var_t0_dn0 = assign53830_e82448_d_n0;
        locals.var_t0_dn2 = assign53830_e82448_d_n2;
        locals.var_t0_dn4 = assign53830_e82448_d_n4;
        locals.var_t0_dn5 = assign53830_e82448_d_n5;
        locals.var_t0_dn6 = assign53830_e82448_d_n6;
        locals.var_t0_dn7 = assign53830_e82448_d_n7;
        locals.var_t0_dn8 = assign53830_e82448_d_n8;
        locals.var_t0_dn9 = assign53830_e82448_d_n9;
        locals.var_t0_dn10 = assign53830_e82448_d_n10;
        locals.var_t0_dn13 = assign53830_e82448_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign53840_e82468, assign53840_e82468_d_n0, assign53840_e82468_d_n2, assign53840_e82468_d_n4, assign53840_e82468_d_n5, assign53840_e82468_d_n6, assign53840_e82468_d_n7, assign53840_e82468_d_n8, assign53840_e82468_d_n9, assign53840_e82468_d_n10, assign53840_e82468_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53840_e82464: f64 = 1e-16;
        let assign53840_e82466: f64 = (assign53840_e82464 - locals.var_tmf0);
        (assign53840_e82466, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign53840_e82468;
        locals.var_w_res_dn0 = assign53840_e82468_d_n0;
        locals.var_w_res_dn2 = assign53840_e82468_d_n2;
        locals.var_w_res_dn4 = assign53840_e82468_d_n4;
        locals.var_w_res_dn5 = assign53840_e82468_d_n5;
        locals.var_w_res_dn6 = assign53840_e82468_d_n6;
        locals.var_w_res_dn7 = assign53840_e82468_d_n7;
        locals.var_w_res_dn8 = assign53840_e82468_d_n8;
        locals.var_w_res_dn9 = assign53840_e82468_d_n9;
        locals.var_w_res_dn10 = assign53840_e82468_d_n10;
        locals.var_w_res_dn13 = assign53840_e82468_d_n13;
        locals.var_w_res_rv = 0.0;

        let (assign53850_e82484, assign53850_e82484_d_n0, assign53850_e82484_d_n2, assign53850_e82484_d_n4, assign53850_e82484_d_n5, assign53850_e82484_d_n6, assign53850_e82484_d_n7, assign53850_e82484_d_n8, assign53850_e82484_d_n9, assign53850_e82484_d_n10, assign53850_e82484_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53850_e82484;
        locals.var_t0_dn0 = assign53850_e82484_d_n0;
        locals.var_t0_dn2 = assign53850_e82484_d_n2;
        locals.var_t0_dn4 = assign53850_e82484_d_n4;
        locals.var_t0_dn5 = assign53850_e82484_d_n5;
        locals.var_t0_dn6 = assign53850_e82484_d_n6;
        locals.var_t0_dn7 = assign53850_e82484_d_n7;
        locals.var_t0_dn8 = assign53850_e82484_d_n8;
        locals.var_t0_dn9 = assign53850_e82484_d_n9;
        locals.var_t0_dn10 = assign53850_e82484_d_n10;
        locals.var_t0_dn13 = assign53850_e82484_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign53860_e82501, assign53860_e82501_d_n0, assign53860_e82501_d_n2, assign53860_e82501_d_n4, assign53860_e82501_d_n5, assign53860_e82501_d_n6, assign53860_e82501_d_n7, assign53860_e82501_d_n8, assign53860_e82501_d_n9, assign53860_e82501_d_n10, assign53860_e82501_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 == 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign53860_e82501;
        locals.var_w_res_dn0 = assign53860_e82501_d_n0;
        locals.var_w_res_dn2 = assign53860_e82501_d_n2;
        locals.var_w_res_dn4 = assign53860_e82501_d_n4;
        locals.var_w_res_dn5 = assign53860_e82501_d_n5;
        locals.var_w_res_dn6 = assign53860_e82501_d_n6;
        locals.var_w_res_dn7 = assign53860_e82501_d_n7;
        locals.var_w_res_dn8 = assign53860_e82501_d_n8;
        locals.var_w_res_dn9 = assign53860_e82501_d_n9;
        locals.var_w_res_dn10 = assign53860_e82501_d_n10;
        locals.var_w_res_dn13 = assign53860_e82501_d_n13;
        locals.var_w_res_rv = 0.0;

        let (assign53870_e82518, assign53870_e82518_d_n0, assign53870_e82518_d_n2, assign53870_e82518_d_n4, assign53870_e82518_d_n5, assign53870_e82518_d_n6, assign53870_e82518_d_n7, assign53870_e82518_d_n8, assign53870_e82518_d_n9, assign53870_e82518_d_n10, assign53870_e82518_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1355 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign53870_e82518;
        locals.var_t0_dn0 = assign53870_e82518_d_n0;
        locals.var_t0_dn2 = assign53870_e82518_d_n2;
        locals.var_t0_dn4 = assign53870_e82518_d_n4;
        locals.var_t0_dn5 = assign53870_e82518_d_n5;
        locals.var_t0_dn6 = assign53870_e82518_d_n6;
        locals.var_t0_dn7 = assign53870_e82518_d_n7;
        locals.var_t0_dn8 = assign53870_e82518_d_n8;
        locals.var_t0_dn9 = assign53870_e82518_d_n9;
        locals.var_t0_dn10 = assign53870_e82518_d_n10;
        locals.var_t0_dn13 = assign53870_e82518_d_n13;
        locals.var_t0_rv = 0.0;

        let assign53880_e82521: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1361 = assign53880_e82521;
        locals.var_guard1361_rv = 0.0;

        let (assign53890_e82537, assign53890_e82537_d_n0, assign53890_e82537_d_n2, assign53890_e82537_d_n4, assign53890_e82537_d_n5, assign53890_e82537_d_n6, assign53890_e82537_d_n7, assign53890_e82537_d_n8, assign53890_e82537_d_n9, assign53890_e82537_d_n10, assign53890_e82537_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1361 != 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn13,)
    }
};
        locals.var_w_res_leak = assign53890_e82537;
        locals.var_w_res_leak_dn0 = assign53890_e82537_d_n0;
        locals.var_w_res_leak_dn2 = assign53890_e82537_d_n2;
        locals.var_w_res_leak_dn4 = assign53890_e82537_d_n4;
        locals.var_w_res_leak_dn5 = assign53890_e82537_d_n5;
        locals.var_w_res_leak_dn6 = assign53890_e82537_d_n6;
        locals.var_w_res_leak_dn7 = assign53890_e82537_d_n7;
        locals.var_w_res_leak_dn8 = assign53890_e82537_d_n8;
        locals.var_w_res_leak_dn9 = assign53890_e82537_d_n9;
        locals.var_w_res_leak_dn10 = assign53890_e82537_d_n10;
        locals.var_w_res_leak_dn13 = assign53890_e82537_d_n13;
        locals.var_w_res_leak_rv = 0.0;

        let assign53900_e82540: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1362 = assign53900_e82540;
        locals.var_guard1362_rv = 0.0;

        let (assign53910_e82558, assign53910_e82558_d_n0, assign53910_e82558_d_n2, assign53910_e82558_d_n4, assign53910_e82558_d_n5, assign53910_e82558_d_n6, assign53910_e82558_d_n7, assign53910_e82558_d_n8, assign53910_e82558_d_n9, assign53910_e82558_d_n10, assign53910_e82558_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1362 != 0.0)) {
        let assign53910_e82556: f64 = (p.p399 * locals.var_vbsc__blk1117);
        (assign53910_e82556, (p.p399 * locals.var_vbsc__blk1117_dn0), (p.p399 * locals.var_vbsc__blk1117_dn2), (p.p399 * locals.var_vbsc__blk1117_dn4), (p.p399 * locals.var_vbsc__blk1117_dn5), (p.p399 * locals.var_vbsc__blk1117_dn6), (p.p399 * locals.var_vbsc__blk1117_dn7), (p.p399 * locals.var_vbsc__blk1117_dn8), (p.p399 * locals.var_vbsc__blk1117_dn9), (p.p399 * locals.var_vbsc__blk1117_dn10), (p.p399 * locals.var_vbsc__blk1117_dn13),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn13,)
    }
};
        locals.var_depvbs = assign53910_e82558;
        locals.var_depvbs_dn0 = assign53910_e82558_d_n0;
        locals.var_depvbs_dn2 = assign53910_e82558_d_n2;
        locals.var_depvbs_dn4 = assign53910_e82558_d_n4;
        locals.var_depvbs_dn5 = assign53910_e82558_d_n5;
        locals.var_depvbs_dn6 = assign53910_e82558_d_n6;
        locals.var_depvbs_dn7 = assign53910_e82558_d_n7;
        locals.var_depvbs_dn8 = assign53910_e82558_d_n8;
        locals.var_depvbs_dn9 = assign53910_e82558_d_n9;
        locals.var_depvbs_dn10 = assign53910_e82558_d_n10;
        locals.var_depvbs_dn13 = assign53910_e82558_d_n13;
        locals.var_depvbs_rv = 0.0;

        let (assign53920_e82576, assign53920_e82576_d_n0, assign53920_e82576_d_n2, assign53920_e82576_d_n4, assign53920_e82576_d_n5, assign53920_e82576_d_n6, assign53920_e82576_d_n7, assign53920_e82576_d_n8, assign53920_e82576_d_n9, assign53920_e82576_d_n10, assign53920_e82576_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1362 != 0.0)) {
        let assign53920_e82574: f64 = (locals.var_depvbs - 1.0);
        (assign53920_e82574, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign53920_e82576;
        locals.var_ps0dep_dn0 = assign53920_e82576_d_n0;
        locals.var_ps0dep_dn2 = assign53920_e82576_d_n2;
        locals.var_ps0dep_dn4 = assign53920_e82576_d_n4;
        locals.var_ps0dep_dn5 = assign53920_e82576_d_n5;
        locals.var_ps0dep_dn6 = assign53920_e82576_d_n6;
        locals.var_ps0dep_dn7 = assign53920_e82576_d_n7;
        locals.var_ps0dep_dn8 = assign53920_e82576_d_n8;
        locals.var_ps0dep_dn9 = assign53920_e82576_d_n9;
        locals.var_ps0dep_dn10 = assign53920_e82576_d_n10;
        locals.var_ps0dep_dn13 = assign53920_e82576_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign53930_e82592, assign53930_e82592_d_n0, assign53930_e82592_d_n2, assign53930_e82592_d_n4, assign53930_e82592_d_n5, assign53930_e82592_d_n6, assign53930_e82592_d_n7, assign53930_e82592_d_n8, assign53930_e82592_d_n9, assign53930_e82592_d_n10, assign53930_e82592_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1362 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn13,)
    }
};
        locals.var_vgp_ws = assign53930_e82592;
        locals.var_vgp_ws_dn0 = assign53930_e82592_d_n0;
        locals.var_vgp_ws_dn2 = assign53930_e82592_d_n2;
        locals.var_vgp_ws_dn4 = assign53930_e82592_d_n4;
        locals.var_vgp_ws_dn5 = assign53930_e82592_d_n5;
        locals.var_vgp_ws_dn6 = assign53930_e82592_d_n6;
        locals.var_vgp_ws_dn7 = assign53930_e82592_d_n7;
        locals.var_vgp_ws_dn8 = assign53930_e82592_d_n8;
        locals.var_vgp_ws_dn9 = assign53930_e82592_d_n9;
        locals.var_vgp_ws_dn10 = assign53930_e82592_d_n10;
        locals.var_vgp_ws_dn13 = assign53930_e82592_d_n13;
        locals.var_vgp_ws_rv = 0.0;

        let (assign53940_e82608, assign53940_e82608_d_n0, assign53940_e82608_d_n2, assign53940_e82608_d_n4, assign53940_e82608_d_n5, assign53940_e82608_d_n6, assign53940_e82608_d_n7, assign53940_e82608_d_n8, assign53940_e82608_d_n9, assign53940_e82608_d_n10, assign53940_e82608_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1362 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn13,)
    }
};
        locals.var_vgp_res_raw = assign53940_e82608;
        locals.var_vgp_res_raw_dn0 = assign53940_e82608_d_n0;
        locals.var_vgp_res_raw_dn2 = assign53940_e82608_d_n2;
        locals.var_vgp_res_raw_dn4 = assign53940_e82608_d_n4;
        locals.var_vgp_res_raw_dn5 = assign53940_e82608_d_n5;
        locals.var_vgp_res_raw_dn6 = assign53940_e82608_d_n6;
        locals.var_vgp_res_raw_dn7 = assign53940_e82608_d_n7;
        locals.var_vgp_res_raw_dn8 = assign53940_e82608_d_n8;
        locals.var_vgp_res_raw_dn9 = assign53940_e82608_d_n9;
        locals.var_vgp_res_raw_dn10 = assign53940_e82608_d_n10;
        locals.var_vgp_res_raw_dn13 = assign53940_e82608_d_n13;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign53950_e82629, assign53950_e82629_d_n0, assign53950_e82629_d_n2, assign53950_e82629_d_n4, assign53950_e82629_d_n5, assign53950_e82629_d_n6, assign53950_e82629_d_n7, assign53950_e82629_d_n8, assign53950_e82629_d_n9, assign53950_e82629_d_n10, assign53950_e82629_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1362 == 0.0)) {
        let assign53950_e82625: f64 = (p.p399 * locals.var_vbsc__blk1117);
        let assign53950_e82627: f64 = (assign53950_e82625 - 0.1);
        (assign53950_e82627, (p.p399 * locals.var_vbsc__blk1117_dn0), (p.p399 * locals.var_vbsc__blk1117_dn2), (p.p399 * locals.var_vbsc__blk1117_dn4), (p.p399 * locals.var_vbsc__blk1117_dn5), (p.p399 * locals.var_vbsc__blk1117_dn6), (p.p399 * locals.var_vbsc__blk1117_dn7), (p.p399 * locals.var_vbsc__blk1117_dn8), (p.p399 * locals.var_vbsc__blk1117_dn9), (p.p399 * locals.var_vbsc__blk1117_dn10), (p.p399 * locals.var_vbsc__blk1117_dn13),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn13,)
    }
};
        locals.var_depvbs = assign53950_e82629;
        locals.var_depvbs_dn0 = assign53950_e82629_d_n0;
        locals.var_depvbs_dn2 = assign53950_e82629_d_n2;
        locals.var_depvbs_dn4 = assign53950_e82629_d_n4;
        locals.var_depvbs_dn5 = assign53950_e82629_d_n5;
        locals.var_depvbs_dn6 = assign53950_e82629_d_n6;
        locals.var_depvbs_dn7 = assign53950_e82629_d_n7;
        locals.var_depvbs_dn8 = assign53950_e82629_d_n8;
        locals.var_depvbs_dn9 = assign53950_e82629_d_n9;
        locals.var_depvbs_dn10 = assign53950_e82629_d_n10;
        locals.var_depvbs_dn13 = assign53950_e82629_d_n13;
        locals.var_depvbs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_188(
        locals: &mut StampLocals,
    ) {
        let (assign53960_e82646, assign53960_e82646_d_n0, assign53960_e82646_d_n2, assign53960_e82646_d_n4, assign53960_e82646_d_n5, assign53960_e82646_d_n6, assign53960_e82646_d_n7, assign53960_e82646_d_n8, assign53960_e82646_d_n9, assign53960_e82646_d_n10, assign53960_e82646_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1362 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign53960_e82646;
        locals.var_ps0dep_dn0 = assign53960_e82646_d_n0;
        locals.var_ps0dep_dn2 = assign53960_e82646_d_n2;
        locals.var_ps0dep_dn4 = assign53960_e82646_d_n4;
        locals.var_ps0dep_dn5 = assign53960_e82646_d_n5;
        locals.var_ps0dep_dn6 = assign53960_e82646_d_n6;
        locals.var_ps0dep_dn7 = assign53960_e82646_d_n7;
        locals.var_ps0dep_dn8 = assign53960_e82646_d_n8;
        locals.var_ps0dep_dn9 = assign53960_e82646_d_n9;
        locals.var_ps0dep_dn10 = assign53960_e82646_d_n10;
        locals.var_ps0dep_dn13 = assign53960_e82646_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign53970_e82663, assign53970_e82663_d_n0, assign53970_e82663_d_n2, assign53970_e82663_d_n4, assign53970_e82663_d_n5, assign53970_e82663_d_n6, assign53970_e82663_d_n7, assign53970_e82663_d_n8, assign53970_e82663_d_n9, assign53970_e82663_d_n10, assign53970_e82663_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1362 == 0.0)) {
        (locals.var_vgp_res__blk1145, locals.var_vgp_res__blk1145_dn0, locals.var_vgp_res__blk1145_dn2, locals.var_vgp_res__blk1145_dn4, locals.var_vgp_res__blk1145_dn5, locals.var_vgp_res__blk1145_dn6, locals.var_vgp_res__blk1145_dn7, locals.var_vgp_res__blk1145_dn8, locals.var_vgp_res__blk1145_dn9, locals.var_vgp_res__blk1145_dn10, locals.var_vgp_res__blk1145_dn13,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn13,)
    }
};
        locals.var_vgp_ws = assign53970_e82663;
        locals.var_vgp_ws_dn0 = assign53970_e82663_d_n0;
        locals.var_vgp_ws_dn2 = assign53970_e82663_d_n2;
        locals.var_vgp_ws_dn4 = assign53970_e82663_d_n4;
        locals.var_vgp_ws_dn5 = assign53970_e82663_d_n5;
        locals.var_vgp_ws_dn6 = assign53970_e82663_d_n6;
        locals.var_vgp_ws_dn7 = assign53970_e82663_d_n7;
        locals.var_vgp_ws_dn8 = assign53970_e82663_d_n8;
        locals.var_vgp_ws_dn9 = assign53970_e82663_d_n9;
        locals.var_vgp_ws_dn10 = assign53970_e82663_d_n10;
        locals.var_vgp_ws_dn13 = assign53970_e82663_d_n13;
        locals.var_vgp_ws_rv = 0.0;

        let (assign53980_e82680, assign53980_e82680_d_n0, assign53980_e82680_d_n2, assign53980_e82680_d_n4, assign53980_e82680_d_n5, assign53980_e82680_d_n6, assign53980_e82680_d_n7, assign53980_e82680_d_n8, assign53980_e82680_d_n9, assign53980_e82680_d_n10, assign53980_e82680_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1362 == 0.0)) {
        (locals.var_vgp_res__blk1145, locals.var_vgp_res__blk1145_dn0, locals.var_vgp_res__blk1145_dn2, locals.var_vgp_res__blk1145_dn4, locals.var_vgp_res__blk1145_dn5, locals.var_vgp_res__blk1145_dn6, locals.var_vgp_res__blk1145_dn7, locals.var_vgp_res__blk1145_dn8, locals.var_vgp_res__blk1145_dn9, locals.var_vgp_res__blk1145_dn10, locals.var_vgp_res__blk1145_dn13,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn13,)
    }
};
        locals.var_vgp_res_raw = assign53980_e82680;
        locals.var_vgp_res_raw_dn0 = assign53980_e82680_d_n0;
        locals.var_vgp_res_raw_dn2 = assign53980_e82680_d_n2;
        locals.var_vgp_res_raw_dn4 = assign53980_e82680_d_n4;
        locals.var_vgp_res_raw_dn5 = assign53980_e82680_d_n5;
        locals.var_vgp_res_raw_dn6 = assign53980_e82680_d_n6;
        locals.var_vgp_res_raw_dn7 = assign53980_e82680_d_n7;
        locals.var_vgp_res_raw_dn8 = assign53980_e82680_d_n8;
        locals.var_vgp_res_raw_dn9 = assign53980_e82680_d_n9;
        locals.var_vgp_res_raw_dn10 = assign53980_e82680_d_n10;
        locals.var_vgp_res_raw_dn13 = assign53980_e82680_d_n13;
        locals.var_vgp_res_raw_rv = 0.0;

        let (assign53990_e82694,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign53990_e82694;
        locals.var_flg_conv_rv = 0.0;

        let (assign54000_e82708,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign54000_e82708;
        locals.var_lp_s0_rv = 0.0;

        let mut assign54010_loop_guard: usize = 0;
        while {
            let assign54010_cond_e82723: f64 = if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign54010_cond_e82723 != 0.0
        } {
            assign54010_loop_guard += 1;
            assert!(assign54010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54010_body0_e82739, assign54010_body0_e82739_d_n0, assign54010_body0_e82739_d_n2, assign54010_body0_e82739_d_n4, assign54010_body0_e82739_d_n5, assign54010_body0_e82739_d_n6, assign54010_body0_e82739_d_n7, assign54010_body0_e82739_d_n8, assign54010_body0_e82739_d_n9, assign54010_body0_e82739_d_n10, assign54010_body0_e82739_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign54010_body0_e82737: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign54010_body0_e82737, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn13 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign54010_body0_e82739;
            locals.var_t1_dn0 = assign54010_body0_e82739_d_n0;
            locals.var_t1_dn2 = assign54010_body0_e82739_d_n2;
            locals.var_t1_dn4 = assign54010_body0_e82739_d_n4;
            locals.var_t1_dn5 = assign54010_body0_e82739_d_n5;
            locals.var_t1_dn6 = assign54010_body0_e82739_d_n6;
            locals.var_t1_dn7 = assign54010_body0_e82739_d_n7;
            locals.var_t1_dn8 = assign54010_body0_e82739_d_n8;
            locals.var_t1_dn9 = assign54010_body0_e82739_d_n9;
            locals.var_t1_dn10 = assign54010_body0_e82739_d_n10;
            locals.var_t1_dn13 = assign54010_body0_e82739_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign54010_body1_e82754, assign54010_body1_e82754_d_n0, assign54010_body1_e82754_d_n2, assign54010_body1_e82754_d_n4, assign54010_body1_e82754_d_n5, assign54010_body1_e82754_d_n6, assign54010_body1_e82754_d_n7, assign54010_body1_e82754_d_n8, assign54010_body1_e82754_d_n9, assign54010_body1_e82754_d_n10, assign54010_body1_e82754_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign54010_body1_e82752: f64 = (locals.var_t1).exp();
        (assign54010_body1_e82752, (assign54010_body1_e82752 * locals.var_t1_dn0), (assign54010_body1_e82752 * locals.var_t1_dn2), (assign54010_body1_e82752 * locals.var_t1_dn4), (assign54010_body1_e82752 * locals.var_t1_dn5), (assign54010_body1_e82752 * locals.var_t1_dn6), (assign54010_body1_e82752 * locals.var_t1_dn7), (assign54010_body1_e82752 * locals.var_t1_dn8), (assign54010_body1_e82752 * locals.var_t1_dn9), (assign54010_body1_e82752 * locals.var_t1_dn10), (assign54010_body1_e82752 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign54010_body1_e82754;
            locals.var_t2_dn0 = assign54010_body1_e82754_d_n0;
            locals.var_t2_dn2 = assign54010_body1_e82754_d_n2;
            locals.var_t2_dn4 = assign54010_body1_e82754_d_n4;
            locals.var_t2_dn5 = assign54010_body1_e82754_d_n5;
            locals.var_t2_dn6 = assign54010_body1_e82754_d_n6;
            locals.var_t2_dn7 = assign54010_body1_e82754_d_n7;
            locals.var_t2_dn8 = assign54010_body1_e82754_d_n8;
            locals.var_t2_dn9 = assign54010_body1_e82754_d_n9;
            locals.var_t2_dn10 = assign54010_body1_e82754_d_n10;
            locals.var_t2_dn13 = assign54010_body1_e82754_d_n13;
            locals.var_t2_rv = 0.0;
            let assign54010_body2_e82757: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1363 = assign54010_body2_e82757;
            locals.var_guard1363_rv = 0.0;
            let (assign54010_body3_e82783, assign54010_body3_e82783_d_n0, assign54010_body3_e82783_d_n2, assign54010_body3_e82783_d_n4, assign54010_body3_e82783_d_n5, assign54010_body3_e82783_d_n6, assign54010_body3_e82783_d_n7, assign54010_body3_e82783_d_n8, assign54010_body3_e82783_d_n9, assign54010_body3_e82783_d_n10, assign54010_body3_e82783_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1363 != 0.0)) {
        let assign54010_body3_e82772: f64 = (-locals.var_cnst0);
        let assign54010_body3_e82775: f64 = (locals.var_t2 - 1.0);
        let assign54010_body3_e82777: f64 = (assign54010_body3_e82775 - locals.var_t1);
        let assign54010_body3_e82779: f64 = (assign54010_body3_e82777 + 1e-15);
        let assign54010_body3_e82780: f64 = (assign54010_body3_e82779).sqrt();
        let assign54010_body3_e82781: f64 = (assign54010_body3_e82772 * assign54010_body3_e82780);
        (assign54010_body3_e82781, (((-locals.var_cnst0_dn0) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn2) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn4) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn5) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn6) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn7) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn8) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn9) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn10) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign54010_body3_e82780)))), (((-locals.var_cnst0_dn13) * assign54010_body3_e82780) + (assign54010_body3_e82772 * ((locals.var_t2_dn13 - locals.var_t1_dn13) / (2.0 * assign54010_body3_e82780)))),)
    } else {
        (locals.var_q_s0__blk1322, locals.var_q_s0__blk1322_dn0, locals.var_q_s0__blk1322_dn2, locals.var_q_s0__blk1322_dn4, locals.var_q_s0__blk1322_dn5, locals.var_q_s0__blk1322_dn6, locals.var_q_s0__blk1322_dn7, locals.var_q_s0__blk1322_dn8, locals.var_q_s0__blk1322_dn9, locals.var_q_s0__blk1322_dn10, locals.var_q_s0__blk1322_dn13,)
    }
};
            locals.var_q_s0__blk1322 = assign54010_body3_e82783;
            locals.var_q_s0__blk1322_dn0 = assign54010_body3_e82783_d_n0;
            locals.var_q_s0__blk1322_dn2 = assign54010_body3_e82783_d_n2;
            locals.var_q_s0__blk1322_dn4 = assign54010_body3_e82783_d_n4;
            locals.var_q_s0__blk1322_dn5 = assign54010_body3_e82783_d_n5;
            locals.var_q_s0__blk1322_dn6 = assign54010_body3_e82783_d_n6;
            locals.var_q_s0__blk1322_dn7 = assign54010_body3_e82783_d_n7;
            locals.var_q_s0__blk1322_dn8 = assign54010_body3_e82783_d_n8;
            locals.var_q_s0__blk1322_dn9 = assign54010_body3_e82783_d_n9;
            locals.var_q_s0__blk1322_dn10 = assign54010_body3_e82783_d_n10;
            locals.var_q_s0__blk1322_dn13 = assign54010_body3_e82783_d_n13;
            locals.var_q_s0__blk1322_rv = 0.0;
            let (assign54010_body4_e82811, assign54010_body4_e82811_d_n0, assign54010_body4_e82811_d_n2, assign54010_body4_e82811_d_n4, assign54010_body4_e82811_d_n5, assign54010_body4_e82811_d_n6, assign54010_body4_e82811_d_n7, assign54010_body4_e82811_d_n8, assign54010_body4_e82811_d_n9, assign54010_body4_e82811_d_n10, assign54010_body4_e82811_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1363 != 0.0)) {
        let assign54010_body4_e82799: f64 = (0.5 * locals.var_cnst0);
        let assign54010_body4_e82801: f64 = (assign54010_body4_e82799 * locals.var_cnst0);
        let assign54010_body4_e82803: f64 = (assign54010_body4_e82801 / locals.var_q_s0__blk1322);
        let assign54010_body4_e82806: f64 = (locals.var_beta * locals.var_t2);
        let assign54010_body4_e82808: f64 = (assign54010_body4_e82806 - locals.var_beta);
        let assign54010_body4_e82809: f64 = (assign54010_body4_e82803 * assign54010_body4_e82808);
        (assign54010_body4_e82809, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn0)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn2)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn4)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn5)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn6)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn7)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn8)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn9)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn10)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn13) * locals.var_cnst0) + (assign54010_body4_e82799 * locals.var_cnst0_dn13)) * locals.var_q_s0__blk1322) - (assign54010_body4_e82801 * locals.var_q_s0__blk1322_dn13)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)) * assign54010_body4_e82808) + (assign54010_body4_e82803 * (((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13)) - locals.var_beta_dn13))),)
    } else {
        (locals.var_q_s0_dps__blk1125, locals.var_q_s0_dps__blk1125_dn0, locals.var_q_s0_dps__blk1125_dn2, locals.var_q_s0_dps__blk1125_dn4, locals.var_q_s0_dps__blk1125_dn5, locals.var_q_s0_dps__blk1125_dn6, locals.var_q_s0_dps__blk1125_dn7, locals.var_q_s0_dps__blk1125_dn8, locals.var_q_s0_dps__blk1125_dn9, locals.var_q_s0_dps__blk1125_dn10, locals.var_q_s0_dps__blk1125_dn13,)
    }
};
            locals.var_q_s0_dps__blk1125 = assign54010_body4_e82811;
            locals.var_q_s0_dps__blk1125_dn0 = assign54010_body4_e82811_d_n0;
            locals.var_q_s0_dps__blk1125_dn2 = assign54010_body4_e82811_d_n2;
            locals.var_q_s0_dps__blk1125_dn4 = assign54010_body4_e82811_d_n4;
            locals.var_q_s0_dps__blk1125_dn5 = assign54010_body4_e82811_d_n5;
            locals.var_q_s0_dps__blk1125_dn6 = assign54010_body4_e82811_d_n6;
            locals.var_q_s0_dps__blk1125_dn7 = assign54010_body4_e82811_d_n7;
            locals.var_q_s0_dps__blk1125_dn8 = assign54010_body4_e82811_d_n8;
            locals.var_q_s0_dps__blk1125_dn9 = assign54010_body4_e82811_d_n9;
            locals.var_q_s0_dps__blk1125_dn10 = assign54010_body4_e82811_d_n10;
            locals.var_q_s0_dps__blk1125_dn13 = assign54010_body4_e82811_d_n13;
            locals.var_q_s0_dps__blk1125_rv = 0.0;
            let (assign54010_body5_e82834, assign54010_body5_e82834_d_n0, assign54010_body5_e82834_d_n2, assign54010_body5_e82834_d_n4, assign54010_body5_e82834_d_n5, assign54010_body5_e82834_d_n6, assign54010_body5_e82834_d_n7, assign54010_body5_e82834_d_n8, assign54010_body5_e82834_d_n9, assign54010_body5_e82834_d_n10, assign54010_body5_e82834_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign54010_body5_e82827: f64 = (-locals.var_beta);
        let assign54010_body5_e82830: f64 = (locals.var_ps0dep - locals.var_depvbs);
        let assign54010_body5_e82831: f64 = (assign54010_body5_e82827 * assign54010_body5_e82830);
        let assign54010_body5_e82832: f64 = (assign54010_body5_e82831).exp();
        (assign54010_body5_e82832, (assign54010_body5_e82832 * (((-locals.var_beta_dn0) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn0 - locals.var_depvbs_dn0)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn2) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn2 - locals.var_depvbs_dn2)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn4) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn4 - locals.var_depvbs_dn4)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn5) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn5 - locals.var_depvbs_dn5)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn6) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn6 - locals.var_depvbs_dn6)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn7) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn7 - locals.var_depvbs_dn7)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn8) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn8 - locals.var_depvbs_dn8)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn9) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn9 - locals.var_depvbs_dn9)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn10) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn10 - locals.var_depvbs_dn10)))), (assign54010_body5_e82832 * (((-locals.var_beta_dn13) * assign54010_body5_e82830) + (assign54010_body5_e82827 * (locals.var_ps0dep_dn13 - locals.var_depvbs_dn13)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign54010_body5_e82834;
            locals.var_t3_dn0 = assign54010_body5_e82834_d_n0;
            locals.var_t3_dn2 = assign54010_body5_e82834_d_n2;
            locals.var_t3_dn4 = assign54010_body5_e82834_d_n4;
            locals.var_t3_dn5 = assign54010_body5_e82834_d_n5;
            locals.var_t3_dn6 = assign54010_body5_e82834_d_n6;
            locals.var_t3_dn7 = assign54010_body5_e82834_d_n7;
            locals.var_t3_dn8 = assign54010_body5_e82834_d_n8;
            locals.var_t3_dn9 = assign54010_body5_e82834_d_n9;
            locals.var_t3_dn10 = assign54010_body5_e82834_d_n10;
            locals.var_t3_dn13 = assign54010_body5_e82834_d_n13;
            locals.var_t3_rv = 0.0;
            let (assign54010_body6_e82854, assign54010_body6_e82854_d_n0, assign54010_body6_e82854_d_n2, assign54010_body6_e82854_d_n4, assign54010_body6_e82854_d_n5, assign54010_body6_e82854_d_n6, assign54010_body6_e82854_d_n7, assign54010_body6_e82854_d_n8, assign54010_body6_e82854_d_n9, assign54010_body6_e82854_d_n10, assign54010_body6_e82854_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign54010_body6_e82851: f64 = (locals.var_beta * locals.var_depvbs);
        let assign54010_body6_e82852: f64 = (assign54010_body6_e82851).exp();
        (assign54010_body6_e82852, (assign54010_body6_e82852 * ((locals.var_beta_dn0 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn0))), (assign54010_body6_e82852 * ((locals.var_beta_dn2 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn2))), (assign54010_body6_e82852 * ((locals.var_beta_dn4 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn4))), (assign54010_body6_e82852 * ((locals.var_beta_dn5 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn5))), (assign54010_body6_e82852 * ((locals.var_beta_dn6 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn6))), (assign54010_body6_e82852 * ((locals.var_beta_dn7 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn7))), (assign54010_body6_e82852 * ((locals.var_beta_dn8 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn8))), (assign54010_body6_e82852 * ((locals.var_beta_dn9 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn9))), (assign54010_body6_e82852 * ((locals.var_beta_dn10 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn10))), (assign54010_body6_e82852 * ((locals.var_beta_dn13 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign54010_body6_e82854;
            locals.var_t4_dn0 = assign54010_body6_e82854_d_n0;
            locals.var_t4_dn2 = assign54010_body6_e82854_d_n2;
            locals.var_t4_dn4 = assign54010_body6_e82854_d_n4;
            locals.var_t4_dn5 = assign54010_body6_e82854_d_n5;
            locals.var_t4_dn6 = assign54010_body6_e82854_d_n6;
            locals.var_t4_dn7 = assign54010_body6_e82854_d_n7;
            locals.var_t4_dn8 = assign54010_body6_e82854_d_n8;
            locals.var_t4_dn9 = assign54010_body6_e82854_d_n9;
            locals.var_t4_dn10 = assign54010_body6_e82854_d_n10;
            locals.var_t4_dn13 = assign54010_body6_e82854_d_n13;
            locals.var_t4_rv = 0.0;
            let (assign54010_body7_e82886, assign54010_body7_e82886_d_n0, assign54010_body7_e82886_d_n2, assign54010_body7_e82886_d_n4, assign54010_body7_e82886_d_n5, assign54010_body7_e82886_d_n6, assign54010_body7_e82886_d_n7, assign54010_body7_e82886_d_n8, assign54010_body7_e82886_d_n9, assign54010_body7_e82886_d_n10, assign54010_body7_e82886_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign54010_body7_e82872: f64 = (locals.var_t2 - 1.0);
        let assign54010_body7_e82874: f64 = (assign54010_body7_e82872 - locals.var_t1);
        let assign54010_body7_e82878: f64 = (locals.var_t3 - locals.var_t4);
        let assign54010_body7_e82879: f64 = (locals.var_cnst1 * assign54010_body7_e82878);
        let assign54010_body7_e82880: f64 = (assign54010_body7_e82874 + assign54010_body7_e82879);
        let assign54010_body7_e82882: f64 = (assign54010_body7_e82880 + 1e-15);
        let assign54010_body7_e82883: f64 = (assign54010_body7_e82882).sqrt();
        let assign54010_body7_e82884: f64 = (locals.var_cnst0 * assign54010_body7_e82883);
        (assign54010_body7_e82884, ((locals.var_cnst0_dn0 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn2 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn4 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn5 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn6 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn7 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn8 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn9 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn10 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign54010_body7_e82883)))), ((locals.var_cnst0_dn13 * assign54010_body7_e82883) + (locals.var_cnst0 * (((locals.var_t2_dn13 - locals.var_t1_dn13) + ((locals.var_cnst1_dn13 * assign54010_body7_e82878) + (locals.var_cnst1 * (locals.var_t3_dn13 - locals.var_t4_dn13)))) / (2.0 * assign54010_body7_e82883)))),)
    } else {
        (locals.var_q_s0__blk1322, locals.var_q_s0__blk1322_dn0, locals.var_q_s0__blk1322_dn2, locals.var_q_s0__blk1322_dn4, locals.var_q_s0__blk1322_dn5, locals.var_q_s0__blk1322_dn6, locals.var_q_s0__blk1322_dn7, locals.var_q_s0__blk1322_dn8, locals.var_q_s0__blk1322_dn9, locals.var_q_s0__blk1322_dn10, locals.var_q_s0__blk1322_dn13,)
    }
};
            locals.var_q_s0__blk1322 = assign54010_body7_e82886;
            locals.var_q_s0__blk1322_dn0 = assign54010_body7_e82886_d_n0;
            locals.var_q_s0__blk1322_dn2 = assign54010_body7_e82886_d_n2;
            locals.var_q_s0__blk1322_dn4 = assign54010_body7_e82886_d_n4;
            locals.var_q_s0__blk1322_dn5 = assign54010_body7_e82886_d_n5;
            locals.var_q_s0__blk1322_dn6 = assign54010_body7_e82886_d_n6;
            locals.var_q_s0__blk1322_dn7 = assign54010_body7_e82886_d_n7;
            locals.var_q_s0__blk1322_dn8 = assign54010_body7_e82886_d_n8;
            locals.var_q_s0__blk1322_dn9 = assign54010_body7_e82886_d_n9;
            locals.var_q_s0__blk1322_dn10 = assign54010_body7_e82886_d_n10;
            locals.var_q_s0__blk1322_dn13 = assign54010_body7_e82886_d_n13;
            locals.var_q_s0__blk1322_rv = 0.0;
            let (assign54010_body8_e82909, assign54010_body8_e82909_d_n0, assign54010_body8_e82909_d_n2, assign54010_body8_e82909_d_n4, assign54010_body8_e82909_d_n5, assign54010_body8_e82909_d_n6, assign54010_body8_e82909_d_n7, assign54010_body8_e82909_d_n8, assign54010_body8_e82909_d_n9, assign54010_body8_e82909_d_n10, assign54010_body8_e82909_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign54010_body8_e82903: f64 = (0.5 * locals.var_cnst0);
        let assign54010_body8_e82905: f64 = (assign54010_body8_e82903 * locals.var_cnst0);
        let assign54010_body8_e82907: f64 = (assign54010_body8_e82905 / locals.var_q_s0__blk1322);
        (assign54010_body8_e82907, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn0)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn2)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn4)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn5)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn6)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn7)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn8)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn9)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn10)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)), ((((((0.5 * locals.var_cnst0_dn13) * locals.var_cnst0) + (assign54010_body8_e82903 * locals.var_cnst0_dn13)) * locals.var_q_s0__blk1322) - (assign54010_body8_e82905 * locals.var_q_s0__blk1322_dn13)) / (locals.var_q_s0__blk1322 * locals.var_q_s0__blk1322)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
            locals.var_t5 = assign54010_body8_e82909;
            locals.var_t5_dn0 = assign54010_body8_e82909_d_n0;
            locals.var_t5_dn2 = assign54010_body8_e82909_d_n2;
            locals.var_t5_dn4 = assign54010_body8_e82909_d_n4;
            locals.var_t5_dn5 = assign54010_body8_e82909_d_n5;
            locals.var_t5_dn6 = assign54010_body8_e82909_d_n6;
            locals.var_t5_dn7 = assign54010_body8_e82909_d_n7;
            locals.var_t5_dn8 = assign54010_body8_e82909_d_n8;
            locals.var_t5_dn9 = assign54010_body8_e82909_d_n9;
            locals.var_t5_dn10 = assign54010_body8_e82909_d_n10;
            locals.var_t5_dn13 = assign54010_body8_e82909_d_n13;
            locals.var_t5_rv = 0.0;
            let (assign54010_body9_e82939, assign54010_body9_e82939_d_n0, assign54010_body9_e82939_d_n2, assign54010_body9_e82939_d_n4, assign54010_body9_e82939_d_n5, assign54010_body9_e82939_d_n6, assign54010_body9_e82939_d_n7, assign54010_body9_e82939_d_n8, assign54010_body9_e82939_d_n9, assign54010_body9_e82939_d_n10, assign54010_body9_e82939_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign54010_body9_e82927: f64 = (locals.var_beta * locals.var_t2);
        let assign54010_body9_e82929: f64 = (assign54010_body9_e82927 - locals.var_beta);
        let assign54010_body9_e82932: f64 = (-locals.var_beta);
        let assign54010_body9_e82934: f64 = (assign54010_body9_e82932 * locals.var_t3);
        let assign54010_body9_e82935: f64 = (locals.var_cnst1 * assign54010_body9_e82934);
        let assign54010_body9_e82936: f64 = (assign54010_body9_e82929 + assign54010_body9_e82935);
        let assign54010_body9_e82937: f64 = (locals.var_t5 * assign54010_body9_e82936);
        (assign54010_body9_e82937, ((locals.var_t5_dn0 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn10))))))), ((locals.var_t5_dn13 * assign54010_body9_e82936) + (locals.var_t5 * ((((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13)) - locals.var_beta_dn13) + ((locals.var_cnst1_dn13 * assign54010_body9_e82934) + (locals.var_cnst1 * (((-locals.var_beta_dn13) * locals.var_t3) + (assign54010_body9_e82932 * locals.var_t3_dn13))))))),)
    } else {
        (locals.var_q_s0_dps__blk1125, locals.var_q_s0_dps__blk1125_dn0, locals.var_q_s0_dps__blk1125_dn2, locals.var_q_s0_dps__blk1125_dn4, locals.var_q_s0_dps__blk1125_dn5, locals.var_q_s0_dps__blk1125_dn6, locals.var_q_s0_dps__blk1125_dn7, locals.var_q_s0_dps__blk1125_dn8, locals.var_q_s0_dps__blk1125_dn9, locals.var_q_s0_dps__blk1125_dn10, locals.var_q_s0_dps__blk1125_dn13,)
    }
};
            locals.var_q_s0_dps__blk1125 = assign54010_body9_e82939;
            locals.var_q_s0_dps__blk1125_dn0 = assign54010_body9_e82939_d_n0;
            locals.var_q_s0_dps__blk1125_dn2 = assign54010_body9_e82939_d_n2;
            locals.var_q_s0_dps__blk1125_dn4 = assign54010_body9_e82939_d_n4;
            locals.var_q_s0_dps__blk1125_dn5 = assign54010_body9_e82939_d_n5;
            locals.var_q_s0_dps__blk1125_dn6 = assign54010_body9_e82939_d_n6;
            locals.var_q_s0_dps__blk1125_dn7 = assign54010_body9_e82939_d_n7;
            locals.var_q_s0_dps__blk1125_dn8 = assign54010_body9_e82939_d_n8;
            locals.var_q_s0_dps__blk1125_dn9 = assign54010_body9_e82939_d_n9;
            locals.var_q_s0_dps__blk1125_dn10 = assign54010_body9_e82939_d_n10;
            locals.var_q_s0_dps__blk1125_dn13 = assign54010_body9_e82939_d_n13;
            locals.var_q_s0_dps__blk1125_rv = 0.0;
            let (assign54010_body10_e82957,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign54010_body10_e82955: f64 = (150.0 + 1.0);
        (assign54010_body10_e82955,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign54010_body10_e82957;
            locals.var_lp_s0_rv = 0.0;
            let (assign54010_body11_e82980, assign54010_body11_e82980_d_n0, assign54010_body11_e82980_d_n2, assign54010_body11_e82980_d_n4, assign54010_body11_e82980_d_n5, assign54010_body11_e82980_d_n6, assign54010_body11_e82980_d_n7, assign54010_body11_e82980_d_n8, assign54010_body11_e82980_d_n9, assign54010_body11_e82980_d_n10, assign54010_body11_e82980_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54010_body11_e82975: f64 = (locals.var_vgp_ws - locals.var_ps0dep);
        let assign54010_body11_e82976: f64 = (locals.var_cox * assign54010_body11_e82975);
        let assign54010_body11_e82978: f64 = (assign54010_body11_e82976 + locals.var_q_s0__blk1322);
        (assign54010_body11_e82978, (((locals.var_cox_dn0 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1322_dn0), (((locals.var_cox_dn2 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1322_dn2), (((locals.var_cox_dn4 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1322_dn4), (((locals.var_cox_dn5 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1322_dn5), (((locals.var_cox_dn6 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1322_dn6), (((locals.var_cox_dn7 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1322_dn7), (((locals.var_cox_dn8 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1322_dn8), (((locals.var_cox_dn9 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1322_dn9), (((locals.var_cox_dn10 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1322_dn10), (((locals.var_cox_dn13 * assign54010_body11_e82975) + (locals.var_cox * (locals.var_vgp_ws_dn13 - locals.var_ps0dep_dn13))) + locals.var_q_s0__blk1322_dn13),)
    } else {
        (locals.var_pf1__blk1100, locals.var_pf1__blk1100_dn0, locals.var_pf1__blk1100_dn2, locals.var_pf1__blk1100_dn4, locals.var_pf1__blk1100_dn5, locals.var_pf1__blk1100_dn6, locals.var_pf1__blk1100_dn7, locals.var_pf1__blk1100_dn8, locals.var_pf1__blk1100_dn9, locals.var_pf1__blk1100_dn10, locals.var_pf1__blk1100_dn13,)
    }
};
            locals.var_pf1__blk1100 = assign54010_body11_e82980;
            locals.var_pf1__blk1100_dn0 = assign54010_body11_e82980_d_n0;
            locals.var_pf1__blk1100_dn2 = assign54010_body11_e82980_d_n2;
            locals.var_pf1__blk1100_dn4 = assign54010_body11_e82980_d_n4;
            locals.var_pf1__blk1100_dn5 = assign54010_body11_e82980_d_n5;
            locals.var_pf1__blk1100_dn6 = assign54010_body11_e82980_d_n6;
            locals.var_pf1__blk1100_dn7 = assign54010_body11_e82980_d_n7;
            locals.var_pf1__blk1100_dn8 = assign54010_body11_e82980_d_n8;
            locals.var_pf1__blk1100_dn9 = assign54010_body11_e82980_d_n9;
            locals.var_pf1__blk1100_dn10 = assign54010_body11_e82980_d_n10;
            locals.var_pf1__blk1100_dn13 = assign54010_body11_e82980_d_n13;
            locals.var_pf1__blk1100_rv = 0.0;
            let (assign54010_body12_e83000, assign54010_body12_e83000_d_n0, assign54010_body12_e83000_d_n2, assign54010_body12_e83000_d_n4, assign54010_body12_e83000_d_n5, assign54010_body12_e83000_d_n6, assign54010_body12_e83000_d_n7, assign54010_body12_e83000_d_n8, assign54010_body12_e83000_d_n9, assign54010_body12_e83000_d_n10, assign54010_body12_e83000_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54010_body12_e82996: f64 = (-locals.var_cox);
        let assign54010_body12_e82998: f64 = (assign54010_body12_e82996 + locals.var_q_s0_dps__blk1125);
        (assign54010_body12_e82998, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1125_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1125_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1125_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1125_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1125_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1125_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1125_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1125_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1125_dn10), ((-locals.var_cox_dn13) + locals.var_q_s0_dps__blk1125_dn13),)
    } else {
        (locals.var_pf11__blk1101, locals.var_pf11__blk1101_dn0, locals.var_pf11__blk1101_dn2, locals.var_pf11__blk1101_dn4, locals.var_pf11__blk1101_dn5, locals.var_pf11__blk1101_dn6, locals.var_pf11__blk1101_dn7, locals.var_pf11__blk1101_dn8, locals.var_pf11__blk1101_dn9, locals.var_pf11__blk1101_dn10, locals.var_pf11__blk1101_dn13,)
    }
};
            locals.var_pf11__blk1101 = assign54010_body12_e83000;
            locals.var_pf11__blk1101_dn0 = assign54010_body12_e83000_d_n0;
            locals.var_pf11__blk1101_dn2 = assign54010_body12_e83000_d_n2;
            locals.var_pf11__blk1101_dn4 = assign54010_body12_e83000_d_n4;
            locals.var_pf11__blk1101_dn5 = assign54010_body12_e83000_d_n5;
            locals.var_pf11__blk1101_dn6 = assign54010_body12_e83000_d_n6;
            locals.var_pf11__blk1101_dn7 = assign54010_body12_e83000_d_n7;
            locals.var_pf11__blk1101_dn8 = assign54010_body12_e83000_d_n8;
            locals.var_pf11__blk1101_dn9 = assign54010_body12_e83000_d_n9;
            locals.var_pf11__blk1101_dn10 = assign54010_body12_e83000_d_n10;
            locals.var_pf11__blk1101_dn13 = assign54010_body12_e83000_d_n13;
            locals.var_pf11__blk1101_rv = 0.0;
            let (assign54010_body13_e83020, assign54010_body13_e83020_d_n0, assign54010_body13_e83020_d_n2, assign54010_body13_e83020_d_n4, assign54010_body13_e83020_d_n5, assign54010_body13_e83020_d_n6, assign54010_body13_e83020_d_n7, assign54010_body13_e83020_d_n8, assign54010_body13_e83020_d_n9, assign54010_body13_e83020_d_n10, assign54010_body13_e83020_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54010_body13_e83016: f64 = (-locals.var_pf1__blk1100);
        let assign54010_body13_e83018: f64 = (assign54010_body13_e83016 / locals.var_pf11__blk1101);
        (assign54010_body13_e83018, ((((-locals.var_pf1__blk1100_dn0) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn0)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn2) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn2)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn4) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn4)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn5) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn5)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn6) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn6)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn7) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn7)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn8) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn8)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn9) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn9)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn10) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn10)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn13) * locals.var_pf11__blk1101) - (assign54010_body13_e83016 * locals.var_pf11__blk1101_dn13)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)),)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign54010_body13_e83020;
            locals.var_dps__blk1112_dn0 = assign54010_body13_e83020_d_n0;
            locals.var_dps__blk1112_dn2 = assign54010_body13_e83020_d_n2;
            locals.var_dps__blk1112_dn4 = assign54010_body13_e83020_d_n4;
            locals.var_dps__blk1112_dn5 = assign54010_body13_e83020_d_n5;
            locals.var_dps__blk1112_dn6 = assign54010_body13_e83020_d_n6;
            locals.var_dps__blk1112_dn7 = assign54010_body13_e83020_d_n7;
            locals.var_dps__blk1112_dn8 = assign54010_body13_e83020_d_n8;
            locals.var_dps__blk1112_dn9 = assign54010_body13_e83020_d_n9;
            locals.var_dps__blk1112_dn10 = assign54010_body13_e83020_d_n10;
            locals.var_dps__blk1112_dn13 = assign54010_body13_e83020_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let assign54010_body14_e83022: f64 = (locals.var_dps__blk1112).abs();
            let assign54010_body14_e83025: f64 = (1e-10 * 100.0);
            let assign54010_body14_e83026: f64 = if assign54010_body14_e83022 < assign54010_body14_e83025 { 1.0 } else { 0.0 };
            locals.var_guard1364 = assign54010_body14_e83026;
            locals.var_guard1364_rv = 0.0;
            let (assign54010_body15_e83045,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1364 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign54010_body15_e83045;
            locals.var_flg_conv_rv = 0.0;
            let assign54010_body16_e83048: f64 = if locals.var_dps__blk1112 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1365 = assign54010_body16_e83048;
            locals.var_guard1365_rv = 0.0;
            let (assign54010_body17_e83070, assign54010_body17_e83070_d_n0, assign54010_body17_e83070_d_n2, assign54010_body17_e83070_d_n4, assign54010_body17_e83070_d_n5, assign54010_body17_e83070_d_n6, assign54010_body17_e83070_d_n7, assign54010_body17_e83070_d_n8, assign54010_body17_e83070_d_n9, assign54010_body17_e83070_d_n10, assign54010_body17_e83070_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1364 == 0.0)) && (locals.var_guard1365 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign54010_body17_e83070;
            locals.var_dps__blk1112_dn0 = assign54010_body17_e83070_d_n0;
            locals.var_dps__blk1112_dn2 = assign54010_body17_e83070_d_n2;
            locals.var_dps__blk1112_dn4 = assign54010_body17_e83070_d_n4;
            locals.var_dps__blk1112_dn5 = assign54010_body17_e83070_d_n5;
            locals.var_dps__blk1112_dn6 = assign54010_body17_e83070_d_n6;
            locals.var_dps__blk1112_dn7 = assign54010_body17_e83070_d_n7;
            locals.var_dps__blk1112_dn8 = assign54010_body17_e83070_d_n8;
            locals.var_dps__blk1112_dn9 = assign54010_body17_e83070_d_n9;
            locals.var_dps__blk1112_dn10 = assign54010_body17_e83070_d_n10;
            locals.var_dps__blk1112_dn13 = assign54010_body17_e83070_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let assign54010_body18_e83073: f64 = (-0.1);
            let assign54010_body18_e83074: f64 = if locals.var_dps__blk1112 < assign54010_body18_e83073 { 1.0 } else { 0.0 };
            locals.var_guard1366 = assign54010_body18_e83074;
            locals.var_guard1366_rv = 0.0;
            let (assign54010_body19_e83100, assign54010_body19_e83100_d_n0, assign54010_body19_e83100_d_n2, assign54010_body19_e83100_d_n4, assign54010_body19_e83100_d_n5, assign54010_body19_e83100_d_n6, assign54010_body19_e83100_d_n7, assign54010_body19_e83100_d_n8, assign54010_body19_e83100_d_n9, assign54010_body19_e83100_d_n10, assign54010_body19_e83100_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1364 == 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        let assign54010_body19_e83098: f64 = (-0.1);
        (assign54010_body19_e83098, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign54010_body19_e83100;
            locals.var_dps__blk1112_dn0 = assign54010_body19_e83100_d_n0;
            locals.var_dps__blk1112_dn2 = assign54010_body19_e83100_d_n2;
            locals.var_dps__blk1112_dn4 = assign54010_body19_e83100_d_n4;
            locals.var_dps__blk1112_dn5 = assign54010_body19_e83100_d_n5;
            locals.var_dps__blk1112_dn6 = assign54010_body19_e83100_d_n6;
            locals.var_dps__blk1112_dn7 = assign54010_body19_e83100_d_n7;
            locals.var_dps__blk1112_dn8 = assign54010_body19_e83100_d_n8;
            locals.var_dps__blk1112_dn9 = assign54010_body19_e83100_d_n9;
            locals.var_dps__blk1112_dn10 = assign54010_body19_e83100_d_n10;
            locals.var_dps__blk1112_dn13 = assign54010_body19_e83100_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let (assign54010_body20_e83119, assign54010_body20_e83119_d_n0, assign54010_body20_e83119_d_n2, assign54010_body20_e83119_d_n4, assign54010_body20_e83119_d_n5, assign54010_body20_e83119_d_n6, assign54010_body20_e83119_d_n7, assign54010_body20_e83119_d_n8, assign54010_body20_e83119_d_n9, assign54010_body20_e83119_d_n10, assign54010_body20_e83119_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54010_body20_e83117: f64 = (locals.var_ps0dep + locals.var_dps__blk1112);
        (assign54010_body20_e83117, (locals.var_ps0dep_dn0 + locals.var_dps__blk1112_dn0), (locals.var_ps0dep_dn2 + locals.var_dps__blk1112_dn2), (locals.var_ps0dep_dn4 + locals.var_dps__blk1112_dn4), (locals.var_ps0dep_dn5 + locals.var_dps__blk1112_dn5), (locals.var_ps0dep_dn6 + locals.var_dps__blk1112_dn6), (locals.var_ps0dep_dn7 + locals.var_dps__blk1112_dn7), (locals.var_ps0dep_dn8 + locals.var_dps__blk1112_dn8), (locals.var_ps0dep_dn9 + locals.var_dps__blk1112_dn9), (locals.var_ps0dep_dn10 + locals.var_dps__blk1112_dn10), (locals.var_ps0dep_dn13 + locals.var_dps__blk1112_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
            locals.var_ps0dep = assign54010_body20_e83119;
            locals.var_ps0dep_dn0 = assign54010_body20_e83119_d_n0;
            locals.var_ps0dep_dn2 = assign54010_body20_e83119_d_n2;
            locals.var_ps0dep_dn4 = assign54010_body20_e83119_d_n4;
            locals.var_ps0dep_dn5 = assign54010_body20_e83119_d_n5;
            locals.var_ps0dep_dn6 = assign54010_body20_e83119_d_n6;
            locals.var_ps0dep_dn7 = assign54010_body20_e83119_d_n7;
            locals.var_ps0dep_dn8 = assign54010_body20_e83119_d_n8;
            locals.var_ps0dep_dn9 = assign54010_body20_e83119_d_n9;
            locals.var_ps0dep_dn10 = assign54010_body20_e83119_d_n10;
            locals.var_ps0dep_dn13 = assign54010_body20_e83119_d_n13;
            locals.var_ps0dep_rv = 0.0;
            let (assign54010_body21_e83135,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign54010_body21_e83133: f64 = (locals.var_lp_s0 + 1.0);
        (assign54010_body21_e83133,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign54010_body21_e83135;
            locals.var_lp_s0_rv = 0.0;
        }

        let assign54030_e83141: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1368 = assign54030_e83141;
        locals.var_guard1368_rv = 0.0;

        let (assign54040_e83157, assign54040_e83157_d_n0, assign54040_e83157_d_n2, assign54040_e83157_d_n4, assign54040_e83157_d_n5, assign54040_e83157_d_n6, assign54040_e83157_d_n7, assign54040_e83157_d_n8, assign54040_e83157_d_n9, assign54040_e83157_d_n10, assign54040_e83157_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 != 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    } else {
        (locals.var_ps0dep0, locals.var_ps0dep0_dn0, locals.var_ps0dep0_dn2, locals.var_ps0dep0_dn4, locals.var_ps0dep0_dn5, locals.var_ps0dep0_dn6, locals.var_ps0dep0_dn7, locals.var_ps0dep0_dn8, locals.var_ps0dep0_dn9, locals.var_ps0dep0_dn10, locals.var_ps0dep0_dn13,)
    }
};
        locals.var_ps0dep0 = assign54040_e83157;
        locals.var_ps0dep0_dn0 = assign54040_e83157_d_n0;
        locals.var_ps0dep0_dn2 = assign54040_e83157_d_n2;
        locals.var_ps0dep0_dn4 = assign54040_e83157_d_n4;
        locals.var_ps0dep0_dn5 = assign54040_e83157_d_n5;
        locals.var_ps0dep0_dn6 = assign54040_e83157_d_n6;
        locals.var_ps0dep0_dn7 = assign54040_e83157_d_n7;
        locals.var_ps0dep0_dn8 = assign54040_e83157_d_n8;
        locals.var_ps0dep0_dn9 = assign54040_e83157_d_n9;
        locals.var_ps0dep0_dn10 = assign54040_e83157_d_n10;
        locals.var_ps0dep0_dn13 = assign54040_e83157_d_n13;
        locals.var_ps0dep0_rv = 0.0;

        let assign54050_e83161: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54050_e83166: f64 = if ((locals.var_ps0dep < assign54050_e83161) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1369 = assign54050_e83166;
        locals.var_guard1369_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_189(
        locals: &mut StampLocals,
    ) {
        let (assign54060_e83189, assign54060_e83189_d_n0, assign54060_e83189_d_n2, assign54060_e83189_d_n4, assign54060_e83189_d_n5, assign54060_e83189_d_n6, assign54060_e83189_d_n7, assign54060_e83189_d_n8, assign54060_e83189_d_n9, assign54060_e83189_d_n10, assign54060_e83189_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54060_e83185: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54060_e83187: f64 = (assign54060_e83185 - locals.var_ps0dep);
        (assign54060_e83187, (locals.var_ps0dep0_dn0 - locals.var_ps0dep_dn0), (locals.var_ps0dep0_dn2 - locals.var_ps0dep_dn2), (locals.var_ps0dep0_dn4 - locals.var_ps0dep_dn4), (locals.var_ps0dep0_dn5 - locals.var_ps0dep_dn5), (locals.var_ps0dep0_dn6 - locals.var_ps0dep_dn6), (locals.var_ps0dep0_dn7 - locals.var_ps0dep_dn7), (locals.var_ps0dep0_dn8 - locals.var_ps0dep_dn8), (locals.var_ps0dep0_dn9 - locals.var_ps0dep_dn9), (locals.var_ps0dep0_dn10 - locals.var_ps0dep_dn10), (locals.var_ps0dep0_dn13 - locals.var_ps0dep_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign54060_e83189;
        locals.var_tmf1_dn0 = assign54060_e83189_d_n0;
        locals.var_tmf1_dn2 = assign54060_e83189_d_n2;
        locals.var_tmf1_dn4 = assign54060_e83189_d_n4;
        locals.var_tmf1_dn5 = assign54060_e83189_d_n5;
        locals.var_tmf1_dn6 = assign54060_e83189_d_n6;
        locals.var_tmf1_dn7 = assign54060_e83189_d_n7;
        locals.var_tmf1_dn8 = assign54060_e83189_d_n8;
        locals.var_tmf1_dn9 = assign54060_e83189_d_n9;
        locals.var_tmf1_dn10 = assign54060_e83189_d_n10;
        locals.var_tmf1_dn13 = assign54060_e83189_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign54070_e83210, assign54070_e83210_d_n0, assign54070_e83210_d_n2, assign54070_e83210_d_n4, assign54070_e83210_d_n5, assign54070_e83210_d_n6, assign54070_e83210_d_n7, assign54070_e83210_d_n8, assign54070_e83210_d_n9, assign54070_e83210_d_n10, assign54070_e83210_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54070_e83208: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign54070_e83208, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign54070_e83210;
        locals.var_x2_dn0 = assign54070_e83210_d_n0;
        locals.var_x2_dn2 = assign54070_e83210_d_n2;
        locals.var_x2_dn4 = assign54070_e83210_d_n4;
        locals.var_x2_dn5 = assign54070_e83210_d_n5;
        locals.var_x2_dn6 = assign54070_e83210_d_n6;
        locals.var_x2_dn7 = assign54070_e83210_d_n7;
        locals.var_x2_dn8 = assign54070_e83210_d_n8;
        locals.var_x2_dn9 = assign54070_e83210_d_n9;
        locals.var_x2_dn10 = assign54070_e83210_d_n10;
        locals.var_x2_dn13 = assign54070_e83210_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign54080_e83231, assign54080_e83231_d_n0, assign54080_e83231_d_n2, assign54080_e83231_d_n4, assign54080_e83231_d_n5, assign54080_e83231_d_n6, assign54080_e83231_d_n7, assign54080_e83231_d_n8, assign54080_e83231_d_n9, assign54080_e83231_d_n10, assign54080_e83231_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54080_e83229: f64 = (0.2 * 0.2);
        (assign54080_e83229, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign54080_e83231;
        locals.var_xmax2_dn0 = assign54080_e83231_d_n0;
        locals.var_xmax2_dn2 = assign54080_e83231_d_n2;
        locals.var_xmax2_dn4 = assign54080_e83231_d_n4;
        locals.var_xmax2_dn5 = assign54080_e83231_d_n5;
        locals.var_xmax2_dn6 = assign54080_e83231_d_n6;
        locals.var_xmax2_dn7 = assign54080_e83231_d_n7;
        locals.var_xmax2_dn8 = assign54080_e83231_d_n8;
        locals.var_xmax2_dn9 = assign54080_e83231_d_n9;
        locals.var_xmax2_dn10 = assign54080_e83231_d_n10;
        locals.var_xmax2_dn13 = assign54080_e83231_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign54090_e83250, assign54090_e83250_d_n0, assign54090_e83250_d_n2, assign54090_e83250_d_n4, assign54090_e83250_d_n5, assign54090_e83250_d_n6, assign54090_e83250_d_n7, assign54090_e83250_d_n8, assign54090_e83250_d_n9, assign54090_e83250_d_n10, assign54090_e83250_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign54090_e83250;
        locals.var_xp_dn0 = assign54090_e83250_d_n0;
        locals.var_xp_dn2 = assign54090_e83250_d_n2;
        locals.var_xp_dn4 = assign54090_e83250_d_n4;
        locals.var_xp_dn5 = assign54090_e83250_d_n5;
        locals.var_xp_dn6 = assign54090_e83250_d_n6;
        locals.var_xp_dn7 = assign54090_e83250_d_n7;
        locals.var_xp_dn8 = assign54090_e83250_d_n8;
        locals.var_xp_dn9 = assign54090_e83250_d_n9;
        locals.var_xp_dn10 = assign54090_e83250_d_n10;
        locals.var_xp_dn13 = assign54090_e83250_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign54100_e83269, assign54100_e83269_d_n0, assign54100_e83269_d_n2, assign54100_e83269_d_n4, assign54100_e83269_d_n5, assign54100_e83269_d_n6, assign54100_e83269_d_n7, assign54100_e83269_d_n8, assign54100_e83269_d_n9, assign54100_e83269_d_n10, assign54100_e83269_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign54100_e83269;
        locals.var_xmp_dn0 = assign54100_e83269_d_n0;
        locals.var_xmp_dn2 = assign54100_e83269_d_n2;
        locals.var_xmp_dn4 = assign54100_e83269_d_n4;
        locals.var_xmp_dn5 = assign54100_e83269_d_n5;
        locals.var_xmp_dn6 = assign54100_e83269_d_n6;
        locals.var_xmp_dn7 = assign54100_e83269_d_n7;
        locals.var_xmp_dn8 = assign54100_e83269_d_n8;
        locals.var_xmp_dn9 = assign54100_e83269_d_n9;
        locals.var_xmp_dn10 = assign54100_e83269_d_n10;
        locals.var_xmp_dn13 = assign54100_e83269_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign54110_e83288,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54110_e83288;
        locals.var_m0_rv = 0.0;

        let (assign54120_e83307,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54120_e83307;
        locals.var_mm_rv = 0.0;

        let (assign54130_e83326, assign54130_e83326_d_n0, assign54130_e83326_d_n2, assign54130_e83326_d_n4, assign54130_e83326_d_n5, assign54130_e83326_d_n6, assign54130_e83326_d_n7, assign54130_e83326_d_n8, assign54130_e83326_d_n9, assign54130_e83326_d_n10, assign54130_e83326_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign54130_e83326;
        locals.var_arg_dn0 = assign54130_e83326_d_n0;
        locals.var_arg_dn2 = assign54130_e83326_d_n2;
        locals.var_arg_dn4 = assign54130_e83326_d_n4;
        locals.var_arg_dn5 = assign54130_e83326_d_n5;
        locals.var_arg_dn6 = assign54130_e83326_d_n6;
        locals.var_arg_dn7 = assign54130_e83326_d_n7;
        locals.var_arg_dn8 = assign54130_e83326_d_n8;
        locals.var_arg_dn9 = assign54130_e83326_d_n9;
        locals.var_arg_dn10 = assign54130_e83326_d_n10;
        locals.var_arg_dn13 = assign54130_e83326_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign54140_e83345, assign54140_e83345_d_n0, assign54140_e83345_d_n2, assign54140_e83345_d_n4, assign54140_e83345_d_n5, assign54140_e83345_d_n6, assign54140_e83345_d_n7, assign54140_e83345_d_n8, assign54140_e83345_d_n9, assign54140_e83345_d_n10, assign54140_e83345_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign54140_e83345;
        locals.var_dnm_dn0 = assign54140_e83345_d_n0;
        locals.var_dnm_dn2 = assign54140_e83345_d_n2;
        locals.var_dnm_dn4 = assign54140_e83345_d_n4;
        locals.var_dnm_dn5 = assign54140_e83345_d_n5;
        locals.var_dnm_dn6 = assign54140_e83345_d_n6;
        locals.var_dnm_dn7 = assign54140_e83345_d_n7;
        locals.var_dnm_dn8 = assign54140_e83345_d_n8;
        locals.var_dnm_dn9 = assign54140_e83345_d_n9;
        locals.var_dnm_dn10 = assign54140_e83345_d_n10;
        locals.var_dnm_dn13 = assign54140_e83345_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign54150_e83366, assign54150_e83366_d_n0, assign54150_e83366_d_n2, assign54150_e83366_d_n4, assign54150_e83366_d_n5, assign54150_e83366_d_n6, assign54150_e83366_d_n7, assign54150_e83366_d_n8, assign54150_e83366_d_n9, assign54150_e83366_d_n10, assign54150_e83366_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54150_e83364: f64 = (locals.var_xp * locals.var_x2);
        (assign54150_e83364, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign54150_e83366;
        locals.var_xp_dn0 = assign54150_e83366_d_n0;
        locals.var_xp_dn2 = assign54150_e83366_d_n2;
        locals.var_xp_dn4 = assign54150_e83366_d_n4;
        locals.var_xp_dn5 = assign54150_e83366_d_n5;
        locals.var_xp_dn6 = assign54150_e83366_d_n6;
        locals.var_xp_dn7 = assign54150_e83366_d_n7;
        locals.var_xp_dn8 = assign54150_e83366_d_n8;
        locals.var_xp_dn9 = assign54150_e83366_d_n9;
        locals.var_xp_dn10 = assign54150_e83366_d_n10;
        locals.var_xp_dn13 = assign54150_e83366_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign54160_e83387, assign54160_e83387_d_n0, assign54160_e83387_d_n2, assign54160_e83387_d_n4, assign54160_e83387_d_n5, assign54160_e83387_d_n6, assign54160_e83387_d_n7, assign54160_e83387_d_n8, assign54160_e83387_d_n9, assign54160_e83387_d_n10, assign54160_e83387_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54160_e83385: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54160_e83385, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign54160_e83387;
        locals.var_xmp_dn0 = assign54160_e83387_d_n0;
        locals.var_xmp_dn2 = assign54160_e83387_d_n2;
        locals.var_xmp_dn4 = assign54160_e83387_d_n4;
        locals.var_xmp_dn5 = assign54160_e83387_d_n5;
        locals.var_xmp_dn6 = assign54160_e83387_d_n6;
        locals.var_xmp_dn7 = assign54160_e83387_d_n7;
        locals.var_xmp_dn8 = assign54160_e83387_d_n8;
        locals.var_xmp_dn9 = assign54160_e83387_d_n9;
        locals.var_xmp_dn10 = assign54160_e83387_d_n10;
        locals.var_xmp_dn13 = assign54160_e83387_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign54170_e83408, assign54170_e83408_d_n0, assign54170_e83408_d_n2, assign54170_e83408_d_n4, assign54170_e83408_d_n5, assign54170_e83408_d_n6, assign54170_e83408_d_n7, assign54170_e83408_d_n8, assign54170_e83408_d_n9, assign54170_e83408_d_n10, assign54170_e83408_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54170_e83406: f64 = (locals.var_xp * locals.var_x2);
        (assign54170_e83406, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign54170_e83408;
        locals.var_xp_dn0 = assign54170_e83408_d_n0;
        locals.var_xp_dn2 = assign54170_e83408_d_n2;
        locals.var_xp_dn4 = assign54170_e83408_d_n4;
        locals.var_xp_dn5 = assign54170_e83408_d_n5;
        locals.var_xp_dn6 = assign54170_e83408_d_n6;
        locals.var_xp_dn7 = assign54170_e83408_d_n7;
        locals.var_xp_dn8 = assign54170_e83408_d_n8;
        locals.var_xp_dn9 = assign54170_e83408_d_n9;
        locals.var_xp_dn10 = assign54170_e83408_d_n10;
        locals.var_xp_dn13 = assign54170_e83408_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign54180_e83429, assign54180_e83429_d_n0, assign54180_e83429_d_n2, assign54180_e83429_d_n4, assign54180_e83429_d_n5, assign54180_e83429_d_n6, assign54180_e83429_d_n7, assign54180_e83429_d_n8, assign54180_e83429_d_n9, assign54180_e83429_d_n10, assign54180_e83429_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54180_e83427: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54180_e83427, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign54180_e83429;
        locals.var_xmp_dn0 = assign54180_e83429_d_n0;
        locals.var_xmp_dn2 = assign54180_e83429_d_n2;
        locals.var_xmp_dn4 = assign54180_e83429_d_n4;
        locals.var_xmp_dn5 = assign54180_e83429_d_n5;
        locals.var_xmp_dn6 = assign54180_e83429_d_n6;
        locals.var_xmp_dn7 = assign54180_e83429_d_n7;
        locals.var_xmp_dn8 = assign54180_e83429_d_n8;
        locals.var_xmp_dn9 = assign54180_e83429_d_n9;
        locals.var_xmp_dn10 = assign54180_e83429_d_n10;
        locals.var_xmp_dn13 = assign54180_e83429_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign54190_e83450, assign54190_e83450_d_n0, assign54190_e83450_d_n2, assign54190_e83450_d_n4, assign54190_e83450_d_n5, assign54190_e83450_d_n6, assign54190_e83450_d_n7, assign54190_e83450_d_n8, assign54190_e83450_d_n9, assign54190_e83450_d_n10, assign54190_e83450_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54190_e83448: f64 = (locals.var_xp + locals.var_xmp);
        (assign54190_e83448, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign54190_e83450;
        locals.var_arg_dn0 = assign54190_e83450_d_n0;
        locals.var_arg_dn2 = assign54190_e83450_d_n2;
        locals.var_arg_dn4 = assign54190_e83450_d_n4;
        locals.var_arg_dn5 = assign54190_e83450_d_n5;
        locals.var_arg_dn6 = assign54190_e83450_d_n6;
        locals.var_arg_dn7 = assign54190_e83450_d_n7;
        locals.var_arg_dn8 = assign54190_e83450_d_n8;
        locals.var_arg_dn9 = assign54190_e83450_d_n9;
        locals.var_arg_dn10 = assign54190_e83450_d_n10;
        locals.var_arg_dn13 = assign54190_e83450_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign54200_e83469, assign54200_e83469_d_n0, assign54200_e83469_d_n2, assign54200_e83469_d_n4, assign54200_e83469_d_n5, assign54200_e83469_d_n6, assign54200_e83469_d_n7, assign54200_e83469_d_n8, assign54200_e83469_d_n9, assign54200_e83469_d_n10, assign54200_e83469_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign54200_e83469;
        locals.var_dnm_dn0 = assign54200_e83469_d_n0;
        locals.var_dnm_dn2 = assign54200_e83469_d_n2;
        locals.var_dnm_dn4 = assign54200_e83469_d_n4;
        locals.var_dnm_dn5 = assign54200_e83469_d_n5;
        locals.var_dnm_dn6 = assign54200_e83469_d_n6;
        locals.var_dnm_dn7 = assign54200_e83469_d_n7;
        locals.var_dnm_dn8 = assign54200_e83469_d_n8;
        locals.var_dnm_dn9 = assign54200_e83469_d_n9;
        locals.var_dnm_dn10 = assign54200_e83469_d_n10;
        locals.var_dnm_dn13 = assign54200_e83469_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign54210_e83484: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1370 = assign54210_e83484;
        locals.var_guard1370_rv = 0.0;

        let assign54220_e83487: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1371 = assign54220_e83487;
        locals.var_guard1371_rv = 0.0;

        let (assign54230_e83510,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54230_e83510;
        locals.var_mm_rv = 0.0;

        let assign54240_e83513: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1372 = assign54240_e83513;
        locals.var_guard1372_rv = 0.0;

        let (assign54250_e83539,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 == 0.0)) && (locals.var_guard1372 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54250_e83539;
        locals.var_mm_rv = 0.0;

        let assign54260_e83542: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1373 = assign54260_e83542;
        locals.var_guard1373_rv = 0.0;

        let (assign54270_e83571,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54270_e83571;
        locals.var_mm_rv = 0.0;

        let assign54280_e83574: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1374 = assign54280_e83574;
        locals.var_guard1374_rv = 0.0;

        let (assign54290_e83606,) = {
    if ((((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 == 0.0)) && (locals.var_guard1374 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54290_e83606;
        locals.var_mm_rv = 0.0;

        let (assign54300_e83627,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54300_e83627;
        locals.var_m0_rv = 0.0;

        let mut assign54310_loop_guard: usize = 0;
        while {
            let assign54310_cond_e83649: f64 = if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign54310_cond_e83649 != 0.0
        } {
            assign54310_loop_guard += 1;
            assert!(assign54310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54310_body0_e83671, assign54310_body0_e83671_d_n0, assign54310_body0_e83671_d_n2, assign54310_body0_e83671_d_n4, assign54310_body0_e83671_d_n5, assign54310_body0_e83671_d_n6, assign54310_body0_e83671_d_n7, assign54310_body0_e83671_d_n8, assign54310_body0_e83671_d_n9, assign54310_body0_e83671_d_n10, assign54310_body0_e83671_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign54310_body0_e83669: f64 = (locals.var_dnm).sqrt();
        (assign54310_body0_e83669, (locals.var_dnm_dn0 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn2 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn4 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn5 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn6 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn7 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn8 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn9 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn10 / (2.0 * assign54310_body0_e83669)), (locals.var_dnm_dn13 / (2.0 * assign54310_body0_e83669)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign54310_body0_e83671;
            locals.var_dnm_dn0 = assign54310_body0_e83671_d_n0;
            locals.var_dnm_dn2 = assign54310_body0_e83671_d_n2;
            locals.var_dnm_dn4 = assign54310_body0_e83671_d_n4;
            locals.var_dnm_dn5 = assign54310_body0_e83671_d_n5;
            locals.var_dnm_dn6 = assign54310_body0_e83671_d_n6;
            locals.var_dnm_dn7 = assign54310_body0_e83671_d_n7;
            locals.var_dnm_dn8 = assign54310_body0_e83671_d_n8;
            locals.var_dnm_dn9 = assign54310_body0_e83671_d_n9;
            locals.var_dnm_dn10 = assign54310_body0_e83671_d_n10;
            locals.var_dnm_dn13 = assign54310_body0_e83671_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign54310_body1_e83694,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign54310_body1_e83692: f64 = (locals.var_m0 + 1.0);
        (assign54310_body1_e83692,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54310_body1_e83694;
            locals.var_m0_rv = 0.0;
        }

        let (assign54320_e83727, assign54320_e83727_d_n0, assign54320_e83727_d_n2, assign54320_e83727_d_n4, assign54320_e83727_d_n5, assign54320_e83727_d_n6, assign54320_e83727_d_n7, assign54320_e83727_d_n8, assign54320_e83727_d_n9, assign54320_e83727_d_n10, assign54320_e83727_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) && (locals.var_guard1370 == 0.0)) {
        let (assign54320_e83725, assign54320_e83725_d_n0, assign54320_e83725_d_n2, assign54320_e83725_d_n4, assign54320_e83725_d_n5, assign54320_e83725_d_n6, assign54320_e83725_d_n7, assign54320_e83725_d_n8, assign54320_e83725_d_n9, assign54320_e83725_d_n10, assign54320_e83725_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign54320_e83722: f64 = (2.0 * 2.0);
                let assign54320_e83723: f64 = (1.0 / assign54320_e83722);
                let assign54320_e83724: f64 = (locals.var_dnm).powf(assign54320_e83723);
                (assign54320_e83724, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn0)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn2)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn4)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn5)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn6)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn7)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn8)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn9)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn10)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54320_e83723) as f64).is_finite() && ((assign54320_e83723) as f64).fract() == 0.0 { if assign54320_e83723 == 0.0 { 0.0 } else { (assign54320_e83723 * ((locals.var_dnm).powf(assign54320_e83723 - 1.0) * locals.var_dnm_dn13)) } } else { (assign54320_e83724 * (assign54320_e83723 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign54320_e83725, assign54320_e83725_d_n0, assign54320_e83725_d_n2, assign54320_e83725_d_n4, assign54320_e83725_d_n5, assign54320_e83725_d_n6, assign54320_e83725_d_n7, assign54320_e83725_d_n8, assign54320_e83725_d_n9, assign54320_e83725_d_n10, assign54320_e83725_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign54320_e83727;
        locals.var_dnm_dn0 = assign54320_e83727_d_n0;
        locals.var_dnm_dn2 = assign54320_e83727_d_n2;
        locals.var_dnm_dn4 = assign54320_e83727_d_n4;
        locals.var_dnm_dn5 = assign54320_e83727_d_n5;
        locals.var_dnm_dn6 = assign54320_e83727_d_n6;
        locals.var_dnm_dn7 = assign54320_e83727_d_n7;
        locals.var_dnm_dn8 = assign54320_e83727_d_n8;
        locals.var_dnm_dn9 = assign54320_e83727_d_n9;
        locals.var_dnm_dn10 = assign54320_e83727_d_n10;
        locals.var_dnm_dn13 = assign54320_e83727_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign54330_e83748, assign54330_e83748_d_n0, assign54330_e83748_d_n2, assign54330_e83748_d_n4, assign54330_e83748_d_n5, assign54330_e83748_d_n6, assign54330_e83748_d_n7, assign54330_e83748_d_n8, assign54330_e83748_d_n9, assign54330_e83748_d_n10, assign54330_e83748_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54330_e83746: f64 = (1.0 / locals.var_dnm);
        (assign54330_e83746, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign54330_e83748;
        locals.var_dnm_dn0 = assign54330_e83748_d_n0;
        locals.var_dnm_dn2 = assign54330_e83748_d_n2;
        locals.var_dnm_dn4 = assign54330_e83748_d_n4;
        locals.var_dnm_dn5 = assign54330_e83748_d_n5;
        locals.var_dnm_dn6 = assign54330_e83748_d_n6;
        locals.var_dnm_dn7 = assign54330_e83748_d_n7;
        locals.var_dnm_dn8 = assign54330_e83748_d_n8;
        locals.var_dnm_dn9 = assign54330_e83748_d_n9;
        locals.var_dnm_dn10 = assign54330_e83748_d_n10;
        locals.var_dnm_dn13 = assign54330_e83748_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign54340_e83771, assign54340_e83771_d_n0, assign54340_e83771_d_n2, assign54340_e83771_d_n4, assign54340_e83771_d_n5, assign54340_e83771_d_n6, assign54340_e83771_d_n7, assign54340_e83771_d_n8, assign54340_e83771_d_n9, assign54340_e83771_d_n10, assign54340_e83771_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54340_e83767: f64 = (locals.var_tmf1 * 0.2);
        let assign54340_e83769: f64 = (assign54340_e83767 * locals.var_dnm);
        (assign54340_e83769, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.2) * locals.var_dnm) + (assign54340_e83767 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign54340_e83771;
        locals.var_tmf0_dn0 = assign54340_e83771_d_n0;
        locals.var_tmf0_dn2 = assign54340_e83771_d_n2;
        locals.var_tmf0_dn4 = assign54340_e83771_d_n4;
        locals.var_tmf0_dn5 = assign54340_e83771_d_n5;
        locals.var_tmf0_dn6 = assign54340_e83771_d_n6;
        locals.var_tmf0_dn7 = assign54340_e83771_d_n7;
        locals.var_tmf0_dn8 = assign54340_e83771_d_n8;
        locals.var_tmf0_dn9 = assign54340_e83771_d_n9;
        locals.var_tmf0_dn10 = assign54340_e83771_d_n10;
        locals.var_tmf0_dn13 = assign54340_e83771_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign54350_e83796, assign54350_e83796_d_n0, assign54350_e83796_d_n2, assign54350_e83796_d_n4, assign54350_e83796_d_n5, assign54350_e83796_d_n6, assign54350_e83796_d_n7, assign54350_e83796_d_n8, assign54350_e83796_d_n9, assign54350_e83796_d_n10, assign54350_e83796_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54350_e83790: f64 = (0.2 * locals.var_xmp);
        let assign54350_e83792: f64 = (assign54350_e83790 * locals.var_dnm);
        let assign54350_e83794: f64 = (assign54350_e83792 / locals.var_arg);
        (assign54350_e83794, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn0)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn2)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn4)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn5)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn6)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn7)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn8)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn9)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn10)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn13) * locals.var_dnm) + (assign54350_e83790 * locals.var_dnm_dn13)) * locals.var_arg) - (assign54350_e83792 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign54350_e83796;
        locals.var_t0_dn0 = assign54350_e83796_d_n0;
        locals.var_t0_dn2 = assign54350_e83796_d_n2;
        locals.var_t0_dn4 = assign54350_e83796_d_n4;
        locals.var_t0_dn5 = assign54350_e83796_d_n5;
        locals.var_t0_dn6 = assign54350_e83796_d_n6;
        locals.var_t0_dn7 = assign54350_e83796_d_n7;
        locals.var_t0_dn8 = assign54350_e83796_d_n8;
        locals.var_t0_dn9 = assign54350_e83796_d_n9;
        locals.var_t0_dn10 = assign54350_e83796_d_n10;
        locals.var_t0_dn13 = assign54350_e83796_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_190(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign54360_e83819, assign54360_e83819_d_n0, assign54360_e83819_d_n2, assign54360_e83819_d_n4, assign54360_e83819_d_n5, assign54360_e83819_d_n6, assign54360_e83819_d_n7, assign54360_e83819_d_n8, assign54360_e83819_d_n9, assign54360_e83819_d_n10, assign54360_e83819_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign54360_e83815: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54360_e83817: f64 = (assign54360_e83815 - locals.var_tmf0);
        (assign54360_e83817, (locals.var_ps0dep0_dn0 - locals.var_tmf0_dn0), (locals.var_ps0dep0_dn2 - locals.var_tmf0_dn2), (locals.var_ps0dep0_dn4 - locals.var_tmf0_dn4), (locals.var_ps0dep0_dn5 - locals.var_tmf0_dn5), (locals.var_ps0dep0_dn6 - locals.var_tmf0_dn6), (locals.var_ps0dep0_dn7 - locals.var_tmf0_dn7), (locals.var_ps0dep0_dn8 - locals.var_tmf0_dn8), (locals.var_ps0dep0_dn9 - locals.var_tmf0_dn9), (locals.var_ps0dep0_dn10 - locals.var_tmf0_dn10), (locals.var_ps0dep0_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign54360_e83819;
        locals.var_ps0dep_dn0 = assign54360_e83819_d_n0;
        locals.var_ps0dep_dn2 = assign54360_e83819_d_n2;
        locals.var_ps0dep_dn4 = assign54360_e83819_d_n4;
        locals.var_ps0dep_dn5 = assign54360_e83819_d_n5;
        locals.var_ps0dep_dn6 = assign54360_e83819_d_n6;
        locals.var_ps0dep_dn7 = assign54360_e83819_d_n7;
        locals.var_ps0dep_dn8 = assign54360_e83819_d_n8;
        locals.var_ps0dep_dn9 = assign54360_e83819_d_n9;
        locals.var_ps0dep_dn10 = assign54360_e83819_d_n10;
        locals.var_ps0dep_dn13 = assign54360_e83819_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign54370_e83838, assign54370_e83838_d_n0, assign54370_e83838_d_n2, assign54370_e83838_d_n4, assign54370_e83838_d_n5, assign54370_e83838_d_n6, assign54370_e83838_d_n7, assign54370_e83838_d_n8, assign54370_e83838_d_n9, assign54370_e83838_d_n10, assign54370_e83838_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign54370_e83838;
        locals.var_t0_dn0 = assign54370_e83838_d_n0;
        locals.var_t0_dn2 = assign54370_e83838_d_n2;
        locals.var_t0_dn4 = assign54370_e83838_d_n4;
        locals.var_t0_dn5 = assign54370_e83838_d_n5;
        locals.var_t0_dn6 = assign54370_e83838_d_n6;
        locals.var_t0_dn7 = assign54370_e83838_d_n7;
        locals.var_t0_dn8 = assign54370_e83838_d_n8;
        locals.var_t0_dn9 = assign54370_e83838_d_n9;
        locals.var_t0_dn10 = assign54370_e83838_d_n10;
        locals.var_t0_dn13 = assign54370_e83838_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign54380_e83858, assign54380_e83858_d_n0, assign54380_e83858_d_n2, assign54380_e83858_d_n4, assign54380_e83858_d_n5, assign54380_e83858_d_n6, assign54380_e83858_d_n7, assign54380_e83858_d_n8, assign54380_e83858_d_n9, assign54380_e83858_d_n10, assign54380_e83858_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign54380_e83858;
        locals.var_ps0dep_dn0 = assign54380_e83858_d_n0;
        locals.var_ps0dep_dn2 = assign54380_e83858_d_n2;
        locals.var_ps0dep_dn4 = assign54380_e83858_d_n4;
        locals.var_ps0dep_dn5 = assign54380_e83858_d_n5;
        locals.var_ps0dep_dn6 = assign54380_e83858_d_n6;
        locals.var_ps0dep_dn7 = assign54380_e83858_d_n7;
        locals.var_ps0dep_dn8 = assign54380_e83858_d_n8;
        locals.var_ps0dep_dn9 = assign54380_e83858_d_n9;
        locals.var_ps0dep_dn10 = assign54380_e83858_d_n10;
        locals.var_ps0dep_dn13 = assign54380_e83858_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign54390_e83878, assign54390_e83878_d_n0, assign54390_e83878_d_n2, assign54390_e83878_d_n4, assign54390_e83878_d_n5, assign54390_e83878_d_n6, assign54390_e83878_d_n7, assign54390_e83878_d_n8, assign54390_e83878_d_n9, assign54390_e83878_d_n10, assign54390_e83878_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign54390_e83878;
        locals.var_t0_dn0 = assign54390_e83878_d_n0;
        locals.var_t0_dn2 = assign54390_e83878_d_n2;
        locals.var_t0_dn4 = assign54390_e83878_d_n4;
        locals.var_t0_dn5 = assign54390_e83878_d_n5;
        locals.var_t0_dn6 = assign54390_e83878_d_n6;
        locals.var_t0_dn7 = assign54390_e83878_d_n7;
        locals.var_t0_dn8 = assign54390_e83878_d_n8;
        locals.var_t0_dn9 = assign54390_e83878_d_n9;
        locals.var_t0_dn10 = assign54390_e83878_d_n10;
        locals.var_t0_dn13 = assign54390_e83878_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign54400_e83892, assign54400_e83892_d_n0, assign54400_e83892_d_n2, assign54400_e83892_d_n4, assign54400_e83892_d_n5, assign54400_e83892_d_n6, assign54400_e83892_d_n7, assign54400_e83892_d_n8, assign54400_e83892_d_n9, assign54400_e83892_d_n10, assign54400_e83892_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    }
};
        locals.var_ps0_res = assign54400_e83892;
        locals.var_ps0_res_dn0 = assign54400_e83892_d_n0;
        locals.var_ps0_res_dn2 = assign54400_e83892_d_n2;
        locals.var_ps0_res_dn4 = assign54400_e83892_d_n4;
        locals.var_ps0_res_dn5 = assign54400_e83892_d_n5;
        locals.var_ps0_res_dn6 = assign54400_e83892_d_n6;
        locals.var_ps0_res_dn7 = assign54400_e83892_d_n7;
        locals.var_ps0_res_dn8 = assign54400_e83892_d_n8;
        locals.var_ps0_res_dn9 = assign54400_e83892_d_n9;
        locals.var_ps0_res_dn10 = assign54400_e83892_d_n10;
        locals.var_ps0_res_dn13 = assign54400_e83892_d_n13;
        locals.var_ps0_res_rv = 0.0;

        let (assign54410_e83911,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let (assign54410_e83909,) = {
            if (1e-6 >= p.p407) {
                (1e-6,)
            } else {
                (p.p407,)
            }
        };
        (assign54410_e83909,)
    } else {
        (locals.var_vgpdep_dlt__blk1142,)
    }
};
        locals.var_vgpdep_dlt__blk1142 = assign54410_e83911;
        locals.var_vgpdep_dlt__blk1142_rv = 0.0;

        let assign54420_e83915: f64 = (-locals.var_vgpdep_dlt__blk1142);
        let assign54420_e83920: f64 = if ((locals.var_ps0_res > assign54420_e83915) && (locals.var_vgpdep_dlt__blk1142 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1375 = assign54420_e83920;
        locals.var_guard1375_rv = 0.0;

        let (assign54430_e83940, assign54430_e83940_d_n0, assign54430_e83940_d_n2, assign54430_e83940_d_n4, assign54430_e83940_d_n5, assign54430_e83940_d_n6, assign54430_e83940_d_n7, assign54430_e83940_d_n8, assign54430_e83940_d_n9, assign54430_e83940_d_n10, assign54430_e83940_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54430_e83936: f64 = locals.var_ps0_res;
        let assign54430_e83938: f64 = (assign54430_e83936 + locals.var_vgpdep_dlt__blk1142);
        (assign54430_e83938, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign54430_e83940;
        locals.var_tmf1_dn0 = assign54430_e83940_d_n0;
        locals.var_tmf1_dn2 = assign54430_e83940_d_n2;
        locals.var_tmf1_dn4 = assign54430_e83940_d_n4;
        locals.var_tmf1_dn5 = assign54430_e83940_d_n5;
        locals.var_tmf1_dn6 = assign54430_e83940_d_n6;
        locals.var_tmf1_dn7 = assign54430_e83940_d_n7;
        locals.var_tmf1_dn8 = assign54430_e83940_d_n8;
        locals.var_tmf1_dn9 = assign54430_e83940_d_n9;
        locals.var_tmf1_dn10 = assign54430_e83940_d_n10;
        locals.var_tmf1_dn13 = assign54430_e83940_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign54440_e83958, assign54440_e83958_d_n0, assign54440_e83958_d_n2, assign54440_e83958_d_n4, assign54440_e83958_d_n5, assign54440_e83958_d_n6, assign54440_e83958_d_n7, assign54440_e83958_d_n8, assign54440_e83958_d_n9, assign54440_e83958_d_n10, assign54440_e83958_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54440_e83956: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign54440_e83956, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign54440_e83958;
        locals.var_x2_dn0 = assign54440_e83958_d_n0;
        locals.var_x2_dn2 = assign54440_e83958_d_n2;
        locals.var_x2_dn4 = assign54440_e83958_d_n4;
        locals.var_x2_dn5 = assign54440_e83958_d_n5;
        locals.var_x2_dn6 = assign54440_e83958_d_n6;
        locals.var_x2_dn7 = assign54440_e83958_d_n7;
        locals.var_x2_dn8 = assign54440_e83958_d_n8;
        locals.var_x2_dn9 = assign54440_e83958_d_n9;
        locals.var_x2_dn10 = assign54440_e83958_d_n10;
        locals.var_x2_dn13 = assign54440_e83958_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign54450_e83976, assign54450_e83976_d_n0, assign54450_e83976_d_n2, assign54450_e83976_d_n4, assign54450_e83976_d_n5, assign54450_e83976_d_n6, assign54450_e83976_d_n7, assign54450_e83976_d_n8, assign54450_e83976_d_n9, assign54450_e83976_d_n10, assign54450_e83976_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54450_e83974: f64 = (locals.var_vgpdep_dlt__blk1142 * locals.var_vgpdep_dlt__blk1142);
        (assign54450_e83974, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign54450_e83976;
        locals.var_xmax2_dn0 = assign54450_e83976_d_n0;
        locals.var_xmax2_dn2 = assign54450_e83976_d_n2;
        locals.var_xmax2_dn4 = assign54450_e83976_d_n4;
        locals.var_xmax2_dn5 = assign54450_e83976_d_n5;
        locals.var_xmax2_dn6 = assign54450_e83976_d_n6;
        locals.var_xmax2_dn7 = assign54450_e83976_d_n7;
        locals.var_xmax2_dn8 = assign54450_e83976_d_n8;
        locals.var_xmax2_dn9 = assign54450_e83976_d_n9;
        locals.var_xmax2_dn10 = assign54450_e83976_d_n10;
        locals.var_xmax2_dn13 = assign54450_e83976_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign54460_e83992, assign54460_e83992_d_n0, assign54460_e83992_d_n2, assign54460_e83992_d_n4, assign54460_e83992_d_n5, assign54460_e83992_d_n6, assign54460_e83992_d_n7, assign54460_e83992_d_n8, assign54460_e83992_d_n9, assign54460_e83992_d_n10, assign54460_e83992_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign54460_e83992;
        locals.var_xp_dn0 = assign54460_e83992_d_n0;
        locals.var_xp_dn2 = assign54460_e83992_d_n2;
        locals.var_xp_dn4 = assign54460_e83992_d_n4;
        locals.var_xp_dn5 = assign54460_e83992_d_n5;
        locals.var_xp_dn6 = assign54460_e83992_d_n6;
        locals.var_xp_dn7 = assign54460_e83992_d_n7;
        locals.var_xp_dn8 = assign54460_e83992_d_n8;
        locals.var_xp_dn9 = assign54460_e83992_d_n9;
        locals.var_xp_dn10 = assign54460_e83992_d_n10;
        locals.var_xp_dn13 = assign54460_e83992_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign54470_e84008, assign54470_e84008_d_n0, assign54470_e84008_d_n2, assign54470_e84008_d_n4, assign54470_e84008_d_n5, assign54470_e84008_d_n6, assign54470_e84008_d_n7, assign54470_e84008_d_n8, assign54470_e84008_d_n9, assign54470_e84008_d_n10, assign54470_e84008_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign54470_e84008;
        locals.var_xmp_dn0 = assign54470_e84008_d_n0;
        locals.var_xmp_dn2 = assign54470_e84008_d_n2;
        locals.var_xmp_dn4 = assign54470_e84008_d_n4;
        locals.var_xmp_dn5 = assign54470_e84008_d_n5;
        locals.var_xmp_dn6 = assign54470_e84008_d_n6;
        locals.var_xmp_dn7 = assign54470_e84008_d_n7;
        locals.var_xmp_dn8 = assign54470_e84008_d_n8;
        locals.var_xmp_dn9 = assign54470_e84008_d_n9;
        locals.var_xmp_dn10 = assign54470_e84008_d_n10;
        locals.var_xmp_dn13 = assign54470_e84008_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign54480_e84024,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54480_e84024;
        locals.var_m0_rv = 0.0;

        let (assign54490_e84040,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54490_e84040;
        locals.var_mm_rv = 0.0;

        let (assign54500_e84056, assign54500_e84056_d_n0, assign54500_e84056_d_n2, assign54500_e84056_d_n4, assign54500_e84056_d_n5, assign54500_e84056_d_n6, assign54500_e84056_d_n7, assign54500_e84056_d_n8, assign54500_e84056_d_n9, assign54500_e84056_d_n10, assign54500_e84056_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign54500_e84056;
        locals.var_arg_dn0 = assign54500_e84056_d_n0;
        locals.var_arg_dn2 = assign54500_e84056_d_n2;
        locals.var_arg_dn4 = assign54500_e84056_d_n4;
        locals.var_arg_dn5 = assign54500_e84056_d_n5;
        locals.var_arg_dn6 = assign54500_e84056_d_n6;
        locals.var_arg_dn7 = assign54500_e84056_d_n7;
        locals.var_arg_dn8 = assign54500_e84056_d_n8;
        locals.var_arg_dn9 = assign54500_e84056_d_n9;
        locals.var_arg_dn10 = assign54500_e84056_d_n10;
        locals.var_arg_dn13 = assign54500_e84056_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign54510_e84072, assign54510_e84072_d_n0, assign54510_e84072_d_n2, assign54510_e84072_d_n4, assign54510_e84072_d_n5, assign54510_e84072_d_n6, assign54510_e84072_d_n7, assign54510_e84072_d_n8, assign54510_e84072_d_n9, assign54510_e84072_d_n10, assign54510_e84072_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign54510_e84072;
        locals.var_dnm_dn0 = assign54510_e84072_d_n0;
        locals.var_dnm_dn2 = assign54510_e84072_d_n2;
        locals.var_dnm_dn4 = assign54510_e84072_d_n4;
        locals.var_dnm_dn5 = assign54510_e84072_d_n5;
        locals.var_dnm_dn6 = assign54510_e84072_d_n6;
        locals.var_dnm_dn7 = assign54510_e84072_d_n7;
        locals.var_dnm_dn8 = assign54510_e84072_d_n8;
        locals.var_dnm_dn9 = assign54510_e84072_d_n9;
        locals.var_dnm_dn10 = assign54510_e84072_d_n10;
        locals.var_dnm_dn13 = assign54510_e84072_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign54520_e84088,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54520_e84088;
        locals.var_m0_rv = 0.0;

        let mut assign54530_loop_guard: usize = 0;
        while {
            let assign54530_cond_e84105: f64 = if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw__blk1143)) { 1.0 } else { 0.0 };
            assign54530_cond_e84105 != 0.0
        } {
            assign54530_loop_guard += 1;
            assert!(assign54530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54530_body0_e84123, assign54530_body0_e84123_d_n0, assign54530_body0_e84123_d_n2, assign54530_body0_e84123_d_n4, assign54530_body0_e84123_d_n5, assign54530_body0_e84123_d_n6, assign54530_body0_e84123_d_n7, assign54530_body0_e84123_d_n8, assign54530_body0_e84123_d_n9, assign54530_body0_e84123_d_n10, assign54530_body0_e84123_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54530_body0_e84121: f64 = (locals.var_xp * locals.var_x2);
        (assign54530_body0_e84121, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign54530_body0_e84123;
            locals.var_xp_dn0 = assign54530_body0_e84123_d_n0;
            locals.var_xp_dn2 = assign54530_body0_e84123_d_n2;
            locals.var_xp_dn4 = assign54530_body0_e84123_d_n4;
            locals.var_xp_dn5 = assign54530_body0_e84123_d_n5;
            locals.var_xp_dn6 = assign54530_body0_e84123_d_n6;
            locals.var_xp_dn7 = assign54530_body0_e84123_d_n7;
            locals.var_xp_dn8 = assign54530_body0_e84123_d_n8;
            locals.var_xp_dn9 = assign54530_body0_e84123_d_n9;
            locals.var_xp_dn10 = assign54530_body0_e84123_d_n10;
            locals.var_xp_dn13 = assign54530_body0_e84123_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign54530_body1_e84141, assign54530_body1_e84141_d_n0, assign54530_body1_e84141_d_n2, assign54530_body1_e84141_d_n4, assign54530_body1_e84141_d_n5, assign54530_body1_e84141_d_n6, assign54530_body1_e84141_d_n7, assign54530_body1_e84141_d_n8, assign54530_body1_e84141_d_n9, assign54530_body1_e84141_d_n10, assign54530_body1_e84141_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54530_body1_e84139: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54530_body1_e84139, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign54530_body1_e84141;
            locals.var_xmp_dn0 = assign54530_body1_e84141_d_n0;
            locals.var_xmp_dn2 = assign54530_body1_e84141_d_n2;
            locals.var_xmp_dn4 = assign54530_body1_e84141_d_n4;
            locals.var_xmp_dn5 = assign54530_body1_e84141_d_n5;
            locals.var_xmp_dn6 = assign54530_body1_e84141_d_n6;
            locals.var_xmp_dn7 = assign54530_body1_e84141_d_n7;
            locals.var_xmp_dn8 = assign54530_body1_e84141_d_n8;
            locals.var_xmp_dn9 = assign54530_body1_e84141_d_n9;
            locals.var_xmp_dn10 = assign54530_body1_e84141_d_n10;
            locals.var_xmp_dn13 = assign54530_body1_e84141_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign54530_body2_e84159,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54530_body2_e84157: f64 = (locals.var_m0 + 1.0);
        (assign54530_body2_e84157,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54530_body2_e84159;
            locals.var_m0_rv = 0.0;
        }

        let (assign54540_e84177, assign54540_e84177_d_n0, assign54540_e84177_d_n2, assign54540_e84177_d_n4, assign54540_e84177_d_n5, assign54540_e84177_d_n6, assign54540_e84177_d_n7, assign54540_e84177_d_n8, assign54540_e84177_d_n9, assign54540_e84177_d_n10, assign54540_e84177_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54540_e84175: f64 = (locals.var_xp + locals.var_xmp);
        (assign54540_e84175, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign54540_e84177;
        locals.var_arg_dn0 = assign54540_e84177_d_n0;
        locals.var_arg_dn2 = assign54540_e84177_d_n2;
        locals.var_arg_dn4 = assign54540_e84177_d_n4;
        locals.var_arg_dn5 = assign54540_e84177_d_n5;
        locals.var_arg_dn6 = assign54540_e84177_d_n6;
        locals.var_arg_dn7 = assign54540_e84177_d_n7;
        locals.var_arg_dn8 = assign54540_e84177_d_n8;
        locals.var_arg_dn9 = assign54540_e84177_d_n9;
        locals.var_arg_dn10 = assign54540_e84177_d_n10;
        locals.var_arg_dn13 = assign54540_e84177_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign54550_e84193, assign54550_e84193_d_n0, assign54550_e84193_d_n2, assign54550_e84193_d_n4, assign54550_e84193_d_n5, assign54550_e84193_d_n6, assign54550_e84193_d_n7, assign54550_e84193_d_n8, assign54550_e84193_d_n9, assign54550_e84193_d_n10, assign54550_e84193_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign54550_e84193;
        locals.var_dnm_dn0 = assign54550_e84193_d_n0;
        locals.var_dnm_dn2 = assign54550_e84193_d_n2;
        locals.var_dnm_dn4 = assign54550_e84193_d_n4;
        locals.var_dnm_dn5 = assign54550_e84193_d_n5;
        locals.var_dnm_dn6 = assign54550_e84193_d_n6;
        locals.var_dnm_dn7 = assign54550_e84193_d_n7;
        locals.var_dnm_dn8 = assign54550_e84193_d_n8;
        locals.var_dnm_dn9 = assign54550_e84193_d_n9;
        locals.var_dnm_dn10 = assign54550_e84193_d_n10;
        locals.var_dnm_dn13 = assign54550_e84193_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign54560_e84208: f64 = if ((((locals.var_vgpdep_pw__blk1143 == 1.0) || (locals.var_vgpdep_pw__blk1143 == 2.0)) || (locals.var_vgpdep_pw__blk1143 == 4.0)) || (locals.var_vgpdep_pw__blk1143 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1376 = assign54560_e84208;
        locals.var_guard1376_rv = 0.0;

        let assign54570_e84211: f64 = if locals.var_vgpdep_pw__blk1143 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1377 = assign54570_e84211;
        locals.var_guard1377_rv = 0.0;

        let (assign54580_e84231,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 != 0.0)) && (locals.var_guard1377 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54580_e84231;
        locals.var_mm_rv = 0.0;

        let assign54590_e84234: f64 = if locals.var_vgpdep_pw__blk1143 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1378 = assign54590_e84234;
        locals.var_guard1378_rv = 0.0;

        let (assign54600_e84257,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 != 0.0)) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54600_e84257;
        locals.var_mm_rv = 0.0;

        let assign54610_e84260: f64 = if locals.var_vgpdep_pw__blk1143 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1379 = assign54610_e84260;
        locals.var_guard1379_rv = 0.0;

        let (assign54620_e84286,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 != 0.0)) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54620_e84286;
        locals.var_mm_rv = 0.0;

        let assign54630_e84289: f64 = if locals.var_vgpdep_pw__blk1143 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1380 = assign54630_e84289;
        locals.var_guard1380_rv = 0.0;

        let (assign54640_e84318,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 != 0.0)) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 == 0.0)) && (locals.var_guard1379 == 0.0)) && (locals.var_guard1380 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54640_e84318;
        locals.var_mm_rv = 0.0;

        let (assign54650_e84336,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54650_e84336;
        locals.var_m0_rv = 0.0;

        let mut assign54660_loop_guard: usize = 0;
        while {
            let assign54660_cond_e84355: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign54660_cond_e84355 != 0.0
        } {
            assign54660_loop_guard += 1;
            assert!(assign54660_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54660_body0_e84374, assign54660_body0_e84374_d_n0, assign54660_body0_e84374_d_n2, assign54660_body0_e84374_d_n4, assign54660_body0_e84374_d_n5, assign54660_body0_e84374_d_n6, assign54660_body0_e84374_d_n7, assign54660_body0_e84374_d_n8, assign54660_body0_e84374_d_n9, assign54660_body0_e84374_d_n10, assign54660_body0_e84374_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 != 0.0)) {
        let assign54660_body0_e84372: f64 = (locals.var_dnm).sqrt();
        (assign54660_body0_e84372, (locals.var_dnm_dn0 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn2 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn4 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn5 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn6 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn7 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn8 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn9 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn10 / (2.0 * assign54660_body0_e84372)), (locals.var_dnm_dn13 / (2.0 * assign54660_body0_e84372)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign54660_body0_e84374;
            locals.var_dnm_dn0 = assign54660_body0_e84374_d_n0;
            locals.var_dnm_dn2 = assign54660_body0_e84374_d_n2;
            locals.var_dnm_dn4 = assign54660_body0_e84374_d_n4;
            locals.var_dnm_dn5 = assign54660_body0_e84374_d_n5;
            locals.var_dnm_dn6 = assign54660_body0_e84374_d_n6;
            locals.var_dnm_dn7 = assign54660_body0_e84374_d_n7;
            locals.var_dnm_dn8 = assign54660_body0_e84374_d_n8;
            locals.var_dnm_dn9 = assign54660_body0_e84374_d_n9;
            locals.var_dnm_dn10 = assign54660_body0_e84374_d_n10;
            locals.var_dnm_dn13 = assign54660_body0_e84374_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign54660_body1_e84394,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 != 0.0)) {
        let assign54660_body1_e84392: f64 = (locals.var_m0 + 1.0);
        (assign54660_body1_e84392,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54660_body1_e84394;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_191(
        locals: &mut StampLocals,
    ) {
        let (assign54670_e84424, assign54670_e84424_d_n0, assign54670_e84424_d_n2, assign54670_e84424_d_n4, assign54670_e84424_d_n5, assign54670_e84424_d_n6, assign54670_e84424_d_n7, assign54670_e84424_d_n8, assign54670_e84424_d_n9, assign54670_e84424_d_n10, assign54670_e84424_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) && (locals.var_guard1376 == 0.0)) {
        let (assign54670_e84422, assign54670_e84422_d_n0, assign54670_e84422_d_n2, assign54670_e84422_d_n4, assign54670_e84422_d_n5, assign54670_e84422_d_n6, assign54670_e84422_d_n7, assign54670_e84422_d_n8, assign54670_e84422_d_n9, assign54670_e84422_d_n10, assign54670_e84422_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign54670_e84419: f64 = (2.0 * locals.var_vgpdep_pw__blk1143);
                let assign54670_e84420: f64 = (1.0 / assign54670_e84419);
                let assign54670_e84421: f64 = (locals.var_dnm).powf(assign54670_e84420);
                (assign54670_e84421, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn0)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn2)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn4)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn5)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn6)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn7)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn8)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn9)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn10)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54670_e84420) as f64).is_finite() && ((assign54670_e84420) as f64).fract() == 0.0 { if assign54670_e84420 == 0.0 { 0.0 } else { (assign54670_e84420 * ((locals.var_dnm).powf(assign54670_e84420 - 1.0) * locals.var_dnm_dn13)) } } else { (assign54670_e84421 * (assign54670_e84420 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign54670_e84422, assign54670_e84422_d_n0, assign54670_e84422_d_n2, assign54670_e84422_d_n4, assign54670_e84422_d_n5, assign54670_e84422_d_n6, assign54670_e84422_d_n7, assign54670_e84422_d_n8, assign54670_e84422_d_n9, assign54670_e84422_d_n10, assign54670_e84422_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign54670_e84424;
        locals.var_dnm_dn0 = assign54670_e84424_d_n0;
        locals.var_dnm_dn2 = assign54670_e84424_d_n2;
        locals.var_dnm_dn4 = assign54670_e84424_d_n4;
        locals.var_dnm_dn5 = assign54670_e84424_d_n5;
        locals.var_dnm_dn6 = assign54670_e84424_d_n6;
        locals.var_dnm_dn7 = assign54670_e84424_d_n7;
        locals.var_dnm_dn8 = assign54670_e84424_d_n8;
        locals.var_dnm_dn9 = assign54670_e84424_d_n9;
        locals.var_dnm_dn10 = assign54670_e84424_d_n10;
        locals.var_dnm_dn13 = assign54670_e84424_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign54680_e84442, assign54680_e84442_d_n0, assign54680_e84442_d_n2, assign54680_e84442_d_n4, assign54680_e84442_d_n5, assign54680_e84442_d_n6, assign54680_e84442_d_n7, assign54680_e84442_d_n8, assign54680_e84442_d_n9, assign54680_e84442_d_n10, assign54680_e84442_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54680_e84440: f64 = (1.0 / locals.var_dnm);
        (assign54680_e84440, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign54680_e84442;
        locals.var_dnm_dn0 = assign54680_e84442_d_n0;
        locals.var_dnm_dn2 = assign54680_e84442_d_n2;
        locals.var_dnm_dn4 = assign54680_e84442_d_n4;
        locals.var_dnm_dn5 = assign54680_e84442_d_n5;
        locals.var_dnm_dn6 = assign54680_e84442_d_n6;
        locals.var_dnm_dn7 = assign54680_e84442_d_n7;
        locals.var_dnm_dn8 = assign54680_e84442_d_n8;
        locals.var_dnm_dn9 = assign54680_e84442_d_n9;
        locals.var_dnm_dn10 = assign54680_e84442_d_n10;
        locals.var_dnm_dn13 = assign54680_e84442_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign54690_e84462, assign54690_e84462_d_n0, assign54690_e84462_d_n2, assign54690_e84462_d_n4, assign54690_e84462_d_n5, assign54690_e84462_d_n6, assign54690_e84462_d_n7, assign54690_e84462_d_n8, assign54690_e84462_d_n9, assign54690_e84462_d_n10, assign54690_e84462_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54690_e84458: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt__blk1142);
        let assign54690_e84460: f64 = (assign54690_e84458 * locals.var_dnm);
        (assign54690_e84460, (((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * locals.var_vgpdep_dlt__blk1142) * locals.var_dnm) + (assign54690_e84458 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign54690_e84462;
        locals.var_tmf0_dn0 = assign54690_e84462_d_n0;
        locals.var_tmf0_dn2 = assign54690_e84462_d_n2;
        locals.var_tmf0_dn4 = assign54690_e84462_d_n4;
        locals.var_tmf0_dn5 = assign54690_e84462_d_n5;
        locals.var_tmf0_dn6 = assign54690_e84462_d_n6;
        locals.var_tmf0_dn7 = assign54690_e84462_d_n7;
        locals.var_tmf0_dn8 = assign54690_e84462_d_n8;
        locals.var_tmf0_dn9 = assign54690_e84462_d_n9;
        locals.var_tmf0_dn10 = assign54690_e84462_d_n10;
        locals.var_tmf0_dn13 = assign54690_e84462_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign54700_e84484, assign54700_e84484_d_n0, assign54700_e84484_d_n2, assign54700_e84484_d_n4, assign54700_e84484_d_n5, assign54700_e84484_d_n6, assign54700_e84484_d_n7, assign54700_e84484_d_n8, assign54700_e84484_d_n9, assign54700_e84484_d_n10, assign54700_e84484_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54700_e84478: f64 = (locals.var_vgpdep_dlt__blk1142 * locals.var_xmp);
        let assign54700_e84480: f64 = (assign54700_e84478 * locals.var_dnm);
        let assign54700_e84482: f64 = (assign54700_e84480 / locals.var_arg);
        (assign54700_e84482, ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn0) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn0)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn2) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn2)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn4) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn4)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn5) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn5)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn6) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn6)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn7) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn7)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn8) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn8)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn9) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn9)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn10) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn10)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1142 * locals.var_xmp_dn13) * locals.var_dnm) + (assign54700_e84478 * locals.var_dnm_dn13)) * locals.var_arg) - (assign54700_e84480 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign54700_e84484;
        locals.var_t0_dn0 = assign54700_e84484_d_n0;
        locals.var_t0_dn2 = assign54700_e84484_d_n2;
        locals.var_t0_dn4 = assign54700_e84484_d_n4;
        locals.var_t0_dn5 = assign54700_e84484_d_n5;
        locals.var_t0_dn6 = assign54700_e84484_d_n6;
        locals.var_t0_dn7 = assign54700_e84484_d_n7;
        locals.var_t0_dn8 = assign54700_e84484_d_n8;
        locals.var_t0_dn9 = assign54700_e84484_d_n9;
        locals.var_t0_dn10 = assign54700_e84484_d_n10;
        locals.var_t0_dn13 = assign54700_e84484_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign54710_e84504, assign54710_e84504_d_n0, assign54710_e84504_d_n2, assign54710_e84504_d_n4, assign54710_e84504_d_n5, assign54710_e84504_d_n6, assign54710_e84504_d_n7, assign54710_e84504_d_n8, assign54710_e84504_d_n9, assign54710_e84504_d_n10, assign54710_e84504_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign54710_e84500: f64 = (-locals.var_vgpdep_dlt__blk1142);
        let assign54710_e84502: f64 = (assign54710_e84500 + locals.var_tmf0);
        (assign54710_e84502, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign54710_e84504;
        locals.var_ps0dep_dn0 = assign54710_e84504_d_n0;
        locals.var_ps0dep_dn2 = assign54710_e84504_d_n2;
        locals.var_ps0dep_dn4 = assign54710_e84504_d_n4;
        locals.var_ps0dep_dn5 = assign54710_e84504_d_n5;
        locals.var_ps0dep_dn6 = assign54710_e84504_d_n6;
        locals.var_ps0dep_dn7 = assign54710_e84504_d_n7;
        locals.var_ps0dep_dn8 = assign54710_e84504_d_n8;
        locals.var_ps0dep_dn9 = assign54710_e84504_d_n9;
        locals.var_ps0dep_dn10 = assign54710_e84504_d_n10;
        locals.var_ps0dep_dn13 = assign54710_e84504_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign54720_e84520, assign54720_e84520_d_n0, assign54720_e84520_d_n2, assign54720_e84520_d_n4, assign54720_e84520_d_n5, assign54720_e84520_d_n6, assign54720_e84520_d_n7, assign54720_e84520_d_n8, assign54720_e84520_d_n9, assign54720_e84520_d_n10, assign54720_e84520_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign54720_e84520;
        locals.var_t0_dn0 = assign54720_e84520_d_n0;
        locals.var_t0_dn2 = assign54720_e84520_d_n2;
        locals.var_t0_dn4 = assign54720_e84520_d_n4;
        locals.var_t0_dn5 = assign54720_e84520_d_n5;
        locals.var_t0_dn6 = assign54720_e84520_d_n6;
        locals.var_t0_dn7 = assign54720_e84520_d_n7;
        locals.var_t0_dn8 = assign54720_e84520_d_n8;
        locals.var_t0_dn9 = assign54720_e84520_d_n9;
        locals.var_t0_dn10 = assign54720_e84520_d_n10;
        locals.var_t0_dn13 = assign54720_e84520_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign54730_e84537, assign54730_e84537_d_n0, assign54730_e84537_d_n2, assign54730_e84537_d_n4, assign54730_e84537_d_n5, assign54730_e84537_d_n6, assign54730_e84537_d_n7, assign54730_e84537_d_n8, assign54730_e84537_d_n9, assign54730_e84537_d_n10, assign54730_e84537_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 == 0.0)) {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign54730_e84537;
        locals.var_ps0dep_dn0 = assign54730_e84537_d_n0;
        locals.var_ps0dep_dn2 = assign54730_e84537_d_n2;
        locals.var_ps0dep_dn4 = assign54730_e84537_d_n4;
        locals.var_ps0dep_dn5 = assign54730_e84537_d_n5;
        locals.var_ps0dep_dn6 = assign54730_e84537_d_n6;
        locals.var_ps0dep_dn7 = assign54730_e84537_d_n7;
        locals.var_ps0dep_dn8 = assign54730_e84537_d_n8;
        locals.var_ps0dep_dn9 = assign54730_e84537_d_n9;
        locals.var_ps0dep_dn10 = assign54730_e84537_d_n10;
        locals.var_ps0dep_dn13 = assign54730_e84537_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign54740_e84554, assign54740_e84554_d_n0, assign54740_e84554_d_n2, assign54740_e84554_d_n4, assign54740_e84554_d_n5, assign54740_e84554_d_n6, assign54740_e84554_d_n7, assign54740_e84554_d_n8, assign54740_e84554_d_n9, assign54740_e84554_d_n10, assign54740_e84554_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1375 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign54740_e84554;
        locals.var_t0_dn0 = assign54740_e84554_d_n0;
        locals.var_t0_dn2 = assign54740_e84554_d_n2;
        locals.var_t0_dn4 = assign54740_e84554_d_n4;
        locals.var_t0_dn5 = assign54740_e84554_d_n5;
        locals.var_t0_dn6 = assign54740_e84554_d_n6;
        locals.var_t0_dn7 = assign54740_e84554_d_n7;
        locals.var_t0_dn8 = assign54740_e84554_d_n8;
        locals.var_t0_dn9 = assign54740_e84554_d_n9;
        locals.var_t0_dn10 = assign54740_e84554_d_n10;
        locals.var_t0_dn13 = assign54740_e84554_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign54750_e84569, assign54750_e84569_d_n0, assign54750_e84569_d_n2, assign54750_e84569_d_n4, assign54750_e84569_d_n5, assign54750_e84569_d_n6, assign54750_e84569_d_n7, assign54750_e84569_d_n8, assign54750_e84569_d_n9, assign54750_e84569_d_n10, assign54750_e84569_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign54750_e84567: f64 = (-locals.var_ps0dep);
        (assign54750_e84567, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn13),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn13,)
    }
};
        locals.var_ps0dep = assign54750_e84569;
        locals.var_ps0dep_dn0 = assign54750_e84569_d_n0;
        locals.var_ps0dep_dn2 = assign54750_e84569_d_n2;
        locals.var_ps0dep_dn4 = assign54750_e84569_d_n4;
        locals.var_ps0dep_dn5 = assign54750_e84569_d_n5;
        locals.var_ps0dep_dn6 = assign54750_e84569_d_n6;
        locals.var_ps0dep_dn7 = assign54750_e84569_d_n7;
        locals.var_ps0dep_dn8 = assign54750_e84569_d_n8;
        locals.var_ps0dep_dn9 = assign54750_e84569_d_n9;
        locals.var_ps0dep_dn10 = assign54750_e84569_d_n10;
        locals.var_ps0dep_dn13 = assign54750_e84569_d_n13;
        locals.var_ps0dep_rv = 0.0;

        let (assign54760_e84591, assign54760_e84591_d_n0, assign54760_e84591_d_n2, assign54760_e84591_d_n4, assign54760_e84591_d_n5, assign54760_e84591_d_n6, assign54760_e84591_d_n7, assign54760_e84591_d_n8, assign54760_e84591_d_n9, assign54760_e84591_d_n10, assign54760_e84591_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign54760_e84583: f64 = (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148);
        let assign54760_e84585: f64 = (assign54760_e84583 * locals.var_tnp__blk1148);
        let assign54760_e84587: f64 = (assign54760_e84585 / 2.0);
        let assign54760_e84589: f64 = (assign54760_e84587 / 1.034943e-10);
        (assign54760_e84589, ((((((locals.var_q_ndepm__blk1133_dn0 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn0)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn2 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn2)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn4 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn4)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn5 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn5)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn6 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn6)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn7 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn7)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn8 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn8)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn9 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn9)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn10 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn10)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1133_dn13 * locals.var_tnp__blk1148) + (locals.var_q_ndepm__blk1133 * locals.var_tnp__blk1148_dn13)) * locals.var_tnp__blk1148) + (assign54760_e84583 * locals.var_tnp__blk1148_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1320, locals.var_dphi_sb__blk1320_dn0, locals.var_dphi_sb__blk1320_dn2, locals.var_dphi_sb__blk1320_dn4, locals.var_dphi_sb__blk1320_dn5, locals.var_dphi_sb__blk1320_dn6, locals.var_dphi_sb__blk1320_dn7, locals.var_dphi_sb__blk1320_dn8, locals.var_dphi_sb__blk1320_dn9, locals.var_dphi_sb__blk1320_dn10, locals.var_dphi_sb__blk1320_dn13,)
    }
};
        locals.var_dphi_sb__blk1320 = assign54760_e84591;
        locals.var_dphi_sb__blk1320_dn0 = assign54760_e84591_d_n0;
        locals.var_dphi_sb__blk1320_dn2 = assign54760_e84591_d_n2;
        locals.var_dphi_sb__blk1320_dn4 = assign54760_e84591_d_n4;
        locals.var_dphi_sb__blk1320_dn5 = assign54760_e84591_d_n5;
        locals.var_dphi_sb__blk1320_dn6 = assign54760_e84591_d_n6;
        locals.var_dphi_sb__blk1320_dn7 = assign54760_e84591_d_n7;
        locals.var_dphi_sb__blk1320_dn8 = assign54760_e84591_d_n8;
        locals.var_dphi_sb__blk1320_dn9 = assign54760_e84591_d_n9;
        locals.var_dphi_sb__blk1320_dn10 = assign54760_e84591_d_n10;
        locals.var_dphi_sb__blk1320_dn13 = assign54760_e84591_d_n13;
        locals.var_dphi_sb__blk1320_rv = 0.0;

        let (assign54770_e84612, assign54770_e84612_d_n0, assign54770_e84612_d_n2, assign54770_e84612_d_n4, assign54770_e84612_d_n5, assign54770_e84612_d_n6, assign54770_e84612_d_n7, assign54770_e84612_d_n8, assign54770_e84612_d_n9, assign54770_e84612_d_n10, assign54770_e84612_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign54770_e84606: f64 = (2.0 * locals.var_beta);
        let assign54770_e84608: f64 = (assign54770_e84606 * locals.var_dphi_sb__blk1320);
        let assign54770_e84609: f64 = (assign54770_e84608).sqrt();
        let assign54770_e84610: f64 = (locals.var_wdepsubsl * assign54770_e84609);
        (assign54770_e84610, ((locals.var_wdepsubsl_dn0 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn0)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn2 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn2)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn4 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn4)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn5 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn5)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn6 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn6)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn7 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn7)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn8 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn8)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn9 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn9)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn10 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn10)) / (2.0 * assign54770_e84609)))), ((locals.var_wdepsubsl_dn13 * assign54770_e84609) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb__blk1320) + (assign54770_e84606 * locals.var_dphi_sb__blk1320_dn13)) / (2.0 * assign54770_e84609)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign54770_e84612;
        locals.var_t0_dn0 = assign54770_e84612_d_n0;
        locals.var_t0_dn2 = assign54770_e84612_d_n2;
        locals.var_t0_dn4 = assign54770_e84612_d_n4;
        locals.var_t0_dn5 = assign54770_e84612_d_n5;
        locals.var_t0_dn6 = assign54770_e84612_d_n6;
        locals.var_t0_dn7 = assign54770_e84612_d_n7;
        locals.var_t0_dn8 = assign54770_e84612_d_n8;
        locals.var_t0_dn9 = assign54770_e84612_d_n9;
        locals.var_t0_dn10 = assign54770_e84612_d_n10;
        locals.var_t0_dn13 = assign54770_e84612_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign54780_e84633, assign54780_e84633_d_n0, assign54780_e84633_d_n2, assign54780_e84633_d_n4, assign54780_e84633_d_n5, assign54780_e84633_d_n6, assign54780_e84633_d_n7, assign54780_e84633_d_n8, assign54780_e84633_d_n9, assign54780_e84633_d_n10, assign54780_e84633_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign54780_e84625: f64 = (locals.var_t0).exp();
        let assign54780_e84627: f64 = (-locals.var_t0);
        let assign54780_e84628: f64 = (assign54780_e84627).exp();
        let assign54780_e84629: f64 = (assign54780_e84625 + assign54780_e84628);
        let assign54780_e84631: f64 = (assign54780_e84629 / 2.0);
        (assign54780_e84631, (((assign54780_e84625 * locals.var_t0_dn0) + (assign54780_e84628 * (-locals.var_t0_dn0))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn2) + (assign54780_e84628 * (-locals.var_t0_dn2))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn4) + (assign54780_e84628 * (-locals.var_t0_dn4))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn5) + (assign54780_e84628 * (-locals.var_t0_dn5))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn6) + (assign54780_e84628 * (-locals.var_t0_dn6))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn7) + (assign54780_e84628 * (-locals.var_t0_dn7))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn8) + (assign54780_e84628 * (-locals.var_t0_dn8))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn9) + (assign54780_e84628 * (-locals.var_t0_dn9))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn10) + (assign54780_e84628 * (-locals.var_t0_dn10))) / 2.0), (((assign54780_e84625 * locals.var_t0_dn13) + (assign54780_e84628 * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign54780_e84633;
        locals.var_t1_dn0 = assign54780_e84633_d_n0;
        locals.var_t1_dn2 = assign54780_e84633_d_n2;
        locals.var_t1_dn4 = assign54780_e84633_d_n4;
        locals.var_t1_dn5 = assign54780_e84633_d_n5;
        locals.var_t1_dn6 = assign54780_e84633_d_n6;
        locals.var_t1_dn7 = assign54780_e84633_d_n7;
        locals.var_t1_dn8 = assign54780_e84633_d_n8;
        locals.var_t1_dn9 = assign54780_e84633_d_n9;
        locals.var_t1_dn10 = assign54780_e84633_d_n10;
        locals.var_t1_dn13 = assign54780_e84633_d_n13;
        locals.var_t1_rv = 0.0;

        let assign54790_e84635: f64 = (locals.var_t0).abs();
        let assign54790_e84637: f64 = if assign54790_e84635 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1381 = assign54790_e84637;
        locals.var_guard1381_rv = 0.0;

        let (assign54800_e84656, assign54800_e84656_d_n0, assign54800_e84656_d_n2, assign54800_e84656_d_n4, assign54800_e84656_d_n5, assign54800_e84656_d_n6, assign54800_e84656_d_n7, assign54800_e84656_d_n8, assign54800_e84656_d_n9, assign54800_e84656_d_n10, assign54800_e84656_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1381 != 0.0)) {
        let assign54800_e84652: f64 = (locals.var_t1).ln();
        let assign54800_e84654: f64 = (assign54800_e84652 / locals.var_dphi_sb__blk1320);
        (assign54800_e84654, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn0)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn2)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn4)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn5)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn6)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn7)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn8)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn9)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn10)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb__blk1320) - (assign54800_e84652 * locals.var_dphi_sb__blk1320_dn13)) / (locals.var_dphi_sb__blk1320 * locals.var_dphi_sb__blk1320)),)
    } else {
        (locals.var_c_sb__blk1321, locals.var_c_sb__blk1321_dn0, locals.var_c_sb__blk1321_dn2, locals.var_c_sb__blk1321_dn4, locals.var_c_sb__blk1321_dn5, locals.var_c_sb__blk1321_dn6, locals.var_c_sb__blk1321_dn7, locals.var_c_sb__blk1321_dn8, locals.var_c_sb__blk1321_dn9, locals.var_c_sb__blk1321_dn10, locals.var_c_sb__blk1321_dn13,)
    }
};
        locals.var_c_sb__blk1321 = assign54800_e84656;
        locals.var_c_sb__blk1321_dn0 = assign54800_e84656_d_n0;
        locals.var_c_sb__blk1321_dn2 = assign54800_e84656_d_n2;
        locals.var_c_sb__blk1321_dn4 = assign54800_e84656_d_n4;
        locals.var_c_sb__blk1321_dn5 = assign54800_e84656_d_n5;
        locals.var_c_sb__blk1321_dn6 = assign54800_e84656_d_n6;
        locals.var_c_sb__blk1321_dn7 = assign54800_e84656_d_n7;
        locals.var_c_sb__blk1321_dn8 = assign54800_e84656_d_n8;
        locals.var_c_sb__blk1321_dn9 = assign54800_e84656_d_n9;
        locals.var_c_sb__blk1321_dn10 = assign54800_e84656_d_n10;
        locals.var_c_sb__blk1321_dn13 = assign54800_e84656_d_n13;
        locals.var_c_sb__blk1321_rv = 0.0;

        let (assign54810_e84685, assign54810_e84685_d_n0, assign54810_e84685_d_n2, assign54810_e84685_d_n4, assign54810_e84685_d_n5, assign54810_e84685_d_n6, assign54810_e84685_d_n7, assign54810_e84685_d_n8, assign54810_e84685_d_n9, assign54810_e84685_d_n10, assign54810_e84685_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1381 == 0.0)) {
        let assign54810_e84673: f64 = (locals.var_wdepsubsl * locals.var_wdepsubsl);
        let assign54810_e84675: f64 = (assign54810_e84673 * locals.var_beta);
        let assign54810_e84679: f64 = (0.1666666666666667 * locals.var_t0);
        let assign54810_e84681: f64 = (assign54810_e84679 * locals.var_t0);
        let assign54810_e84682: f64 = (1.0 - assign54810_e84681);
        let assign54810_e84683: f64 = (assign54810_e84675 * assign54810_e84682);
        (assign54810_e84683, ((((((locals.var_wdepsubsl_dn0 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn0)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn0)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn0) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn0))))), ((((((locals.var_wdepsubsl_dn2 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn2)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn2)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn2) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn2))))), ((((((locals.var_wdepsubsl_dn4 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn4)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn4)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn4) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn4))))), ((((((locals.var_wdepsubsl_dn5 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn5)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn5)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn5) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn5))))), ((((((locals.var_wdepsubsl_dn6 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn6)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn6)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn6) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn6))))), ((((((locals.var_wdepsubsl_dn7 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn7)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn7)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn7) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn7))))), ((((((locals.var_wdepsubsl_dn8 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn8)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn8)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn8) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn8))))), ((((((locals.var_wdepsubsl_dn9 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn9)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn9)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn9) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn9))))), ((((((locals.var_wdepsubsl_dn10 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn10)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn10)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn10) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn10))))), ((((((locals.var_wdepsubsl_dn13 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn13)) * locals.var_beta) + (assign54810_e84673 * locals.var_beta_dn13)) * assign54810_e84682) + (assign54810_e84675 * (-(((0.1666666666666667 * locals.var_t0_dn13) * locals.var_t0) + (assign54810_e84679 * locals.var_t0_dn13))))),)
    } else {
        (locals.var_c_sb__blk1321, locals.var_c_sb__blk1321_dn0, locals.var_c_sb__blk1321_dn2, locals.var_c_sb__blk1321_dn4, locals.var_c_sb__blk1321_dn5, locals.var_c_sb__blk1321_dn6, locals.var_c_sb__blk1321_dn7, locals.var_c_sb__blk1321_dn8, locals.var_c_sb__blk1321_dn9, locals.var_c_sb__blk1321_dn10, locals.var_c_sb__blk1321_dn13,)
    }
};
        locals.var_c_sb__blk1321 = assign54810_e84685;
        locals.var_c_sb__blk1321_dn0 = assign54810_e84685_d_n0;
        locals.var_c_sb__blk1321_dn2 = assign54810_e84685_d_n2;
        locals.var_c_sb__blk1321_dn4 = assign54810_e84685_d_n4;
        locals.var_c_sb__blk1321_dn5 = assign54810_e84685_d_n5;
        locals.var_c_sb__blk1321_dn6 = assign54810_e84685_d_n6;
        locals.var_c_sb__blk1321_dn7 = assign54810_e84685_d_n7;
        locals.var_c_sb__blk1321_dn8 = assign54810_e84685_d_n8;
        locals.var_c_sb__blk1321_dn9 = assign54810_e84685_d_n9;
        locals.var_c_sb__blk1321_dn10 = assign54810_e84685_d_n10;
        locals.var_c_sb__blk1321_dn13 = assign54810_e84685_d_n13;
        locals.var_c_sb__blk1321_rv = 0.0;

        let (assign54820_e84701, assign54820_e84701_d_n0, assign54820_e84701_d_n2, assign54820_e84701_d_n4, assign54820_e84701_d_n5, assign54820_e84701_d_n6, assign54820_e84701_d_n7, assign54820_e84701_d_n8, assign54820_e84701_d_n9, assign54820_e84701_d_n10, assign54820_e84701_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign54820_e84699: f64 = (locals.var_c_sb__blk1321 * locals.var_ps0dep);
        (assign54820_e84699, ((locals.var_c_sb__blk1321_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1321_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1321_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1321_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1321_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1321_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1321_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1321_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1321_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1321_dn13 * locals.var_ps0dep) + (locals.var_c_sb__blk1321 * locals.var_ps0dep_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign54820_e84701;
        locals.var_tx_dn0 = assign54820_e84701_d_n0;
        locals.var_tx_dn2 = assign54820_e84701_d_n2;
        locals.var_tx_dn4 = assign54820_e84701_d_n4;
        locals.var_tx_dn5 = assign54820_e84701_d_n5;
        locals.var_tx_dn6 = assign54820_e84701_d_n6;
        locals.var_tx_dn7 = assign54820_e84701_d_n7;
        locals.var_tx_dn8 = assign54820_e84701_d_n8;
        locals.var_tx_dn9 = assign54820_e84701_d_n9;
        locals.var_tx_dn10 = assign54820_e84701_d_n10;
        locals.var_tx_dn13 = assign54820_e84701_d_n13;
        locals.var_tx_rv = 0.0;

        let assign54830_e84704: f64 = if locals.var_tx > 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1382 = assign54830_e84704;
        locals.var_guard1382_rv = 0.0;

        let (assign54840_e84722, assign54840_e84722_d_n0, assign54840_e84722_d_n2, assign54840_e84722_d_n4, assign54840_e84722_d_n5, assign54840_e84722_d_n6, assign54840_e84722_d_n7, assign54840_e84722_d_n8, assign54840_e84722_d_n9, assign54840_e84722_d_n10, assign54840_e84722_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 != 0.0)) {
        let assign54840_e84720: f64 = (locals.var_ps0dep - locals.var_dphi_sb__blk1320);
        (assign54840_e84720, (locals.var_ps0dep_dn0 - locals.var_dphi_sb__blk1320_dn0), (locals.var_ps0dep_dn2 - locals.var_dphi_sb__blk1320_dn2), (locals.var_ps0dep_dn4 - locals.var_dphi_sb__blk1320_dn4), (locals.var_ps0dep_dn5 - locals.var_dphi_sb__blk1320_dn5), (locals.var_ps0dep_dn6 - locals.var_dphi_sb__blk1320_dn6), (locals.var_ps0dep_dn7 - locals.var_dphi_sb__blk1320_dn7), (locals.var_ps0dep_dn8 - locals.var_dphi_sb__blk1320_dn8), (locals.var_ps0dep_dn9 - locals.var_dphi_sb__blk1320_dn9), (locals.var_ps0dep_dn10 - locals.var_dphi_sb__blk1320_dn10), (locals.var_ps0dep_dn13 - locals.var_dphi_sb__blk1320_dn13),)
    } else {
        (locals.var_pb0dep__blk1165, locals.var_pb0dep__blk1165_dn0, locals.var_pb0dep__blk1165_dn2, locals.var_pb0dep__blk1165_dn4, locals.var_pb0dep__blk1165_dn5, locals.var_pb0dep__blk1165_dn6, locals.var_pb0dep__blk1165_dn7, locals.var_pb0dep__blk1165_dn8, locals.var_pb0dep__blk1165_dn9, locals.var_pb0dep__blk1165_dn10, locals.var_pb0dep__blk1165_dn13,)
    }
};
        locals.var_pb0dep__blk1165 = assign54840_e84722;
        locals.var_pb0dep__blk1165_dn0 = assign54840_e84722_d_n0;
        locals.var_pb0dep__blk1165_dn2 = assign54840_e84722_d_n2;
        locals.var_pb0dep__blk1165_dn4 = assign54840_e84722_d_n4;
        locals.var_pb0dep__blk1165_dn5 = assign54840_e84722_d_n5;
        locals.var_pb0dep__blk1165_dn6 = assign54840_e84722_d_n6;
        locals.var_pb0dep__blk1165_dn7 = assign54840_e84722_d_n7;
        locals.var_pb0dep__blk1165_dn8 = assign54840_e84722_d_n8;
        locals.var_pb0dep__blk1165_dn9 = assign54840_e84722_d_n9;
        locals.var_pb0dep__blk1165_dn10 = assign54840_e84722_d_n10;
        locals.var_pb0dep__blk1165_dn13 = assign54840_e84722_d_n13;
        locals.var_pb0dep__blk1165_rv = 0.0;

        let (assign54850_e84743, assign54850_e84743_d_n0, assign54850_e84743_d_n2, assign54850_e84743_d_n4, assign54850_e84743_d_n5, assign54850_e84743_d_n6, assign54850_e84743_d_n7, assign54850_e84743_d_n8, assign54850_e84743_d_n9, assign54850_e84743_d_n10, assign54850_e84743_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) {
        let assign54850_e84738: f64 = (-locals.var_c_sb__blk1321);
        let assign54850_e84740: f64 = (assign54850_e84738 * locals.var_dphi_sb__blk1320);
        let assign54850_e84741: f64 = (assign54850_e84740).exp();
        (assign54850_e84741, (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn0) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn0))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn2) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn2))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn4) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn4))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn5) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn5))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn6) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn6))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn7) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn7))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn8) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn8))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn9) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn9))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn10) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn10))), (assign54850_e84741 * (((-locals.var_c_sb__blk1321_dn13) * locals.var_dphi_sb__blk1320) + (assign54850_e84738 * locals.var_dphi_sb__blk1320_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign54850_e84743;
        locals.var_t0_dn0 = assign54850_e84743_d_n0;
        locals.var_t0_dn2 = assign54850_e84743_d_n2;
        locals.var_t0_dn4 = assign54850_e84743_d_n4;
        locals.var_t0_dn5 = assign54850_e84743_d_n5;
        locals.var_t0_dn6 = assign54850_e84743_d_n6;
        locals.var_t0_dn7 = assign54850_e84743_d_n7;
        locals.var_t0_dn8 = assign54850_e84743_d_n8;
        locals.var_t0_dn9 = assign54850_e84743_d_n9;
        locals.var_t0_dn10 = assign54850_e84743_d_n10;
        locals.var_t0_dn13 = assign54850_e84743_d_n13;
        locals.var_t0_rv = 0.0;

        let assign54860_e84745: f64 = (locals.var_tx).abs();
        let assign54860_e84747: f64 = if assign54860_e84745 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1383 = assign54860_e84747;
        locals.var_guard1383_rv = 0.0;

        let assign54870_e84750: f64 = if locals.var_tx >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1384 = assign54870_e84750;
        locals.var_guard1384_rv = 0.0;

        let (assign54880_e84777, assign54880_e84777_d_n0, assign54880_e84777_d_n2, assign54880_e84777_d_n4, assign54880_e84777_d_n5, assign54880_e84777_d_n6, assign54880_e84777_d_n7, assign54880_e84777_d_n8, assign54880_e84777_d_n9, assign54880_e84777_d_n10, assign54880_e84777_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 != 0.0)) {
        let assign54880_e84772: f64 = (1.0 + locals.var_tx);
        let assign54880_e84774: f64 = (assign54880_e84772 - 500.0);
        let assign54880_e84775: f64 = (1.403592217853e217 * assign54880_e84774);
        (assign54880_e84775, (1.403592217853e217 * locals.var_tx_dn0), (1.403592217853e217 * locals.var_tx_dn2), (1.403592217853e217 * locals.var_tx_dn4), (1.403592217853e217 * locals.var_tx_dn5), (1.403592217853e217 * locals.var_tx_dn6), (1.403592217853e217 * locals.var_tx_dn7), (1.403592217853e217 * locals.var_tx_dn8), (1.403592217853e217 * locals.var_tx_dn9), (1.403592217853e217 * locals.var_tx_dn10), (1.403592217853e217 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign54880_e84777;
        locals.var_t1_dn0 = assign54880_e84777_d_n0;
        locals.var_t1_dn2 = assign54880_e84777_d_n2;
        locals.var_t1_dn4 = assign54880_e84777_d_n4;
        locals.var_t1_dn5 = assign54880_e84777_d_n5;
        locals.var_t1_dn6 = assign54880_e84777_d_n6;
        locals.var_t1_dn7 = assign54880_e84777_d_n7;
        locals.var_t1_dn8 = assign54880_e84777_d_n8;
        locals.var_t1_dn9 = assign54880_e84777_d_n9;
        locals.var_t1_dn10 = assign54880_e84777_d_n10;
        locals.var_t1_dn13 = assign54880_e84777_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign54890_e84798, assign54890_e84798_d_n0, assign54890_e84798_d_n2, assign54890_e84798_d_n4, assign54890_e84798_d_n5, assign54890_e84798_d_n6, assign54890_e84798_d_n7, assign54890_e84798_d_n8, assign54890_e84798_d_n9, assign54890_e84798_d_n10, assign54890_e84798_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign54890_e84798;
        locals.var_t3_dn0 = assign54890_e84798_d_n0;
        locals.var_t3_dn2 = assign54890_e84798_d_n2;
        locals.var_t3_dn4 = assign54890_e84798_d_n4;
        locals.var_t3_dn5 = assign54890_e84798_d_n5;
        locals.var_t3_dn6 = assign54890_e84798_d_n6;
        locals.var_t3_dn7 = assign54890_e84798_d_n7;
        locals.var_t3_dn8 = assign54890_e84798_d_n8;
        locals.var_t3_dn9 = assign54890_e84798_d_n9;
        locals.var_t3_dn10 = assign54890_e84798_d_n10;
        locals.var_t3_dn13 = assign54890_e84798_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign54900_e84820, assign54900_e84820_d_n0, assign54900_e84820_d_n2, assign54900_e84820_d_n4, assign54900_e84820_d_n5, assign54900_e84820_d_n6, assign54900_e84820_d_n7, assign54900_e84820_d_n8, assign54900_e84820_d_n9, assign54900_e84820_d_n10, assign54900_e84820_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 == 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign54900_e84820;
        locals.var_tmf1_dn0 = assign54900_e84820_d_n0;
        locals.var_tmf1_dn2 = assign54900_e84820_d_n2;
        locals.var_tmf1_dn4 = assign54900_e84820_d_n4;
        locals.var_tmf1_dn5 = assign54900_e84820_d_n5;
        locals.var_tmf1_dn6 = assign54900_e84820_d_n6;
        locals.var_tmf1_dn7 = assign54900_e84820_d_n7;
        locals.var_tmf1_dn8 = assign54900_e84820_d_n8;
        locals.var_tmf1_dn9 = assign54900_e84820_d_n9;
        locals.var_tmf1_dn10 = assign54900_e84820_d_n10;
        locals.var_tmf1_dn13 = assign54900_e84820_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign54910_e84842, assign54910_e84842_d_n0, assign54910_e84842_d_n2, assign54910_e84842_d_n4, assign54910_e84842_d_n5, assign54910_e84842_d_n6, assign54910_e84842_d_n7, assign54910_e84842_d_n8, assign54910_e84842_d_n9, assign54910_e84842_d_n10, assign54910_e84842_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign54910_e84842;
        locals.var_t1_dn0 = assign54910_e84842_d_n0;
        locals.var_t1_dn2 = assign54910_e84842_d_n2;
        locals.var_t1_dn4 = assign54910_e84842_d_n4;
        locals.var_t1_dn5 = assign54910_e84842_d_n5;
        locals.var_t1_dn6 = assign54910_e84842_d_n6;
        locals.var_t1_dn7 = assign54910_e84842_d_n7;
        locals.var_t1_dn8 = assign54910_e84842_d_n8;
        locals.var_t1_dn9 = assign54910_e84842_d_n9;
        locals.var_t1_dn10 = assign54910_e84842_d_n10;
        locals.var_t1_dn13 = assign54910_e84842_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_192(
        locals: &mut StampLocals,
    ) {
        let mut assign54920_loop_guard: usize = 0;
        while {
            let assign54920_cond_e84865: f64 = if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign54920_cond_e84865 != 0.0
        } {
            assign54920_loop_guard += 1;
            assert!(assign54920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54920_body0_e84889, assign54920_body0_e84889_d_n0, assign54920_body0_e84889_d_n2, assign54920_body0_e84889_d_n4, assign54920_body0_e84889_d_n5, assign54920_body0_e84889_d_n6, assign54920_body0_e84889_d_n7, assign54920_body0_e84889_d_n8, assign54920_body0_e84889_d_n9, assign54920_body0_e84889_d_n10, assign54920_body0_e84889_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 == 0.0)) {
        let assign54920_body0_e84887: f64 = (locals.var_t1 * 1.14200738981568e26);
        (assign54920_body0_e84887, (locals.var_t1_dn0 * 1.14200738981568e26), (locals.var_t1_dn2 * 1.14200738981568e26), (locals.var_t1_dn4 * 1.14200738981568e26), (locals.var_t1_dn5 * 1.14200738981568e26), (locals.var_t1_dn6 * 1.14200738981568e26), (locals.var_t1_dn7 * 1.14200738981568e26), (locals.var_t1_dn8 * 1.14200738981568e26), (locals.var_t1_dn9 * 1.14200738981568e26), (locals.var_t1_dn10 * 1.14200738981568e26), (locals.var_t1_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign54920_body0_e84889;
            locals.var_t1_dn0 = assign54920_body0_e84889_d_n0;
            locals.var_t1_dn2 = assign54920_body0_e84889_d_n2;
            locals.var_t1_dn4 = assign54920_body0_e84889_d_n4;
            locals.var_t1_dn5 = assign54920_body0_e84889_d_n5;
            locals.var_t1_dn6 = assign54920_body0_e84889_d_n6;
            locals.var_t1_dn7 = assign54920_body0_e84889_d_n7;
            locals.var_t1_dn8 = assign54920_body0_e84889_d_n8;
            locals.var_t1_dn9 = assign54920_body0_e84889_d_n9;
            locals.var_t1_dn10 = assign54920_body0_e84889_d_n10;
            locals.var_t1_dn13 = assign54920_body0_e84889_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign54920_body1_e84913, assign54920_body1_e84913_d_n0, assign54920_body1_e84913_d_n2, assign54920_body1_e84913_d_n4, assign54920_body1_e84913_d_n5, assign54920_body1_e84913_d_n6, assign54920_body1_e84913_d_n7, assign54920_body1_e84913_d_n8, assign54920_body1_e84913_d_n9, assign54920_body1_e84913_d_n10, assign54920_body1_e84913_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 == 0.0)) {
        let assign54920_body1_e84911: f64 = (locals.var_tmf1 - 60.0);
        (assign54920_body1_e84911, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign54920_body1_e84913;
            locals.var_tmf1_dn0 = assign54920_body1_e84913_d_n0;
            locals.var_tmf1_dn2 = assign54920_body1_e84913_d_n2;
            locals.var_tmf1_dn4 = assign54920_body1_e84913_d_n4;
            locals.var_tmf1_dn5 = assign54920_body1_e84913_d_n5;
            locals.var_tmf1_dn6 = assign54920_body1_e84913_d_n6;
            locals.var_tmf1_dn7 = assign54920_body1_e84913_d_n7;
            locals.var_tmf1_dn8 = assign54920_body1_e84913_d_n8;
            locals.var_tmf1_dn9 = assign54920_body1_e84913_d_n9;
            locals.var_tmf1_dn10 = assign54920_body1_e84913_d_n10;
            locals.var_tmf1_dn13 = assign54920_body1_e84913_d_n13;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign54930_e84938, assign54930_e84938_d_n0, assign54930_e84938_d_n2, assign54930_e84938_d_n4, assign54930_e84938_d_n5, assign54930_e84938_d_n6, assign54930_e84938_d_n7, assign54930_e84938_d_n8, assign54930_e84938_d_n9, assign54930_e84938_d_n10, assign54930_e84938_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 == 0.0)) {
        let assign54930_e84935: f64 = (locals.var_tmf1).exp();
        let assign54930_e84936: f64 = (locals.var_t1 * assign54930_e84935);
        (assign54930_e84936, ((locals.var_t1_dn0 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn0))), ((locals.var_t1_dn2 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn2))), ((locals.var_t1_dn4 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn4))), ((locals.var_t1_dn5 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn5))), ((locals.var_t1_dn6 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn6))), ((locals.var_t1_dn7 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn7))), ((locals.var_t1_dn8 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn8))), ((locals.var_t1_dn9 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn9))), ((locals.var_t1_dn10 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn10))), ((locals.var_t1_dn13 * assign54930_e84935) + (locals.var_t1 * (assign54930_e84935 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign54930_e84938;
        locals.var_t1_dn0 = assign54930_e84938_d_n0;
        locals.var_t1_dn2 = assign54930_e84938_d_n2;
        locals.var_t1_dn4 = assign54930_e84938_d_n4;
        locals.var_t1_dn5 = assign54930_e84938_d_n5;
        locals.var_t1_dn6 = assign54930_e84938_d_n6;
        locals.var_t1_dn7 = assign54930_e84938_d_n7;
        locals.var_t1_dn8 = assign54930_e84938_d_n8;
        locals.var_t1_dn9 = assign54930_e84938_d_n9;
        locals.var_t1_dn10 = assign54930_e84938_d_n10;
        locals.var_t1_dn13 = assign54930_e84938_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign54940_e84960, assign54940_e84960_d_n0, assign54940_e84960_d_n2, assign54940_e84960_d_n4, assign54940_e84960_d_n5, assign54940_e84960_d_n6, assign54940_e84960_d_n7, assign54940_e84960_d_n8, assign54940_e84960_d_n9, assign54940_e84960_d_n10, assign54940_e84960_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) && (locals.var_guard1384 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign54940_e84960;
        locals.var_t3_dn0 = assign54940_e84960_d_n0;
        locals.var_t3_dn2 = assign54940_e84960_d_n2;
        locals.var_t3_dn4 = assign54940_e84960_d_n4;
        locals.var_t3_dn5 = assign54940_e84960_d_n5;
        locals.var_t3_dn6 = assign54940_e84960_d_n6;
        locals.var_t3_dn7 = assign54940_e84960_d_n7;
        locals.var_t3_dn8 = assign54940_e84960_d_n8;
        locals.var_t3_dn9 = assign54940_e84960_d_n9;
        locals.var_t3_dn10 = assign54940_e84960_d_n10;
        locals.var_t3_dn13 = assign54940_e84960_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign54950_e84981, assign54950_e84981_d_n0, assign54950_e84981_d_n2, assign54950_e84981_d_n4, assign54950_e84981_d_n5, assign54950_e84981_d_n6, assign54950_e84981_d_n7, assign54950_e84981_d_n8, assign54950_e84981_d_n9, assign54950_e84981_d_n10, assign54950_e84981_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) {
        let assign54950_e84979: f64 = (locals.var_t1 * locals.var_t0);
        (assign54950_e84979, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn13 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign54950_e84981;
        locals.var_t1_dn0 = assign54950_e84981_d_n0;
        locals.var_t1_dn2 = assign54950_e84981_d_n2;
        locals.var_t1_dn4 = assign54950_e84981_d_n4;
        locals.var_t1_dn5 = assign54950_e84981_d_n5;
        locals.var_t1_dn6 = assign54950_e84981_d_n6;
        locals.var_t1_dn7 = assign54950_e84981_d_n7;
        locals.var_t1_dn8 = assign54950_e84981_d_n8;
        locals.var_t1_dn9 = assign54950_e84981_d_n9;
        locals.var_t1_dn10 = assign54950_e84981_d_n10;
        locals.var_t1_dn13 = assign54950_e84981_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign54960_e85002, assign54960_e85002_d_n0, assign54960_e85002_d_n2, assign54960_e85002_d_n4, assign54960_e85002_d_n5, assign54960_e85002_d_n6, assign54960_e85002_d_n7, assign54960_e85002_d_n8, assign54960_e85002_d_n9, assign54960_e85002_d_n10, assign54960_e85002_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) {
        let assign54960_e85000: f64 = (locals.var_t1 - locals.var_t0);
        (assign54960_e85000, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign54960_e85002;
        locals.var_t2_dn0 = assign54960_e85002_d_n0;
        locals.var_t2_dn2 = assign54960_e85002_d_n2;
        locals.var_t2_dn4 = assign54960_e85002_d_n4;
        locals.var_t2_dn5 = assign54960_e85002_d_n5;
        locals.var_t2_dn6 = assign54960_e85002_d_n6;
        locals.var_t2_dn7 = assign54960_e85002_d_n7;
        locals.var_t2_dn8 = assign54960_e85002_d_n8;
        locals.var_t2_dn9 = assign54960_e85002_d_n9;
        locals.var_t2_dn10 = assign54960_e85002_d_n10;
        locals.var_t2_dn13 = assign54960_e85002_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign54970_e85026, assign54970_e85026_d_n0, assign54970_e85026_d_n2, assign54970_e85026_d_n4, assign54970_e85026_d_n5, assign54970_e85026_d_n6, assign54970_e85026_d_n7, assign54970_e85026_d_n8, assign54970_e85026_d_n9, assign54970_e85026_d_n10, assign54970_e85026_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 == 0.0)) {
        let assign54970_e85022: f64 = (1.0 + locals.var_tx);
        let assign54970_e85024: f64 = (assign54970_e85022 * locals.var_t0);
        (assign54970_e85024, ((locals.var_tx_dn0 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn10)), ((locals.var_tx_dn13 * locals.var_t0) + (assign54970_e85022 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign54970_e85026;
        locals.var_t1_dn0 = assign54970_e85026_d_n0;
        locals.var_t1_dn2 = assign54970_e85026_d_n2;
        locals.var_t1_dn4 = assign54970_e85026_d_n4;
        locals.var_t1_dn5 = assign54970_e85026_d_n5;
        locals.var_t1_dn6 = assign54970_e85026_d_n6;
        locals.var_t1_dn7 = assign54970_e85026_d_n7;
        locals.var_t1_dn8 = assign54970_e85026_d_n8;
        locals.var_t1_dn9 = assign54970_e85026_d_n9;
        locals.var_t1_dn10 = assign54970_e85026_d_n10;
        locals.var_t1_dn13 = assign54970_e85026_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign54980_e85054, assign54980_e85054_d_n0, assign54980_e85054_d_n2, assign54980_e85054_d_n4, assign54980_e85054_d_n5, assign54980_e85054_d_n6, assign54980_e85054_d_n7, assign54980_e85054_d_n8, assign54980_e85054_d_n9, assign54980_e85054_d_n10, assign54980_e85054_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 == 0.0)) {
        let assign54980_e85048: f64 = (locals.var_tx / 2.0);
        let assign54980_e85049: f64 = (1.0 + assign54980_e85048);
        let assign54980_e85050: f64 = (locals.var_tx * assign54980_e85049);
        let assign54980_e85052: f64 = (assign54980_e85050 * locals.var_t0);
        (assign54980_e85052, ((((locals.var_tx_dn0 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn10)), ((((locals.var_tx_dn13 * assign54980_e85049) + (locals.var_tx * (locals.var_tx_dn13 / 2.0))) * locals.var_t0) + (assign54980_e85050 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign54980_e85054;
        locals.var_t2_dn0 = assign54980_e85054_d_n0;
        locals.var_t2_dn2 = assign54980_e85054_d_n2;
        locals.var_t2_dn4 = assign54980_e85054_d_n4;
        locals.var_t2_dn5 = assign54980_e85054_d_n5;
        locals.var_t2_dn6 = assign54980_e85054_d_n6;
        locals.var_t2_dn7 = assign54980_e85054_d_n7;
        locals.var_t2_dn8 = assign54980_e85054_d_n8;
        locals.var_t2_dn9 = assign54980_e85054_d_n9;
        locals.var_t2_dn10 = assign54980_e85054_d_n10;
        locals.var_t2_dn13 = assign54980_e85054_d_n13;
        locals.var_t2_rv = 0.0;

        let assign54990_e85056: f64 = (locals.var_t2).abs();
        let assign54990_e85058: f64 = if assign54990_e85056 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1385 = assign54990_e85058;
        locals.var_guard1385_rv = 0.0;

        let (assign55000_e85082, assign55000_e85082_d_n0, assign55000_e85082_d_n2, assign55000_e85082_d_n4, assign55000_e85082_d_n5, assign55000_e85082_d_n6, assign55000_e85082_d_n7, assign55000_e85082_d_n8, assign55000_e85082_d_n9, assign55000_e85082_d_n10, assign55000_e85082_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1385 != 0.0)) {
        let assign55000_e85077: f64 = (1.0 + locals.var_t2);
        let assign55000_e85078: f64 = (assign55000_e85077).ln();
        let assign55000_e85080: f64 = (assign55000_e85078 / locals.var_c_sb__blk1321);
        (assign55000_e85080, ((((locals.var_t2_dn0 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn0)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn2 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn2)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn4 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn4)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn5 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn5)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn6 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn6)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn7 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn7)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn8 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn8)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn9 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn9)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn10 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn10)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), ((((locals.var_t2_dn13 / assign55000_e85077) * locals.var_c_sb__blk1321) - (assign55000_e85078 * locals.var_c_sb__blk1321_dn13)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)),)
    } else {
        (locals.var_pb0dep__blk1165, locals.var_pb0dep__blk1165_dn0, locals.var_pb0dep__blk1165_dn2, locals.var_pb0dep__blk1165_dn4, locals.var_pb0dep__blk1165_dn5, locals.var_pb0dep__blk1165_dn6, locals.var_pb0dep__blk1165_dn7, locals.var_pb0dep__blk1165_dn8, locals.var_pb0dep__blk1165_dn9, locals.var_pb0dep__blk1165_dn10, locals.var_pb0dep__blk1165_dn13,)
    }
};
        locals.var_pb0dep__blk1165 = assign55000_e85082;
        locals.var_pb0dep__blk1165_dn0 = assign55000_e85082_d_n0;
        locals.var_pb0dep__blk1165_dn2 = assign55000_e85082_d_n2;
        locals.var_pb0dep__blk1165_dn4 = assign55000_e85082_d_n4;
        locals.var_pb0dep__blk1165_dn5 = assign55000_e85082_d_n5;
        locals.var_pb0dep__blk1165_dn6 = assign55000_e85082_d_n6;
        locals.var_pb0dep__blk1165_dn7 = assign55000_e85082_d_n7;
        locals.var_pb0dep__blk1165_dn8 = assign55000_e85082_d_n8;
        locals.var_pb0dep__blk1165_dn9 = assign55000_e85082_d_n9;
        locals.var_pb0dep__blk1165_dn10 = assign55000_e85082_d_n10;
        locals.var_pb0dep__blk1165_dn13 = assign55000_e85082_d_n13;
        locals.var_pb0dep__blk1165_rv = 0.0;

        let (assign55010_e85104, assign55010_e85104_d_n0, assign55010_e85104_d_n2, assign55010_e85104_d_n4, assign55010_e85104_d_n5, assign55010_e85104_d_n6, assign55010_e85104_d_n7, assign55010_e85104_d_n8, assign55010_e85104_d_n9, assign55010_e85104_d_n10, assign55010_e85104_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1385 == 0.0)) {
        let assign55010_e85102: f64 = (locals.var_t2 / locals.var_c_sb__blk1321);
        (assign55010_e85102, (((locals.var_t2_dn0 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn0)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn2)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn4)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn5)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn6)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn7)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn8)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn9)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn10)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)), (((locals.var_t2_dn13 * locals.var_c_sb__blk1321) - (locals.var_t2 * locals.var_c_sb__blk1321_dn13)) / (locals.var_c_sb__blk1321 * locals.var_c_sb__blk1321)),)
    } else {
        (locals.var_pb0dep__blk1165, locals.var_pb0dep__blk1165_dn0, locals.var_pb0dep__blk1165_dn2, locals.var_pb0dep__blk1165_dn4, locals.var_pb0dep__blk1165_dn5, locals.var_pb0dep__blk1165_dn6, locals.var_pb0dep__blk1165_dn7, locals.var_pb0dep__blk1165_dn8, locals.var_pb0dep__blk1165_dn9, locals.var_pb0dep__blk1165_dn10, locals.var_pb0dep__blk1165_dn13,)
    }
};
        locals.var_pb0dep__blk1165 = assign55010_e85104;
        locals.var_pb0dep__blk1165_dn0 = assign55010_e85104_d_n0;
        locals.var_pb0dep__blk1165_dn2 = assign55010_e85104_d_n2;
        locals.var_pb0dep__blk1165_dn4 = assign55010_e85104_d_n4;
        locals.var_pb0dep__blk1165_dn5 = assign55010_e85104_d_n5;
        locals.var_pb0dep__blk1165_dn6 = assign55010_e85104_d_n6;
        locals.var_pb0dep__blk1165_dn7 = assign55010_e85104_d_n7;
        locals.var_pb0dep__blk1165_dn8 = assign55010_e85104_d_n8;
        locals.var_pb0dep__blk1165_dn9 = assign55010_e85104_d_n9;
        locals.var_pb0dep__blk1165_dn10 = assign55010_e85104_d_n10;
        locals.var_pb0dep__blk1165_dn13 = assign55010_e85104_d_n13;
        locals.var_pb0dep__blk1165_rv = 0.0;

        let (assign55020_e85120, assign55020_e85120_d_n0, assign55020_e85120_d_n2, assign55020_e85120_d_n4, assign55020_e85120_d_n5, assign55020_e85120_d_n6, assign55020_e85120_d_n7, assign55020_e85120_d_n8, assign55020_e85120_d_n9, assign55020_e85120_d_n10, assign55020_e85120_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign55020_e85118: f64 = (locals.var_ps0dep - locals.var_pb0dep__blk1165);
        (assign55020_e85118, (locals.var_ps0dep_dn0 - locals.var_pb0dep__blk1165_dn0), (locals.var_ps0dep_dn2 - locals.var_pb0dep__blk1165_dn2), (locals.var_ps0dep_dn4 - locals.var_pb0dep__blk1165_dn4), (locals.var_ps0dep_dn5 - locals.var_pb0dep__blk1165_dn5), (locals.var_ps0dep_dn6 - locals.var_pb0dep__blk1165_dn6), (locals.var_ps0dep_dn7 - locals.var_pb0dep__blk1165_dn7), (locals.var_ps0dep_dn8 - locals.var_pb0dep__blk1165_dn8), (locals.var_ps0dep_dn9 - locals.var_pb0dep__blk1165_dn9), (locals.var_ps0dep_dn10 - locals.var_pb0dep__blk1165_dn10), (locals.var_ps0dep_dn13 - locals.var_pb0dep__blk1165_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign55020_e85120;
        locals.var_t2_dn0 = assign55020_e85120_d_n0;
        locals.var_t2_dn2 = assign55020_e85120_d_n2;
        locals.var_t2_dn4 = assign55020_e85120_d_n4;
        locals.var_t2_dn5 = assign55020_e85120_d_n5;
        locals.var_t2_dn6 = assign55020_e85120_d_n6;
        locals.var_t2_dn7 = assign55020_e85120_d_n7;
        locals.var_t2_dn8 = assign55020_e85120_d_n8;
        locals.var_t2_dn9 = assign55020_e85120_d_n9;
        locals.var_t2_dn10 = assign55020_e85120_d_n10;
        locals.var_t2_dn13 = assign55020_e85120_d_n13;
        locals.var_t2_rv = 0.0;

        let assign55030_e85123: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1386 = assign55030_e85123;
        locals.var_guard1386_rv = 0.0;

        let (assign55040_e85152, assign55040_e85152_d_n0, assign55040_e85152_d_n2, assign55040_e85152_d_n4, assign55040_e85152_d_n5, assign55040_e85152_d_n6, assign55040_e85152_d_n7, assign55040_e85152_d_n8, assign55040_e85152_d_n9, assign55040_e85152_d_n10, assign55040_e85152_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1386 != 0.0)) {
        let (assign55040_e85150, assign55040_e85150_d_n0, assign55040_e85150_d_n2, assign55040_e85150_d_n4, assign55040_e85150_d_n5, assign55040_e85150_d_n6, assign55040_e85150_d_n7, assign55040_e85150_d_n8, assign55040_e85150_d_n9, assign55040_e85150_d_n10, assign55040_e85150_d_n13,) = {
            if (locals.var_t2 < 0.0) {
                let assign55040_e85141: f64 = (-locals.var_c_2esipq_ndepm__blk1136);
                let assign55040_e85143: f64 = (assign55040_e85141 * locals.var_t2);
                let assign55040_e85144: f64 = (assign55040_e85143).sqrt();
                let assign55040_e85145: f64 = (-assign55040_e85144);
                (assign55040_e85145, (-((((-locals.var_c_2esipq_ndepm__blk1136_dn0) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn0)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn2) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn2)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn4) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn4)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn5) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn5)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn6) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn6)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn7) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn7)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn8) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn8)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn9) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn9)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn10) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn10)) / (2.0 * assign55040_e85144))), (-((((-locals.var_c_2esipq_ndepm__blk1136_dn13) * locals.var_t2) + (assign55040_e85141 * locals.var_t2_dn13)) / (2.0 * assign55040_e85144))),)
            } else {
                let assign55040_e85148: f64 = (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2);
                let assign55040_e85149: f64 = (assign55040_e85148).sqrt();
                (assign55040_e85149, (((locals.var_c_2esipq_ndepm__blk1136_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn0)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn2)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn4)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn5)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn6)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn7)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn8)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn9)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn10)) / (2.0 * assign55040_e85149)), (((locals.var_c_2esipq_ndepm__blk1136_dn13 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_t2_dn13)) / (2.0 * assign55040_e85149)),)
            }
        };
        (assign55040_e85150, assign55040_e85150_d_n0, assign55040_e85150_d_n2, assign55040_e85150_d_n4, assign55040_e85150_d_n5, assign55040_e85150_d_n6, assign55040_e85150_d_n7, assign55040_e85150_d_n8, assign55040_e85150_d_n9, assign55040_e85150_d_n10, assign55040_e85150_d_n13,)
    } else {
        (locals.var_ws__blk1147, locals.var_ws__blk1147_dn0, locals.var_ws__blk1147_dn2, locals.var_ws__blk1147_dn4, locals.var_ws__blk1147_dn5, locals.var_ws__blk1147_dn6, locals.var_ws__blk1147_dn7, locals.var_ws__blk1147_dn8, locals.var_ws__blk1147_dn9, locals.var_ws__blk1147_dn10, locals.var_ws__blk1147_dn13,)
    }
};
        locals.var_ws__blk1147 = assign55040_e85152;
        locals.var_ws__blk1147_dn0 = assign55040_e85152_d_n0;
        locals.var_ws__blk1147_dn2 = assign55040_e85152_d_n2;
        locals.var_ws__blk1147_dn4 = assign55040_e85152_d_n4;
        locals.var_ws__blk1147_dn5 = assign55040_e85152_d_n5;
        locals.var_ws__blk1147_dn6 = assign55040_e85152_d_n6;
        locals.var_ws__blk1147_dn7 = assign55040_e85152_d_n7;
        locals.var_ws__blk1147_dn8 = assign55040_e85152_d_n8;
        locals.var_ws__blk1147_dn9 = assign55040_e85152_d_n9;
        locals.var_ws__blk1147_dn10 = assign55040_e85152_d_n10;
        locals.var_ws__blk1147_dn13 = assign55040_e85152_d_n13;
        locals.var_ws__blk1147_rv = 0.0;

        let assign55050_e85155: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1387 = assign55050_e85155;
        locals.var_guard1387_rv = 0.0;

        let (assign55060_e85176, assign55060_e85176_d_n0, assign55060_e85176_d_n2, assign55060_e85176_d_n4, assign55060_e85176_d_n5, assign55060_e85176_d_n6, assign55060_e85176_d_n7, assign55060_e85176_d_n8, assign55060_e85176_d_n9, assign55060_e85176_d_n10, assign55060_e85176_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) {
        let assign55060_e85174: f64 = (locals.var_beta * locals.var_t2);
        (assign55060_e85174, ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)), ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)), ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)), ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)), ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)), ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)), ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)), ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)), ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)), ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign55060_e85176;
        locals.var_t3_dn0 = assign55060_e85176_d_n0;
        locals.var_t3_dn2 = assign55060_e85176_d_n2;
        locals.var_t3_dn4 = assign55060_e85176_d_n4;
        locals.var_t3_dn5 = assign55060_e85176_d_n5;
        locals.var_t3_dn6 = assign55060_e85176_d_n6;
        locals.var_t3_dn7 = assign55060_e85176_d_n7;
        locals.var_t3_dn8 = assign55060_e85176_d_n8;
        locals.var_t3_dn9 = assign55060_e85176_d_n9;
        locals.var_t3_dn10 = assign55060_e85176_d_n10;
        locals.var_t3_dn13 = assign55060_e85176_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign55070_e85206, assign55070_e85206_d_n0, assign55070_e85206_d_n2, assign55070_e85206_d_n4, assign55070_e85206_d_n5, assign55070_e85206_d_n6, assign55070_e85206_d_n7, assign55070_e85206_d_n8, assign55070_e85206_d_n9, assign55070_e85206_d_n10, assign55070_e85206_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) {
        let assign55070_e85195: f64 = (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv);
        let assign55070_e85197: f64 = (locals.var_t3).exp();
        let assign55070_e85199: f64 = (assign55070_e85197 - locals.var_t3);
        let assign55070_e85201: f64 = (assign55070_e85199 - 1.0);
        let assign55070_e85202: f64 = (assign55070_e85195 * assign55070_e85201);
        let assign55070_e85203: f64 = (assign55070_e85202).sqrt();
        let assign55070_e85204: f64 = (-assign55070_e85203);
        (assign55070_e85204, (-(((((locals.var_c_2esipq_ndepm__blk1136_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn0)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn2)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn4)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn5)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn6)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn7)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn8)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn9)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn10)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign55070_e85203))), (-(((((locals.var_c_2esipq_ndepm__blk1136_dn13 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn13)) * assign55070_e85201) + (assign55070_e85195 * ((assign55070_e85197 * locals.var_t3_dn13) - locals.var_t3_dn13))) / (2.0 * assign55070_e85203))),)
    } else {
        (locals.var_ws__blk1147, locals.var_ws__blk1147_dn0, locals.var_ws__blk1147_dn2, locals.var_ws__blk1147_dn4, locals.var_ws__blk1147_dn5, locals.var_ws__blk1147_dn6, locals.var_ws__blk1147_dn7, locals.var_ws__blk1147_dn8, locals.var_ws__blk1147_dn9, locals.var_ws__blk1147_dn10, locals.var_ws__blk1147_dn13,)
    }
};
        locals.var_ws__blk1147 = assign55070_e85206;
        locals.var_ws__blk1147_dn0 = assign55070_e85206_d_n0;
        locals.var_ws__blk1147_dn2 = assign55070_e85206_d_n2;
        locals.var_ws__blk1147_dn4 = assign55070_e85206_d_n4;
        locals.var_ws__blk1147_dn5 = assign55070_e85206_d_n5;
        locals.var_ws__blk1147_dn6 = assign55070_e85206_d_n6;
        locals.var_ws__blk1147_dn7 = assign55070_e85206_d_n7;
        locals.var_ws__blk1147_dn8 = assign55070_e85206_d_n8;
        locals.var_ws__blk1147_dn9 = assign55070_e85206_d_n9;
        locals.var_ws__blk1147_dn10 = assign55070_e85206_d_n10;
        locals.var_ws__blk1147_dn13 = assign55070_e85206_d_n13;
        locals.var_ws__blk1147_rv = 0.0;

        let (assign55080_e85229, assign55080_e85229_d_n0, assign55080_e85229_d_n2, assign55080_e85229_d_n4, assign55080_e85229_d_n5, assign55080_e85229_d_n6, assign55080_e85229_d_n7, assign55080_e85229_d_n8, assign55080_e85229_d_n9, assign55080_e85229_d_n10, assign55080_e85229_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 == 0.0)) {
        let assign55080_e85225: f64 = (-locals.var_beta);
        let assign55080_e85227: f64 = (assign55080_e85225 * locals.var_t2);
        (assign55080_e85227, (((-locals.var_beta_dn0) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn0)), (((-locals.var_beta_dn2) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn2)), (((-locals.var_beta_dn4) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn4)), (((-locals.var_beta_dn5) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn5)), (((-locals.var_beta_dn6) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn6)), (((-locals.var_beta_dn7) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn7)), (((-locals.var_beta_dn8) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn8)), (((-locals.var_beta_dn9) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn9)), (((-locals.var_beta_dn10) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn10)), (((-locals.var_beta_dn13) * locals.var_t2) + (assign55080_e85225 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign55080_e85229;
        locals.var_t3_dn0 = assign55080_e85229_d_n0;
        locals.var_t3_dn2 = assign55080_e85229_d_n2;
        locals.var_t3_dn4 = assign55080_e85229_d_n4;
        locals.var_t3_dn5 = assign55080_e85229_d_n5;
        locals.var_t3_dn6 = assign55080_e85229_d_n6;
        locals.var_t3_dn7 = assign55080_e85229_d_n7;
        locals.var_t3_dn8 = assign55080_e85229_d_n8;
        locals.var_t3_dn9 = assign55080_e85229_d_n9;
        locals.var_t3_dn10 = assign55080_e85229_d_n10;
        locals.var_t3_dn13 = assign55080_e85229_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign55090_e85259, assign55090_e85259_d_n0, assign55090_e85259_d_n2, assign55090_e85259_d_n4, assign55090_e85259_d_n5, assign55090_e85259_d_n6, assign55090_e85259_d_n7, assign55090_e85259_d_n8, assign55090_e85259_d_n9, assign55090_e85259_d_n10, assign55090_e85259_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 == 0.0)) {
        let assign55090_e85249: f64 = (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv);
        let assign55090_e85251: f64 = (locals.var_t3).exp();
        let assign55090_e85253: f64 = (assign55090_e85251 - locals.var_t3);
        let assign55090_e85255: f64 = (assign55090_e85253 - 1.0);
        let assign55090_e85256: f64 = (assign55090_e85249 * assign55090_e85255);
        let assign55090_e85257: f64 = (assign55090_e85256).sqrt();
        (assign55090_e85257, (((((locals.var_c_2esipq_ndepm__blk1136_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn0)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn2)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn4)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn5)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn6)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn7)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn8)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn9)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn10)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign55090_e85257)), (((((locals.var_c_2esipq_ndepm__blk1136_dn13 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1136 * locals.var_beta_inv_dn13)) * assign55090_e85255) + (assign55090_e85249 * ((assign55090_e85251 * locals.var_t3_dn13) - locals.var_t3_dn13))) / (2.0 * assign55090_e85257)),)
    } else {
        (locals.var_ws__blk1147, locals.var_ws__blk1147_dn0, locals.var_ws__blk1147_dn2, locals.var_ws__blk1147_dn4, locals.var_ws__blk1147_dn5, locals.var_ws__blk1147_dn6, locals.var_ws__blk1147_dn7, locals.var_ws__blk1147_dn8, locals.var_ws__blk1147_dn9, locals.var_ws__blk1147_dn10, locals.var_ws__blk1147_dn13,)
    }
};
        locals.var_ws__blk1147 = assign55090_e85259;
        locals.var_ws__blk1147_dn0 = assign55090_e85259_d_n0;
        locals.var_ws__blk1147_dn2 = assign55090_e85259_d_n2;
        locals.var_ws__blk1147_dn4 = assign55090_e85259_d_n4;
        locals.var_ws__blk1147_dn5 = assign55090_e85259_d_n5;
        locals.var_ws__blk1147_dn6 = assign55090_e85259_d_n6;
        locals.var_ws__blk1147_dn7 = assign55090_e85259_d_n7;
        locals.var_ws__blk1147_dn8 = assign55090_e85259_d_n8;
        locals.var_ws__blk1147_dn9 = assign55090_e85259_d_n9;
        locals.var_ws__blk1147_dn10 = assign55090_e85259_d_n10;
        locals.var_ws__blk1147_dn13 = assign55090_e85259_d_n13;
        locals.var_ws__blk1147_rv = 0.0;

        let (assign55100_e85275, assign55100_e85275_d_n0, assign55100_e85275_d_n2, assign55100_e85275_d_n4, assign55100_e85275_d_n5, assign55100_e85275_d_n6, assign55100_e85275_d_n7, assign55100_e85275_d_n8, assign55100_e85275_d_n9, assign55100_e85275_d_n10, assign55100_e85275_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) {
        let assign55100_e85273: f64 = (locals.var_tnp__blk1148 - locals.var_ws__blk1147);
        (assign55100_e85273, (locals.var_tnp__blk1148_dn0 - locals.var_ws__blk1147_dn0), (locals.var_tnp__blk1148_dn2 - locals.var_ws__blk1147_dn2), (locals.var_tnp__blk1148_dn4 - locals.var_ws__blk1147_dn4), (locals.var_tnp__blk1148_dn5 - locals.var_ws__blk1147_dn5), (locals.var_tnp__blk1148_dn6 - locals.var_ws__blk1147_dn6), (locals.var_tnp__blk1148_dn7 - locals.var_ws__blk1147_dn7), (locals.var_tnp__blk1148_dn8 - locals.var_ws__blk1147_dn8), (locals.var_tnp__blk1148_dn9 - locals.var_ws__blk1147_dn9), (locals.var_tnp__blk1148_dn10 - locals.var_ws__blk1147_dn10), (locals.var_tnp__blk1148_dn13 - locals.var_ws__blk1147_dn13),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign55100_e85275;
        locals.var_w_res_dn0 = assign55100_e85275_d_n0;
        locals.var_w_res_dn2 = assign55100_e85275_d_n2;
        locals.var_w_res_dn4 = assign55100_e85275_d_n4;
        locals.var_w_res_dn5 = assign55100_e85275_d_n5;
        locals.var_w_res_dn6 = assign55100_e85275_d_n6;
        locals.var_w_res_dn7 = assign55100_e85275_d_n7;
        locals.var_w_res_dn8 = assign55100_e85275_d_n8;
        locals.var_w_res_dn9 = assign55100_e85275_d_n9;
        locals.var_w_res_dn10 = assign55100_e85275_d_n10;
        locals.var_w_res_dn13 = assign55100_e85275_d_n13;
        locals.var_w_res_rv = 0.0;

        let assign55110_e85279: f64 = 1e-16;
        let assign55110_e85284: f64 = if ((locals.var_w_res < assign55110_e85279) && (1e-16 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1388 = assign55110_e85284;
        locals.var_guard1388_rv = 0.0;

        let (assign55120_e85304, assign55120_e85304_d_n0, assign55120_e85304_d_n2, assign55120_e85304_d_n4, assign55120_e85304_d_n5, assign55120_e85304_d_n6, assign55120_e85304_d_n7, assign55120_e85304_d_n8, assign55120_e85304_d_n9, assign55120_e85304_d_n10, assign55120_e85304_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55120_e85300: f64 = 1e-16;
        let assign55120_e85302: f64 = (assign55120_e85300 - locals.var_w_res);
        (assign55120_e85302, (-locals.var_w_res_dn0), (-locals.var_w_res_dn2), (-locals.var_w_res_dn4), (-locals.var_w_res_dn5), (-locals.var_w_res_dn6), (-locals.var_w_res_dn7), (-locals.var_w_res_dn8), (-locals.var_w_res_dn9), (-locals.var_w_res_dn10), (-locals.var_w_res_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign55120_e85304;
        locals.var_tmf1_dn0 = assign55120_e85304_d_n0;
        locals.var_tmf1_dn2 = assign55120_e85304_d_n2;
        locals.var_tmf1_dn4 = assign55120_e85304_d_n4;
        locals.var_tmf1_dn5 = assign55120_e85304_d_n5;
        locals.var_tmf1_dn6 = assign55120_e85304_d_n6;
        locals.var_tmf1_dn7 = assign55120_e85304_d_n7;
        locals.var_tmf1_dn8 = assign55120_e85304_d_n8;
        locals.var_tmf1_dn9 = assign55120_e85304_d_n9;
        locals.var_tmf1_dn10 = assign55120_e85304_d_n10;
        locals.var_tmf1_dn13 = assign55120_e85304_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign55130_e85322, assign55130_e85322_d_n0, assign55130_e85322_d_n2, assign55130_e85322_d_n4, assign55130_e85322_d_n5, assign55130_e85322_d_n6, assign55130_e85322_d_n7, assign55130_e85322_d_n8, assign55130_e85322_d_n9, assign55130_e85322_d_n10, assign55130_e85322_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55130_e85320: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55130_e85320, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign55130_e85322;
        locals.var_x2_dn0 = assign55130_e85322_d_n0;
        locals.var_x2_dn2 = assign55130_e85322_d_n2;
        locals.var_x2_dn4 = assign55130_e85322_d_n4;
        locals.var_x2_dn5 = assign55130_e85322_d_n5;
        locals.var_x2_dn6 = assign55130_e85322_d_n6;
        locals.var_x2_dn7 = assign55130_e85322_d_n7;
        locals.var_x2_dn8 = assign55130_e85322_d_n8;
        locals.var_x2_dn9 = assign55130_e85322_d_n9;
        locals.var_x2_dn10 = assign55130_e85322_d_n10;
        locals.var_x2_dn13 = assign55130_e85322_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign55140_e85340, assign55140_e85340_d_n0, assign55140_e85340_d_n2, assign55140_e85340_d_n4, assign55140_e85340_d_n5, assign55140_e85340_d_n6, assign55140_e85340_d_n7, assign55140_e85340_d_n8, assign55140_e85340_d_n9, assign55140_e85340_d_n10, assign55140_e85340_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55140_e85338: f64 = (1e-16 * 1e-16);
        (assign55140_e85338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign55140_e85340;
        locals.var_xmax2_dn0 = assign55140_e85340_d_n0;
        locals.var_xmax2_dn2 = assign55140_e85340_d_n2;
        locals.var_xmax2_dn4 = assign55140_e85340_d_n4;
        locals.var_xmax2_dn5 = assign55140_e85340_d_n5;
        locals.var_xmax2_dn6 = assign55140_e85340_d_n6;
        locals.var_xmax2_dn7 = assign55140_e85340_d_n7;
        locals.var_xmax2_dn8 = assign55140_e85340_d_n8;
        locals.var_xmax2_dn9 = assign55140_e85340_d_n9;
        locals.var_xmax2_dn10 = assign55140_e85340_d_n10;
        locals.var_xmax2_dn13 = assign55140_e85340_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign55150_e85356, assign55150_e85356_d_n0, assign55150_e85356_d_n2, assign55150_e85356_d_n4, assign55150_e85356_d_n5, assign55150_e85356_d_n6, assign55150_e85356_d_n7, assign55150_e85356_d_n8, assign55150_e85356_d_n9, assign55150_e85356_d_n10, assign55150_e85356_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign55150_e85356;
        locals.var_xp_dn0 = assign55150_e85356_d_n0;
        locals.var_xp_dn2 = assign55150_e85356_d_n2;
        locals.var_xp_dn4 = assign55150_e85356_d_n4;
        locals.var_xp_dn5 = assign55150_e85356_d_n5;
        locals.var_xp_dn6 = assign55150_e85356_d_n6;
        locals.var_xp_dn7 = assign55150_e85356_d_n7;
        locals.var_xp_dn8 = assign55150_e85356_d_n8;
        locals.var_xp_dn9 = assign55150_e85356_d_n9;
        locals.var_xp_dn10 = assign55150_e85356_d_n10;
        locals.var_xp_dn13 = assign55150_e85356_d_n13;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_193(
        locals: &mut StampLocals,
    ) {
        let (assign55160_e85372, assign55160_e85372_d_n0, assign55160_e85372_d_n2, assign55160_e85372_d_n4, assign55160_e85372_d_n5, assign55160_e85372_d_n6, assign55160_e85372_d_n7, assign55160_e85372_d_n8, assign55160_e85372_d_n9, assign55160_e85372_d_n10, assign55160_e85372_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign55160_e85372;
        locals.var_xmp_dn0 = assign55160_e85372_d_n0;
        locals.var_xmp_dn2 = assign55160_e85372_d_n2;
        locals.var_xmp_dn4 = assign55160_e85372_d_n4;
        locals.var_xmp_dn5 = assign55160_e85372_d_n5;
        locals.var_xmp_dn6 = assign55160_e85372_d_n6;
        locals.var_xmp_dn7 = assign55160_e85372_d_n7;
        locals.var_xmp_dn8 = assign55160_e85372_d_n8;
        locals.var_xmp_dn9 = assign55160_e85372_d_n9;
        locals.var_xmp_dn10 = assign55160_e85372_d_n10;
        locals.var_xmp_dn13 = assign55160_e85372_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign55170_e85388,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55170_e85388;
        locals.var_m0_rv = 0.0;

        let (assign55180_e85404,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55180_e85404;
        locals.var_mm_rv = 0.0;

        let (assign55190_e85420, assign55190_e85420_d_n0, assign55190_e85420_d_n2, assign55190_e85420_d_n4, assign55190_e85420_d_n5, assign55190_e85420_d_n6, assign55190_e85420_d_n7, assign55190_e85420_d_n8, assign55190_e85420_d_n9, assign55190_e85420_d_n10, assign55190_e85420_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign55190_e85420;
        locals.var_arg_dn0 = assign55190_e85420_d_n0;
        locals.var_arg_dn2 = assign55190_e85420_d_n2;
        locals.var_arg_dn4 = assign55190_e85420_d_n4;
        locals.var_arg_dn5 = assign55190_e85420_d_n5;
        locals.var_arg_dn6 = assign55190_e85420_d_n6;
        locals.var_arg_dn7 = assign55190_e85420_d_n7;
        locals.var_arg_dn8 = assign55190_e85420_d_n8;
        locals.var_arg_dn9 = assign55190_e85420_d_n9;
        locals.var_arg_dn10 = assign55190_e85420_d_n10;
        locals.var_arg_dn13 = assign55190_e85420_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign55200_e85436, assign55200_e85436_d_n0, assign55200_e85436_d_n2, assign55200_e85436_d_n4, assign55200_e85436_d_n5, assign55200_e85436_d_n6, assign55200_e85436_d_n7, assign55200_e85436_d_n8, assign55200_e85436_d_n9, assign55200_e85436_d_n10, assign55200_e85436_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55200_e85436;
        locals.var_dnm_dn0 = assign55200_e85436_d_n0;
        locals.var_dnm_dn2 = assign55200_e85436_d_n2;
        locals.var_dnm_dn4 = assign55200_e85436_d_n4;
        locals.var_dnm_dn5 = assign55200_e85436_d_n5;
        locals.var_dnm_dn6 = assign55200_e85436_d_n6;
        locals.var_dnm_dn7 = assign55200_e85436_d_n7;
        locals.var_dnm_dn8 = assign55200_e85436_d_n8;
        locals.var_dnm_dn9 = assign55200_e85436_d_n9;
        locals.var_dnm_dn10 = assign55200_e85436_d_n10;
        locals.var_dnm_dn13 = assign55200_e85436_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign55210_e85454, assign55210_e85454_d_n0, assign55210_e85454_d_n2, assign55210_e85454_d_n4, assign55210_e85454_d_n5, assign55210_e85454_d_n6, assign55210_e85454_d_n7, assign55210_e85454_d_n8, assign55210_e85454_d_n9, assign55210_e85454_d_n10, assign55210_e85454_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55210_e85452: f64 = (locals.var_xp * locals.var_x2);
        (assign55210_e85452, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign55210_e85454;
        locals.var_xp_dn0 = assign55210_e85454_d_n0;
        locals.var_xp_dn2 = assign55210_e85454_d_n2;
        locals.var_xp_dn4 = assign55210_e85454_d_n4;
        locals.var_xp_dn5 = assign55210_e85454_d_n5;
        locals.var_xp_dn6 = assign55210_e85454_d_n6;
        locals.var_xp_dn7 = assign55210_e85454_d_n7;
        locals.var_xp_dn8 = assign55210_e85454_d_n8;
        locals.var_xp_dn9 = assign55210_e85454_d_n9;
        locals.var_xp_dn10 = assign55210_e85454_d_n10;
        locals.var_xp_dn13 = assign55210_e85454_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign55220_e85472, assign55220_e85472_d_n0, assign55220_e85472_d_n2, assign55220_e85472_d_n4, assign55220_e85472_d_n5, assign55220_e85472_d_n6, assign55220_e85472_d_n7, assign55220_e85472_d_n8, assign55220_e85472_d_n9, assign55220_e85472_d_n10, assign55220_e85472_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55220_e85470: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55220_e85470, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign55220_e85472;
        locals.var_xmp_dn0 = assign55220_e85472_d_n0;
        locals.var_xmp_dn2 = assign55220_e85472_d_n2;
        locals.var_xmp_dn4 = assign55220_e85472_d_n4;
        locals.var_xmp_dn5 = assign55220_e85472_d_n5;
        locals.var_xmp_dn6 = assign55220_e85472_d_n6;
        locals.var_xmp_dn7 = assign55220_e85472_d_n7;
        locals.var_xmp_dn8 = assign55220_e85472_d_n8;
        locals.var_xmp_dn9 = assign55220_e85472_d_n9;
        locals.var_xmp_dn10 = assign55220_e85472_d_n10;
        locals.var_xmp_dn13 = assign55220_e85472_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign55230_e85490, assign55230_e85490_d_n0, assign55230_e85490_d_n2, assign55230_e85490_d_n4, assign55230_e85490_d_n5, assign55230_e85490_d_n6, assign55230_e85490_d_n7, assign55230_e85490_d_n8, assign55230_e85490_d_n9, assign55230_e85490_d_n10, assign55230_e85490_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55230_e85488: f64 = (locals.var_xp * locals.var_x2);
        (assign55230_e85488, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign55230_e85490;
        locals.var_xp_dn0 = assign55230_e85490_d_n0;
        locals.var_xp_dn2 = assign55230_e85490_d_n2;
        locals.var_xp_dn4 = assign55230_e85490_d_n4;
        locals.var_xp_dn5 = assign55230_e85490_d_n5;
        locals.var_xp_dn6 = assign55230_e85490_d_n6;
        locals.var_xp_dn7 = assign55230_e85490_d_n7;
        locals.var_xp_dn8 = assign55230_e85490_d_n8;
        locals.var_xp_dn9 = assign55230_e85490_d_n9;
        locals.var_xp_dn10 = assign55230_e85490_d_n10;
        locals.var_xp_dn13 = assign55230_e85490_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign55240_e85508, assign55240_e85508_d_n0, assign55240_e85508_d_n2, assign55240_e85508_d_n4, assign55240_e85508_d_n5, assign55240_e85508_d_n6, assign55240_e85508_d_n7, assign55240_e85508_d_n8, assign55240_e85508_d_n9, assign55240_e85508_d_n10, assign55240_e85508_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55240_e85506: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55240_e85506, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign55240_e85508;
        locals.var_xmp_dn0 = assign55240_e85508_d_n0;
        locals.var_xmp_dn2 = assign55240_e85508_d_n2;
        locals.var_xmp_dn4 = assign55240_e85508_d_n4;
        locals.var_xmp_dn5 = assign55240_e85508_d_n5;
        locals.var_xmp_dn6 = assign55240_e85508_d_n6;
        locals.var_xmp_dn7 = assign55240_e85508_d_n7;
        locals.var_xmp_dn8 = assign55240_e85508_d_n8;
        locals.var_xmp_dn9 = assign55240_e85508_d_n9;
        locals.var_xmp_dn10 = assign55240_e85508_d_n10;
        locals.var_xmp_dn13 = assign55240_e85508_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign55250_e85526, assign55250_e85526_d_n0, assign55250_e85526_d_n2, assign55250_e85526_d_n4, assign55250_e85526_d_n5, assign55250_e85526_d_n6, assign55250_e85526_d_n7, assign55250_e85526_d_n8, assign55250_e85526_d_n9, assign55250_e85526_d_n10, assign55250_e85526_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55250_e85524: f64 = (locals.var_xp + locals.var_xmp);
        (assign55250_e85524, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign55250_e85526;
        locals.var_arg_dn0 = assign55250_e85526_d_n0;
        locals.var_arg_dn2 = assign55250_e85526_d_n2;
        locals.var_arg_dn4 = assign55250_e85526_d_n4;
        locals.var_arg_dn5 = assign55250_e85526_d_n5;
        locals.var_arg_dn6 = assign55250_e85526_d_n6;
        locals.var_arg_dn7 = assign55250_e85526_d_n7;
        locals.var_arg_dn8 = assign55250_e85526_d_n8;
        locals.var_arg_dn9 = assign55250_e85526_d_n9;
        locals.var_arg_dn10 = assign55250_e85526_d_n10;
        locals.var_arg_dn13 = assign55250_e85526_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign55260_e85542, assign55260_e85542_d_n0, assign55260_e85542_d_n2, assign55260_e85542_d_n4, assign55260_e85542_d_n5, assign55260_e85542_d_n6, assign55260_e85542_d_n7, assign55260_e85542_d_n8, assign55260_e85542_d_n9, assign55260_e85542_d_n10, assign55260_e85542_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55260_e85542;
        locals.var_dnm_dn0 = assign55260_e85542_d_n0;
        locals.var_dnm_dn2 = assign55260_e85542_d_n2;
        locals.var_dnm_dn4 = assign55260_e85542_d_n4;
        locals.var_dnm_dn5 = assign55260_e85542_d_n5;
        locals.var_dnm_dn6 = assign55260_e85542_d_n6;
        locals.var_dnm_dn7 = assign55260_e85542_d_n7;
        locals.var_dnm_dn8 = assign55260_e85542_d_n8;
        locals.var_dnm_dn9 = assign55260_e85542_d_n9;
        locals.var_dnm_dn10 = assign55260_e85542_d_n10;
        locals.var_dnm_dn13 = assign55260_e85542_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign55270_e85557: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1389 = assign55270_e85557;
        locals.var_guard1389_rv = 0.0;

        let assign55280_e85560: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1390 = assign55280_e85560;
        locals.var_guard1390_rv = 0.0;

        let (assign55290_e85580,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 != 0.0)) && (locals.var_guard1390 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55290_e85580;
        locals.var_mm_rv = 0.0;

        let assign55300_e85583: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1391 = assign55300_e85583;
        locals.var_guard1391_rv = 0.0;

        let (assign55310_e85606,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 != 0.0)) && (locals.var_guard1390 == 0.0)) && (locals.var_guard1391 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55310_e85606;
        locals.var_mm_rv = 0.0;

        let assign55320_e85609: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1392 = assign55320_e85609;
        locals.var_guard1392_rv = 0.0;

        let (assign55330_e85635,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 != 0.0)) && (locals.var_guard1390 == 0.0)) && (locals.var_guard1391 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55330_e85635;
        locals.var_mm_rv = 0.0;

        let assign55340_e85638: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1393 = assign55340_e85638;
        locals.var_guard1393_rv = 0.0;

        let (assign55350_e85667,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 != 0.0)) && (locals.var_guard1390 == 0.0)) && (locals.var_guard1391 == 0.0)) && (locals.var_guard1392 == 0.0)) && (locals.var_guard1393 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55350_e85667;
        locals.var_mm_rv = 0.0;

        let (assign55360_e85685,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55360_e85685;
        locals.var_m0_rv = 0.0;

        let mut assign55370_loop_guard: usize = 0;
        while {
            let assign55370_cond_e85704: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign55370_cond_e85704 != 0.0
        } {
            assign55370_loop_guard += 1;
            assert!(assign55370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign55370_body0_e85723, assign55370_body0_e85723_d_n0, assign55370_body0_e85723_d_n2, assign55370_body0_e85723_d_n4, assign55370_body0_e85723_d_n5, assign55370_body0_e85723_d_n6, assign55370_body0_e85723_d_n7, assign55370_body0_e85723_d_n8, assign55370_body0_e85723_d_n9, assign55370_body0_e85723_d_n10, assign55370_body0_e85723_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 != 0.0)) {
        let assign55370_body0_e85721: f64 = (locals.var_dnm).sqrt();
        (assign55370_body0_e85721, (locals.var_dnm_dn0 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn2 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn4 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn5 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn6 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn7 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn8 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn9 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn10 / (2.0 * assign55370_body0_e85721)), (locals.var_dnm_dn13 / (2.0 * assign55370_body0_e85721)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign55370_body0_e85723;
            locals.var_dnm_dn0 = assign55370_body0_e85723_d_n0;
            locals.var_dnm_dn2 = assign55370_body0_e85723_d_n2;
            locals.var_dnm_dn4 = assign55370_body0_e85723_d_n4;
            locals.var_dnm_dn5 = assign55370_body0_e85723_d_n5;
            locals.var_dnm_dn6 = assign55370_body0_e85723_d_n6;
            locals.var_dnm_dn7 = assign55370_body0_e85723_d_n7;
            locals.var_dnm_dn8 = assign55370_body0_e85723_d_n8;
            locals.var_dnm_dn9 = assign55370_body0_e85723_d_n9;
            locals.var_dnm_dn10 = assign55370_body0_e85723_d_n10;
            locals.var_dnm_dn13 = assign55370_body0_e85723_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign55370_body1_e85743,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 != 0.0)) {
        let assign55370_body1_e85741: f64 = (locals.var_m0 + 1.0);
        (assign55370_body1_e85741,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign55370_body1_e85743;
            locals.var_m0_rv = 0.0;
        }

        let (assign55380_e85773, assign55380_e85773_d_n0, assign55380_e85773_d_n2, assign55380_e85773_d_n4, assign55380_e85773_d_n5, assign55380_e85773_d_n6, assign55380_e85773_d_n7, assign55380_e85773_d_n8, assign55380_e85773_d_n9, assign55380_e85773_d_n10, assign55380_e85773_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) && (locals.var_guard1389 == 0.0)) {
        let (assign55380_e85771, assign55380_e85771_d_n0, assign55380_e85771_d_n2, assign55380_e85771_d_n4, assign55380_e85771_d_n5, assign55380_e85771_d_n6, assign55380_e85771_d_n7, assign55380_e85771_d_n8, assign55380_e85771_d_n9, assign55380_e85771_d_n10, assign55380_e85771_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign55380_e85768: f64 = (2.0 * 2.0);
                let assign55380_e85769: f64 = (1.0 / assign55380_e85768);
                let assign55380_e85770: f64 = (locals.var_dnm).powf(assign55380_e85769);
                (assign55380_e85770, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn0)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn2)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn4)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn5)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn6)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn7)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn8)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn9)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn10)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55380_e85769) as f64).is_finite() && ((assign55380_e85769) as f64).fract() == 0.0 { if assign55380_e85769 == 0.0 { 0.0 } else { (assign55380_e85769 * ((locals.var_dnm).powf(assign55380_e85769 - 1.0) * locals.var_dnm_dn13)) } } else { (assign55380_e85770 * (assign55380_e85769 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign55380_e85771, assign55380_e85771_d_n0, assign55380_e85771_d_n2, assign55380_e85771_d_n4, assign55380_e85771_d_n5, assign55380_e85771_d_n6, assign55380_e85771_d_n7, assign55380_e85771_d_n8, assign55380_e85771_d_n9, assign55380_e85771_d_n10, assign55380_e85771_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55380_e85773;
        locals.var_dnm_dn0 = assign55380_e85773_d_n0;
        locals.var_dnm_dn2 = assign55380_e85773_d_n2;
        locals.var_dnm_dn4 = assign55380_e85773_d_n4;
        locals.var_dnm_dn5 = assign55380_e85773_d_n5;
        locals.var_dnm_dn6 = assign55380_e85773_d_n6;
        locals.var_dnm_dn7 = assign55380_e85773_d_n7;
        locals.var_dnm_dn8 = assign55380_e85773_d_n8;
        locals.var_dnm_dn9 = assign55380_e85773_d_n9;
        locals.var_dnm_dn10 = assign55380_e85773_d_n10;
        locals.var_dnm_dn13 = assign55380_e85773_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign55390_e85791, assign55390_e85791_d_n0, assign55390_e85791_d_n2, assign55390_e85791_d_n4, assign55390_e85791_d_n5, assign55390_e85791_d_n6, assign55390_e85791_d_n7, assign55390_e85791_d_n8, assign55390_e85791_d_n9, assign55390_e85791_d_n10, assign55390_e85791_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55390_e85789: f64 = (1.0 / locals.var_dnm);
        (assign55390_e85789, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55390_e85791;
        locals.var_dnm_dn0 = assign55390_e85791_d_n0;
        locals.var_dnm_dn2 = assign55390_e85791_d_n2;
        locals.var_dnm_dn4 = assign55390_e85791_d_n4;
        locals.var_dnm_dn5 = assign55390_e85791_d_n5;
        locals.var_dnm_dn6 = assign55390_e85791_d_n6;
        locals.var_dnm_dn7 = assign55390_e85791_d_n7;
        locals.var_dnm_dn8 = assign55390_e85791_d_n8;
        locals.var_dnm_dn9 = assign55390_e85791_d_n9;
        locals.var_dnm_dn10 = assign55390_e85791_d_n10;
        locals.var_dnm_dn13 = assign55390_e85791_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign55400_e85811, assign55400_e85811_d_n0, assign55400_e85811_d_n2, assign55400_e85811_d_n4, assign55400_e85811_d_n5, assign55400_e85811_d_n6, assign55400_e85811_d_n7, assign55400_e85811_d_n8, assign55400_e85811_d_n9, assign55400_e85811_d_n10, assign55400_e85811_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55400_e85807: f64 = (locals.var_tmf1 * 1e-16);
        let assign55400_e85809: f64 = (assign55400_e85807 * locals.var_dnm);
        (assign55400_e85809, (((locals.var_tmf1_dn0 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-16) * locals.var_dnm) + (assign55400_e85807 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign55400_e85811;
        locals.var_tmf0_dn0 = assign55400_e85811_d_n0;
        locals.var_tmf0_dn2 = assign55400_e85811_d_n2;
        locals.var_tmf0_dn4 = assign55400_e85811_d_n4;
        locals.var_tmf0_dn5 = assign55400_e85811_d_n5;
        locals.var_tmf0_dn6 = assign55400_e85811_d_n6;
        locals.var_tmf0_dn7 = assign55400_e85811_d_n7;
        locals.var_tmf0_dn8 = assign55400_e85811_d_n8;
        locals.var_tmf0_dn9 = assign55400_e85811_d_n9;
        locals.var_tmf0_dn10 = assign55400_e85811_d_n10;
        locals.var_tmf0_dn13 = assign55400_e85811_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign55410_e85833, assign55410_e85833_d_n0, assign55410_e85833_d_n2, assign55410_e85833_d_n4, assign55410_e85833_d_n5, assign55410_e85833_d_n6, assign55410_e85833_d_n7, assign55410_e85833_d_n8, assign55410_e85833_d_n9, assign55410_e85833_d_n10, assign55410_e85833_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55410_e85827: f64 = (1e-16 * locals.var_xmp);
        let assign55410_e85829: f64 = (assign55410_e85827 * locals.var_dnm);
        let assign55410_e85831: f64 = (assign55410_e85829 / locals.var_arg);
        (assign55410_e85831, ((((((1e-16 * locals.var_xmp_dn0) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn0)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn2) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn2)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn4) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn4)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn5) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn5)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn6) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn6)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn7) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn7)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn8) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn8)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn9) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn9)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn10) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn10)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn13) * locals.var_dnm) + (assign55410_e85827 * locals.var_dnm_dn13)) * locals.var_arg) - (assign55410_e85829 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign55410_e85833;
        locals.var_t0_dn0 = assign55410_e85833_d_n0;
        locals.var_t0_dn2 = assign55410_e85833_d_n2;
        locals.var_t0_dn4 = assign55410_e85833_d_n4;
        locals.var_t0_dn5 = assign55410_e85833_d_n5;
        locals.var_t0_dn6 = assign55410_e85833_d_n6;
        locals.var_t0_dn7 = assign55410_e85833_d_n7;
        locals.var_t0_dn8 = assign55410_e85833_d_n8;
        locals.var_t0_dn9 = assign55410_e85833_d_n9;
        locals.var_t0_dn10 = assign55410_e85833_d_n10;
        locals.var_t0_dn13 = assign55410_e85833_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign55420_e85853, assign55420_e85853_d_n0, assign55420_e85853_d_n2, assign55420_e85853_d_n4, assign55420_e85853_d_n5, assign55420_e85853_d_n6, assign55420_e85853_d_n7, assign55420_e85853_d_n8, assign55420_e85853_d_n9, assign55420_e85853_d_n10, assign55420_e85853_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign55420_e85849: f64 = 1e-16;
        let assign55420_e85851: f64 = (assign55420_e85849 - locals.var_tmf0);
        (assign55420_e85851, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign55420_e85853;
        locals.var_w_res_dn0 = assign55420_e85853_d_n0;
        locals.var_w_res_dn2 = assign55420_e85853_d_n2;
        locals.var_w_res_dn4 = assign55420_e85853_d_n4;
        locals.var_w_res_dn5 = assign55420_e85853_d_n5;
        locals.var_w_res_dn6 = assign55420_e85853_d_n6;
        locals.var_w_res_dn7 = assign55420_e85853_d_n7;
        locals.var_w_res_dn8 = assign55420_e85853_d_n8;
        locals.var_w_res_dn9 = assign55420_e85853_d_n9;
        locals.var_w_res_dn10 = assign55420_e85853_d_n10;
        locals.var_w_res_dn13 = assign55420_e85853_d_n13;
        locals.var_w_res_rv = 0.0;

        let (assign55430_e85869, assign55430_e85869_d_n0, assign55430_e85869_d_n2, assign55430_e85869_d_n4, assign55430_e85869_d_n5, assign55430_e85869_d_n6, assign55430_e85869_d_n7, assign55430_e85869_d_n8, assign55430_e85869_d_n9, assign55430_e85869_d_n10, assign55430_e85869_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign55430_e85869;
        locals.var_t0_dn0 = assign55430_e85869_d_n0;
        locals.var_t0_dn2 = assign55430_e85869_d_n2;
        locals.var_t0_dn4 = assign55430_e85869_d_n4;
        locals.var_t0_dn5 = assign55430_e85869_d_n5;
        locals.var_t0_dn6 = assign55430_e85869_d_n6;
        locals.var_t0_dn7 = assign55430_e85869_d_n7;
        locals.var_t0_dn8 = assign55430_e85869_d_n8;
        locals.var_t0_dn9 = assign55430_e85869_d_n9;
        locals.var_t0_dn10 = assign55430_e85869_d_n10;
        locals.var_t0_dn13 = assign55430_e85869_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign55440_e85886, assign55440_e85886_d_n0, assign55440_e85886_d_n2, assign55440_e85886_d_n4, assign55440_e85886_d_n5, assign55440_e85886_d_n6, assign55440_e85886_d_n7, assign55440_e85886_d_n8, assign55440_e85886_d_n9, assign55440_e85886_d_n10, assign55440_e85886_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 == 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    }
};
        locals.var_w_res = assign55440_e85886;
        locals.var_w_res_dn0 = assign55440_e85886_d_n0;
        locals.var_w_res_dn2 = assign55440_e85886_d_n2;
        locals.var_w_res_dn4 = assign55440_e85886_d_n4;
        locals.var_w_res_dn5 = assign55440_e85886_d_n5;
        locals.var_w_res_dn6 = assign55440_e85886_d_n6;
        locals.var_w_res_dn7 = assign55440_e85886_d_n7;
        locals.var_w_res_dn8 = assign55440_e85886_d_n8;
        locals.var_w_res_dn9 = assign55440_e85886_d_n9;
        locals.var_w_res_dn10 = assign55440_e85886_d_n10;
        locals.var_w_res_dn13 = assign55440_e85886_d_n13;
        locals.var_w_res_rv = 0.0;

        let (assign55450_e85903, assign55450_e85903_d_n0, assign55450_e85903_d_n2, assign55450_e85903_d_n4, assign55450_e85903_d_n5, assign55450_e85903_d_n6, assign55450_e85903_d_n7, assign55450_e85903_d_n8, assign55450_e85903_d_n9, assign55450_e85903_d_n10, assign55450_e85903_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1388 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign55450_e85903;
        locals.var_t0_dn0 = assign55450_e85903_d_n0;
        locals.var_t0_dn2 = assign55450_e85903_d_n2;
        locals.var_t0_dn4 = assign55450_e85903_d_n4;
        locals.var_t0_dn5 = assign55450_e85903_d_n5;
        locals.var_t0_dn6 = assign55450_e85903_d_n6;
        locals.var_t0_dn7 = assign55450_e85903_d_n7;
        locals.var_t0_dn8 = assign55450_e85903_d_n8;
        locals.var_t0_dn9 = assign55450_e85903_d_n9;
        locals.var_t0_dn10 = assign55450_e85903_d_n10;
        locals.var_t0_dn13 = assign55450_e85903_d_n13;
        locals.var_t0_rv = 0.0;

        let assign55460_e85906: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1394 = assign55460_e85906;
        locals.var_guard1394_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_194(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign55470_e85922, assign55470_e85922_d_n0, assign55470_e85922_d_n2, assign55470_e85922_d_n4, assign55470_e85922_d_n5, assign55470_e85922_d_n6, assign55470_e85922_d_n7, assign55470_e85922_d_n8, assign55470_e85922_d_n9, assign55470_e85922_d_n10, assign55470_e85922_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1328 == 0.0)) && (locals.var_guard1394 != 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn13,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn13,)
    }
};
        locals.var_w_res_leak = assign55470_e85922;
        locals.var_w_res_leak_dn0 = assign55470_e85922_d_n0;
        locals.var_w_res_leak_dn2 = assign55470_e85922_d_n2;
        locals.var_w_res_leak_dn4 = assign55470_e85922_d_n4;
        locals.var_w_res_leak_dn5 = assign55470_e85922_d_n5;
        locals.var_w_res_leak_dn6 = assign55470_e85922_d_n6;
        locals.var_w_res_leak_dn7 = assign55470_e85922_d_n7;
        locals.var_w_res_leak_dn8 = assign55470_e85922_d_n8;
        locals.var_w_res_leak_dn9 = assign55470_e85922_d_n9;
        locals.var_w_res_leak_dn10 = assign55470_e85922_d_n10;
        locals.var_w_res_leak_dn13 = assign55470_e85922_d_n13;
        locals.var_w_res_leak_rv = 0.0;

        let assign55480_e85925: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1395 = assign55480_e85925;
        locals.var_guard1395_rv = 0.0;

        let (assign55490_e85938, assign55490_e85938_d_n0, assign55490_e85938_d_n2, assign55490_e85938_d_n4, assign55490_e85938_d_n5, assign55490_e85938_d_n6, assign55490_e85938_d_n7, assign55490_e85938_d_n8, assign55490_e85938_d_n9, assign55490_e85938_d_n10, assign55490_e85938_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn13,)
    }
};
        locals.var_vds_res = assign55490_e85938;
        locals.var_vds_res_dn0 = assign55490_e85938_d_n0;
        locals.var_vds_res_dn2 = assign55490_e85938_d_n2;
        locals.var_vds_res_dn4 = assign55490_e85938_d_n4;
        locals.var_vds_res_dn5 = assign55490_e85938_d_n5;
        locals.var_vds_res_dn6 = assign55490_e85938_d_n6;
        locals.var_vds_res_dn7 = assign55490_e85938_d_n7;
        locals.var_vds_res_dn8 = assign55490_e85938_d_n8;
        locals.var_vds_res_dn9 = assign55490_e85938_d_n9;
        locals.var_vds_res_dn10 = assign55490_e85938_d_n10;
        locals.var_vds_res_dn13 = assign55490_e85938_d_n13;
        locals.var_vds_res_rv = 0.0;

        let (assign55500_e85955, assign55500_e85955_d_n0, assign55500_e85955_d_n2, assign55500_e85955_d_n4, assign55500_e85955_d_n5, assign55500_e85955_d_n6, assign55500_e85955_d_n7, assign55500_e85955_d_n8, assign55500_e85955_d_n9, assign55500_e85955_d_n10, assign55500_e85955_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) {
        let assign55500_e85951: f64 = (locals.var_vbsc__blk1117 + locals.var_beta_inv);
        let assign55500_e85953: f64 = (assign55500_e85951 * p.p396);
        (assign55500_e85953, ((locals.var_vbsc__blk1117_dn0 + locals.var_beta_inv_dn0) * p.p396), ((locals.var_vbsc__blk1117_dn2 + locals.var_beta_inv_dn2) * p.p396), ((locals.var_vbsc__blk1117_dn4 + locals.var_beta_inv_dn4) * p.p396), ((locals.var_vbsc__blk1117_dn5 + locals.var_beta_inv_dn5) * p.p396), ((locals.var_vbsc__blk1117_dn6 + locals.var_beta_inv_dn6) * p.p396), ((locals.var_vbsc__blk1117_dn7 + locals.var_beta_inv_dn7) * p.p396), ((locals.var_vbsc__blk1117_dn8 + locals.var_beta_inv_dn8) * p.p396), ((locals.var_vbsc__blk1117_dn9 + locals.var_beta_inv_dn9) * p.p396), ((locals.var_vbsc__blk1117_dn10 + locals.var_beta_inv_dn10) * p.p396), ((locals.var_vbsc__blk1117_dn13 + locals.var_beta_inv_dn13) * p.p396),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign55500_e85955;
        locals.var_t10_dn0 = assign55500_e85955_d_n0;
        locals.var_t10_dn2 = assign55500_e85955_d_n2;
        locals.var_t10_dn4 = assign55500_e85955_d_n4;
        locals.var_t10_dn5 = assign55500_e85955_d_n5;
        locals.var_t10_dn6 = assign55500_e85955_d_n6;
        locals.var_t10_dn7 = assign55500_e85955_d_n7;
        locals.var_t10_dn8 = assign55500_e85955_d_n8;
        locals.var_t10_dn9 = assign55500_e85955_d_n9;
        locals.var_t10_dn10 = assign55500_e85955_d_n10;
        locals.var_t10_dn13 = assign55500_e85955_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign55510_e85974, assign55510_e85974_d_n0, assign55510_e85974_d_n2, assign55510_e85974_d_n4, assign55510_e85974_d_n5, assign55510_e85974_d_n6, assign55510_e85974_d_n7, assign55510_e85974_d_n8, assign55510_e85974_d_n9, assign55510_e85974_d_n10, assign55510_e85974_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) {
        let assign55510_e85970: f64 = (locals.var_vgp - locals.var_t10);
        let assign55510_e85971: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * assign55510_e85970);
        let assign55510_e85972: f64 = (1.0 + assign55510_e85971);
        (assign55510_e85972, ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn0 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn2 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn4 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn4 - locals.var_t10_dn4))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn5 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn5 - locals.var_t10_dn5))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn6 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn7 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn8 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn8 - locals.var_t10_dn8))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn9 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn9 - locals.var_t10_dn9))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn10 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn13 * assign55510_e85970) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * (locals.var_vgp_dn13 - locals.var_t10_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign55510_e85974;
        locals.var_t4_dn0 = assign55510_e85974_d_n0;
        locals.var_t4_dn2 = assign55510_e85974_d_n2;
        locals.var_t4_dn4 = assign55510_e85974_d_n4;
        locals.var_t4_dn5 = assign55510_e85974_d_n5;
        locals.var_t4_dn6 = assign55510_e85974_d_n6;
        locals.var_t4_dn7 = assign55510_e85974_d_n7;
        locals.var_t4_dn8 = assign55510_e85974_d_n8;
        locals.var_t4_dn9 = assign55510_e85974_d_n9;
        locals.var_t4_dn10 = assign55510_e85974_d_n10;
        locals.var_t4_dn13 = assign55510_e85974_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign55520_e85989, assign55520_e85989_d_n0, assign55520_e85989_d_n2, assign55520_e85989_d_n4, assign55520_e85989_d_n5, assign55520_e85989_d_n6, assign55520_e85989_d_n7, assign55520_e85989_d_n8, assign55520_e85989_d_n9, assign55520_e85989_d_n10, assign55520_e85989_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) {
        let assign55520_e85987: f64 = (1.0 + locals.var_c2_q_ndepm_esi_cox_inv2__blk1135);
        (assign55520_e85987, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn0, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn2, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn4, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn5, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn6, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn7, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn8, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn9, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn10, locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign55520_e85989;
        locals.var_t5_dn0 = assign55520_e85989_d_n0;
        locals.var_t5_dn2 = assign55520_e85989_d_n2;
        locals.var_t5_dn4 = assign55520_e85989_d_n4;
        locals.var_t5_dn5 = assign55520_e85989_d_n5;
        locals.var_t5_dn6 = assign55520_e85989_d_n6;
        locals.var_t5_dn7 = assign55520_e85989_d_n7;
        locals.var_t5_dn8 = assign55520_e85989_d_n8;
        locals.var_t5_dn9 = assign55520_e85989_d_n9;
        locals.var_t5_dn10 = assign55520_e85989_d_n10;
        locals.var_t5_dn13 = assign55520_e85989_d_n13;
        locals.var_t5_rv = 0.0;

        let assign55530_e85993: f64 = locals.var_t5;
        let assign55530_e85998: f64 = if ((locals.var_t4 < assign55530_e85993) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1396 = assign55530_e85998;
        locals.var_guard1396_rv = 0.0;

        let (assign55540_e86017, assign55540_e86017_d_n0, assign55540_e86017_d_n2, assign55540_e86017_d_n4, assign55540_e86017_d_n5, assign55540_e86017_d_n6, assign55540_e86017_d_n7, assign55540_e86017_d_n8, assign55540_e86017_d_n9, assign55540_e86017_d_n10, assign55540_e86017_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55540_e86013: f64 = locals.var_t5;
        let assign55540_e86015: f64 = (assign55540_e86013 - locals.var_t4);
        (assign55540_e86015, (locals.var_t5_dn0 - locals.var_t4_dn0), (locals.var_t5_dn2 - locals.var_t4_dn2), (locals.var_t5_dn4 - locals.var_t4_dn4), (locals.var_t5_dn5 - locals.var_t4_dn5), (locals.var_t5_dn6 - locals.var_t4_dn6), (locals.var_t5_dn7 - locals.var_t4_dn7), (locals.var_t5_dn8 - locals.var_t4_dn8), (locals.var_t5_dn9 - locals.var_t4_dn9), (locals.var_t5_dn10 - locals.var_t4_dn10), (locals.var_t5_dn13 - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign55540_e86017;
        locals.var_tmf1_dn0 = assign55540_e86017_d_n0;
        locals.var_tmf1_dn2 = assign55540_e86017_d_n2;
        locals.var_tmf1_dn4 = assign55540_e86017_d_n4;
        locals.var_tmf1_dn5 = assign55540_e86017_d_n5;
        locals.var_tmf1_dn6 = assign55540_e86017_d_n6;
        locals.var_tmf1_dn7 = assign55540_e86017_d_n7;
        locals.var_tmf1_dn8 = assign55540_e86017_d_n8;
        locals.var_tmf1_dn9 = assign55540_e86017_d_n9;
        locals.var_tmf1_dn10 = assign55540_e86017_d_n10;
        locals.var_tmf1_dn13 = assign55540_e86017_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign55550_e86034, assign55550_e86034_d_n0, assign55550_e86034_d_n2, assign55550_e86034_d_n4, assign55550_e86034_d_n5, assign55550_e86034_d_n6, assign55550_e86034_d_n7, assign55550_e86034_d_n8, assign55550_e86034_d_n9, assign55550_e86034_d_n10, assign55550_e86034_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55550_e86032: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55550_e86032, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign55550_e86034;
        locals.var_x2_dn0 = assign55550_e86034_d_n0;
        locals.var_x2_dn2 = assign55550_e86034_d_n2;
        locals.var_x2_dn4 = assign55550_e86034_d_n4;
        locals.var_x2_dn5 = assign55550_e86034_d_n5;
        locals.var_x2_dn6 = assign55550_e86034_d_n6;
        locals.var_x2_dn7 = assign55550_e86034_d_n7;
        locals.var_x2_dn8 = assign55550_e86034_d_n8;
        locals.var_x2_dn9 = assign55550_e86034_d_n9;
        locals.var_x2_dn10 = assign55550_e86034_d_n10;
        locals.var_x2_dn13 = assign55550_e86034_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign55560_e86051, assign55560_e86051_d_n0, assign55560_e86051_d_n2, assign55560_e86051_d_n4, assign55560_e86051_d_n5, assign55560_e86051_d_n6, assign55560_e86051_d_n7, assign55560_e86051_d_n8, assign55560_e86051_d_n9, assign55560_e86051_d_n10, assign55560_e86051_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55560_e86049: f64 = (locals.var_t5 * locals.var_t5);
        (assign55560_e86049, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign55560_e86051;
        locals.var_xmax2_dn0 = assign55560_e86051_d_n0;
        locals.var_xmax2_dn2 = assign55560_e86051_d_n2;
        locals.var_xmax2_dn4 = assign55560_e86051_d_n4;
        locals.var_xmax2_dn5 = assign55560_e86051_d_n5;
        locals.var_xmax2_dn6 = assign55560_e86051_d_n6;
        locals.var_xmax2_dn7 = assign55560_e86051_d_n7;
        locals.var_xmax2_dn8 = assign55560_e86051_d_n8;
        locals.var_xmax2_dn9 = assign55560_e86051_d_n9;
        locals.var_xmax2_dn10 = assign55560_e86051_d_n10;
        locals.var_xmax2_dn13 = assign55560_e86051_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign55570_e86066, assign55570_e86066_d_n0, assign55570_e86066_d_n2, assign55570_e86066_d_n4, assign55570_e86066_d_n5, assign55570_e86066_d_n6, assign55570_e86066_d_n7, assign55570_e86066_d_n8, assign55570_e86066_d_n9, assign55570_e86066_d_n10, assign55570_e86066_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign55570_e86066;
        locals.var_xp_dn0 = assign55570_e86066_d_n0;
        locals.var_xp_dn2 = assign55570_e86066_d_n2;
        locals.var_xp_dn4 = assign55570_e86066_d_n4;
        locals.var_xp_dn5 = assign55570_e86066_d_n5;
        locals.var_xp_dn6 = assign55570_e86066_d_n6;
        locals.var_xp_dn7 = assign55570_e86066_d_n7;
        locals.var_xp_dn8 = assign55570_e86066_d_n8;
        locals.var_xp_dn9 = assign55570_e86066_d_n9;
        locals.var_xp_dn10 = assign55570_e86066_d_n10;
        locals.var_xp_dn13 = assign55570_e86066_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign55580_e86081, assign55580_e86081_d_n0, assign55580_e86081_d_n2, assign55580_e86081_d_n4, assign55580_e86081_d_n5, assign55580_e86081_d_n6, assign55580_e86081_d_n7, assign55580_e86081_d_n8, assign55580_e86081_d_n9, assign55580_e86081_d_n10, assign55580_e86081_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign55580_e86081;
        locals.var_xmp_dn0 = assign55580_e86081_d_n0;
        locals.var_xmp_dn2 = assign55580_e86081_d_n2;
        locals.var_xmp_dn4 = assign55580_e86081_d_n4;
        locals.var_xmp_dn5 = assign55580_e86081_d_n5;
        locals.var_xmp_dn6 = assign55580_e86081_d_n6;
        locals.var_xmp_dn7 = assign55580_e86081_d_n7;
        locals.var_xmp_dn8 = assign55580_e86081_d_n8;
        locals.var_xmp_dn9 = assign55580_e86081_d_n9;
        locals.var_xmp_dn10 = assign55580_e86081_d_n10;
        locals.var_xmp_dn13 = assign55580_e86081_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign55590_e86096,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55590_e86096;
        locals.var_m0_rv = 0.0;

        let (assign55600_e86111,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55600_e86111;
        locals.var_mm_rv = 0.0;

        let (assign55610_e86126, assign55610_e86126_d_n0, assign55610_e86126_d_n2, assign55610_e86126_d_n4, assign55610_e86126_d_n5, assign55610_e86126_d_n6, assign55610_e86126_d_n7, assign55610_e86126_d_n8, assign55610_e86126_d_n9, assign55610_e86126_d_n10, assign55610_e86126_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign55610_e86126;
        locals.var_arg_dn0 = assign55610_e86126_d_n0;
        locals.var_arg_dn2 = assign55610_e86126_d_n2;
        locals.var_arg_dn4 = assign55610_e86126_d_n4;
        locals.var_arg_dn5 = assign55610_e86126_d_n5;
        locals.var_arg_dn6 = assign55610_e86126_d_n6;
        locals.var_arg_dn7 = assign55610_e86126_d_n7;
        locals.var_arg_dn8 = assign55610_e86126_d_n8;
        locals.var_arg_dn9 = assign55610_e86126_d_n9;
        locals.var_arg_dn10 = assign55610_e86126_d_n10;
        locals.var_arg_dn13 = assign55610_e86126_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign55620_e86141, assign55620_e86141_d_n0, assign55620_e86141_d_n2, assign55620_e86141_d_n4, assign55620_e86141_d_n5, assign55620_e86141_d_n6, assign55620_e86141_d_n7, assign55620_e86141_d_n8, assign55620_e86141_d_n9, assign55620_e86141_d_n10, assign55620_e86141_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55620_e86141;
        locals.var_dnm_dn0 = assign55620_e86141_d_n0;
        locals.var_dnm_dn2 = assign55620_e86141_d_n2;
        locals.var_dnm_dn4 = assign55620_e86141_d_n4;
        locals.var_dnm_dn5 = assign55620_e86141_d_n5;
        locals.var_dnm_dn6 = assign55620_e86141_d_n6;
        locals.var_dnm_dn7 = assign55620_e86141_d_n7;
        locals.var_dnm_dn8 = assign55620_e86141_d_n8;
        locals.var_dnm_dn9 = assign55620_e86141_d_n9;
        locals.var_dnm_dn10 = assign55620_e86141_d_n10;
        locals.var_dnm_dn13 = assign55620_e86141_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign55630_e86158, assign55630_e86158_d_n0, assign55630_e86158_d_n2, assign55630_e86158_d_n4, assign55630_e86158_d_n5, assign55630_e86158_d_n6, assign55630_e86158_d_n7, assign55630_e86158_d_n8, assign55630_e86158_d_n9, assign55630_e86158_d_n10, assign55630_e86158_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55630_e86156: f64 = (locals.var_xp * locals.var_x2);
        (assign55630_e86156, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign55630_e86158;
        locals.var_xp_dn0 = assign55630_e86158_d_n0;
        locals.var_xp_dn2 = assign55630_e86158_d_n2;
        locals.var_xp_dn4 = assign55630_e86158_d_n4;
        locals.var_xp_dn5 = assign55630_e86158_d_n5;
        locals.var_xp_dn6 = assign55630_e86158_d_n6;
        locals.var_xp_dn7 = assign55630_e86158_d_n7;
        locals.var_xp_dn8 = assign55630_e86158_d_n8;
        locals.var_xp_dn9 = assign55630_e86158_d_n9;
        locals.var_xp_dn10 = assign55630_e86158_d_n10;
        locals.var_xp_dn13 = assign55630_e86158_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign55640_e86175, assign55640_e86175_d_n0, assign55640_e86175_d_n2, assign55640_e86175_d_n4, assign55640_e86175_d_n5, assign55640_e86175_d_n6, assign55640_e86175_d_n7, assign55640_e86175_d_n8, assign55640_e86175_d_n9, assign55640_e86175_d_n10, assign55640_e86175_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55640_e86173: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55640_e86173, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign55640_e86175;
        locals.var_xmp_dn0 = assign55640_e86175_d_n0;
        locals.var_xmp_dn2 = assign55640_e86175_d_n2;
        locals.var_xmp_dn4 = assign55640_e86175_d_n4;
        locals.var_xmp_dn5 = assign55640_e86175_d_n5;
        locals.var_xmp_dn6 = assign55640_e86175_d_n6;
        locals.var_xmp_dn7 = assign55640_e86175_d_n7;
        locals.var_xmp_dn8 = assign55640_e86175_d_n8;
        locals.var_xmp_dn9 = assign55640_e86175_d_n9;
        locals.var_xmp_dn10 = assign55640_e86175_d_n10;
        locals.var_xmp_dn13 = assign55640_e86175_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign55650_e86192, assign55650_e86192_d_n0, assign55650_e86192_d_n2, assign55650_e86192_d_n4, assign55650_e86192_d_n5, assign55650_e86192_d_n6, assign55650_e86192_d_n7, assign55650_e86192_d_n8, assign55650_e86192_d_n9, assign55650_e86192_d_n10, assign55650_e86192_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55650_e86190: f64 = (locals.var_xp * locals.var_x2);
        (assign55650_e86190, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign55650_e86192;
        locals.var_xp_dn0 = assign55650_e86192_d_n0;
        locals.var_xp_dn2 = assign55650_e86192_d_n2;
        locals.var_xp_dn4 = assign55650_e86192_d_n4;
        locals.var_xp_dn5 = assign55650_e86192_d_n5;
        locals.var_xp_dn6 = assign55650_e86192_d_n6;
        locals.var_xp_dn7 = assign55650_e86192_d_n7;
        locals.var_xp_dn8 = assign55650_e86192_d_n8;
        locals.var_xp_dn9 = assign55650_e86192_d_n9;
        locals.var_xp_dn10 = assign55650_e86192_d_n10;
        locals.var_xp_dn13 = assign55650_e86192_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign55660_e86209, assign55660_e86209_d_n0, assign55660_e86209_d_n2, assign55660_e86209_d_n4, assign55660_e86209_d_n5, assign55660_e86209_d_n6, assign55660_e86209_d_n7, assign55660_e86209_d_n8, assign55660_e86209_d_n9, assign55660_e86209_d_n10, assign55660_e86209_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55660_e86207: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55660_e86207, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign55660_e86209;
        locals.var_xmp_dn0 = assign55660_e86209_d_n0;
        locals.var_xmp_dn2 = assign55660_e86209_d_n2;
        locals.var_xmp_dn4 = assign55660_e86209_d_n4;
        locals.var_xmp_dn5 = assign55660_e86209_d_n5;
        locals.var_xmp_dn6 = assign55660_e86209_d_n6;
        locals.var_xmp_dn7 = assign55660_e86209_d_n7;
        locals.var_xmp_dn8 = assign55660_e86209_d_n8;
        locals.var_xmp_dn9 = assign55660_e86209_d_n9;
        locals.var_xmp_dn10 = assign55660_e86209_d_n10;
        locals.var_xmp_dn13 = assign55660_e86209_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign55670_e86226, assign55670_e86226_d_n0, assign55670_e86226_d_n2, assign55670_e86226_d_n4, assign55670_e86226_d_n5, assign55670_e86226_d_n6, assign55670_e86226_d_n7, assign55670_e86226_d_n8, assign55670_e86226_d_n9, assign55670_e86226_d_n10, assign55670_e86226_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55670_e86224: f64 = (locals.var_xp + locals.var_xmp);
        (assign55670_e86224, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign55670_e86226;
        locals.var_arg_dn0 = assign55670_e86226_d_n0;
        locals.var_arg_dn2 = assign55670_e86226_d_n2;
        locals.var_arg_dn4 = assign55670_e86226_d_n4;
        locals.var_arg_dn5 = assign55670_e86226_d_n5;
        locals.var_arg_dn6 = assign55670_e86226_d_n6;
        locals.var_arg_dn7 = assign55670_e86226_d_n7;
        locals.var_arg_dn8 = assign55670_e86226_d_n8;
        locals.var_arg_dn9 = assign55670_e86226_d_n9;
        locals.var_arg_dn10 = assign55670_e86226_d_n10;
        locals.var_arg_dn13 = assign55670_e86226_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign55680_e86241, assign55680_e86241_d_n0, assign55680_e86241_d_n2, assign55680_e86241_d_n4, assign55680_e86241_d_n5, assign55680_e86241_d_n6, assign55680_e86241_d_n7, assign55680_e86241_d_n8, assign55680_e86241_d_n9, assign55680_e86241_d_n10, assign55680_e86241_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55680_e86241;
        locals.var_dnm_dn0 = assign55680_e86241_d_n0;
        locals.var_dnm_dn2 = assign55680_e86241_d_n2;
        locals.var_dnm_dn4 = assign55680_e86241_d_n4;
        locals.var_dnm_dn5 = assign55680_e86241_d_n5;
        locals.var_dnm_dn6 = assign55680_e86241_d_n6;
        locals.var_dnm_dn7 = assign55680_e86241_d_n7;
        locals.var_dnm_dn8 = assign55680_e86241_d_n8;
        locals.var_dnm_dn9 = assign55680_e86241_d_n9;
        locals.var_dnm_dn10 = assign55680_e86241_d_n10;
        locals.var_dnm_dn13 = assign55680_e86241_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign55690_e86256: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1397 = assign55690_e86256;
        locals.var_guard1397_rv = 0.0;

        let assign55700_e86259: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1398 = assign55700_e86259;
        locals.var_guard1398_rv = 0.0;

        let (assign55710_e86278,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55710_e86278;
        locals.var_mm_rv = 0.0;

        let assign55720_e86281: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1399 = assign55720_e86281;
        locals.var_guard1399_rv = 0.0;

        let (assign55730_e86303,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 == 0.0)) && (locals.var_guard1399 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55730_e86303;
        locals.var_mm_rv = 0.0;

        let assign55740_e86306: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1400 = assign55740_e86306;
        locals.var_guard1400_rv = 0.0;

        let (assign55750_e86331,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 == 0.0)) && (locals.var_guard1399 == 0.0)) && (locals.var_guard1400 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55750_e86331;
        locals.var_mm_rv = 0.0;

        let assign55760_e86334: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1401 = assign55760_e86334;
        locals.var_guard1401_rv = 0.0;

        let (assign55770_e86362,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) && (locals.var_guard1398 == 0.0)) && (locals.var_guard1399 == 0.0)) && (locals.var_guard1400 == 0.0)) && (locals.var_guard1401 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55770_e86362;
        locals.var_mm_rv = 0.0;

        let (assign55780_e86379,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55780_e86379;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_195(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign55790_loop_guard: usize = 0;
        while {
            let assign55790_cond_e86397: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign55790_cond_e86397 != 0.0
        } {
            assign55790_loop_guard += 1;
            assert!(assign55790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign55790_body0_e86415, assign55790_body0_e86415_d_n0, assign55790_body0_e86415_d_n2, assign55790_body0_e86415_d_n4, assign55790_body0_e86415_d_n5, assign55790_body0_e86415_d_n6, assign55790_body0_e86415_d_n7, assign55790_body0_e86415_d_n8, assign55790_body0_e86415_d_n9, assign55790_body0_e86415_d_n10, assign55790_body0_e86415_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) {
        let assign55790_body0_e86413: f64 = (locals.var_dnm).sqrt();
        (assign55790_body0_e86413, (locals.var_dnm_dn0 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn2 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn4 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn5 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn6 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn7 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn8 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn9 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn10 / (2.0 * assign55790_body0_e86413)), (locals.var_dnm_dn13 / (2.0 * assign55790_body0_e86413)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign55790_body0_e86415;
            locals.var_dnm_dn0 = assign55790_body0_e86415_d_n0;
            locals.var_dnm_dn2 = assign55790_body0_e86415_d_n2;
            locals.var_dnm_dn4 = assign55790_body0_e86415_d_n4;
            locals.var_dnm_dn5 = assign55790_body0_e86415_d_n5;
            locals.var_dnm_dn6 = assign55790_body0_e86415_d_n6;
            locals.var_dnm_dn7 = assign55790_body0_e86415_d_n7;
            locals.var_dnm_dn8 = assign55790_body0_e86415_d_n8;
            locals.var_dnm_dn9 = assign55790_body0_e86415_d_n9;
            locals.var_dnm_dn10 = assign55790_body0_e86415_d_n10;
            locals.var_dnm_dn13 = assign55790_body0_e86415_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign55790_body1_e86434,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) {
        let assign55790_body1_e86432: f64 = (locals.var_m0 + 1.0);
        (assign55790_body1_e86432,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign55790_body1_e86434;
            locals.var_m0_rv = 0.0;
        }

        let (assign55800_e86463, assign55800_e86463_d_n0, assign55800_e86463_d_n2, assign55800_e86463_d_n4, assign55800_e86463_d_n5, assign55800_e86463_d_n6, assign55800_e86463_d_n7, assign55800_e86463_d_n8, assign55800_e86463_d_n9, assign55800_e86463_d_n10, assign55800_e86463_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 == 0.0)) {
        let (assign55800_e86461, assign55800_e86461_d_n0, assign55800_e86461_d_n2, assign55800_e86461_d_n4, assign55800_e86461_d_n5, assign55800_e86461_d_n6, assign55800_e86461_d_n7, assign55800_e86461_d_n8, assign55800_e86461_d_n9, assign55800_e86461_d_n10, assign55800_e86461_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign55800_e86458: f64 = (2.0 * 2.0);
                let assign55800_e86459: f64 = (1.0 / assign55800_e86458);
                let assign55800_e86460: f64 = (locals.var_dnm).powf(assign55800_e86459);
                (assign55800_e86460, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn0)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn2)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn4)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn5)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn6)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn7)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn8)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn9)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn10)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55800_e86459) as f64).is_finite() && ((assign55800_e86459) as f64).fract() == 0.0 { if assign55800_e86459 == 0.0 { 0.0 } else { (assign55800_e86459 * ((locals.var_dnm).powf(assign55800_e86459 - 1.0) * locals.var_dnm_dn13)) } } else { (assign55800_e86460 * (assign55800_e86459 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign55800_e86461, assign55800_e86461_d_n0, assign55800_e86461_d_n2, assign55800_e86461_d_n4, assign55800_e86461_d_n5, assign55800_e86461_d_n6, assign55800_e86461_d_n7, assign55800_e86461_d_n8, assign55800_e86461_d_n9, assign55800_e86461_d_n10, assign55800_e86461_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55800_e86463;
        locals.var_dnm_dn0 = assign55800_e86463_d_n0;
        locals.var_dnm_dn2 = assign55800_e86463_d_n2;
        locals.var_dnm_dn4 = assign55800_e86463_d_n4;
        locals.var_dnm_dn5 = assign55800_e86463_d_n5;
        locals.var_dnm_dn6 = assign55800_e86463_d_n6;
        locals.var_dnm_dn7 = assign55800_e86463_d_n7;
        locals.var_dnm_dn8 = assign55800_e86463_d_n8;
        locals.var_dnm_dn9 = assign55800_e86463_d_n9;
        locals.var_dnm_dn10 = assign55800_e86463_d_n10;
        locals.var_dnm_dn13 = assign55800_e86463_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign55810_e86480, assign55810_e86480_d_n0, assign55810_e86480_d_n2, assign55810_e86480_d_n4, assign55810_e86480_d_n5, assign55810_e86480_d_n6, assign55810_e86480_d_n7, assign55810_e86480_d_n8, assign55810_e86480_d_n9, assign55810_e86480_d_n10, assign55810_e86480_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55810_e86478: f64 = (1.0 / locals.var_dnm);
        (assign55810_e86478, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55810_e86480;
        locals.var_dnm_dn0 = assign55810_e86480_d_n0;
        locals.var_dnm_dn2 = assign55810_e86480_d_n2;
        locals.var_dnm_dn4 = assign55810_e86480_d_n4;
        locals.var_dnm_dn5 = assign55810_e86480_d_n5;
        locals.var_dnm_dn6 = assign55810_e86480_d_n6;
        locals.var_dnm_dn7 = assign55810_e86480_d_n7;
        locals.var_dnm_dn8 = assign55810_e86480_d_n8;
        locals.var_dnm_dn9 = assign55810_e86480_d_n9;
        locals.var_dnm_dn10 = assign55810_e86480_d_n10;
        locals.var_dnm_dn13 = assign55810_e86480_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign55820_e86499, assign55820_e86499_d_n0, assign55820_e86499_d_n2, assign55820_e86499_d_n4, assign55820_e86499_d_n5, assign55820_e86499_d_n6, assign55820_e86499_d_n7, assign55820_e86499_d_n8, assign55820_e86499_d_n9, assign55820_e86499_d_n10, assign55820_e86499_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55820_e86495: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign55820_e86497: f64 = (assign55820_e86495 * locals.var_dnm);
        (assign55820_e86497, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn4)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn5)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn8)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn9)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn13)) * locals.var_dnm) + (assign55820_e86495 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign55820_e86499;
        locals.var_tmf0_dn0 = assign55820_e86499_d_n0;
        locals.var_tmf0_dn2 = assign55820_e86499_d_n2;
        locals.var_tmf0_dn4 = assign55820_e86499_d_n4;
        locals.var_tmf0_dn5 = assign55820_e86499_d_n5;
        locals.var_tmf0_dn6 = assign55820_e86499_d_n6;
        locals.var_tmf0_dn7 = assign55820_e86499_d_n7;
        locals.var_tmf0_dn8 = assign55820_e86499_d_n8;
        locals.var_tmf0_dn9 = assign55820_e86499_d_n9;
        locals.var_tmf0_dn10 = assign55820_e86499_d_n10;
        locals.var_tmf0_dn13 = assign55820_e86499_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign55830_e86520, assign55830_e86520_d_n0, assign55830_e86520_d_n2, assign55830_e86520_d_n4, assign55830_e86520_d_n5, assign55830_e86520_d_n6, assign55830_e86520_d_n7, assign55830_e86520_d_n8, assign55830_e86520_d_n9, assign55830_e86520_d_n10, assign55830_e86520_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55830_e86514: f64 = (locals.var_t5 * locals.var_xmp);
        let assign55830_e86516: f64 = (assign55830_e86514 * locals.var_dnm);
        let assign55830_e86518: f64 = (assign55830_e86516 / locals.var_arg);
        (assign55830_e86518, (((((((locals.var_t5_dn0 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn0)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn2 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn2)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn4 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn4)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn5 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn5)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn6 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn6)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn7 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn7)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn8 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn8)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn9 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn9)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn10 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn10)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn13 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign55830_e86514 * locals.var_dnm_dn13)) * locals.var_arg) - (assign55830_e86516 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign55830_e86520;
        locals.var_t0_dn0 = assign55830_e86520_d_n0;
        locals.var_t0_dn2 = assign55830_e86520_d_n2;
        locals.var_t0_dn4 = assign55830_e86520_d_n4;
        locals.var_t0_dn5 = assign55830_e86520_d_n5;
        locals.var_t0_dn6 = assign55830_e86520_d_n6;
        locals.var_t0_dn7 = assign55830_e86520_d_n7;
        locals.var_t0_dn8 = assign55830_e86520_d_n8;
        locals.var_t0_dn9 = assign55830_e86520_d_n9;
        locals.var_t0_dn10 = assign55830_e86520_d_n10;
        locals.var_t0_dn13 = assign55830_e86520_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign55840_e86539, assign55840_e86539_d_n0, assign55840_e86539_d_n2, assign55840_e86539_d_n4, assign55840_e86539_d_n5, assign55840_e86539_d_n6, assign55840_e86539_d_n7, assign55840_e86539_d_n8, assign55840_e86539_d_n9, assign55840_e86539_d_n10, assign55840_e86539_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign55840_e86535: f64 = locals.var_t5;
        let assign55840_e86537: f64 = (assign55840_e86535 - locals.var_tmf0);
        (assign55840_e86537, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn4 - locals.var_tmf0_dn4), (locals.var_t5_dn5 - locals.var_tmf0_dn5), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn8 - locals.var_tmf0_dn8), (locals.var_t5_dn9 - locals.var_tmf0_dn9), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign55840_e86539;
        locals.var_t4_dn0 = assign55840_e86539_d_n0;
        locals.var_t4_dn2 = assign55840_e86539_d_n2;
        locals.var_t4_dn4 = assign55840_e86539_d_n4;
        locals.var_t4_dn5 = assign55840_e86539_d_n5;
        locals.var_t4_dn6 = assign55840_e86539_d_n6;
        locals.var_t4_dn7 = assign55840_e86539_d_n7;
        locals.var_t4_dn8 = assign55840_e86539_d_n8;
        locals.var_t4_dn9 = assign55840_e86539_d_n9;
        locals.var_t4_dn10 = assign55840_e86539_d_n10;
        locals.var_t4_dn13 = assign55840_e86539_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign55850_e86554, assign55850_e86554_d_n0, assign55850_e86554_d_n2, assign55850_e86554_d_n4, assign55850_e86554_d_n5, assign55850_e86554_d_n6, assign55850_e86554_d_n7, assign55850_e86554_d_n8, assign55850_e86554_d_n9, assign55850_e86554_d_n10, assign55850_e86554_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign55850_e86554;
        locals.var_t0_dn0 = assign55850_e86554_d_n0;
        locals.var_t0_dn2 = assign55850_e86554_d_n2;
        locals.var_t0_dn4 = assign55850_e86554_d_n4;
        locals.var_t0_dn5 = assign55850_e86554_d_n5;
        locals.var_t0_dn6 = assign55850_e86554_d_n6;
        locals.var_t0_dn7 = assign55850_e86554_d_n7;
        locals.var_t0_dn8 = assign55850_e86554_d_n8;
        locals.var_t0_dn9 = assign55850_e86554_d_n9;
        locals.var_t0_dn10 = assign55850_e86554_d_n10;
        locals.var_t0_dn13 = assign55850_e86554_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign55860_e86570, assign55860_e86570_d_n0, assign55860_e86570_d_n2, assign55860_e86570_d_n4, assign55860_e86570_d_n5, assign55860_e86570_d_n6, assign55860_e86570_d_n7, assign55860_e86570_d_n8, assign55860_e86570_d_n9, assign55860_e86570_d_n10, assign55860_e86570_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign55860_e86570;
        locals.var_t4_dn0 = assign55860_e86570_d_n0;
        locals.var_t4_dn2 = assign55860_e86570_d_n2;
        locals.var_t4_dn4 = assign55860_e86570_d_n4;
        locals.var_t4_dn5 = assign55860_e86570_d_n5;
        locals.var_t4_dn6 = assign55860_e86570_d_n6;
        locals.var_t4_dn7 = assign55860_e86570_d_n7;
        locals.var_t4_dn8 = assign55860_e86570_d_n8;
        locals.var_t4_dn9 = assign55860_e86570_d_n9;
        locals.var_t4_dn10 = assign55860_e86570_d_n10;
        locals.var_t4_dn13 = assign55860_e86570_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign55870_e86586, assign55870_e86586_d_n0, assign55870_e86586_d_n2, assign55870_e86586_d_n4, assign55870_e86586_d_n5, assign55870_e86586_d_n6, assign55870_e86586_d_n7, assign55870_e86586_d_n8, assign55870_e86586_d_n9, assign55870_e86586_d_n10, assign55870_e86586_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1396 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign55870_e86586;
        locals.var_t0_dn0 = assign55870_e86586_d_n0;
        locals.var_t0_dn2 = assign55870_e86586_d_n2;
        locals.var_t0_dn4 = assign55870_e86586_d_n4;
        locals.var_t0_dn5 = assign55870_e86586_d_n5;
        locals.var_t0_dn6 = assign55870_e86586_d_n6;
        locals.var_t0_dn7 = assign55870_e86586_d_n7;
        locals.var_t0_dn8 = assign55870_e86586_d_n8;
        locals.var_t0_dn9 = assign55870_e86586_d_n9;
        locals.var_t0_dn10 = assign55870_e86586_d_n10;
        locals.var_t0_dn13 = assign55870_e86586_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign55880_e86600, assign55880_e86600_d_n0, assign55880_e86600_d_n2, assign55880_e86600_d_n4, assign55880_e86600_d_n5, assign55880_e86600_d_n6, assign55880_e86600_d_n7, assign55880_e86600_d_n8, assign55880_e86600_d_n9, assign55880_e86600_d_n10, assign55880_e86600_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) {
        let assign55880_e86598: f64 = (locals.var_t4).sqrt();
        (assign55880_e86598, (locals.var_t4_dn0 / (2.0 * assign55880_e86598)), (locals.var_t4_dn2 / (2.0 * assign55880_e86598)), (locals.var_t4_dn4 / (2.0 * assign55880_e86598)), (locals.var_t4_dn5 / (2.0 * assign55880_e86598)), (locals.var_t4_dn6 / (2.0 * assign55880_e86598)), (locals.var_t4_dn7 / (2.0 * assign55880_e86598)), (locals.var_t4_dn8 / (2.0 * assign55880_e86598)), (locals.var_t4_dn9 / (2.0 * assign55880_e86598)), (locals.var_t4_dn10 / (2.0 * assign55880_e86598)), (locals.var_t4_dn13 / (2.0 * assign55880_e86598)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign55880_e86600;
        locals.var_t3_dn0 = assign55880_e86600_d_n0;
        locals.var_t3_dn2 = assign55880_e86600_d_n2;
        locals.var_t3_dn4 = assign55880_e86600_d_n4;
        locals.var_t3_dn5 = assign55880_e86600_d_n5;
        locals.var_t3_dn6 = assign55880_e86600_d_n6;
        locals.var_t3_dn7 = assign55880_e86600_d_n7;
        locals.var_t3_dn8 = assign55880_e86600_d_n8;
        locals.var_t3_dn9 = assign55880_e86600_d_n9;
        locals.var_t3_dn10 = assign55880_e86600_d_n10;
        locals.var_t3_dn13 = assign55880_e86600_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign55890_e86619, assign55890_e86619_d_n0, assign55890_e86619_d_n2, assign55890_e86619_d_n4, assign55890_e86619_d_n5, assign55890_e86619_d_n6, assign55890_e86619_d_n7, assign55890_e86619_d_n8, assign55890_e86619_d_n9, assign55890_e86619_d_n10, assign55890_e86619_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) {
        let assign55890_e86615: f64 = (1.0 - locals.var_t3);
        let assign55890_e86616: f64 = (locals.var_q_ndepm_esi_cox_inv2__blk1134 * assign55890_e86615);
        let assign55890_e86617: f64 = (locals.var_vgp + assign55890_e86616);
        (assign55890_e86617, (locals.var_vgp_dn0 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn0 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn2 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn4 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn5 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn6 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn7 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn8 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn9 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn10 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn13 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn13 * assign55890_e86615) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn13)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign55890_e86619;
        locals.var_t10_dn0 = assign55890_e86619_d_n0;
        locals.var_t10_dn2 = assign55890_e86619_d_n2;
        locals.var_t10_dn4 = assign55890_e86619_d_n4;
        locals.var_t10_dn5 = assign55890_e86619_d_n5;
        locals.var_t10_dn6 = assign55890_e86619_d_n6;
        locals.var_t10_dn7 = assign55890_e86619_d_n7;
        locals.var_t10_dn8 = assign55890_e86619_d_n8;
        locals.var_t10_dn9 = assign55890_e86619_d_n9;
        locals.var_t10_dn10 = assign55890_e86619_d_n10;
        locals.var_t10_dn13 = assign55890_e86619_d_n13;
        locals.var_t10_rv = 0.0;

        let assign55900_e86623: f64 = (locals.var_uc_depleak + p.p405);
        let assign55900_e86628: f64 = if ((locals.var_t10 < assign55900_e86623) && (p.p405 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1402 = assign55900_e86628;
        locals.var_guard1402_rv = 0.0;

        let (assign55910_e86647, assign55910_e86647_d_n0, assign55910_e86647_d_n2, assign55910_e86647_d_n4, assign55910_e86647_d_n5, assign55910_e86647_d_n6, assign55910_e86647_d_n7, assign55910_e86647_d_n8, assign55910_e86647_d_n9, assign55910_e86647_d_n10, assign55910_e86647_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign55910_e86643: f64 = (locals.var_uc_depleak + p.p405);
        let assign55910_e86645: f64 = (assign55910_e86643 - locals.var_t10);
        (assign55910_e86645, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn13 - locals.var_t10_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign55910_e86647;
        locals.var_tmf1_dn0 = assign55910_e86647_d_n0;
        locals.var_tmf1_dn2 = assign55910_e86647_d_n2;
        locals.var_tmf1_dn4 = assign55910_e86647_d_n4;
        locals.var_tmf1_dn5 = assign55910_e86647_d_n5;
        locals.var_tmf1_dn6 = assign55910_e86647_d_n6;
        locals.var_tmf1_dn7 = assign55910_e86647_d_n7;
        locals.var_tmf1_dn8 = assign55910_e86647_d_n8;
        locals.var_tmf1_dn9 = assign55910_e86647_d_n9;
        locals.var_tmf1_dn10 = assign55910_e86647_d_n10;
        locals.var_tmf1_dn13 = assign55910_e86647_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign55920_e86664, assign55920_e86664_d_n0, assign55920_e86664_d_n2, assign55920_e86664_d_n4, assign55920_e86664_d_n5, assign55920_e86664_d_n6, assign55920_e86664_d_n7, assign55920_e86664_d_n8, assign55920_e86664_d_n9, assign55920_e86664_d_n10, assign55920_e86664_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign55920_e86662: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55920_e86662, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign55920_e86664;
        locals.var_x2_dn0 = assign55920_e86664_d_n0;
        locals.var_x2_dn2 = assign55920_e86664_d_n2;
        locals.var_x2_dn4 = assign55920_e86664_d_n4;
        locals.var_x2_dn5 = assign55920_e86664_d_n5;
        locals.var_x2_dn6 = assign55920_e86664_d_n6;
        locals.var_x2_dn7 = assign55920_e86664_d_n7;
        locals.var_x2_dn8 = assign55920_e86664_d_n8;
        locals.var_x2_dn9 = assign55920_e86664_d_n9;
        locals.var_x2_dn10 = assign55920_e86664_d_n10;
        locals.var_x2_dn13 = assign55920_e86664_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign55930_e86681, assign55930_e86681_d_n0, assign55930_e86681_d_n2, assign55930_e86681_d_n4, assign55930_e86681_d_n5, assign55930_e86681_d_n6, assign55930_e86681_d_n7, assign55930_e86681_d_n8, assign55930_e86681_d_n9, assign55930_e86681_d_n10, assign55930_e86681_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign55930_e86679: f64 = (p.p405 * p.p405);
        (assign55930_e86679, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign55930_e86681;
        locals.var_xmax2_dn0 = assign55930_e86681_d_n0;
        locals.var_xmax2_dn2 = assign55930_e86681_d_n2;
        locals.var_xmax2_dn4 = assign55930_e86681_d_n4;
        locals.var_xmax2_dn5 = assign55930_e86681_d_n5;
        locals.var_xmax2_dn6 = assign55930_e86681_d_n6;
        locals.var_xmax2_dn7 = assign55930_e86681_d_n7;
        locals.var_xmax2_dn8 = assign55930_e86681_d_n8;
        locals.var_xmax2_dn9 = assign55930_e86681_d_n9;
        locals.var_xmax2_dn10 = assign55930_e86681_d_n10;
        locals.var_xmax2_dn13 = assign55930_e86681_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign55940_e86696, assign55940_e86696_d_n0, assign55940_e86696_d_n2, assign55940_e86696_d_n4, assign55940_e86696_d_n5, assign55940_e86696_d_n6, assign55940_e86696_d_n7, assign55940_e86696_d_n8, assign55940_e86696_d_n9, assign55940_e86696_d_n10, assign55940_e86696_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign55940_e86696;
        locals.var_xp_dn0 = assign55940_e86696_d_n0;
        locals.var_xp_dn2 = assign55940_e86696_d_n2;
        locals.var_xp_dn4 = assign55940_e86696_d_n4;
        locals.var_xp_dn5 = assign55940_e86696_d_n5;
        locals.var_xp_dn6 = assign55940_e86696_d_n6;
        locals.var_xp_dn7 = assign55940_e86696_d_n7;
        locals.var_xp_dn8 = assign55940_e86696_d_n8;
        locals.var_xp_dn9 = assign55940_e86696_d_n9;
        locals.var_xp_dn10 = assign55940_e86696_d_n10;
        locals.var_xp_dn13 = assign55940_e86696_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign55950_e86711, assign55950_e86711_d_n0, assign55950_e86711_d_n2, assign55950_e86711_d_n4, assign55950_e86711_d_n5, assign55950_e86711_d_n6, assign55950_e86711_d_n7, assign55950_e86711_d_n8, assign55950_e86711_d_n9, assign55950_e86711_d_n10, assign55950_e86711_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign55950_e86711;
        locals.var_xmp_dn0 = assign55950_e86711_d_n0;
        locals.var_xmp_dn2 = assign55950_e86711_d_n2;
        locals.var_xmp_dn4 = assign55950_e86711_d_n4;
        locals.var_xmp_dn5 = assign55950_e86711_d_n5;
        locals.var_xmp_dn6 = assign55950_e86711_d_n6;
        locals.var_xmp_dn7 = assign55950_e86711_d_n7;
        locals.var_xmp_dn8 = assign55950_e86711_d_n8;
        locals.var_xmp_dn9 = assign55950_e86711_d_n9;
        locals.var_xmp_dn10 = assign55950_e86711_d_n10;
        locals.var_xmp_dn13 = assign55950_e86711_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign55960_e86726,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55960_e86726;
        locals.var_m0_rv = 0.0;

        let (assign55970_e86741,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55970_e86741;
        locals.var_mm_rv = 0.0;

        let (assign55980_e86756, assign55980_e86756_d_n0, assign55980_e86756_d_n2, assign55980_e86756_d_n4, assign55980_e86756_d_n5, assign55980_e86756_d_n6, assign55980_e86756_d_n7, assign55980_e86756_d_n8, assign55980_e86756_d_n9, assign55980_e86756_d_n10, assign55980_e86756_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign55980_e86756;
        locals.var_arg_dn0 = assign55980_e86756_d_n0;
        locals.var_arg_dn2 = assign55980_e86756_d_n2;
        locals.var_arg_dn4 = assign55980_e86756_d_n4;
        locals.var_arg_dn5 = assign55980_e86756_d_n5;
        locals.var_arg_dn6 = assign55980_e86756_d_n6;
        locals.var_arg_dn7 = assign55980_e86756_d_n7;
        locals.var_arg_dn8 = assign55980_e86756_d_n8;
        locals.var_arg_dn9 = assign55980_e86756_d_n9;
        locals.var_arg_dn10 = assign55980_e86756_d_n10;
        locals.var_arg_dn13 = assign55980_e86756_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign55990_e86771, assign55990_e86771_d_n0, assign55990_e86771_d_n2, assign55990_e86771_d_n4, assign55990_e86771_d_n5, assign55990_e86771_d_n6, assign55990_e86771_d_n7, assign55990_e86771_d_n8, assign55990_e86771_d_n9, assign55990_e86771_d_n10, assign55990_e86771_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign55990_e86771;
        locals.var_dnm_dn0 = assign55990_e86771_d_n0;
        locals.var_dnm_dn2 = assign55990_e86771_d_n2;
        locals.var_dnm_dn4 = assign55990_e86771_d_n4;
        locals.var_dnm_dn5 = assign55990_e86771_d_n5;
        locals.var_dnm_dn6 = assign55990_e86771_d_n6;
        locals.var_dnm_dn7 = assign55990_e86771_d_n7;
        locals.var_dnm_dn8 = assign55990_e86771_d_n8;
        locals.var_dnm_dn9 = assign55990_e86771_d_n9;
        locals.var_dnm_dn10 = assign55990_e86771_d_n10;
        locals.var_dnm_dn13 = assign55990_e86771_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign56000_e86788, assign56000_e86788_d_n0, assign56000_e86788_d_n2, assign56000_e86788_d_n4, assign56000_e86788_d_n5, assign56000_e86788_d_n6, assign56000_e86788_d_n7, assign56000_e86788_d_n8, assign56000_e86788_d_n9, assign56000_e86788_d_n10, assign56000_e86788_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56000_e86786: f64 = (locals.var_xp * locals.var_x2);
        (assign56000_e86786, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign56000_e86788;
        locals.var_xp_dn0 = assign56000_e86788_d_n0;
        locals.var_xp_dn2 = assign56000_e86788_d_n2;
        locals.var_xp_dn4 = assign56000_e86788_d_n4;
        locals.var_xp_dn5 = assign56000_e86788_d_n5;
        locals.var_xp_dn6 = assign56000_e86788_d_n6;
        locals.var_xp_dn7 = assign56000_e86788_d_n7;
        locals.var_xp_dn8 = assign56000_e86788_d_n8;
        locals.var_xp_dn9 = assign56000_e86788_d_n9;
        locals.var_xp_dn10 = assign56000_e86788_d_n10;
        locals.var_xp_dn13 = assign56000_e86788_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign56010_e86805, assign56010_e86805_d_n0, assign56010_e86805_d_n2, assign56010_e86805_d_n4, assign56010_e86805_d_n5, assign56010_e86805_d_n6, assign56010_e86805_d_n7, assign56010_e86805_d_n8, assign56010_e86805_d_n9, assign56010_e86805_d_n10, assign56010_e86805_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56010_e86803: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56010_e86803, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign56010_e86805;
        locals.var_xmp_dn0 = assign56010_e86805_d_n0;
        locals.var_xmp_dn2 = assign56010_e86805_d_n2;
        locals.var_xmp_dn4 = assign56010_e86805_d_n4;
        locals.var_xmp_dn5 = assign56010_e86805_d_n5;
        locals.var_xmp_dn6 = assign56010_e86805_d_n6;
        locals.var_xmp_dn7 = assign56010_e86805_d_n7;
        locals.var_xmp_dn8 = assign56010_e86805_d_n8;
        locals.var_xmp_dn9 = assign56010_e86805_d_n9;
        locals.var_xmp_dn10 = assign56010_e86805_d_n10;
        locals.var_xmp_dn13 = assign56010_e86805_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign56020_e86822, assign56020_e86822_d_n0, assign56020_e86822_d_n2, assign56020_e86822_d_n4, assign56020_e86822_d_n5, assign56020_e86822_d_n6, assign56020_e86822_d_n7, assign56020_e86822_d_n8, assign56020_e86822_d_n9, assign56020_e86822_d_n10, assign56020_e86822_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56020_e86820: f64 = (locals.var_xp * locals.var_x2);
        (assign56020_e86820, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign56020_e86822;
        locals.var_xp_dn0 = assign56020_e86822_d_n0;
        locals.var_xp_dn2 = assign56020_e86822_d_n2;
        locals.var_xp_dn4 = assign56020_e86822_d_n4;
        locals.var_xp_dn5 = assign56020_e86822_d_n5;
        locals.var_xp_dn6 = assign56020_e86822_d_n6;
        locals.var_xp_dn7 = assign56020_e86822_d_n7;
        locals.var_xp_dn8 = assign56020_e86822_d_n8;
        locals.var_xp_dn9 = assign56020_e86822_d_n9;
        locals.var_xp_dn10 = assign56020_e86822_d_n10;
        locals.var_xp_dn13 = assign56020_e86822_d_n13;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_196(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56030_e86839, assign56030_e86839_d_n0, assign56030_e86839_d_n2, assign56030_e86839_d_n4, assign56030_e86839_d_n5, assign56030_e86839_d_n6, assign56030_e86839_d_n7, assign56030_e86839_d_n8, assign56030_e86839_d_n9, assign56030_e86839_d_n10, assign56030_e86839_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56030_e86837: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56030_e86837, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign56030_e86839;
        locals.var_xmp_dn0 = assign56030_e86839_d_n0;
        locals.var_xmp_dn2 = assign56030_e86839_d_n2;
        locals.var_xmp_dn4 = assign56030_e86839_d_n4;
        locals.var_xmp_dn5 = assign56030_e86839_d_n5;
        locals.var_xmp_dn6 = assign56030_e86839_d_n6;
        locals.var_xmp_dn7 = assign56030_e86839_d_n7;
        locals.var_xmp_dn8 = assign56030_e86839_d_n8;
        locals.var_xmp_dn9 = assign56030_e86839_d_n9;
        locals.var_xmp_dn10 = assign56030_e86839_d_n10;
        locals.var_xmp_dn13 = assign56030_e86839_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign56040_e86856, assign56040_e86856_d_n0, assign56040_e86856_d_n2, assign56040_e86856_d_n4, assign56040_e86856_d_n5, assign56040_e86856_d_n6, assign56040_e86856_d_n7, assign56040_e86856_d_n8, assign56040_e86856_d_n9, assign56040_e86856_d_n10, assign56040_e86856_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56040_e86854: f64 = (locals.var_xp + locals.var_xmp);
        (assign56040_e86854, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign56040_e86856;
        locals.var_arg_dn0 = assign56040_e86856_d_n0;
        locals.var_arg_dn2 = assign56040_e86856_d_n2;
        locals.var_arg_dn4 = assign56040_e86856_d_n4;
        locals.var_arg_dn5 = assign56040_e86856_d_n5;
        locals.var_arg_dn6 = assign56040_e86856_d_n6;
        locals.var_arg_dn7 = assign56040_e86856_d_n7;
        locals.var_arg_dn8 = assign56040_e86856_d_n8;
        locals.var_arg_dn9 = assign56040_e86856_d_n9;
        locals.var_arg_dn10 = assign56040_e86856_d_n10;
        locals.var_arg_dn13 = assign56040_e86856_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign56050_e86871, assign56050_e86871_d_n0, assign56050_e86871_d_n2, assign56050_e86871_d_n4, assign56050_e86871_d_n5, assign56050_e86871_d_n6, assign56050_e86871_d_n7, assign56050_e86871_d_n8, assign56050_e86871_d_n9, assign56050_e86871_d_n10, assign56050_e86871_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign56050_e86871;
        locals.var_dnm_dn0 = assign56050_e86871_d_n0;
        locals.var_dnm_dn2 = assign56050_e86871_d_n2;
        locals.var_dnm_dn4 = assign56050_e86871_d_n4;
        locals.var_dnm_dn5 = assign56050_e86871_d_n5;
        locals.var_dnm_dn6 = assign56050_e86871_d_n6;
        locals.var_dnm_dn7 = assign56050_e86871_d_n7;
        locals.var_dnm_dn8 = assign56050_e86871_d_n8;
        locals.var_dnm_dn9 = assign56050_e86871_d_n9;
        locals.var_dnm_dn10 = assign56050_e86871_d_n10;
        locals.var_dnm_dn13 = assign56050_e86871_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign56060_e86886: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1403 = assign56060_e86886;
        locals.var_guard1403_rv = 0.0;

        let assign56070_e86889: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1404 = assign56070_e86889;
        locals.var_guard1404_rv = 0.0;

        let (assign56080_e86908,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 != 0.0)) && (locals.var_guard1404 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56080_e86908;
        locals.var_mm_rv = 0.0;

        let assign56090_e86911: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1405 = assign56090_e86911;
        locals.var_guard1405_rv = 0.0;

        let (assign56100_e86933,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 != 0.0)) && (locals.var_guard1404 == 0.0)) && (locals.var_guard1405 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56100_e86933;
        locals.var_mm_rv = 0.0;

        let assign56110_e86936: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1406 = assign56110_e86936;
        locals.var_guard1406_rv = 0.0;

        let (assign56120_e86961,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 != 0.0)) && (locals.var_guard1404 == 0.0)) && (locals.var_guard1405 == 0.0)) && (locals.var_guard1406 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56120_e86961;
        locals.var_mm_rv = 0.0;

        let assign56130_e86964: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1407 = assign56130_e86964;
        locals.var_guard1407_rv = 0.0;

        let (assign56140_e86992,) = {
    if (((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 != 0.0)) && (locals.var_guard1404 == 0.0)) && (locals.var_guard1405 == 0.0)) && (locals.var_guard1406 == 0.0)) && (locals.var_guard1407 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56140_e86992;
        locals.var_mm_rv = 0.0;

        let (assign56150_e87009,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56150_e87009;
        locals.var_m0_rv = 0.0;

        let mut assign56160_loop_guard: usize = 0;
        while {
            let assign56160_cond_e87027: f64 = if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign56160_cond_e87027 != 0.0
        } {
            assign56160_loop_guard += 1;
            assert!(assign56160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign56160_body0_e87045, assign56160_body0_e87045_d_n0, assign56160_body0_e87045_d_n2, assign56160_body0_e87045_d_n4, assign56160_body0_e87045_d_n5, assign56160_body0_e87045_d_n6, assign56160_body0_e87045_d_n7, assign56160_body0_e87045_d_n8, assign56160_body0_e87045_d_n9, assign56160_body0_e87045_d_n10, assign56160_body0_e87045_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 != 0.0)) {
        let assign56160_body0_e87043: f64 = (locals.var_dnm).sqrt();
        (assign56160_body0_e87043, (locals.var_dnm_dn0 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn2 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn4 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn5 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn6 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn7 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn8 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn9 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn10 / (2.0 * assign56160_body0_e87043)), (locals.var_dnm_dn13 / (2.0 * assign56160_body0_e87043)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign56160_body0_e87045;
            locals.var_dnm_dn0 = assign56160_body0_e87045_d_n0;
            locals.var_dnm_dn2 = assign56160_body0_e87045_d_n2;
            locals.var_dnm_dn4 = assign56160_body0_e87045_d_n4;
            locals.var_dnm_dn5 = assign56160_body0_e87045_d_n5;
            locals.var_dnm_dn6 = assign56160_body0_e87045_d_n6;
            locals.var_dnm_dn7 = assign56160_body0_e87045_d_n7;
            locals.var_dnm_dn8 = assign56160_body0_e87045_d_n8;
            locals.var_dnm_dn9 = assign56160_body0_e87045_d_n9;
            locals.var_dnm_dn10 = assign56160_body0_e87045_d_n10;
            locals.var_dnm_dn13 = assign56160_body0_e87045_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign56160_body1_e87064,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 != 0.0)) {
        let assign56160_body1_e87062: f64 = (locals.var_m0 + 1.0);
        (assign56160_body1_e87062,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign56160_body1_e87064;
            locals.var_m0_rv = 0.0;
        }

        let (assign56170_e87093, assign56170_e87093_d_n0, assign56170_e87093_d_n2, assign56170_e87093_d_n4, assign56170_e87093_d_n5, assign56170_e87093_d_n6, assign56170_e87093_d_n7, assign56170_e87093_d_n8, assign56170_e87093_d_n9, assign56170_e87093_d_n10, assign56170_e87093_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) && (locals.var_guard1403 == 0.0)) {
        let (assign56170_e87091, assign56170_e87091_d_n0, assign56170_e87091_d_n2, assign56170_e87091_d_n4, assign56170_e87091_d_n5, assign56170_e87091_d_n6, assign56170_e87091_d_n7, assign56170_e87091_d_n8, assign56170_e87091_d_n9, assign56170_e87091_d_n10, assign56170_e87091_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56170_e87088: f64 = (2.0 * 2.0);
                let assign56170_e87089: f64 = (1.0 / assign56170_e87088);
                let assign56170_e87090: f64 = (locals.var_dnm).powf(assign56170_e87089);
                (assign56170_e87090, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn0)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn2)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn4)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn5)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn6)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn7)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn8)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn9)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn10)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign56170_e87089) as f64).is_finite() && ((assign56170_e87089) as f64).fract() == 0.0 { if assign56170_e87089 == 0.0 { 0.0 } else { (assign56170_e87089 * ((locals.var_dnm).powf(assign56170_e87089 - 1.0) * locals.var_dnm_dn13)) } } else { (assign56170_e87090 * (assign56170_e87089 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign56170_e87091, assign56170_e87091_d_n0, assign56170_e87091_d_n2, assign56170_e87091_d_n4, assign56170_e87091_d_n5, assign56170_e87091_d_n6, assign56170_e87091_d_n7, assign56170_e87091_d_n8, assign56170_e87091_d_n9, assign56170_e87091_d_n10, assign56170_e87091_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign56170_e87093;
        locals.var_dnm_dn0 = assign56170_e87093_d_n0;
        locals.var_dnm_dn2 = assign56170_e87093_d_n2;
        locals.var_dnm_dn4 = assign56170_e87093_d_n4;
        locals.var_dnm_dn5 = assign56170_e87093_d_n5;
        locals.var_dnm_dn6 = assign56170_e87093_d_n6;
        locals.var_dnm_dn7 = assign56170_e87093_d_n7;
        locals.var_dnm_dn8 = assign56170_e87093_d_n8;
        locals.var_dnm_dn9 = assign56170_e87093_d_n9;
        locals.var_dnm_dn10 = assign56170_e87093_d_n10;
        locals.var_dnm_dn13 = assign56170_e87093_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign56180_e87110, assign56180_e87110_d_n0, assign56180_e87110_d_n2, assign56180_e87110_d_n4, assign56180_e87110_d_n5, assign56180_e87110_d_n6, assign56180_e87110_d_n7, assign56180_e87110_d_n8, assign56180_e87110_d_n9, assign56180_e87110_d_n10, assign56180_e87110_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56180_e87108: f64 = (1.0 / locals.var_dnm);
        (assign56180_e87108, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign56180_e87110;
        locals.var_dnm_dn0 = assign56180_e87110_d_n0;
        locals.var_dnm_dn2 = assign56180_e87110_d_n2;
        locals.var_dnm_dn4 = assign56180_e87110_d_n4;
        locals.var_dnm_dn5 = assign56180_e87110_d_n5;
        locals.var_dnm_dn6 = assign56180_e87110_d_n6;
        locals.var_dnm_dn7 = assign56180_e87110_d_n7;
        locals.var_dnm_dn8 = assign56180_e87110_d_n8;
        locals.var_dnm_dn9 = assign56180_e87110_d_n9;
        locals.var_dnm_dn10 = assign56180_e87110_d_n10;
        locals.var_dnm_dn13 = assign56180_e87110_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign56190_e87129, assign56190_e87129_d_n0, assign56190_e87129_d_n2, assign56190_e87129_d_n4, assign56190_e87129_d_n5, assign56190_e87129_d_n6, assign56190_e87129_d_n7, assign56190_e87129_d_n8, assign56190_e87129_d_n9, assign56190_e87129_d_n10, assign56190_e87129_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56190_e87125: f64 = (locals.var_tmf1 * p.p405);
        let assign56190_e87127: f64 = (assign56190_e87125 * locals.var_dnm);
        (assign56190_e87127, (((locals.var_tmf1_dn0 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * p.p405) * locals.var_dnm) + (assign56190_e87125 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign56190_e87129;
        locals.var_tmf0_dn0 = assign56190_e87129_d_n0;
        locals.var_tmf0_dn2 = assign56190_e87129_d_n2;
        locals.var_tmf0_dn4 = assign56190_e87129_d_n4;
        locals.var_tmf0_dn5 = assign56190_e87129_d_n5;
        locals.var_tmf0_dn6 = assign56190_e87129_d_n6;
        locals.var_tmf0_dn7 = assign56190_e87129_d_n7;
        locals.var_tmf0_dn8 = assign56190_e87129_d_n8;
        locals.var_tmf0_dn9 = assign56190_e87129_d_n9;
        locals.var_tmf0_dn10 = assign56190_e87129_d_n10;
        locals.var_tmf0_dn13 = assign56190_e87129_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign56200_e87150, assign56200_e87150_d_n0, assign56200_e87150_d_n2, assign56200_e87150_d_n4, assign56200_e87150_d_n5, assign56200_e87150_d_n6, assign56200_e87150_d_n7, assign56200_e87150_d_n8, assign56200_e87150_d_n9, assign56200_e87150_d_n10, assign56200_e87150_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56200_e87144: f64 = (p.p405 * locals.var_xmp);
        let assign56200_e87146: f64 = (assign56200_e87144 * locals.var_dnm);
        let assign56200_e87148: f64 = (assign56200_e87146 / locals.var_arg);
        (assign56200_e87148, ((((((p.p405 * locals.var_xmp_dn0) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn0)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn2) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn2)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn4) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn4)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn5) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn5)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn6) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn6)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn7) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn7)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn8) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn8)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn9) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn9)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn10) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn10)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((p.p405 * locals.var_xmp_dn13) * locals.var_dnm) + (assign56200_e87144 * locals.var_dnm_dn13)) * locals.var_arg) - (assign56200_e87146 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign56200_e87150;
        locals.var_t0_dn0 = assign56200_e87150_d_n0;
        locals.var_t0_dn2 = assign56200_e87150_d_n2;
        locals.var_t0_dn4 = assign56200_e87150_d_n4;
        locals.var_t0_dn5 = assign56200_e87150_d_n5;
        locals.var_t0_dn6 = assign56200_e87150_d_n6;
        locals.var_t0_dn7 = assign56200_e87150_d_n7;
        locals.var_t0_dn8 = assign56200_e87150_d_n8;
        locals.var_t0_dn9 = assign56200_e87150_d_n9;
        locals.var_t0_dn10 = assign56200_e87150_d_n10;
        locals.var_t0_dn13 = assign56200_e87150_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign56210_e87169, assign56210_e87169_d_n0, assign56210_e87169_d_n2, assign56210_e87169_d_n4, assign56210_e87169_d_n5, assign56210_e87169_d_n6, assign56210_e87169_d_n7, assign56210_e87169_d_n8, assign56210_e87169_d_n9, assign56210_e87169_d_n10, assign56210_e87169_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign56210_e87165: f64 = (locals.var_uc_depleak + p.p405);
        let assign56210_e87167: f64 = (assign56210_e87165 - locals.var_tmf0);
        (assign56210_e87167, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn13,)
    }
};
        locals.var_vdssat_res = assign56210_e87169;
        locals.var_vdssat_res_dn0 = assign56210_e87169_d_n0;
        locals.var_vdssat_res_dn2 = assign56210_e87169_d_n2;
        locals.var_vdssat_res_dn4 = assign56210_e87169_d_n4;
        locals.var_vdssat_res_dn5 = assign56210_e87169_d_n5;
        locals.var_vdssat_res_dn6 = assign56210_e87169_d_n6;
        locals.var_vdssat_res_dn7 = assign56210_e87169_d_n7;
        locals.var_vdssat_res_dn8 = assign56210_e87169_d_n8;
        locals.var_vdssat_res_dn9 = assign56210_e87169_d_n9;
        locals.var_vdssat_res_dn10 = assign56210_e87169_d_n10;
        locals.var_vdssat_res_dn13 = assign56210_e87169_d_n13;
        locals.var_vdssat_res_rv = 0.0;

        let (assign56220_e87184, assign56220_e87184_d_n0, assign56220_e87184_d_n2, assign56220_e87184_d_n4, assign56220_e87184_d_n5, assign56220_e87184_d_n6, assign56220_e87184_d_n7, assign56220_e87184_d_n8, assign56220_e87184_d_n9, assign56220_e87184_d_n10, assign56220_e87184_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign56220_e87184;
        locals.var_t0_dn0 = assign56220_e87184_d_n0;
        locals.var_t0_dn2 = assign56220_e87184_d_n2;
        locals.var_t0_dn4 = assign56220_e87184_d_n4;
        locals.var_t0_dn5 = assign56220_e87184_d_n5;
        locals.var_t0_dn6 = assign56220_e87184_d_n6;
        locals.var_t0_dn7 = assign56220_e87184_d_n7;
        locals.var_t0_dn8 = assign56220_e87184_d_n8;
        locals.var_t0_dn9 = assign56220_e87184_d_n9;
        locals.var_t0_dn10 = assign56220_e87184_d_n10;
        locals.var_t0_dn13 = assign56220_e87184_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign56230_e87200, assign56230_e87200_d_n0, assign56230_e87200_d_n2, assign56230_e87200_d_n4, assign56230_e87200_d_n5, assign56230_e87200_d_n6, assign56230_e87200_d_n7, assign56230_e87200_d_n8, assign56230_e87200_d_n9, assign56230_e87200_d_n10, assign56230_e87200_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn13,)
    }
};
        locals.var_vdssat_res = assign56230_e87200;
        locals.var_vdssat_res_dn0 = assign56230_e87200_d_n0;
        locals.var_vdssat_res_dn2 = assign56230_e87200_d_n2;
        locals.var_vdssat_res_dn4 = assign56230_e87200_d_n4;
        locals.var_vdssat_res_dn5 = assign56230_e87200_d_n5;
        locals.var_vdssat_res_dn6 = assign56230_e87200_d_n6;
        locals.var_vdssat_res_dn7 = assign56230_e87200_d_n7;
        locals.var_vdssat_res_dn8 = assign56230_e87200_d_n8;
        locals.var_vdssat_res_dn9 = assign56230_e87200_d_n9;
        locals.var_vdssat_res_dn10 = assign56230_e87200_d_n10;
        locals.var_vdssat_res_dn13 = assign56230_e87200_d_n13;
        locals.var_vdssat_res_rv = 0.0;

        let (assign56240_e87216, assign56240_e87216_d_n0, assign56240_e87216_d_n2, assign56240_e87216_d_n4, assign56240_e87216_d_n5, assign56240_e87216_d_n6, assign56240_e87216_d_n7, assign56240_e87216_d_n8, assign56240_e87216_d_n9, assign56240_e87216_d_n10, assign56240_e87216_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 != 0.0)) && (locals.var_guard1402 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign56240_e87216;
        locals.var_t0_dn0 = assign56240_e87216_d_n0;
        locals.var_t0_dn2 = assign56240_e87216_d_n2;
        locals.var_t0_dn4 = assign56240_e87216_d_n4;
        locals.var_t0_dn5 = assign56240_e87216_d_n5;
        locals.var_t0_dn6 = assign56240_e87216_d_n6;
        locals.var_t0_dn7 = assign56240_e87216_d_n7;
        locals.var_t0_dn8 = assign56240_e87216_d_n8;
        locals.var_t0_dn9 = assign56240_e87216_d_n9;
        locals.var_t0_dn10 = assign56240_e87216_d_n10;
        locals.var_t0_dn13 = assign56240_e87216_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign56250_e87230, assign56250_e87230_d_n0, assign56250_e87230_d_n2, assign56250_e87230_d_n4, assign56250_e87230_d_n5, assign56250_e87230_d_n6, assign56250_e87230_d_n7, assign56250_e87230_d_n8, assign56250_e87230_d_n9, assign56250_e87230_d_n10, assign56250_e87230_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        (locals.var_vgp_res__blk1145, locals.var_vgp_res__blk1145_dn0, locals.var_vgp_res__blk1145_dn2, locals.var_vgp_res__blk1145_dn4, locals.var_vgp_res__blk1145_dn5, locals.var_vgp_res__blk1145_dn6, locals.var_vgp_res__blk1145_dn7, locals.var_vgp_res__blk1145_dn8, locals.var_vgp_res__blk1145_dn9, locals.var_vgp_res__blk1145_dn10, locals.var_vgp_res__blk1145_dn13,)
    } else {
        (locals.var_vgpsat, locals.var_vgpsat_dn0, locals.var_vgpsat_dn2, locals.var_vgpsat_dn4, locals.var_vgpsat_dn5, locals.var_vgpsat_dn6, locals.var_vgpsat_dn7, locals.var_vgpsat_dn8, locals.var_vgpsat_dn9, locals.var_vgpsat_dn10, locals.var_vgpsat_dn13,)
    }
};
        locals.var_vgpsat = assign56250_e87230;
        locals.var_vgpsat_dn0 = assign56250_e87230_d_n0;
        locals.var_vgpsat_dn2 = assign56250_e87230_d_n2;
        locals.var_vgpsat_dn4 = assign56250_e87230_d_n4;
        locals.var_vgpsat_dn5 = assign56250_e87230_d_n5;
        locals.var_vgpsat_dn6 = assign56250_e87230_d_n6;
        locals.var_vgpsat_dn7 = assign56250_e87230_d_n7;
        locals.var_vgpsat_dn8 = assign56250_e87230_d_n8;
        locals.var_vgpsat_dn9 = assign56250_e87230_d_n9;
        locals.var_vgpsat_dn10 = assign56250_e87230_d_n10;
        locals.var_vgpsat_dn13 = assign56250_e87230_d_n13;
        locals.var_vgpsat_rv = 0.0;

        let (assign56260_e87248, assign56260_e87248_d_n0, assign56260_e87248_d_n2, assign56260_e87248_d_n4, assign56260_e87248_d_n5, assign56260_e87248_d_n6, assign56260_e87248_d_n7, assign56260_e87248_d_n8, assign56260_e87248_d_n9, assign56260_e87248_d_n10, assign56260_e87248_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56260_e87245: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat);
        let assign56260_e87246: f64 = (1.0 + assign56260_e87245);
        (assign56260_e87246, ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn0 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn0)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn2 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn2)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn4 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn4)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn5 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn5)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn6 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn6)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn7 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn7)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn8 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn8)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn9 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn9)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn10 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn10)), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1135_dn13 * locals.var_vgpsat) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1135 * locals.var_vgpsat_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign56260_e87248;
        locals.var_t4_dn0 = assign56260_e87248_d_n0;
        locals.var_t4_dn2 = assign56260_e87248_d_n2;
        locals.var_t4_dn4 = assign56260_e87248_d_n4;
        locals.var_t4_dn5 = assign56260_e87248_d_n5;
        locals.var_t4_dn6 = assign56260_e87248_d_n6;
        locals.var_t4_dn7 = assign56260_e87248_d_n7;
        locals.var_t4_dn8 = assign56260_e87248_d_n8;
        locals.var_t4_dn9 = assign56260_e87248_d_n9;
        locals.var_t4_dn10 = assign56260_e87248_d_n10;
        locals.var_t4_dn13 = assign56260_e87248_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign56270_e87271, assign56270_e87271_d_n0, assign56270_e87271_d_n2, assign56270_e87271_d_n4, assign56270_e87271_d_n5, assign56270_e87271_d_n6, assign56270_e87271_d_n7, assign56270_e87271_d_n8, assign56270_e87271_d_n9, assign56270_e87271_d_n10, assign56270_e87271_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let (assign56270_e87269, assign56270_e87269_d_n0, assign56270_e87269_d_n2, assign56270_e87269_d_n4, assign56270_e87269_d_n5, assign56270_e87269_d_n6, assign56270_e87269_d_n7, assign56270_e87269_d_n8, assign56270_e87269_d_n9, assign56270_e87269_d_n10, assign56270_e87269_d_n13,) = {
            if (locals.var_t4 > 0.0) {
                let assign56270_e87264: f64 = (locals.var_t4).sqrt();
                (assign56270_e87264, (locals.var_t4_dn0 / (2.0 * assign56270_e87264)), (locals.var_t4_dn2 / (2.0 * assign56270_e87264)), (locals.var_t4_dn4 / (2.0 * assign56270_e87264)), (locals.var_t4_dn5 / (2.0 * assign56270_e87264)), (locals.var_t4_dn6 / (2.0 * assign56270_e87264)), (locals.var_t4_dn7 / (2.0 * assign56270_e87264)), (locals.var_t4_dn8 / (2.0 * assign56270_e87264)), (locals.var_t4_dn9 / (2.0 * assign56270_e87264)), (locals.var_t4_dn10 / (2.0 * assign56270_e87264)), (locals.var_t4_dn13 / (2.0 * assign56270_e87264)),)
            } else {
                let assign56270_e87266: f64 = (-locals.var_t4);
                let assign56270_e87267: f64 = (assign56270_e87266).sqrt();
                let assign56270_e87268: f64 = (-assign56270_e87267);
                (assign56270_e87268, (-((-locals.var_t4_dn0) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn2) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn4) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn5) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn6) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn7) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn8) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn9) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn10) / (2.0 * assign56270_e87267))), (-((-locals.var_t4_dn13) / (2.0 * assign56270_e87267))),)
            }
        };
        (assign56270_e87269, assign56270_e87269_d_n0, assign56270_e87269_d_n2, assign56270_e87269_d_n4, assign56270_e87269_d_n5, assign56270_e87269_d_n6, assign56270_e87269_d_n7, assign56270_e87269_d_n8, assign56270_e87269_d_n9, assign56270_e87269_d_n10, assign56270_e87269_d_n13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign56270_e87271;
        locals.var_t3_dn0 = assign56270_e87271_d_n0;
        locals.var_t3_dn2 = assign56270_e87271_d_n2;
        locals.var_t3_dn4 = assign56270_e87271_d_n4;
        locals.var_t3_dn5 = assign56270_e87271_d_n5;
        locals.var_t3_dn6 = assign56270_e87271_d_n6;
        locals.var_t3_dn7 = assign56270_e87271_d_n7;
        locals.var_t3_dn8 = assign56270_e87271_d_n8;
        locals.var_t3_dn9 = assign56270_e87271_d_n9;
        locals.var_t3_dn10 = assign56270_e87271_d_n10;
        locals.var_t3_dn13 = assign56270_e87271_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign56280_e87291, assign56280_e87291_d_n0, assign56280_e87291_d_n2, assign56280_e87291_d_n4, assign56280_e87291_d_n5, assign56280_e87291_d_n6, assign56280_e87291_d_n7, assign56280_e87291_d_n8, assign56280_e87291_d_n9, assign56280_e87291_d_n10, assign56280_e87291_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56280_e87287: f64 = (1.0 - locals.var_t3);
        let assign56280_e87288: f64 = (locals.var_q_ndepm_esi_cox_inv2__blk1134 * assign56280_e87287);
        let assign56280_e87289: f64 = (locals.var_vgpsat + assign56280_e87288);
        (assign56280_e87289, (locals.var_vgpsat_dn0 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn0 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn0)))), (locals.var_vgpsat_dn2 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn2 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn2)))), (locals.var_vgpsat_dn4 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn4 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn4)))), (locals.var_vgpsat_dn5 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn5 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn5)))), (locals.var_vgpsat_dn6 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn6 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn6)))), (locals.var_vgpsat_dn7 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn7 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn7)))), (locals.var_vgpsat_dn8 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn8 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn8)))), (locals.var_vgpsat_dn9 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn9 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn9)))), (locals.var_vgpsat_dn10 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn10 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn10)))), (locals.var_vgpsat_dn13 + ((locals.var_q_ndepm_esi_cox_inv2__blk1134_dn13 * assign56280_e87287) + (locals.var_q_ndepm_esi_cox_inv2__blk1134 * (-locals.var_t3_dn13)))),)
    } else {
        (locals.var_vdssat_ini, locals.var_vdssat_ini_dn0, locals.var_vdssat_ini_dn2, locals.var_vdssat_ini_dn4, locals.var_vdssat_ini_dn5, locals.var_vdssat_ini_dn6, locals.var_vdssat_ini_dn7, locals.var_vdssat_ini_dn8, locals.var_vdssat_ini_dn9, locals.var_vdssat_ini_dn10, locals.var_vdssat_ini_dn13,)
    }
};
        locals.var_vdssat_ini = assign56280_e87291;
        locals.var_vdssat_ini_dn0 = assign56280_e87291_d_n0;
        locals.var_vdssat_ini_dn2 = assign56280_e87291_d_n2;
        locals.var_vdssat_ini_dn4 = assign56280_e87291_d_n4;
        locals.var_vdssat_ini_dn5 = assign56280_e87291_d_n5;
        locals.var_vdssat_ini_dn6 = assign56280_e87291_d_n6;
        locals.var_vdssat_ini_dn7 = assign56280_e87291_d_n7;
        locals.var_vdssat_ini_dn8 = assign56280_e87291_d_n8;
        locals.var_vdssat_ini_dn9 = assign56280_e87291_d_n9;
        locals.var_vdssat_ini_dn10 = assign56280_e87291_d_n10;
        locals.var_vdssat_ini_dn13 = assign56280_e87291_d_n13;
        locals.var_vdssat_ini_rv = 0.0;

        let (assign56290_e87305, assign56290_e87305_d_n0, assign56290_e87305_d_n2, assign56290_e87305_d_n4, assign56290_e87305_d_n5, assign56290_e87305_d_n6, assign56290_e87305_d_n7, assign56290_e87305_d_n8, assign56290_e87305_d_n9, assign56290_e87305_d_n10, assign56290_e87305_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        (locals.var_vdssat_ini, locals.var_vdssat_ini_dn0, locals.var_vdssat_ini_dn2, locals.var_vdssat_ini_dn4, locals.var_vdssat_ini_dn5, locals.var_vdssat_ini_dn6, locals.var_vdssat_ini_dn7, locals.var_vdssat_ini_dn8, locals.var_vdssat_ini_dn9, locals.var_vdssat_ini_dn10, locals.var_vdssat_ini_dn13,)
    } else {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn13,)
    }
};
        locals.var_phi_vsat = assign56290_e87305;
        locals.var_phi_vsat_dn0 = assign56290_e87305_d_n0;
        locals.var_phi_vsat_dn2 = assign56290_e87305_d_n2;
        locals.var_phi_vsat_dn4 = assign56290_e87305_d_n4;
        locals.var_phi_vsat_dn5 = assign56290_e87305_d_n5;
        locals.var_phi_vsat_dn6 = assign56290_e87305_d_n6;
        locals.var_phi_vsat_dn7 = assign56290_e87305_d_n7;
        locals.var_phi_vsat_dn8 = assign56290_e87305_d_n8;
        locals.var_phi_vsat_dn9 = assign56290_e87305_d_n9;
        locals.var_phi_vsat_dn10 = assign56290_e87305_d_n10;
        locals.var_phi_vsat_dn13 = assign56290_e87305_d_n13;
        locals.var_phi_vsat_rv = 0.0;

        let (assign56300_e87319,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign56300_e87319;
        locals.var_flg_conv_rv = 0.0;

        let (assign56310_e87333,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign56310_e87333;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_197(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign56320_loop_guard: usize = 0;
        while {
            let assign56320_cond_e87348: f64 = if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign56320_cond_e87348 != 0.0
        } {
            assign56320_loop_guard += 1;
            assert!(assign56320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign56320_body0_e87365, assign56320_body0_e87365_d_n0, assign56320_body0_e87365_d_n2, assign56320_body0_e87365_d_n4, assign56320_body0_e87365_d_n5, assign56320_body0_e87365_d_n6, assign56320_body0_e87365_d_n7, assign56320_body0_e87365_d_n8, assign56320_body0_e87365_d_n9, assign56320_body0_e87365_d_n10, assign56320_body0_e87365_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56320_body0_e87361: f64 = (-locals.var_beta);
        let assign56320_body0_e87363: f64 = (assign56320_body0_e87361 * locals.var_phi_vsat);
        (assign56320_body0_e87363, (((-locals.var_beta_dn0) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn0)), (((-locals.var_beta_dn2) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn2)), (((-locals.var_beta_dn4) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn4)), (((-locals.var_beta_dn5) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn5)), (((-locals.var_beta_dn6) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn6)), (((-locals.var_beta_dn7) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn7)), (((-locals.var_beta_dn8) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn8)), (((-locals.var_beta_dn9) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn9)), (((-locals.var_beta_dn10) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn10)), (((-locals.var_beta_dn13) * locals.var_phi_vsat) + (assign56320_body0_e87361 * locals.var_phi_vsat_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign56320_body0_e87365;
            locals.var_t1_dn0 = assign56320_body0_e87365_d_n0;
            locals.var_t1_dn2 = assign56320_body0_e87365_d_n2;
            locals.var_t1_dn4 = assign56320_body0_e87365_d_n4;
            locals.var_t1_dn5 = assign56320_body0_e87365_d_n5;
            locals.var_t1_dn6 = assign56320_body0_e87365_d_n6;
            locals.var_t1_dn7 = assign56320_body0_e87365_d_n7;
            locals.var_t1_dn8 = assign56320_body0_e87365_d_n8;
            locals.var_t1_dn9 = assign56320_body0_e87365_d_n9;
            locals.var_t1_dn10 = assign56320_body0_e87365_d_n10;
            locals.var_t1_dn13 = assign56320_body0_e87365_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign56320_body1_e87380, assign56320_body1_e87380_d_n0, assign56320_body1_e87380_d_n2, assign56320_body1_e87380_d_n4, assign56320_body1_e87380_d_n5, assign56320_body1_e87380_d_n6, assign56320_body1_e87380_d_n7, assign56320_body1_e87380_d_n8, assign56320_body1_e87380_d_n9, assign56320_body1_e87380_d_n10, assign56320_body1_e87380_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56320_body1_e87378: f64 = (locals.var_t1).exp();
        (assign56320_body1_e87378, (assign56320_body1_e87378 * locals.var_t1_dn0), (assign56320_body1_e87378 * locals.var_t1_dn2), (assign56320_body1_e87378 * locals.var_t1_dn4), (assign56320_body1_e87378 * locals.var_t1_dn5), (assign56320_body1_e87378 * locals.var_t1_dn6), (assign56320_body1_e87378 * locals.var_t1_dn7), (assign56320_body1_e87378 * locals.var_t1_dn8), (assign56320_body1_e87378 * locals.var_t1_dn9), (assign56320_body1_e87378 * locals.var_t1_dn10), (assign56320_body1_e87378 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign56320_body1_e87380;
            locals.var_t2_dn0 = assign56320_body1_e87380_d_n0;
            locals.var_t2_dn2 = assign56320_body1_e87380_d_n2;
            locals.var_t2_dn4 = assign56320_body1_e87380_d_n4;
            locals.var_t2_dn5 = assign56320_body1_e87380_d_n5;
            locals.var_t2_dn6 = assign56320_body1_e87380_d_n6;
            locals.var_t2_dn7 = assign56320_body1_e87380_d_n7;
            locals.var_t2_dn8 = assign56320_body1_e87380_d_n8;
            locals.var_t2_dn9 = assign56320_body1_e87380_d_n9;
            locals.var_t2_dn10 = assign56320_body1_e87380_d_n10;
            locals.var_t2_dn13 = assign56320_body1_e87380_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign56320_body2_e87399, assign56320_body2_e87399_d_n0, assign56320_body2_e87399_d_n2, assign56320_body2_e87399_d_n4, assign56320_body2_e87399_d_n5, assign56320_body2_e87399_d_n6, assign56320_body2_e87399_d_n7, assign56320_body2_e87399_d_n8, assign56320_body2_e87399_d_n9, assign56320_body2_e87399_d_n10, assign56320_body2_e87399_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56320_body2_e87394: f64 = (2.0 * locals.var_q_ndepm_esi__blk1114);
        let assign56320_body2_e87396: f64 = (assign56320_body2_e87394 / locals.var_beta);
        let assign56320_body2_e87397: f64 = (assign56320_body2_e87396).sqrt();
        (assign56320_body2_e87397, (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn0) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn0)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn2) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn2)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn4) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn5) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn5)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn6) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn6)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn7) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn7)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn8) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn8)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn9) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn9)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn10) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)), (((((2.0 * locals.var_q_ndepm_esi__blk1114_dn13) * locals.var_beta) - (assign56320_body2_e87394 * locals.var_beta_dn13)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign56320_body2_e87397)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign56320_body2_e87399;
            locals.var_t4_dn0 = assign56320_body2_e87399_d_n0;
            locals.var_t4_dn2 = assign56320_body2_e87399_d_n2;
            locals.var_t4_dn4 = assign56320_body2_e87399_d_n4;
            locals.var_t4_dn5 = assign56320_body2_e87399_d_n5;
            locals.var_t4_dn6 = assign56320_body2_e87399_d_n6;
            locals.var_t4_dn7 = assign56320_body2_e87399_d_n7;
            locals.var_t4_dn8 = assign56320_body2_e87399_d_n8;
            locals.var_t4_dn9 = assign56320_body2_e87399_d_n9;
            locals.var_t4_dn10 = assign56320_body2_e87399_d_n10;
            locals.var_t4_dn13 = assign56320_body2_e87399_d_n13;
            locals.var_t4_rv = 0.0;
            let (assign56320_body3_e87417, assign56320_body3_e87417_d_n0, assign56320_body3_e87417_d_n2, assign56320_body3_e87417_d_n4, assign56320_body3_e87417_d_n5, assign56320_body3_e87417_d_n6, assign56320_body3_e87417_d_n7, assign56320_body3_e87417_d_n8, assign56320_body3_e87417_d_n9, assign56320_body3_e87417_d_n10, assign56320_body3_e87417_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56320_body3_e87413: f64 = (locals.var_t2 - locals.var_t1);
        let assign56320_body3_e87415: f64 = (assign56320_body3_e87413 - 1.0);
        (assign56320_body3_e87415, (locals.var_t2_dn0 - locals.var_t1_dn0), (locals.var_t2_dn2 - locals.var_t1_dn2), (locals.var_t2_dn4 - locals.var_t1_dn4), (locals.var_t2_dn5 - locals.var_t1_dn5), (locals.var_t2_dn6 - locals.var_t1_dn6), (locals.var_t2_dn7 - locals.var_t1_dn7), (locals.var_t2_dn8 - locals.var_t1_dn8), (locals.var_t2_dn9 - locals.var_t1_dn9), (locals.var_t2_dn10 - locals.var_t1_dn10), (locals.var_t2_dn13 - locals.var_t1_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
            locals.var_t10 = assign56320_body3_e87417;
            locals.var_t10_dn0 = assign56320_body3_e87417_d_n0;
            locals.var_t10_dn2 = assign56320_body3_e87417_d_n2;
            locals.var_t10_dn4 = assign56320_body3_e87417_d_n4;
            locals.var_t10_dn5 = assign56320_body3_e87417_d_n5;
            locals.var_t10_dn6 = assign56320_body3_e87417_d_n6;
            locals.var_t10_dn7 = assign56320_body3_e87417_d_n7;
            locals.var_t10_dn8 = assign56320_body3_e87417_d_n8;
            locals.var_t10_dn9 = assign56320_body3_e87417_d_n9;
            locals.var_t10_dn10 = assign56320_body3_e87417_d_n10;
            locals.var_t10_dn13 = assign56320_body3_e87417_d_n13;
            locals.var_t10_rv = 0.0;
            let (assign56320_body4_e87436, assign56320_body4_e87436_d_n0, assign56320_body4_e87436_d_n2, assign56320_body4_e87436_d_n4, assign56320_body4_e87436_d_n5, assign56320_body4_e87436_d_n6, assign56320_body4_e87436_d_n7, assign56320_body4_e87436_d_n8, assign56320_body4_e87436_d_n9, assign56320_body4_e87436_d_n10, assign56320_body4_e87436_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56320_body4_e87432: f64 = (locals.var_t10 + 1e-15);
        let assign56320_body4_e87433: f64 = (assign56320_body4_e87432).sqrt();
        let assign56320_body4_e87434: f64 = (locals.var_t4 * assign56320_body4_e87433);
        (assign56320_body4_e87434, ((locals.var_t4_dn0 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn0 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn2 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn2 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn4 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn4 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn5 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn5 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn6 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn6 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn7 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn7 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn8 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn8 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn9 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn9 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn10 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn10 / (2.0 * assign56320_body4_e87433)))), ((locals.var_t4_dn13 * assign56320_body4_e87433) + (locals.var_t4 * (locals.var_t10_dn13 / (2.0 * assign56320_body4_e87433)))),)
    } else {
        (locals.var_q_sat, locals.var_q_sat_dn0, locals.var_q_sat_dn2, locals.var_q_sat_dn4, locals.var_q_sat_dn5, locals.var_q_sat_dn6, locals.var_q_sat_dn7, locals.var_q_sat_dn8, locals.var_q_sat_dn9, locals.var_q_sat_dn10, locals.var_q_sat_dn13,)
    }
};
            locals.var_q_sat = assign56320_body4_e87436;
            locals.var_q_sat_dn0 = assign56320_body4_e87436_d_n0;
            locals.var_q_sat_dn2 = assign56320_body4_e87436_d_n2;
            locals.var_q_sat_dn4 = assign56320_body4_e87436_d_n4;
            locals.var_q_sat_dn5 = assign56320_body4_e87436_d_n5;
            locals.var_q_sat_dn6 = assign56320_body4_e87436_d_n6;
            locals.var_q_sat_dn7 = assign56320_body4_e87436_d_n7;
            locals.var_q_sat_dn8 = assign56320_body4_e87436_d_n8;
            locals.var_q_sat_dn9 = assign56320_body4_e87436_d_n9;
            locals.var_q_sat_dn10 = assign56320_body4_e87436_d_n10;
            locals.var_q_sat_dn13 = assign56320_body4_e87436_d_n13;
            locals.var_q_sat_rv = 0.0;
            let assign56320_body5_e87439: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1408 = assign56320_body5_e87439;
            locals.var_guard1408_rv = 0.0;
            let (assign56320_body6_e87456, assign56320_body6_e87456_d_n0, assign56320_body6_e87456_d_n2, assign56320_body6_e87456_d_n4, assign56320_body6_e87456_d_n5, assign56320_body6_e87456_d_n6, assign56320_body6_e87456_d_n7, assign56320_body6_e87456_d_n8, assign56320_body6_e87456_d_n9, assign56320_body6_e87456_d_n10, assign56320_body6_e87456_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_guard1408 != 0.0)) {
        let assign56320_body6_e87454: f64 = (-locals.var_q_sat);
        (assign56320_body6_e87454, (-locals.var_q_sat_dn0), (-locals.var_q_sat_dn2), (-locals.var_q_sat_dn4), (-locals.var_q_sat_dn5), (-locals.var_q_sat_dn6), (-locals.var_q_sat_dn7), (-locals.var_q_sat_dn8), (-locals.var_q_sat_dn9), (-locals.var_q_sat_dn10), (-locals.var_q_sat_dn13),)
    } else {
        (locals.var_q_sat, locals.var_q_sat_dn0, locals.var_q_sat_dn2, locals.var_q_sat_dn4, locals.var_q_sat_dn5, locals.var_q_sat_dn6, locals.var_q_sat_dn7, locals.var_q_sat_dn8, locals.var_q_sat_dn9, locals.var_q_sat_dn10, locals.var_q_sat_dn13,)
    }
};
            locals.var_q_sat = assign56320_body6_e87456;
            locals.var_q_sat_dn0 = assign56320_body6_e87456_d_n0;
            locals.var_q_sat_dn2 = assign56320_body6_e87456_d_n2;
            locals.var_q_sat_dn4 = assign56320_body6_e87456_d_n4;
            locals.var_q_sat_dn5 = assign56320_body6_e87456_d_n5;
            locals.var_q_sat_dn6 = assign56320_body6_e87456_d_n6;
            locals.var_q_sat_dn7 = assign56320_body6_e87456_d_n7;
            locals.var_q_sat_dn8 = assign56320_body6_e87456_d_n8;
            locals.var_q_sat_dn9 = assign56320_body6_e87456_d_n9;
            locals.var_q_sat_dn10 = assign56320_body6_e87456_d_n10;
            locals.var_q_sat_dn13 = assign56320_body6_e87456_d_n13;
            locals.var_q_sat_rv = 0.0;
            let (assign56320_body7_e87478, assign56320_body7_e87478_d_n0, assign56320_body7_e87478_d_n2, assign56320_body7_e87478_d_n4, assign56320_body7_e87478_d_n5, assign56320_body7_e87478_d_n6, assign56320_body7_e87478_d_n7, assign56320_body7_e87478_d_n8, assign56320_body7_e87478_d_n9, assign56320_body7_e87478_d_n10, assign56320_body7_e87478_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56320_body7_e87470: f64 = (0.5 * locals.var_t4);
        let assign56320_body7_e87472: f64 = (assign56320_body7_e87470 * locals.var_t4);
        let assign56320_body7_e87474: f64 = (assign56320_body7_e87472 * locals.var_beta);
        let assign56320_body7_e87476: f64 = (assign56320_body7_e87474 / locals.var_q_sat);
        (assign56320_body7_e87476, ((((((((0.5 * locals.var_t4_dn0) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn0)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn0)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn0)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn2) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn2)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn2)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn2)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn4) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn4)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn4)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn4)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn5) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn5)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn5)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn5)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn6) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn6)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn6)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn6)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn7) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn7)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn7)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn7)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn8) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn8)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn8)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn8)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn9) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn9)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn9)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn9)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn10) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn10)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn10)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn10)) / (locals.var_q_sat * locals.var_q_sat)), ((((((((0.5 * locals.var_t4_dn13) * locals.var_t4) + (assign56320_body7_e87470 * locals.var_t4_dn13)) * locals.var_beta) + (assign56320_body7_e87472 * locals.var_beta_dn13)) * locals.var_q_sat) - (assign56320_body7_e87474 * locals.var_q_sat_dn13)) / (locals.var_q_sat * locals.var_q_sat)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
            locals.var_t11 = assign56320_body7_e87478;
            locals.var_t11_dn0 = assign56320_body7_e87478_d_n0;
            locals.var_t11_dn2 = assign56320_body7_e87478_d_n2;
            locals.var_t11_dn4 = assign56320_body7_e87478_d_n4;
            locals.var_t11_dn5 = assign56320_body7_e87478_d_n5;
            locals.var_t11_dn6 = assign56320_body7_e87478_d_n6;
            locals.var_t11_dn7 = assign56320_body7_e87478_d_n7;
            locals.var_t11_dn8 = assign56320_body7_e87478_d_n8;
            locals.var_t11_dn9 = assign56320_body7_e87478_d_n9;
            locals.var_t11_dn10 = assign56320_body7_e87478_d_n10;
            locals.var_t11_dn13 = assign56320_body7_e87478_d_n13;
            locals.var_t11_rv = 0.0;
            let (assign56320_body8_e87497, assign56320_body8_e87497_d_n0, assign56320_body8_e87497_d_n2, assign56320_body8_e87497_d_n4, assign56320_body8_e87497_d_n5, assign56320_body8_e87497_d_n6, assign56320_body8_e87497_d_n7, assign56320_body8_e87497_d_n8, assign56320_body8_e87497_d_n9, assign56320_body8_e87497_d_n10, assign56320_body8_e87497_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56320_body8_e87492: f64 = (-locals.var_t2);
        let assign56320_body8_e87494: f64 = (assign56320_body8_e87492 + 1.0);
        let assign56320_body8_e87495: f64 = (locals.var_t11 * assign56320_body8_e87494);
        (assign56320_body8_e87495, ((locals.var_t11_dn0 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn0))), ((locals.var_t11_dn2 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn2))), ((locals.var_t11_dn4 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn4))), ((locals.var_t11_dn5 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn5))), ((locals.var_t11_dn6 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn6))), ((locals.var_t11_dn7 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn7))), ((locals.var_t11_dn8 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn8))), ((locals.var_t11_dn9 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn9))), ((locals.var_t11_dn10 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn10))), ((locals.var_t11_dn13 * assign56320_body8_e87494) + (locals.var_t11 * (-locals.var_t2_dn13))),)
    } else {
        (locals.var_q_sat_dps, locals.var_q_sat_dps_dn0, locals.var_q_sat_dps_dn2, locals.var_q_sat_dps_dn4, locals.var_q_sat_dps_dn5, locals.var_q_sat_dps_dn6, locals.var_q_sat_dps_dn7, locals.var_q_sat_dps_dn8, locals.var_q_sat_dps_dn9, locals.var_q_sat_dps_dn10, locals.var_q_sat_dps_dn13,)
    }
};
            locals.var_q_sat_dps = assign56320_body8_e87497;
            locals.var_q_sat_dps_dn0 = assign56320_body8_e87497_d_n0;
            locals.var_q_sat_dps_dn2 = assign56320_body8_e87497_d_n2;
            locals.var_q_sat_dps_dn4 = assign56320_body8_e87497_d_n4;
            locals.var_q_sat_dps_dn5 = assign56320_body8_e87497_d_n5;
            locals.var_q_sat_dps_dn6 = assign56320_body8_e87497_d_n6;
            locals.var_q_sat_dps_dn7 = assign56320_body8_e87497_d_n7;
            locals.var_q_sat_dps_dn8 = assign56320_body8_e87497_d_n8;
            locals.var_q_sat_dps_dn9 = assign56320_body8_e87497_d_n9;
            locals.var_q_sat_dps_dn10 = assign56320_body8_e87497_d_n10;
            locals.var_q_sat_dps_dn13 = assign56320_body8_e87497_d_n13;
            locals.var_q_sat_dps_rv = 0.0;
            let (assign56320_body9_e87515,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign56320_body9_e87513: f64 = (150.0 + 1.0);
        (assign56320_body9_e87513,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign56320_body9_e87515;
            locals.var_lp_s0_rv = 0.0;
            let (assign56320_body10_e87539, assign56320_body10_e87539_d_n0, assign56320_body10_e87539_d_n2, assign56320_body10_e87539_d_n4, assign56320_body10_e87539_d_n5, assign56320_body10_e87539_d_n6, assign56320_body10_e87539_d_n7, assign56320_body10_e87539_d_n8, assign56320_body10_e87539_d_n9, assign56320_body10_e87539_d_n10, assign56320_body10_e87539_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56320_body10_e87531: f64 = (-locals.var_cox);
        let assign56320_body10_e87534: f64 = (locals.var_vgpsat - locals.var_phi_vsat);
        let assign56320_body10_e87535: f64 = (assign56320_body10_e87531 * assign56320_body10_e87534);
        let assign56320_body10_e87537: f64 = (assign56320_body10_e87535 + locals.var_q_sat);
        (assign56320_body10_e87537, ((((-locals.var_cox_dn0) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn0 - locals.var_phi_vsat_dn0))) + locals.var_q_sat_dn0), ((((-locals.var_cox_dn2) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn2 - locals.var_phi_vsat_dn2))) + locals.var_q_sat_dn2), ((((-locals.var_cox_dn4) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn4 - locals.var_phi_vsat_dn4))) + locals.var_q_sat_dn4), ((((-locals.var_cox_dn5) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn5 - locals.var_phi_vsat_dn5))) + locals.var_q_sat_dn5), ((((-locals.var_cox_dn6) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn6 - locals.var_phi_vsat_dn6))) + locals.var_q_sat_dn6), ((((-locals.var_cox_dn7) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn7 - locals.var_phi_vsat_dn7))) + locals.var_q_sat_dn7), ((((-locals.var_cox_dn8) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn8 - locals.var_phi_vsat_dn8))) + locals.var_q_sat_dn8), ((((-locals.var_cox_dn9) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn9 - locals.var_phi_vsat_dn9))) + locals.var_q_sat_dn9), ((((-locals.var_cox_dn10) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn10 - locals.var_phi_vsat_dn10))) + locals.var_q_sat_dn10), ((((-locals.var_cox_dn13) * assign56320_body10_e87534) + (assign56320_body10_e87531 * (locals.var_vgpsat_dn13 - locals.var_phi_vsat_dn13))) + locals.var_q_sat_dn13),)
    } else {
        (locals.var_pf1__blk1100, locals.var_pf1__blk1100_dn0, locals.var_pf1__blk1100_dn2, locals.var_pf1__blk1100_dn4, locals.var_pf1__blk1100_dn5, locals.var_pf1__blk1100_dn6, locals.var_pf1__blk1100_dn7, locals.var_pf1__blk1100_dn8, locals.var_pf1__blk1100_dn9, locals.var_pf1__blk1100_dn10, locals.var_pf1__blk1100_dn13,)
    }
};
            locals.var_pf1__blk1100 = assign56320_body10_e87539;
            locals.var_pf1__blk1100_dn0 = assign56320_body10_e87539_d_n0;
            locals.var_pf1__blk1100_dn2 = assign56320_body10_e87539_d_n2;
            locals.var_pf1__blk1100_dn4 = assign56320_body10_e87539_d_n4;
            locals.var_pf1__blk1100_dn5 = assign56320_body10_e87539_d_n5;
            locals.var_pf1__blk1100_dn6 = assign56320_body10_e87539_d_n6;
            locals.var_pf1__blk1100_dn7 = assign56320_body10_e87539_d_n7;
            locals.var_pf1__blk1100_dn8 = assign56320_body10_e87539_d_n8;
            locals.var_pf1__blk1100_dn9 = assign56320_body10_e87539_d_n9;
            locals.var_pf1__blk1100_dn10 = assign56320_body10_e87539_d_n10;
            locals.var_pf1__blk1100_dn13 = assign56320_body10_e87539_d_n13;
            locals.var_pf1__blk1100_rv = 0.0;
            let (assign56320_body11_e87558, assign56320_body11_e87558_d_n0, assign56320_body11_e87558_d_n2, assign56320_body11_e87558_d_n4, assign56320_body11_e87558_d_n5, assign56320_body11_e87558_d_n6, assign56320_body11_e87558_d_n7, assign56320_body11_e87558_d_n8, assign56320_body11_e87558_d_n9, assign56320_body11_e87558_d_n10, assign56320_body11_e87558_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56320_body11_e87556: f64 = (locals.var_cox + locals.var_q_sat_dps);
        (assign56320_body11_e87556, (locals.var_cox_dn0 + locals.var_q_sat_dps_dn0), (locals.var_cox_dn2 + locals.var_q_sat_dps_dn2), (locals.var_cox_dn4 + locals.var_q_sat_dps_dn4), (locals.var_cox_dn5 + locals.var_q_sat_dps_dn5), (locals.var_cox_dn6 + locals.var_q_sat_dps_dn6), (locals.var_cox_dn7 + locals.var_q_sat_dps_dn7), (locals.var_cox_dn8 + locals.var_q_sat_dps_dn8), (locals.var_cox_dn9 + locals.var_q_sat_dps_dn9), (locals.var_cox_dn10 + locals.var_q_sat_dps_dn10), (locals.var_cox_dn13 + locals.var_q_sat_dps_dn13),)
    } else {
        (locals.var_pf11__blk1101, locals.var_pf11__blk1101_dn0, locals.var_pf11__blk1101_dn2, locals.var_pf11__blk1101_dn4, locals.var_pf11__blk1101_dn5, locals.var_pf11__blk1101_dn6, locals.var_pf11__blk1101_dn7, locals.var_pf11__blk1101_dn8, locals.var_pf11__blk1101_dn9, locals.var_pf11__blk1101_dn10, locals.var_pf11__blk1101_dn13,)
    }
};
            locals.var_pf11__blk1101 = assign56320_body11_e87558;
            locals.var_pf11__blk1101_dn0 = assign56320_body11_e87558_d_n0;
            locals.var_pf11__blk1101_dn2 = assign56320_body11_e87558_d_n2;
            locals.var_pf11__blk1101_dn4 = assign56320_body11_e87558_d_n4;
            locals.var_pf11__blk1101_dn5 = assign56320_body11_e87558_d_n5;
            locals.var_pf11__blk1101_dn6 = assign56320_body11_e87558_d_n6;
            locals.var_pf11__blk1101_dn7 = assign56320_body11_e87558_d_n7;
            locals.var_pf11__blk1101_dn8 = assign56320_body11_e87558_d_n8;
            locals.var_pf11__blk1101_dn9 = assign56320_body11_e87558_d_n9;
            locals.var_pf11__blk1101_dn10 = assign56320_body11_e87558_d_n10;
            locals.var_pf11__blk1101_dn13 = assign56320_body11_e87558_d_n13;
            locals.var_pf11__blk1101_rv = 0.0;
            let (assign56320_body12_e87578, assign56320_body12_e87578_d_n0, assign56320_body12_e87578_d_n2, assign56320_body12_e87578_d_n4, assign56320_body12_e87578_d_n5, assign56320_body12_e87578_d_n6, assign56320_body12_e87578_d_n7, assign56320_body12_e87578_d_n8, assign56320_body12_e87578_d_n9, assign56320_body12_e87578_d_n10, assign56320_body12_e87578_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56320_body12_e87574: f64 = (-locals.var_pf1__blk1100);
        let assign56320_body12_e87576: f64 = (assign56320_body12_e87574 / locals.var_pf11__blk1101);
        (assign56320_body12_e87576, ((((-locals.var_pf1__blk1100_dn0) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn0)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn2) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn2)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn4) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn4)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn5) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn5)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn6) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn6)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn7) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn7)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn8) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn8)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn9) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn9)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn10) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn10)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)), ((((-locals.var_pf1__blk1100_dn13) * locals.var_pf11__blk1101) - (assign56320_body12_e87574 * locals.var_pf11__blk1101_dn13)) / (locals.var_pf11__blk1101 * locals.var_pf11__blk1101)),)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign56320_body12_e87578;
            locals.var_dps__blk1112_dn0 = assign56320_body12_e87578_d_n0;
            locals.var_dps__blk1112_dn2 = assign56320_body12_e87578_d_n2;
            locals.var_dps__blk1112_dn4 = assign56320_body12_e87578_d_n4;
            locals.var_dps__blk1112_dn5 = assign56320_body12_e87578_d_n5;
            locals.var_dps__blk1112_dn6 = assign56320_body12_e87578_d_n6;
            locals.var_dps__blk1112_dn7 = assign56320_body12_e87578_d_n7;
            locals.var_dps__blk1112_dn8 = assign56320_body12_e87578_d_n8;
            locals.var_dps__blk1112_dn9 = assign56320_body12_e87578_d_n9;
            locals.var_dps__blk1112_dn10 = assign56320_body12_e87578_d_n10;
            locals.var_dps__blk1112_dn13 = assign56320_body12_e87578_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let assign56320_body13_e87580: f64 = (locals.var_dps__blk1112).abs();
            let assign56320_body13_e87582: f64 = if assign56320_body13_e87580 < 1e-10 { 1.0 } else { 0.0 };
            locals.var_guard1409 = assign56320_body13_e87582;
            locals.var_guard1409_rv = 0.0;
            let (assign56320_body14_e87601,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1409 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign56320_body14_e87601;
            locals.var_flg_conv_rv = 0.0;
            let assign56320_body15_e87604: f64 = if locals.var_dps__blk1112 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1410 = assign56320_body15_e87604;
            locals.var_guard1410_rv = 0.0;
            let (assign56320_body16_e87626, assign56320_body16_e87626_d_n0, assign56320_body16_e87626_d_n2, assign56320_body16_e87626_d_n4, assign56320_body16_e87626_d_n5, assign56320_body16_e87626_d_n6, assign56320_body16_e87626_d_n7, assign56320_body16_e87626_d_n8, assign56320_body16_e87626_d_n9, assign56320_body16_e87626_d_n10, assign56320_body16_e87626_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1409 == 0.0)) && (locals.var_guard1410 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign56320_body16_e87626;
            locals.var_dps__blk1112_dn0 = assign56320_body16_e87626_d_n0;
            locals.var_dps__blk1112_dn2 = assign56320_body16_e87626_d_n2;
            locals.var_dps__blk1112_dn4 = assign56320_body16_e87626_d_n4;
            locals.var_dps__blk1112_dn5 = assign56320_body16_e87626_d_n5;
            locals.var_dps__blk1112_dn6 = assign56320_body16_e87626_d_n6;
            locals.var_dps__blk1112_dn7 = assign56320_body16_e87626_d_n7;
            locals.var_dps__blk1112_dn8 = assign56320_body16_e87626_d_n8;
            locals.var_dps__blk1112_dn9 = assign56320_body16_e87626_d_n9;
            locals.var_dps__blk1112_dn10 = assign56320_body16_e87626_d_n10;
            locals.var_dps__blk1112_dn13 = assign56320_body16_e87626_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let assign56320_body17_e87629: f64 = (-0.1);
            let assign56320_body17_e87630: f64 = if locals.var_dps__blk1112 < assign56320_body17_e87629 { 1.0 } else { 0.0 };
            locals.var_guard1411 = assign56320_body17_e87630;
            locals.var_guard1411_rv = 0.0;
            let (assign56320_body18_e87656, assign56320_body18_e87656_d_n0, assign56320_body18_e87656_d_n2, assign56320_body18_e87656_d_n4, assign56320_body18_e87656_d_n5, assign56320_body18_e87656_d_n6, assign56320_body18_e87656_d_n7, assign56320_body18_e87656_d_n8, assign56320_body18_e87656_d_n9, assign56320_body18_e87656_d_n10, assign56320_body18_e87656_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1409 == 0.0)) && (locals.var_guard1410 == 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign56320_body18_e87654: f64 = (-0.1);
        (assign56320_body18_e87654, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1112, locals.var_dps__blk1112_dn0, locals.var_dps__blk1112_dn2, locals.var_dps__blk1112_dn4, locals.var_dps__blk1112_dn5, locals.var_dps__blk1112_dn6, locals.var_dps__blk1112_dn7, locals.var_dps__blk1112_dn8, locals.var_dps__blk1112_dn9, locals.var_dps__blk1112_dn10, locals.var_dps__blk1112_dn13,)
    }
};
            locals.var_dps__blk1112 = assign56320_body18_e87656;
            locals.var_dps__blk1112_dn0 = assign56320_body18_e87656_d_n0;
            locals.var_dps__blk1112_dn2 = assign56320_body18_e87656_d_n2;
            locals.var_dps__blk1112_dn4 = assign56320_body18_e87656_d_n4;
            locals.var_dps__blk1112_dn5 = assign56320_body18_e87656_d_n5;
            locals.var_dps__blk1112_dn6 = assign56320_body18_e87656_d_n6;
            locals.var_dps__blk1112_dn7 = assign56320_body18_e87656_d_n7;
            locals.var_dps__blk1112_dn8 = assign56320_body18_e87656_d_n8;
            locals.var_dps__blk1112_dn9 = assign56320_body18_e87656_d_n9;
            locals.var_dps__blk1112_dn10 = assign56320_body18_e87656_d_n10;
            locals.var_dps__blk1112_dn13 = assign56320_body18_e87656_d_n13;
            locals.var_dps__blk1112_rv = 0.0;
            let (assign56320_body19_e87675, assign56320_body19_e87675_d_n0, assign56320_body19_e87675_d_n2, assign56320_body19_e87675_d_n4, assign56320_body19_e87675_d_n5, assign56320_body19_e87675_d_n6, assign56320_body19_e87675_d_n7, assign56320_body19_e87675_d_n8, assign56320_body19_e87675_d_n9, assign56320_body19_e87675_d_n10, assign56320_body19_e87675_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign56320_body19_e87673: f64 = (locals.var_phi_vsat + locals.var_dps__blk1112);
        (assign56320_body19_e87673, (locals.var_phi_vsat_dn0 + locals.var_dps__blk1112_dn0), (locals.var_phi_vsat_dn2 + locals.var_dps__blk1112_dn2), (locals.var_phi_vsat_dn4 + locals.var_dps__blk1112_dn4), (locals.var_phi_vsat_dn5 + locals.var_dps__blk1112_dn5), (locals.var_phi_vsat_dn6 + locals.var_dps__blk1112_dn6), (locals.var_phi_vsat_dn7 + locals.var_dps__blk1112_dn7), (locals.var_phi_vsat_dn8 + locals.var_dps__blk1112_dn8), (locals.var_phi_vsat_dn9 + locals.var_dps__blk1112_dn9), (locals.var_phi_vsat_dn10 + locals.var_dps__blk1112_dn10), (locals.var_phi_vsat_dn13 + locals.var_dps__blk1112_dn13),)
    } else {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn13,)
    }
};
            locals.var_phi_vsat = assign56320_body19_e87675;
            locals.var_phi_vsat_dn0 = assign56320_body19_e87675_d_n0;
            locals.var_phi_vsat_dn2 = assign56320_body19_e87675_d_n2;
            locals.var_phi_vsat_dn4 = assign56320_body19_e87675_d_n4;
            locals.var_phi_vsat_dn5 = assign56320_body19_e87675_d_n5;
            locals.var_phi_vsat_dn6 = assign56320_body19_e87675_d_n6;
            locals.var_phi_vsat_dn7 = assign56320_body19_e87675_d_n7;
            locals.var_phi_vsat_dn8 = assign56320_body19_e87675_d_n8;
            locals.var_phi_vsat_dn9 = assign56320_body19_e87675_d_n9;
            locals.var_phi_vsat_dn10 = assign56320_body19_e87675_d_n10;
            locals.var_phi_vsat_dn13 = assign56320_body19_e87675_d_n13;
            locals.var_phi_vsat_rv = 0.0;
            let (assign56320_body20_e87691,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56320_body20_e87689: f64 = (locals.var_lp_s0 + 1.0);
        (assign56320_body20_e87689,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign56320_body20_e87691;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign56330_e87705, assign56330_e87705_d_n0, assign56330_e87705_d_n2, assign56330_e87705_d_n4, assign56330_e87705_d_n5, assign56330_e87705_d_n6, assign56330_e87705_d_n7, assign56330_e87705_d_n8, assign56330_e87705_d_n9, assign56330_e87705_d_n10, assign56330_e87705_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        (locals.var_phi_vsat, locals.var_phi_vsat_dn0, locals.var_phi_vsat_dn2, locals.var_phi_vsat_dn4, locals.var_phi_vsat_dn5, locals.var_phi_vsat_dn6, locals.var_phi_vsat_dn7, locals.var_phi_vsat_dn8, locals.var_phi_vsat_dn9, locals.var_phi_vsat_dn10, locals.var_phi_vsat_dn13,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn13,)
    }
};
        locals.var_ps0_res = assign56330_e87705;
        locals.var_ps0_res_dn0 = assign56330_e87705_d_n0;
        locals.var_ps0_res_dn2 = assign56330_e87705_d_n2;
        locals.var_ps0_res_dn4 = assign56330_e87705_d_n4;
        locals.var_ps0_res_dn5 = assign56330_e87705_d_n5;
        locals.var_ps0_res_dn6 = assign56330_e87705_d_n6;
        locals.var_ps0_res_dn7 = assign56330_e87705_d_n7;
        locals.var_ps0_res_dn8 = assign56330_e87705_d_n8;
        locals.var_ps0_res_dn9 = assign56330_e87705_d_n9;
        locals.var_ps0_res_dn10 = assign56330_e87705_d_n10;
        locals.var_ps0_res_dn13 = assign56330_e87705_d_n13;
        locals.var_ps0_res_rv = 0.0;

        let (assign56340_e87719, assign56340_e87719_d_n0, assign56340_e87719_d_n2, assign56340_e87719_d_n4, assign56340_e87719_d_n5, assign56340_e87719_d_n6, assign56340_e87719_d_n7, assign56340_e87719_d_n8, assign56340_e87719_d_n9, assign56340_e87719_d_n10, assign56340_e87719_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn13,)
    }
};
        locals.var_vds_res = assign56340_e87719;
        locals.var_vds_res_dn0 = assign56340_e87719_d_n0;
        locals.var_vds_res_dn2 = assign56340_e87719_d_n2;
        locals.var_vds_res_dn4 = assign56340_e87719_d_n4;
        locals.var_vds_res_dn5 = assign56340_e87719_d_n5;
        locals.var_vds_res_dn6 = assign56340_e87719_d_n6;
        locals.var_vds_res_dn7 = assign56340_e87719_d_n7;
        locals.var_vds_res_dn8 = assign56340_e87719_d_n8;
        locals.var_vds_res_dn9 = assign56340_e87719_d_n9;
        locals.var_vds_res_dn10 = assign56340_e87719_d_n10;
        locals.var_vds_res_dn13 = assign56340_e87719_d_n13;
        locals.var_vds_res_rv = 0.0;

        let (assign56350_e87742, assign56350_e87742_d_n0, assign56350_e87742_d_n2, assign56350_e87742_d_n4, assign56350_e87742_d_n5, assign56350_e87742_d_n6, assign56350_e87742_d_n7, assign56350_e87742_d_n8, assign56350_e87742_d_n9, assign56350_e87742_d_n10, assign56350_e87742_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56350_e87733: f64 = (locals.var_ps0_res * locals.var_ps0_res);
        let assign56350_e87736: f64 = (4.0 * p.p405);
        let assign56350_e87738: f64 = (assign56350_e87736 * p.p405);
        let assign56350_e87739: f64 = (assign56350_e87733 + assign56350_e87738);
        let assign56350_e87740: f64 = (assign56350_e87739).sqrt();
        (assign56350_e87740, (((locals.var_ps0_res_dn0 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn0)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn2 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn2)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn4 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn4)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn5 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn5)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn6 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn6)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn7 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn7)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn8 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn8)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn9 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn9)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn10 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn10)) / (2.0 * assign56350_e87740)), (((locals.var_ps0_res_dn13 * locals.var_ps0_res) + (locals.var_ps0_res * locals.var_ps0_res_dn13)) / (2.0 * assign56350_e87740)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign56350_e87742;
        locals.var_tmf2_dn0 = assign56350_e87742_d_n0;
        locals.var_tmf2_dn2 = assign56350_e87742_d_n2;
        locals.var_tmf2_dn4 = assign56350_e87742_d_n4;
        locals.var_tmf2_dn5 = assign56350_e87742_d_n5;
        locals.var_tmf2_dn6 = assign56350_e87742_d_n6;
        locals.var_tmf2_dn7 = assign56350_e87742_d_n7;
        locals.var_tmf2_dn8 = assign56350_e87742_d_n8;
        locals.var_tmf2_dn9 = assign56350_e87742_d_n9;
        locals.var_tmf2_dn10 = assign56350_e87742_d_n10;
        locals.var_tmf2_dn13 = assign56350_e87742_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign56360_e87762, assign56360_e87762_d_n0, assign56360_e87762_d_n2, assign56360_e87762_d_n4, assign56360_e87762_d_n5, assign56360_e87762_d_n6, assign56360_e87762_d_n7, assign56360_e87762_d_n8, assign56360_e87762_d_n9, assign56360_e87762_d_n10, assign56360_e87762_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56360_e87758: f64 = (locals.var_ps0_res / locals.var_tmf2);
        let assign56360_e87759: f64 = (1.0 + assign56360_e87758);
        let assign56360_e87760: f64 = (0.5 * assign56360_e87759);
        (assign56360_e87760, (0.5 * (((locals.var_ps0_res_dn0 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn2 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn4 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn5 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn6 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn7 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn8 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn9 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn10 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_ps0_res_dn13 * locals.var_tmf2) - (locals.var_ps0_res * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign56360_e87762;
        locals.var_t0_dn0 = assign56360_e87762_d_n0;
        locals.var_t0_dn2 = assign56360_e87762_d_n2;
        locals.var_t0_dn4 = assign56360_e87762_d_n4;
        locals.var_t0_dn5 = assign56360_e87762_d_n5;
        locals.var_t0_dn6 = assign56360_e87762_d_n6;
        locals.var_t0_dn7 = assign56360_e87762_d_n7;
        locals.var_t0_dn8 = assign56360_e87762_d_n8;
        locals.var_t0_dn9 = assign56360_e87762_d_n9;
        locals.var_t0_dn10 = assign56360_e87762_d_n10;
        locals.var_t0_dn13 = assign56360_e87762_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign56370_e87780, assign56370_e87780_d_n0, assign56370_e87780_d_n2, assign56370_e87780_d_n4, assign56370_e87780_d_n5, assign56370_e87780_d_n6, assign56370_e87780_d_n7, assign56370_e87780_d_n8, assign56370_e87780_d_n9, assign56370_e87780_d_n10, assign56370_e87780_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) {
        let assign56370_e87777: f64 = (locals.var_ps0_res + locals.var_tmf2);
        let assign56370_e87778: f64 = (0.5 * assign56370_e87777);
        (assign56370_e87778, (0.5 * (locals.var_ps0_res_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_ps0_res_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_ps0_res_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_ps0_res_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_ps0_res_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_ps0_res_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_ps0_res_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_ps0_res_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_ps0_res_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_ps0_res_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn13,)
    }
};
        locals.var_vdssat_res = assign56370_e87780;
        locals.var_vdssat_res_dn0 = assign56370_e87780_d_n0;
        locals.var_vdssat_res_dn2 = assign56370_e87780_d_n2;
        locals.var_vdssat_res_dn4 = assign56370_e87780_d_n4;
        locals.var_vdssat_res_dn5 = assign56370_e87780_d_n5;
        locals.var_vdssat_res_dn6 = assign56370_e87780_d_n6;
        locals.var_vdssat_res_dn7 = assign56370_e87780_d_n7;
        locals.var_vdssat_res_dn8 = assign56370_e87780_d_n8;
        locals.var_vdssat_res_dn9 = assign56370_e87780_d_n9;
        locals.var_vdssat_res_dn10 = assign56370_e87780_d_n10;
        locals.var_vdssat_res_dn13 = assign56370_e87780_d_n13;
        locals.var_vdssat_res_rv = 0.0;

        let assign56380_e87783: f64 = if locals.var_vdssat_res < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1412 = assign56380_e87783;
        locals.var_guard1412_rv = 0.0;

        let (assign56390_e87799, assign56390_e87799_d_n0, assign56390_e87799_d_n2, assign56390_e87799_d_n4, assign56390_e87799_d_n5, assign56390_e87799_d_n6, assign56390_e87799_d_n7, assign56390_e87799_d_n8, assign56390_e87799_d_n9, assign56390_e87799_d_n10, assign56390_e87799_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_guard1412 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdssat_res, locals.var_vdssat_res_dn0, locals.var_vdssat_res_dn2, locals.var_vdssat_res_dn4, locals.var_vdssat_res_dn5, locals.var_vdssat_res_dn6, locals.var_vdssat_res_dn7, locals.var_vdssat_res_dn8, locals.var_vdssat_res_dn9, locals.var_vdssat_res_dn10, locals.var_vdssat_res_dn13,)
    }
};
        locals.var_vdssat_res = assign56390_e87799;
        locals.var_vdssat_res_dn0 = assign56390_e87799_d_n0;
        locals.var_vdssat_res_dn2 = assign56390_e87799_d_n2;
        locals.var_vdssat_res_dn4 = assign56390_e87799_d_n4;
        locals.var_vdssat_res_dn5 = assign56390_e87799_d_n5;
        locals.var_vdssat_res_dn6 = assign56390_e87799_d_n6;
        locals.var_vdssat_res_dn7 = assign56390_e87799_d_n7;
        locals.var_vdssat_res_dn8 = assign56390_e87799_d_n8;
        locals.var_vdssat_res_dn9 = assign56390_e87799_d_n9;
        locals.var_vdssat_res_dn10 = assign56390_e87799_d_n10;
        locals.var_vdssat_res_dn13 = assign56390_e87799_d_n13;
        locals.var_vdssat_res_rv = 0.0;

        let (assign56400_e87815, assign56400_e87815_d_n0, assign56400_e87815_d_n2, assign56400_e87815_d_n4, assign56400_e87815_d_n5, assign56400_e87815_d_n6, assign56400_e87815_d_n7, assign56400_e87815_d_n8, assign56400_e87815_d_n9, assign56400_e87815_d_n10, assign56400_e87815_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1395 == 0.0)) && (locals.var_guard1412 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign56400_e87815;
        locals.var_t0_dn0 = assign56400_e87815_d_n0;
        locals.var_t0_dn2 = assign56400_e87815_d_n2;
        locals.var_t0_dn4 = assign56400_e87815_d_n4;
        locals.var_t0_dn5 = assign56400_e87815_d_n5;
        locals.var_t0_dn6 = assign56400_e87815_d_n6;
        locals.var_t0_dn7 = assign56400_e87815_d_n7;
        locals.var_t0_dn8 = assign56400_e87815_d_n8;
        locals.var_t0_dn9 = assign56400_e87815_d_n9;
        locals.var_t0_dn10 = assign56400_e87815_d_n10;
        locals.var_t0_dn13 = assign56400_e87815_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_198(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign56410_e87828, assign56410_e87828_d_n0, assign56410_e87828_d_n2, assign56410_e87828_d_n4, assign56410_e87828_d_n5, assign56410_e87828_d_n6, assign56410_e87828_d_n7, assign56410_e87828_d_n8, assign56410_e87828_d_n9, assign56410_e87828_d_n10, assign56410_e87828_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign56410_e87826: f64 = (locals.var_vds_res / locals.var_vdssat_res);
        (assign56410_e87826, (((locals.var_vds_res_dn0 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn0)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn2 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn2)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn4 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn4)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn5 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn5)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn6 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn6)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn7 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn7)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn8 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn8)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn9 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn9)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn10 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn10)) / (locals.var_vdssat_res * locals.var_vdssat_res)), (((locals.var_vds_res_dn13 * locals.var_vdssat_res) - (locals.var_vds_res * locals.var_vdssat_res_dn13)) / (locals.var_vdssat_res * locals.var_vdssat_res)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign56410_e87828;
        locals.var_t1_dn0 = assign56410_e87828_d_n0;
        locals.var_t1_dn2 = assign56410_e87828_d_n2;
        locals.var_t1_dn4 = assign56410_e87828_d_n4;
        locals.var_t1_dn5 = assign56410_e87828_d_n5;
        locals.var_t1_dn6 = assign56410_e87828_d_n6;
        locals.var_t1_dn7 = assign56410_e87828_d_n7;
        locals.var_t1_dn8 = assign56410_e87828_d_n8;
        locals.var_t1_dn9 = assign56410_e87828_d_n9;
        locals.var_t1_dn10 = assign56410_e87828_d_n10;
        locals.var_t1_dn13 = assign56410_e87828_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign56420_e87848, assign56420_e87848_d_n0, assign56420_e87848_d_n2, assign56420_e87848_d_n4, assign56420_e87848_d_n5, assign56420_e87848_d_n6, assign56420_e87848_d_n7, assign56420_e87848_d_n8, assign56420_e87848_d_n9, assign56420_e87848_d_n10, assign56420_e87848_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let (assign56420_e87846, assign56420_e87846_d_n0, assign56420_e87846_d_n2, assign56420_e87846_d_n4, assign56420_e87846_d_n5, assign56420_e87846_d_n6, assign56420_e87846_d_n7, assign56420_e87846_d_n8, assign56420_e87846_d_n9, assign56420_e87846_d_n10, assign56420_e87846_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56420_e87844: f64 = (p.p383 - 1.0);
                let assign56420_e87845: f64 = (locals.var_t1).powf(assign56420_e87844);
                (assign56420_e87845, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn0)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn2)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn4)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn5)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn6)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn7)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn8)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn9)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn10)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign56420_e87844) as f64).is_finite() && ((assign56420_e87844) as f64).fract() == 0.0 { if assign56420_e87844 == 0.0 { 0.0 } else { (assign56420_e87844 * ((locals.var_t1).powf(assign56420_e87844 - 1.0) * locals.var_t1_dn13)) } } else { (assign56420_e87845 * (assign56420_e87844 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign56420_e87846, assign56420_e87846_d_n0, assign56420_e87846_d_n2, assign56420_e87846_d_n4, assign56420_e87846_d_n5, assign56420_e87846_d_n6, assign56420_e87846_d_n7, assign56420_e87846_d_n8, assign56420_e87846_d_n9, assign56420_e87846_d_n10, assign56420_e87846_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign56420_e87848;
        locals.var_t2_dn0 = assign56420_e87848_d_n0;
        locals.var_t2_dn2 = assign56420_e87848_d_n2;
        locals.var_t2_dn4 = assign56420_e87848_d_n4;
        locals.var_t2_dn5 = assign56420_e87848_d_n5;
        locals.var_t2_dn6 = assign56420_e87848_d_n6;
        locals.var_t2_dn7 = assign56420_e87848_d_n7;
        locals.var_t2_dn8 = assign56420_e87848_d_n8;
        locals.var_t2_dn9 = assign56420_e87848_d_n9;
        locals.var_t2_dn10 = assign56420_e87848_d_n10;
        locals.var_t2_dn13 = assign56420_e87848_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign56430_e87863, assign56430_e87863_d_n0, assign56430_e87863_d_n2, assign56430_e87863_d_n4, assign56430_e87863_d_n5, assign56430_e87863_d_n6, assign56430_e87863_d_n7, assign56430_e87863_d_n8, assign56430_e87863_d_n9, assign56430_e87863_d_n10, assign56430_e87863_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign56430_e87860: f64 = (locals.var_t2 * locals.var_t1);
        let assign56430_e87861: f64 = (1.0 + assign56430_e87860);
        (assign56430_e87861, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn13 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign56430_e87863;
        locals.var_t3_dn0 = assign56430_e87863_d_n0;
        locals.var_t3_dn2 = assign56430_e87863_d_n2;
        locals.var_t3_dn4 = assign56430_e87863_d_n4;
        locals.var_t3_dn5 = assign56430_e87863_d_n5;
        locals.var_t3_dn6 = assign56430_e87863_d_n6;
        locals.var_t3_dn7 = assign56430_e87863_d_n7;
        locals.var_t3_dn8 = assign56430_e87863_d_n8;
        locals.var_t3_dn9 = assign56430_e87863_d_n9;
        locals.var_t3_dn10 = assign56430_e87863_d_n10;
        locals.var_t3_dn13 = assign56430_e87863_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign56440_e87885, assign56440_e87885_d_n0, assign56440_e87885_d_n2, assign56440_e87885_d_n4, assign56440_e87885_d_n5, assign56440_e87885_d_n6, assign56440_e87885_d_n7, assign56440_e87885_d_n8, assign56440_e87885_d_n9, assign56440_e87885_d_n10, assign56440_e87885_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let (assign56440_e87883, assign56440_e87883_d_n0, assign56440_e87883_d_n2, assign56440_e87883_d_n4, assign56440_e87883_d_n5, assign56440_e87883_d_n6, assign56440_e87883_d_n7, assign56440_e87883_d_n8, assign56440_e87883_d_n9, assign56440_e87883_d_n10, assign56440_e87883_d_n13,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign56440_e87879: f64 = (1.0 / p.p383);
                let assign56440_e87881: f64 = (assign56440_e87879 - 1.0);
                let assign56440_e87882: f64 = (locals.var_t3).powf(assign56440_e87881);
                (assign56440_e87882, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn0)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn2)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn4)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn5)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn6)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn7)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn8)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn9)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn10)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign56440_e87881) as f64).is_finite() && ((assign56440_e87881) as f64).fract() == 0.0 { if assign56440_e87881 == 0.0 { 0.0 } else { (assign56440_e87881 * ((locals.var_t3).powf(assign56440_e87881 - 1.0) * locals.var_t3_dn13)) } } else { (assign56440_e87882 * (assign56440_e87881 * (locals.var_t3_dn13 / locals.var_t3))) },)
            }
        };
        (assign56440_e87883, assign56440_e87883_d_n0, assign56440_e87883_d_n2, assign56440_e87883_d_n4, assign56440_e87883_d_n5, assign56440_e87883_d_n6, assign56440_e87883_d_n7, assign56440_e87883_d_n8, assign56440_e87883_d_n9, assign56440_e87883_d_n10, assign56440_e87883_d_n13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign56440_e87885;
        locals.var_t4_dn0 = assign56440_e87885_d_n0;
        locals.var_t4_dn2 = assign56440_e87885_d_n2;
        locals.var_t4_dn4 = assign56440_e87885_d_n4;
        locals.var_t4_dn5 = assign56440_e87885_d_n5;
        locals.var_t4_dn6 = assign56440_e87885_d_n6;
        locals.var_t4_dn7 = assign56440_e87885_d_n7;
        locals.var_t4_dn8 = assign56440_e87885_d_n8;
        locals.var_t4_dn9 = assign56440_e87885_d_n9;
        locals.var_t4_dn10 = assign56440_e87885_d_n10;
        locals.var_t4_dn13 = assign56440_e87885_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign56450_e87898, assign56450_e87898_d_n0, assign56450_e87898_d_n2, assign56450_e87898_d_n4, assign56450_e87898_d_n5, assign56450_e87898_d_n6, assign56450_e87898_d_n7, assign56450_e87898_d_n8, assign56450_e87898_d_n9, assign56450_e87898_d_n10, assign56450_e87898_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) {
        let assign56450_e87896: f64 = (locals.var_t4 * locals.var_t3);
        (assign56450_e87896, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn13 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign56450_e87898;
        locals.var_t6_dn0 = assign56450_e87898_d_n0;
        locals.var_t6_dn2 = assign56450_e87898_d_n2;
        locals.var_t6_dn4 = assign56450_e87898_d_n4;
        locals.var_t6_dn5 = assign56450_e87898_d_n5;
        locals.var_t6_dn6 = assign56450_e87898_d_n6;
        locals.var_t6_dn7 = assign56450_e87898_d_n7;
        locals.var_t6_dn8 = assign56450_e87898_d_n8;
        locals.var_t6_dn9 = assign56450_e87898_d_n9;
        locals.var_t6_dn10 = assign56450_e87898_d_n10;
        locals.var_t6_dn13 = assign56450_e87898_d_n13;
        locals.var_t6_rv = 0.0;

        let assign56460_e87903: f64 = (locals.var_uc_depleak * 0.5);
        let assign56460_e87904: f64 = (locals.var_uc_depleak - assign56460_e87903);
        let assign56460_e87908: f64 = (locals.var_uc_depleak * 0.5);
        let assign56460_e87911: f64 = if ((locals.var_vdsorg > assign56460_e87904) && (assign56460_e87908 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1413 = assign56460_e87911;
        locals.var_guard1413_rv = 0.0;

        let (assign56470_e87930, assign56470_e87930_d_n0, assign56470_e87930_d_n2, assign56470_e87930_d_n4, assign56470_e87930_d_n5, assign56470_e87930_d_n6, assign56470_e87930_d_n7, assign56470_e87930_d_n8, assign56470_e87930_d_n9, assign56470_e87930_d_n10, assign56470_e87930_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        let assign56470_e87924: f64 = (locals.var_vdsorg - locals.var_uc_depleak);
        let assign56470_e87927: f64 = (locals.var_uc_depleak * 0.5);
        let assign56470_e87928: f64 = (assign56470_e87924 + assign56470_e87927);
        (assign56470_e87928, ((locals.var_vdsorg_dn0 - locals.var_uc_depleak_dn0) + (locals.var_uc_depleak_dn0 * 0.5)), ((locals.var_vdsorg_dn2 - locals.var_uc_depleak_dn2) + (locals.var_uc_depleak_dn2 * 0.5)), ((locals.var_vdsorg_dn4 - locals.var_uc_depleak_dn4) + (locals.var_uc_depleak_dn4 * 0.5)), ((locals.var_vdsorg_dn5 - locals.var_uc_depleak_dn5) + (locals.var_uc_depleak_dn5 * 0.5)), ((locals.var_vdsorg_dn6 - locals.var_uc_depleak_dn6) + (locals.var_uc_depleak_dn6 * 0.5)), ((locals.var_vdsorg_dn7 - locals.var_uc_depleak_dn7) + (locals.var_uc_depleak_dn7 * 0.5)), ((locals.var_vdsorg_dn8 - locals.var_uc_depleak_dn8) + (locals.var_uc_depleak_dn8 * 0.5)), ((locals.var_vdsorg_dn9 - locals.var_uc_depleak_dn9) + (locals.var_uc_depleak_dn9 * 0.5)), ((locals.var_vdsorg_dn10 - locals.var_uc_depleak_dn10) + (locals.var_uc_depleak_dn10 * 0.5)), ((locals.var_vdsorg_dn13 - locals.var_uc_depleak_dn13) + (locals.var_uc_depleak_dn13 * 0.5)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign56470_e87930;
        locals.var_tmf1_dn0 = assign56470_e87930_d_n0;
        locals.var_tmf1_dn2 = assign56470_e87930_d_n2;
        locals.var_tmf1_dn4 = assign56470_e87930_d_n4;
        locals.var_tmf1_dn5 = assign56470_e87930_d_n5;
        locals.var_tmf1_dn6 = assign56470_e87930_d_n6;
        locals.var_tmf1_dn7 = assign56470_e87930_d_n7;
        locals.var_tmf1_dn8 = assign56470_e87930_d_n8;
        locals.var_tmf1_dn9 = assign56470_e87930_d_n9;
        locals.var_tmf1_dn10 = assign56470_e87930_d_n10;
        locals.var_tmf1_dn13 = assign56470_e87930_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign56480_e87945, assign56480_e87945_d_n0, assign56480_e87945_d_n2, assign56480_e87945_d_n4, assign56480_e87945_d_n5, assign56480_e87945_d_n6, assign56480_e87945_d_n7, assign56480_e87945_d_n8, assign56480_e87945_d_n9, assign56480_e87945_d_n10, assign56480_e87945_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        let assign56480_e87943: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign56480_e87943, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign56480_e87945;
        locals.var_x2_dn0 = assign56480_e87945_d_n0;
        locals.var_x2_dn2 = assign56480_e87945_d_n2;
        locals.var_x2_dn4 = assign56480_e87945_d_n4;
        locals.var_x2_dn5 = assign56480_e87945_d_n5;
        locals.var_x2_dn6 = assign56480_e87945_d_n6;
        locals.var_x2_dn7 = assign56480_e87945_d_n7;
        locals.var_x2_dn8 = assign56480_e87945_d_n8;
        locals.var_x2_dn9 = assign56480_e87945_d_n9;
        locals.var_x2_dn10 = assign56480_e87945_d_n10;
        locals.var_x2_dn13 = assign56480_e87945_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign56490_e87964, assign56490_e87964_d_n0, assign56490_e87964_d_n2, assign56490_e87964_d_n4, assign56490_e87964_d_n5, assign56490_e87964_d_n6, assign56490_e87964_d_n7, assign56490_e87964_d_n8, assign56490_e87964_d_n9, assign56490_e87964_d_n10, assign56490_e87964_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        let assign56490_e87958: f64 = (locals.var_uc_depleak * 0.5);
        let assign56490_e87961: f64 = (locals.var_uc_depleak * 0.5);
        let assign56490_e87962: f64 = (assign56490_e87958 * assign56490_e87961);
        (assign56490_e87962, (((locals.var_uc_depleak_dn0 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn0 * 0.5))), (((locals.var_uc_depleak_dn2 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn2 * 0.5))), (((locals.var_uc_depleak_dn4 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn4 * 0.5))), (((locals.var_uc_depleak_dn5 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn5 * 0.5))), (((locals.var_uc_depleak_dn6 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn6 * 0.5))), (((locals.var_uc_depleak_dn7 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn7 * 0.5))), (((locals.var_uc_depleak_dn8 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn8 * 0.5))), (((locals.var_uc_depleak_dn9 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn9 * 0.5))), (((locals.var_uc_depleak_dn10 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn10 * 0.5))), (((locals.var_uc_depleak_dn13 * 0.5) * assign56490_e87961) + (assign56490_e87958 * (locals.var_uc_depleak_dn13 * 0.5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign56490_e87964;
        locals.var_xmax2_dn0 = assign56490_e87964_d_n0;
        locals.var_xmax2_dn2 = assign56490_e87964_d_n2;
        locals.var_xmax2_dn4 = assign56490_e87964_d_n4;
        locals.var_xmax2_dn5 = assign56490_e87964_d_n5;
        locals.var_xmax2_dn6 = assign56490_e87964_d_n6;
        locals.var_xmax2_dn7 = assign56490_e87964_d_n7;
        locals.var_xmax2_dn8 = assign56490_e87964_d_n8;
        locals.var_xmax2_dn9 = assign56490_e87964_d_n9;
        locals.var_xmax2_dn10 = assign56490_e87964_d_n10;
        locals.var_xmax2_dn13 = assign56490_e87964_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign56500_e87977, assign56500_e87977_d_n0, assign56500_e87977_d_n2, assign56500_e87977_d_n4, assign56500_e87977_d_n5, assign56500_e87977_d_n6, assign56500_e87977_d_n7, assign56500_e87977_d_n8, assign56500_e87977_d_n9, assign56500_e87977_d_n10, assign56500_e87977_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign56500_e87977;
        locals.var_xp_dn0 = assign56500_e87977_d_n0;
        locals.var_xp_dn2 = assign56500_e87977_d_n2;
        locals.var_xp_dn4 = assign56500_e87977_d_n4;
        locals.var_xp_dn5 = assign56500_e87977_d_n5;
        locals.var_xp_dn6 = assign56500_e87977_d_n6;
        locals.var_xp_dn7 = assign56500_e87977_d_n7;
        locals.var_xp_dn8 = assign56500_e87977_d_n8;
        locals.var_xp_dn9 = assign56500_e87977_d_n9;
        locals.var_xp_dn10 = assign56500_e87977_d_n10;
        locals.var_xp_dn13 = assign56500_e87977_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign56510_e87990, assign56510_e87990_d_n0, assign56510_e87990_d_n2, assign56510_e87990_d_n4, assign56510_e87990_d_n5, assign56510_e87990_d_n6, assign56510_e87990_d_n7, assign56510_e87990_d_n8, assign56510_e87990_d_n9, assign56510_e87990_d_n10, assign56510_e87990_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign56510_e87990;
        locals.var_xmp_dn0 = assign56510_e87990_d_n0;
        locals.var_xmp_dn2 = assign56510_e87990_d_n2;
        locals.var_xmp_dn4 = assign56510_e87990_d_n4;
        locals.var_xmp_dn5 = assign56510_e87990_d_n5;
        locals.var_xmp_dn6 = assign56510_e87990_d_n6;
        locals.var_xmp_dn7 = assign56510_e87990_d_n7;
        locals.var_xmp_dn8 = assign56510_e87990_d_n8;
        locals.var_xmp_dn9 = assign56510_e87990_d_n9;
        locals.var_xmp_dn10 = assign56510_e87990_d_n10;
        locals.var_xmp_dn13 = assign56510_e87990_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign56520_e88003,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56520_e88003;
        locals.var_m0_rv = 0.0;

        let (assign56530_e88016,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56530_e88016;
        locals.var_mm_rv = 0.0;

        let (assign56540_e88029, assign56540_e88029_d_n0, assign56540_e88029_d_n2, assign56540_e88029_d_n4, assign56540_e88029_d_n5, assign56540_e88029_d_n6, assign56540_e88029_d_n7, assign56540_e88029_d_n8, assign56540_e88029_d_n9, assign56540_e88029_d_n10, assign56540_e88029_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign56540_e88029;
        locals.var_arg_dn0 = assign56540_e88029_d_n0;
        locals.var_arg_dn2 = assign56540_e88029_d_n2;
        locals.var_arg_dn4 = assign56540_e88029_d_n4;
        locals.var_arg_dn5 = assign56540_e88029_d_n5;
        locals.var_arg_dn6 = assign56540_e88029_d_n6;
        locals.var_arg_dn7 = assign56540_e88029_d_n7;
        locals.var_arg_dn8 = assign56540_e88029_d_n8;
        locals.var_arg_dn9 = assign56540_e88029_d_n9;
        locals.var_arg_dn10 = assign56540_e88029_d_n10;
        locals.var_arg_dn13 = assign56540_e88029_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign56550_e88042, assign56550_e88042_d_n0, assign56550_e88042_d_n2, assign56550_e88042_d_n4, assign56550_e88042_d_n5, assign56550_e88042_d_n6, assign56550_e88042_d_n7, assign56550_e88042_d_n8, assign56550_e88042_d_n9, assign56550_e88042_d_n10, assign56550_e88042_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign56550_e88042;
        locals.var_dnm_dn0 = assign56550_e88042_d_n0;
        locals.var_dnm_dn2 = assign56550_e88042_d_n2;
        locals.var_dnm_dn4 = assign56550_e88042_d_n4;
        locals.var_dnm_dn5 = assign56550_e88042_d_n5;
        locals.var_dnm_dn6 = assign56550_e88042_d_n6;
        locals.var_dnm_dn7 = assign56550_e88042_d_n7;
        locals.var_dnm_dn8 = assign56550_e88042_d_n8;
        locals.var_dnm_dn9 = assign56550_e88042_d_n9;
        locals.var_dnm_dn10 = assign56550_e88042_d_n10;
        locals.var_dnm_dn13 = assign56550_e88042_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign56560_e88057, assign56560_e88057_d_n0, assign56560_e88057_d_n2, assign56560_e88057_d_n4, assign56560_e88057_d_n5, assign56560_e88057_d_n6, assign56560_e88057_d_n7, assign56560_e88057_d_n8, assign56560_e88057_d_n9, assign56560_e88057_d_n10, assign56560_e88057_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        let assign56560_e88055: f64 = (locals.var_xp * locals.var_x2);
        (assign56560_e88055, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign56560_e88057;
        locals.var_xp_dn0 = assign56560_e88057_d_n0;
        locals.var_xp_dn2 = assign56560_e88057_d_n2;
        locals.var_xp_dn4 = assign56560_e88057_d_n4;
        locals.var_xp_dn5 = assign56560_e88057_d_n5;
        locals.var_xp_dn6 = assign56560_e88057_d_n6;
        locals.var_xp_dn7 = assign56560_e88057_d_n7;
        locals.var_xp_dn8 = assign56560_e88057_d_n8;
        locals.var_xp_dn9 = assign56560_e88057_d_n9;
        locals.var_xp_dn10 = assign56560_e88057_d_n10;
        locals.var_xp_dn13 = assign56560_e88057_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign56570_e88072, assign56570_e88072_d_n0, assign56570_e88072_d_n2, assign56570_e88072_d_n4, assign56570_e88072_d_n5, assign56570_e88072_d_n6, assign56570_e88072_d_n7, assign56570_e88072_d_n8, assign56570_e88072_d_n9, assign56570_e88072_d_n10, assign56570_e88072_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        let assign56570_e88070: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56570_e88070, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign56570_e88072;
        locals.var_xmp_dn0 = assign56570_e88072_d_n0;
        locals.var_xmp_dn2 = assign56570_e88072_d_n2;
        locals.var_xmp_dn4 = assign56570_e88072_d_n4;
        locals.var_xmp_dn5 = assign56570_e88072_d_n5;
        locals.var_xmp_dn6 = assign56570_e88072_d_n6;
        locals.var_xmp_dn7 = assign56570_e88072_d_n7;
        locals.var_xmp_dn8 = assign56570_e88072_d_n8;
        locals.var_xmp_dn9 = assign56570_e88072_d_n9;
        locals.var_xmp_dn10 = assign56570_e88072_d_n10;
        locals.var_xmp_dn13 = assign56570_e88072_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign56580_e88087, assign56580_e88087_d_n0, assign56580_e88087_d_n2, assign56580_e88087_d_n4, assign56580_e88087_d_n5, assign56580_e88087_d_n6, assign56580_e88087_d_n7, assign56580_e88087_d_n8, assign56580_e88087_d_n9, assign56580_e88087_d_n10, assign56580_e88087_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        let assign56580_e88085: f64 = (locals.var_xp * locals.var_x2);
        (assign56580_e88085, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign56580_e88087;
        locals.var_xp_dn0 = assign56580_e88087_d_n0;
        locals.var_xp_dn2 = assign56580_e88087_d_n2;
        locals.var_xp_dn4 = assign56580_e88087_d_n4;
        locals.var_xp_dn5 = assign56580_e88087_d_n5;
        locals.var_xp_dn6 = assign56580_e88087_d_n6;
        locals.var_xp_dn7 = assign56580_e88087_d_n7;
        locals.var_xp_dn8 = assign56580_e88087_d_n8;
        locals.var_xp_dn9 = assign56580_e88087_d_n9;
        locals.var_xp_dn10 = assign56580_e88087_d_n10;
        locals.var_xp_dn13 = assign56580_e88087_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign56590_e88102, assign56590_e88102_d_n0, assign56590_e88102_d_n2, assign56590_e88102_d_n4, assign56590_e88102_d_n5, assign56590_e88102_d_n6, assign56590_e88102_d_n7, assign56590_e88102_d_n8, assign56590_e88102_d_n9, assign56590_e88102_d_n10, assign56590_e88102_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        let assign56590_e88100: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign56590_e88100, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign56590_e88102;
        locals.var_xmp_dn0 = assign56590_e88102_d_n0;
        locals.var_xmp_dn2 = assign56590_e88102_d_n2;
        locals.var_xmp_dn4 = assign56590_e88102_d_n4;
        locals.var_xmp_dn5 = assign56590_e88102_d_n5;
        locals.var_xmp_dn6 = assign56590_e88102_d_n6;
        locals.var_xmp_dn7 = assign56590_e88102_d_n7;
        locals.var_xmp_dn8 = assign56590_e88102_d_n8;
        locals.var_xmp_dn9 = assign56590_e88102_d_n9;
        locals.var_xmp_dn10 = assign56590_e88102_d_n10;
        locals.var_xmp_dn13 = assign56590_e88102_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign56600_e88117, assign56600_e88117_d_n0, assign56600_e88117_d_n2, assign56600_e88117_d_n4, assign56600_e88117_d_n5, assign56600_e88117_d_n6, assign56600_e88117_d_n7, assign56600_e88117_d_n8, assign56600_e88117_d_n9, assign56600_e88117_d_n10, assign56600_e88117_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        let assign56600_e88115: f64 = (locals.var_xp + locals.var_xmp);
        (assign56600_e88115, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign56600_e88117;
        locals.var_arg_dn0 = assign56600_e88117_d_n0;
        locals.var_arg_dn2 = assign56600_e88117_d_n2;
        locals.var_arg_dn4 = assign56600_e88117_d_n4;
        locals.var_arg_dn5 = assign56600_e88117_d_n5;
        locals.var_arg_dn6 = assign56600_e88117_d_n6;
        locals.var_arg_dn7 = assign56600_e88117_d_n7;
        locals.var_arg_dn8 = assign56600_e88117_d_n8;
        locals.var_arg_dn9 = assign56600_e88117_d_n9;
        locals.var_arg_dn10 = assign56600_e88117_d_n10;
        locals.var_arg_dn13 = assign56600_e88117_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign56610_e88130, assign56610_e88130_d_n0, assign56610_e88130_d_n2, assign56610_e88130_d_n4, assign56610_e88130_d_n5, assign56610_e88130_d_n6, assign56610_e88130_d_n7, assign56610_e88130_d_n8, assign56610_e88130_d_n9, assign56610_e88130_d_n10, assign56610_e88130_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign56610_e88130;
        locals.var_dnm_dn0 = assign56610_e88130_d_n0;
        locals.var_dnm_dn2 = assign56610_e88130_d_n2;
        locals.var_dnm_dn4 = assign56610_e88130_d_n4;
        locals.var_dnm_dn5 = assign56610_e88130_d_n5;
        locals.var_dnm_dn6 = assign56610_e88130_d_n6;
        locals.var_dnm_dn7 = assign56610_e88130_d_n7;
        locals.var_dnm_dn8 = assign56610_e88130_d_n8;
        locals.var_dnm_dn9 = assign56610_e88130_d_n9;
        locals.var_dnm_dn10 = assign56610_e88130_d_n10;
        locals.var_dnm_dn13 = assign56610_e88130_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign56620_e88145: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1414 = assign56620_e88145;
        locals.var_guard1414_rv = 0.0;

        let assign56630_e88148: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1415 = assign56630_e88148;
        locals.var_guard1415_rv = 0.0;

        let (assign56640_e88165,) = {
    if (((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) && (locals.var_guard1414 != 0.0)) && (locals.var_guard1415 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56640_e88165;
        locals.var_mm_rv = 0.0;

        let assign56650_e88168: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1416 = assign56650_e88168;
        locals.var_guard1416_rv = 0.0;

        let (assign56660_e88188,) = {
    if ((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) && (locals.var_guard1414 != 0.0)) && (locals.var_guard1415 == 0.0)) && (locals.var_guard1416 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56660_e88188;
        locals.var_mm_rv = 0.0;

        let assign56670_e88191: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1417 = assign56670_e88191;
        locals.var_guard1417_rv = 0.0;

        let (assign56680_e88214,) = {
    if (((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) && (locals.var_guard1414 != 0.0)) && (locals.var_guard1415 == 0.0)) && (locals.var_guard1416 == 0.0)) && (locals.var_guard1417 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56680_e88214;
        locals.var_mm_rv = 0.0;

        let assign56690_e88217: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1418 = assign56690_e88217;
        locals.var_guard1418_rv = 0.0;

        let (assign56700_e88243,) = {
    if ((((((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) && (locals.var_guard1414 != 0.0)) && (locals.var_guard1415 == 0.0)) && (locals.var_guard1416 == 0.0)) && (locals.var_guard1417 == 0.0)) && (locals.var_guard1418 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign56700_e88243;
        locals.var_mm_rv = 0.0;

        let (assign56710_e88258,) = {
    if ((((locals.var_guard443 != 0.0) && ((locals.var_guard446 != 0.0) && (!((locals.var_guard444 != 0.0) || (locals.var_guard445 != 0.0))))) && (locals.var_guard1413 != 0.0)) && (locals.var_guard1414 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign56710_e88258;
        locals.var_m0_rv = 0.0;

    }
}
