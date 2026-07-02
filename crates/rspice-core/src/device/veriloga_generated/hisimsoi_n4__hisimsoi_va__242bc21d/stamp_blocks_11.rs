#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_48(
        locals: &mut StampLocals,
    ) {
        let (assign13870_e19634, assign13870_e19634_d_n0, assign13870_e19634_d_n2, assign13870_e19634_d_n6, assign13870_e19634_d_n7, assign13870_e19634_d_n10, assign13870_e19634_d_n11, assign13870_e19634_d_n12, assign13870_e19634_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13870_e19632: f64 = (1e-5 * 1e-5);
        (assign13870_e19632, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13870_e19634;
        locals.var_xmax2_dn0 = assign13870_e19634_d_n0;
        locals.var_xmax2_dn2 = assign13870_e19634_d_n2;
        locals.var_xmax2_dn6 = assign13870_e19634_d_n6;
        locals.var_xmax2_dn7 = assign13870_e19634_d_n7;
        locals.var_xmax2_dn10 = assign13870_e19634_d_n10;
        locals.var_xmax2_dn11 = assign13870_e19634_d_n11;
        locals.var_xmax2_dn12 = assign13870_e19634_d_n12;
        locals.var_xmax2_dn17 = assign13870_e19634_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign13880_e19643, assign13880_e19643_d_n0, assign13880_e19643_d_n2, assign13880_e19643_d_n6, assign13880_e19643_d_n7, assign13880_e19643_d_n10, assign13880_e19643_d_n11, assign13880_e19643_d_n12, assign13880_e19643_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13880_e19643;
        locals.var_xp_dn0 = assign13880_e19643_d_n0;
        locals.var_xp_dn2 = assign13880_e19643_d_n2;
        locals.var_xp_dn6 = assign13880_e19643_d_n6;
        locals.var_xp_dn7 = assign13880_e19643_d_n7;
        locals.var_xp_dn10 = assign13880_e19643_d_n10;
        locals.var_xp_dn11 = assign13880_e19643_d_n11;
        locals.var_xp_dn12 = assign13880_e19643_d_n12;
        locals.var_xp_dn17 = assign13880_e19643_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13890_e19652, assign13890_e19652_d_n0, assign13890_e19652_d_n2, assign13890_e19652_d_n6, assign13890_e19652_d_n7, assign13890_e19652_d_n10, assign13890_e19652_d_n11, assign13890_e19652_d_n12, assign13890_e19652_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13890_e19652;
        locals.var_xmp_dn0 = assign13890_e19652_d_n0;
        locals.var_xmp_dn2 = assign13890_e19652_d_n2;
        locals.var_xmp_dn6 = assign13890_e19652_d_n6;
        locals.var_xmp_dn7 = assign13890_e19652_d_n7;
        locals.var_xmp_dn10 = assign13890_e19652_d_n10;
        locals.var_xmp_dn11 = assign13890_e19652_d_n11;
        locals.var_xmp_dn12 = assign13890_e19652_d_n12;
        locals.var_xmp_dn17 = assign13890_e19652_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign13900_e19661,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13900_e19661;
        locals.var_m0_rv = 0.0;

        let (assign13910_e19670,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13910_e19670;
        locals.var_mm_rv = 0.0;

        let (assign13920_e19679, assign13920_e19679_d_n0, assign13920_e19679_d_n2, assign13920_e19679_d_n6, assign13920_e19679_d_n7, assign13920_e19679_d_n10, assign13920_e19679_d_n11, assign13920_e19679_d_n12, assign13920_e19679_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13920_e19679;
        locals.var_arg_dn0 = assign13920_e19679_d_n0;
        locals.var_arg_dn2 = assign13920_e19679_d_n2;
        locals.var_arg_dn6 = assign13920_e19679_d_n6;
        locals.var_arg_dn7 = assign13920_e19679_d_n7;
        locals.var_arg_dn10 = assign13920_e19679_d_n10;
        locals.var_arg_dn11 = assign13920_e19679_d_n11;
        locals.var_arg_dn12 = assign13920_e19679_d_n12;
        locals.var_arg_dn17 = assign13920_e19679_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign13930_e19688, assign13930_e19688_d_n0, assign13930_e19688_d_n2, assign13930_e19688_d_n6, assign13930_e19688_d_n7, assign13930_e19688_d_n10, assign13930_e19688_d_n11, assign13930_e19688_d_n12, assign13930_e19688_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13930_e19688;
        locals.var_dnm_dn0 = assign13930_e19688_d_n0;
        locals.var_dnm_dn2 = assign13930_e19688_d_n2;
        locals.var_dnm_dn6 = assign13930_e19688_d_n6;
        locals.var_dnm_dn7 = assign13930_e19688_d_n7;
        locals.var_dnm_dn10 = assign13930_e19688_d_n10;
        locals.var_dnm_dn11 = assign13930_e19688_d_n11;
        locals.var_dnm_dn12 = assign13930_e19688_d_n12;
        locals.var_dnm_dn17 = assign13930_e19688_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign13940_e19699, assign13940_e19699_d_n0, assign13940_e19699_d_n2, assign13940_e19699_d_n6, assign13940_e19699_d_n7, assign13940_e19699_d_n10, assign13940_e19699_d_n11, assign13940_e19699_d_n12, assign13940_e19699_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13940_e19697: f64 = (locals.var_xp * locals.var_x2);
        (assign13940_e19697, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13940_e19699;
        locals.var_xp_dn0 = assign13940_e19699_d_n0;
        locals.var_xp_dn2 = assign13940_e19699_d_n2;
        locals.var_xp_dn6 = assign13940_e19699_d_n6;
        locals.var_xp_dn7 = assign13940_e19699_d_n7;
        locals.var_xp_dn10 = assign13940_e19699_d_n10;
        locals.var_xp_dn11 = assign13940_e19699_d_n11;
        locals.var_xp_dn12 = assign13940_e19699_d_n12;
        locals.var_xp_dn17 = assign13940_e19699_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13950_e19710, assign13950_e19710_d_n0, assign13950_e19710_d_n2, assign13950_e19710_d_n6, assign13950_e19710_d_n7, assign13950_e19710_d_n10, assign13950_e19710_d_n11, assign13950_e19710_d_n12, assign13950_e19710_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13950_e19708: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13950_e19708, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13950_e19710;
        locals.var_xmp_dn0 = assign13950_e19710_d_n0;
        locals.var_xmp_dn2 = assign13950_e19710_d_n2;
        locals.var_xmp_dn6 = assign13950_e19710_d_n6;
        locals.var_xmp_dn7 = assign13950_e19710_d_n7;
        locals.var_xmp_dn10 = assign13950_e19710_d_n10;
        locals.var_xmp_dn11 = assign13950_e19710_d_n11;
        locals.var_xmp_dn12 = assign13950_e19710_d_n12;
        locals.var_xmp_dn17 = assign13950_e19710_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign13960_e19721, assign13960_e19721_d_n0, assign13960_e19721_d_n2, assign13960_e19721_d_n6, assign13960_e19721_d_n7, assign13960_e19721_d_n10, assign13960_e19721_d_n11, assign13960_e19721_d_n12, assign13960_e19721_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13960_e19719: f64 = (locals.var_xp * locals.var_x2);
        (assign13960_e19719, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13960_e19721;
        locals.var_xp_dn0 = assign13960_e19721_d_n0;
        locals.var_xp_dn2 = assign13960_e19721_d_n2;
        locals.var_xp_dn6 = assign13960_e19721_d_n6;
        locals.var_xp_dn7 = assign13960_e19721_d_n7;
        locals.var_xp_dn10 = assign13960_e19721_d_n10;
        locals.var_xp_dn11 = assign13960_e19721_d_n11;
        locals.var_xp_dn12 = assign13960_e19721_d_n12;
        locals.var_xp_dn17 = assign13960_e19721_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13970_e19732, assign13970_e19732_d_n0, assign13970_e19732_d_n2, assign13970_e19732_d_n6, assign13970_e19732_d_n7, assign13970_e19732_d_n10, assign13970_e19732_d_n11, assign13970_e19732_d_n12, assign13970_e19732_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13970_e19730: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13970_e19730, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13970_e19732;
        locals.var_xmp_dn0 = assign13970_e19732_d_n0;
        locals.var_xmp_dn2 = assign13970_e19732_d_n2;
        locals.var_xmp_dn6 = assign13970_e19732_d_n6;
        locals.var_xmp_dn7 = assign13970_e19732_d_n7;
        locals.var_xmp_dn10 = assign13970_e19732_d_n10;
        locals.var_xmp_dn11 = assign13970_e19732_d_n11;
        locals.var_xmp_dn12 = assign13970_e19732_d_n12;
        locals.var_xmp_dn17 = assign13970_e19732_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign13980_e19743, assign13980_e19743_d_n0, assign13980_e19743_d_n2, assign13980_e19743_d_n6, assign13980_e19743_d_n7, assign13980_e19743_d_n10, assign13980_e19743_d_n11, assign13980_e19743_d_n12, assign13980_e19743_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign13980_e19741: f64 = (locals.var_xp + locals.var_xmp);
        (assign13980_e19741, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13980_e19743;
        locals.var_arg_dn0 = assign13980_e19743_d_n0;
        locals.var_arg_dn2 = assign13980_e19743_d_n2;
        locals.var_arg_dn6 = assign13980_e19743_d_n6;
        locals.var_arg_dn7 = assign13980_e19743_d_n7;
        locals.var_arg_dn10 = assign13980_e19743_d_n10;
        locals.var_arg_dn11 = assign13980_e19743_d_n11;
        locals.var_arg_dn12 = assign13980_e19743_d_n12;
        locals.var_arg_dn17 = assign13980_e19743_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign13990_e19752, assign13990_e19752_d_n0, assign13990_e19752_d_n2, assign13990_e19752_d_n6, assign13990_e19752_d_n7, assign13990_e19752_d_n10, assign13990_e19752_d_n11, assign13990_e19752_d_n12, assign13990_e19752_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13990_e19752;
        locals.var_dnm_dn0 = assign13990_e19752_d_n0;
        locals.var_dnm_dn2 = assign13990_e19752_d_n2;
        locals.var_dnm_dn6 = assign13990_e19752_d_n6;
        locals.var_dnm_dn7 = assign13990_e19752_d_n7;
        locals.var_dnm_dn10 = assign13990_e19752_d_n10;
        locals.var_dnm_dn11 = assign13990_e19752_d_n11;
        locals.var_dnm_dn12 = assign13990_e19752_d_n12;
        locals.var_dnm_dn17 = assign13990_e19752_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign14000_e19767: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard428 = assign14000_e19767;
        locals.var_guard428_rv = 0.0;

        let assign14010_e19770: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign14010_e19770;
        locals.var_guard429_rv = 0.0;

        let (assign14020_e19783,) = {
    if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_guard429 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14020_e19783;
        locals.var_mm_rv = 0.0;

        let assign14030_e19786: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign14030_e19786;
        locals.var_guard430_rv = 0.0;

        let (assign14040_e19802,) = {
    if ((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14040_e19802;
        locals.var_mm_rv = 0.0;

        let assign14050_e19805: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign14050_e19805;
        locals.var_guard431_rv = 0.0;

        let (assign14060_e19824,) = {
    if (((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14060_e19824;
        locals.var_mm_rv = 0.0;

        let assign14070_e19827: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign14070_e19827;
        locals.var_guard432_rv = 0.0;

        let (assign14080_e19849,) = {
    if ((((((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14080_e19849;
        locals.var_mm_rv = 0.0;

        let (assign14090_e19860,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign14090_e19860;
        locals.var_m0_rv = 0.0;

        let mut assign14100_loop_guard: usize = 0;
        while {
            let assign14100_cond_e19872: f64 = if (((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign14100_cond_e19872 != 0.0
        } {
            assign14100_loop_guard += 1;
            assert!(assign14100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign14100_body0_e19884, assign14100_body0_e19884_d_n0, assign14100_body0_e19884_d_n2, assign14100_body0_e19884_d_n6, assign14100_body0_e19884_d_n7, assign14100_body0_e19884_d_n10, assign14100_body0_e19884_d_n11, assign14100_body0_e19884_d_n12, assign14100_body0_e19884_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) {
        let assign14100_body0_e19882: f64 = (locals.var_dnm).sqrt();
        (assign14100_body0_e19882, (locals.var_dnm_dn0 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn2 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn6 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn7 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn10 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn11 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn12 / (2.0 * assign14100_body0_e19882)), (locals.var_dnm_dn17 / (2.0 * assign14100_body0_e19882)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign14100_body0_e19884;
            locals.var_dnm_dn0 = assign14100_body0_e19884_d_n0;
            locals.var_dnm_dn2 = assign14100_body0_e19884_d_n2;
            locals.var_dnm_dn6 = assign14100_body0_e19884_d_n6;
            locals.var_dnm_dn7 = assign14100_body0_e19884_d_n7;
            locals.var_dnm_dn10 = assign14100_body0_e19884_d_n10;
            locals.var_dnm_dn11 = assign14100_body0_e19884_d_n11;
            locals.var_dnm_dn12 = assign14100_body0_e19884_d_n12;
            locals.var_dnm_dn17 = assign14100_body0_e19884_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign14100_body1_e19897,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 != 0.0)) {
        let assign14100_body1_e19895: f64 = (locals.var_m0 + 1.0);
        (assign14100_body1_e19895,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign14100_body1_e19897;
            locals.var_m0_rv = 0.0;
        }

        let (assign14110_e19915, assign14110_e19915_d_n0, assign14110_e19915_d_n2, assign14110_e19915_d_n6, assign14110_e19915_d_n7, assign14110_e19915_d_n10, assign14110_e19915_d_n11, assign14110_e19915_d_n12, assign14110_e19915_d_n17,) = {
    if ((((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) && (locals.var_guard428 == 0.0)) {
        let assign14110_e19911: f64 = (2.0 * 2.0);
        let assign14110_e19912: f64 = (1.0 / assign14110_e19911);
        let assign14110_e19913: f64 = (locals.var_dnm).powf(assign14110_e19912);
        (assign14110_e19913, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn0)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn2)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn6)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn7)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn10)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn11)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn12)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14110_e19912) as f64).is_finite() && ((assign14110_e19912) as f64).fract() == 0.0 { if assign14110_e19912 == 0.0 { 0.0 } else { (assign14110_e19912 * ((locals.var_dnm).powf(assign14110_e19912 - 1.0) * locals.var_dnm_dn17)) } } else { (assign14110_e19913 * (assign14110_e19912 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14110_e19915;
        locals.var_dnm_dn0 = assign14110_e19915_d_n0;
        locals.var_dnm_dn2 = assign14110_e19915_d_n2;
        locals.var_dnm_dn6 = assign14110_e19915_d_n6;
        locals.var_dnm_dn7 = assign14110_e19915_d_n7;
        locals.var_dnm_dn10 = assign14110_e19915_d_n10;
        locals.var_dnm_dn11 = assign14110_e19915_d_n11;
        locals.var_dnm_dn12 = assign14110_e19915_d_n12;
        locals.var_dnm_dn17 = assign14110_e19915_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign14120_e19926, assign14120_e19926_d_n0, assign14120_e19926_d_n2, assign14120_e19926_d_n6, assign14120_e19926_d_n7, assign14120_e19926_d_n10, assign14120_e19926_d_n11, assign14120_e19926_d_n12, assign14120_e19926_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign14120_e19924: f64 = (1.0 / locals.var_dnm);
        (assign14120_e19924, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14120_e19926;
        locals.var_dnm_dn0 = assign14120_e19926_d_n0;
        locals.var_dnm_dn2 = assign14120_e19926_d_n2;
        locals.var_dnm_dn6 = assign14120_e19926_d_n6;
        locals.var_dnm_dn7 = assign14120_e19926_d_n7;
        locals.var_dnm_dn10 = assign14120_e19926_d_n10;
        locals.var_dnm_dn11 = assign14120_e19926_d_n11;
        locals.var_dnm_dn12 = assign14120_e19926_d_n12;
        locals.var_dnm_dn17 = assign14120_e19926_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign14130_e19939, assign14130_e19939_d_n0, assign14130_e19939_d_n2, assign14130_e19939_d_n6, assign14130_e19939_d_n7, assign14130_e19939_d_n10, assign14130_e19939_d_n11, assign14130_e19939_d_n12, assign14130_e19939_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign14130_e19935: f64 = (locals.var_tmf1 * 1e-5);
        let assign14130_e19937: f64 = (assign14130_e19935 * locals.var_dnm);
        (assign14130_e19937, (((locals.var_tmf1_dn0 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 1e-5) * locals.var_dnm) + (assign14130_e19935 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign14130_e19939;
        locals.var_tmf0_dn0 = assign14130_e19939_d_n0;
        locals.var_tmf0_dn2 = assign14130_e19939_d_n2;
        locals.var_tmf0_dn6 = assign14130_e19939_d_n6;
        locals.var_tmf0_dn7 = assign14130_e19939_d_n7;
        locals.var_tmf0_dn10 = assign14130_e19939_d_n10;
        locals.var_tmf0_dn11 = assign14130_e19939_d_n11;
        locals.var_tmf0_dn12 = assign14130_e19939_d_n12;
        locals.var_tmf0_dn17 = assign14130_e19939_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign14140_e19952, assign14140_e19952_d_n0, assign14140_e19952_d_n2, assign14140_e19952_d_n6, assign14140_e19952_d_n7, assign14140_e19952_d_n10, assign14140_e19952_d_n11, assign14140_e19952_d_n12, assign14140_e19952_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 != 0.0)) {
        let assign14140_e19948: f64 = 1e-5;
        let assign14140_e19950: f64 = (assign14140_e19948 - locals.var_tmf0);
        (assign14140_e19950, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14140_e19952;
        locals.var_rrr_eta_dn0 = assign14140_e19952_d_n0;
        locals.var_rrr_eta_dn2 = assign14140_e19952_d_n2;
        locals.var_rrr_eta_dn6 = assign14140_e19952_d_n6;
        locals.var_rrr_eta_dn7 = assign14140_e19952_d_n7;
        locals.var_rrr_eta_dn10 = assign14140_e19952_d_n10;
        locals.var_rrr_eta_dn11 = assign14140_e19952_d_n11;
        locals.var_rrr_eta_dn12 = assign14140_e19952_d_n12;
        locals.var_rrr_eta_dn17 = assign14140_e19952_d_n17;
        locals.var_rrr_eta_rv = 0.0;

        let (assign14150_e19962, assign14150_e19962_d_n0, assign14150_e19962_d_n2, assign14150_e19962_d_n6, assign14150_e19962_d_n7, assign14150_e19962_d_n10, assign14150_e19962_d_n11, assign14150_e19962_d_n12, assign14150_e19962_d_n17,) = {
    if (((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard427 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14150_e19962;
        locals.var_rrr_eta_dn0 = assign14150_e19962_d_n0;
        locals.var_rrr_eta_dn2 = assign14150_e19962_d_n2;
        locals.var_rrr_eta_dn6 = assign14150_e19962_d_n6;
        locals.var_rrr_eta_dn7 = assign14150_e19962_d_n7;
        locals.var_rrr_eta_dn10 = assign14150_e19962_d_n10;
        locals.var_rrr_eta_dn11 = assign14150_e19962_d_n11;
        locals.var_rrr_eta_dn12 = assign14150_e19962_d_n12;
        locals.var_rrr_eta_dn17 = assign14150_e19962_d_n17;
        locals.var_rrr_eta_rv = 0.0;

        let (assign14160_e19969, assign14160_e19969_d_n0, assign14160_e19969_d_n2, assign14160_e19969_d_n6, assign14160_e19969_d_n7, assign14160_e19969_d_n10, assign14160_e19969_d_n11, assign14160_e19969_d_n12, assign14160_e19969_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign14160_e19969;
        locals.var_alpha_dn0 = assign14160_e19969_d_n0;
        locals.var_alpha_dn2 = assign14160_e19969_d_n2;
        locals.var_alpha_dn6 = assign14160_e19969_d_n6;
        locals.var_alpha_dn7 = assign14160_e19969_d_n7;
        locals.var_alpha_dn10 = assign14160_e19969_d_n10;
        locals.var_alpha_dn11 = assign14160_e19969_d_n11;
        locals.var_alpha_dn12 = assign14160_e19969_d_n12;
        locals.var_alpha_dn17 = assign14160_e19969_d_n17;
        locals.var_alpha_rv = 0.0;

        let (assign14170_e19982, assign14170_e19982_d_n0, assign14170_e19982_d_n2, assign14170_e19982_d_n6, assign14170_e19982_d_n7, assign14170_e19982_d_n10, assign14170_e19982_d_n11, assign14170_e19982_d_n12, assign14170_e19982_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign14170_e19978: f64 = (1.0 + locals.var_alpha);
        let assign14170_e19979: f64 = (locals.var_alpha * assign14170_e19978);
        let assign14170_e19980: f64 = (1.0 + assign14170_e19979);
        (assign14170_e19980, ((locals.var_alpha_dn0 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign14170_e19978) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign14170_e19982;
        locals.var_qinm_dn0 = assign14170_e19982_d_n0;
        locals.var_qinm_dn2 = assign14170_e19982_d_n2;
        locals.var_qinm_dn6 = assign14170_e19982_d_n6;
        locals.var_qinm_dn7 = assign14170_e19982_d_n7;
        locals.var_qinm_dn10 = assign14170_e19982_d_n10;
        locals.var_qinm_dn11 = assign14170_e19982_d_n11;
        locals.var_qinm_dn12 = assign14170_e19982_d_n12;
        locals.var_qinm_dn17 = assign14170_e19982_d_n17;
        locals.var_qinm_rv = 0.0;

        let (assign14180_e20002, assign14180_e20002_d_n0, assign14180_e20002_d_n2, assign14180_e20002_d_n6, assign14180_e20002_d_n7, assign14180_e20002_d_n10, assign14180_e20002_d_n11, assign14180_e20002_d_n12, assign14180_e20002_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign14180_e19989: f64 = (1.0 + locals.var_alpha);
        let assign14180_e19992: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14180_e20000, assign14180_e20000_d_n0, assign14180_e20000_d_n2, assign14180_e20000_d_n6, assign14180_e20000_d_n7, assign14180_e20000_d_n10, assign14180_e20000_d_n11, assign14180_e20000_d_n12, assign14180_e20000_d_n17,) = {
            if (assign14180_e19989 >= assign14180_e19992) {
                let assign14180_e19996: f64 = (1.0 + locals.var_alpha);
                (assign14180_e19996, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign14180_e19999: f64 = (10.0 * 2.220446049250313e-16);
                (assign14180_e19999, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14180_e20000, assign14180_e20000_d_n0, assign14180_e20000_d_n2, assign14180_e20000_d_n6, assign14180_e20000_d_n7, assign14180_e20000_d_n10, assign14180_e20000_d_n11, assign14180_e20000_d_n12, assign14180_e20000_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign14180_e20002;
        locals.var_qidn_dn0 = assign14180_e20002_d_n0;
        locals.var_qidn_dn2 = assign14180_e20002_d_n2;
        locals.var_qidn_dn6 = assign14180_e20002_d_n6;
        locals.var_qidn_dn7 = assign14180_e20002_d_n7;
        locals.var_qidn_dn10 = assign14180_e20002_d_n10;
        locals.var_qidn_dn11 = assign14180_e20002_d_n11;
        locals.var_qidn_dn12 = assign14180_e20002_d_n12;
        locals.var_qidn_dn17 = assign14180_e20002_d_n17;
        locals.var_qidn_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14190_e20014, assign14190_e20014_d_n0, assign14190_e20014_d_n2, assign14190_e20014_d_n6, assign14190_e20014_d_n7, assign14190_e20014_d_n10, assign14190_e20014_d_n11, assign14190_e20014_d_n12, assign14190_e20014_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard301 == 0.0)) {
        let assign14190_e20008: f64 = (-0.5);
        let assign14190_e20011: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign14190_e20012: f64 = (assign14190_e20008 * assign14190_e20011);
        (assign14190_e20012, (assign14190_e20008 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign14190_e20008 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign14190_e20008 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign14190_e20008 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign14190_e20008 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign14190_e20008 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign14190_e20008 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign14190_e20008 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign14190_e20014;
        locals.var_qiu_dn0 = assign14190_e20014_d_n0;
        locals.var_qiu_dn2 = assign14190_e20014_d_n2;
        locals.var_qiu_dn6 = assign14190_e20014_d_n6;
        locals.var_qiu_dn7 = assign14190_e20014_d_n7;
        locals.var_qiu_dn10 = assign14190_e20014_d_n10;
        locals.var_qiu_dn11 = assign14190_e20014_d_n11;
        locals.var_qiu_dn12 = assign14190_e20014_d_n12;
        locals.var_qiu_dn17 = assign14190_e20014_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign14260_e20047, assign14260_e20047_d_n0, assign14260_e20047_d_n2, assign14260_e20047_d_n6, assign14260_e20047_d_n7, assign14260_e20047_d_n10, assign14260_e20047_d_n11, assign14260_e20047_d_n12, assign14260_e20047_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    } else {
        (locals.var_vbcs_cl, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    }
};
        locals.var_vbcs_cl = assign14260_e20047;
        locals.var_vbcs_cl_dn0 = assign14260_e20047_d_n0;
        locals.var_vbcs_cl_dn2 = assign14260_e20047_d_n2;
        locals.var_vbcs_cl_dn6 = assign14260_e20047_d_n6;
        locals.var_vbcs_cl_dn7 = assign14260_e20047_d_n7;
        locals.var_vbcs_cl_dn10 = assign14260_e20047_d_n10;
        locals.var_vbcs_cl_dn11 = assign14260_e20047_d_n11;
        locals.var_vbcs_cl_dn12 = assign14260_e20047_d_n12;
        locals.var_vbcs_cl_dn17 = assign14260_e20047_d_n17;
        locals.var_vbcs_cl_rv = 0.0;

        let assign14270_e20050: f64 = if locals.var_wdsoi_ini < p.p237 { 1.0 } else { 0.0 };
        locals.var_guard439 = assign14270_e20050;
        locals.var_guard439_rv = 0.0;

        let (assign14280_e20057,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard439 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14280_e20057;
        locals.var_flg_depmode_rv = 0.0;

        let (assign14290_e20065,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard439 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14290_e20065;
        locals.var_flg_depmode_rv = 0.0;

        let (assign14300_e20076, assign14300_e20076_d_n0, assign14300_e20076_d_n2, assign14300_e20076_d_n6, assign14300_e20076_d_n7, assign14300_e20076_d_n10, assign14300_e20076_d_n11, assign14300_e20076_d_n12, assign14300_e20076_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14300_e20070: f64 = (locals.var_vfb - locals.var_dvth);
        let assign14300_e20072: f64 = (assign14300_e20070 + locals.var_dppg);
        let assign14300_e20074: f64 = (assign14300_e20072 + locals.var_vbcs_cl);
        (assign14300_e20074, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbcs_cl_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbcs_cl_dn2), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbcs_cl_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbcs_cl_dn7), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbcs_cl_dn10), (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbcs_cl_dn11), (((-locals.var_dvth_dn12) + locals.var_dppg_dn12) + locals.var_vbcs_cl_dn12), (((-locals.var_dvth_dn17) + locals.var_dppg_dn17) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12, locals.var_vgs_fb_dn17,)
    }
};
        locals.var_vgs_fb = assign14300_e20076;
        locals.var_vgs_fb_dn0 = assign14300_e20076_d_n0;
        locals.var_vgs_fb_dn2 = assign14300_e20076_d_n2;
        locals.var_vgs_fb_dn6 = assign14300_e20076_d_n6;
        locals.var_vgs_fb_dn7 = assign14300_e20076_d_n7;
        locals.var_vgs_fb_dn10 = assign14300_e20076_d_n10;
        locals.var_vgs_fb_dn11 = assign14300_e20076_d_n11;
        locals.var_vgs_fb_dn12 = assign14300_e20076_d_n12;
        locals.var_vgs_fb_dn17 = assign14300_e20076_d_n17;
        locals.var_vgs_fb_rv = 0.0;

        let assign14310_e20079: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard440 = assign14310_e20079;
        locals.var_guard440_rv = 0.0;

        let (assign14320_e20087,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14320_e20085: f64 = (-1.0);
        (assign14320_e20085,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign14320_e20087;
        locals.var_flg_zone_rv = 0.0;

        let (assign14330_e20102, assign14330_e20102_d_n0, assign14330_e20102_d_n2, assign14330_e20102_d_n6, assign14330_e20102_d_n7, assign14330_e20102_d_n10, assign14330_e20102_d_n11, assign14330_e20102_d_n12, assign14330_e20102_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14330_e20094: f64 = (2.0 * locals.var_beta_inv);
        let assign14330_e20096: f64 = (-locals.var_vgs_min);
        let assign14330_e20098: f64 = (assign14330_e20096 / locals.var_fac1);
        let assign14330_e20099: f64 = (assign14330_e20098).ln();
        let assign14330_e20100: f64 = (assign14330_e20094 * assign14330_e20099);
        (assign14330_e20100, (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (((2.0 * locals.var_beta_inv_dn10) * assign14330_e20099) + (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098))), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)), (assign14330_e20094 * ((-((assign14330_e20096 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign14330_e20098)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign14330_e20102;
        locals.var_ps0_min_dn0 = assign14330_e20102_d_n0;
        locals.var_ps0_min_dn2 = assign14330_e20102_d_n2;
        locals.var_ps0_min_dn6 = assign14330_e20102_d_n6;
        locals.var_ps0_min_dn7 = assign14330_e20102_d_n7;
        locals.var_ps0_min_dn10 = assign14330_e20102_d_n10;
        locals.var_ps0_min_dn11 = assign14330_e20102_d_n11;
        locals.var_ps0_min_dn12 = assign14330_e20102_d_n12;
        locals.var_ps0_min_dn17 = assign14330_e20102_d_n17;
        locals.var_ps0_min_rv = 0.0;

        let (assign14340_e20113, assign14340_e20113_d_n0, assign14340_e20113_d_n2, assign14340_e20113_d_n6, assign14340_e20113_d_n7, assign14340_e20113_d_n10, assign14340_e20113_d_n11, assign14340_e20113_d_n12, assign14340_e20113_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14340_e20110: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14340_e20111: f64 = (locals.var_beta * assign14340_e20110);
        (assign14340_e20111, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14340_e20110) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14340_e20113;
        locals.var_tx_dn0 = assign14340_e20113_d_n0;
        locals.var_tx_dn2 = assign14340_e20113_d_n2;
        locals.var_tx_dn6 = assign14340_e20113_d_n6;
        locals.var_tx_dn7 = assign14340_e20113_d_n7;
        locals.var_tx_dn10 = assign14340_e20113_d_n10;
        locals.var_tx_dn11 = assign14340_e20113_d_n11;
        locals.var_tx_dn12 = assign14340_e20113_d_n12;
        locals.var_tx_dn17 = assign14340_e20113_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14350_e20124, assign14350_e20124_d_n0, assign14350_e20124_d_n2, assign14350_e20124_d_n6, assign14350_e20124_d_n7, assign14350_e20124_d_n10, assign14350_e20124_d_n11, assign14350_e20124_d_n12, assign14350_e20124_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14350_e20121: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign14350_e20122: f64 = (1.0 / assign14350_e20121);
        (assign14350_e20122, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign14350_e20121 * assign14350_e20121))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign14350_e20121 * assign14350_e20121))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign14350_e20121 * assign14350_e20121))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14350_e20124;
        locals.var_t1_dn0 = assign14350_e20124_d_n0;
        locals.var_t1_dn2 = assign14350_e20124_d_n2;
        locals.var_t1_dn6 = assign14350_e20124_d_n6;
        locals.var_t1_dn7 = assign14350_e20124_d_n7;
        locals.var_t1_dn10 = assign14350_e20124_d_n10;
        locals.var_t1_dn11 = assign14350_e20124_d_n11;
        locals.var_t1_dn12 = assign14350_e20124_d_n12;
        locals.var_t1_dn17 = assign14350_e20124_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14360_e20133, assign14360_e20133_d_n0, assign14360_e20133_d_n2, assign14360_e20133_d_n6, assign14360_e20133_d_n7, assign14360_e20133_d_n10, assign14360_e20133_d_n11, assign14360_e20133_d_n12, assign14360_e20133_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14360_e20131: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign14360_e20131, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14360_e20133;
        locals.var_ty_dn0 = assign14360_e20133_d_n0;
        locals.var_ty_dn2 = assign14360_e20133_d_n2;
        locals.var_ty_dn6 = assign14360_e20133_d_n6;
        locals.var_ty_dn7 = assign14360_e20133_d_n7;
        locals.var_ty_dn10 = assign14360_e20133_d_n10;
        locals.var_ty_dn11 = assign14360_e20133_d_n11;
        locals.var_ty_dn12 = assign14360_e20133_d_n12;
        locals.var_ty_dn17 = assign14360_e20133_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign14370_e20146, assign14370_e20146_d_n0, assign14370_e20146_d_n2, assign14370_e20146_d_n6, assign14370_e20146_d_n7, assign14370_e20146_d_n10, assign14370_e20146_d_n11, assign14370_e20146_d_n12, assign14370_e20146_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14370_e20141: f64 = (3.0 * 1.414213562373095);
        let assign14370_e20143: f64 = (assign14370_e20141 * locals.var_ty);
        let assign14370_e20144: f64 = (2.0 + assign14370_e20143);
        (assign14370_e20144, (assign14370_e20141 * locals.var_ty_dn0), (assign14370_e20141 * locals.var_ty_dn2), (assign14370_e20141 * locals.var_ty_dn6), (assign14370_e20141 * locals.var_ty_dn7), (assign14370_e20141 * locals.var_ty_dn10), (assign14370_e20141 * locals.var_ty_dn11), (assign14370_e20141 * locals.var_ty_dn12), (assign14370_e20141 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign14370_e20146;
        locals.var_ac41_dn0 = assign14370_e20146_d_n0;
        locals.var_ac41_dn2 = assign14370_e20146_d_n2;
        locals.var_ac41_dn6 = assign14370_e20146_d_n6;
        locals.var_ac41_dn7 = assign14370_e20146_d_n7;
        locals.var_ac41_dn10 = assign14370_e20146_d_n10;
        locals.var_ac41_dn11 = assign14370_e20146_d_n11;
        locals.var_ac41_dn12 = assign14370_e20146_d_n12;
        locals.var_ac41_dn17 = assign14370_e20146_d_n17;
        locals.var_ac41_rv = 0.0;

        let (assign14380_e20159, assign14380_e20159_d_n0, assign14380_e20159_d_n2, assign14380_e20159_d_n6, assign14380_e20159_d_n7, assign14380_e20159_d_n10, assign14380_e20159_d_n11, assign14380_e20159_d_n12, assign14380_e20159_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14380_e20153: f64 = (8.0 * locals.var_ac41);
        let assign14380_e20155: f64 = (assign14380_e20153 * locals.var_ac41);
        let assign14380_e20157: f64 = (assign14380_e20155 * locals.var_ac41);
        (assign14380_e20157, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign14380_e20153 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign14380_e20155 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign14380_e20159;
        locals.var_ac4_dn0 = assign14380_e20159_d_n0;
        locals.var_ac4_dn2 = assign14380_e20159_d_n2;
        locals.var_ac4_dn6 = assign14380_e20159_d_n6;
        locals.var_ac4_dn7 = assign14380_e20159_d_n7;
        locals.var_ac4_dn10 = assign14380_e20159_d_n10;
        locals.var_ac4_dn11 = assign14380_e20159_d_n11;
        locals.var_ac4_dn12 = assign14380_e20159_d_n12;
        locals.var_ac4_dn17 = assign14380_e20159_d_n17;
        locals.var_ac4_rv = 0.0;

        let (assign14390_e20168, assign14390_e20168_d_n0, assign14390_e20168_d_n2, assign14390_e20168_d_n6, assign14390_e20168_d_n7, assign14390_e20168_d_n10, assign14390_e20168_d_n11, assign14390_e20168_d_n12, assign14390_e20168_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14390_e20166: f64 = (locals.var_tx - 2.0);
        (assign14390_e20166, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14390_e20168;
        locals.var_t4_dn0 = assign14390_e20168_d_n0;
        locals.var_t4_dn2 = assign14390_e20168_d_n2;
        locals.var_t4_dn6 = assign14390_e20168_d_n6;
        locals.var_t4_dn7 = assign14390_e20168_d_n7;
        locals.var_t4_dn10 = assign14390_e20168_d_n10;
        locals.var_t4_dn11 = assign14390_e20168_d_n11;
        locals.var_t4_dn12 = assign14390_e20168_d_n12;
        locals.var_t4_dn17 = assign14390_e20168_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14400_e20179, assign14400_e20179_d_n0, assign14400_e20179_d_n2, assign14400_e20179_d_n6, assign14400_e20179_d_n7, assign14400_e20179_d_n10, assign14400_e20179_d_n11, assign14400_e20179_d_n12, assign14400_e20179_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14400_e20175: f64 = (9.0 * locals.var_ty);
        let assign14400_e20177: f64 = (assign14400_e20175 * locals.var_t4);
        (assign14400_e20177, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign14400_e20175 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14400_e20179;
        locals.var_t5_dn0 = assign14400_e20179_d_n0;
        locals.var_t5_dn2 = assign14400_e20179_d_n2;
        locals.var_t5_dn6 = assign14400_e20179_d_n6;
        locals.var_t5_dn7 = assign14400_e20179_d_n7;
        locals.var_t5_dn10 = assign14400_e20179_d_n10;
        locals.var_t5_dn11 = assign14400_e20179_d_n11;
        locals.var_t5_dn12 = assign14400_e20179_d_n12;
        locals.var_t5_dn17 = assign14400_e20179_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign14410_e20190, assign14410_e20190_d_n0, assign14410_e20190_d_n2, assign14410_e20190_d_n6, assign14410_e20190_d_n7, assign14410_e20190_d_n10, assign14410_e20190_d_n11, assign14410_e20190_d_n12, assign14410_e20190_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14410_e20186: f64 = (7.0 * 1.414213562373095);
        let assign14410_e20188: f64 = (assign14410_e20186 - locals.var_t5);
        (assign14410_e20188, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign14410_e20190;
        locals.var_ac31_dn0 = assign14410_e20190_d_n0;
        locals.var_ac31_dn2 = assign14410_e20190_d_n2;
        locals.var_ac31_dn6 = assign14410_e20190_d_n6;
        locals.var_ac31_dn7 = assign14410_e20190_d_n7;
        locals.var_ac31_dn10 = assign14410_e20190_d_n10;
        locals.var_ac31_dn11 = assign14410_e20190_d_n11;
        locals.var_ac31_dn12 = assign14410_e20190_d_n12;
        locals.var_ac31_dn17 = assign14410_e20190_d_n17;
        locals.var_ac31_rv = 0.0;

        let (assign14420_e20199, assign14420_e20199_d_n0, assign14420_e20199_d_n2, assign14420_e20199_d_n6, assign14420_e20199_d_n7, assign14420_e20199_d_n10, assign14420_e20199_d_n11, assign14420_e20199_d_n12, assign14420_e20199_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14420_e20197: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign14420_e20197, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign14420_e20199;
        locals.var_ac3_dn0 = assign14420_e20199_d_n0;
        locals.var_ac3_dn2 = assign14420_e20199_d_n2;
        locals.var_ac3_dn6 = assign14420_e20199_d_n6;
        locals.var_ac3_dn7 = assign14420_e20199_d_n7;
        locals.var_ac3_dn10 = assign14420_e20199_d_n10;
        locals.var_ac3_dn11 = assign14420_e20199_d_n11;
        locals.var_ac3_dn12 = assign14420_e20199_d_n12;
        locals.var_ac3_dn17 = assign14420_e20199_d_n17;
        locals.var_ac3_rv = 0.0;

        let assign14430_e20203: f64 = (locals.var_ac3 * 1e-8);
        let assign14430_e20204: f64 = if locals.var_ac4 < assign14430_e20203 { 1.0 } else { 0.0 };
        locals.var_guard441 = assign14430_e20204;
        locals.var_guard441_rv = 0.0;

        let (assign14440_e20226, assign14440_e20226_d_n0, assign14440_e20226_d_n2, assign14440_e20226_d_n6, assign14440_e20226_d_n7, assign14440_e20226_d_n10, assign14440_e20226_d_n11, assign14440_e20226_d_n12, assign14440_e20226_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) && (locals.var_guard441 != 0.0)) {
        let assign14440_e20212: f64 = (-7.0);
        let assign14440_e20214: f64 = (assign14440_e20212 * 1.414213562373095);
        let assign14440_e20216: f64 = (assign14440_e20214 + locals.var_ac31);
        let assign14440_e20219: f64 = (0.5 * locals.var_ac4);
        let assign14440_e20221: f64 = (assign14440_e20219 / locals.var_ac31);
        let assign14440_e20222: f64 = (assign14440_e20216 + assign14440_e20221);
        let assign14440_e20224: f64 = (assign14440_e20222 + locals.var_t5);
        (assign14440_e20224, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign14440_e20219 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14440_e20226;
        locals.var_ac1_dn0 = assign14440_e20226_d_n0;
        locals.var_ac1_dn2 = assign14440_e20226_d_n2;
        locals.var_ac1_dn6 = assign14440_e20226_d_n6;
        locals.var_ac1_dn7 = assign14440_e20226_d_n7;
        locals.var_ac1_dn10 = assign14440_e20226_d_n10;
        locals.var_ac1_dn11 = assign14440_e20226_d_n11;
        locals.var_ac1_dn12 = assign14440_e20226_d_n12;
        locals.var_ac1_dn17 = assign14440_e20226_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign14450_e20239, assign14450_e20239_d_n0, assign14450_e20239_d_n2, assign14450_e20239_d_n6, assign14450_e20239_d_n7, assign14450_e20239_d_n10, assign14450_e20239_d_n11, assign14450_e20239_d_n12, assign14450_e20239_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) && (locals.var_guard441 == 0.0)) {
        let assign14450_e20236: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign14450_e20237: f64 = (assign14450_e20236).sqrt();
        (assign14450_e20237, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign14450_e20237)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign14450_e20237)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign14450_e20239;
        locals.var_ac2_dn0 = assign14450_e20239_d_n0;
        locals.var_ac2_dn2 = assign14450_e20239_d_n2;
        locals.var_ac2_dn6 = assign14450_e20239_d_n6;
        locals.var_ac2_dn7 = assign14450_e20239_d_n7;
        locals.var_ac2_dn10 = assign14450_e20239_d_n10;
        locals.var_ac2_dn11 = assign14450_e20239_d_n11;
        locals.var_ac2_dn12 = assign14450_e20239_d_n12;
        locals.var_ac2_dn17 = assign14450_e20239_d_n17;
        locals.var_ac2_rv = 0.0;

        let (assign14460_e20256, assign14460_e20256_d_n0, assign14460_e20256_d_n2, assign14460_e20256_d_n6, assign14460_e20256_d_n7, assign14460_e20256_d_n10, assign14460_e20256_d_n11, assign14460_e20256_d_n12, assign14460_e20256_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) && (locals.var_guard441 == 0.0)) {
        let assign14460_e20248: f64 = (-7.0);
        let assign14460_e20250: f64 = (assign14460_e20248 * 1.414213562373095);
        let assign14460_e20252: f64 = (assign14460_e20250 + locals.var_ac2);
        let assign14460_e20254: f64 = (assign14460_e20252 + locals.var_t5);
        (assign14460_e20254, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14460_e20256;
        locals.var_ac1_dn0 = assign14460_e20256_d_n0;
        locals.var_ac1_dn2 = assign14460_e20256_d_n2;
        locals.var_ac1_dn6 = assign14460_e20256_d_n6;
        locals.var_ac1_dn7 = assign14460_e20256_d_n7;
        locals.var_ac1_dn10 = assign14460_e20256_d_n10;
        locals.var_ac1_dn11 = assign14460_e20256_d_n11;
        locals.var_ac1_dn12 = assign14460_e20256_d_n12;
        locals.var_ac1_dn17 = assign14460_e20256_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign14470_e20265, assign14470_e20265_d_n0, assign14470_e20265_d_n2, assign14470_e20265_d_n6, assign14470_e20265_d_n7, assign14470_e20265_d_n10, assign14470_e20265_d_n11, assign14470_e20265_d_n12, assign14470_e20265_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14470_e20263: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign14470_e20263, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign14470_e20263 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign14470_e20265;
        locals.var_acd_dn0 = assign14470_e20265_d_n0;
        locals.var_acd_dn2 = assign14470_e20265_d_n2;
        locals.var_acd_dn6 = assign14470_e20265_d_n6;
        locals.var_acd_dn7 = assign14470_e20265_d_n7;
        locals.var_acd_dn10 = assign14470_e20265_d_n10;
        locals.var_acd_dn11 = assign14470_e20265_d_n11;
        locals.var_acd_dn12 = assign14470_e20265_d_n12;
        locals.var_acd_dn17 = assign14470_e20265_d_n17;
        locals.var_acd_rv = 0.0;

        let (assign14480_e20289, assign14480_e20289_d_n0, assign14480_e20289_d_n2, assign14480_e20289_d_n6, assign14480_e20289_d_n7, assign14480_e20289_d_n10, assign14480_e20289_d_n11, assign14480_e20289_d_n12, assign14480_e20289_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14480_e20271: f64 = (-4.0);
        let assign14480_e20273: f64 = (assign14480_e20271 * 1.414213562373095);
        let assign14480_e20276: f64 = (12.0 * locals.var_ty);
        let assign14480_e20277: f64 = (assign14480_e20273 - assign14480_e20276);
        let assign14480_e20280: f64 = (2.0 * locals.var_acd);
        let assign14480_e20281: f64 = (assign14480_e20277 + assign14480_e20280);
        let assign14480_e20284: f64 = (1.414213562373095 * locals.var_acd);
        let assign14480_e20286: f64 = (assign14480_e20284 * locals.var_acd);
        let assign14480_e20287: f64 = (assign14480_e20281 + assign14480_e20286);
        (assign14480_e20287, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign14480_e20284 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign14480_e20289;
        locals.var_acn_dn0 = assign14480_e20289_d_n0;
        locals.var_acn_dn2 = assign14480_e20289_d_n2;
        locals.var_acn_dn6 = assign14480_e20289_d_n6;
        locals.var_acn_dn7 = assign14480_e20289_d_n7;
        locals.var_acn_dn10 = assign14480_e20289_d_n10;
        locals.var_acn_dn11 = assign14480_e20289_d_n11;
        locals.var_acn_dn12 = assign14480_e20289_d_n12;
        locals.var_acn_dn17 = assign14480_e20289_d_n17;
        locals.var_acn_rv = 0.0;

        let (assign14490_e20298, assign14490_e20298_d_n0, assign14490_e20298_d_n2, assign14490_e20298_d_n6, assign14490_e20298_d_n7, assign14490_e20298_d_n10, assign14490_e20298_d_n11, assign14490_e20298_d_n12, assign14490_e20298_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14490_e20296: f64 = (1.0 / locals.var_acd);
        (assign14490_e20296, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14490_e20298;
        locals.var_t1_dn0 = assign14490_e20298_d_n0;
        locals.var_t1_dn2 = assign14490_e20298_d_n2;
        locals.var_t1_dn6 = assign14490_e20298_d_n6;
        locals.var_t1_dn7 = assign14490_e20298_d_n7;
        locals.var_t1_dn10 = assign14490_e20298_d_n10;
        locals.var_t1_dn11 = assign14490_e20298_d_n11;
        locals.var_t1_dn12 = assign14490_e20298_d_n12;
        locals.var_t1_dn17 = assign14490_e20298_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14500_e20307, assign14500_e20307_d_n0, assign14500_e20307_d_n2, assign14500_e20307_d_n6, assign14500_e20307_d_n7, assign14500_e20307_d_n10, assign14500_e20307_d_n11, assign14500_e20307_d_n12, assign14500_e20307_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14500_e20305: f64 = (locals.var_acn * locals.var_t1);
        (assign14500_e20305, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14500_e20307;
        locals.var_chi_dn0 = assign14500_e20307_d_n0;
        locals.var_chi_dn2 = assign14500_e20307_d_n2;
        locals.var_chi_dn6 = assign14500_e20307_d_n6;
        locals.var_chi_dn7 = assign14500_e20307_d_n7;
        locals.var_chi_dn10 = assign14500_e20307_d_n10;
        locals.var_chi_dn11 = assign14500_e20307_d_n11;
        locals.var_chi_dn12 = assign14500_e20307_d_n12;
        locals.var_chi_dn17 = assign14500_e20307_d_n17;
        locals.var_chi_rv = 0.0;

        let (assign14510_e20318, assign14510_e20318_d_n0, assign14510_e20318_d_n2, assign14510_e20318_d_n6, assign14510_e20318_d_n7, assign14510_e20318_d_n10, assign14510_e20318_d_n11, assign14510_e20318_d_n12, assign14510_e20318_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14510_e20314: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign14510_e20316: f64 = (assign14510_e20314 + locals.var_vbcs_cl);
        (assign14510_e20316, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign14510_e20318;
        locals.var_psa_dn0 = assign14510_e20318_d_n0;
        locals.var_psa_dn2 = assign14510_e20318_d_n2;
        locals.var_psa_dn6 = assign14510_e20318_d_n6;
        locals.var_psa_dn7 = assign14510_e20318_d_n7;
        locals.var_psa_dn10 = assign14510_e20318_d_n10;
        locals.var_psa_dn11 = assign14510_e20318_d_n11;
        locals.var_psa_dn12 = assign14510_e20318_d_n12;
        locals.var_psa_dn17 = assign14510_e20318_d_n17;
        locals.var_psa_rv = 0.0;

        let (assign14520_e20327, assign14520_e20327_d_n0, assign14520_e20327_d_n2, assign14520_e20327_d_n6, assign14520_e20327_d_n7, assign14520_e20327_d_n10, assign14520_e20327_d_n11, assign14520_e20327_d_n12, assign14520_e20327_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14520_e20325: f64 = (locals.var_psa - locals.var_vbcs_cl);
        (assign14520_e20325, (locals.var_psa_dn0 - locals.var_vbcs_cl_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14520_e20327;
        locals.var_t1_dn0 = assign14520_e20327_d_n0;
        locals.var_t1_dn2 = assign14520_e20327_d_n2;
        locals.var_t1_dn6 = assign14520_e20327_d_n6;
        locals.var_t1_dn7 = assign14520_e20327_d_n7;
        locals.var_t1_dn10 = assign14520_e20327_d_n10;
        locals.var_t1_dn11 = assign14520_e20327_d_n11;
        locals.var_t1_dn12 = assign14520_e20327_d_n12;
        locals.var_t1_dn17 = assign14520_e20327_d_n17;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        locals: &mut StampLocals,
    ) {
        let (assign14530_e20336, assign14530_e20336_d_n0, assign14530_e20336_d_n2, assign14530_e20336_d_n6, assign14530_e20336_d_n7, assign14530_e20336_d_n10, assign14530_e20336_d_n11, assign14530_e20336_d_n12, assign14530_e20336_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14530_e20334: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign14530_e20334, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14530_e20336;
        locals.var_t2_dn0 = assign14530_e20336_d_n0;
        locals.var_t2_dn2 = assign14530_e20336_d_n2;
        locals.var_t2_dn6 = assign14530_e20336_d_n6;
        locals.var_t2_dn7 = assign14530_e20336_d_n7;
        locals.var_t2_dn10 = assign14530_e20336_d_n10;
        locals.var_t2_dn11 = assign14530_e20336_d_n11;
        locals.var_t2_dn12 = assign14530_e20336_d_n12;
        locals.var_t2_dn17 = assign14530_e20336_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14540_e20348, assign14540_e20348_d_n0, assign14540_e20348_d_n2, assign14540_e20348_d_n6, assign14540_e20348_d_n7, assign14540_e20348_d_n10, assign14540_e20348_d_n11, assign14540_e20348_d_n12, assign14540_e20348_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14540_e20344: f64 = (locals.var_t2 * locals.var_t2);
        let assign14540_e20345: f64 = (1.0 + assign14540_e20344);
        let assign14540_e20346: f64 = (assign14540_e20345).sqrt();
        (assign14540_e20346, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign14540_e20346)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign14540_e20346)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14540_e20348;
        locals.var_t3_dn0 = assign14540_e20348_d_n0;
        locals.var_t3_dn2 = assign14540_e20348_d_n2;
        locals.var_t3_dn6 = assign14540_e20348_d_n6;
        locals.var_t3_dn7 = assign14540_e20348_d_n7;
        locals.var_t3_dn10 = assign14540_e20348_d_n10;
        locals.var_t3_dn11 = assign14540_e20348_d_n11;
        locals.var_t3_dn12 = assign14540_e20348_d_n12;
        locals.var_t3_dn17 = assign14540_e20348_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign14550_e20359, assign14550_e20359_d_n0, assign14550_e20359_d_n2, assign14550_e20359_d_n6, assign14550_e20359_d_n7, assign14550_e20359_d_n10, assign14550_e20359_d_n11, assign14550_e20359_d_n12, assign14550_e20359_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 != 0.0)) {
        let assign14550_e20355: f64 = (locals.var_t1 / locals.var_t3);
        let assign14550_e20357: f64 = (assign14550_e20355 + locals.var_vbcs_cl);
        (assign14550_e20357, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14550_e20359;
        locals.var_ps0_dn0 = assign14550_e20359_d_n0;
        locals.var_ps0_dn2 = assign14550_e20359_d_n2;
        locals.var_ps0_dn6 = assign14550_e20359_d_n6;
        locals.var_ps0_dn7 = assign14550_e20359_d_n7;
        locals.var_ps0_dn10 = assign14550_e20359_d_n10;
        locals.var_ps0_dn11 = assign14550_e20359_d_n11;
        locals.var_ps0_dn12 = assign14550_e20359_d_n12;
        locals.var_ps0_dn17 = assign14550_e20359_d_n17;
        locals.var_ps0_rv = 0.0;

        let assign14560_e20362: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard442 = assign14560_e20362;
        locals.var_guard442_rv = 0.0;

        let (assign14570_e20372, assign14570_e20372_d_n0, assign14570_e20372_d_n2, assign14570_e20372_d_n6, assign14570_e20372_d_n7, assign14570_e20372_d_n10, assign14570_e20372_d_n11, assign14570_e20372_d_n12, assign14570_e20372_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14570_e20372;
        locals.var_phi_s0_soi_dn0 = assign14570_e20372_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14570_e20372_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14570_e20372_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14570_e20372_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14570_e20372_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14570_e20372_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14570_e20372_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14570_e20372_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign14580_e20382, assign14580_e20382_d_n0, assign14580_e20382_d_n2, assign14580_e20382_d_n6, assign14580_e20382_d_n7, assign14580_e20382_d_n10, assign14580_e20382_d_n11, assign14580_e20382_d_n12, assign14580_e20382_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14580_e20382;
        locals.var_ps0_ini_dn0 = assign14580_e20382_d_n0;
        locals.var_ps0_ini_dn2 = assign14580_e20382_d_n2;
        locals.var_ps0_ini_dn6 = assign14580_e20382_d_n6;
        locals.var_ps0_ini_dn7 = assign14580_e20382_d_n7;
        locals.var_ps0_ini_dn10 = assign14580_e20382_d_n10;
        locals.var_ps0_ini_dn11 = assign14580_e20382_d_n11;
        locals.var_ps0_ini_dn12 = assign14580_e20382_d_n12;
        locals.var_ps0_ini_dn17 = assign14580_e20382_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14590_e20407, assign14590_e20407_d_n0, assign14590_e20407_d_n2, assign14590_e20407_d_n6, assign14590_e20407_d_n7, assign14590_e20407_d_n10, assign14590_e20407_d_n11, assign14590_e20407_d_n12, assign14590_e20407_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14590_e20396: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14590_e20397: f64 = (locals.var_beta * assign14590_e20396);
        let assign14590_e20399: f64 = (assign14590_e20397 - 1.0);
        let assign14590_e20400: f64 = (4.0 * assign14590_e20399);
        let assign14590_e20403: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign14590_e20404: f64 = (assign14590_e20400 / assign14590_e20403);
        let assign14590_e20405: f64 = (1.0 + assign14590_e20404);
        (assign14590_e20405, ((((4.0 * (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * ((locals.var_beta_dn10 * assign14590_e20396) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10)))) * assign14590_e20403) - (assign14590_e20400 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17))) * assign14590_e20403) - (assign14590_e20400 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign14590_e20403 * assign14590_e20403)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14590_e20407;
        locals.var_tx_dn0 = assign14590_e20407_d_n0;
        locals.var_tx_dn2 = assign14590_e20407_d_n2;
        locals.var_tx_dn6 = assign14590_e20407_d_n6;
        locals.var_tx_dn7 = assign14590_e20407_d_n7;
        locals.var_tx_dn10 = assign14590_e20407_d_n10;
        locals.var_tx_dn11 = assign14590_e20407_d_n11;
        locals.var_tx_dn12 = assign14590_e20407_d_n12;
        locals.var_tx_dn17 = assign14590_e20407_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14600_e20427, assign14600_e20427_d_n0, assign14600_e20427_d_n2, assign14600_e20427_d_n6, assign14600_e20427_d_n7, assign14600_e20427_d_n10, assign14600_e20427_d_n11, assign14600_e20427_d_n12, assign14600_e20427_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14600_e20419: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14600_e20425, assign14600_e20425_d_n0, assign14600_e20425_d_n2, assign14600_e20425_d_n6, assign14600_e20425_d_n7, assign14600_e20425_d_n10, assign14600_e20425_d_n11, assign14600_e20425_d_n12, assign14600_e20425_d_n17,) = {
            if (locals.var_tx >= assign14600_e20419) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign14600_e20424: f64 = (10.0 * 2.220446049250313e-16);
                (assign14600_e20424, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14600_e20425, assign14600_e20425_d_n0, assign14600_e20425_d_n2, assign14600_e20425_d_n6, assign14600_e20425_d_n7, assign14600_e20425_d_n10, assign14600_e20425_d_n11, assign14600_e20425_d_n12, assign14600_e20425_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14600_e20427;
        locals.var_tx_dn0 = assign14600_e20427_d_n0;
        locals.var_tx_dn2 = assign14600_e20427_d_n2;
        locals.var_tx_dn6 = assign14600_e20427_d_n6;
        locals.var_tx_dn7 = assign14600_e20427_d_n7;
        locals.var_tx_dn10 = assign14600_e20427_d_n10;
        locals.var_tx_dn11 = assign14600_e20427_d_n11;
        locals.var_tx_dn12 = assign14600_e20427_d_n12;
        locals.var_tx_dn17 = assign14600_e20427_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14610_e20449, assign14610_e20449_d_n0, assign14610_e20449_d_n2, assign14610_e20449_d_n6, assign14610_e20449_d_n7, assign14610_e20449_d_n10, assign14610_e20449_d_n11, assign14610_e20449_d_n12, assign14610_e20449_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14610_e20439: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign14610_e20441: f64 = (assign14610_e20439 * 0.5);
        let assign14610_e20444: f64 = (locals.var_tx).sqrt();
        let assign14610_e20445: f64 = (1.0 - assign14610_e20444);
        let assign14610_e20446: f64 = (assign14610_e20441 * assign14610_e20445);
        let assign14610_e20447: f64 = (locals.var_vgp + assign14610_e20446);
        (assign14610_e20447, (locals.var_vgp_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn0 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn2 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn6 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn7 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn10 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn11 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn12 / (2.0 * assign14610_e20444)))))), (locals.var_vgp_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign14610_e20445) + (assign14610_e20441 * (-(locals.var_tx_dn17 / (2.0 * assign14610_e20444)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14610_e20449;
        locals.var_ps0_inia_dn0 = assign14610_e20449_d_n0;
        locals.var_ps0_inia_dn2 = assign14610_e20449_d_n2;
        locals.var_ps0_inia_dn6 = assign14610_e20449_d_n6;
        locals.var_ps0_inia_dn7 = assign14610_e20449_d_n7;
        locals.var_ps0_inia_dn10 = assign14610_e20449_d_n10;
        locals.var_ps0_inia_dn11 = assign14610_e20449_d_n11;
        locals.var_ps0_inia_dn12 = assign14610_e20449_d_n12;
        locals.var_ps0_inia_dn17 = assign14610_e20449_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign14620_e20464, assign14620_e20464_d_n0, assign14620_e20464_d_n2, assign14620_e20464_d_n6, assign14620_e20464_d_n7, assign14620_e20464_d_n10, assign14620_e20464_d_n11, assign14620_e20464_d_n12, assign14620_e20464_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14620_e20461: f64 = (locals.var_ps0_inia - locals.var_vbcs_cl);
        let assign14620_e20462: f64 = (locals.var_beta * assign14620_e20461);
        (assign14620_e20462, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14620_e20461) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14620_e20464;
        locals.var_chi_dn0 = assign14620_e20464_d_n0;
        locals.var_chi_dn2 = assign14620_e20464_d_n2;
        locals.var_chi_dn6 = assign14620_e20464_d_n6;
        locals.var_chi_dn7 = assign14620_e20464_d_n7;
        locals.var_chi_dn10 = assign14620_e20464_d_n10;
        locals.var_chi_dn11 = assign14620_e20464_d_n11;
        locals.var_chi_dn12 = assign14620_e20464_d_n12;
        locals.var_chi_dn17 = assign14620_e20464_d_n17;
        locals.var_chi_rv = 0.0;

        let assign14630_e20467: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign14630_e20467;
        locals.var_guard443_rv = 0.0;

        let (assign14640_e20484, assign14640_e20484_d_n0, assign14640_e20484_d_n2, assign14640_e20484_d_n6, assign14640_e20484_d_n7, assign14640_e20484_d_n10, assign14640_e20484_d_n11, assign14640_e20484_d_n12, assign14640_e20484_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14640_e20481: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14640_e20482: f64 = (locals.var_beta * assign14640_e20481);
        (assign14640_e20482, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14640_e20481) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14640_e20484;
        locals.var_ty_dn0 = assign14640_e20484_d_n0;
        locals.var_ty_dn2 = assign14640_e20484_d_n2;
        locals.var_ty_dn6 = assign14640_e20484_d_n6;
        locals.var_ty_dn7 = assign14640_e20484_d_n7;
        locals.var_ty_dn10 = assign14640_e20484_d_n10;
        locals.var_ty_dn11 = assign14640_e20484_d_n11;
        locals.var_ty_dn12 = assign14640_e20484_d_n12;
        locals.var_ty_dn17 = assign14640_e20484_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign14650_e20505, assign14650_e20505_d_n0, assign14650_e20505_d_n2, assign14650_e20505_d_n6, assign14650_e20505_d_n7, assign14650_e20505_d_n10, assign14650_e20505_d_n11, assign14650_e20505_d_n12, assign14650_e20505_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14650_e20498: f64 = (1.414213562373095 / 108.0);
        let assign14650_e20500: f64 = (assign14650_e20498 * locals.var_beta);
        let assign14650_e20502: f64 = (assign14650_e20500 * locals.var_fac1);
        let assign14650_e20503: f64 = (1.0 / assign14650_e20502);
        (assign14650_e20503, (-((assign14650_e20500 * locals.var_fac1_dn0) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn2) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn6) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn7) / (assign14650_e20502 * assign14650_e20502))), (-((((assign14650_e20498 * locals.var_beta_dn10) * locals.var_fac1) + (assign14650_e20500 * locals.var_fac1_dn10)) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn11) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn12) / (assign14650_e20502 * assign14650_e20502))), (-((assign14650_e20500 * locals.var_fac1_dn17) / (assign14650_e20502 * assign14650_e20502))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14650_e20505;
        locals.var_t1_dn0 = assign14650_e20505_d_n0;
        locals.var_t1_dn2 = assign14650_e20505_d_n2;
        locals.var_t1_dn6 = assign14650_e20505_d_n6;
        locals.var_t1_dn7 = assign14650_e20505_d_n7;
        locals.var_t1_dn10 = assign14650_e20505_d_n10;
        locals.var_t1_dn11 = assign14650_e20505_d_n11;
        locals.var_t1_dn12 = assign14650_e20505_d_n12;
        locals.var_t1_dn17 = assign14650_e20505_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14660_e20522, assign14660_e20522_d_n0, assign14660_e20522_d_n2, assign14660_e20522_d_n6, assign14660_e20522_d_n7, assign14660_e20522_d_n10, assign14660_e20522_d_n11, assign14660_e20522_d_n12, assign14660_e20522_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14660_e20519: f64 = (3.0 * locals.var_t1);
        let assign14660_e20520: f64 = (81.0 + assign14660_e20519);
        (assign14660_e20520, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14660_e20522;
        locals.var_t2_dn0 = assign14660_e20522_d_n0;
        locals.var_t2_dn2 = assign14660_e20522_d_n2;
        locals.var_t2_dn6 = assign14660_e20522_d_n6;
        locals.var_t2_dn7 = assign14660_e20522_d_n7;
        locals.var_t2_dn10 = assign14660_e20522_d_n10;
        locals.var_t2_dn11 = assign14660_e20522_d_n11;
        locals.var_t2_dn12 = assign14660_e20522_d_n12;
        locals.var_t2_dn17 = assign14660_e20522_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14670_e20546, assign14670_e20546_d_n0, assign14670_e20546_d_n2, assign14670_e20546_d_n6, assign14670_e20546_d_n7, assign14670_e20546_d_n10, assign14670_e20546_d_n11, assign14670_e20546_d_n12, assign14670_e20546_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14670_e20534: f64 = (-2916.0);
        let assign14670_e20537: f64 = (81.0 * locals.var_t1);
        let assign14670_e20538: f64 = (assign14670_e20534 - assign14670_e20537);
        let assign14670_e20541: f64 = (27.0 * locals.var_t1);
        let assign14670_e20543: f64 = (assign14670_e20541 * locals.var_ty);
        let assign14670_e20544: f64 = (assign14670_e20538 + assign14670_e20543);
        (assign14670_e20544, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14670_e20541 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14670_e20546;
        locals.var_t3_dn0 = assign14670_e20546_d_n0;
        locals.var_t3_dn2 = assign14670_e20546_d_n2;
        locals.var_t3_dn6 = assign14670_e20546_d_n6;
        locals.var_t3_dn7 = assign14670_e20546_d_n7;
        locals.var_t3_dn10 = assign14670_e20546_d_n10;
        locals.var_t3_dn11 = assign14670_e20546_d_n11;
        locals.var_t3_dn12 = assign14670_e20546_d_n12;
        locals.var_t3_dn17 = assign14670_e20546_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign14680_e20571, assign14680_e20571_d_n0, assign14680_e20571_d_n2, assign14680_e20571_d_n6, assign14680_e20571_d_n7, assign14680_e20571_d_n10, assign14680_e20571_d_n11, assign14680_e20571_d_n12, assign14680_e20571_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14680_e20561: f64 = (54.0 + locals.var_t1);
        let assign14680_e20562: f64 = (81.0 * assign14680_e20561);
        let assign14680_e20563: f64 = (1458.0 - assign14680_e20562);
        let assign14680_e20566: f64 = (27.0 * locals.var_t1);
        let assign14680_e20568: f64 = (assign14680_e20566 * locals.var_ty);
        let assign14680_e20569: f64 = (assign14680_e20563 + assign14680_e20568);
        (assign14680_e20569, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14680_e20566 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14680_e20571;
        locals.var_t4_dn0 = assign14680_e20571_d_n0;
        locals.var_t4_dn2 = assign14680_e20571_d_n2;
        locals.var_t4_dn6 = assign14680_e20571_d_n6;
        locals.var_t4_dn7 = assign14680_e20571_d_n7;
        locals.var_t4_dn10 = assign14680_e20571_d_n10;
        locals.var_t4_dn11 = assign14680_e20571_d_n11;
        locals.var_t4_dn12 = assign14680_e20571_d_n12;
        locals.var_t4_dn17 = assign14680_e20571_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14690_e20586, assign14690_e20586_d_n0, assign14690_e20586_d_n2, assign14690_e20586_d_n6, assign14690_e20586_d_n7, assign14690_e20586_d_n10, assign14690_e20586_d_n11, assign14690_e20586_d_n12, assign14690_e20586_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14690_e20584: f64 = (locals.var_t4 * locals.var_t4);
        (assign14690_e20584, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14690_e20586;
        locals.var_t4_dn0 = assign14690_e20586_d_n0;
        locals.var_t4_dn2 = assign14690_e20586_d_n2;
        locals.var_t4_dn6 = assign14690_e20586_d_n6;
        locals.var_t4_dn7 = assign14690_e20586_d_n7;
        locals.var_t4_dn10 = assign14690_e20586_d_n10;
        locals.var_t4_dn11 = assign14690_e20586_d_n11;
        locals.var_t4_dn12 = assign14690_e20586_d_n12;
        locals.var_t4_dn17 = assign14690_e20586_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14700_e20612, assign14700_e20612_d_n0, assign14700_e20612_d_n2, assign14700_e20612_d_n6, assign14700_e20612_d_n7, assign14700_e20612_d_n10, assign14700_e20612_d_n11, assign14700_e20612_d_n12, assign14700_e20612_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14700_e20600: f64 = (4.0 * locals.var_t2);
        let assign14700_e20602: f64 = (assign14700_e20600 * locals.var_t2);
        let assign14700_e20604: f64 = (assign14700_e20602 * locals.var_t2);
        let assign14700_e20606: f64 = (assign14700_e20604 + locals.var_t4);
        let assign14700_e20607: f64 = (assign14700_e20606).sqrt();
        let assign14700_e20608: f64 = (locals.var_t3 + assign14700_e20607);
        let assign14700_e20610: f64 = (assign14700_e20608).powf(0.3333333333333333);
        (assign14700_e20610, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn0)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn0)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn2)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn2)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn6)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn6)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn7)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn7)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn10)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn10)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn11)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn11)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn12)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn12)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14700_e20607))) / assign14700_e20608))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14700_e20608).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn17)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14700_e20607))))) } } else { (assign14700_e20610 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14700_e20600 * locals.var_t2_dn17)) * locals.var_t2) + (assign14700_e20602 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14700_e20607))) / assign14700_e20608))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14700_e20612;
        locals.var_t5_dn0 = assign14700_e20612_d_n0;
        locals.var_t5_dn2 = assign14700_e20612_d_n2;
        locals.var_t5_dn6 = assign14700_e20612_d_n6;
        locals.var_t5_dn7 = assign14700_e20612_d_n7;
        locals.var_t5_dn10 = assign14700_e20612_d_n10;
        locals.var_t5_dn11 = assign14700_e20612_d_n11;
        locals.var_t5_dn12 = assign14700_e20612_d_n12;
        locals.var_t5_dn17 = assign14700_e20612_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign14710_e20641, assign14710_e20641_d_n0, assign14710_e20641_d_n2, assign14710_e20641_d_n6, assign14710_e20641_d_n7, assign14710_e20641_d_n10, assign14710_e20641_d_n11, assign14710_e20641_d_n12, assign14710_e20641_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14710_e20626: f64 = (1.259921049894873 * locals.var_t2);
        let assign14710_e20629: f64 = (3.0 * locals.var_t5);
        let assign14710_e20630: f64 = (assign14710_e20626 / assign14710_e20629);
        let assign14710_e20631: f64 = (3.0 - assign14710_e20630);
        let assign14710_e20635: f64 = (3.0 * 1.259921049894873);
        let assign14710_e20636: f64 = (1.0 / assign14710_e20635);
        let assign14710_e20638: f64 = (assign14710_e20636 * locals.var_t5);
        let assign14710_e20639: f64 = (assign14710_e20631 + assign14710_e20638);
        (assign14710_e20639, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn0))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn2))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn6))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn7))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn10))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn11))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn12))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign14710_e20629) - (assign14710_e20626 * (3.0 * locals.var_t5_dn17))) / (assign14710_e20629 * assign14710_e20629))) + (assign14710_e20636 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14710_e20641;
        locals.var_tx_dn0 = assign14710_e20641_d_n0;
        locals.var_tx_dn2 = assign14710_e20641_d_n2;
        locals.var_tx_dn6 = assign14710_e20641_d_n6;
        locals.var_tx_dn7 = assign14710_e20641_d_n7;
        locals.var_tx_dn10 = assign14710_e20641_d_n10;
        locals.var_tx_dn11 = assign14710_e20641_d_n11;
        locals.var_tx_dn12 = assign14710_e20641_d_n12;
        locals.var_tx_dn17 = assign14710_e20641_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14720_e20658, assign14720_e20658_d_n0, assign14720_e20658_d_n2, assign14720_e20658_d_n6, assign14720_e20658_d_n7, assign14720_e20658_d_n10, assign14720_e20658_d_n11, assign14720_e20658_d_n12, assign14720_e20658_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14720_e20654: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign14720_e20656: f64 = (assign14720_e20654 + locals.var_vbcs_cl);
        (assign14720_e20656, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14720_e20658;
        locals.var_ps0_inia_dn0 = assign14720_e20658_d_n0;
        locals.var_ps0_inia_dn2 = assign14720_e20658_d_n2;
        locals.var_ps0_inia_dn6 = assign14720_e20658_d_n6;
        locals.var_ps0_inia_dn7 = assign14720_e20658_d_n7;
        locals.var_ps0_inia_dn10 = assign14720_e20658_d_n10;
        locals.var_ps0_inia_dn11 = assign14720_e20658_d_n11;
        locals.var_ps0_inia_dn12 = assign14720_e20658_d_n12;
        locals.var_ps0_inia_dn17 = assign14720_e20658_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign14730_e20671, assign14730_e20671_d_n0, assign14730_e20671_d_n2, assign14730_e20671_d_n6, assign14730_e20671_d_n7, assign14730_e20671_d_n10, assign14730_e20671_d_n11, assign14730_e20671_d_n12, assign14730_e20671_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14730_e20671;
        locals.var_ps0_ini_dn0 = assign14730_e20671_d_n0;
        locals.var_ps0_ini_dn2 = assign14730_e20671_d_n2;
        locals.var_ps0_ini_dn6 = assign14730_e20671_d_n6;
        locals.var_ps0_ini_dn7 = assign14730_e20671_d_n7;
        locals.var_ps0_ini_dn10 = assign14730_e20671_d_n10;
        locals.var_ps0_ini_dn11 = assign14730_e20671_d_n11;
        locals.var_ps0_ini_dn12 = assign14730_e20671_d_n12;
        locals.var_ps0_ini_dn17 = assign14730_e20671_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign14740_e20674: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard444 = assign14740_e20674;
        locals.var_guard444_rv = 0.0;

        let (assign14750_e20690, assign14750_e20690_d_n0, assign14750_e20690_d_n2, assign14750_e20690_d_n6, assign14750_e20690_d_n7, assign14750_e20690_d_n10, assign14750_e20690_d_n11, assign14750_e20690_d_n12, assign14750_e20690_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14750_e20690;
        locals.var_ps0_ini_dn0 = assign14750_e20690_d_n0;
        locals.var_ps0_ini_dn2 = assign14750_e20690_d_n2;
        locals.var_ps0_ini_dn6 = assign14750_e20690_d_n6;
        locals.var_ps0_ini_dn7 = assign14750_e20690_d_n7;
        locals.var_ps0_ini_dn10 = assign14750_e20690_d_n10;
        locals.var_ps0_ini_dn11 = assign14750_e20690_d_n11;
        locals.var_ps0_ini_dn12 = assign14750_e20690_d_n12;
        locals.var_ps0_ini_dn17 = assign14750_e20690_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14760_e20711, assign14760_e20711_d_n0, assign14760_e20711_d_n2, assign14760_e20711_d_n6, assign14760_e20711_d_n7, assign14760_e20711_d_n10, assign14760_e20711_d_n11, assign14760_e20711_d_n12, assign14760_e20711_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14760_e20707: f64 = (1.0 / locals.var_cnst1soi);
        let assign14760_e20709: f64 = (assign14760_e20707 / locals.var_cnstc_foxi);
        (assign14760_e20709, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14760_e20707 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14760_e20711;
        locals.var_t1_dn0 = assign14760_e20711_d_n0;
        locals.var_t1_dn2 = assign14760_e20711_d_n2;
        locals.var_t1_dn6 = assign14760_e20711_d_n6;
        locals.var_t1_dn7 = assign14760_e20711_d_n7;
        locals.var_t1_dn10 = assign14760_e20711_d_n10;
        locals.var_t1_dn11 = assign14760_e20711_d_n11;
        locals.var_t1_dn12 = assign14760_e20711_d_n12;
        locals.var_t1_dn17 = assign14760_e20711_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14770_e20732, assign14770_e20732_d_n0, assign14770_e20732_d_n2, assign14770_e20732_d_n6, assign14770_e20732_d_n7, assign14770_e20732_d_n10, assign14770_e20732_d_n11, assign14770_e20732_d_n12, assign14770_e20732_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14770_e20728: f64 = (locals.var_t1 * locals.var_vgp);
        let assign14770_e20730: f64 = (assign14770_e20728 * locals.var_vgp);
        (assign14770_e20730, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn2)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn7)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn11)), ((((locals.var_t1_dn12 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn12)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn12)), ((((locals.var_t1_dn17 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn17)) * locals.var_vgp) + (assign14770_e20728 * locals.var_vgp_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14770_e20732;
        locals.var_t2_dn0 = assign14770_e20732_d_n0;
        locals.var_t2_dn2 = assign14770_e20732_d_n2;
        locals.var_t2_dn6 = assign14770_e20732_d_n6;
        locals.var_t2_dn7 = assign14770_e20732_d_n7;
        locals.var_t2_dn10 = assign14770_e20732_d_n10;
        locals.var_t2_dn11 = assign14770_e20732_d_n11;
        locals.var_t2_dn12 = assign14770_e20732_d_n12;
        locals.var_t2_dn17 = assign14770_e20732_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14780_e20753, assign14780_e20753_d_n0, assign14780_e20753_d_n2, assign14780_e20753_d_n6, assign14780_e20753_d_n7, assign14780_e20753_d_n10, assign14780_e20753_d_n11, assign14780_e20753_d_n12, assign14780_e20753_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14780_e20750: f64 = (2.0 / locals.var_vgp);
        let assign14780_e20751: f64 = (locals.var_beta + assign14780_e20750);
        (assign14780_e20751, (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn12) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn17) / (locals.var_vgp * locals.var_vgp))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14780_e20753;
        locals.var_t3_dn0 = assign14780_e20753_d_n0;
        locals.var_t3_dn2 = assign14780_e20753_d_n2;
        locals.var_t3_dn6 = assign14780_e20753_d_n6;
        locals.var_t3_dn7 = assign14780_e20753_d_n7;
        locals.var_t3_dn10 = assign14780_e20753_d_n10;
        locals.var_t3_dn11 = assign14780_e20753_d_n11;
        locals.var_t3_dn12 = assign14780_e20753_d_n12;
        locals.var_t3_dn17 = assign14780_e20753_d_n17;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign14790_e20773, assign14790_e20773_d_n0, assign14790_e20773_d_n2, assign14790_e20773_d_n6, assign14790_e20773_d_n7, assign14790_e20773_d_n10, assign14790_e20773_d_n11, assign14790_e20773_d_n12, assign14790_e20773_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14790_e20769: f64 = (locals.var_t2).ln();
        let assign14790_e20771: f64 = (assign14790_e20769 / locals.var_t3);
        (assign14790_e20771, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign14790_e20769 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign14790_e20773;
        locals.var_ps0_inib_dn0 = assign14790_e20773_d_n0;
        locals.var_ps0_inib_dn2 = assign14790_e20773_d_n2;
        locals.var_ps0_inib_dn6 = assign14790_e20773_d_n6;
        locals.var_ps0_inib_dn7 = assign14790_e20773_d_n7;
        locals.var_ps0_inib_dn10 = assign14790_e20773_d_n10;
        locals.var_ps0_inib_dn11 = assign14790_e20773_d_n11;
        locals.var_ps0_inib_dn12 = assign14790_e20773_d_n12;
        locals.var_ps0_inib_dn17 = assign14790_e20773_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let (assign14800_e20794, assign14800_e20794_d_n0, assign14800_e20794_d_n2, assign14800_e20794_d_n6, assign14800_e20794_d_n7, assign14800_e20794_d_n10, assign14800_e20794_d_n11, assign14800_e20794_d_n12, assign14800_e20794_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14800_e20790: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign14800_e20792: f64 = (assign14800_e20790 - 0.0008);
        (assign14800_e20792, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign14800_e20794;
        locals.var_tmf1_dn0 = assign14800_e20794_d_n0;
        locals.var_tmf1_dn2 = assign14800_e20794_d_n2;
        locals.var_tmf1_dn6 = assign14800_e20794_d_n6;
        locals.var_tmf1_dn7 = assign14800_e20794_d_n7;
        locals.var_tmf1_dn10 = assign14800_e20794_d_n10;
        locals.var_tmf1_dn11 = assign14800_e20794_d_n11;
        locals.var_tmf1_dn12 = assign14800_e20794_d_n12;
        locals.var_tmf1_dn17 = assign14800_e20794_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign14810_e20815, assign14810_e20815_d_n0, assign14810_e20815_d_n2, assign14810_e20815_d_n6, assign14810_e20815_d_n7, assign14810_e20815_d_n10, assign14810_e20815_d_n11, assign14810_e20815_d_n12, assign14810_e20815_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14810_e20811: f64 = (4.0 * locals.var_ps0_inib);
        let assign14810_e20813: f64 = (assign14810_e20811 * 0.0008);
        (assign14810_e20813, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14810_e20815;
        locals.var_tmf2_dn0 = assign14810_e20815_d_n0;
        locals.var_tmf2_dn2 = assign14810_e20815_d_n2;
        locals.var_tmf2_dn6 = assign14810_e20815_d_n6;
        locals.var_tmf2_dn7 = assign14810_e20815_d_n7;
        locals.var_tmf2_dn10 = assign14810_e20815_d_n10;
        locals.var_tmf2_dn11 = assign14810_e20815_d_n11;
        locals.var_tmf2_dn12 = assign14810_e20815_d_n12;
        locals.var_tmf2_dn17 = assign14810_e20815_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14820_e20838, assign14820_e20838_d_n0, assign14820_e20838_d_n2, assign14820_e20838_d_n6, assign14820_e20838_d_n7, assign14820_e20838_d_n10, assign14820_e20838_d_n11, assign14820_e20838_d_n12, assign14820_e20838_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let (assign14820_e20836, assign14820_e20836_d_n0, assign14820_e20836_d_n2, assign14820_e20836_d_n6, assign14820_e20836_d_n7, assign14820_e20836_d_n10, assign14820_e20836_d_n11, assign14820_e20836_d_n12, assign14820_e20836_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign14820_e20835: f64 = (-locals.var_tmf2);
                (assign14820_e20835, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign14820_e20836, assign14820_e20836_d_n0, assign14820_e20836_d_n2, assign14820_e20836_d_n6, assign14820_e20836_d_n7, assign14820_e20836_d_n10, assign14820_e20836_d_n11, assign14820_e20836_d_n12, assign14820_e20836_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14820_e20838;
        locals.var_tmf2_dn0 = assign14820_e20838_d_n0;
        locals.var_tmf2_dn2 = assign14820_e20838_d_n2;
        locals.var_tmf2_dn6 = assign14820_e20838_d_n6;
        locals.var_tmf2_dn7 = assign14820_e20838_d_n7;
        locals.var_tmf2_dn10 = assign14820_e20838_d_n10;
        locals.var_tmf2_dn11 = assign14820_e20838_d_n11;
        locals.var_tmf2_dn12 = assign14820_e20838_d_n12;
        locals.var_tmf2_dn17 = assign14820_e20838_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14830_e20860, assign14830_e20860_d_n0, assign14830_e20860_d_n2, assign14830_e20860_d_n6, assign14830_e20860_d_n7, assign14830_e20860_d_n10, assign14830_e20860_d_n11, assign14830_e20860_d_n12, assign14830_e20860_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14830_e20855: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14830_e20857: f64 = (assign14830_e20855 + locals.var_tmf2);
        let assign14830_e20858: f64 = (assign14830_e20857).sqrt();
        (assign14830_e20858, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign14830_e20858)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign14830_e20858)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14830_e20860;
        locals.var_tmf2_dn0 = assign14830_e20860_d_n0;
        locals.var_tmf2_dn2 = assign14830_e20860_d_n2;
        locals.var_tmf2_dn6 = assign14830_e20860_d_n6;
        locals.var_tmf2_dn7 = assign14830_e20860_d_n7;
        locals.var_tmf2_dn10 = assign14830_e20860_d_n10;
        locals.var_tmf2_dn11 = assign14830_e20860_d_n11;
        locals.var_tmf2_dn12 = assign14830_e20860_d_n12;
        locals.var_tmf2_dn17 = assign14830_e20860_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14840_e20883, assign14840_e20883_d_n0, assign14840_e20883_d_n2, assign14840_e20883_d_n6, assign14840_e20883_d_n7, assign14840_e20883_d_n10, assign14840_e20883_d_n11, assign14840_e20883_d_n12, assign14840_e20883_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard443 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14840_e20879: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14840_e20880: f64 = (0.5 * assign14840_e20879);
        let assign14840_e20881: f64 = (locals.var_ps0_inib - assign14840_e20880);
        (assign14840_e20881, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14840_e20883;
        locals.var_ps0_ini_dn0 = assign14840_e20883_d_n0;
        locals.var_ps0_ini_dn2 = assign14840_e20883_d_n2;
        locals.var_ps0_ini_dn6 = assign14840_e20883_d_n6;
        locals.var_ps0_ini_dn7 = assign14840_e20883_d_n7;
        locals.var_ps0_ini_dn10 = assign14840_e20883_d_n10;
        locals.var_ps0_ini_dn11 = assign14840_e20883_d_n11;
        locals.var_ps0_ini_dn12 = assign14840_e20883_d_n12;
        locals.var_ps0_ini_dn17 = assign14840_e20883_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14850_e20898, assign14850_e20898_d_n0, assign14850_e20898_d_n2, assign14850_e20898_d_n6, assign14850_e20898_d_n7, assign14850_e20898_d_n10, assign14850_e20898_d_n11, assign14850_e20898_d_n12, assign14850_e20898_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) {
        let assign14850_e20895: f64 = (5e-12 / 2.0);
        let assign14850_e20896: f64 = (locals.var_vbcs_cl + assign14850_e20895);
        (assign14850_e20896, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14850_e20898;
        locals.var_tx_dn0 = assign14850_e20898_d_n0;
        locals.var_tx_dn2 = assign14850_e20898_d_n2;
        locals.var_tx_dn6 = assign14850_e20898_d_n6;
        locals.var_tx_dn7 = assign14850_e20898_d_n7;
        locals.var_tx_dn10 = assign14850_e20898_d_n10;
        locals.var_tx_dn11 = assign14850_e20898_d_n11;
        locals.var_tx_dn12 = assign14850_e20898_d_n12;
        locals.var_tx_dn17 = assign14850_e20898_d_n17;
        locals.var_tx_rv = 0.0;

        let assign14860_e20901: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard445 = assign14860_e20901;
        locals.var_guard445_rv = 0.0;

        let (assign14870_e20914, assign14870_e20914_d_n0, assign14870_e20914_d_n2, assign14870_e20914_d_n6, assign14870_e20914_d_n7, assign14870_e20914_d_n10, assign14870_e20914_d_n11, assign14870_e20914_d_n12, assign14870_e20914_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) && (locals.var_guard442 == 0.0)) && (locals.var_guard445 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14870_e20914;
        locals.var_ps0_ini_dn0 = assign14870_e20914_d_n0;
        locals.var_ps0_ini_dn2 = assign14870_e20914_d_n2;
        locals.var_ps0_ini_dn6 = assign14870_e20914_d_n6;
        locals.var_ps0_ini_dn7 = assign14870_e20914_d_n7;
        locals.var_ps0_ini_dn10 = assign14870_e20914_d_n10;
        locals.var_ps0_ini_dn11 = assign14870_e20914_d_n11;
        locals.var_ps0_ini_dn12 = assign14870_e20914_d_n12;
        locals.var_ps0_ini_dn17 = assign14870_e20914_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14880_e20922, assign14880_e20922_d_n0, assign14880_e20922_d_n2, assign14880_e20922_d_n6, assign14880_e20922_d_n7, assign14880_e20922_d_n10, assign14880_e20922_d_n11, assign14880_e20922_d_n12, assign14880_e20922_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14880_e20922;
        locals.var_ps0_dn0 = assign14880_e20922_d_n0;
        locals.var_ps0_dn2 = assign14880_e20922_d_n2;
        locals.var_ps0_dn6 = assign14880_e20922_d_n6;
        locals.var_ps0_dn7 = assign14880_e20922_d_n7;
        locals.var_ps0_dn10 = assign14880_e20922_d_n10;
        locals.var_ps0_dn11 = assign14880_e20922_d_n11;
        locals.var_ps0_dn12 = assign14880_e20922_d_n12;
        locals.var_ps0_dn17 = assign14880_e20922_d_n17;
        locals.var_ps0_rv = 0.0;

        let (assign14890_e20930, assign14890_e20930_d_n0, assign14890_e20930_d_n2, assign14890_e20930_d_n6, assign14890_e20930_d_n7, assign14890_e20930_d_n10, assign14890_e20930_d_n11, assign14890_e20930_d_n12, assign14890_e20930_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard440 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign14890_e20930;
        locals.var_psl_lim_dn0 = assign14890_e20930_d_n0;
        locals.var_psl_lim_dn2 = assign14890_e20930_d_n2;
        locals.var_psl_lim_dn6 = assign14890_e20930_d_n6;
        locals.var_psl_lim_dn7 = assign14890_e20930_d_n7;
        locals.var_psl_lim_dn10 = assign14890_e20930_d_n10;
        locals.var_psl_lim_dn11 = assign14890_e20930_d_n11;
        locals.var_psl_lim_dn12 = assign14890_e20930_d_n12;
        locals.var_psl_lim_dn17 = assign14890_e20930_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let assign14900_e20937: f64 = if ((p.p25 == 1.0) && (p.p26 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard446 = assign14900_e20937;
        locals.var_guard446_rv = 0.0;

        let (assign14910_e20948, assign14910_e20948_d_n0, assign14910_e20948_d_n2, assign14910_e20948_d_n6, assign14910_e20948_d_n7, assign14910_e20948_d_n10, assign14910_e20948_d_n11, assign14910_e20948_d_n12, assign14910_e20948_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard446 != 0.0)) {
        let assign14910_e20944: f64 = (1e-9 / 0.0001);
        let assign14910_e20946: f64 = (assign14910_e20944 * (nv17 - 0.0));
        (assign14910_e20946, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign14910_e20944,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign14910_e20948;
        locals.var_qhs_dn0 = assign14910_e20948_d_n0;
        locals.var_qhs_dn2 = assign14910_e20948_d_n2;
        locals.var_qhs_dn6 = assign14910_e20948_d_n6;
        locals.var_qhs_dn7 = assign14910_e20948_d_n7;
        locals.var_qhs_dn10 = assign14910_e20948_d_n10;
        locals.var_qhs_dn11 = assign14910_e20948_d_n11;
        locals.var_qhs_dn12 = assign14910_e20948_d_n12;
        locals.var_qhs_dn17 = assign14910_e20948_d_n17;
        locals.var_qhs_rv = 0.0;

        let (assign14920_e20956, assign14920_e20956_d_n0, assign14920_e20956_d_n2, assign14920_e20956_d_n6, assign14920_e20956_d_n7, assign14920_e20956_d_n10, assign14920_e20956_d_n11, assign14920_e20956_d_n12, assign14920_e20956_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard446 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign14920_e20956;
        locals.var_qhs_dn0 = assign14920_e20956_d_n0;
        locals.var_qhs_dn2 = assign14920_e20956_d_n2;
        locals.var_qhs_dn6 = assign14920_e20956_d_n6;
        locals.var_qhs_dn7 = assign14920_e20956_d_n7;
        locals.var_qhs_dn10 = assign14920_e20956_d_n10;
        locals.var_qhs_dn11 = assign14920_e20956_d_n11;
        locals.var_qhs_dn12 = assign14920_e20956_d_n12;
        locals.var_qhs_dn17 = assign14920_e20956_d_n17;
        locals.var_qhs_rv = 0.0;

        let (assign14940_e20969, assign14940_e20969_d_n0, assign14940_e20969_d_n2, assign14940_e20969_d_n6, assign14940_e20969_d_n7, assign14940_e20969_d_n10, assign14940_e20969_d_n11, assign14940_e20969_d_n12, assign14940_e20969_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14940_e20966: f64 = (locals.var_beta * locals.var_vbcs_cl);
        let assign14940_e20967: f64 = (assign14940_e20966).exp();
        (assign14940_e20967, (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn0)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn2)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn6)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn7)), (assign14940_e20967 * ((locals.var_beta_dn10 * locals.var_vbcs_cl) + (locals.var_beta * locals.var_vbcs_cl_dn10))), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn11)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn12)), (assign14940_e20967 * (locals.var_beta * locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12, locals.var_exp_bvbs_dn17,)
    }
};
        locals.var_exp_bvbs = assign14940_e20969;
        locals.var_exp_bvbs_dn0 = assign14940_e20969_d_n0;
        locals.var_exp_bvbs_dn2 = assign14940_e20969_d_n2;
        locals.var_exp_bvbs_dn6 = assign14940_e20969_d_n6;
        locals.var_exp_bvbs_dn7 = assign14940_e20969_d_n7;
        locals.var_exp_bvbs_dn10 = assign14940_e20969_d_n10;
        locals.var_exp_bvbs_dn11 = assign14940_e20969_d_n11;
        locals.var_exp_bvbs_dn12 = assign14940_e20969_d_n12;
        locals.var_exp_bvbs_dn17 = assign14940_e20969_d_n17;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign14950_e20976, assign14950_e20976_d_n0, assign14950_e20976_d_n2, assign14950_e20976_d_n6, assign14950_e20976_d_n7, assign14950_e20976_d_n10, assign14950_e20976_d_n11, assign14950_e20976_d_n12, assign14950_e20976_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14950_e20974: f64 = (locals.var_cnst1soi * locals.var_exp_bvbs);
        (assign14950_e20974, ((locals.var_cnst1soi_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1soi_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1soi_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1soi_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1soi_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1soi_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1soi_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn12)), ((locals.var_cnst1soi_dn17 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn17)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn12, locals.var_cfs1_dn17,)
    }
};
        locals.var_cfs1 = assign14950_e20976;
        locals.var_cfs1_dn0 = assign14950_e20976_d_n0;
        locals.var_cfs1_dn2 = assign14950_e20976_d_n2;
        locals.var_cfs1_dn6 = assign14950_e20976_d_n6;
        locals.var_cfs1_dn7 = assign14950_e20976_d_n7;
        locals.var_cfs1_dn10 = assign14950_e20976_d_n10;
        locals.var_cfs1_dn11 = assign14950_e20976_d_n11;
        locals.var_cfs1_dn12 = assign14950_e20976_d_n12;
        locals.var_cfs1_dn17 = assign14950_e20976_d_n17;
        locals.var_cfs1_rv = 0.0;

        let (assign14960_e20981,) = {
    if (locals.var_guard109 == 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign14960_e20981;
        locals.var_flg_conv_rv = 0.0;

        let (assign14970_e20986, assign14970_e20986_d_n0, assign14970_e20986_d_n2, assign14970_e20986_d_n6, assign14970_e20986_d_n7, assign14970_e20986_d_n10, assign14970_e20986_d_n11, assign14970_e20986_d_n12, assign14970_e20986_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14970_e20986;
        locals.var_phi_s0_soi_dn0 = assign14970_e20986_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14970_e20986_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14970_e20986_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14970_e20986_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14970_e20986_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14970_e20986_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14970_e20986_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14970_e20986_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign14980_e20999, assign14980_e20999_d_n0, assign14980_e20999_d_n2, assign14980_e20999_d_n6, assign14980_e20999_d_n7, assign14980_e20999_d_n10, assign14980_e20999_d_n11, assign14980_e20999_d_n12, assign14980_e20999_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14980_e20991: f64 = (locals.var_q_nsub * p.p237);
        let assign14980_e20993: f64 = (assign14980_e20991 * p.p237);
        let assign14980_e20995: f64 = (assign14980_e20993 / 2.0);
        let assign14980_e20997: f64 = (assign14980_e20995 / 1.034943e-10);
        (assign14980_e20997, ((((locals.var_q_nsub_dn0 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn12 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn17 * p.p237) * p.p237) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn12, locals.var_dphi_sb_dn17,)
    }
};
        locals.var_dphi_sb = assign14980_e20999;
        locals.var_dphi_sb_dn0 = assign14980_e20999_d_n0;
        locals.var_dphi_sb_dn2 = assign14980_e20999_d_n2;
        locals.var_dphi_sb_dn6 = assign14980_e20999_d_n6;
        locals.var_dphi_sb_dn7 = assign14980_e20999_d_n7;
        locals.var_dphi_sb_dn10 = assign14980_e20999_d_n10;
        locals.var_dphi_sb_dn11 = assign14980_e20999_d_n11;
        locals.var_dphi_sb_dn12 = assign14980_e20999_d_n12;
        locals.var_dphi_sb_dn17 = assign14980_e20999_d_n17;
        locals.var_dphi_sb_rv = 0.0;

        let (assign14990_e21009, assign14990_e21009_d_n0, assign14990_e21009_d_n2, assign14990_e21009_d_n6, assign14990_e21009_d_n7, assign14990_e21009_d_n10, assign14990_e21009_d_n11, assign14990_e21009_d_n12, assign14990_e21009_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign14990_e21004: f64 = (2.0 * locals.var_beta);
        let assign14990_e21006: f64 = (assign14990_e21004 * locals.var_dphi_sb);
        let assign14990_e21007: f64 = (assign14990_e21006).sqrt();
        (assign14990_e21007, ((assign14990_e21004 * locals.var_dphi_sb_dn0) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn2) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn6) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn7) / (2.0 * assign14990_e21007)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign14990_e21004 * locals.var_dphi_sb_dn10)) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn11) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn12) / (2.0 * assign14990_e21007)), ((assign14990_e21004 * locals.var_dphi_sb_dn17) / (2.0 * assign14990_e21007)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign14990_e21009;
        locals.var_t0_dn0 = assign14990_e21009_d_n0;
        locals.var_t0_dn2 = assign14990_e21009_d_n2;
        locals.var_t0_dn6 = assign14990_e21009_d_n6;
        locals.var_t0_dn7 = assign14990_e21009_d_n7;
        locals.var_t0_dn10 = assign14990_e21009_d_n10;
        locals.var_t0_dn11 = assign14990_e21009_d_n11;
        locals.var_t0_dn12 = assign14990_e21009_d_n12;
        locals.var_t0_dn17 = assign14990_e21009_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign15000_e21021, assign15000_e21021_d_n0, assign15000_e21021_d_n2, assign15000_e21021_d_n6, assign15000_e21021_d_n7, assign15000_e21021_d_n10, assign15000_e21021_d_n11, assign15000_e21021_d_n12, assign15000_e21021_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15000_e21013: f64 = (locals.var_t0).exp();
        let assign15000_e21015: f64 = (-locals.var_t0);
        let assign15000_e21016: f64 = (assign15000_e21015).exp();
        let assign15000_e21017: f64 = (assign15000_e21013 + assign15000_e21016);
        let assign15000_e21019: f64 = (assign15000_e21017 / 2.0);
        (assign15000_e21019, (((assign15000_e21013 * locals.var_t0_dn0) + (assign15000_e21016 * (-locals.var_t0_dn0))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn2) + (assign15000_e21016 * (-locals.var_t0_dn2))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn6) + (assign15000_e21016 * (-locals.var_t0_dn6))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn7) + (assign15000_e21016 * (-locals.var_t0_dn7))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn10) + (assign15000_e21016 * (-locals.var_t0_dn10))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn11) + (assign15000_e21016 * (-locals.var_t0_dn11))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn12) + (assign15000_e21016 * (-locals.var_t0_dn12))) / 2.0), (((assign15000_e21013 * locals.var_t0_dn17) + (assign15000_e21016 * (-locals.var_t0_dn17))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15000_e21021;
        locals.var_t1_dn0 = assign15000_e21021_d_n0;
        locals.var_t1_dn2 = assign15000_e21021_d_n2;
        locals.var_t1_dn6 = assign15000_e21021_d_n6;
        locals.var_t1_dn7 = assign15000_e21021_d_n7;
        locals.var_t1_dn10 = assign15000_e21021_d_n10;
        locals.var_t1_dn11 = assign15000_e21021_d_n11;
        locals.var_t1_dn12 = assign15000_e21021_d_n12;
        locals.var_t1_dn17 = assign15000_e21021_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15010_e21029, assign15010_e21029_d_n0, assign15010_e21029_d_n2, assign15010_e21029_d_n6, assign15010_e21029_d_n7, assign15010_e21029_d_n10, assign15010_e21029_d_n11, assign15010_e21029_d_n12, assign15010_e21029_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15010_e21025: f64 = (locals.var_t1).ln();
        let assign15010_e21027: f64 = (assign15010_e21025 / locals.var_dphi_sb);
        (assign15010_e21027, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn12 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn12)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn17 / locals.var_t1) * locals.var_dphi_sb) - (assign15010_e21025 * locals.var_dphi_sb_dn17)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn12, locals.var_c_sb_dn17,)
    }
};
        locals.var_c_sb = assign15010_e21029;
        locals.var_c_sb_dn0 = assign15010_e21029_d_n0;
        locals.var_c_sb_dn2 = assign15010_e21029_d_n2;
        locals.var_c_sb_dn6 = assign15010_e21029_d_n6;
        locals.var_c_sb_dn7 = assign15010_e21029_d_n7;
        locals.var_c_sb_dn10 = assign15010_e21029_d_n10;
        locals.var_c_sb_dn11 = assign15010_e21029_d_n11;
        locals.var_c_sb_dn12 = assign15010_e21029_d_n12;
        locals.var_c_sb_dn17 = assign15010_e21029_d_n17;
        locals.var_c_sb_rv = 0.0;

        let (assign15020_e21034,) = {
    if (locals.var_guard109 == 0.0) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign15020_e21034;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign15030_loop_guard: usize = 0;
        while {
            let assign15030_cond_e21040: f64 = (locals.var_lp_s0_max + 1.0);
            let assign15030_cond_e21042: f64 = if ((locals.var_guard109 == 0.0) && (locals.var_lp_s0 <= assign15030_cond_e21040)) { 1.0 } else { 0.0 };
            assign15030_cond_e21042 != 0.0
        } {
            assign15030_loop_guard += 1;
            assert!(assign15030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15030_body0_e21049, assign15030_body0_e21049_d_n0, assign15030_body0_e21049_d_n2, assign15030_body0_e21049_d_n6, assign15030_body0_e21049_d_n7, assign15030_body0_e21049_d_n10, assign15030_body0_e21049_d_n11, assign15030_body0_e21049_d_n12, assign15030_body0_e21049_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body0_e21047: f64 = (locals.var_phi_s0_soi - locals.var_vbcs_cl);
        (assign15030_body0_e21047, (locals.var_phi_s0_soi_dn0 - locals.var_vbcs_cl_dn0), (locals.var_phi_s0_soi_dn2 - locals.var_vbcs_cl_dn2), (locals.var_phi_s0_soi_dn6 - locals.var_vbcs_cl_dn6), (locals.var_phi_s0_soi_dn7 - locals.var_vbcs_cl_dn7), (locals.var_phi_s0_soi_dn10 - locals.var_vbcs_cl_dn10), (locals.var_phi_s0_soi_dn11 - locals.var_vbcs_cl_dn11), (locals.var_phi_s0_soi_dn12 - locals.var_vbcs_cl_dn12), (locals.var_phi_s0_soi_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_phi_soi0, locals.var_phi_soi0_dn0, locals.var_phi_soi0_dn2, locals.var_phi_soi0_dn6, locals.var_phi_soi0_dn7, locals.var_phi_soi0_dn10, locals.var_phi_soi0_dn11, locals.var_phi_soi0_dn12, locals.var_phi_soi0_dn17,)
    }
};
            locals.var_phi_soi0 = assign15030_body0_e21049;
            locals.var_phi_soi0_dn0 = assign15030_body0_e21049_d_n0;
            locals.var_phi_soi0_dn2 = assign15030_body0_e21049_d_n2;
            locals.var_phi_soi0_dn6 = assign15030_body0_e21049_d_n6;
            locals.var_phi_soi0_dn7 = assign15030_body0_e21049_d_n7;
            locals.var_phi_soi0_dn10 = assign15030_body0_e21049_d_n10;
            locals.var_phi_soi0_dn11 = assign15030_body0_e21049_d_n11;
            locals.var_phi_soi0_dn12 = assign15030_body0_e21049_d_n12;
            locals.var_phi_soi0_dn17 = assign15030_body0_e21049_d_n17;
            locals.var_phi_soi0_rv = 0.0;
            let (assign15030_body1_e21056, assign15030_body1_e21056_d_n0, assign15030_body1_e21056_d_n2, assign15030_body1_e21056_d_n6, assign15030_body1_e21056_d_n7, assign15030_body1_e21056_d_n10, assign15030_body1_e21056_d_n11, assign15030_body1_e21056_d_n12, assign15030_body1_e21056_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body1_e21054: f64 = (locals.var_beta * locals.var_phi_soi0);
        (assign15030_body1_e21054, (locals.var_beta * locals.var_phi_soi0_dn0), (locals.var_beta * locals.var_phi_soi0_dn2), (locals.var_beta * locals.var_phi_soi0_dn6), (locals.var_beta * locals.var_phi_soi0_dn7), ((locals.var_beta_dn10 * locals.var_phi_soi0) + (locals.var_beta * locals.var_phi_soi0_dn10)), (locals.var_beta * locals.var_phi_soi0_dn11), (locals.var_beta * locals.var_phi_soi0_dn12), (locals.var_beta * locals.var_phi_soi0_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign15030_body1_e21056;
            locals.var_chi_dn0 = assign15030_body1_e21056_d_n0;
            locals.var_chi_dn2 = assign15030_body1_e21056_d_n2;
            locals.var_chi_dn6 = assign15030_body1_e21056_d_n6;
            locals.var_chi_dn7 = assign15030_body1_e21056_d_n7;
            locals.var_chi_dn10 = assign15030_body1_e21056_d_n10;
            locals.var_chi_dn11 = assign15030_body1_e21056_d_n11;
            locals.var_chi_dn12 = assign15030_body1_e21056_d_n12;
            locals.var_chi_dn17 = assign15030_body1_e21056_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign15030_body2_e21065, assign15030_body2_e21065_d_n0, assign15030_body2_e21065_d_n2, assign15030_body2_e21065_d_n6, assign15030_body2_e21065_d_n7, assign15030_body2_e21065_d_n10, assign15030_body2_e21065_d_n11, assign15030_body2_e21065_d_n12, assign15030_body2_e21065_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body2_e21062: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        let assign15030_body2_e21063: f64 = (locals.var_c_sb * assign15030_body2_e21062);
        (assign15030_body2_e21063, ((locals.var_c_sb_dn0 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign15030_body2_e21062) + (locals.var_c_sb * (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign15030_body2_e21065;
            locals.var_ty_dn0 = assign15030_body2_e21065_d_n0;
            locals.var_ty_dn2 = assign15030_body2_e21065_d_n2;
            locals.var_ty_dn6 = assign15030_body2_e21065_d_n6;
            locals.var_ty_dn7 = assign15030_body2_e21065_d_n7;
            locals.var_ty_dn10 = assign15030_body2_e21065_d_n10;
            locals.var_ty_dn11 = assign15030_body2_e21065_d_n11;
            locals.var_ty_dn12 = assign15030_body2_e21065_d_n12;
            locals.var_ty_dn17 = assign15030_body2_e21065_d_n17;
            locals.var_ty_rv = 0.0;
            let assign15030_body3_e21068: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard447 = assign15030_body3_e21068;
            locals.var_guard447_rv = 0.0;
            let (assign15030_body4_e21076, assign15030_body4_e21076_d_n0, assign15030_body4_e21076_d_n2, assign15030_body4_e21076_d_n6, assign15030_body4_e21076_d_n7, assign15030_body4_e21076_d_n10, assign15030_body4_e21076_d_n11, assign15030_body4_e21076_d_n12, assign15030_body4_e21076_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body4_e21074: f64 = (locals.var_ty).exp();
        (assign15030_body4_e21074, (assign15030_body4_e21074 * locals.var_ty_dn0), (assign15030_body4_e21074 * locals.var_ty_dn2), (assign15030_body4_e21074 * locals.var_ty_dn6), (assign15030_body4_e21074 * locals.var_ty_dn7), (assign15030_body4_e21074 * locals.var_ty_dn10), (assign15030_body4_e21074 * locals.var_ty_dn11), (assign15030_body4_e21074 * locals.var_ty_dn12), (assign15030_body4_e21074 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15030_body4_e21076;
            locals.var_t1_dn0 = assign15030_body4_e21076_d_n0;
            locals.var_t1_dn2 = assign15030_body4_e21076_d_n2;
            locals.var_t1_dn6 = assign15030_body4_e21076_d_n6;
            locals.var_t1_dn7 = assign15030_body4_e21076_d_n7;
            locals.var_t1_dn10 = assign15030_body4_e21076_d_n10;
            locals.var_t1_dn11 = assign15030_body4_e21076_d_n11;
            locals.var_t1_dn12 = assign15030_body4_e21076_d_n12;
            locals.var_t1_dn17 = assign15030_body4_e21076_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15030_body5_e21087, assign15030_body5_e21087_d_n0, assign15030_body5_e21087_d_n2, assign15030_body5_e21087_d_n6, assign15030_body5_e21087_d_n7, assign15030_body5_e21087_d_n10, assign15030_body5_e21087_d_n11, assign15030_body5_e21087_d_n12, assign15030_body5_e21087_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body5_e21082: f64 = (-locals.var_c_sb);
        let assign15030_body5_e21084: f64 = (assign15030_body5_e21082 * locals.var_dphi_sb);
        let assign15030_body5_e21085: f64 = (assign15030_body5_e21084).exp();
        (assign15030_body5_e21085, (assign15030_body5_e21085 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn0))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn2))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn6))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn7))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn10))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn11))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn12))), (assign15030_body5_e21085 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign15030_body5_e21082 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15030_body5_e21087;
            locals.var_t0_dn0 = assign15030_body5_e21087_d_n0;
            locals.var_t0_dn2 = assign15030_body5_e21087_d_n2;
            locals.var_t0_dn6 = assign15030_body5_e21087_d_n6;
            locals.var_t0_dn7 = assign15030_body5_e21087_d_n7;
            locals.var_t0_dn10 = assign15030_body5_e21087_d_n10;
            locals.var_t0_dn11 = assign15030_body5_e21087_d_n11;
            locals.var_t0_dn12 = assign15030_body5_e21087_d_n12;
            locals.var_t0_dn17 = assign15030_body5_e21087_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15030_body6_e21096, assign15030_body6_e21096_d_n0, assign15030_body6_e21096_d_n2, assign15030_body6_e21096_d_n6, assign15030_body6_e21096_d_n7, assign15030_body6_e21096_d_n10, assign15030_body6_e21096_d_n11, assign15030_body6_e21096_d_n12, assign15030_body6_e21096_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body6_e21094: f64 = (locals.var_t1 - locals.var_t0);
        (assign15030_body6_e21094, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign15030_body6_e21096;
            locals.var_t2_dn0 = assign15030_body6_e21096_d_n0;
            locals.var_t2_dn2 = assign15030_body6_e21096_d_n2;
            locals.var_t2_dn6 = assign15030_body6_e21096_d_n6;
            locals.var_t2_dn7 = assign15030_body6_e21096_d_n7;
            locals.var_t2_dn10 = assign15030_body6_e21096_d_n10;
            locals.var_t2_dn11 = assign15030_body6_e21096_d_n11;
            locals.var_t2_dn12 = assign15030_body6_e21096_d_n12;
            locals.var_t2_dn17 = assign15030_body6_e21096_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign15030_body7_e21108, assign15030_body7_e21108_d_n0, assign15030_body7_e21108_d_n2, assign15030_body7_e21108_d_n6, assign15030_body7_e21108_d_n7, assign15030_body7_e21108_d_n10, assign15030_body7_e21108_d_n11, assign15030_body7_e21108_d_n12, assign15030_body7_e21108_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body7_e21103: f64 = (1.0 + locals.var_t2);
        let assign15030_body7_e21104: f64 = (assign15030_body7_e21103).ln();
        let assign15030_body7_e21106: f64 = (assign15030_body7_e21104 / locals.var_c_sb);
        (assign15030_body7_e21106, ((((locals.var_t2_dn0 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign15030_body7_e21103) * locals.var_c_sb) - (assign15030_body7_e21104 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign15030_body7_e21108;
            locals.var_phi_soib_dn0 = assign15030_body7_e21108_d_n0;
            locals.var_phi_soib_dn2 = assign15030_body7_e21108_d_n2;
            locals.var_phi_soib_dn6 = assign15030_body7_e21108_d_n6;
            locals.var_phi_soib_dn7 = assign15030_body7_e21108_d_n7;
            locals.var_phi_soib_dn10 = assign15030_body7_e21108_d_n10;
            locals.var_phi_soib_dn11 = assign15030_body7_e21108_d_n11;
            locals.var_phi_soib_dn12 = assign15030_body7_e21108_d_n12;
            locals.var_phi_soib_dn17 = assign15030_body7_e21108_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign15030_body8_e21119, assign15030_body8_e21119_d_n0, assign15030_body8_e21119_d_n2, assign15030_body8_e21119_d_n6, assign15030_body8_e21119_d_n7, assign15030_body8_e21119_d_n10, assign15030_body8_e21119_d_n11, assign15030_body8_e21119_d_n12, assign15030_body8_e21119_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 != 0.0)) {
        let assign15030_body8_e21116: f64 = (1.0 + locals.var_t2);
        let assign15030_body8_e21117: f64 = (locals.var_t1 / assign15030_body8_e21116);
        (assign15030_body8_e21117, (((locals.var_t1_dn0 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn0)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn2 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn2)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn6 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn6)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn7 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn7)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn10 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn10)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn11 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn11)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn12 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn12)) / (assign15030_body8_e21116 * assign15030_body8_e21116)), (((locals.var_t1_dn17 * assign15030_body8_e21116) - (locals.var_t1 * locals.var_t2_dn17)) / (assign15030_body8_e21116 * assign15030_body8_e21116)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign15030_body8_e21119;
            locals.var_phi_soib_dpss_dn0 = assign15030_body8_e21119_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign15030_body8_e21119_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign15030_body8_e21119_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign15030_body8_e21119_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign15030_body8_e21119_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign15030_body8_e21119_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign15030_body8_e21119_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign15030_body8_e21119_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign15030_body9_e21129, assign15030_body9_e21129_d_n0, assign15030_body9_e21129_d_n2, assign15030_body9_e21129_d_n6, assign15030_body9_e21129_d_n7, assign15030_body9_e21129_d_n10, assign15030_body9_e21129_d_n11, assign15030_body9_e21129_d_n12, assign15030_body9_e21129_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 == 0.0)) {
        let assign15030_body9_e21127: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        (assign15030_body9_e21127, (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign15030_body9_e21129;
            locals.var_phi_soib_dn0 = assign15030_body9_e21129_d_n0;
            locals.var_phi_soib_dn2 = assign15030_body9_e21129_d_n2;
            locals.var_phi_soib_dn6 = assign15030_body9_e21129_d_n6;
            locals.var_phi_soib_dn7 = assign15030_body9_e21129_d_n7;
            locals.var_phi_soib_dn10 = assign15030_body9_e21129_d_n10;
            locals.var_phi_soib_dn11 = assign15030_body9_e21129_d_n11;
            locals.var_phi_soib_dn12 = assign15030_body9_e21129_d_n12;
            locals.var_phi_soib_dn17 = assign15030_body9_e21129_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign15030_body10_e21137, assign15030_body10_e21137_d_n0, assign15030_body10_e21137_d_n2, assign15030_body10_e21137_d_n6, assign15030_body10_e21137_d_n7, assign15030_body10_e21137_d_n10, assign15030_body10_e21137_d_n11, assign15030_body10_e21137_d_n12, assign15030_body10_e21137_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard447 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign15030_body10_e21137;
            locals.var_phi_soib_dpss_dn0 = assign15030_body10_e21137_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign15030_body10_e21137_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign15030_body10_e21137_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign15030_body10_e21137_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign15030_body10_e21137_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign15030_body10_e21137_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign15030_body10_e21137_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign15030_body10_e21137_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign15030_body11_e21144, assign15030_body11_e21144_d_n0, assign15030_body11_e21144_d_n2, assign15030_body11_e21144_d_n6, assign15030_body11_e21144_d_n7, assign15030_body11_e21144_d_n10, assign15030_body11_e21144_d_n11, assign15030_body11_e21144_d_n12, assign15030_body11_e21144_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body11_e21142: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign15030_body11_e21142, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign15030_body11_e21144;
            locals.var_chib_dn0 = assign15030_body11_e21144_d_n0;
            locals.var_chib_dn2 = assign15030_body11_e21144_d_n2;
            locals.var_chib_dn6 = assign15030_body11_e21144_d_n6;
            locals.var_chib_dn7 = assign15030_body11_e21144_d_n7;
            locals.var_chib_dn10 = assign15030_body11_e21144_d_n10;
            locals.var_chib_dn11 = assign15030_body11_e21144_d_n11;
            locals.var_chib_dn12 = assign15030_body11_e21144_d_n12;
            locals.var_chib_dn17 = assign15030_body11_e21144_d_n17;
            locals.var_chib_rv = 0.0;
            let assign15030_body12_e21146: f64 = (locals.var_chi).abs();
            let assign15030_body12_e21148: f64 = if assign15030_body12_e21146 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard448 = assign15030_body12_e21148;
            locals.var_guard448_rv = 0.0;
            let (assign15030_body13_e21162, assign15030_body13_e21162_d_n0, assign15030_body13_e21162_d_n2, assign15030_body13_e21162_d_n6, assign15030_body13_e21162_d_n7, assign15030_body13_e21162_d_n10, assign15030_body13_e21162_d_n11, assign15030_body13_e21162_d_n12, assign15030_body13_e21162_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) {
        let assign15030_body13_e21156: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign15030_body13_e21157: f64 = (1.0 - assign15030_body13_e21156);
        let assign15030_body13_e21159: f64 = (assign15030_body13_e21157 / 2.0);
        let assign15030_body13_e21160: f64 = (assign15030_body13_e21159).sqrt();
        (assign15030_body13_e21160, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign15030_body13_e21160)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign15030_body13_e21160)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15030_body13_e21162;
            locals.var_t0_dn0 = assign15030_body13_e21162_d_n0;
            locals.var_t0_dn2 = assign15030_body13_e21162_d_n2;
            locals.var_t0_dn6 = assign15030_body13_e21162_d_n6;
            locals.var_t0_dn7 = assign15030_body13_e21162_d_n7;
            locals.var_t0_dn10 = assign15030_body13_e21162_d_n10;
            locals.var_t0_dn11 = assign15030_body13_e21162_d_n11;
            locals.var_t0_dn12 = assign15030_body13_e21162_d_n12;
            locals.var_t0_dn17 = assign15030_body13_e21162_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15030_body14_e21171, assign15030_body14_e21171_d_n0, assign15030_body14_e21171_d_n2, assign15030_body14_e21171_d_n6, assign15030_body14_e21171_d_n7, assign15030_body14_e21171_d_n10, assign15030_body14_e21171_d_n11, assign15030_body14_e21171_d_n12, assign15030_body14_e21171_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) {
        let assign15030_body14_e21169: f64 = (locals.var_chi * locals.var_t0);
        (assign15030_body14_e21169, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15030_body14_e21171;
            locals.var_fb_dn0 = assign15030_body14_e21171_d_n0;
            locals.var_fb_dn2 = assign15030_body14_e21171_d_n2;
            locals.var_fb_dn6 = assign15030_body14_e21171_d_n6;
            locals.var_fb_dn7 = assign15030_body14_e21171_d_n7;
            locals.var_fb_dn10 = assign15030_body14_e21171_d_n10;
            locals.var_fb_dn11 = assign15030_body14_e21171_d_n11;
            locals.var_fb_dn12 = assign15030_body14_e21171_d_n12;
            locals.var_fb_dn17 = assign15030_body14_e21171_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15030_body15_e21180, assign15030_body15_e21180_d_n0, assign15030_body15_e21180_d_n2, assign15030_body15_e21180_d_n6, assign15030_body15_e21180_d_n7, assign15030_body15_e21180_d_n10, assign15030_body15_e21180_d_n11, assign15030_body15_e21180_d_n12, assign15030_body15_e21180_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) {
        let assign15030_body15_e21178: f64 = (locals.var_beta * locals.var_t0);
        (assign15030_body15_e21178, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15030_body15_e21180;
            locals.var_fb_dpss_dn0 = assign15030_body15_e21180_d_n0;
            locals.var_fb_dpss_dn2 = assign15030_body15_e21180_d_n2;
            locals.var_fb_dpss_dn6 = assign15030_body15_e21180_d_n6;
            locals.var_fb_dpss_dn7 = assign15030_body15_e21180_d_n7;
            locals.var_fb_dpss_dn10 = assign15030_body15_e21180_d_n10;
            locals.var_fb_dpss_dn11 = assign15030_body15_e21180_d_n11;
            locals.var_fb_dpss_dn12 = assign15030_body15_e21180_d_n12;
            locals.var_fb_dpss_dn17 = assign15030_body15_e21180_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15030_body16_e21183: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard449 = assign15030_body16_e21183;
            locals.var_guard449_rv = 0.0;
            let (assign15030_body17_e21193, assign15030_body17_e21193_d_n0, assign15030_body17_e21193_d_n2, assign15030_body17_e21193_d_n6, assign15030_body17_e21193_d_n7, assign15030_body17_e21193_d_n10, assign15030_body17_e21193_d_n11, assign15030_body17_e21193_d_n12, assign15030_body17_e21193_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard449 != 0.0)) {
        let assign15030_body17_e21191: f64 = (-locals.var_fb);
        (assign15030_body17_e21191, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15030_body17_e21193;
            locals.var_fb_dn0 = assign15030_body17_e21193_d_n0;
            locals.var_fb_dn2 = assign15030_body17_e21193_d_n2;
            locals.var_fb_dn6 = assign15030_body17_e21193_d_n6;
            locals.var_fb_dn7 = assign15030_body17_e21193_d_n7;
            locals.var_fb_dn10 = assign15030_body17_e21193_d_n10;
            locals.var_fb_dn11 = assign15030_body17_e21193_d_n11;
            locals.var_fb_dn12 = assign15030_body17_e21193_d_n12;
            locals.var_fb_dn17 = assign15030_body17_e21193_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15030_body18_e21203, assign15030_body18_e21203_d_n0, assign15030_body18_e21203_d_n2, assign15030_body18_e21203_d_n6, assign15030_body18_e21203_d_n7, assign15030_body18_e21203_d_n10, assign15030_body18_e21203_d_n11, assign15030_body18_e21203_d_n12, assign15030_body18_e21203_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard449 != 0.0)) {
        let assign15030_body18_e21201: f64 = (-locals.var_fb_dpss);
        (assign15030_body18_e21201, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15030_body18_e21203;
            locals.var_fb_dpss_dn0 = assign15030_body18_e21203_d_n0;
            locals.var_fb_dpss_dn2 = assign15030_body18_e21203_d_n2;
            locals.var_fb_dpss_dn6 = assign15030_body18_e21203_d_n6;
            locals.var_fb_dpss_dn7 = assign15030_body18_e21203_d_n7;
            locals.var_fb_dpss_dn10 = assign15030_body18_e21203_d_n10;
            locals.var_fb_dpss_dn11 = assign15030_body18_e21203_d_n11;
            locals.var_fb_dpss_dn12 = assign15030_body18_e21203_d_n12;
            locals.var_fb_dpss_dn17 = assign15030_body18_e21203_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15030_body19_e21205: f64 = (locals.var_chi).abs();
            let assign15030_body19_e21207: f64 = if assign15030_body19_e21205 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard450 = assign15030_body19_e21207;
            locals.var_guard450_rv = 0.0;
            let (assign15030_body20_e21239, assign15030_body20_e21239_d_n0, assign15030_body20_e21239_d_n2, assign15030_body20_e21239_d_n6, assign15030_body20_e21239_d_n7, assign15030_body20_e21239_d_n10, assign15030_body20_e21239_d_n11, assign15030_body20_e21239_d_n12, assign15030_body20_e21239_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body20_e21217: f64 = (locals.var_chi * locals.var_chi);
        let assign15030_body20_e21219: f64 = (assign15030_body20_e21217 / 2.0);
        let assign15030_body20_e21223: f64 = (locals.var_chi / 3.0);
        let assign15030_body20_e21227: f64 = (locals.var_chi / 4.0);
        let assign15030_body20_e21231: f64 = (locals.var_chi / 5.0);
        let assign15030_body20_e21232: f64 = (1.0 - assign15030_body20_e21231);
        let assign15030_body20_e21233: f64 = (assign15030_body20_e21227 * assign15030_body20_e21232);
        let assign15030_body20_e21234: f64 = (1.0 - assign15030_body20_e21233);
        let assign15030_body20_e21235: f64 = (assign15030_body20_e21223 * assign15030_body20_e21234);
        let assign15030_body20_e21236: f64 = (1.0 - assign15030_body20_e21235);
        let assign15030_body20_e21237: f64 = (assign15030_body20_e21219 * assign15030_body20_e21236);
        (assign15030_body20_e21237, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn0 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn0 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn2 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn2 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn6 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn6 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn7 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn7 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn10 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn10 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn11 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn11 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn12 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn12 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign15030_body20_e21236) + (assign15030_body20_e21219 * (-(((locals.var_chi_dn17 / 3.0) * assign15030_body20_e21234) + (assign15030_body20_e21223 * (-(((locals.var_chi_dn17 / 4.0) * assign15030_body20_e21232) + (assign15030_body20_e21227 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15030_body20_e21239;
            locals.var_t0_dn0 = assign15030_body20_e21239_d_n0;
            locals.var_t0_dn2 = assign15030_body20_e21239_d_n2;
            locals.var_t0_dn6 = assign15030_body20_e21239_d_n6;
            locals.var_t0_dn7 = assign15030_body20_e21239_d_n7;
            locals.var_t0_dn10 = assign15030_body20_e21239_d_n10;
            locals.var_t0_dn11 = assign15030_body20_e21239_d_n11;
            locals.var_t0_dn12 = assign15030_body20_e21239_d_n12;
            locals.var_t0_dn17 = assign15030_body20_e21239_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15030_body21_e21267, assign15030_body21_e21267_d_n0, assign15030_body21_e21267_d_n2, assign15030_body21_e21267_d_n6, assign15030_body21_e21267_d_n7, assign15030_body21_e21267_d_n10, assign15030_body21_e21267_d_n11, assign15030_body21_e21267_d_n12, assign15030_body21_e21267_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body21_e21251: f64 = (locals.var_chi / 2.0);
        let assign15030_body21_e21255: f64 = (locals.var_chi / 3.0);
        let assign15030_body21_e21259: f64 = (locals.var_chi / 4.0);
        let assign15030_body21_e21260: f64 = (1.0 - assign15030_body21_e21259);
        let assign15030_body21_e21261: f64 = (assign15030_body21_e21255 * assign15030_body21_e21260);
        let assign15030_body21_e21262: f64 = (1.0 - assign15030_body21_e21261);
        let assign15030_body21_e21263: f64 = (assign15030_body21_e21251 * assign15030_body21_e21262);
        let assign15030_body21_e21264: f64 = (1.0 - assign15030_body21_e21263);
        let assign15030_body21_e21265: f64 = (locals.var_chi * assign15030_body21_e21264);
        (assign15030_body21_e21265, ((locals.var_chi_dn0 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn0 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn2 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn6 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn7 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn10 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn11 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn12 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign15030_body21_e21264) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign15030_body21_e21262) + (assign15030_body21_e21251 * (-(((locals.var_chi_dn17 / 3.0) * assign15030_body21_e21260) + (assign15030_body21_e21255 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15030_body21_e21267;
            locals.var_t1_dn0 = assign15030_body21_e21267_d_n0;
            locals.var_t1_dn2 = assign15030_body21_e21267_d_n2;
            locals.var_t1_dn6 = assign15030_body21_e21267_d_n6;
            locals.var_t1_dn7 = assign15030_body21_e21267_d_n7;
            locals.var_t1_dn10 = assign15030_body21_e21267_d_n10;
            locals.var_t1_dn11 = assign15030_body21_e21267_d_n11;
            locals.var_t1_dn12 = assign15030_body21_e21267_d_n12;
            locals.var_t1_dn17 = assign15030_body21_e21267_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15030_body22_e21299, assign15030_body22_e21299_d_n0, assign15030_body22_e21299_d_n2, assign15030_body22_e21299_d_n6, assign15030_body22_e21299_d_n7, assign15030_body22_e21299_d_n10, assign15030_body22_e21299_d_n11, assign15030_body22_e21299_d_n12, assign15030_body22_e21299_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body22_e21277: f64 = (locals.var_chib * locals.var_chib);
        let assign15030_body22_e21279: f64 = (assign15030_body22_e21277 / 2.0);
        let assign15030_body22_e21283: f64 = (locals.var_chib / 3.0);
        let assign15030_body22_e21287: f64 = (locals.var_chib / 4.0);
        let assign15030_body22_e21291: f64 = (locals.var_chib / 5.0);
        let assign15030_body22_e21292: f64 = (1.0 - assign15030_body22_e21291);
        let assign15030_body22_e21293: f64 = (assign15030_body22_e21287 * assign15030_body22_e21292);
        let assign15030_body22_e21294: f64 = (1.0 - assign15030_body22_e21293);
        let assign15030_body22_e21295: f64 = (assign15030_body22_e21283 * assign15030_body22_e21294);
        let assign15030_body22_e21296: f64 = (1.0 - assign15030_body22_e21295);
        let assign15030_body22_e21297: f64 = (assign15030_body22_e21279 * assign15030_body22_e21296);
        (assign15030_body22_e21297, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn0 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn0 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn2 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn2 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn6 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn6 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn7 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn7 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn10 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn10 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn11 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn11 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn12 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn12 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign15030_body22_e21296) + (assign15030_body22_e21279 * (-(((locals.var_chib_dn17 / 3.0) * assign15030_body22_e21294) + (assign15030_body22_e21283 * (-(((locals.var_chib_dn17 / 4.0) * assign15030_body22_e21292) + (assign15030_body22_e21287 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign15030_body22_e21299;
            locals.var_t2_dn0 = assign15030_body22_e21299_d_n0;
            locals.var_t2_dn2 = assign15030_body22_e21299_d_n2;
            locals.var_t2_dn6 = assign15030_body22_e21299_d_n6;
            locals.var_t2_dn7 = assign15030_body22_e21299_d_n7;
            locals.var_t2_dn10 = assign15030_body22_e21299_d_n10;
            locals.var_t2_dn11 = assign15030_body22_e21299_d_n11;
            locals.var_t2_dn12 = assign15030_body22_e21299_d_n12;
            locals.var_t2_dn17 = assign15030_body22_e21299_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign15030_body23_e21327, assign15030_body23_e21327_d_n0, assign15030_body23_e21327_d_n2, assign15030_body23_e21327_d_n6, assign15030_body23_e21327_d_n7, assign15030_body23_e21327_d_n10, assign15030_body23_e21327_d_n11, assign15030_body23_e21327_d_n12, assign15030_body23_e21327_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body23_e21311: f64 = (locals.var_chib / 2.0);
        let assign15030_body23_e21315: f64 = (locals.var_chib / 3.0);
        let assign15030_body23_e21319: f64 = (locals.var_chib / 4.0);
        let assign15030_body23_e21320: f64 = (1.0 - assign15030_body23_e21319);
        let assign15030_body23_e21321: f64 = (assign15030_body23_e21315 * assign15030_body23_e21320);
        let assign15030_body23_e21322: f64 = (1.0 - assign15030_body23_e21321);
        let assign15030_body23_e21323: f64 = (assign15030_body23_e21311 * assign15030_body23_e21322);
        let assign15030_body23_e21324: f64 = (1.0 - assign15030_body23_e21323);
        let assign15030_body23_e21325: f64 = (locals.var_chib * assign15030_body23_e21324);
        (assign15030_body23_e21325, ((locals.var_chib_dn0 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn0 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn2 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn6 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn7 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn10 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn11 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn12 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign15030_body23_e21324) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign15030_body23_e21322) + (assign15030_body23_e21311 * (-(((locals.var_chib_dn17 / 3.0) * assign15030_body23_e21320) + (assign15030_body23_e21315 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign15030_body23_e21327;
            locals.var_t3_dn0 = assign15030_body23_e21327_d_n0;
            locals.var_t3_dn2 = assign15030_body23_e21327_d_n2;
            locals.var_t3_dn6 = assign15030_body23_e21327_d_n6;
            locals.var_t3_dn7 = assign15030_body23_e21327_d_n7;
            locals.var_t3_dn10 = assign15030_body23_e21327_d_n10;
            locals.var_t3_dn11 = assign15030_body23_e21327_d_n11;
            locals.var_t3_dn12 = assign15030_body23_e21327_d_n12;
            locals.var_t3_dn17 = assign15030_body23_e21327_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign15030_body24_e21340, assign15030_body24_e21340_d_n0, assign15030_body24_e21340_d_n2, assign15030_body24_e21340_d_n6, assign15030_body24_e21340_d_n7, assign15030_body24_e21340_d_n10, assign15030_body24_e21340_d_n11, assign15030_body24_e21340_d_n12, assign15030_body24_e21340_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body24_e21337: f64 = (locals.var_t0 - locals.var_t2);
        let assign15030_body24_e21338: f64 = (assign15030_body24_e21337).sqrt();
        (assign15030_body24_e21338, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign15030_body24_e21338)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign15030_body24_e21338)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15030_body24_e21340;
            locals.var_fb_dn0 = assign15030_body24_e21340_d_n0;
            locals.var_fb_dn2 = assign15030_body24_e21340_d_n2;
            locals.var_fb_dn6 = assign15030_body24_e21340_d_n6;
            locals.var_fb_dn7 = assign15030_body24_e21340_d_n7;
            locals.var_fb_dn10 = assign15030_body24_e21340_d_n10;
            locals.var_fb_dn11 = assign15030_body24_e21340_d_n11;
            locals.var_fb_dn12 = assign15030_body24_e21340_d_n12;
            locals.var_fb_dn17 = assign15030_body24_e21340_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15030_body25_e21360, assign15030_body25_e21360_d_n0, assign15030_body25_e21360_d_n2, assign15030_body25_e21360_d_n6, assign15030_body25_e21360_d_n7, assign15030_body25_e21360_d_n10, assign15030_body25_e21360_d_n11, assign15030_body25_e21360_d_n12, assign15030_body25_e21360_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 != 0.0)) {
        let assign15030_body25_e21350: f64 = (locals.var_beta * 0.5);
        let assign15030_body25_e21354: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign15030_body25_e21355: f64 = (locals.var_t1 - assign15030_body25_e21354);
        let assign15030_body25_e21356: f64 = (assign15030_body25_e21350 * assign15030_body25_e21355);
        let assign15030_body25_e21358: f64 = (assign15030_body25_e21356 / locals.var_fb);
        (assign15030_body25_e21358, ((((assign15030_body25_e21350 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign15030_body25_e21355) + (assign15030_body25_e21350 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body25_e21350 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign15030_body25_e21356 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15030_body25_e21360;
            locals.var_fb_dpss_dn0 = assign15030_body25_e21360_d_n0;
            locals.var_fb_dpss_dn2 = assign15030_body25_e21360_d_n2;
            locals.var_fb_dpss_dn6 = assign15030_body25_e21360_d_n6;
            locals.var_fb_dpss_dn7 = assign15030_body25_e21360_d_n7;
            locals.var_fb_dpss_dn10 = assign15030_body25_e21360_d_n10;
            locals.var_fb_dpss_dn11 = assign15030_body25_e21360_d_n11;
            locals.var_fb_dpss_dn12 = assign15030_body25_e21360_d_n12;
            locals.var_fb_dpss_dn17 = assign15030_body25_e21360_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign15030_body26_e21373, assign15030_body26_e21373_d_n0, assign15030_body26_e21373_d_n2, assign15030_body26_e21373_d_n6, assign15030_body26_e21373_d_n7, assign15030_body26_e21373_d_n10, assign15030_body26_e21373_d_n11, assign15030_body26_e21373_d_n12, assign15030_body26_e21373_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) {
        let assign15030_body26_e21370: f64 = (-locals.var_chi);
        let assign15030_body26_e21371: f64 = (assign15030_body26_e21370).exp();
        (assign15030_body26_e21371, (assign15030_body26_e21371 * (-locals.var_chi_dn0)), (assign15030_body26_e21371 * (-locals.var_chi_dn2)), (assign15030_body26_e21371 * (-locals.var_chi_dn6)), (assign15030_body26_e21371 * (-locals.var_chi_dn7)), (assign15030_body26_e21371 * (-locals.var_chi_dn10)), (assign15030_body26_e21371 * (-locals.var_chi_dn11)), (assign15030_body26_e21371 * (-locals.var_chi_dn12)), (assign15030_body26_e21371 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15030_body26_e21373;
            locals.var_t0_dn0 = assign15030_body26_e21373_d_n0;
            locals.var_t0_dn2 = assign15030_body26_e21373_d_n2;
            locals.var_t0_dn6 = assign15030_body26_e21373_d_n6;
            locals.var_t0_dn7 = assign15030_body26_e21373_d_n7;
            locals.var_t0_dn10 = assign15030_body26_e21373_d_n10;
            locals.var_t0_dn11 = assign15030_body26_e21373_d_n11;
            locals.var_t0_dn12 = assign15030_body26_e21373_d_n12;
            locals.var_t0_dn17 = assign15030_body26_e21373_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15030_body27_e21386, assign15030_body27_e21386_d_n0, assign15030_body27_e21386_d_n2, assign15030_body27_e21386_d_n6, assign15030_body27_e21386_d_n7, assign15030_body27_e21386_d_n10, assign15030_body27_e21386_d_n11, assign15030_body27_e21386_d_n12, assign15030_body27_e21386_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) {
        let assign15030_body27_e21383: f64 = (-locals.var_chib);
        let assign15030_body27_e21384: f64 = (assign15030_body27_e21383).exp();
        (assign15030_body27_e21384, (assign15030_body27_e21384 * (-locals.var_chib_dn0)), (assign15030_body27_e21384 * (-locals.var_chib_dn2)), (assign15030_body27_e21384 * (-locals.var_chib_dn6)), (assign15030_body27_e21384 * (-locals.var_chib_dn7)), (assign15030_body27_e21384 * (-locals.var_chib_dn10)), (assign15030_body27_e21384 * (-locals.var_chib_dn11)), (assign15030_body27_e21384 * (-locals.var_chib_dn12)), (assign15030_body27_e21384 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15030_body27_e21386;
            locals.var_t1_dn0 = assign15030_body27_e21386_d_n0;
            locals.var_t1_dn2 = assign15030_body27_e21386_d_n2;
            locals.var_t1_dn6 = assign15030_body27_e21386_d_n6;
            locals.var_t1_dn7 = assign15030_body27_e21386_d_n7;
            locals.var_t1_dn10 = assign15030_body27_e21386_d_n10;
            locals.var_t1_dn11 = assign15030_body27_e21386_d_n11;
            locals.var_t1_dn12 = assign15030_body27_e21386_d_n12;
            locals.var_t1_dn17 = assign15030_body27_e21386_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15030_body28_e21404, assign15030_body28_e21404_d_n0, assign15030_body28_e21404_d_n2, assign15030_body28_e21404_d_n6, assign15030_body28_e21404_d_n7, assign15030_body28_e21404_d_n10, assign15030_body28_e21404_d_n11, assign15030_body28_e21404_d_n12, assign15030_body28_e21404_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) {
        let assign15030_body28_e21397: f64 = (locals.var_chi - locals.var_chib);
        let assign15030_body28_e21400: f64 = (locals.var_t0 - locals.var_t1);
        let assign15030_body28_e21401: f64 = (assign15030_body28_e21397 + assign15030_body28_e21400);
        let assign15030_body28_e21402: f64 = (assign15030_body28_e21401).sqrt();
        (assign15030_body28_e21402, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign15030_body28_e21402)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign15030_body28_e21402)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15030_body28_e21404;
            locals.var_fb_dn0 = assign15030_body28_e21404_d_n0;
            locals.var_fb_dn2 = assign15030_body28_e21404_d_n2;
            locals.var_fb_dn6 = assign15030_body28_e21404_d_n6;
            locals.var_fb_dn7 = assign15030_body28_e21404_d_n7;
            locals.var_fb_dn10 = assign15030_body28_e21404_d_n10;
            locals.var_fb_dn11 = assign15030_body28_e21404_d_n11;
            locals.var_fb_dn12 = assign15030_body28_e21404_d_n12;
            locals.var_fb_dn17 = assign15030_body28_e21404_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15030_body29_e21429, assign15030_body29_e21429_d_n0, assign15030_body29_e21429_d_n2, assign15030_body29_e21429_d_n6, assign15030_body29_e21429_d_n7, assign15030_body29_e21429_d_n10, assign15030_body29_e21429_d_n11, assign15030_body29_e21429_d_n12, assign15030_body29_e21429_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard448 == 0.0)) && (locals.var_guard450 == 0.0)) {
        let assign15030_body29_e21415: f64 = (locals.var_beta * 0.5);
        let assign15030_body29_e21418: f64 = (1.0 - locals.var_t0);
        let assign15030_body29_e21422: f64 = (1.0 - locals.var_t1);
        let assign15030_body29_e21423: f64 = (locals.var_phi_soib_dpss * assign15030_body29_e21422);
        let assign15030_body29_e21424: f64 = (assign15030_body29_e21418 - assign15030_body29_e21423);
        let assign15030_body29_e21425: f64 = (assign15030_body29_e21415 * assign15030_body29_e21424);
        let assign15030_body29_e21427: f64 = (assign15030_body29_e21425 / locals.var_fb);
        (assign15030_body29_e21427, ((((assign15030_body29_e21415 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign15030_body29_e21424) + (assign15030_body29_e21415 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign15030_body29_e21415 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign15030_body29_e21422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign15030_body29_e21425 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15030_body29_e21429;
            locals.var_fb_dpss_dn0 = assign15030_body29_e21429_d_n0;
            locals.var_fb_dpss_dn2 = assign15030_body29_e21429_d_n2;
            locals.var_fb_dpss_dn6 = assign15030_body29_e21429_d_n6;
            locals.var_fb_dpss_dn7 = assign15030_body29_e21429_d_n7;
            locals.var_fb_dpss_dn10 = assign15030_body29_e21429_d_n10;
            locals.var_fb_dpss_dn11 = assign15030_body29_e21429_d_n11;
            locals.var_fb_dpss_dn12 = assign15030_body29_e21429_d_n12;
            locals.var_fb_dpss_dn17 = assign15030_body29_e21429_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15030_body30_e21436: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard451 = assign15030_body30_e21436;
            locals.var_guard451_rv = 0.0;
            let (assign15030_body31_e21444,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard451 != 0.0)) {
        let assign15030_body31_e21442: f64 = (-1.0);
        (assign15030_body31_e21442,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign15030_body31_e21444;
            locals.var_flg_zone_rv = 0.0;
            let assign15030_body32_e21447: f64 = (-1.0);
            let assign15030_body32_e21448: f64 = if locals.var_flg_zone == assign15030_body32_e21447 { 1.0 } else { 0.0 };
            locals.var_guard452 = assign15030_body32_e21448;
            locals.var_guard452_rv = 0.0;
            let (assign15030_body33_e21455, assign15030_body33_e21455_d_n0, assign15030_body33_e21455_d_n2, assign15030_body33_e21455_d_n6, assign15030_body33_e21455_d_n7, assign15030_body33_e21455_d_n10, assign15030_body33_e21455_d_n11, assign15030_body33_e21455_d_n12, assign15030_body33_e21455_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard452 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign15030_body33_e21455;
            locals.var_wdsoi_dn0 = assign15030_body33_e21455_d_n0;
            locals.var_wdsoi_dn2 = assign15030_body33_e21455_d_n2;
            locals.var_wdsoi_dn6 = assign15030_body33_e21455_d_n6;
            locals.var_wdsoi_dn7 = assign15030_body33_e21455_d_n7;
            locals.var_wdsoi_dn10 = assign15030_body33_e21455_d_n10;
            locals.var_wdsoi_dn11 = assign15030_body33_e21455_d_n11;
            locals.var_wdsoi_dn12 = assign15030_body33_e21455_d_n12;
            locals.var_wdsoi_dn17 = assign15030_body33_e21455_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign15030_body34_e21465, assign15030_body34_e21465_d_n0, assign15030_body34_e21465_d_n2, assign15030_body34_e21465_d_n6, assign15030_body34_e21465_d_n7, assign15030_body34_e21465_d_n10, assign15030_body34_e21465_d_n11, assign15030_body34_e21465_d_n12, assign15030_body34_e21465_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard452 == 0.0)) {
        let assign15030_body34_e21463: f64 = (locals.var_c_w_soi * locals.var_fb);
        (assign15030_body34_e21463, ((locals.var_c_w_soi_dn0 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn0)), ((locals.var_c_w_soi_dn2 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn2)), ((locals.var_c_w_soi_dn6 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn6)), ((locals.var_c_w_soi_dn7 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn7)), ((locals.var_c_w_soi_dn10 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn10)), ((locals.var_c_w_soi_dn11 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn11)), ((locals.var_c_w_soi_dn12 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn12)), ((locals.var_c_w_soi_dn17 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn17)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign15030_body34_e21465;
            locals.var_wdsoi_dn0 = assign15030_body34_e21465_d_n0;
            locals.var_wdsoi_dn2 = assign15030_body34_e21465_d_n2;
            locals.var_wdsoi_dn6 = assign15030_body34_e21465_d_n6;
            locals.var_wdsoi_dn7 = assign15030_body34_e21465_d_n7;
            locals.var_wdsoi_dn10 = assign15030_body34_e21465_d_n10;
            locals.var_wdsoi_dn11 = assign15030_body34_e21465_d_n11;
            locals.var_wdsoi_dn12 = assign15030_body34_e21465_d_n12;
            locals.var_wdsoi_dn17 = assign15030_body34_e21465_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let assign15030_body35_e21469: f64 = (p.p237 * 1.01);
            let assign15030_body35_e21470: f64 = if locals.var_wdsoi < assign15030_body35_e21469 { 1.0 } else { 0.0 };
            locals.var_guard453 = assign15030_body35_e21470;
            locals.var_guard453_rv = 0.0;
            let (assign15030_body36_e21477,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard453 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
            locals.var_flg_depmode = assign15030_body36_e21477;
            locals.var_flg_depmode_rv = 0.0;
            let (assign15030_body37_e21485,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard453 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
            locals.var_flg_depmode = assign15030_body37_e21485;
            locals.var_flg_depmode_rv = 0.0;
            let (assign15030_body38_e21492, assign15030_body38_e21492_d_n0, assign15030_body38_e21492_d_n2, assign15030_body38_e21492_d_n6, assign15030_body38_e21492_d_n7, assign15030_body38_e21492_d_n10, assign15030_body38_e21492_d_n11, assign15030_body38_e21492_d_n12, assign15030_body38_e21492_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body38_e21490: f64 = (locals.var_q_nsub * locals.var_wdsoi);
        (assign15030_body38_e21490, ((locals.var_q_nsub_dn0 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn0)), ((locals.var_q_nsub_dn2 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn2)), ((locals.var_q_nsub_dn6 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn6)), ((locals.var_q_nsub_dn7 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn7)), ((locals.var_q_nsub_dn10 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn10)), ((locals.var_q_nsub_dn11 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn11)), ((locals.var_q_nsub_dn12 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn12)), ((locals.var_q_nsub_dn17 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn17)),)
    } else {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    }
};
            locals.var_q_dep_soi = assign15030_body38_e21492;
            locals.var_q_dep_soi_dn0 = assign15030_body38_e21492_d_n0;
            locals.var_q_dep_soi_dn2 = assign15030_body38_e21492_d_n2;
            locals.var_q_dep_soi_dn6 = assign15030_body38_e21492_d_n6;
            locals.var_q_dep_soi_dn7 = assign15030_body38_e21492_d_n7;
            locals.var_q_dep_soi_dn10 = assign15030_body38_e21492_d_n10;
            locals.var_q_dep_soi_dn11 = assign15030_body38_e21492_d_n11;
            locals.var_q_dep_soi_dn12 = assign15030_body38_e21492_d_n12;
            locals.var_q_dep_soi_dn17 = assign15030_body38_e21492_d_n17;
            locals.var_q_dep_soi_rv = 0.0;
            let assign15030_body39_e21495: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard454 = assign15030_body39_e21495;
            locals.var_guard454_rv = 0.0;
            let (assign15030_body40_e21503, assign15030_body40_e21503_d_n0, assign15030_body40_e21503_d_n2, assign15030_body40_e21503_d_n6, assign15030_body40_e21503_d_n7, assign15030_body40_e21503_d_n10, assign15030_body40_e21503_d_n11, assign15030_body40_e21503_d_n12, assign15030_body40_e21503_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard454 != 0.0)) {
        let assign15030_body40_e21501: f64 = (-locals.var_fb);
        (assign15030_body40_e21501, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15030_body40_e21503;
            locals.var_fs02_dn0 = assign15030_body40_e21503_d_n0;
            locals.var_fs02_dn2 = assign15030_body40_e21503_d_n2;
            locals.var_fs02_dn6 = assign15030_body40_e21503_d_n6;
            locals.var_fs02_dn7 = assign15030_body40_e21503_d_n7;
            locals.var_fs02_dn10 = assign15030_body40_e21503_d_n10;
            locals.var_fs02_dn11 = assign15030_body40_e21503_d_n11;
            locals.var_fs02_dn12 = assign15030_body40_e21503_d_n12;
            locals.var_fs02_dn17 = assign15030_body40_e21503_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15030_body41_e21511, assign15030_body41_e21511_d_n0, assign15030_body41_e21511_d_n2, assign15030_body41_e21511_d_n6, assign15030_body41_e21511_d_n7, assign15030_body41_e21511_d_n10, assign15030_body41_e21511_d_n11, assign15030_body41_e21511_d_n12, assign15030_body41_e21511_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard454 != 0.0)) {
        let assign15030_body41_e21509: f64 = (-locals.var_fb_dpss);
        (assign15030_body41_e21509, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15030_body41_e21511;
            locals.var_fs02_dps0_dn0 = assign15030_body41_e21511_d_n0;
            locals.var_fs02_dps0_dn2 = assign15030_body41_e21511_d_n2;
            locals.var_fs02_dps0_dn6 = assign15030_body41_e21511_d_n6;
            locals.var_fs02_dps0_dn7 = assign15030_body41_e21511_d_n7;
            locals.var_fs02_dps0_dn10 = assign15030_body41_e21511_d_n10;
            locals.var_fs02_dps0_dn11 = assign15030_body41_e21511_d_n11;
            locals.var_fs02_dps0_dn12 = assign15030_body41_e21511_d_n12;
            locals.var_fs02_dps0_dn17 = assign15030_body41_e21511_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign15030_body42_e21514: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard455 = assign15030_body42_e21514;
            locals.var_guard455_rv = 0.0;
            let (assign15030_body43_e21524, assign15030_body43_e21524_d_n0, assign15030_body43_e21524_d_n2, assign15030_body43_e21524_d_n6, assign15030_body43_e21524_d_n7, assign15030_body43_e21524_d_n10, assign15030_body43_e21524_d_n11, assign15030_body43_e21524_d_n12, assign15030_body43_e21524_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15030_body43_e21524;
            locals.var_fs02_dn0 = assign15030_body43_e21524_d_n0;
            locals.var_fs02_dn2 = assign15030_body43_e21524_d_n2;
            locals.var_fs02_dn6 = assign15030_body43_e21524_d_n6;
            locals.var_fs02_dn7 = assign15030_body43_e21524_d_n7;
            locals.var_fs02_dn10 = assign15030_body43_e21524_d_n10;
            locals.var_fs02_dn11 = assign15030_body43_e21524_d_n11;
            locals.var_fs02_dn12 = assign15030_body43_e21524_d_n12;
            locals.var_fs02_dn17 = assign15030_body43_e21524_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15030_body44_e21534, assign15030_body44_e21534_d_n0, assign15030_body44_e21534_d_n2, assign15030_body44_e21534_d_n6, assign15030_body44_e21534_d_n7, assign15030_body44_e21534_d_n10, assign15030_body44_e21534_d_n11, assign15030_body44_e21534_d_n12, assign15030_body44_e21534_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15030_body44_e21534;
            locals.var_fs02_dps0_dn0 = assign15030_body44_e21534_d_n0;
            locals.var_fs02_dps0_dn2 = assign15030_body44_e21534_d_n2;
            locals.var_fs02_dps0_dn6 = assign15030_body44_e21534_d_n6;
            locals.var_fs02_dps0_dn7 = assign15030_body44_e21534_d_n7;
            locals.var_fs02_dps0_dn10 = assign15030_body44_e21534_d_n10;
            locals.var_fs02_dps0_dn11 = assign15030_body44_e21534_d_n11;
            locals.var_fs02_dps0_dn12 = assign15030_body44_e21534_d_n12;
            locals.var_fs02_dps0_dn17 = assign15030_body44_e21534_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign15030_body45_e21537: f64 = if locals.var_chi < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard456 = assign15030_body45_e21537;
            locals.var_guard456_rv = 0.0;
            let (assign15030_body46_e21551, assign15030_body46_e21551_d_n0, assign15030_body46_e21551_d_n2, assign15030_body46_e21551_d_n6, assign15030_body46_e21551_d_n7, assign15030_body46_e21551_d_n10, assign15030_body46_e21551_d_n11, assign15030_body46_e21551_d_n12, assign15030_body46_e21551_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign15030_body46_e21549: f64 = (locals.var_chi).exp();
        (assign15030_body46_e21549, (assign15030_body46_e21549 * locals.var_chi_dn0), (assign15030_body46_e21549 * locals.var_chi_dn2), (assign15030_body46_e21549 * locals.var_chi_dn6), (assign15030_body46_e21549 * locals.var_chi_dn7), (assign15030_body46_e21549 * locals.var_chi_dn10), (assign15030_body46_e21549 * locals.var_chi_dn11), (assign15030_body46_e21549 * locals.var_chi_dn12), (assign15030_body46_e21549 * locals.var_chi_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign15030_body46_e21551;
            locals.var_exp_chi_dn0 = assign15030_body46_e21551_d_n0;
            locals.var_exp_chi_dn2 = assign15030_body46_e21551_d_n2;
            locals.var_exp_chi_dn6 = assign15030_body46_e21551_d_n6;
            locals.var_exp_chi_dn7 = assign15030_body46_e21551_d_n7;
            locals.var_exp_chi_dn10 = assign15030_body46_e21551_d_n10;
            locals.var_exp_chi_dn11 = assign15030_body46_e21551_d_n11;
            locals.var_exp_chi_dn12 = assign15030_body46_e21551_d_n12;
            locals.var_exp_chi_dn17 = assign15030_body46_e21551_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign15030_body47_e21570, assign15030_body47_e21570_d_n0, assign15030_body47_e21570_d_n2, assign15030_body47_e21570_d_n6, assign15030_body47_e21570_d_n7, assign15030_body47_e21570_d_n10, assign15030_body47_e21570_d_n11, assign15030_body47_e21570_d_n12, assign15030_body47_e21570_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign15030_body47_e21566: f64 = (locals.var_chi + 1.0);
        let assign15030_body47_e21567: f64 = (locals.var_exp_chi - assign15030_body47_e21566);
        let assign15030_body47_e21568: f64 = (locals.var_cfs1 * assign15030_body47_e21567);
        (assign15030_body47_e21568, ((locals.var_cfs1_dn0 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn6 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn10 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn12 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn12 - locals.var_chi_dn12))), ((locals.var_cfs1_dn17 * assign15030_body47_e21567) + (locals.var_cfs1 * (locals.var_exp_chi_dn17 - locals.var_chi_dn17))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign15030_body47_e21570;
            locals.var_fs01_dn0 = assign15030_body47_e21570_d_n0;
            locals.var_fs01_dn2 = assign15030_body47_e21570_d_n2;
            locals.var_fs01_dn6 = assign15030_body47_e21570_d_n6;
            locals.var_fs01_dn7 = assign15030_body47_e21570_d_n7;
            locals.var_fs01_dn10 = assign15030_body47_e21570_d_n10;
            locals.var_fs01_dn11 = assign15030_body47_e21570_d_n11;
            locals.var_fs01_dn12 = assign15030_body47_e21570_d_n12;
            locals.var_fs01_dn17 = assign15030_body47_e21570_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign15030_body48_e21589, assign15030_body48_e21589_d_n0, assign15030_body48_e21589_d_n2, assign15030_body48_e21589_d_n6, assign15030_body48_e21589_d_n7, assign15030_body48_e21589_d_n10, assign15030_body48_e21589_d_n11, assign15030_body48_e21589_d_n12, assign15030_body48_e21589_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 != 0.0)) {
        let assign15030_body48_e21583: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign15030_body48_e21586: f64 = (locals.var_exp_chi - 1.0);
        let assign15030_body48_e21587: f64 = (assign15030_body48_e21583 * assign15030_body48_e21586);
        (assign15030_body48_e21587, (((locals.var_cfs1_dn0 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn0)), (((locals.var_cfs1_dn2 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn2)), (((locals.var_cfs1_dn6 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn6)), (((locals.var_cfs1_dn7 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn10)), (((locals.var_cfs1_dn11 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn11)), (((locals.var_cfs1_dn12 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn12)), (((locals.var_cfs1_dn17 * locals.var_beta) * assign15030_body48_e21586) + (assign15030_body48_e21583 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign15030_body48_e21589;
            locals.var_fs01_dps0_dn0 = assign15030_body48_e21589_d_n0;
            locals.var_fs01_dps0_dn2 = assign15030_body48_e21589_d_n2;
            locals.var_fs01_dps0_dn6 = assign15030_body48_e21589_d_n6;
            locals.var_fs01_dps0_dn7 = assign15030_body48_e21589_d_n7;
            locals.var_fs01_dps0_dn10 = assign15030_body48_e21589_d_n10;
            locals.var_fs01_dps0_dn11 = assign15030_body48_e21589_d_n11;
            locals.var_fs01_dps0_dn12 = assign15030_body48_e21589_d_n12;
            locals.var_fs01_dps0_dn17 = assign15030_body48_e21589_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign15030_body49_e21606, assign15030_body49_e21606_d_n0, assign15030_body49_e21606_d_n2, assign15030_body49_e21606_d_n6, assign15030_body49_e21606_d_n7, assign15030_body49_e21606_d_n10, assign15030_body49_e21606_d_n11, assign15030_body49_e21606_d_n12, assign15030_body49_e21606_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
        let assign15030_body49_e21603: f64 = (locals.var_beta * locals.var_phi_s0_soi);
        let assign15030_body49_e21604: f64 = (assign15030_body49_e21603).exp();
        (assign15030_body49_e21604, (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn0)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn2)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn6)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn7)), (assign15030_body49_e21604 * ((locals.var_beta_dn10 * locals.var_phi_s0_soi) + (locals.var_beta * locals.var_phi_s0_soi_dn10))), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn11)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn12)), (assign15030_body49_e21604 * (locals.var_beta * locals.var_phi_s0_soi_dn17)),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn12, locals.var_exp_bps0_dn17,)
    }
};
            locals.var_exp_bps0 = assign15030_body49_e21606;
            locals.var_exp_bps0_dn0 = assign15030_body49_e21606_d_n0;
            locals.var_exp_bps0_dn2 = assign15030_body49_e21606_d_n2;
            locals.var_exp_bps0_dn6 = assign15030_body49_e21606_d_n6;
            locals.var_exp_bps0_dn7 = assign15030_body49_e21606_d_n7;
            locals.var_exp_bps0_dn10 = assign15030_body49_e21606_d_n10;
            locals.var_exp_bps0_dn11 = assign15030_body49_e21606_d_n11;
            locals.var_exp_bps0_dn12 = assign15030_body49_e21606_d_n12;
            locals.var_exp_bps0_dn17 = assign15030_body49_e21606_d_n17;
            locals.var_exp_bps0_rv = 0.0;
            let (assign15030_body50_e21628, assign15030_body50_e21628_d_n0, assign15030_body50_e21628_d_n2, assign15030_body50_e21628_d_n6, assign15030_body50_e21628_d_n7, assign15030_body50_e21628_d_n10, assign15030_body50_e21628_d_n11, assign15030_body50_e21628_d_n12, assign15030_body50_e21628_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
        let assign15030_body50_e21623: f64 = (locals.var_chi + 1.0);
        let assign15030_body50_e21624: f64 = (locals.var_exp_bvbs * assign15030_body50_e21623);
        let assign15030_body50_e21625: f64 = (locals.var_exp_bps0 - assign15030_body50_e21624);
        let assign15030_body50_e21626: f64 = (locals.var_cnst1soi * assign15030_body50_e21625);
        (assign15030_body50_e21626, ((locals.var_cnst1soi_dn0 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn12 - ((locals.var_exp_bvbs_dn12 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign15030_body50_e21625) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn17 - ((locals.var_exp_bvbs_dn17 * assign15030_body50_e21623) + (locals.var_exp_bvbs * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign15030_body50_e21628;
            locals.var_fs01_dn0 = assign15030_body50_e21628_d_n0;
            locals.var_fs01_dn2 = assign15030_body50_e21628_d_n2;
            locals.var_fs01_dn6 = assign15030_body50_e21628_d_n6;
            locals.var_fs01_dn7 = assign15030_body50_e21628_d_n7;
            locals.var_fs01_dn10 = assign15030_body50_e21628_d_n10;
            locals.var_fs01_dn11 = assign15030_body50_e21628_d_n11;
            locals.var_fs01_dn12 = assign15030_body50_e21628_d_n12;
            locals.var_fs01_dn17 = assign15030_body50_e21628_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign15030_body51_e21648, assign15030_body51_e21648_d_n0, assign15030_body51_e21648_d_n2, assign15030_body51_e21648_d_n6, assign15030_body51_e21648_d_n7, assign15030_body51_e21648_d_n10, assign15030_body51_e21648_d_n11, assign15030_body51_e21648_d_n12, assign15030_body51_e21648_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) && (locals.var_guard456 == 0.0)) {
        let assign15030_body51_e21642: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign15030_body51_e21645: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign15030_body51_e21646: f64 = (assign15030_body51_e21642 * assign15030_body51_e21645);
        (assign15030_body51_e21646, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn12 - locals.var_exp_bvbs_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign15030_body51_e21645) + (assign15030_body51_e21642 * (locals.var_exp_bps0_dn17 - locals.var_exp_bvbs_dn17))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign15030_body51_e21648;
            locals.var_fs01_dps0_dn0 = assign15030_body51_e21648_d_n0;
            locals.var_fs01_dps0_dn2 = assign15030_body51_e21648_d_n2;
            locals.var_fs01_dps0_dn6 = assign15030_body51_e21648_d_n6;
            locals.var_fs01_dps0_dn7 = assign15030_body51_e21648_d_n7;
            locals.var_fs01_dps0_dn10 = assign15030_body51_e21648_d_n10;
            locals.var_fs01_dps0_dn11 = assign15030_body51_e21648_d_n11;
            locals.var_fs01_dps0_dn12 = assign15030_body51_e21648_d_n12;
            locals.var_fs01_dps0_dn17 = assign15030_body51_e21648_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign15030_body52_e21664, assign15030_body52_e21664_d_n0, assign15030_body52_e21664_d_n2, assign15030_body52_e21664_d_n6, assign15030_body52_e21664_d_n7, assign15030_body52_e21664_d_n10, assign15030_body52_e21664_d_n11, assign15030_body52_e21664_d_n12, assign15030_body52_e21664_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) {
        let assign15030_body52_e21659: f64 = (locals.var_fb * locals.var_fb);
        let assign15030_body52_e21661: f64 = (assign15030_body52_e21659 + locals.var_fs01);
        let assign15030_body52_e21662: f64 = (assign15030_body52_e21661).sqrt();
        (assign15030_body52_e21662, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign15030_body52_e21662)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fs01_dn17) / (2.0 * assign15030_body52_e21662)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15030_body52_e21664;
            locals.var_fs02_dn0 = assign15030_body52_e21664_d_n0;
            locals.var_fs02_dn2 = assign15030_body52_e21664_d_n2;
            locals.var_fs02_dn6 = assign15030_body52_e21664_d_n6;
            locals.var_fs02_dn7 = assign15030_body52_e21664_d_n7;
            locals.var_fs02_dn10 = assign15030_body52_e21664_d_n10;
            locals.var_fs02_dn11 = assign15030_body52_e21664_d_n11;
            locals.var_fs02_dn12 = assign15030_body52_e21664_d_n12;
            locals.var_fs02_dn17 = assign15030_body52_e21664_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15030_body53_e21685, assign15030_body53_e21685_d_n0, assign15030_body53_e21685_d_n2, assign15030_body53_e21685_d_n6, assign15030_body53_e21685_d_n7, assign15030_body53_e21685_d_n10, assign15030_body53_e21685_d_n11, assign15030_body53_e21685_d_n12, assign15030_body53_e21685_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard454 == 0.0)) && (locals.var_guard455 == 0.0)) {
        let assign15030_body53_e21676: f64 = (2.0 * locals.var_fb_dpss);
        let assign15030_body53_e21678: f64 = (assign15030_body53_e21676 * locals.var_fb);
        let assign15030_body53_e21680: f64 = (assign15030_body53_e21678 + locals.var_fs01_dps0);
        let assign15030_body53_e21681: f64 = (0.5 * assign15030_body53_e21680);
        let assign15030_body53_e21683: f64 = (assign15030_body53_e21681 / locals.var_fs02);
        (assign15030_body53_e21683, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign15030_body53_e21676 * locals.var_fb_dn17)) + locals.var_fs01_dps0_dn17)) * locals.var_fs02) - (assign15030_body53_e21681 * locals.var_fs02_dn17)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15030_body53_e21685;
            locals.var_fs02_dps0_dn0 = assign15030_body53_e21685_d_n0;
            locals.var_fs02_dps0_dn2 = assign15030_body53_e21685_d_n2;
            locals.var_fs02_dps0_dn6 = assign15030_body53_e21685_d_n6;
            locals.var_fs02_dps0_dn7 = assign15030_body53_e21685_d_n7;
            locals.var_fs02_dps0_dn10 = assign15030_body53_e21685_d_n10;
            locals.var_fs02_dps0_dn11 = assign15030_body53_e21685_d_n11;
            locals.var_fs02_dps0_dn12 = assign15030_body53_e21685_d_n12;
            locals.var_fs02_dps0_dn17 = assign15030_body53_e21685_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign15030_body54_e21701, assign15030_body54_e21701_d_n0, assign15030_body54_e21701_d_n2, assign15030_body54_e21701_d_n6, assign15030_body54_e21701_d_n7, assign15030_body54_e21701_d_n10, assign15030_body54_e21701_d_n11, assign15030_body54_e21701_d_n12, assign15030_body54_e21701_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body54_e21689: f64 = (-locals.var_vgp);
        let assign15030_body54_e21691: f64 = (assign15030_body54_e21689 + locals.var_phi_s0_soi);
        let assign15030_body54_e21694: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign15030_body54_e21695: f64 = (assign15030_body54_e21691 + assign15030_body54_e21694);
        let assign15030_body54_e21698: f64 = (locals.var_c_fox_inv * locals.var_qhs);
        let assign15030_body54_e21699: f64 = (assign15030_body54_e21695 - assign15030_body54_e21698);
        (assign15030_body54_e21699, ((((-locals.var_vgp_dn0) + locals.var_phi_s0_soi_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))) - ((locals.var_c_fox_inv_dn0 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn0))), ((((-locals.var_vgp_dn2) + locals.var_phi_s0_soi_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))) - ((locals.var_c_fox_inv_dn2 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn2))), ((((-locals.var_vgp_dn6) + locals.var_phi_s0_soi_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))) - ((locals.var_c_fox_inv_dn6 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn6))), ((((-locals.var_vgp_dn7) + locals.var_phi_s0_soi_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))) - ((locals.var_c_fox_inv_dn7 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn7))), ((((-locals.var_vgp_dn10) + locals.var_phi_s0_soi_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))) - ((locals.var_c_fox_inv_dn10 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn10))), ((((-locals.var_vgp_dn11) + locals.var_phi_s0_soi_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))) - ((locals.var_c_fox_inv_dn11 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn11))), ((((-locals.var_vgp_dn12) + locals.var_phi_s0_soi_dn12) + ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))) - ((locals.var_c_fox_inv_dn12 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn12))), ((((-locals.var_vgp_dn17) + locals.var_phi_s0_soi_dn17) + ((locals.var_fac1_dn17 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn17))) - ((locals.var_c_fox_inv_dn17 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn17))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12, locals.var_fs0_dn17,)
    }
};
            locals.var_fs0 = assign15030_body54_e21701;
            locals.var_fs0_dn0 = assign15030_body54_e21701_d_n0;
            locals.var_fs0_dn2 = assign15030_body54_e21701_d_n2;
            locals.var_fs0_dn6 = assign15030_body54_e21701_d_n6;
            locals.var_fs0_dn7 = assign15030_body54_e21701_d_n7;
            locals.var_fs0_dn10 = assign15030_body54_e21701_d_n10;
            locals.var_fs0_dn11 = assign15030_body54_e21701_d_n11;
            locals.var_fs0_dn12 = assign15030_body54_e21701_d_n12;
            locals.var_fs0_dn17 = assign15030_body54_e21701_d_n17;
            locals.var_fs0_rv = 0.0;
            let (assign15030_body55_e21710, assign15030_body55_e21710_d_n0, assign15030_body55_e21710_d_n2, assign15030_body55_e21710_d_n6, assign15030_body55_e21710_d_n7, assign15030_body55_e21710_d_n10, assign15030_body55_e21710_d_n11, assign15030_body55_e21710_d_n12, assign15030_body55_e21710_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body55_e21707: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign15030_body55_e21708: f64 = (1.0 + assign15030_body55_e21707);
        (assign15030_body55_e21708, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12)), ((locals.var_fac1_dn17 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn17)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12, locals.var_fs0_dps0_dn17,)
    }
};
            locals.var_fs0_dps0 = assign15030_body55_e21710;
            locals.var_fs0_dps0_dn0 = assign15030_body55_e21710_d_n0;
            locals.var_fs0_dps0_dn2 = assign15030_body55_e21710_d_n2;
            locals.var_fs0_dps0_dn6 = assign15030_body55_e21710_d_n6;
            locals.var_fs0_dps0_dn7 = assign15030_body55_e21710_d_n7;
            locals.var_fs0_dps0_dn10 = assign15030_body55_e21710_d_n10;
            locals.var_fs0_dps0_dn11 = assign15030_body55_e21710_d_n11;
            locals.var_fs0_dps0_dn12 = assign15030_body55_e21710_d_n12;
            locals.var_fs0_dps0_dn17 = assign15030_body55_e21710_d_n17;
            locals.var_fs0_dps0_rv = 0.0;
            let assign15030_body56_e21713: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard457 = assign15030_body56_e21713;
            locals.var_guard457_rv = 0.0;
            let (assign15030_body57_e21722,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard457 != 0.0)) {
        let assign15030_body57_e21720: f64 = (locals.var_lp_s0_max + 1.0);
        (assign15030_body57_e21720,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign15030_body57_e21722;
            locals.var_lp_s0_rv = 0.0;
            let (assign15030_body58_e21733, assign15030_body58_e21733_d_n0, assign15030_body58_e21733_d_n2, assign15030_body58_e21733_d_n6, assign15030_body58_e21733_d_n7, assign15030_body58_e21733_d_n10, assign15030_body58_e21733_d_n11, assign15030_body58_e21733_d_n12, assign15030_body58_e21733_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) {
        let assign15030_body58_e21729: f64 = (-locals.var_fs0);
        let assign15030_body58_e21731: f64 = (assign15030_body58_e21729 / locals.var_fs0_dps0);
        (assign15030_body58_e21731, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn17) * locals.var_fs0_dps0) - (assign15030_body58_e21729 * locals.var_fs0_dps0_dn17)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign15030_body58_e21733;
            locals.var_dps0_dn0 = assign15030_body58_e21733_d_n0;
            locals.var_dps0_dn2 = assign15030_body58_e21733_d_n2;
            locals.var_dps0_dn6 = assign15030_body58_e21733_d_n6;
            locals.var_dps0_dn7 = assign15030_body58_e21733_d_n7;
            locals.var_dps0_dn10 = assign15030_body58_e21733_d_n10;
            locals.var_dps0_dn11 = assign15030_body58_e21733_d_n11;
            locals.var_dps0_dn12 = assign15030_body58_e21733_d_n12;
            locals.var_dps0_dn17 = assign15030_body58_e21733_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign15030_body59_e21754, assign15030_body59_e21754_d_n0, assign15030_body59_e21754_d_n2, assign15030_body59_e21754_d_n6, assign15030_body59_e21754_d_n7, assign15030_body59_e21754_d_n10, assign15030_body59_e21754_d_n11, assign15030_body59_e21754_d_n12, assign15030_body59_e21754_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) {
        let assign15030_body59_e21741: f64 = (0.5 * 0.1);
        let assign15030_body59_e21745: f64 = (locals.var_phi_s0_soi).abs();
        let (assign15030_body59_e21750, assign15030_body59_e21750_d_n0, assign15030_body59_e21750_d_n2, assign15030_body59_e21750_d_n6, assign15030_body59_e21750_d_n7, assign15030_body59_e21750_d_n10, assign15030_body59_e21750_d_n11, assign15030_body59_e21750_d_n12, assign15030_body59_e21750_d_n17,) = {
            if (1.0 >= assign15030_body59_e21745) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15030_body59_e21749: f64 = (locals.var_phi_s0_soi).abs();
                (assign15030_body59_e21749, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn0 } else { (-locals.var_phi_s0_soi_dn0) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn2 } else { (-locals.var_phi_s0_soi_dn2) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn6 } else { (-locals.var_phi_s0_soi_dn6) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn7 } else { (-locals.var_phi_s0_soi_dn7) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn10 } else { (-locals.var_phi_s0_soi_dn10) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn11 } else { (-locals.var_phi_s0_soi_dn11) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn12 } else { (-locals.var_phi_s0_soi_dn12) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn17 } else { (-locals.var_phi_s0_soi_dn17) },)
            }
        };
        let assign15030_body59_e21751: f64 = (1.0 + assign15030_body59_e21750);
        let assign15030_body59_e21752: f64 = (assign15030_body59_e21741 * assign15030_body59_e21751);
        (assign15030_body59_e21752, (assign15030_body59_e21741 * assign15030_body59_e21750_d_n0), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n2), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n6), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n7), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n10), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n11), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n12), (assign15030_body59_e21741 * assign15030_body59_e21750_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign15030_body59_e21754;
            locals.var_dplim_dn0 = assign15030_body59_e21754_d_n0;
            locals.var_dplim_dn2 = assign15030_body59_e21754_d_n2;
            locals.var_dplim_dn6 = assign15030_body59_e21754_d_n6;
            locals.var_dplim_dn7 = assign15030_body59_e21754_d_n7;
            locals.var_dplim_dn10 = assign15030_body59_e21754_d_n10;
            locals.var_dplim_dn11 = assign15030_body59_e21754_d_n11;
            locals.var_dplim_dn12 = assign15030_body59_e21754_d_n12;
            locals.var_dplim_dn17 = assign15030_body59_e21754_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign15030_body60_e21756: f64 = (locals.var_dps0).abs();
            let assign15030_body60_e21758: f64 = if assign15030_body60_e21756 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard458 = assign15030_body60_e21758;
            locals.var_guard458_rv = 0.0;
            let (assign15030_body61_e21776, assign15030_body61_e21776_d_n0, assign15030_body61_e21776_d_n2, assign15030_body61_e21776_d_n6, assign15030_body61_e21776_d_n7, assign15030_body61_e21776_d_n10, assign15030_body61_e21776_d_n11, assign15030_body61_e21776_d_n12, assign15030_body61_e21776_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard458 != 0.0)) {
        let (assign15030_body61_e21773,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign15030_body61_e21772: f64 = (-1.0);
                (assign15030_body61_e21772,)
            }
        };
        let assign15030_body61_e21774: f64 = (locals.var_dplim * assign15030_body61_e21773);
        (assign15030_body61_e21774, (locals.var_dplim_dn0 * assign15030_body61_e21773), (locals.var_dplim_dn2 * assign15030_body61_e21773), (locals.var_dplim_dn6 * assign15030_body61_e21773), (locals.var_dplim_dn7 * assign15030_body61_e21773), (locals.var_dplim_dn10 * assign15030_body61_e21773), (locals.var_dplim_dn11 * assign15030_body61_e21773), (locals.var_dplim_dn12 * assign15030_body61_e21773), (locals.var_dplim_dn17 * assign15030_body61_e21773),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign15030_body61_e21776;
            locals.var_dps0_dn0 = assign15030_body61_e21776_d_n0;
            locals.var_dps0_dn2 = assign15030_body61_e21776_d_n2;
            locals.var_dps0_dn6 = assign15030_body61_e21776_d_n6;
            locals.var_dps0_dn7 = assign15030_body61_e21776_d_n7;
            locals.var_dps0_dn10 = assign15030_body61_e21776_d_n10;
            locals.var_dps0_dn11 = assign15030_body61_e21776_d_n11;
            locals.var_dps0_dn12 = assign15030_body61_e21776_d_n12;
            locals.var_dps0_dn17 = assign15030_body61_e21776_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign15030_body62_e21786, assign15030_body62_e21786_d_n0, assign15030_body62_e21786_d_n2, assign15030_body62_e21786_d_n6, assign15030_body62_e21786_d_n7, assign15030_body62_e21786_d_n10, assign15030_body62_e21786_d_n11, assign15030_body62_e21786_d_n12, assign15030_body62_e21786_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) {
        let assign15030_body62_e21784: f64 = (locals.var_phi_s0_soi + locals.var_dps0);
        (assign15030_body62_e21784, (locals.var_phi_s0_soi_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_soi_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_soi_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_soi_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_soi_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_soi_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_soi_dn12 + locals.var_dps0_dn12), (locals.var_phi_s0_soi_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
            locals.var_phi_s0_soi = assign15030_body62_e21786;
            locals.var_phi_s0_soi_dn0 = assign15030_body62_e21786_d_n0;
            locals.var_phi_s0_soi_dn2 = assign15030_body62_e21786_d_n2;
            locals.var_phi_s0_soi_dn6 = assign15030_body62_e21786_d_n6;
            locals.var_phi_s0_soi_dn7 = assign15030_body62_e21786_d_n7;
            locals.var_phi_s0_soi_dn10 = assign15030_body62_e21786_d_n10;
            locals.var_phi_s0_soi_dn11 = assign15030_body62_e21786_d_n11;
            locals.var_phi_s0_soi_dn12 = assign15030_body62_e21786_d_n12;
            locals.var_phi_s0_soi_dn17 = assign15030_body62_e21786_d_n17;
            locals.var_phi_s0_soi_rv = 0.0;
            let assign15030_body63_e21788: f64 = (locals.var_dps0).abs();
            let assign15030_body63_e21792: f64 = (locals.var_fs0).abs();
            let assign15030_body63_e21795: f64 = if ((assign15030_body63_e21788 <= 5e-12) && (assign15030_body63_e21792 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard459 = assign15030_body63_e21795;
            locals.var_guard459_rv = 0.0;
            let (assign15030_body64_e21805,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard457 == 0.0)) && (locals.var_guard459 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign15030_body64_e21805;
            locals.var_flg_conv_rv = 0.0;
            let (assign15030_body65_e21812,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15030_body65_e21810: f64 = (locals.var_lp_s0 + 1.0);
        (assign15030_body65_e21810,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign15030_body65_e21812;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_53(
        locals: &mut StampLocals,
    ) {
        let (assign15040_e21819,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15040_e21817: f64 = (locals.var_lp_s0 - 1.0);
        (assign15040_e21817,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign15040_e21819;
        locals.var_lp_s0_rv = 0.0;

        let (assign15050_e21824, assign15050_e21824_d_n0, assign15050_e21824_d_n2, assign15050_e21824_d_n6, assign15050_e21824_d_n7, assign15050_e21824_d_n10, assign15050_e21824_d_n11, assign15050_e21824_d_n12, assign15050_e21824_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    } else {
        (locals.var_q_deps0, locals.var_q_deps0_dn0, locals.var_q_deps0_dn2, locals.var_q_deps0_dn6, locals.var_q_deps0_dn7, locals.var_q_deps0_dn10, locals.var_q_deps0_dn11, locals.var_q_deps0_dn12, locals.var_q_deps0_dn17,)
    }
};
        locals.var_q_deps0 = assign15050_e21824;
        locals.var_q_deps0_dn0 = assign15050_e21824_d_n0;
        locals.var_q_deps0_dn2 = assign15050_e21824_d_n2;
        locals.var_q_deps0_dn6 = assign15050_e21824_d_n6;
        locals.var_q_deps0_dn7 = assign15050_e21824_d_n7;
        locals.var_q_deps0_dn10 = assign15050_e21824_d_n10;
        locals.var_q_deps0_dn11 = assign15050_e21824_d_n11;
        locals.var_q_deps0_dn12 = assign15050_e21824_d_n12;
        locals.var_q_deps0_dn17 = assign15050_e21824_d_n17;
        locals.var_q_deps0_rv = 0.0;

        let (assign15060_e21829, assign15060_e21829_d_n0, assign15060_e21829_d_n2, assign15060_e21829_d_n6, assign15060_e21829_d_n7, assign15060_e21829_d_n10, assign15060_e21829_d_n11, assign15060_e21829_d_n12, assign15060_e21829_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_q_deps0, locals.var_q_deps0_dn0, locals.var_q_deps0_dn2, locals.var_q_deps0_dn6, locals.var_q_deps0_dn7, locals.var_q_deps0_dn10, locals.var_q_deps0_dn11, locals.var_q_deps0_dn12, locals.var_q_deps0_dn17,)
    } else {
        (locals.var_q_dep0, locals.var_q_dep0_dn0, locals.var_q_dep0_dn2, locals.var_q_dep0_dn6, locals.var_q_dep0_dn7, locals.var_q_dep0_dn10, locals.var_q_dep0_dn11, locals.var_q_dep0_dn12, locals.var_q_dep0_dn17,)
    }
};
        locals.var_q_dep0 = assign15060_e21829;
        locals.var_q_dep0_dn0 = assign15060_e21829_d_n0;
        locals.var_q_dep0_dn2 = assign15060_e21829_d_n2;
        locals.var_q_dep0_dn6 = assign15060_e21829_d_n6;
        locals.var_q_dep0_dn7 = assign15060_e21829_d_n7;
        locals.var_q_dep0_dn10 = assign15060_e21829_d_n10;
        locals.var_q_dep0_dn11 = assign15060_e21829_d_n11;
        locals.var_q_dep0_dn12 = assign15060_e21829_d_n12;
        locals.var_q_dep0_dn17 = assign15060_e21829_d_n17;
        locals.var_q_dep0_rv = 0.0;

        let (assign15070_e21834, assign15070_e21834_d_n0, assign15070_e21834_d_n2, assign15070_e21834_d_n6, assign15070_e21834_d_n7, assign15070_e21834_d_n10, assign15070_e21834_d_n11, assign15070_e21834_d_n12, assign15070_e21834_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign15070_e21834;
        locals.var_ps0_dn0 = assign15070_e21834_d_n0;
        locals.var_ps0_dn2 = assign15070_e21834_d_n2;
        locals.var_ps0_dn6 = assign15070_e21834_d_n6;
        locals.var_ps0_dn7 = assign15070_e21834_d_n7;
        locals.var_ps0_dn10 = assign15070_e21834_d_n10;
        locals.var_ps0_dn11 = assign15070_e21834_d_n11;
        locals.var_ps0_dn12 = assign15070_e21834_d_n12;
        locals.var_ps0_dn17 = assign15070_e21834_d_n17;
        locals.var_ps0_rv = 0.0;

        let (assign15090_e21846, assign15090_e21846_d_n0, assign15090_e21846_d_n2, assign15090_e21846_d_n6, assign15090_e21846_d_n7, assign15090_e21846_d_n10, assign15090_e21846_d_n11, assign15090_e21846_d_n12, assign15090_e21846_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15090_e21844: f64 = (locals.var_q_deps0 / locals.var_cnst0soi);
        (assign15090_e21844, (((locals.var_q_deps0_dn0 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn0)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn2 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn2)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn6 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn6)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn7 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn7)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn10 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn10)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn11 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn11)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn12 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn12)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn17 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn17)) / (locals.var_cnst0soi * locals.var_cnst0soi)),)
    } else {
        (locals.var_q_deps0_soi_o_cnst0soi, locals.var_q_deps0_soi_o_cnst0soi_dn0, locals.var_q_deps0_soi_o_cnst0soi_dn2, locals.var_q_deps0_soi_o_cnst0soi_dn6, locals.var_q_deps0_soi_o_cnst0soi_dn7, locals.var_q_deps0_soi_o_cnst0soi_dn10, locals.var_q_deps0_soi_o_cnst0soi_dn11, locals.var_q_deps0_soi_o_cnst0soi_dn12, locals.var_q_deps0_soi_o_cnst0soi_dn17,)
    }
};
        locals.var_q_deps0_soi_o_cnst0soi = assign15090_e21846;
        locals.var_q_deps0_soi_o_cnst0soi_dn0 = assign15090_e21846_d_n0;
        locals.var_q_deps0_soi_o_cnst0soi_dn2 = assign15090_e21846_d_n2;
        locals.var_q_deps0_soi_o_cnst0soi_dn6 = assign15090_e21846_d_n6;
        locals.var_q_deps0_soi_o_cnst0soi_dn7 = assign15090_e21846_d_n7;
        locals.var_q_deps0_soi_o_cnst0soi_dn10 = assign15090_e21846_d_n10;
        locals.var_q_deps0_soi_o_cnst0soi_dn11 = assign15090_e21846_d_n11;
        locals.var_q_deps0_soi_o_cnst0soi_dn12 = assign15090_e21846_d_n12;
        locals.var_q_deps0_soi_o_cnst0soi_dn17 = assign15090_e21846_d_n17;
        locals.var_q_deps0_soi_o_cnst0soi_rv = 0.0;

        let (assign15100_e21857, assign15100_e21857_d_n0, assign15100_e21857_d_n2, assign15100_e21857_d_n6, assign15100_e21857_d_n7, assign15100_e21857_d_n10, assign15100_e21857_d_n11, assign15100_e21857_d_n12, assign15100_e21857_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15100_e21851: f64 = (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi);
        let assign15100_e21854: f64 = (10.0 * 2.220446049250313e-16);
        let assign15100_e21855: f64 = (assign15100_e21851 + assign15100_e21854);
        (assign15100_e21855, ((locals.var_q_deps0_soi_o_cnst0soi_dn0 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn0)), ((locals.var_q_deps0_soi_o_cnst0soi_dn2 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn2)), ((locals.var_q_deps0_soi_o_cnst0soi_dn6 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn6)), ((locals.var_q_deps0_soi_o_cnst0soi_dn7 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn7)), ((locals.var_q_deps0_soi_o_cnst0soi_dn10 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn10)), ((locals.var_q_deps0_soi_o_cnst0soi_dn11 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn11)), ((locals.var_q_deps0_soi_o_cnst0soi_dn12 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn12)), ((locals.var_q_deps0_soi_o_cnst0soi_dn17 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn17)),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn6, locals.var_xi0_dn7, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12, locals.var_xi0_dn17,)
    }
};
        locals.var_xi0 = assign15100_e21857;
        locals.var_xi0_dn0 = assign15100_e21857_d_n0;
        locals.var_xi0_dn2 = assign15100_e21857_d_n2;
        locals.var_xi0_dn6 = assign15100_e21857_d_n6;
        locals.var_xi0_dn7 = assign15100_e21857_d_n7;
        locals.var_xi0_dn10 = assign15100_e21857_d_n10;
        locals.var_xi0_dn11 = assign15100_e21857_d_n11;
        locals.var_xi0_dn12 = assign15100_e21857_d_n12;
        locals.var_xi0_dn17 = assign15100_e21857_d_n17;
        locals.var_xi0_rv = 0.0;

        let (assign15110_e21864, assign15110_e21864_d_n0, assign15110_e21864_d_n2, assign15110_e21864_d_n6, assign15110_e21864_d_n7, assign15110_e21864_d_n10, assign15110_e21864_d_n11, assign15110_e21864_d_n12, assign15110_e21864_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15110_e21862: f64 = (2.0 * locals.var_q_deps0_soi_o_cnst0soi);
        (assign15110_e21862, (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn0), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn2), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn6), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn7), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn10), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn11), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn12), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15110_e21864;
        locals.var_t1_dn0 = assign15110_e21864_d_n0;
        locals.var_t1_dn2 = assign15110_e21864_d_n2;
        locals.var_t1_dn6 = assign15110_e21864_d_n6;
        locals.var_t1_dn7 = assign15110_e21864_d_n7;
        locals.var_t1_dn10 = assign15110_e21864_d_n10;
        locals.var_t1_dn11 = assign15110_e21864_d_n11;
        locals.var_t1_dn12 = assign15110_e21864_d_n12;
        locals.var_t1_dn17 = assign15110_e21864_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15120_e21873, assign15120_e21873_d_n0, assign15120_e21873_d_n2, assign15120_e21873_d_n6, assign15120_e21873_d_n7, assign15120_e21873_d_n10, assign15120_e21873_d_n11, assign15120_e21873_d_n12, assign15120_e21873_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15120_e21870: f64 = (10.0 * 2.220446049250313e-16);
        let assign15120_e21871: f64 = (locals.var_q_deps0_soi_o_cnst0soi + assign15120_e21870);
        (assign15120_e21871, locals.var_q_deps0_soi_o_cnst0soi_dn0, locals.var_q_deps0_soi_o_cnst0soi_dn2, locals.var_q_deps0_soi_o_cnst0soi_dn6, locals.var_q_deps0_soi_o_cnst0soi_dn7, locals.var_q_deps0_soi_o_cnst0soi_dn10, locals.var_q_deps0_soi_o_cnst0soi_dn11, locals.var_q_deps0_soi_o_cnst0soi_dn12, locals.var_q_deps0_soi_o_cnst0soi_dn17,)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12, locals.var_xi0p12_dn17,)
    }
};
        locals.var_xi0p12 = assign15120_e21873;
        locals.var_xi0p12_dn0 = assign15120_e21873_d_n0;
        locals.var_xi0p12_dn2 = assign15120_e21873_d_n2;
        locals.var_xi0p12_dn6 = assign15120_e21873_d_n6;
        locals.var_xi0p12_dn7 = assign15120_e21873_d_n7;
        locals.var_xi0p12_dn10 = assign15120_e21873_d_n10;
        locals.var_xi0p12_dn11 = assign15120_e21873_d_n11;
        locals.var_xi0p12_dn12 = assign15120_e21873_d_n12;
        locals.var_xi0p12_dn17 = assign15120_e21873_d_n17;
        locals.var_xi0p12_rv = 0.0;

        let (assign15130_e21880, assign15130_e21880_d_n0, assign15130_e21880_d_n2, assign15130_e21880_d_n6, assign15130_e21880_d_n7, assign15130_e21880_d_n10, assign15130_e21880_d_n11, assign15130_e21880_d_n12, assign15130_e21880_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15130_e21878: f64 = (locals.var_cnst0soi * locals.var_xi0p12);
        (assign15130_e21878, ((locals.var_cnst0soi_dn0 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn17)),)
    } else {
        (locals.var_qb0, locals.var_qb0_dn0, locals.var_qb0_dn2, locals.var_qb0_dn6, locals.var_qb0_dn7, locals.var_qb0_dn10, locals.var_qb0_dn11, locals.var_qb0_dn12, locals.var_qb0_dn17,)
    }
};
        locals.var_qb0 = assign15130_e21880;
        locals.var_qb0_dn0 = assign15130_e21880_d_n0;
        locals.var_qb0_dn2 = assign15130_e21880_d_n2;
        locals.var_qb0_dn6 = assign15130_e21880_d_n6;
        locals.var_qb0_dn7 = assign15130_e21880_d_n7;
        locals.var_qb0_dn10 = assign15130_e21880_d_n10;
        locals.var_qb0_dn11 = assign15130_e21880_d_n11;
        locals.var_qb0_dn12 = assign15130_e21880_d_n12;
        locals.var_qb0_dn17 = assign15130_e21880_d_n17;
        locals.var_qb0_rv = 0.0;

        let (assign15140_e21889, assign15140_e21889_d_n0, assign15140_e21889_d_n2, assign15140_e21889_d_n6, assign15140_e21889_d_n7, assign15140_e21889_d_n10, assign15140_e21889_d_n11, assign15140_e21889_d_n12, assign15140_e21889_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15140_e21886: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign15140_e21887: f64 = (1.0 / assign15140_e21886);
        (assign15140_e21887, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn12 + locals.var_xi0p12_dn12) / (assign15140_e21886 * assign15140_e21886))), (-((locals.var_fs02_dn17 + locals.var_xi0p12_dn17) / (assign15140_e21886 * assign15140_e21886))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15140_e21889;
        locals.var_t1_dn0 = assign15140_e21889_d_n0;
        locals.var_t1_dn2 = assign15140_e21889_d_n2;
        locals.var_t1_dn6 = assign15140_e21889_d_n6;
        locals.var_t1_dn7 = assign15140_e21889_d_n7;
        locals.var_t1_dn10 = assign15140_e21889_d_n10;
        locals.var_t1_dn11 = assign15140_e21889_d_n11;
        locals.var_t1_dn12 = assign15140_e21889_d_n12;
        locals.var_t1_dn17 = assign15140_e21889_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15150_e21898, assign15150_e21898_d_n0, assign15150_e21898_d_n2, assign15150_e21898_d_n6, assign15150_e21898_d_n7, assign15150_e21898_d_n10, assign15150_e21898_d_n11, assign15150_e21898_d_n12, assign15150_e21898_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15150_e21894: f64 = (locals.var_cnst0soi * locals.var_fs01);
        let assign15150_e21896: f64 = (assign15150_e21894 * locals.var_t1);
        (assign15150_e21896, ((((locals.var_cnst0soi_dn0 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn0)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn2)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn6)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn7)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn10)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn11)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn12)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn17)) * locals.var_t1) + (assign15150_e21894 * locals.var_t1_dn17)),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign15150_e21898;
        locals.var_qn0_dn0 = assign15150_e21898_d_n0;
        locals.var_qn0_dn2 = assign15150_e21898_d_n2;
        locals.var_qn0_dn6 = assign15150_e21898_d_n6;
        locals.var_qn0_dn7 = assign15150_e21898_d_n7;
        locals.var_qn0_dn10 = assign15150_e21898_d_n10;
        locals.var_qn0_dn11 = assign15150_e21898_d_n11;
        locals.var_qn0_dn12 = assign15150_e21898_d_n12;
        locals.var_qn0_dn17 = assign15150_e21898_d_n17;
        locals.var_qn0_rv = 0.0;

        let (assign15160_e21904, assign15160_e21904_d_n0, assign15160_e21904_d_n2, assign15160_e21904_d_n6, assign15160_e21904_d_n7, assign15160_e21904_d_n10, assign15160_e21904_d_n11, assign15160_e21904_d_n12, assign15160_e21904_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15160_e21902: f64 = (-locals.var_qn0);
        (assign15160_e21902, (-locals.var_qn0_dn0), (-locals.var_qn0_dn2), (-locals.var_qn0_dn6), (-locals.var_qn0_dn7), (-locals.var_qn0_dn10), (-locals.var_qn0_dn11), (-locals.var_qn0_dn12), (-locals.var_qn0_dn17),)
    } else {
        (locals.var_q_n0, locals.var_q_n0_dn0, locals.var_q_n0_dn2, locals.var_q_n0_dn6, locals.var_q_n0_dn7, locals.var_q_n0_dn10, locals.var_q_n0_dn11, locals.var_q_n0_dn12, locals.var_q_n0_dn17,)
    }
};
        locals.var_q_n0 = assign15160_e21904;
        locals.var_q_n0_dn0 = assign15160_e21904_d_n0;
        locals.var_q_n0_dn2 = assign15160_e21904_d_n2;
        locals.var_q_n0_dn6 = assign15160_e21904_d_n6;
        locals.var_q_n0_dn7 = assign15160_e21904_d_n7;
        locals.var_q_n0_dn10 = assign15160_e21904_d_n10;
        locals.var_q_n0_dn11 = assign15160_e21904_d_n11;
        locals.var_q_n0_dn12 = assign15160_e21904_d_n12;
        locals.var_q_n0_dn17 = assign15160_e21904_d_n17;
        locals.var_q_n0_rv = 0.0;

        let (assign15170_e21911, assign15170_e21911_d_n0, assign15170_e21911_d_n2, assign15170_e21911_d_n6, assign15170_e21911_d_n7, assign15170_e21911_d_n10, assign15170_e21911_d_n11, assign15170_e21911_d_n12, assign15170_e21911_d_n17,) = {
    if (locals.var_guard109 == 0.0) {
        let assign15170_e21909: f64 = (locals.var_qn0 * locals.var_c_fox_inv);
        (assign15170_e21909, ((locals.var_qn0_dn0 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn2)), ((locals.var_qn0_dn6 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn7)), ((locals.var_qn0_dn10 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn10)), ((locals.var_qn0_dn11 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn11)), ((locals.var_qn0_dn12 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn12)), ((locals.var_qn0_dn17 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign15170_e21911;
        locals.var_vgvt_dn0 = assign15170_e21911_d_n0;
        locals.var_vgvt_dn2 = assign15170_e21911_d_n2;
        locals.var_vgvt_dn6 = assign15170_e21911_d_n6;
        locals.var_vgvt_dn7 = assign15170_e21911_d_n7;
        locals.var_vgvt_dn10 = assign15170_e21911_d_n10;
        locals.var_vgvt_dn11 = assign15170_e21911_d_n11;
        locals.var_vgvt_dn12 = assign15170_e21911_d_n12;
        locals.var_vgvt_dn17 = assign15170_e21911_d_n17;
        locals.var_vgvt_rv = 0.0;

        let assign15180_e21914: f64 = (-1.0);
        let assign15180_e21919: f64 = if ((locals.var_flg_zone == assign15180_e21914) || (locals.var_vgvt <= 1e-12)) { 1.0 } else { 0.0 };
        locals.var_guard460 = assign15180_e21919;
        locals.var_guard460_rv = 0.0;

        let (assign15190_e21926,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign15190_e21926;
        locals.var_flg_zone_rv = 0.0;

        let (assign15200_e21933,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign15200_e21933;
        locals.var_flg_noqi_rv = 0.0;

        let (assign15210_e21942, assign15210_e21942_d_n0, assign15210_e21942_d_n2, assign15210_e21942_d_n6, assign15210_e21942_d_n7, assign15210_e21942_d_n10, assign15210_e21942_d_n11, assign15210_e21942_d_n12, assign15210_e21942_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15210_e21940: f64 = (locals.var_vgp - locals.var_ps0);
        (assign15210_e21940, (locals.var_vgp_dn0 - locals.var_ps0_dn0), (locals.var_vgp_dn2 - locals.var_ps0_dn2), (locals.var_vgp_dn6 - locals.var_ps0_dn6), (locals.var_vgp_dn7 - locals.var_ps0_dn7), (locals.var_vgp_dn10 - locals.var_ps0_dn10), (locals.var_vgp_dn11 - locals.var_ps0_dn11), (locals.var_vgp_dn12 - locals.var_ps0_dn12), (locals.var_vgp_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign15210_e21942;
        locals.var_t2_dn0 = assign15210_e21942_d_n0;
        locals.var_t2_dn2 = assign15210_e21942_d_n2;
        locals.var_t2_dn6 = assign15210_e21942_d_n6;
        locals.var_t2_dn7 = assign15210_e21942_d_n7;
        locals.var_t2_dn10 = assign15210_e21942_d_n10;
        locals.var_t2_dn11 = assign15210_e21942_d_n11;
        locals.var_t2_dn12 = assign15210_e21942_d_n12;
        locals.var_t2_dn17 = assign15210_e21942_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign15220_e21951, assign15220_e21951_d_n0, assign15220_e21951_d_n2, assign15220_e21951_d_n6, assign15220_e21951_d_n7, assign15220_e21951_d_n10, assign15220_e21951_d_n11, assign15220_e21951_d_n12, assign15220_e21951_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15220_e21949: f64 = (locals.var_c_fox * locals.var_t2);
        (assign15220_e21949, ((locals.var_c_fox_dn0 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn0)), ((locals.var_c_fox_dn2 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn2)), ((locals.var_c_fox_dn6 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn6)), ((locals.var_c_fox_dn7 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn7)), ((locals.var_c_fox_dn10 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn10)), ((locals.var_c_fox_dn11 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn11)), ((locals.var_c_fox_dn12 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn12)), ((locals.var_c_fox_dn17 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign15220_e21951;
        locals.var_qbu_dn0 = assign15220_e21951_d_n0;
        locals.var_qbu_dn2 = assign15220_e21951_d_n2;
        locals.var_qbu_dn6 = assign15220_e21951_d_n6;
        locals.var_qbu_dn7 = assign15220_e21951_d_n7;
        locals.var_qbu_dn10 = assign15220_e21951_d_n10;
        locals.var_qbu_dn11 = assign15220_e21951_d_n11;
        locals.var_qbu_dn12 = assign15220_e21951_d_n12;
        locals.var_qbu_dn17 = assign15220_e21951_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign15230_e21961, assign15230_e21961_d_n0, assign15230_e21961_d_n2, assign15230_e21961_d_n6, assign15230_e21961_d_n7, assign15230_e21961_d_n10, assign15230_e21961_d_n11, assign15230_e21961_d_n12, assign15230_e21961_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15230_e21957: f64 = (-locals.var_weffcv_nf);
        let assign15230_e21959: f64 = (assign15230_e21957 * locals.var_leff_cv);
        (assign15230_e21959, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign15230_e21961;
        locals.var_t0_dn0 = assign15230_e21961_d_n0;
        locals.var_t0_dn2 = assign15230_e21961_d_n2;
        locals.var_t0_dn6 = assign15230_e21961_d_n6;
        locals.var_t0_dn7 = assign15230_e21961_d_n7;
        locals.var_t0_dn10 = assign15230_e21961_d_n10;
        locals.var_t0_dn11 = assign15230_e21961_d_n11;
        locals.var_t0_dn12 = assign15230_e21961_d_n12;
        locals.var_t0_dn17 = assign15230_e21961_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign15240_e21970, assign15240_e21970_d_n0, assign15240_e21970_d_n2, assign15240_e21970_d_n6, assign15240_e21970_d_n7, assign15240_e21970_d_n10, assign15240_e21970_d_n11, assign15240_e21970_d_n12, assign15240_e21970_d_n13, assign15240_e21970_d_n15, assign15240_e21970_d_n16, assign15240_e21970_d_n17, assign15240_e21970_d_n18,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15240_e21968: f64 = (locals.var_t0 * locals.var_qbu);
        (assign15240_e21968, ((locals.var_t0_dn0 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn0)), ((locals.var_t0_dn2 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn2)), ((locals.var_t0_dn6 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn6)), ((locals.var_t0_dn7 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn7)), ((locals.var_t0_dn10 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn10)), ((locals.var_t0_dn11 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn11)), ((locals.var_t0_dn12 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t0_dn17 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign15240_e21970;
        locals.var_qb_dn0 = assign15240_e21970_d_n0;
        locals.var_qb_dn2 = assign15240_e21970_d_n2;
        locals.var_qb_dn6 = assign15240_e21970_d_n6;
        locals.var_qb_dn7 = assign15240_e21970_d_n7;
        locals.var_qb_dn10 = assign15240_e21970_d_n10;
        locals.var_qb_dn11 = assign15240_e21970_d_n11;
        locals.var_qb_dn12 = assign15240_e21970_d_n12;
        locals.var_qb_dn13 = assign15240_e21970_d_n13;
        locals.var_qb_dn15 = assign15240_e21970_d_n15;
        locals.var_qb_dn16 = assign15240_e21970_d_n16;
        locals.var_qb_dn17 = assign15240_e21970_d_n17;
        locals.var_qb_dn18 = assign15240_e21970_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign15250_e21977, assign15250_e21977_d_n0, assign15250_e21977_d_n2, assign15250_e21977_d_n6, assign15250_e21977_d_n7, assign15250_e21977_d_n10, assign15250_e21977_d_n11, assign15250_e21977_d_n12, assign15250_e21977_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign15250_e21977;
        locals.var_qi_dn0 = assign15250_e21977_d_n0;
        locals.var_qi_dn2 = assign15250_e21977_d_n2;
        locals.var_qi_dn6 = assign15250_e21977_d_n6;
        locals.var_qi_dn7 = assign15250_e21977_d_n7;
        locals.var_qi_dn10 = assign15250_e21977_d_n10;
        locals.var_qi_dn11 = assign15250_e21977_d_n11;
        locals.var_qi_dn12 = assign15250_e21977_d_n12;
        locals.var_qi_dn17 = assign15250_e21977_d_n17;
        locals.var_qi_rv = 0.0;

        let (assign15260_e21984, assign15260_e21984_d_n0, assign15260_e21984_d_n2, assign15260_e21984_d_n6, assign15260_e21984_d_n7, assign15260_e21984_d_n10, assign15260_e21984_d_n11, assign15260_e21984_d_n12, assign15260_e21984_d_n13, assign15260_e21984_d_n15, assign15260_e21984_d_n16, assign15260_e21984_d_n17, assign15260_e21984_d_n18,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign15260_e21984;
        locals.var_qd_dn0 = assign15260_e21984_d_n0;
        locals.var_qd_dn2 = assign15260_e21984_d_n2;
        locals.var_qd_dn6 = assign15260_e21984_d_n6;
        locals.var_qd_dn7 = assign15260_e21984_d_n7;
        locals.var_qd_dn10 = assign15260_e21984_d_n10;
        locals.var_qd_dn11 = assign15260_e21984_d_n11;
        locals.var_qd_dn12 = assign15260_e21984_d_n12;
        locals.var_qd_dn13 = assign15260_e21984_d_n13;
        locals.var_qd_dn15 = assign15260_e21984_d_n15;
        locals.var_qd_dn16 = assign15260_e21984_d_n16;
        locals.var_qd_dn17 = assign15260_e21984_d_n17;
        locals.var_qd_dn18 = assign15260_e21984_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign15270_e21994, assign15270_e21994_d_n0, assign15270_e21994_d_n2, assign15270_e21994_d_n6, assign15270_e21994_d_n7, assign15270_e21994_d_n10, assign15270_e21994_d_n11, assign15270_e21994_d_n12, assign15270_e21994_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15270_e21990: f64 = (-locals.var_area_bt_n);
        let assign15270_e21992: f64 = (assign15270_e21990 * locals.var_qbu);
        (assign15270_e21992, (assign15270_e21990 * locals.var_qbu_dn0), (assign15270_e21990 * locals.var_qbu_dn2), (assign15270_e21990 * locals.var_qbu_dn6), (assign15270_e21990 * locals.var_qbu_dn7), (assign15270_e21990 * locals.var_qbu_dn10), (assign15270_e21990 * locals.var_qbu_dn11), (assign15270_e21990 * locals.var_qbu_dn12), (assign15270_e21990 * locals.var_qbu_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign15270_e21994;
        locals.var_t2_dn0 = assign15270_e21994_d_n0;
        locals.var_t2_dn2 = assign15270_e21994_d_n2;
        locals.var_t2_dn6 = assign15270_e21994_d_n6;
        locals.var_t2_dn7 = assign15270_e21994_d_n7;
        locals.var_t2_dn10 = assign15270_e21994_d_n10;
        locals.var_t2_dn11 = assign15270_e21994_d_n11;
        locals.var_t2_dn12 = assign15270_e21994_d_n12;
        locals.var_t2_dn17 = assign15270_e21994_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign15280_e22003, assign15280_e22003_d_n0, assign15280_e22003_d_n2, assign15280_e22003_d_n6, assign15280_e22003_d_n7, assign15280_e22003_d_n10, assign15280_e22003_d_n11, assign15280_e22003_d_n12, assign15280_e22003_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15280_e22001: f64 = (locals.var_t2 * locals.var_qdrat);
        (assign15280_e22001, ((locals.var_t2_dn0 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn0)), ((locals.var_t2_dn2 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn2)), ((locals.var_t2_dn6 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn6)), ((locals.var_t2_dn7 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn7)), ((locals.var_t2_dn10 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn10)), ((locals.var_t2_dn11 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn11)), ((locals.var_t2_dn12 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn12)), ((locals.var_t2_dn17 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign15280_e22003;
        locals.var_qbody_bt_n_sud_dn0 = assign15280_e22003_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign15280_e22003_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign15280_e22003_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign15280_e22003_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign15280_e22003_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign15280_e22003_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign15280_e22003_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign15280_e22003_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign15290_e22012, assign15290_e22012_d_n0, assign15290_e22012_d_n2, assign15290_e22012_d_n6, assign15290_e22012_d_n7, assign15290_e22012_d_n10, assign15290_e22012_d_n11, assign15290_e22012_d_n12, assign15290_e22012_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        let assign15290_e22010: f64 = (locals.var_t2 - locals.var_qbody_bt_n_sud);
        (assign15290_e22010, (locals.var_t2_dn0 - locals.var_qbody_bt_n_sud_dn0), (locals.var_t2_dn2 - locals.var_qbody_bt_n_sud_dn2), (locals.var_t2_dn6 - locals.var_qbody_bt_n_sud_dn6), (locals.var_t2_dn7 - locals.var_qbody_bt_n_sud_dn7), (locals.var_t2_dn10 - locals.var_qbody_bt_n_sud_dn10), (locals.var_t2_dn11 - locals.var_qbody_bt_n_sud_dn11), (locals.var_t2_dn12 - locals.var_qbody_bt_n_sud_dn12), (locals.var_t2_dn17 - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign15290_e22012;
        locals.var_qbody_bt_n_sus_dn0 = assign15290_e22012_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign15290_e22012_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign15290_e22012_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign15290_e22012_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign15290_e22012_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign15290_e22012_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign15290_e22012_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign15290_e22012_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign15300_e22019, assign15300_e22019_d_n0, assign15300_e22019_d_n2, assign15300_e22019_d_n6, assign15300_e22019_d_n7, assign15300_e22019_d_n10, assign15300_e22019_d_n11, assign15300_e22019_d_n12, assign15300_e22019_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign15300_e22019;
        locals.var_qbody_bt_n_iud_dn0 = assign15300_e22019_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign15300_e22019_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign15300_e22019_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign15300_e22019_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign15300_e22019_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign15300_e22019_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign15300_e22019_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign15300_e22019_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        let (assign15310_e22026, assign15310_e22026_d_n0, assign15310_e22026_d_n2, assign15310_e22026_d_n6, assign15310_e22026_d_n7, assign15310_e22026_d_n10, assign15310_e22026_d_n11, assign15310_e22026_d_n12, assign15310_e22026_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign15310_e22026;
        locals.var_qbody_bt_n_ius_dn0 = assign15310_e22026_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign15310_e22026_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign15310_e22026_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign15310_e22026_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign15310_e22026_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign15310_e22026_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign15310_e22026_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign15310_e22026_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign15320_e22033, assign15320_e22033_d_n0, assign15320_e22033_d_n2, assign15320_e22033_d_n6, assign15320_e22033_d_n7, assign15320_e22033_d_n10, assign15320_e22033_d_n11, assign15320_e22033_d_n12, assign15320_e22033_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign15320_e22033;
        locals.var_ids_dn0 = assign15320_e22033_d_n0;
        locals.var_ids_dn2 = assign15320_e22033_d_n2;
        locals.var_ids_dn6 = assign15320_e22033_d_n6;
        locals.var_ids_dn7 = assign15320_e22033_d_n7;
        locals.var_ids_dn10 = assign15320_e22033_d_n10;
        locals.var_ids_dn11 = assign15320_e22033_d_n11;
        locals.var_ids_dn12 = assign15320_e22033_d_n12;
        locals.var_ids_dn17 = assign15320_e22033_d_n17;
        locals.var_ids_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        locals: &mut StampLocals,
    ) {
        let (assign15330_e22040, assign15330_e22040_d_n0, assign15330_e22040_d_n2, assign15330_e22040_d_n6, assign15330_e22040_d_n7, assign15330_e22040_d_n10, assign15330_e22040_d_n11, assign15330_e22040_d_n12, assign15330_e22040_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign15330_e22040;
        locals.var_vgvt_dn0 = assign15330_e22040_d_n0;
        locals.var_vgvt_dn2 = assign15330_e22040_d_n2;
        locals.var_vgvt_dn6 = assign15330_e22040_d_n6;
        locals.var_vgvt_dn7 = assign15330_e22040_d_n7;
        locals.var_vgvt_dn10 = assign15330_e22040_d_n10;
        locals.var_vgvt_dn11 = assign15330_e22040_d_n11;
        locals.var_vgvt_dn12 = assign15330_e22040_d_n12;
        locals.var_vgvt_dn17 = assign15330_e22040_d_n17;
        locals.var_vgvt_rv = 0.0;

        let (assign15340_e22047,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign15340_e22047;
        locals.var_flg_noqi_rv = 0.0;

        let (assign15350_e22054, assign15350_e22054_d_n0, assign15350_e22054_d_n2, assign15350_e22054_d_n6, assign15350_e22054_d_n7, assign15350_e22054_d_n10, assign15350_e22054_d_n11, assign15350_e22054_d_n12, assign15350_e22054_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign15350_e22054;
        locals.var_phi_sl_soi_dn0 = assign15350_e22054_d_n0;
        locals.var_phi_sl_soi_dn2 = assign15350_e22054_d_n2;
        locals.var_phi_sl_soi_dn6 = assign15350_e22054_d_n6;
        locals.var_phi_sl_soi_dn7 = assign15350_e22054_d_n7;
        locals.var_phi_sl_soi_dn10 = assign15350_e22054_d_n10;
        locals.var_phi_sl_soi_dn11 = assign15350_e22054_d_n11;
        locals.var_phi_sl_soi_dn12 = assign15350_e22054_d_n12;
        locals.var_phi_sl_soi_dn17 = assign15350_e22054_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign15360_e22061, assign15360_e22061_d_n0, assign15360_e22061_d_n2, assign15360_e22061_d_n6, assign15360_e22061_d_n7, assign15360_e22061_d_n10, assign15360_e22061_d_n11, assign15360_e22061_d_n12, assign15360_e22061_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign15360_e22061;
        locals.var_psl_dn0 = assign15360_e22061_d_n0;
        locals.var_psl_dn2 = assign15360_e22061_d_n2;
        locals.var_psl_dn6 = assign15360_e22061_d_n6;
        locals.var_psl_dn7 = assign15360_e22061_d_n7;
        locals.var_psl_dn10 = assign15360_e22061_d_n10;
        locals.var_psl_dn11 = assign15360_e22061_d_n11;
        locals.var_psl_dn12 = assign15360_e22061_d_n12;
        locals.var_psl_dn17 = assign15360_e22061_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign15370_e22068, assign15370_e22068_d_n0, assign15370_e22068_d_n2, assign15370_e22068_d_n6, assign15370_e22068_d_n7, assign15370_e22068_d_n10, assign15370_e22068_d_n11, assign15370_e22068_d_n12, assign15370_e22068_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign15370_e22068;
        locals.var_psdl_dn0 = assign15370_e22068_d_n0;
        locals.var_psdl_dn2 = assign15370_e22068_d_n2;
        locals.var_psdl_dn6 = assign15370_e22068_d_n6;
        locals.var_psdl_dn7 = assign15370_e22068_d_n7;
        locals.var_psdl_dn10 = assign15370_e22068_d_n10;
        locals.var_psdl_dn11 = assign15370_e22068_d_n11;
        locals.var_psdl_dn12 = assign15370_e22068_d_n12;
        locals.var_psdl_dn17 = assign15370_e22068_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign15380_e22075,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard460 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign15380_e22075;
        locals.var_end_of_part_1_rv = 0.0;

        let assign15390_e22078: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign15390_e22078;
        locals.var_guard461_rv = 0.0;

        let (assign15400_e22085, assign15400_e22085_d_n0, assign15400_e22085_d_n2, assign15400_e22085_d_n6, assign15400_e22085_d_n7, assign15400_e22085_d_n10, assign15400_e22085_d_n11, assign15400_e22085_d_n12, assign15400_e22085_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    }
};
        locals.var_vdsorg = assign15400_e22085;
        locals.var_vdsorg_dn0 = assign15400_e22085_d_n0;
        locals.var_vdsorg_dn2 = assign15400_e22085_d_n2;
        locals.var_vdsorg_dn6 = assign15400_e22085_d_n6;
        locals.var_vdsorg_dn7 = assign15400_e22085_d_n7;
        locals.var_vdsorg_dn10 = assign15400_e22085_d_n10;
        locals.var_vdsorg_dn11 = assign15400_e22085_d_n11;
        locals.var_vdsorg_dn12 = assign15400_e22085_d_n12;
        locals.var_vdsorg_dn17 = assign15400_e22085_d_n17;
        locals.var_vdsorg_rv = 0.0;

        let (assign15410_e22092, assign15410_e22092_d_n0, assign15410_e22092_d_n2, assign15410_e22092_d_n6, assign15410_e22092_d_n7, assign15410_e22092_d_n10, assign15410_e22092_d_n11, assign15410_e22092_d_n12, assign15410_e22092_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10__blk468, locals.var_t10__blk468_dn0, locals.var_t10__blk468_dn2, locals.var_t10__blk468_dn6, locals.var_t10__blk468_dn7, locals.var_t10__blk468_dn10, locals.var_t10__blk468_dn11, locals.var_t10__blk468_dn12, locals.var_t10__blk468_dn17,)
    }
};
        locals.var_t10__blk468 = assign15410_e22092;
        locals.var_t10__blk468_dn0 = assign15410_e22092_d_n0;
        locals.var_t10__blk468_dn2 = assign15410_e22092_d_n2;
        locals.var_t10__blk468_dn6 = assign15410_e22092_d_n6;
        locals.var_t10__blk468_dn7 = assign15410_e22092_d_n7;
        locals.var_t10__blk468_dn10 = assign15410_e22092_d_n10;
        locals.var_t10__blk468_dn11 = assign15410_e22092_d_n11;
        locals.var_t10__blk468_dn12 = assign15410_e22092_d_n12;
        locals.var_t10__blk468_dn17 = assign15410_e22092_d_n17;
        locals.var_t10__blk468_rv = 0.0;

        let (assign15420_e22103, assign15420_e22103_d_n0, assign15420_e22103_d_n2, assign15420_e22103_d_n6, assign15420_e22103_d_n7, assign15420_e22103_d_n10, assign15420_e22103_d_n11, assign15420_e22103_d_n12, assign15420_e22103_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15420_e22100: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign15420_e22101: f64 = (locals.var_qnsub_esi / assign15420_e22100);
        (assign15420_e22101, (((locals.var_qnsub_esi_dn0 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn2 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn6 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn7 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn10 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn11 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn12 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign15420_e22100 * assign15420_e22100)), (((locals.var_qnsub_esi_dn17 * assign15420_e22100) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign15420_e22100 * assign15420_e22100)),)
    } else {
        (locals.var_t2__blk463, locals.var_t2__blk463_dn0, locals.var_t2__blk463_dn2, locals.var_t2__blk463_dn6, locals.var_t2__blk463_dn7, locals.var_t2__blk463_dn10, locals.var_t2__blk463_dn11, locals.var_t2__blk463_dn12, locals.var_t2__blk463_dn17,)
    }
};
        locals.var_t2__blk463 = assign15420_e22103;
        locals.var_t2__blk463_dn0 = assign15420_e22103_d_n0;
        locals.var_t2__blk463_dn2 = assign15420_e22103_d_n2;
        locals.var_t2__blk463_dn6 = assign15420_e22103_d_n6;
        locals.var_t2__blk463_dn7 = assign15420_e22103_d_n7;
        locals.var_t2__blk463_dn10 = assign15420_e22103_d_n10;
        locals.var_t2__blk463_dn11 = assign15420_e22103_d_n11;
        locals.var_t2__blk463_dn12 = assign15420_e22103_d_n12;
        locals.var_t2__blk463_dn17 = assign15420_e22103_d_n17;
        locals.var_t2__blk463_rv = 0.0;

        let (assign15430_e22118, assign15430_e22118_d_n0, assign15430_e22118_d_n2, assign15430_e22118_d_n6, assign15430_e22118_d_n7, assign15430_e22118_d_n10, assign15430_e22118_d_n11, assign15430_e22118_d_n12, assign15430_e22118_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15430_e22111: f64 = (2.0 / locals.var_t2__blk463);
        let assign15430_e22114: f64 = (locals.var_vgp - locals.var_t10__blk468);
        let assign15430_e22115: f64 = (assign15430_e22111 * assign15430_e22114);
        let assign15430_e22116: f64 = (1.0 + assign15430_e22115);
        (assign15430_e22116, (((-((2.0 * locals.var_t2__blk463_dn0) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn0 - locals.var_t10__blk468_dn0))), (((-((2.0 * locals.var_t2__blk463_dn2) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn2 - locals.var_t10__blk468_dn2))), (((-((2.0 * locals.var_t2__blk463_dn6) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn6 - locals.var_t10__blk468_dn6))), (((-((2.0 * locals.var_t2__blk463_dn7) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn7 - locals.var_t10__blk468_dn7))), (((-((2.0 * locals.var_t2__blk463_dn10) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn10 - locals.var_t10__blk468_dn10))), (((-((2.0 * locals.var_t2__blk463_dn11) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn11 - locals.var_t10__blk468_dn11))), (((-((2.0 * locals.var_t2__blk463_dn12) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn12 - locals.var_t10__blk468_dn12))), (((-((2.0 * locals.var_t2__blk463_dn17) / (locals.var_t2__blk463 * locals.var_t2__blk463))) * assign15430_e22114) + (assign15430_e22111 * (locals.var_vgp_dn17 - locals.var_t10__blk468_dn17))),)
    } else {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    }
};
        locals.var_t4__blk465 = assign15430_e22118;
        locals.var_t4__blk465_dn0 = assign15430_e22118_d_n0;
        locals.var_t4__blk465_dn2 = assign15430_e22118_d_n2;
        locals.var_t4__blk465_dn6 = assign15430_e22118_d_n6;
        locals.var_t4__blk465_dn7 = assign15430_e22118_d_n7;
        locals.var_t4__blk465_dn10 = assign15430_e22118_d_n10;
        locals.var_t4__blk465_dn11 = assign15430_e22118_d_n11;
        locals.var_t4__blk465_dn12 = assign15430_e22118_d_n12;
        locals.var_t4__blk465_dn17 = assign15430_e22118_d_n17;
        locals.var_t4__blk465_rv = 0.0;

        let (assign15440_e22129, assign15440_e22129_d_n0, assign15440_e22129_d_n2, assign15440_e22129_d_n6, assign15440_e22129_d_n7, assign15440_e22129_d_n10, assign15440_e22129_d_n11, assign15440_e22129_d_n12, assign15440_e22129_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15440_e22126: f64 = (2.0 / locals.var_t2__blk463);
        let assign15440_e22127: f64 = (1.0 + assign15440_e22126);
        (assign15440_e22127, (-((2.0 * locals.var_t2__blk463_dn0) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn2) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn6) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn7) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn10) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn11) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn12) / (locals.var_t2__blk463 * locals.var_t2__blk463))), (-((2.0 * locals.var_t2__blk463_dn17) / (locals.var_t2__blk463 * locals.var_t2__blk463))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign15440_e22129;
        locals.var_t5_dn0 = assign15440_e22129_d_n0;
        locals.var_t5_dn2 = assign15440_e22129_d_n2;
        locals.var_t5_dn6 = assign15440_e22129_d_n6;
        locals.var_t5_dn7 = assign15440_e22129_d_n7;
        locals.var_t5_dn10 = assign15440_e22129_d_n10;
        locals.var_t5_dn11 = assign15440_e22129_d_n11;
        locals.var_t5_dn12 = assign15440_e22129_d_n12;
        locals.var_t5_dn17 = assign15440_e22129_d_n17;
        locals.var_t5_rv = 0.0;

        let assign15450_e22133: f64 = locals.var_t5;
        let assign15450_e22138: f64 = if ((locals.var_t4__blk465 < assign15450_e22133) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard469 = assign15450_e22138;
        locals.var_guard469_rv = 0.0;

        let (assign15460_e22151, assign15460_e22151_d_n0, assign15460_e22151_d_n2, assign15460_e22151_d_n6, assign15460_e22151_d_n7, assign15460_e22151_d_n10, assign15460_e22151_d_n11, assign15460_e22151_d_n12, assign15460_e22151_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15460_e22147: f64 = locals.var_t5;
        let assign15460_e22149: f64 = (assign15460_e22147 - locals.var_t4__blk465);
        (assign15460_e22149, (locals.var_t5_dn0 - locals.var_t4__blk465_dn0), (locals.var_t5_dn2 - locals.var_t4__blk465_dn2), (locals.var_t5_dn6 - locals.var_t4__blk465_dn6), (locals.var_t5_dn7 - locals.var_t4__blk465_dn7), (locals.var_t5_dn10 - locals.var_t4__blk465_dn10), (locals.var_t5_dn11 - locals.var_t4__blk465_dn11), (locals.var_t5_dn12 - locals.var_t4__blk465_dn12), (locals.var_t5_dn17 - locals.var_t4__blk465_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign15460_e22151;
        locals.var_tmf1_dn0 = assign15460_e22151_d_n0;
        locals.var_tmf1_dn2 = assign15460_e22151_d_n2;
        locals.var_tmf1_dn6 = assign15460_e22151_d_n6;
        locals.var_tmf1_dn7 = assign15460_e22151_d_n7;
        locals.var_tmf1_dn10 = assign15460_e22151_d_n10;
        locals.var_tmf1_dn11 = assign15460_e22151_d_n11;
        locals.var_tmf1_dn12 = assign15460_e22151_d_n12;
        locals.var_tmf1_dn17 = assign15460_e22151_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign15470_e22162, assign15470_e22162_d_n0, assign15470_e22162_d_n2, assign15470_e22162_d_n6, assign15470_e22162_d_n7, assign15470_e22162_d_n10, assign15470_e22162_d_n11, assign15470_e22162_d_n12, assign15470_e22162_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15470_e22160: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign15470_e22160, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign15470_e22162;
        locals.var_x2_dn0 = assign15470_e22162_d_n0;
        locals.var_x2_dn2 = assign15470_e22162_d_n2;
        locals.var_x2_dn6 = assign15470_e22162_d_n6;
        locals.var_x2_dn7 = assign15470_e22162_d_n7;
        locals.var_x2_dn10 = assign15470_e22162_d_n10;
        locals.var_x2_dn11 = assign15470_e22162_d_n11;
        locals.var_x2_dn12 = assign15470_e22162_d_n12;
        locals.var_x2_dn17 = assign15470_e22162_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign15480_e22173, assign15480_e22173_d_n0, assign15480_e22173_d_n2, assign15480_e22173_d_n6, assign15480_e22173_d_n7, assign15480_e22173_d_n10, assign15480_e22173_d_n11, assign15480_e22173_d_n12, assign15480_e22173_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15480_e22171: f64 = (locals.var_t5 * locals.var_t5);
        (assign15480_e22171, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn12 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn12)), ((locals.var_t5_dn17 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign15480_e22173;
        locals.var_xmax2_dn0 = assign15480_e22173_d_n0;
        locals.var_xmax2_dn2 = assign15480_e22173_d_n2;
        locals.var_xmax2_dn6 = assign15480_e22173_d_n6;
        locals.var_xmax2_dn7 = assign15480_e22173_d_n7;
        locals.var_xmax2_dn10 = assign15480_e22173_d_n10;
        locals.var_xmax2_dn11 = assign15480_e22173_d_n11;
        locals.var_xmax2_dn12 = assign15480_e22173_d_n12;
        locals.var_xmax2_dn17 = assign15480_e22173_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign15490_e22182, assign15490_e22182_d_n0, assign15490_e22182_d_n2, assign15490_e22182_d_n6, assign15490_e22182_d_n7, assign15490_e22182_d_n10, assign15490_e22182_d_n11, assign15490_e22182_d_n12, assign15490_e22182_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15490_e22182;
        locals.var_xp_dn0 = assign15490_e22182_d_n0;
        locals.var_xp_dn2 = assign15490_e22182_d_n2;
        locals.var_xp_dn6 = assign15490_e22182_d_n6;
        locals.var_xp_dn7 = assign15490_e22182_d_n7;
        locals.var_xp_dn10 = assign15490_e22182_d_n10;
        locals.var_xp_dn11 = assign15490_e22182_d_n11;
        locals.var_xp_dn12 = assign15490_e22182_d_n12;
        locals.var_xp_dn17 = assign15490_e22182_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15500_e22191, assign15500_e22191_d_n0, assign15500_e22191_d_n2, assign15500_e22191_d_n6, assign15500_e22191_d_n7, assign15500_e22191_d_n10, assign15500_e22191_d_n11, assign15500_e22191_d_n12, assign15500_e22191_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15500_e22191;
        locals.var_xmp_dn0 = assign15500_e22191_d_n0;
        locals.var_xmp_dn2 = assign15500_e22191_d_n2;
        locals.var_xmp_dn6 = assign15500_e22191_d_n6;
        locals.var_xmp_dn7 = assign15500_e22191_d_n7;
        locals.var_xmp_dn10 = assign15500_e22191_d_n10;
        locals.var_xmp_dn11 = assign15500_e22191_d_n11;
        locals.var_xmp_dn12 = assign15500_e22191_d_n12;
        locals.var_xmp_dn17 = assign15500_e22191_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15510_e22200,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15510_e22200;
        locals.var_m0_rv = 0.0;

        let (assign15520_e22209,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15520_e22209;
        locals.var_mm_rv = 0.0;

        let (assign15530_e22218, assign15530_e22218_d_n0, assign15530_e22218_d_n2, assign15530_e22218_d_n6, assign15530_e22218_d_n7, assign15530_e22218_d_n10, assign15530_e22218_d_n11, assign15530_e22218_d_n12, assign15530_e22218_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign15530_e22218;
        locals.var_arg_dn0 = assign15530_e22218_d_n0;
        locals.var_arg_dn2 = assign15530_e22218_d_n2;
        locals.var_arg_dn6 = assign15530_e22218_d_n6;
        locals.var_arg_dn7 = assign15530_e22218_d_n7;
        locals.var_arg_dn10 = assign15530_e22218_d_n10;
        locals.var_arg_dn11 = assign15530_e22218_d_n11;
        locals.var_arg_dn12 = assign15530_e22218_d_n12;
        locals.var_arg_dn17 = assign15530_e22218_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign15540_e22227, assign15540_e22227_d_n0, assign15540_e22227_d_n2, assign15540_e22227_d_n6, assign15540_e22227_d_n7, assign15540_e22227_d_n10, assign15540_e22227_d_n11, assign15540_e22227_d_n12, assign15540_e22227_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15540_e22227;
        locals.var_dnm_dn0 = assign15540_e22227_d_n0;
        locals.var_dnm_dn2 = assign15540_e22227_d_n2;
        locals.var_dnm_dn6 = assign15540_e22227_d_n6;
        locals.var_dnm_dn7 = assign15540_e22227_d_n7;
        locals.var_dnm_dn10 = assign15540_e22227_d_n10;
        locals.var_dnm_dn11 = assign15540_e22227_d_n11;
        locals.var_dnm_dn12 = assign15540_e22227_d_n12;
        locals.var_dnm_dn17 = assign15540_e22227_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15550_e22238, assign15550_e22238_d_n0, assign15550_e22238_d_n2, assign15550_e22238_d_n6, assign15550_e22238_d_n7, assign15550_e22238_d_n10, assign15550_e22238_d_n11, assign15550_e22238_d_n12, assign15550_e22238_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15550_e22236: f64 = (locals.var_xp * locals.var_x2);
        (assign15550_e22236, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15550_e22238;
        locals.var_xp_dn0 = assign15550_e22238_d_n0;
        locals.var_xp_dn2 = assign15550_e22238_d_n2;
        locals.var_xp_dn6 = assign15550_e22238_d_n6;
        locals.var_xp_dn7 = assign15550_e22238_d_n7;
        locals.var_xp_dn10 = assign15550_e22238_d_n10;
        locals.var_xp_dn11 = assign15550_e22238_d_n11;
        locals.var_xp_dn12 = assign15550_e22238_d_n12;
        locals.var_xp_dn17 = assign15550_e22238_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15560_e22249, assign15560_e22249_d_n0, assign15560_e22249_d_n2, assign15560_e22249_d_n6, assign15560_e22249_d_n7, assign15560_e22249_d_n10, assign15560_e22249_d_n11, assign15560_e22249_d_n12, assign15560_e22249_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15560_e22247: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15560_e22247, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15560_e22249;
        locals.var_xmp_dn0 = assign15560_e22249_d_n0;
        locals.var_xmp_dn2 = assign15560_e22249_d_n2;
        locals.var_xmp_dn6 = assign15560_e22249_d_n6;
        locals.var_xmp_dn7 = assign15560_e22249_d_n7;
        locals.var_xmp_dn10 = assign15560_e22249_d_n10;
        locals.var_xmp_dn11 = assign15560_e22249_d_n11;
        locals.var_xmp_dn12 = assign15560_e22249_d_n12;
        locals.var_xmp_dn17 = assign15560_e22249_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15570_e22260, assign15570_e22260_d_n0, assign15570_e22260_d_n2, assign15570_e22260_d_n6, assign15570_e22260_d_n7, assign15570_e22260_d_n10, assign15570_e22260_d_n11, assign15570_e22260_d_n12, assign15570_e22260_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15570_e22258: f64 = (locals.var_xp * locals.var_x2);
        (assign15570_e22258, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15570_e22260;
        locals.var_xp_dn0 = assign15570_e22260_d_n0;
        locals.var_xp_dn2 = assign15570_e22260_d_n2;
        locals.var_xp_dn6 = assign15570_e22260_d_n6;
        locals.var_xp_dn7 = assign15570_e22260_d_n7;
        locals.var_xp_dn10 = assign15570_e22260_d_n10;
        locals.var_xp_dn11 = assign15570_e22260_d_n11;
        locals.var_xp_dn12 = assign15570_e22260_d_n12;
        locals.var_xp_dn17 = assign15570_e22260_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15580_e22271, assign15580_e22271_d_n0, assign15580_e22271_d_n2, assign15580_e22271_d_n6, assign15580_e22271_d_n7, assign15580_e22271_d_n10, assign15580_e22271_d_n11, assign15580_e22271_d_n12, assign15580_e22271_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15580_e22269: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15580_e22269, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15580_e22271;
        locals.var_xmp_dn0 = assign15580_e22271_d_n0;
        locals.var_xmp_dn2 = assign15580_e22271_d_n2;
        locals.var_xmp_dn6 = assign15580_e22271_d_n6;
        locals.var_xmp_dn7 = assign15580_e22271_d_n7;
        locals.var_xmp_dn10 = assign15580_e22271_d_n10;
        locals.var_xmp_dn11 = assign15580_e22271_d_n11;
        locals.var_xmp_dn12 = assign15580_e22271_d_n12;
        locals.var_xmp_dn17 = assign15580_e22271_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15590_e22282, assign15590_e22282_d_n0, assign15590_e22282_d_n2, assign15590_e22282_d_n6, assign15590_e22282_d_n7, assign15590_e22282_d_n10, assign15590_e22282_d_n11, assign15590_e22282_d_n12, assign15590_e22282_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15590_e22280: f64 = (locals.var_xp * locals.var_x2);
        (assign15590_e22280, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15590_e22282;
        locals.var_xp_dn0 = assign15590_e22282_d_n0;
        locals.var_xp_dn2 = assign15590_e22282_d_n2;
        locals.var_xp_dn6 = assign15590_e22282_d_n6;
        locals.var_xp_dn7 = assign15590_e22282_d_n7;
        locals.var_xp_dn10 = assign15590_e22282_d_n10;
        locals.var_xp_dn11 = assign15590_e22282_d_n11;
        locals.var_xp_dn12 = assign15590_e22282_d_n12;
        locals.var_xp_dn17 = assign15590_e22282_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15600_e22293, assign15600_e22293_d_n0, assign15600_e22293_d_n2, assign15600_e22293_d_n6, assign15600_e22293_d_n7, assign15600_e22293_d_n10, assign15600_e22293_d_n11, assign15600_e22293_d_n12, assign15600_e22293_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15600_e22291: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15600_e22291, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15600_e22293;
        locals.var_xmp_dn0 = assign15600_e22293_d_n0;
        locals.var_xmp_dn2 = assign15600_e22293_d_n2;
        locals.var_xmp_dn6 = assign15600_e22293_d_n6;
        locals.var_xmp_dn7 = assign15600_e22293_d_n7;
        locals.var_xmp_dn10 = assign15600_e22293_d_n10;
        locals.var_xmp_dn11 = assign15600_e22293_d_n11;
        locals.var_xmp_dn12 = assign15600_e22293_d_n12;
        locals.var_xmp_dn17 = assign15600_e22293_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15610_e22304, assign15610_e22304_d_n0, assign15610_e22304_d_n2, assign15610_e22304_d_n6, assign15610_e22304_d_n7, assign15610_e22304_d_n10, assign15610_e22304_d_n11, assign15610_e22304_d_n12, assign15610_e22304_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15610_e22302: f64 = (locals.var_xp * locals.var_x2);
        (assign15610_e22302, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15610_e22304;
        locals.var_xp_dn0 = assign15610_e22304_d_n0;
        locals.var_xp_dn2 = assign15610_e22304_d_n2;
        locals.var_xp_dn6 = assign15610_e22304_d_n6;
        locals.var_xp_dn7 = assign15610_e22304_d_n7;
        locals.var_xp_dn10 = assign15610_e22304_d_n10;
        locals.var_xp_dn11 = assign15610_e22304_d_n11;
        locals.var_xp_dn12 = assign15610_e22304_d_n12;
        locals.var_xp_dn17 = assign15610_e22304_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15620_e22315, assign15620_e22315_d_n0, assign15620_e22315_d_n2, assign15620_e22315_d_n6, assign15620_e22315_d_n7, assign15620_e22315_d_n10, assign15620_e22315_d_n11, assign15620_e22315_d_n12, assign15620_e22315_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15620_e22313: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15620_e22313, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15620_e22315;
        locals.var_xmp_dn0 = assign15620_e22315_d_n0;
        locals.var_xmp_dn2 = assign15620_e22315_d_n2;
        locals.var_xmp_dn6 = assign15620_e22315_d_n6;
        locals.var_xmp_dn7 = assign15620_e22315_d_n7;
        locals.var_xmp_dn10 = assign15620_e22315_d_n10;
        locals.var_xmp_dn11 = assign15620_e22315_d_n11;
        locals.var_xmp_dn12 = assign15620_e22315_d_n12;
        locals.var_xmp_dn17 = assign15620_e22315_d_n17;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign15630_e22326, assign15630_e22326_d_n0, assign15630_e22326_d_n2, assign15630_e22326_d_n6, assign15630_e22326_d_n7, assign15630_e22326_d_n10, assign15630_e22326_d_n11, assign15630_e22326_d_n12, assign15630_e22326_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15630_e22324: f64 = (locals.var_xp + locals.var_xmp);
        (assign15630_e22324, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign15630_e22326;
        locals.var_arg_dn0 = assign15630_e22326_d_n0;
        locals.var_arg_dn2 = assign15630_e22326_d_n2;
        locals.var_arg_dn6 = assign15630_e22326_d_n6;
        locals.var_arg_dn7 = assign15630_e22326_d_n7;
        locals.var_arg_dn10 = assign15630_e22326_d_n10;
        locals.var_arg_dn11 = assign15630_e22326_d_n11;
        locals.var_arg_dn12 = assign15630_e22326_d_n12;
        locals.var_arg_dn17 = assign15630_e22326_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign15640_e22335, assign15640_e22335_d_n0, assign15640_e22335_d_n2, assign15640_e22335_d_n6, assign15640_e22335_d_n7, assign15640_e22335_d_n10, assign15640_e22335_d_n11, assign15640_e22335_d_n12, assign15640_e22335_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15640_e22335;
        locals.var_dnm_dn0 = assign15640_e22335_d_n0;
        locals.var_dnm_dn2 = assign15640_e22335_d_n2;
        locals.var_dnm_dn6 = assign15640_e22335_d_n6;
        locals.var_dnm_dn7 = assign15640_e22335_d_n7;
        locals.var_dnm_dn10 = assign15640_e22335_d_n10;
        locals.var_dnm_dn11 = assign15640_e22335_d_n11;
        locals.var_dnm_dn12 = assign15640_e22335_d_n12;
        locals.var_dnm_dn17 = assign15640_e22335_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign15650_e22350: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard470 = assign15650_e22350;
        locals.var_guard470_rv = 0.0;

        let assign15660_e22353: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign15660_e22353;
        locals.var_guard471_rv = 0.0;

        let (assign15670_e22366,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15670_e22366;
        locals.var_mm_rv = 0.0;

        let assign15680_e22369: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard472 = assign15680_e22369;
        locals.var_guard472_rv = 0.0;

        let (assign15690_e22385,) = {
    if ((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15690_e22385;
        locals.var_mm_rv = 0.0;

        let assign15700_e22388: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign15700_e22388;
        locals.var_guard473_rv = 0.0;

        let (assign15710_e22407,) = {
    if (((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 == 0.0)) && (locals.var_guard473 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15710_e22407;
        locals.var_mm_rv = 0.0;

        let assign15720_e22410: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign15720_e22410;
        locals.var_guard474_rv = 0.0;

        let (assign15730_e22432,) = {
    if ((((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_guard471 == 0.0)) && (locals.var_guard472 == 0.0)) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15730_e22432;
        locals.var_mm_rv = 0.0;

        let (assign15740_e22443,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15740_e22443;
        locals.var_m0_rv = 0.0;

        let mut assign15750_loop_guard: usize = 0;
        while {
            let assign15750_cond_e22455: f64 = if (((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign15750_cond_e22455 != 0.0
        } {
            assign15750_loop_guard += 1;
            assert!(assign15750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15750_body0_e22467, assign15750_body0_e22467_d_n0, assign15750_body0_e22467_d_n2, assign15750_body0_e22467_d_n6, assign15750_body0_e22467_d_n7, assign15750_body0_e22467_d_n10, assign15750_body0_e22467_d_n11, assign15750_body0_e22467_d_n12, assign15750_body0_e22467_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) {
        let assign15750_body0_e22465: f64 = (locals.var_dnm).sqrt();
        (assign15750_body0_e22465, (locals.var_dnm_dn0 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn2 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn6 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn7 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn10 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn11 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn12 / (2.0 * assign15750_body0_e22465)), (locals.var_dnm_dn17 / (2.0 * assign15750_body0_e22465)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign15750_body0_e22467;
            locals.var_dnm_dn0 = assign15750_body0_e22467_d_n0;
            locals.var_dnm_dn2 = assign15750_body0_e22467_d_n2;
            locals.var_dnm_dn6 = assign15750_body0_e22467_d_n6;
            locals.var_dnm_dn7 = assign15750_body0_e22467_d_n7;
            locals.var_dnm_dn10 = assign15750_body0_e22467_d_n10;
            locals.var_dnm_dn11 = assign15750_body0_e22467_d_n11;
            locals.var_dnm_dn12 = assign15750_body0_e22467_d_n12;
            locals.var_dnm_dn17 = assign15750_body0_e22467_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign15750_body1_e22480,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 != 0.0)) {
        let assign15750_body1_e22478: f64 = (locals.var_m0 + 1.0);
        (assign15750_body1_e22478,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign15750_body1_e22480;
            locals.var_m0_rv = 0.0;
        }

        let (assign15760_e22498, assign15760_e22498_d_n0, assign15760_e22498_d_n2, assign15760_e22498_d_n6, assign15760_e22498_d_n7, assign15760_e22498_d_n10, assign15760_e22498_d_n11, assign15760_e22498_d_n12, assign15760_e22498_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) && (locals.var_guard470 == 0.0)) {
        let assign15760_e22494: f64 = (2.0 * 4.0);
        let assign15760_e22495: f64 = (1.0 / assign15760_e22494);
        let assign15760_e22496: f64 = (locals.var_dnm).powf(assign15760_e22495);
        (assign15760_e22496, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn0)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn2)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn6)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn7)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn10)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn11)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn12)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15760_e22495) as f64).is_finite() && ((assign15760_e22495) as f64).fract() == 0.0 { if assign15760_e22495 == 0.0 { 0.0 } else { (assign15760_e22495 * ((locals.var_dnm).powf(assign15760_e22495 - 1.0) * locals.var_dnm_dn17)) } } else { (assign15760_e22496 * (assign15760_e22495 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15760_e22498;
        locals.var_dnm_dn0 = assign15760_e22498_d_n0;
        locals.var_dnm_dn2 = assign15760_e22498_d_n2;
        locals.var_dnm_dn6 = assign15760_e22498_d_n6;
        locals.var_dnm_dn7 = assign15760_e22498_d_n7;
        locals.var_dnm_dn10 = assign15760_e22498_d_n10;
        locals.var_dnm_dn11 = assign15760_e22498_d_n11;
        locals.var_dnm_dn12 = assign15760_e22498_d_n12;
        locals.var_dnm_dn17 = assign15760_e22498_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15770_e22509, assign15770_e22509_d_n0, assign15770_e22509_d_n2, assign15770_e22509_d_n6, assign15770_e22509_d_n7, assign15770_e22509_d_n10, assign15770_e22509_d_n11, assign15770_e22509_d_n12, assign15770_e22509_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15770_e22507: f64 = (1.0 / locals.var_dnm);
        (assign15770_e22507, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15770_e22509;
        locals.var_dnm_dn0 = assign15770_e22509_d_n0;
        locals.var_dnm_dn2 = assign15770_e22509_d_n2;
        locals.var_dnm_dn6 = assign15770_e22509_d_n6;
        locals.var_dnm_dn7 = assign15770_e22509_d_n7;
        locals.var_dnm_dn10 = assign15770_e22509_d_n10;
        locals.var_dnm_dn11 = assign15770_e22509_d_n11;
        locals.var_dnm_dn12 = assign15770_e22509_d_n12;
        locals.var_dnm_dn17 = assign15770_e22509_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15780_e22522, assign15780_e22522_d_n0, assign15780_e22522_d_n2, assign15780_e22522_d_n6, assign15780_e22522_d_n7, assign15780_e22522_d_n10, assign15780_e22522_d_n11, assign15780_e22522_d_n12, assign15780_e22522_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15780_e22518: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign15780_e22520: f64 = (assign15780_e22518 * locals.var_dnm);
        (assign15780_e22520, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn12)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn17)) * locals.var_dnm) + (assign15780_e22518 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign15780_e22522;
        locals.var_tmf0_dn0 = assign15780_e22522_d_n0;
        locals.var_tmf0_dn2 = assign15780_e22522_d_n2;
        locals.var_tmf0_dn6 = assign15780_e22522_d_n6;
        locals.var_tmf0_dn7 = assign15780_e22522_d_n7;
        locals.var_tmf0_dn10 = assign15780_e22522_d_n10;
        locals.var_tmf0_dn11 = assign15780_e22522_d_n11;
        locals.var_tmf0_dn12 = assign15780_e22522_d_n12;
        locals.var_tmf0_dn17 = assign15780_e22522_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign15790_e22535, assign15790_e22535_d_n0, assign15790_e22535_d_n2, assign15790_e22535_d_n6, assign15790_e22535_d_n7, assign15790_e22535_d_n10, assign15790_e22535_d_n11, assign15790_e22535_d_n12, assign15790_e22535_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 != 0.0)) {
        let assign15790_e22531: f64 = locals.var_t5;
        let assign15790_e22533: f64 = (assign15790_e22531 - locals.var_tmf0);
        (assign15790_e22533, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn12 - locals.var_tmf0_dn12), (locals.var_t5_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    }
};
        locals.var_t4__blk465 = assign15790_e22535;
        locals.var_t4__blk465_dn0 = assign15790_e22535_d_n0;
        locals.var_t4__blk465_dn2 = assign15790_e22535_d_n2;
        locals.var_t4__blk465_dn6 = assign15790_e22535_d_n6;
        locals.var_t4__blk465_dn7 = assign15790_e22535_d_n7;
        locals.var_t4__blk465_dn10 = assign15790_e22535_d_n10;
        locals.var_t4__blk465_dn11 = assign15790_e22535_d_n11;
        locals.var_t4__blk465_dn12 = assign15790_e22535_d_n12;
        locals.var_t4__blk465_dn17 = assign15790_e22535_d_n17;
        locals.var_t4__blk465_rv = 0.0;

        let (assign15800_e22545, assign15800_e22545_d_n0, assign15800_e22545_d_n2, assign15800_e22545_d_n6, assign15800_e22545_d_n7, assign15800_e22545_d_n10, assign15800_e22545_d_n11, assign15800_e22545_d_n12, assign15800_e22545_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard469 == 0.0)) {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    } else {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    }
};
        locals.var_t4__blk465 = assign15800_e22545;
        locals.var_t4__blk465_dn0 = assign15800_e22545_d_n0;
        locals.var_t4__blk465_dn2 = assign15800_e22545_d_n2;
        locals.var_t4__blk465_dn6 = assign15800_e22545_d_n6;
        locals.var_t4__blk465_dn7 = assign15800_e22545_d_n7;
        locals.var_t4__blk465_dn10 = assign15800_e22545_d_n10;
        locals.var_t4__blk465_dn11 = assign15800_e22545_d_n11;
        locals.var_t4__blk465_dn12 = assign15800_e22545_d_n12;
        locals.var_t4__blk465_dn17 = assign15800_e22545_d_n17;
        locals.var_t4__blk465_rv = 0.0;

        let (assign15810_e22553, assign15810_e22553_d_n0, assign15810_e22553_d_n2, assign15810_e22553_d_n6, assign15810_e22553_d_n7, assign15810_e22553_d_n10, assign15810_e22553_d_n11, assign15810_e22553_d_n12, assign15810_e22553_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15810_e22551: f64 = (locals.var_t4__blk465).sqrt();
        (assign15810_e22551, (locals.var_t4__blk465_dn0 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn2 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn6 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn7 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn10 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn11 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn12 / (2.0 * assign15810_e22551)), (locals.var_t4__blk465_dn17 / (2.0 * assign15810_e22551)),)
    } else {
        (locals.var_t3__blk464, locals.var_t3__blk464_dn0, locals.var_t3__blk464_dn2, locals.var_t3__blk464_dn6, locals.var_t3__blk464_dn7, locals.var_t3__blk464_dn10, locals.var_t3__blk464_dn11, locals.var_t3__blk464_dn12, locals.var_t3__blk464_dn17,)
    }
};
        locals.var_t3__blk464 = assign15810_e22553;
        locals.var_t3__blk464_dn0 = assign15810_e22553_d_n0;
        locals.var_t3__blk464_dn2 = assign15810_e22553_d_n2;
        locals.var_t3__blk464_dn6 = assign15810_e22553_d_n6;
        locals.var_t3__blk464_dn7 = assign15810_e22553_d_n7;
        locals.var_t3__blk464_dn10 = assign15810_e22553_d_n10;
        locals.var_t3__blk464_dn11 = assign15810_e22553_d_n11;
        locals.var_t3__blk464_dn12 = assign15810_e22553_d_n12;
        locals.var_t3__blk464_dn17 = assign15810_e22553_d_n17;
        locals.var_t3__blk464_rv = 0.0;

        let (assign15820_e22566, assign15820_e22566_d_n0, assign15820_e22566_d_n2, assign15820_e22566_d_n6, assign15820_e22566_d_n7, assign15820_e22566_d_n10, assign15820_e22566_d_n11, assign15820_e22566_d_n12, assign15820_e22566_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15820_e22562: f64 = (1.0 - locals.var_t3__blk464);
        let assign15820_e22563: f64 = (locals.var_t2__blk463 * assign15820_e22562);
        let assign15820_e22564: f64 = (locals.var_vgp + assign15820_e22563);
        (assign15820_e22564, (locals.var_vgp_dn0 + ((locals.var_t2__blk463_dn0 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2__blk463_dn2 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn2)))), (locals.var_vgp_dn6 + ((locals.var_t2__blk463_dn6 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2__blk463_dn7 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn7)))), (locals.var_vgp_dn10 + ((locals.var_t2__blk463_dn10 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2__blk463_dn11 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn11)))), (locals.var_vgp_dn12 + ((locals.var_t2__blk463_dn12 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn12)))), (locals.var_vgp_dn17 + ((locals.var_t2__blk463_dn17 * assign15820_e22562) + (locals.var_t2__blk463 * (-locals.var_t3__blk464_dn17)))),)
    } else {
        (locals.var_t10__blk468, locals.var_t10__blk468_dn0, locals.var_t10__blk468_dn2, locals.var_t10__blk468_dn6, locals.var_t10__blk468_dn7, locals.var_t10__blk468_dn10, locals.var_t10__blk468_dn11, locals.var_t10__blk468_dn12, locals.var_t10__blk468_dn17,)
    }
};
        locals.var_t10__blk468 = assign15820_e22566;
        locals.var_t10__blk468_dn0 = assign15820_e22566_d_n0;
        locals.var_t10__blk468_dn2 = assign15820_e22566_d_n2;
        locals.var_t10__blk468_dn6 = assign15820_e22566_d_n6;
        locals.var_t10__blk468_dn7 = assign15820_e22566_d_n7;
        locals.var_t10__blk468_dn10 = assign15820_e22566_d_n10;
        locals.var_t10__blk468_dn11 = assign15820_e22566_d_n11;
        locals.var_t10__blk468_dn12 = assign15820_e22566_d_n12;
        locals.var_t10__blk468_dn17 = assign15820_e22566_d_n17;
        locals.var_t10__blk468_rv = 0.0;

        let (assign15830_e22582, assign15830_e22582_d_n0, assign15830_e22582_d_n2, assign15830_e22582_d_n6, assign15830_e22582_d_n7, assign15830_e22582_d_n10, assign15830_e22582_d_n11, assign15830_e22582_d_n12, assign15830_e22582_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15830_e22573: f64 = (locals.var_t10__blk468 * locals.var_t10__blk468);
        let assign15830_e22576: f64 = (4.0 * 0.01);
        let assign15830_e22578: f64 = (assign15830_e22576 * 0.01);
        let assign15830_e22579: f64 = (assign15830_e22573 + assign15830_e22578);
        let assign15830_e22580: f64 = (assign15830_e22579).sqrt();
        (assign15830_e22580, (((locals.var_t10__blk468_dn0 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn0)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn2 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn2)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn6 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn6)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn7 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn7)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn10 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn10)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn11 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn11)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn12 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn12)) / (2.0 * assign15830_e22580)), (((locals.var_t10__blk468_dn17 * locals.var_t10__blk468) + (locals.var_t10__blk468 * locals.var_t10__blk468_dn17)) / (2.0 * assign15830_e22580)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign15830_e22582;
        locals.var_tmf1_dn0 = assign15830_e22582_d_n0;
        locals.var_tmf1_dn2 = assign15830_e22582_d_n2;
        locals.var_tmf1_dn6 = assign15830_e22582_d_n6;
        locals.var_tmf1_dn7 = assign15830_e22582_d_n7;
        locals.var_tmf1_dn10 = assign15830_e22582_d_n10;
        locals.var_tmf1_dn11 = assign15830_e22582_d_n11;
        locals.var_tmf1_dn12 = assign15830_e22582_d_n12;
        locals.var_tmf1_dn17 = assign15830_e22582_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign15840_e22597, assign15840_e22597_d_n0, assign15840_e22597_d_n2, assign15840_e22597_d_n6, assign15840_e22597_d_n7, assign15840_e22597_d_n10, assign15840_e22597_d_n11, assign15840_e22597_d_n12, assign15840_e22597_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15840_e22590: f64 = (locals.var_t10__blk468 + locals.var_tmf1);
        let assign15840_e22591: f64 = (0.5 * assign15840_e22590);
        let assign15840_e22594: f64 = (1e-10 * 0.01);
        let assign15840_e22595: f64 = (assign15840_e22591 + assign15840_e22594);
        (assign15840_e22595, (0.5 * (locals.var_t10__blk468_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t10__blk468_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t10__blk468_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t10__blk468_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t10__blk468_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t10__blk468_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t10__blk468_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t10__blk468_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t10__blk468, locals.var_t10__blk468_dn0, locals.var_t10__blk468_dn2, locals.var_t10__blk468_dn6, locals.var_t10__blk468_dn7, locals.var_t10__blk468_dn10, locals.var_t10__blk468_dn11, locals.var_t10__blk468_dn12, locals.var_t10__blk468_dn17,)
    }
};
        locals.var_t10__blk468 = assign15840_e22597;
        locals.var_t10__blk468_dn0 = assign15840_e22597_d_n0;
        locals.var_t10__blk468_dn2 = assign15840_e22597_d_n2;
        locals.var_t10__blk468_dn6 = assign15840_e22597_d_n6;
        locals.var_t10__blk468_dn7 = assign15840_e22597_d_n7;
        locals.var_t10__blk468_dn10 = assign15840_e22597_d_n10;
        locals.var_t10__blk468_dn11 = assign15840_e22597_d_n11;
        locals.var_t10__blk468_dn12 = assign15840_e22597_d_n12;
        locals.var_t10__blk468_dn17 = assign15840_e22597_d_n17;
        locals.var_t10__blk468_rv = 0.0;

        let assign15850_e22600: f64 = if locals.var_t10__blk468 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign15850_e22600;
        locals.var_guard475_rv = 0.0;

        let (assign15860_e22609, assign15860_e22609_d_n0, assign15860_e22609_d_n2, assign15860_e22609_d_n6, assign15860_e22609_d_n7, assign15860_e22609_d_n10, assign15860_e22609_d_n11, assign15860_e22609_d_n12, assign15860_e22609_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard475 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10__blk468, locals.var_t10__blk468_dn0, locals.var_t10__blk468_dn2, locals.var_t10__blk468_dn6, locals.var_t10__blk468_dn7, locals.var_t10__blk468_dn10, locals.var_t10__blk468_dn11, locals.var_t10__blk468_dn12, locals.var_t10__blk468_dn17,)
    }
};
        locals.var_t10__blk468 = assign15860_e22609;
        locals.var_t10__blk468_dn0 = assign15860_e22609_d_n0;
        locals.var_t10__blk468_dn2 = assign15860_e22609_d_n2;
        locals.var_t10__blk468_dn6 = assign15860_e22609_d_n6;
        locals.var_t10__blk468_dn7 = assign15860_e22609_d_n7;
        locals.var_t10__blk468_dn10 = assign15860_e22609_d_n10;
        locals.var_t10__blk468_dn11 = assign15860_e22609_d_n11;
        locals.var_t10__blk468_dn12 = assign15860_e22609_d_n12;
        locals.var_t10__blk468_dn17 = assign15860_e22609_d_n17;
        locals.var_t10__blk468_rv = 0.0;

        let (assign15880_e22625, assign15880_e22625_d_n0, assign15880_e22625_d_n2, assign15880_e22625_d_n6, assign15880_e22625_d_n7, assign15880_e22625_d_n10, assign15880_e22625_d_n11, assign15880_e22625_d_n12, assign15880_e22625_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15880_e22623: f64 = (locals.var_vds / locals.var_t10__blk468);
        (assign15880_e22623, (((locals.var_vds_dn0 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn0)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn2 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn2)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn6 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn6)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn7 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn7)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn10 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn10)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn11 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn11)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn12 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn12)) / (locals.var_t10__blk468 * locals.var_t10__blk468)), (((locals.var_vds_dn17 * locals.var_t10__blk468) - (locals.var_vds * locals.var_t10__blk468_dn17)) / (locals.var_t10__blk468 * locals.var_t10__blk468)),)
    } else {
        (locals.var_t1__blk462, locals.var_t1__blk462_dn0, locals.var_t1__blk462_dn2, locals.var_t1__blk462_dn6, locals.var_t1__blk462_dn7, locals.var_t1__blk462_dn10, locals.var_t1__blk462_dn11, locals.var_t1__blk462_dn12, locals.var_t1__blk462_dn17,)
    }
};
        locals.var_t1__blk462 = assign15880_e22625;
        locals.var_t1__blk462_dn0 = assign15880_e22625_d_n0;
        locals.var_t1__blk462_dn2 = assign15880_e22625_d_n2;
        locals.var_t1__blk462_dn6 = assign15880_e22625_d_n6;
        locals.var_t1__blk462_dn7 = assign15880_e22625_d_n7;
        locals.var_t1__blk462_dn10 = assign15880_e22625_d_n10;
        locals.var_t1__blk462_dn11 = assign15880_e22625_d_n11;
        locals.var_t1__blk462_dn12 = assign15880_e22625_d_n12;
        locals.var_t1__blk462_dn17 = assign15880_e22625_d_n17;
        locals.var_t1__blk462_rv = 0.0;

        let (assign15890_e22636, assign15890_e22636_d_n0, assign15890_e22636_d_n2, assign15890_e22636_d_n6, assign15890_e22636_d_n7, assign15890_e22636_d_n10, assign15890_e22636_d_n11, assign15890_e22636_d_n12, assign15890_e22636_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15890_e22633: f64 = (locals.var_ddlte - 1.0);
        let assign15890_e22634: f64 = (locals.var_t1__blk462).powf(assign15890_e22633);
        (assign15890_e22634, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn0)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn0 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn2)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn2 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn6)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn6 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn7)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn7 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn10)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn10 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn11)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn11 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn12)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn12 / locals.var_t1__blk462))) }, if 0.0 == 0.0 && ((assign15890_e22633) as f64).is_finite() && ((assign15890_e22633) as f64).fract() == 0.0 { if assign15890_e22633 == 0.0 { 0.0 } else { (assign15890_e22633 * ((locals.var_t1__blk462).powf(assign15890_e22633 - 1.0) * locals.var_t1__blk462_dn17)) } } else { (assign15890_e22634 * (assign15890_e22633 * (locals.var_t1__blk462_dn17 / locals.var_t1__blk462))) },)
    } else {
        (locals.var_t2__blk463, locals.var_t2__blk463_dn0, locals.var_t2__blk463_dn2, locals.var_t2__blk463_dn6, locals.var_t2__blk463_dn7, locals.var_t2__blk463_dn10, locals.var_t2__blk463_dn11, locals.var_t2__blk463_dn12, locals.var_t2__blk463_dn17,)
    }
};
        locals.var_t2__blk463 = assign15890_e22636;
        locals.var_t2__blk463_dn0 = assign15890_e22636_d_n0;
        locals.var_t2__blk463_dn2 = assign15890_e22636_d_n2;
        locals.var_t2__blk463_dn6 = assign15890_e22636_d_n6;
        locals.var_t2__blk463_dn7 = assign15890_e22636_d_n7;
        locals.var_t2__blk463_dn10 = assign15890_e22636_d_n10;
        locals.var_t2__blk463_dn11 = assign15890_e22636_d_n11;
        locals.var_t2__blk463_dn12 = assign15890_e22636_d_n12;
        locals.var_t2__blk463_dn17 = assign15890_e22636_d_n17;
        locals.var_t2__blk463_rv = 0.0;

        let (assign15900_e22645, assign15900_e22645_d_n0, assign15900_e22645_d_n2, assign15900_e22645_d_n6, assign15900_e22645_d_n7, assign15900_e22645_d_n10, assign15900_e22645_d_n11, assign15900_e22645_d_n12, assign15900_e22645_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15900_e22643: f64 = (locals.var_t2__blk463 * locals.var_t1__blk462);
        (assign15900_e22643, ((locals.var_t2__blk463_dn0 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn0)), ((locals.var_t2__blk463_dn2 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn2)), ((locals.var_t2__blk463_dn6 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn6)), ((locals.var_t2__blk463_dn7 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn7)), ((locals.var_t2__blk463_dn10 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn10)), ((locals.var_t2__blk463_dn11 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn11)), ((locals.var_t2__blk463_dn12 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn12)), ((locals.var_t2__blk463_dn17 * locals.var_t1__blk462) + (locals.var_t2__blk463 * locals.var_t1__blk462_dn17)),)
    } else {
        (locals.var_t7__blk467, locals.var_t7__blk467_dn0, locals.var_t7__blk467_dn2, locals.var_t7__blk467_dn6, locals.var_t7__blk467_dn7, locals.var_t7__blk467_dn10, locals.var_t7__blk467_dn11, locals.var_t7__blk467_dn12, locals.var_t7__blk467_dn17,)
    }
};
        locals.var_t7__blk467 = assign15900_e22645;
        locals.var_t7__blk467_dn0 = assign15900_e22645_d_n0;
        locals.var_t7__blk467_dn2 = assign15900_e22645_d_n2;
        locals.var_t7__blk467_dn6 = assign15900_e22645_d_n6;
        locals.var_t7__blk467_dn7 = assign15900_e22645_d_n7;
        locals.var_t7__blk467_dn10 = assign15900_e22645_d_n10;
        locals.var_t7__blk467_dn11 = assign15900_e22645_d_n11;
        locals.var_t7__blk467_dn12 = assign15900_e22645_d_n12;
        locals.var_t7__blk467_dn17 = assign15900_e22645_d_n17;
        locals.var_t7__blk467_rv = 0.0;

        let (assign15910_e22654, assign15910_e22654_d_n0, assign15910_e22654_d_n2, assign15910_e22654_d_n6, assign15910_e22654_d_n7, assign15910_e22654_d_n10, assign15910_e22654_d_n11, assign15910_e22654_d_n12, assign15910_e22654_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15910_e22652: f64 = (1.0 + locals.var_t7__blk467);
        (assign15910_e22652, locals.var_t7__blk467_dn0, locals.var_t7__blk467_dn2, locals.var_t7__blk467_dn6, locals.var_t7__blk467_dn7, locals.var_t7__blk467_dn10, locals.var_t7__blk467_dn11, locals.var_t7__blk467_dn12, locals.var_t7__blk467_dn17,)
    } else {
        (locals.var_t3__blk464, locals.var_t3__blk464_dn0, locals.var_t3__blk464_dn2, locals.var_t3__blk464_dn6, locals.var_t3__blk464_dn7, locals.var_t3__blk464_dn10, locals.var_t3__blk464_dn11, locals.var_t3__blk464_dn12, locals.var_t3__blk464_dn17,)
    }
};
        locals.var_t3__blk464 = assign15910_e22654;
        locals.var_t3__blk464_dn0 = assign15910_e22654_d_n0;
        locals.var_t3__blk464_dn2 = assign15910_e22654_d_n2;
        locals.var_t3__blk464_dn6 = assign15910_e22654_d_n6;
        locals.var_t3__blk464_dn7 = assign15910_e22654_d_n7;
        locals.var_t3__blk464_dn10 = assign15910_e22654_d_n10;
        locals.var_t3__blk464_dn11 = assign15910_e22654_d_n11;
        locals.var_t3__blk464_dn12 = assign15910_e22654_d_n12;
        locals.var_t3__blk464_dn17 = assign15910_e22654_d_n17;
        locals.var_t3__blk464_rv = 0.0;

        let (assign15920_e22667, assign15920_e22667_d_n0, assign15920_e22667_d_n2, assign15920_e22667_d_n6, assign15920_e22667_d_n7, assign15920_e22667_d_n10, assign15920_e22667_d_n11, assign15920_e22667_d_n12, assign15920_e22667_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15920_e22662: f64 = (1.0 / locals.var_ddlte);
        let assign15920_e22664: f64 = (assign15920_e22662 - 1.0);
        let assign15920_e22665: f64 = (locals.var_t3__blk464).powf(assign15920_e22664);
        (assign15920_e22665, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn0)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn0 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn2)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn2 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn6)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn6 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn7)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn7 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn10)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn10 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn11)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn11 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn12)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn12 / locals.var_t3__blk464))) }, if 0.0 == 0.0 && ((assign15920_e22664) as f64).is_finite() && ((assign15920_e22664) as f64).fract() == 0.0 { if assign15920_e22664 == 0.0 { 0.0 } else { (assign15920_e22664 * ((locals.var_t3__blk464).powf(assign15920_e22664 - 1.0) * locals.var_t3__blk464_dn17)) } } else { (assign15920_e22665 * (assign15920_e22664 * (locals.var_t3__blk464_dn17 / locals.var_t3__blk464))) },)
    } else {
        (locals.var_t4__blk465, locals.var_t4__blk465_dn0, locals.var_t4__blk465_dn2, locals.var_t4__blk465_dn6, locals.var_t4__blk465_dn7, locals.var_t4__blk465_dn10, locals.var_t4__blk465_dn11, locals.var_t4__blk465_dn12, locals.var_t4__blk465_dn17,)
    }
};
        locals.var_t4__blk465 = assign15920_e22667;
        locals.var_t4__blk465_dn0 = assign15920_e22667_d_n0;
        locals.var_t4__blk465_dn2 = assign15920_e22667_d_n2;
        locals.var_t4__blk465_dn6 = assign15920_e22667_d_n6;
        locals.var_t4__blk465_dn7 = assign15920_e22667_d_n7;
        locals.var_t4__blk465_dn10 = assign15920_e22667_d_n10;
        locals.var_t4__blk465_dn11 = assign15920_e22667_d_n11;
        locals.var_t4__blk465_dn12 = assign15920_e22667_d_n12;
        locals.var_t4__blk465_dn17 = assign15920_e22667_d_n17;
        locals.var_t4__blk465_rv = 0.0;

        let (assign15930_e22676, assign15930_e22676_d_n0, assign15930_e22676_d_n2, assign15930_e22676_d_n6, assign15930_e22676_d_n7, assign15930_e22676_d_n10, assign15930_e22676_d_n11, assign15930_e22676_d_n12, assign15930_e22676_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15930_e22674: f64 = (locals.var_t4__blk465 * locals.var_t3__blk464);
        (assign15930_e22674, ((locals.var_t4__blk465_dn0 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn0)), ((locals.var_t4__blk465_dn2 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn2)), ((locals.var_t4__blk465_dn6 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn6)), ((locals.var_t4__blk465_dn7 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn7)), ((locals.var_t4__blk465_dn10 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn10)), ((locals.var_t4__blk465_dn11 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn11)), ((locals.var_t4__blk465_dn12 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn12)), ((locals.var_t4__blk465_dn17 * locals.var_t3__blk464) + (locals.var_t4__blk465 * locals.var_t3__blk464_dn17)),)
    } else {
        (locals.var_t6__blk466, locals.var_t6__blk466_dn0, locals.var_t6__blk466_dn2, locals.var_t6__blk466_dn6, locals.var_t6__blk466_dn7, locals.var_t6__blk466_dn10, locals.var_t6__blk466_dn11, locals.var_t6__blk466_dn12, locals.var_t6__blk466_dn17,)
    }
};
        locals.var_t6__blk466 = assign15930_e22676;
        locals.var_t6__blk466_dn0 = assign15930_e22676_d_n0;
        locals.var_t6__blk466_dn2 = assign15930_e22676_d_n2;
        locals.var_t6__blk466_dn6 = assign15930_e22676_d_n6;
        locals.var_t6__blk466_dn7 = assign15930_e22676_d_n7;
        locals.var_t6__blk466_dn10 = assign15930_e22676_d_n10;
        locals.var_t6__blk466_dn11 = assign15930_e22676_d_n11;
        locals.var_t6__blk466_dn12 = assign15930_e22676_d_n12;
        locals.var_t6__blk466_dn17 = assign15930_e22676_d_n17;
        locals.var_t6__blk466_rv = 0.0;

        let (assign15940_e22685, assign15940_e22685_d_n0, assign15940_e22685_d_n2, assign15940_e22685_d_n6, assign15940_e22685_d_n7, assign15940_e22685_d_n10, assign15940_e22685_d_n11, assign15940_e22685_d_n12, assign15940_e22685_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15940_e22683: f64 = (locals.var_vds / locals.var_t6__blk466);
        (assign15940_e22683, (((locals.var_vds_dn0 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn0)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn2 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn2)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn6 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn6)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn7 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn7)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn10 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn10)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn11 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn11)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn12 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn12)) / (locals.var_t6__blk466 * locals.var_t6__blk466)), (((locals.var_vds_dn17 * locals.var_t6__blk466) - (locals.var_vds * locals.var_t6__blk466_dn17)) / (locals.var_t6__blk466 * locals.var_t6__blk466)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    }
};
        locals.var_vdseff = assign15940_e22685;
        locals.var_vdseff_dn0 = assign15940_e22685_d_n0;
        locals.var_vdseff_dn2 = assign15940_e22685_d_n2;
        locals.var_vdseff_dn6 = assign15940_e22685_d_n6;
        locals.var_vdseff_dn7 = assign15940_e22685_d_n7;
        locals.var_vdseff_dn10 = assign15940_e22685_d_n10;
        locals.var_vdseff_dn11 = assign15940_e22685_d_n11;
        locals.var_vdseff_dn12 = assign15940_e22685_d_n12;
        locals.var_vdseff_dn17 = assign15940_e22685_d_n17;
        locals.var_vdseff_rv = 0.0;

        let (assign15950_e22692, assign15950_e22692_d_n0, assign15950_e22692_d_n2, assign15950_e22692_d_n6, assign15950_e22692_d_n7, assign15950_e22692_d_n10, assign15950_e22692_d_n11, assign15950_e22692_d_n12, assign15950_e22692_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign15950_e22692;
        locals.var_vds_dn0 = assign15950_e22692_d_n0;
        locals.var_vds_dn2 = assign15950_e22692_d_n2;
        locals.var_vds_dn6 = assign15950_e22692_d_n6;
        locals.var_vds_dn7 = assign15950_e22692_d_n7;
        locals.var_vds_dn10 = assign15950_e22692_d_n10;
        locals.var_vds_dn11 = assign15950_e22692_d_n11;
        locals.var_vds_dn12 = assign15950_e22692_d_n12;
        locals.var_vds_dn17 = assign15950_e22692_d_n17;
        locals.var_vds_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
        let (assign15960_e22704, assign15960_e22704_d_n0, assign15960_e22704_d_n2, assign15960_e22704_d_n6, assign15960_e22704_d_n7, assign15960_e22704_d_n10, assign15960_e22704_d_n11, assign15960_e22704_d_n12, assign15960_e22704_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15960_e22700: f64 = (locals.var_vbcs_cl - locals.var_vds);
        let assign15960_e22701: f64 = (locals.var_beta * assign15960_e22700);
        let assign15960_e22702: f64 = (assign15960_e22701).exp();
        (assign15960_e22702, (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn0 - locals.var_vds_dn0))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn2 - locals.var_vds_dn2))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn6 - locals.var_vds_dn6))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn7 - locals.var_vds_dn7))), (assign15960_e22702 * ((locals.var_beta_dn10 * assign15960_e22700) + (locals.var_beta * (locals.var_vbcs_cl_dn10 - locals.var_vds_dn10)))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn11 - locals.var_vds_dn11))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn12 - locals.var_vds_dn12))), (assign15960_e22702 * (locals.var_beta * (locals.var_vbcs_cl_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn12, locals.var_exp_bvbsvds_dn17,)
    }
};
        locals.var_exp_bvbsvds = assign15960_e22704;
        locals.var_exp_bvbsvds_dn0 = assign15960_e22704_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign15960_e22704_d_n2;
        locals.var_exp_bvbsvds_dn6 = assign15960_e22704_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign15960_e22704_d_n7;
        locals.var_exp_bvbsvds_dn10 = assign15960_e22704_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign15960_e22704_d_n11;
        locals.var_exp_bvbsvds_dn12 = assign15960_e22704_d_n12;
        locals.var_exp_bvbsvds_dn17 = assign15960_e22704_d_n17;
        locals.var_exp_bvbsvds_rv = 0.0;

        let assign15970_e22707: f64 = if locals.var_vds <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign15970_e22707;
        locals.var_guard476_rv = 0.0;

        let (assign15980_e22716, assign15980_e22716_d_n0, assign15980_e22716_d_n2, assign15980_e22716_d_n6, assign15980_e22716_d_n7, assign15980_e22716_d_n10, assign15980_e22716_d_n11, assign15980_e22716_d_n12, assign15980_e22716_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign15980_e22716;
        locals.var_pds_dn0 = assign15980_e22716_d_n0;
        locals.var_pds_dn2 = assign15980_e22716_d_n2;
        locals.var_pds_dn6 = assign15980_e22716_d_n6;
        locals.var_pds_dn7 = assign15980_e22716_d_n7;
        locals.var_pds_dn10 = assign15980_e22716_d_n10;
        locals.var_pds_dn11 = assign15980_e22716_d_n11;
        locals.var_pds_dn12 = assign15980_e22716_d_n12;
        locals.var_pds_dn17 = assign15980_e22716_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign15990_e22725, assign15990_e22725_d_n0, assign15990_e22725_d_n2, assign15990_e22725_d_n6, assign15990_e22725_d_n7, assign15990_e22725_d_n10, assign15990_e22725_d_n11, assign15990_e22725_d_n12, assign15990_e22725_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign15990_e22725;
        locals.var_psl_dn0 = assign15990_e22725_d_n0;
        locals.var_psl_dn2 = assign15990_e22725_d_n2;
        locals.var_psl_dn6 = assign15990_e22725_d_n6;
        locals.var_psl_dn7 = assign15990_e22725_d_n7;
        locals.var_psl_dn10 = assign15990_e22725_d_n10;
        locals.var_psl_dn11 = assign15990_e22725_d_n11;
        locals.var_psl_dn12 = assign15990_e22725_d_n12;
        locals.var_psl_dn17 = assign15990_e22725_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16000_e22734,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign16000_e22734;
        locals.var_flg_conv_rv = 0.0;

        let assign16010_e22737: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign16010_e22737;
        locals.var_guard477_rv = 0.0;

        let (assign16020_e22749, assign16020_e22749_d_n0, assign16020_e22749_d_n2, assign16020_e22749_d_n6, assign16020_e22749_d_n7, assign16020_e22749_d_n10, assign16020_e22749_d_n11, assign16020_e22749_d_n12, assign16020_e22749_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard477 != 0.0)) {
        (locals.var_pssl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign16020_e22749;
        locals.var_phi_sl_soi_dn0 = assign16020_e22749_d_n0;
        locals.var_phi_sl_soi_dn2 = assign16020_e22749_d_n2;
        locals.var_phi_sl_soi_dn6 = assign16020_e22749_d_n6;
        locals.var_phi_sl_soi_dn7 = assign16020_e22749_d_n7;
        locals.var_phi_sl_soi_dn10 = assign16020_e22749_d_n10;
        locals.var_phi_sl_soi_dn11 = assign16020_e22749_d_n11;
        locals.var_phi_sl_soi_dn12 = assign16020_e22749_d_n12;
        locals.var_phi_sl_soi_dn17 = assign16020_e22749_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign16030_e22763, assign16030_e22763_d_n0, assign16030_e22763_d_n2, assign16030_e22763_d_n6, assign16030_e22763_d_n7, assign16030_e22763_d_n10, assign16030_e22763_d_n11, assign16030_e22763_d_n12, assign16030_e22763_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard477 != 0.0)) {
        let assign16030_e22761: f64 = (locals.var_pssl_ini - locals.var_ps0);
        (assign16030_e22761, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn12), (-locals.var_ps0_dn17),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16030_e22763;
        locals.var_pds_ini_dn0 = assign16030_e22763_d_n0;
        locals.var_pds_ini_dn2 = assign16030_e22763_d_n2;
        locals.var_pds_ini_dn6 = assign16030_e22763_d_n6;
        locals.var_pds_ini_dn7 = assign16030_e22763_d_n7;
        locals.var_pds_ini_dn10 = assign16030_e22763_d_n10;
        locals.var_pds_ini_dn11 = assign16030_e22763_d_n11;
        locals.var_pds_ini_dn12 = assign16030_e22763_d_n12;
        locals.var_pds_ini_dn17 = assign16030_e22763_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16040_e22766: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign16040_e22766;
        locals.var_guard478_rv = 0.0;

        let (assign16050_e22787, assign16050_e22787_d_n0, assign16050_e22787_d_n2, assign16050_e22787_d_n6, assign16050_e22787_d_n7, assign16050_e22787_d_n10, assign16050_e22787_d_n11, assign16050_e22787_d_n12, assign16050_e22787_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16050_e22778: f64 = (locals.var_psl_lim - locals.var_ps0);
        let (assign16050_e22785, assign16050_e22785_d_n0, assign16050_e22785_d_n2, assign16050_e22785_d_n6, assign16050_e22785_d_n7, assign16050_e22785_d_n10, assign16050_e22785_d_n11, assign16050_e22785_d_n12, assign16050_e22785_d_n17,) = {
            if (assign16050_e22778 >= 0.0) {
                let assign16050_e22783: f64 = (locals.var_psl_lim - locals.var_ps0);
                (assign16050_e22783, (locals.var_psl_lim_dn0 - locals.var_ps0_dn0), (locals.var_psl_lim_dn2 - locals.var_ps0_dn2), (locals.var_psl_lim_dn6 - locals.var_ps0_dn6), (locals.var_psl_lim_dn7 - locals.var_ps0_dn7), (locals.var_psl_lim_dn10 - locals.var_ps0_dn10), (locals.var_psl_lim_dn11 - locals.var_ps0_dn11), (locals.var_psl_lim_dn12 - locals.var_ps0_dn12), (locals.var_psl_lim_dn17 - locals.var_ps0_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign16050_e22785, assign16050_e22785_d_n0, assign16050_e22785_d_n2, assign16050_e22785_d_n6, assign16050_e22785_d_n7, assign16050_e22785_d_n10, assign16050_e22785_d_n11, assign16050_e22785_d_n12, assign16050_e22785_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign16050_e22787;
        locals.var_pds_max_dn0 = assign16050_e22787_d_n0;
        locals.var_pds_max_dn2 = assign16050_e22787_d_n2;
        locals.var_pds_max_dn6 = assign16050_e22787_d_n6;
        locals.var_pds_max_dn7 = assign16050_e22787_d_n7;
        locals.var_pds_max_dn10 = assign16050_e22787_d_n10;
        locals.var_pds_max_dn11 = assign16050_e22787_d_n11;
        locals.var_pds_max_dn12 = assign16050_e22787_d_n12;
        locals.var_pds_max_dn17 = assign16050_e22787_d_n17;
        locals.var_pds_max_rv = 0.0;

        let (assign16060_e22807, assign16060_e22807_d_n0, assign16060_e22807_d_n2, assign16060_e22807_d_n6, assign16060_e22807_d_n7, assign16060_e22807_d_n10, assign16060_e22807_d_n11, assign16060_e22807_d_n12, assign16060_e22807_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16060_e22799: f64 = (1.0 + 0.3);
        let assign16060_e22801: f64 = (assign16060_e22799 * locals.var_pds_max);
        let assign16060_e22803: f64 = (assign16060_e22801 - locals.var_vds);
        let assign16060_e22805: f64 = (assign16060_e22803 - 0.03);
        (assign16060_e22805, ((assign16060_e22799 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign16060_e22799 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign16060_e22799 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign16060_e22799 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign16060_e22799 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign16060_e22799 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign16060_e22799 * locals.var_pds_max_dn12) - locals.var_vds_dn12), ((assign16060_e22799 * locals.var_pds_max_dn17) - locals.var_vds_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign16060_e22807;
        locals.var_tmf1_dn0 = assign16060_e22807_d_n0;
        locals.var_tmf1_dn2 = assign16060_e22807_d_n2;
        locals.var_tmf1_dn6 = assign16060_e22807_d_n6;
        locals.var_tmf1_dn7 = assign16060_e22807_d_n7;
        locals.var_tmf1_dn10 = assign16060_e22807_d_n10;
        locals.var_tmf1_dn11 = assign16060_e22807_d_n11;
        locals.var_tmf1_dn12 = assign16060_e22807_d_n12;
        locals.var_tmf1_dn17 = assign16060_e22807_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign16070_e22827, assign16070_e22827_d_n0, assign16070_e22827_d_n2, assign16070_e22827_d_n6, assign16070_e22827_d_n7, assign16070_e22827_d_n10, assign16070_e22827_d_n11, assign16070_e22827_d_n12, assign16070_e22827_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16070_e22820: f64 = (1.0 + 0.3);
        let assign16070_e22822: f64 = (assign16070_e22820 * locals.var_pds_max);
        let assign16070_e22823: f64 = (4.0 * assign16070_e22822);
        let assign16070_e22825: f64 = (assign16070_e22823 * 0.03);
        (assign16070_e22825, ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign16070_e22820 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16070_e22827;
        locals.var_tmf2_dn0 = assign16070_e22827_d_n0;
        locals.var_tmf2_dn2 = assign16070_e22827_d_n2;
        locals.var_tmf2_dn6 = assign16070_e22827_d_n6;
        locals.var_tmf2_dn7 = assign16070_e22827_d_n7;
        locals.var_tmf2_dn10 = assign16070_e22827_d_n10;
        locals.var_tmf2_dn11 = assign16070_e22827_d_n11;
        locals.var_tmf2_dn12 = assign16070_e22827_d_n12;
        locals.var_tmf2_dn17 = assign16070_e22827_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16080_e22845, assign16080_e22845_d_n0, assign16080_e22845_d_n2, assign16080_e22845_d_n6, assign16080_e22845_d_n7, assign16080_e22845_d_n10, assign16080_e22845_d_n11, assign16080_e22845_d_n12, assign16080_e22845_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let (assign16080_e22843, assign16080_e22843_d_n0, assign16080_e22843_d_n2, assign16080_e22843_d_n6, assign16080_e22843_d_n7, assign16080_e22843_d_n10, assign16080_e22843_d_n11, assign16080_e22843_d_n12, assign16080_e22843_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign16080_e22842: f64 = (-locals.var_tmf2);
                (assign16080_e22842, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign16080_e22843, assign16080_e22843_d_n0, assign16080_e22843_d_n2, assign16080_e22843_d_n6, assign16080_e22843_d_n7, assign16080_e22843_d_n10, assign16080_e22843_d_n11, assign16080_e22843_d_n12, assign16080_e22843_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16080_e22845;
        locals.var_tmf2_dn0 = assign16080_e22845_d_n0;
        locals.var_tmf2_dn2 = assign16080_e22845_d_n2;
        locals.var_tmf2_dn6 = assign16080_e22845_d_n6;
        locals.var_tmf2_dn7 = assign16080_e22845_d_n7;
        locals.var_tmf2_dn10 = assign16080_e22845_d_n10;
        locals.var_tmf2_dn11 = assign16080_e22845_d_n11;
        locals.var_tmf2_dn12 = assign16080_e22845_d_n12;
        locals.var_tmf2_dn17 = assign16080_e22845_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16090_e22862, assign16090_e22862_d_n0, assign16090_e22862_d_n2, assign16090_e22862_d_n6, assign16090_e22862_d_n7, assign16090_e22862_d_n10, assign16090_e22862_d_n11, assign16090_e22862_d_n12, assign16090_e22862_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16090_e22857: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign16090_e22859: f64 = (assign16090_e22857 + locals.var_tmf2);
        let assign16090_e22860: f64 = (assign16090_e22859).sqrt();
        (assign16090_e22860, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign16090_e22860)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign16090_e22860)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16090_e22862;
        locals.var_tmf2_dn0 = assign16090_e22862_d_n0;
        locals.var_tmf2_dn2 = assign16090_e22862_d_n2;
        locals.var_tmf2_dn6 = assign16090_e22862_d_n6;
        locals.var_tmf2_dn7 = assign16090_e22862_d_n7;
        locals.var_tmf2_dn10 = assign16090_e22862_d_n10;
        locals.var_tmf2_dn11 = assign16090_e22862_d_n11;
        locals.var_tmf2_dn12 = assign16090_e22862_d_n12;
        locals.var_tmf2_dn17 = assign16090_e22862_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16100_e22884, assign16100_e22884_d_n0, assign16100_e22884_d_n2, assign16100_e22884_d_n6, assign16100_e22884_d_n7, assign16100_e22884_d_n10, assign16100_e22884_d_n11, assign16100_e22884_d_n12, assign16100_e22884_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign16100_e22874: f64 = (1.0 + 0.3);
        let assign16100_e22876: f64 = (assign16100_e22874 * locals.var_pds_max);
        let assign16100_e22880: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign16100_e22881: f64 = (0.5 * assign16100_e22880);
        let assign16100_e22882: f64 = (assign16100_e22876 - assign16100_e22881);
        (assign16100_e22882, ((assign16100_e22874 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign16100_e22874 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign16100_e22874 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign16100_e22874 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign16100_e22874 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign16100_e22874 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign16100_e22874 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign16100_e22874 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16100_e22884;
        locals.var_pds_ini_dn0 = assign16100_e22884_d_n0;
        locals.var_pds_ini_dn2 = assign16100_e22884_d_n2;
        locals.var_pds_ini_dn6 = assign16100_e22884_d_n6;
        locals.var_pds_ini_dn7 = assign16100_e22884_d_n7;
        locals.var_pds_ini_dn10 = assign16100_e22884_d_n10;
        locals.var_pds_ini_dn11 = assign16100_e22884_d_n11;
        locals.var_pds_ini_dn12 = assign16100_e22884_d_n12;
        locals.var_pds_ini_dn17 = assign16100_e22884_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign16110_e22901, assign16110_e22901_d_n0, assign16110_e22901_d_n2, assign16110_e22901_d_n6, assign16110_e22901_d_n7, assign16110_e22901_d_n10, assign16110_e22901_d_n11, assign16110_e22901_d_n12, assign16110_e22901_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard478 != 0.0)) {
        let (assign16110_e22899, assign16110_e22899_d_n0, assign16110_e22899_d_n2, assign16110_e22899_d_n6, assign16110_e22899_d_n7, assign16110_e22899_d_n10, assign16110_e22899_d_n11, assign16110_e22899_d_n12, assign16110_e22899_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign16110_e22899, assign16110_e22899_d_n0, assign16110_e22899_d_n2, assign16110_e22899_d_n6, assign16110_e22899_d_n7, assign16110_e22899_d_n10, assign16110_e22899_d_n11, assign16110_e22899_d_n12, assign16110_e22899_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16110_e22901;
        locals.var_pds_ini_dn0 = assign16110_e22901_d_n0;
        locals.var_pds_ini_dn2 = assign16110_e22901_d_n2;
        locals.var_pds_ini_dn6 = assign16110_e22901_d_n6;
        locals.var_pds_ini_dn7 = assign16110_e22901_d_n7;
        locals.var_pds_ini_dn10 = assign16110_e22901_d_n10;
        locals.var_pds_ini_dn11 = assign16110_e22901_d_n11;
        locals.var_pds_ini_dn12 = assign16110_e22901_d_n12;
        locals.var_pds_ini_dn17 = assign16110_e22901_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16120_e22904: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign16120_e22904;
        locals.var_guard479_rv = 0.0;

        let (assign16130_e22916, assign16130_e22916_d_n0, assign16130_e22916_d_n2, assign16130_e22916_d_n6, assign16130_e22916_d_n7, assign16130_e22916_d_n10, assign16130_e22916_d_n11, assign16130_e22916_d_n12, assign16130_e22916_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard479 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16130_e22916;
        locals.var_pds_ini_dn0 = assign16130_e22916_d_n0;
        locals.var_pds_ini_dn2 = assign16130_e22916_d_n2;
        locals.var_pds_ini_dn6 = assign16130_e22916_d_n6;
        locals.var_pds_ini_dn7 = assign16130_e22916_d_n7;
        locals.var_pds_ini_dn10 = assign16130_e22916_d_n10;
        locals.var_pds_ini_dn11 = assign16130_e22916_d_n11;
        locals.var_pds_ini_dn12 = assign16130_e22916_d_n12;
        locals.var_pds_ini_dn17 = assign16130_e22916_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16140_e22919: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard480 = assign16140_e22919;
        locals.var_guard480_rv = 0.0;

        let (assign16150_e22934, assign16150_e22934_d_n0, assign16150_e22934_d_n2, assign16150_e22934_d_n6, assign16150_e22934_d_n7, assign16150_e22934_d_n10, assign16150_e22934_d_n11, assign16150_e22934_d_n12, assign16150_e22934_d_n17,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard479 == 0.0)) && (locals.var_guard480 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16150_e22934;
        locals.var_pds_ini_dn0 = assign16150_e22934_d_n0;
        locals.var_pds_ini_dn2 = assign16150_e22934_d_n2;
        locals.var_pds_ini_dn6 = assign16150_e22934_d_n6;
        locals.var_pds_ini_dn7 = assign16150_e22934_d_n7;
        locals.var_pds_ini_dn10 = assign16150_e22934_d_n10;
        locals.var_pds_ini_dn11 = assign16150_e22934_d_n11;
        locals.var_pds_ini_dn12 = assign16150_e22934_d_n12;
        locals.var_pds_ini_dn17 = assign16150_e22934_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign16160_e22944, assign16160_e22944_d_n0, assign16160_e22944_d_n2, assign16160_e22944_d_n6, assign16160_e22944_d_n7, assign16160_e22944_d_n10, assign16160_e22944_d_n11, assign16160_e22944_d_n12, assign16160_e22944_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16160_e22944;
        locals.var_pds_dn0 = assign16160_e22944_d_n0;
        locals.var_pds_dn2 = assign16160_e22944_d_n2;
        locals.var_pds_dn6 = assign16160_e22944_d_n6;
        locals.var_pds_dn7 = assign16160_e22944_d_n7;
        locals.var_pds_dn10 = assign16160_e22944_d_n10;
        locals.var_pds_dn11 = assign16160_e22944_d_n11;
        locals.var_pds_dn12 = assign16160_e22944_d_n12;
        locals.var_pds_dn17 = assign16160_e22944_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign16170_e22956, assign16170_e22956_d_n0, assign16170_e22956_d_n2, assign16170_e22956_d_n6, assign16170_e22956_d_n7, assign16170_e22956_d_n10, assign16170_e22956_d_n11, assign16170_e22956_d_n12, assign16170_e22956_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) {
        let assign16170_e22954: f64 = (locals.var_ps0 + locals.var_pds);
        (assign16170_e22954, (locals.var_ps0_dn0 + locals.var_pds_dn0), (locals.var_ps0_dn2 + locals.var_pds_dn2), (locals.var_ps0_dn6 + locals.var_pds_dn6), (locals.var_ps0_dn7 + locals.var_pds_dn7), (locals.var_ps0_dn10 + locals.var_pds_dn10), (locals.var_ps0_dn11 + locals.var_pds_dn11), (locals.var_ps0_dn12 + locals.var_pds_dn12), (locals.var_ps0_dn17 + locals.var_pds_dn17),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16170_e22956;
        locals.var_psl_dn0 = assign16170_e22956_d_n0;
        locals.var_psl_dn2 = assign16170_e22956_d_n2;
        locals.var_psl_dn6 = assign16170_e22956_d_n6;
        locals.var_psl_dn7 = assign16170_e22956_d_n7;
        locals.var_psl_dn10 = assign16170_e22956_d_n10;
        locals.var_psl_dn11 = assign16170_e22956_d_n11;
        locals.var_psl_dn12 = assign16170_e22956_d_n12;
        locals.var_psl_dn17 = assign16170_e22956_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16180_e22966,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard476 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign16180_e22966;
        locals.var_flg_conv_rv = 0.0;

        let (assign16190_e22973, assign16190_e22973_d_n0, assign16190_e22973_d_n2, assign16190_e22973_d_n6, assign16190_e22973_d_n7, assign16190_e22973_d_n10, assign16190_e22973_d_n11, assign16190_e22973_d_n12, assign16190_e22973_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign16190_e22973;
        locals.var_phi_sl_soi_dn0 = assign16190_e22973_d_n0;
        locals.var_phi_sl_soi_dn2 = assign16190_e22973_d_n2;
        locals.var_phi_sl_soi_dn6 = assign16190_e22973_d_n6;
        locals.var_phi_sl_soi_dn7 = assign16190_e22973_d_n7;
        locals.var_phi_sl_soi_dn10 = assign16190_e22973_d_n10;
        locals.var_phi_sl_soi_dn11 = assign16190_e22973_d_n11;
        locals.var_phi_sl_soi_dn12 = assign16190_e22973_d_n12;
        locals.var_phi_sl_soi_dn17 = assign16190_e22973_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign16200_e22980,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign16200_e22980;
        locals.var_lp_sl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        locals: &mut StampLocals,
    ) {
        let mut assign16210_loop_guard: usize = 0;
        while {
            let assign16210_cond_e22988: f64 = (locals.var_lp_sl_max + 1.0);
            let assign16210_cond_e22990: f64 = if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_lp_sl <= assign16210_cond_e22988)) { 1.0 } else { 0.0 };
            assign16210_cond_e22990 != 0.0
        } {
            assign16210_loop_guard += 1;
            assert!(assign16210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign16210_body0_e22999, assign16210_body0_e22999_d_n0, assign16210_body0_e22999_d_n2, assign16210_body0_e22999_d_n6, assign16210_body0_e22999_d_n7, assign16210_body0_e22999_d_n10, assign16210_body0_e22999_d_n11, assign16210_body0_e22999_d_n12, assign16210_body0_e22999_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body0_e22997: f64 = (locals.var_phi_sl_soi - locals.var_vbcs_cl);
        (assign16210_body0_e22997, (locals.var_phi_sl_soi_dn0 - locals.var_vbcs_cl_dn0), (locals.var_phi_sl_soi_dn2 - locals.var_vbcs_cl_dn2), (locals.var_phi_sl_soi_dn6 - locals.var_vbcs_cl_dn6), (locals.var_phi_sl_soi_dn7 - locals.var_vbcs_cl_dn7), (locals.var_phi_sl_soi_dn10 - locals.var_vbcs_cl_dn10), (locals.var_phi_sl_soi_dn11 - locals.var_vbcs_cl_dn11), (locals.var_phi_sl_soi_dn12 - locals.var_vbcs_cl_dn12), (locals.var_phi_sl_soi_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_phi_soil, locals.var_phi_soil_dn0, locals.var_phi_soil_dn2, locals.var_phi_soil_dn6, locals.var_phi_soil_dn7, locals.var_phi_soil_dn10, locals.var_phi_soil_dn11, locals.var_phi_soil_dn12, locals.var_phi_soil_dn17,)
    }
};
            locals.var_phi_soil = assign16210_body0_e22999;
            locals.var_phi_soil_dn0 = assign16210_body0_e22999_d_n0;
            locals.var_phi_soil_dn2 = assign16210_body0_e22999_d_n2;
            locals.var_phi_soil_dn6 = assign16210_body0_e22999_d_n6;
            locals.var_phi_soil_dn7 = assign16210_body0_e22999_d_n7;
            locals.var_phi_soil_dn10 = assign16210_body0_e22999_d_n10;
            locals.var_phi_soil_dn11 = assign16210_body0_e22999_d_n11;
            locals.var_phi_soil_dn12 = assign16210_body0_e22999_d_n12;
            locals.var_phi_soil_dn17 = assign16210_body0_e22999_d_n17;
            locals.var_phi_soil_rv = 0.0;
            let (assign16210_body1_e23008, assign16210_body1_e23008_d_n0, assign16210_body1_e23008_d_n2, assign16210_body1_e23008_d_n6, assign16210_body1_e23008_d_n7, assign16210_body1_e23008_d_n10, assign16210_body1_e23008_d_n11, assign16210_body1_e23008_d_n12, assign16210_body1_e23008_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body1_e23006: f64 = (locals.var_beta * locals.var_phi_soil);
        (assign16210_body1_e23006, (locals.var_beta * locals.var_phi_soil_dn0), (locals.var_beta * locals.var_phi_soil_dn2), (locals.var_beta * locals.var_phi_soil_dn6), (locals.var_beta * locals.var_phi_soil_dn7), ((locals.var_beta_dn10 * locals.var_phi_soil) + (locals.var_beta * locals.var_phi_soil_dn10)), (locals.var_beta * locals.var_phi_soil_dn11), (locals.var_beta * locals.var_phi_soil_dn12), (locals.var_beta * locals.var_phi_soil_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign16210_body1_e23008;
            locals.var_chi_dn0 = assign16210_body1_e23008_d_n0;
            locals.var_chi_dn2 = assign16210_body1_e23008_d_n2;
            locals.var_chi_dn6 = assign16210_body1_e23008_d_n6;
            locals.var_chi_dn7 = assign16210_body1_e23008_d_n7;
            locals.var_chi_dn10 = assign16210_body1_e23008_d_n10;
            locals.var_chi_dn11 = assign16210_body1_e23008_d_n11;
            locals.var_chi_dn12 = assign16210_body1_e23008_d_n12;
            locals.var_chi_dn17 = assign16210_body1_e23008_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign16210_body2_e23019, assign16210_body2_e23019_d_n0, assign16210_body2_e23019_d_n2, assign16210_body2_e23019_d_n6, assign16210_body2_e23019_d_n7, assign16210_body2_e23019_d_n10, assign16210_body2_e23019_d_n11, assign16210_body2_e23019_d_n12, assign16210_body2_e23019_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body2_e23016: f64 = (locals.var_phi_soil - locals.var_dphi_sb);
        let assign16210_body2_e23017: f64 = (locals.var_c_sb * assign16210_body2_e23016);
        (assign16210_body2_e23017, ((locals.var_c_sb_dn0 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign16210_body2_e23016) + (locals.var_c_sb * (locals.var_phi_soil_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign16210_body2_e23019;
            locals.var_ty_dn0 = assign16210_body2_e23019_d_n0;
            locals.var_ty_dn2 = assign16210_body2_e23019_d_n2;
            locals.var_ty_dn6 = assign16210_body2_e23019_d_n6;
            locals.var_ty_dn7 = assign16210_body2_e23019_d_n7;
            locals.var_ty_dn10 = assign16210_body2_e23019_d_n10;
            locals.var_ty_dn11 = assign16210_body2_e23019_d_n11;
            locals.var_ty_dn12 = assign16210_body2_e23019_d_n12;
            locals.var_ty_dn17 = assign16210_body2_e23019_d_n17;
            locals.var_ty_rv = 0.0;
            let assign16210_body3_e23022: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard481 = assign16210_body3_e23022;
            locals.var_guard481_rv = 0.0;
            let (assign16210_body4_e23032, assign16210_body4_e23032_d_n0, assign16210_body4_e23032_d_n2, assign16210_body4_e23032_d_n6, assign16210_body4_e23032_d_n7, assign16210_body4_e23032_d_n10, assign16210_body4_e23032_d_n11, assign16210_body4_e23032_d_n12, assign16210_body4_e23032_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body4_e23030: f64 = (locals.var_ty).exp();
        (assign16210_body4_e23030, (assign16210_body4_e23030 * locals.var_ty_dn0), (assign16210_body4_e23030 * locals.var_ty_dn2), (assign16210_body4_e23030 * locals.var_ty_dn6), (assign16210_body4_e23030 * locals.var_ty_dn7), (assign16210_body4_e23030 * locals.var_ty_dn10), (assign16210_body4_e23030 * locals.var_ty_dn11), (assign16210_body4_e23030 * locals.var_ty_dn12), (assign16210_body4_e23030 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16210_body4_e23032;
            locals.var_t1_dn0 = assign16210_body4_e23032_d_n0;
            locals.var_t1_dn2 = assign16210_body4_e23032_d_n2;
            locals.var_t1_dn6 = assign16210_body4_e23032_d_n6;
            locals.var_t1_dn7 = assign16210_body4_e23032_d_n7;
            locals.var_t1_dn10 = assign16210_body4_e23032_d_n10;
            locals.var_t1_dn11 = assign16210_body4_e23032_d_n11;
            locals.var_t1_dn12 = assign16210_body4_e23032_d_n12;
            locals.var_t1_dn17 = assign16210_body4_e23032_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16210_body5_e23045, assign16210_body5_e23045_d_n0, assign16210_body5_e23045_d_n2, assign16210_body5_e23045_d_n6, assign16210_body5_e23045_d_n7, assign16210_body5_e23045_d_n10, assign16210_body5_e23045_d_n11, assign16210_body5_e23045_d_n12, assign16210_body5_e23045_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body5_e23040: f64 = (-locals.var_c_sb);
        let assign16210_body5_e23042: f64 = (assign16210_body5_e23040 * locals.var_dphi_sb);
        let assign16210_body5_e23043: f64 = (assign16210_body5_e23042).exp();
        (assign16210_body5_e23043, (assign16210_body5_e23043 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn0))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn2))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn6))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn7))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn10))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn11))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn12))), (assign16210_body5_e23043 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign16210_body5_e23040 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16210_body5_e23045;
            locals.var_t0_dn0 = assign16210_body5_e23045_d_n0;
            locals.var_t0_dn2 = assign16210_body5_e23045_d_n2;
            locals.var_t0_dn6 = assign16210_body5_e23045_d_n6;
            locals.var_t0_dn7 = assign16210_body5_e23045_d_n7;
            locals.var_t0_dn10 = assign16210_body5_e23045_d_n10;
            locals.var_t0_dn11 = assign16210_body5_e23045_d_n11;
            locals.var_t0_dn12 = assign16210_body5_e23045_d_n12;
            locals.var_t0_dn17 = assign16210_body5_e23045_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16210_body6_e23056, assign16210_body6_e23056_d_n0, assign16210_body6_e23056_d_n2, assign16210_body6_e23056_d_n6, assign16210_body6_e23056_d_n7, assign16210_body6_e23056_d_n10, assign16210_body6_e23056_d_n11, assign16210_body6_e23056_d_n12, assign16210_body6_e23056_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body6_e23054: f64 = (locals.var_t1 - locals.var_t0);
        (assign16210_body6_e23054, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign16210_body6_e23056;
            locals.var_t2_dn0 = assign16210_body6_e23056_d_n0;
            locals.var_t2_dn2 = assign16210_body6_e23056_d_n2;
            locals.var_t2_dn6 = assign16210_body6_e23056_d_n6;
            locals.var_t2_dn7 = assign16210_body6_e23056_d_n7;
            locals.var_t2_dn10 = assign16210_body6_e23056_d_n10;
            locals.var_t2_dn11 = assign16210_body6_e23056_d_n11;
            locals.var_t2_dn12 = assign16210_body6_e23056_d_n12;
            locals.var_t2_dn17 = assign16210_body6_e23056_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign16210_body7_e23070, assign16210_body7_e23070_d_n0, assign16210_body7_e23070_d_n2, assign16210_body7_e23070_d_n6, assign16210_body7_e23070_d_n7, assign16210_body7_e23070_d_n10, assign16210_body7_e23070_d_n11, assign16210_body7_e23070_d_n12, assign16210_body7_e23070_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body7_e23065: f64 = (1.0 + locals.var_t2);
        let assign16210_body7_e23066: f64 = (assign16210_body7_e23065).ln();
        let assign16210_body7_e23068: f64 = (assign16210_body7_e23066 / locals.var_c_sb);
        (assign16210_body7_e23068, ((((locals.var_t2_dn0 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign16210_body7_e23065) * locals.var_c_sb) - (assign16210_body7_e23066 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign16210_body7_e23070;
            locals.var_phi_soib_dn0 = assign16210_body7_e23070_d_n0;
            locals.var_phi_soib_dn2 = assign16210_body7_e23070_d_n2;
            locals.var_phi_soib_dn6 = assign16210_body7_e23070_d_n6;
            locals.var_phi_soib_dn7 = assign16210_body7_e23070_d_n7;
            locals.var_phi_soib_dn10 = assign16210_body7_e23070_d_n10;
            locals.var_phi_soib_dn11 = assign16210_body7_e23070_d_n11;
            locals.var_phi_soib_dn12 = assign16210_body7_e23070_d_n12;
            locals.var_phi_soib_dn17 = assign16210_body7_e23070_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign16210_body8_e23083, assign16210_body8_e23083_d_n0, assign16210_body8_e23083_d_n2, assign16210_body8_e23083_d_n6, assign16210_body8_e23083_d_n7, assign16210_body8_e23083_d_n10, assign16210_body8_e23083_d_n11, assign16210_body8_e23083_d_n12, assign16210_body8_e23083_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16210_body8_e23080: f64 = (1.0 + locals.var_t2);
        let assign16210_body8_e23081: f64 = (locals.var_t1 / assign16210_body8_e23080);
        (assign16210_body8_e23081, (((locals.var_t1_dn0 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn0)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn2 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn2)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn6 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn6)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn7 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn7)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn10 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn10)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn11 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn11)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn12 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn12)) / (assign16210_body8_e23080 * assign16210_body8_e23080)), (((locals.var_t1_dn17 * assign16210_body8_e23080) - (locals.var_t1 * locals.var_t2_dn17)) / (assign16210_body8_e23080 * assign16210_body8_e23080)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign16210_body8_e23083;
            locals.var_phi_soib_dpss_dn0 = assign16210_body8_e23083_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign16210_body8_e23083_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign16210_body8_e23083_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign16210_body8_e23083_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign16210_body8_e23083_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign16210_body8_e23083_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign16210_body8_e23083_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign16210_body8_e23083_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign16210_body9_e23095, assign16210_body9_e23095_d_n0, assign16210_body9_e23095_d_n2, assign16210_body9_e23095_d_n6, assign16210_body9_e23095_d_n7, assign16210_body9_e23095_d_n10, assign16210_body9_e23095_d_n11, assign16210_body9_e23095_d_n12, assign16210_body9_e23095_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 == 0.0)) {
        let assign16210_body9_e23093: f64 = (locals.var_phi_soil - locals.var_dphi_sb);
        (assign16210_body9_e23093, (locals.var_phi_soil_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soil_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soil_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soil_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soil_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soil_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soil_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soil_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign16210_body9_e23095;
            locals.var_phi_soib_dn0 = assign16210_body9_e23095_d_n0;
            locals.var_phi_soib_dn2 = assign16210_body9_e23095_d_n2;
            locals.var_phi_soib_dn6 = assign16210_body9_e23095_d_n6;
            locals.var_phi_soib_dn7 = assign16210_body9_e23095_d_n7;
            locals.var_phi_soib_dn10 = assign16210_body9_e23095_d_n10;
            locals.var_phi_soib_dn11 = assign16210_body9_e23095_d_n11;
            locals.var_phi_soib_dn12 = assign16210_body9_e23095_d_n12;
            locals.var_phi_soib_dn17 = assign16210_body9_e23095_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign16210_body10_e23105, assign16210_body10_e23105_d_n0, assign16210_body10_e23105_d_n2, assign16210_body10_e23105_d_n6, assign16210_body10_e23105_d_n7, assign16210_body10_e23105_d_n10, assign16210_body10_e23105_d_n11, assign16210_body10_e23105_d_n12, assign16210_body10_e23105_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard481 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign16210_body10_e23105;
            locals.var_phi_soib_dpss_dn0 = assign16210_body10_e23105_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign16210_body10_e23105_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign16210_body10_e23105_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign16210_body10_e23105_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign16210_body10_e23105_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign16210_body10_e23105_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign16210_body10_e23105_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign16210_body10_e23105_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign16210_body11_e23114, assign16210_body11_e23114_d_n0, assign16210_body11_e23114_d_n2, assign16210_body11_e23114_d_n6, assign16210_body11_e23114_d_n7, assign16210_body11_e23114_d_n10, assign16210_body11_e23114_d_n11, assign16210_body11_e23114_d_n12, assign16210_body11_e23114_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body11_e23112: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign16210_body11_e23112, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign16210_body11_e23114;
            locals.var_chib_dn0 = assign16210_body11_e23114_d_n0;
            locals.var_chib_dn2 = assign16210_body11_e23114_d_n2;
            locals.var_chib_dn6 = assign16210_body11_e23114_d_n6;
            locals.var_chib_dn7 = assign16210_body11_e23114_d_n7;
            locals.var_chib_dn10 = assign16210_body11_e23114_d_n10;
            locals.var_chib_dn11 = assign16210_body11_e23114_d_n11;
            locals.var_chib_dn12 = assign16210_body11_e23114_d_n12;
            locals.var_chib_dn17 = assign16210_body11_e23114_d_n17;
            locals.var_chib_rv = 0.0;
            let assign16210_body12_e23116: f64 = (locals.var_chi).abs();
            let assign16210_body12_e23118: f64 = if assign16210_body12_e23116 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard482 = assign16210_body12_e23118;
            locals.var_guard482_rv = 0.0;
            let (assign16210_body13_e23134, assign16210_body13_e23134_d_n0, assign16210_body13_e23134_d_n2, assign16210_body13_e23134_d_n6, assign16210_body13_e23134_d_n7, assign16210_body13_e23134_d_n10, assign16210_body13_e23134_d_n11, assign16210_body13_e23134_d_n12, assign16210_body13_e23134_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16210_body13_e23128: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign16210_body13_e23129: f64 = (1.0 - assign16210_body13_e23128);
        let assign16210_body13_e23131: f64 = (assign16210_body13_e23129 / 2.0);
        let assign16210_body13_e23132: f64 = (assign16210_body13_e23131).sqrt();
        (assign16210_body13_e23132, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign16210_body13_e23132)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign16210_body13_e23132)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16210_body13_e23134;
            locals.var_t0_dn0 = assign16210_body13_e23134_d_n0;
            locals.var_t0_dn2 = assign16210_body13_e23134_d_n2;
            locals.var_t0_dn6 = assign16210_body13_e23134_d_n6;
            locals.var_t0_dn7 = assign16210_body13_e23134_d_n7;
            locals.var_t0_dn10 = assign16210_body13_e23134_d_n10;
            locals.var_t0_dn11 = assign16210_body13_e23134_d_n11;
            locals.var_t0_dn12 = assign16210_body13_e23134_d_n12;
            locals.var_t0_dn17 = assign16210_body13_e23134_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16210_body14_e23145, assign16210_body14_e23145_d_n0, assign16210_body14_e23145_d_n2, assign16210_body14_e23145_d_n6, assign16210_body14_e23145_d_n7, assign16210_body14_e23145_d_n10, assign16210_body14_e23145_d_n11, assign16210_body14_e23145_d_n12, assign16210_body14_e23145_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16210_body14_e23143: f64 = (locals.var_chi * locals.var_t0);
        (assign16210_body14_e23143, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16210_body14_e23145;
            locals.var_fb_dn0 = assign16210_body14_e23145_d_n0;
            locals.var_fb_dn2 = assign16210_body14_e23145_d_n2;
            locals.var_fb_dn6 = assign16210_body14_e23145_d_n6;
            locals.var_fb_dn7 = assign16210_body14_e23145_d_n7;
            locals.var_fb_dn10 = assign16210_body14_e23145_d_n10;
            locals.var_fb_dn11 = assign16210_body14_e23145_d_n11;
            locals.var_fb_dn12 = assign16210_body14_e23145_d_n12;
            locals.var_fb_dn17 = assign16210_body14_e23145_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16210_body15_e23156, assign16210_body15_e23156_d_n0, assign16210_body15_e23156_d_n2, assign16210_body15_e23156_d_n6, assign16210_body15_e23156_d_n7, assign16210_body15_e23156_d_n10, assign16210_body15_e23156_d_n11, assign16210_body15_e23156_d_n12, assign16210_body15_e23156_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16210_body15_e23154: f64 = (locals.var_beta * locals.var_t0);
        (assign16210_body15_e23154, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16210_body15_e23156;
            locals.var_fb_dpss_dn0 = assign16210_body15_e23156_d_n0;
            locals.var_fb_dpss_dn2 = assign16210_body15_e23156_d_n2;
            locals.var_fb_dpss_dn6 = assign16210_body15_e23156_d_n6;
            locals.var_fb_dpss_dn7 = assign16210_body15_e23156_d_n7;
            locals.var_fb_dpss_dn10 = assign16210_body15_e23156_d_n10;
            locals.var_fb_dpss_dn11 = assign16210_body15_e23156_d_n11;
            locals.var_fb_dpss_dn12 = assign16210_body15_e23156_d_n12;
            locals.var_fb_dpss_dn17 = assign16210_body15_e23156_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16210_body16_e23159: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard483 = assign16210_body16_e23159;
            locals.var_guard483_rv = 0.0;
            let (assign16210_body17_e23171, assign16210_body17_e23171_d_n0, assign16210_body17_e23171_d_n2, assign16210_body17_e23171_d_n6, assign16210_body17_e23171_d_n7, assign16210_body17_e23171_d_n10, assign16210_body17_e23171_d_n11, assign16210_body17_e23171_d_n12, assign16210_body17_e23171_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16210_body17_e23169: f64 = (-locals.var_fb);
        (assign16210_body17_e23169, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16210_body17_e23171;
            locals.var_fb_dn0 = assign16210_body17_e23171_d_n0;
            locals.var_fb_dn2 = assign16210_body17_e23171_d_n2;
            locals.var_fb_dn6 = assign16210_body17_e23171_d_n6;
            locals.var_fb_dn7 = assign16210_body17_e23171_d_n7;
            locals.var_fb_dn10 = assign16210_body17_e23171_d_n10;
            locals.var_fb_dn11 = assign16210_body17_e23171_d_n11;
            locals.var_fb_dn12 = assign16210_body17_e23171_d_n12;
            locals.var_fb_dn17 = assign16210_body17_e23171_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16210_body18_e23183, assign16210_body18_e23183_d_n0, assign16210_body18_e23183_d_n2, assign16210_body18_e23183_d_n6, assign16210_body18_e23183_d_n7, assign16210_body18_e23183_d_n10, assign16210_body18_e23183_d_n11, assign16210_body18_e23183_d_n12, assign16210_body18_e23183_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 != 0.0)) && (locals.var_guard483 != 0.0)) {
        let assign16210_body18_e23181: f64 = (-locals.var_fb_dpss);
        (assign16210_body18_e23181, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16210_body18_e23183;
            locals.var_fb_dpss_dn0 = assign16210_body18_e23183_d_n0;
            locals.var_fb_dpss_dn2 = assign16210_body18_e23183_d_n2;
            locals.var_fb_dpss_dn6 = assign16210_body18_e23183_d_n6;
            locals.var_fb_dpss_dn7 = assign16210_body18_e23183_d_n7;
            locals.var_fb_dpss_dn10 = assign16210_body18_e23183_d_n10;
            locals.var_fb_dpss_dn11 = assign16210_body18_e23183_d_n11;
            locals.var_fb_dpss_dn12 = assign16210_body18_e23183_d_n12;
            locals.var_fb_dpss_dn17 = assign16210_body18_e23183_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16210_body19_e23185: f64 = (locals.var_chi).abs();
            let assign16210_body19_e23187: f64 = if assign16210_body19_e23185 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard484 = assign16210_body19_e23187;
            locals.var_guard484_rv = 0.0;
            let (assign16210_body20_e23221, assign16210_body20_e23221_d_n0, assign16210_body20_e23221_d_n2, assign16210_body20_e23221_d_n6, assign16210_body20_e23221_d_n7, assign16210_body20_e23221_d_n10, assign16210_body20_e23221_d_n11, assign16210_body20_e23221_d_n12, assign16210_body20_e23221_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body20_e23199: f64 = (locals.var_chi * locals.var_chi);
        let assign16210_body20_e23201: f64 = (assign16210_body20_e23199 / 2.0);
        let assign16210_body20_e23205: f64 = (locals.var_chi / 3.0);
        let assign16210_body20_e23209: f64 = (locals.var_chi / 4.0);
        let assign16210_body20_e23213: f64 = (locals.var_chi / 5.0);
        let assign16210_body20_e23214: f64 = (1.0 - assign16210_body20_e23213);
        let assign16210_body20_e23215: f64 = (assign16210_body20_e23209 * assign16210_body20_e23214);
        let assign16210_body20_e23216: f64 = (1.0 - assign16210_body20_e23215);
        let assign16210_body20_e23217: f64 = (assign16210_body20_e23205 * assign16210_body20_e23216);
        let assign16210_body20_e23218: f64 = (1.0 - assign16210_body20_e23217);
        let assign16210_body20_e23219: f64 = (assign16210_body20_e23201 * assign16210_body20_e23218);
        (assign16210_body20_e23219, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn0 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn0 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn2 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn2 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn6 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn6 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn7 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn7 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn10 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn10 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn11 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn11 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn12 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn12 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign16210_body20_e23218) + (assign16210_body20_e23201 * (-(((locals.var_chi_dn17 / 3.0) * assign16210_body20_e23216) + (assign16210_body20_e23205 * (-(((locals.var_chi_dn17 / 4.0) * assign16210_body20_e23214) + (assign16210_body20_e23209 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16210_body20_e23221;
            locals.var_t0_dn0 = assign16210_body20_e23221_d_n0;
            locals.var_t0_dn2 = assign16210_body20_e23221_d_n2;
            locals.var_t0_dn6 = assign16210_body20_e23221_d_n6;
            locals.var_t0_dn7 = assign16210_body20_e23221_d_n7;
            locals.var_t0_dn10 = assign16210_body20_e23221_d_n10;
            locals.var_t0_dn11 = assign16210_body20_e23221_d_n11;
            locals.var_t0_dn12 = assign16210_body20_e23221_d_n12;
            locals.var_t0_dn17 = assign16210_body20_e23221_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16210_body21_e23251, assign16210_body21_e23251_d_n0, assign16210_body21_e23251_d_n2, assign16210_body21_e23251_d_n6, assign16210_body21_e23251_d_n7, assign16210_body21_e23251_d_n10, assign16210_body21_e23251_d_n11, assign16210_body21_e23251_d_n12, assign16210_body21_e23251_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body21_e23235: f64 = (locals.var_chi / 2.0);
        let assign16210_body21_e23239: f64 = (locals.var_chi / 3.0);
        let assign16210_body21_e23243: f64 = (locals.var_chi / 4.0);
        let assign16210_body21_e23244: f64 = (1.0 - assign16210_body21_e23243);
        let assign16210_body21_e23245: f64 = (assign16210_body21_e23239 * assign16210_body21_e23244);
        let assign16210_body21_e23246: f64 = (1.0 - assign16210_body21_e23245);
        let assign16210_body21_e23247: f64 = (assign16210_body21_e23235 * assign16210_body21_e23246);
        let assign16210_body21_e23248: f64 = (1.0 - assign16210_body21_e23247);
        let assign16210_body21_e23249: f64 = (locals.var_chi * assign16210_body21_e23248);
        (assign16210_body21_e23249, ((locals.var_chi_dn0 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn0 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn2 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn6 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn7 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn10 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn11 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn12 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign16210_body21_e23248) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign16210_body21_e23246) + (assign16210_body21_e23235 * (-(((locals.var_chi_dn17 / 3.0) * assign16210_body21_e23244) + (assign16210_body21_e23239 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16210_body21_e23251;
            locals.var_t1_dn0 = assign16210_body21_e23251_d_n0;
            locals.var_t1_dn2 = assign16210_body21_e23251_d_n2;
            locals.var_t1_dn6 = assign16210_body21_e23251_d_n6;
            locals.var_t1_dn7 = assign16210_body21_e23251_d_n7;
            locals.var_t1_dn10 = assign16210_body21_e23251_d_n10;
            locals.var_t1_dn11 = assign16210_body21_e23251_d_n11;
            locals.var_t1_dn12 = assign16210_body21_e23251_d_n12;
            locals.var_t1_dn17 = assign16210_body21_e23251_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16210_body22_e23285, assign16210_body22_e23285_d_n0, assign16210_body22_e23285_d_n2, assign16210_body22_e23285_d_n6, assign16210_body22_e23285_d_n7, assign16210_body22_e23285_d_n10, assign16210_body22_e23285_d_n11, assign16210_body22_e23285_d_n12, assign16210_body22_e23285_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body22_e23263: f64 = (locals.var_chib * locals.var_chib);
        let assign16210_body22_e23265: f64 = (assign16210_body22_e23263 / 2.0);
        let assign16210_body22_e23269: f64 = (locals.var_chib / 3.0);
        let assign16210_body22_e23273: f64 = (locals.var_chib / 4.0);
        let assign16210_body22_e23277: f64 = (locals.var_chib / 5.0);
        let assign16210_body22_e23278: f64 = (1.0 - assign16210_body22_e23277);
        let assign16210_body22_e23279: f64 = (assign16210_body22_e23273 * assign16210_body22_e23278);
        let assign16210_body22_e23280: f64 = (1.0 - assign16210_body22_e23279);
        let assign16210_body22_e23281: f64 = (assign16210_body22_e23269 * assign16210_body22_e23280);
        let assign16210_body22_e23282: f64 = (1.0 - assign16210_body22_e23281);
        let assign16210_body22_e23283: f64 = (assign16210_body22_e23265 * assign16210_body22_e23282);
        (assign16210_body22_e23283, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn0 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn0 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn2 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn2 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn6 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn6 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn7 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn7 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn10 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn10 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn11 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn11 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn12 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn12 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign16210_body22_e23282) + (assign16210_body22_e23265 * (-(((locals.var_chib_dn17 / 3.0) * assign16210_body22_e23280) + (assign16210_body22_e23269 * (-(((locals.var_chib_dn17 / 4.0) * assign16210_body22_e23278) + (assign16210_body22_e23273 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign16210_body22_e23285;
            locals.var_t2_dn0 = assign16210_body22_e23285_d_n0;
            locals.var_t2_dn2 = assign16210_body22_e23285_d_n2;
            locals.var_t2_dn6 = assign16210_body22_e23285_d_n6;
            locals.var_t2_dn7 = assign16210_body22_e23285_d_n7;
            locals.var_t2_dn10 = assign16210_body22_e23285_d_n10;
            locals.var_t2_dn11 = assign16210_body22_e23285_d_n11;
            locals.var_t2_dn12 = assign16210_body22_e23285_d_n12;
            locals.var_t2_dn17 = assign16210_body22_e23285_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign16210_body23_e23315, assign16210_body23_e23315_d_n0, assign16210_body23_e23315_d_n2, assign16210_body23_e23315_d_n6, assign16210_body23_e23315_d_n7, assign16210_body23_e23315_d_n10, assign16210_body23_e23315_d_n11, assign16210_body23_e23315_d_n12, assign16210_body23_e23315_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body23_e23299: f64 = (locals.var_chib / 2.0);
        let assign16210_body23_e23303: f64 = (locals.var_chib / 3.0);
        let assign16210_body23_e23307: f64 = (locals.var_chib / 4.0);
        let assign16210_body23_e23308: f64 = (1.0 - assign16210_body23_e23307);
        let assign16210_body23_e23309: f64 = (assign16210_body23_e23303 * assign16210_body23_e23308);
        let assign16210_body23_e23310: f64 = (1.0 - assign16210_body23_e23309);
        let assign16210_body23_e23311: f64 = (assign16210_body23_e23299 * assign16210_body23_e23310);
        let assign16210_body23_e23312: f64 = (1.0 - assign16210_body23_e23311);
        let assign16210_body23_e23313: f64 = (locals.var_chib * assign16210_body23_e23312);
        (assign16210_body23_e23313, ((locals.var_chib_dn0 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn0 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn2 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn6 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn7 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn10 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn11 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn12 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign16210_body23_e23312) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign16210_body23_e23310) + (assign16210_body23_e23299 * (-(((locals.var_chib_dn17 / 3.0) * assign16210_body23_e23308) + (assign16210_body23_e23303 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign16210_body23_e23315;
            locals.var_t3_dn0 = assign16210_body23_e23315_d_n0;
            locals.var_t3_dn2 = assign16210_body23_e23315_d_n2;
            locals.var_t3_dn6 = assign16210_body23_e23315_d_n6;
            locals.var_t3_dn7 = assign16210_body23_e23315_d_n7;
            locals.var_t3_dn10 = assign16210_body23_e23315_d_n10;
            locals.var_t3_dn11 = assign16210_body23_e23315_d_n11;
            locals.var_t3_dn12 = assign16210_body23_e23315_d_n12;
            locals.var_t3_dn17 = assign16210_body23_e23315_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign16210_body24_e23330, assign16210_body24_e23330_d_n0, assign16210_body24_e23330_d_n2, assign16210_body24_e23330_d_n6, assign16210_body24_e23330_d_n7, assign16210_body24_e23330_d_n10, assign16210_body24_e23330_d_n11, assign16210_body24_e23330_d_n12, assign16210_body24_e23330_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body24_e23327: f64 = (locals.var_t0 - locals.var_t2);
        let assign16210_body24_e23328: f64 = (assign16210_body24_e23327).sqrt();
        (assign16210_body24_e23328, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign16210_body24_e23328)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign16210_body24_e23328)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16210_body24_e23330;
            locals.var_fb_dn0 = assign16210_body24_e23330_d_n0;
            locals.var_fb_dn2 = assign16210_body24_e23330_d_n2;
            locals.var_fb_dn6 = assign16210_body24_e23330_d_n6;
            locals.var_fb_dn7 = assign16210_body24_e23330_d_n7;
            locals.var_fb_dn10 = assign16210_body24_e23330_d_n10;
            locals.var_fb_dn11 = assign16210_body24_e23330_d_n11;
            locals.var_fb_dn12 = assign16210_body24_e23330_d_n12;
            locals.var_fb_dn17 = assign16210_body24_e23330_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16210_body25_e23352, assign16210_body25_e23352_d_n0, assign16210_body25_e23352_d_n2, assign16210_body25_e23352_d_n6, assign16210_body25_e23352_d_n7, assign16210_body25_e23352_d_n10, assign16210_body25_e23352_d_n11, assign16210_body25_e23352_d_n12, assign16210_body25_e23352_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 != 0.0)) {
        let assign16210_body25_e23342: f64 = (locals.var_beta * 0.5);
        let assign16210_body25_e23346: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign16210_body25_e23347: f64 = (locals.var_t1 - assign16210_body25_e23346);
        let assign16210_body25_e23348: f64 = (assign16210_body25_e23342 * assign16210_body25_e23347);
        let assign16210_body25_e23350: f64 = (assign16210_body25_e23348 / locals.var_fb);
        (assign16210_body25_e23350, ((((assign16210_body25_e23342 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign16210_body25_e23347) + (assign16210_body25_e23342 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body25_e23342 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign16210_body25_e23348 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16210_body25_e23352;
            locals.var_fb_dpss_dn0 = assign16210_body25_e23352_d_n0;
            locals.var_fb_dpss_dn2 = assign16210_body25_e23352_d_n2;
            locals.var_fb_dpss_dn6 = assign16210_body25_e23352_d_n6;
            locals.var_fb_dpss_dn7 = assign16210_body25_e23352_d_n7;
            locals.var_fb_dpss_dn10 = assign16210_body25_e23352_d_n10;
            locals.var_fb_dpss_dn11 = assign16210_body25_e23352_d_n11;
            locals.var_fb_dpss_dn12 = assign16210_body25_e23352_d_n12;
            locals.var_fb_dpss_dn17 = assign16210_body25_e23352_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign16210_body26_e23367, assign16210_body26_e23367_d_n0, assign16210_body26_e23367_d_n2, assign16210_body26_e23367_d_n6, assign16210_body26_e23367_d_n7, assign16210_body26_e23367_d_n10, assign16210_body26_e23367_d_n11, assign16210_body26_e23367_d_n12, assign16210_body26_e23367_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 == 0.0)) {
        let assign16210_body26_e23364: f64 = (-locals.var_chi);
        let assign16210_body26_e23365: f64 = (assign16210_body26_e23364).exp();
        (assign16210_body26_e23365, (assign16210_body26_e23365 * (-locals.var_chi_dn0)), (assign16210_body26_e23365 * (-locals.var_chi_dn2)), (assign16210_body26_e23365 * (-locals.var_chi_dn6)), (assign16210_body26_e23365 * (-locals.var_chi_dn7)), (assign16210_body26_e23365 * (-locals.var_chi_dn10)), (assign16210_body26_e23365 * (-locals.var_chi_dn11)), (assign16210_body26_e23365 * (-locals.var_chi_dn12)), (assign16210_body26_e23365 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16210_body26_e23367;
            locals.var_t0_dn0 = assign16210_body26_e23367_d_n0;
            locals.var_t0_dn2 = assign16210_body26_e23367_d_n2;
            locals.var_t0_dn6 = assign16210_body26_e23367_d_n6;
            locals.var_t0_dn7 = assign16210_body26_e23367_d_n7;
            locals.var_t0_dn10 = assign16210_body26_e23367_d_n10;
            locals.var_t0_dn11 = assign16210_body26_e23367_d_n11;
            locals.var_t0_dn12 = assign16210_body26_e23367_d_n12;
            locals.var_t0_dn17 = assign16210_body26_e23367_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16210_body27_e23382, assign16210_body27_e23382_d_n0, assign16210_body27_e23382_d_n2, assign16210_body27_e23382_d_n6, assign16210_body27_e23382_d_n7, assign16210_body27_e23382_d_n10, assign16210_body27_e23382_d_n11, assign16210_body27_e23382_d_n12, assign16210_body27_e23382_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 == 0.0)) {
        let assign16210_body27_e23379: f64 = (-locals.var_chib);
        let assign16210_body27_e23380: f64 = (assign16210_body27_e23379).exp();
        (assign16210_body27_e23380, (assign16210_body27_e23380 * (-locals.var_chib_dn0)), (assign16210_body27_e23380 * (-locals.var_chib_dn2)), (assign16210_body27_e23380 * (-locals.var_chib_dn6)), (assign16210_body27_e23380 * (-locals.var_chib_dn7)), (assign16210_body27_e23380 * (-locals.var_chib_dn10)), (assign16210_body27_e23380 * (-locals.var_chib_dn11)), (assign16210_body27_e23380 * (-locals.var_chib_dn12)), (assign16210_body27_e23380 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16210_body27_e23382;
            locals.var_t1_dn0 = assign16210_body27_e23382_d_n0;
            locals.var_t1_dn2 = assign16210_body27_e23382_d_n2;
            locals.var_t1_dn6 = assign16210_body27_e23382_d_n6;
            locals.var_t1_dn7 = assign16210_body27_e23382_d_n7;
            locals.var_t1_dn10 = assign16210_body27_e23382_d_n10;
            locals.var_t1_dn11 = assign16210_body27_e23382_d_n11;
            locals.var_t1_dn12 = assign16210_body27_e23382_d_n12;
            locals.var_t1_dn17 = assign16210_body27_e23382_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16210_body28_e23402, assign16210_body28_e23402_d_n0, assign16210_body28_e23402_d_n2, assign16210_body28_e23402_d_n6, assign16210_body28_e23402_d_n7, assign16210_body28_e23402_d_n10, assign16210_body28_e23402_d_n11, assign16210_body28_e23402_d_n12, assign16210_body28_e23402_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 == 0.0)) {
        let assign16210_body28_e23395: f64 = (locals.var_chi - locals.var_chib);
        let assign16210_body28_e23398: f64 = (locals.var_t0 - locals.var_t1);
        let assign16210_body28_e23399: f64 = (assign16210_body28_e23395 + assign16210_body28_e23398);
        let assign16210_body28_e23400: f64 = (assign16210_body28_e23399).sqrt();
        (assign16210_body28_e23400, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign16210_body28_e23400)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign16210_body28_e23400)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16210_body28_e23402;
            locals.var_fb_dn0 = assign16210_body28_e23402_d_n0;
            locals.var_fb_dn2 = assign16210_body28_e23402_d_n2;
            locals.var_fb_dn6 = assign16210_body28_e23402_d_n6;
            locals.var_fb_dn7 = assign16210_body28_e23402_d_n7;
            locals.var_fb_dn10 = assign16210_body28_e23402_d_n10;
            locals.var_fb_dn11 = assign16210_body28_e23402_d_n11;
            locals.var_fb_dn12 = assign16210_body28_e23402_d_n12;
            locals.var_fb_dn17 = assign16210_body28_e23402_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16210_body29_e23429, assign16210_body29_e23429_d_n0, assign16210_body29_e23429_d_n2, assign16210_body29_e23429_d_n6, assign16210_body29_e23429_d_n7, assign16210_body29_e23429_d_n10, assign16210_body29_e23429_d_n11, assign16210_body29_e23429_d_n12, assign16210_body29_e23429_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard482 == 0.0)) && (locals.var_guard484 == 0.0)) {
        let assign16210_body29_e23415: f64 = (locals.var_beta * 0.5);
        let assign16210_body29_e23418: f64 = (1.0 - locals.var_t0);
        let assign16210_body29_e23422: f64 = (1.0 - locals.var_t1);
        let assign16210_body29_e23423: f64 = (locals.var_phi_soib_dpss * assign16210_body29_e23422);
        let assign16210_body29_e23424: f64 = (assign16210_body29_e23418 - assign16210_body29_e23423);
        let assign16210_body29_e23425: f64 = (assign16210_body29_e23415 * assign16210_body29_e23424);
        let assign16210_body29_e23427: f64 = (assign16210_body29_e23425 / locals.var_fb);
        (assign16210_body29_e23427, ((((assign16210_body29_e23415 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign16210_body29_e23424) + (assign16210_body29_e23415 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign16210_body29_e23415 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign16210_body29_e23422) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign16210_body29_e23425 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16210_body29_e23429;
            locals.var_fb_dpss_dn0 = assign16210_body29_e23429_d_n0;
            locals.var_fb_dpss_dn2 = assign16210_body29_e23429_d_n2;
            locals.var_fb_dpss_dn6 = assign16210_body29_e23429_d_n6;
            locals.var_fb_dpss_dn7 = assign16210_body29_e23429_d_n7;
            locals.var_fb_dpss_dn10 = assign16210_body29_e23429_d_n10;
            locals.var_fb_dpss_dn11 = assign16210_body29_e23429_d_n11;
            locals.var_fb_dpss_dn12 = assign16210_body29_e23429_d_n12;
            locals.var_fb_dpss_dn17 = assign16210_body29_e23429_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16210_body30_e23432: f64 = (-1.0);
            let assign16210_body30_e23433: f64 = if locals.var_flg_zone == assign16210_body30_e23432 { 1.0 } else { 0.0 };
            locals.var_guard485 = assign16210_body30_e23433;
            locals.var_guard485_rv = 0.0;
            let (assign16210_body31_e23442, assign16210_body31_e23442_d_n0, assign16210_body31_e23442_d_n2, assign16210_body31_e23442_d_n6, assign16210_body31_e23442_d_n7, assign16210_body31_e23442_d_n10, assign16210_body31_e23442_d_n11, assign16210_body31_e23442_d_n12, assign16210_body31_e23442_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard485 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign16210_body31_e23442;
            locals.var_wdsoi_dn0 = assign16210_body31_e23442_d_n0;
            locals.var_wdsoi_dn2 = assign16210_body31_e23442_d_n2;
            locals.var_wdsoi_dn6 = assign16210_body31_e23442_d_n6;
            locals.var_wdsoi_dn7 = assign16210_body31_e23442_d_n7;
            locals.var_wdsoi_dn10 = assign16210_body31_e23442_d_n10;
            locals.var_wdsoi_dn11 = assign16210_body31_e23442_d_n11;
            locals.var_wdsoi_dn12 = assign16210_body31_e23442_d_n12;
            locals.var_wdsoi_dn17 = assign16210_body31_e23442_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign16210_body32_e23454, assign16210_body32_e23454_d_n0, assign16210_body32_e23454_d_n2, assign16210_body32_e23454_d_n6, assign16210_body32_e23454_d_n7, assign16210_body32_e23454_d_n10, assign16210_body32_e23454_d_n11, assign16210_body32_e23454_d_n12, assign16210_body32_e23454_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard485 == 0.0)) {
        let assign16210_body32_e23452: f64 = (locals.var_c_w_soi * locals.var_fb);
        (assign16210_body32_e23452, ((locals.var_c_w_soi_dn0 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn0)), ((locals.var_c_w_soi_dn2 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn2)), ((locals.var_c_w_soi_dn6 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn6)), ((locals.var_c_w_soi_dn7 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn7)), ((locals.var_c_w_soi_dn10 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn10)), ((locals.var_c_w_soi_dn11 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn11)), ((locals.var_c_w_soi_dn12 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn12)), ((locals.var_c_w_soi_dn17 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn17)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign16210_body32_e23454;
            locals.var_wdsoi_dn0 = assign16210_body32_e23454_d_n0;
            locals.var_wdsoi_dn2 = assign16210_body32_e23454_d_n2;
            locals.var_wdsoi_dn6 = assign16210_body32_e23454_d_n6;
            locals.var_wdsoi_dn7 = assign16210_body32_e23454_d_n7;
            locals.var_wdsoi_dn10 = assign16210_body32_e23454_d_n10;
            locals.var_wdsoi_dn11 = assign16210_body32_e23454_d_n11;
            locals.var_wdsoi_dn12 = assign16210_body32_e23454_d_n12;
            locals.var_wdsoi_dn17 = assign16210_body32_e23454_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign16210_body33_e23463, assign16210_body33_e23463_d_n0, assign16210_body33_e23463_d_n2, assign16210_body33_e23463_d_n6, assign16210_body33_e23463_d_n7, assign16210_body33_e23463_d_n10, assign16210_body33_e23463_d_n11, assign16210_body33_e23463_d_n12, assign16210_body33_e23463_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body33_e23461: f64 = (locals.var_q_nsub * locals.var_wdsoi);
        (assign16210_body33_e23461, ((locals.var_q_nsub_dn0 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn0)), ((locals.var_q_nsub_dn2 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn2)), ((locals.var_q_nsub_dn6 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn6)), ((locals.var_q_nsub_dn7 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn7)), ((locals.var_q_nsub_dn10 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn10)), ((locals.var_q_nsub_dn11 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn11)), ((locals.var_q_nsub_dn12 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn12)), ((locals.var_q_nsub_dn17 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn17)),)
    } else {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    }
};
            locals.var_q_dep_soi = assign16210_body33_e23463;
            locals.var_q_dep_soi_dn0 = assign16210_body33_e23463_d_n0;
            locals.var_q_dep_soi_dn2 = assign16210_body33_e23463_d_n2;
            locals.var_q_dep_soi_dn6 = assign16210_body33_e23463_d_n6;
            locals.var_q_dep_soi_dn7 = assign16210_body33_e23463_d_n7;
            locals.var_q_dep_soi_dn10 = assign16210_body33_e23463_d_n10;
            locals.var_q_dep_soi_dn11 = assign16210_body33_e23463_d_n11;
            locals.var_q_dep_soi_dn12 = assign16210_body33_e23463_d_n12;
            locals.var_q_dep_soi_dn17 = assign16210_body33_e23463_d_n17;
            locals.var_q_dep_soi_rv = 0.0;
            let assign16210_body34_e23466: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard486 = assign16210_body34_e23466;
            locals.var_guard486_rv = 0.0;
            let (assign16210_body35_e23476, assign16210_body35_e23476_d_n0, assign16210_body35_e23476_d_n2, assign16210_body35_e23476_d_n6, assign16210_body35_e23476_d_n7, assign16210_body35_e23476_d_n10, assign16210_body35_e23476_d_n11, assign16210_body35_e23476_d_n12, assign16210_body35_e23476_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16210_body35_e23474: f64 = (-locals.var_fb);
        (assign16210_body35_e23474, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16210_body35_e23476;
            locals.var_fsl2_dn0 = assign16210_body35_e23476_d_n0;
            locals.var_fsl2_dn2 = assign16210_body35_e23476_d_n2;
            locals.var_fsl2_dn6 = assign16210_body35_e23476_d_n6;
            locals.var_fsl2_dn7 = assign16210_body35_e23476_d_n7;
            locals.var_fsl2_dn10 = assign16210_body35_e23476_d_n10;
            locals.var_fsl2_dn11 = assign16210_body35_e23476_d_n11;
            locals.var_fsl2_dn12 = assign16210_body35_e23476_d_n12;
            locals.var_fsl2_dn17 = assign16210_body35_e23476_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16210_body36_e23486, assign16210_body36_e23486_d_n0, assign16210_body36_e23486_d_n2, assign16210_body36_e23486_d_n6, assign16210_body36_e23486_d_n7, assign16210_body36_e23486_d_n10, assign16210_body36_e23486_d_n11, assign16210_body36_e23486_d_n12, assign16210_body36_e23486_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16210_body36_e23484: f64 = (-locals.var_fb_dpss);
        (assign16210_body36_e23484, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16210_body36_e23486;
            locals.var_fsl2_dpsl_dn0 = assign16210_body36_e23486_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16210_body36_e23486_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16210_body36_e23486_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16210_body36_e23486_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16210_body36_e23486_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16210_body36_e23486_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16210_body36_e23486_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16210_body36_e23486_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let assign16210_body37_e23489: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard487 = assign16210_body37_e23489;
            locals.var_guard487_rv = 0.0;
            let (assign16210_body38_e23501, assign16210_body38_e23501_d_n0, assign16210_body38_e23501_d_n2, assign16210_body38_e23501_d_n6, assign16210_body38_e23501_d_n7, assign16210_body38_e23501_d_n10, assign16210_body38_e23501_d_n11, assign16210_body38_e23501_d_n12, assign16210_body38_e23501_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16210_body38_e23501;
            locals.var_fsl2_dn0 = assign16210_body38_e23501_d_n0;
            locals.var_fsl2_dn2 = assign16210_body38_e23501_d_n2;
            locals.var_fsl2_dn6 = assign16210_body38_e23501_d_n6;
            locals.var_fsl2_dn7 = assign16210_body38_e23501_d_n7;
            locals.var_fsl2_dn10 = assign16210_body38_e23501_d_n10;
            locals.var_fsl2_dn11 = assign16210_body38_e23501_d_n11;
            locals.var_fsl2_dn12 = assign16210_body38_e23501_d_n12;
            locals.var_fsl2_dn17 = assign16210_body38_e23501_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16210_body39_e23513, assign16210_body39_e23513_d_n0, assign16210_body39_e23513_d_n2, assign16210_body39_e23513_d_n6, assign16210_body39_e23513_d_n7, assign16210_body39_e23513_d_n10, assign16210_body39_e23513_d_n11, assign16210_body39_e23513_d_n12, assign16210_body39_e23513_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16210_body39_e23513;
            locals.var_fsl2_dpsl_dn0 = assign16210_body39_e23513_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16210_body39_e23513_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16210_body39_e23513_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16210_body39_e23513_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16210_body39_e23513_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16210_body39_e23513_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16210_body39_e23513_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16210_body39_e23513_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign16210_body40_e23530, assign16210_body40_e23530_d_n0, assign16210_body40_e23530_d_n2, assign16210_body40_e23530_d_n6, assign16210_body40_e23530_d_n7, assign16210_body40_e23530_d_n10, assign16210_body40_e23530_d_n11, assign16210_body40_e23530_d_n12, assign16210_body40_e23530_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body40_e23527: f64 = (locals.var_phi_sl_soi - locals.var_vds);
        let assign16210_body40_e23528: f64 = (locals.var_beta * assign16210_body40_e23527);
        (assign16210_body40_e23528, (locals.var_beta * (locals.var_phi_sl_soi_dn0 - locals.var_vds_dn0)), (locals.var_beta * (locals.var_phi_sl_soi_dn2 - locals.var_vds_dn2)), (locals.var_beta * (locals.var_phi_sl_soi_dn6 - locals.var_vds_dn6)), (locals.var_beta * (locals.var_phi_sl_soi_dn7 - locals.var_vds_dn7)), ((locals.var_beta_dn10 * assign16210_body40_e23527) + (locals.var_beta * (locals.var_phi_sl_soi_dn10 - locals.var_vds_dn10))), (locals.var_beta * (locals.var_phi_sl_soi_dn11 - locals.var_vds_dn11)), (locals.var_beta * (locals.var_phi_sl_soi_dn12 - locals.var_vds_dn12)), (locals.var_beta * (locals.var_phi_sl_soi_dn17 - locals.var_vds_dn17)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn17,)
    }
};
            locals.var_rho = assign16210_body40_e23530;
            locals.var_rho_dn0 = assign16210_body40_e23530_d_n0;
            locals.var_rho_dn2 = assign16210_body40_e23530_d_n2;
            locals.var_rho_dn6 = assign16210_body40_e23530_d_n6;
            locals.var_rho_dn7 = assign16210_body40_e23530_d_n7;
            locals.var_rho_dn10 = assign16210_body40_e23530_d_n10;
            locals.var_rho_dn11 = assign16210_body40_e23530_d_n11;
            locals.var_rho_dn12 = assign16210_body40_e23530_d_n12;
            locals.var_rho_dn17 = assign16210_body40_e23530_d_n17;
            locals.var_rho_rv = 0.0;
            let (assign16210_body41_e23544, assign16210_body41_e23544_d_n0, assign16210_body41_e23544_d_n2, assign16210_body41_e23544_d_n6, assign16210_body41_e23544_d_n7, assign16210_body41_e23544_d_n10, assign16210_body41_e23544_d_n11, assign16210_body41_e23544_d_n12, assign16210_body41_e23544_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body41_e23542: f64 = (locals.var_rho).exp();
        (assign16210_body41_e23542, (assign16210_body41_e23542 * locals.var_rho_dn0), (assign16210_body41_e23542 * locals.var_rho_dn2), (assign16210_body41_e23542 * locals.var_rho_dn6), (assign16210_body41_e23542 * locals.var_rho_dn7), (assign16210_body41_e23542 * locals.var_rho_dn10), (assign16210_body41_e23542 * locals.var_rho_dn11), (assign16210_body41_e23542 * locals.var_rho_dn12), (assign16210_body41_e23542 * locals.var_rho_dn17),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn12, locals.var_exp_rho_dn17,)
    }
};
            locals.var_exp_rho = assign16210_body41_e23544;
            locals.var_exp_rho_dn0 = assign16210_body41_e23544_d_n0;
            locals.var_exp_rho_dn2 = assign16210_body41_e23544_d_n2;
            locals.var_exp_rho_dn6 = assign16210_body41_e23544_d_n6;
            locals.var_exp_rho_dn7 = assign16210_body41_e23544_d_n7;
            locals.var_exp_rho_dn10 = assign16210_body41_e23544_d_n10;
            locals.var_exp_rho_dn11 = assign16210_body41_e23544_d_n11;
            locals.var_exp_rho_dn12 = assign16210_body41_e23544_d_n12;
            locals.var_exp_rho_dn17 = assign16210_body41_e23544_d_n17;
            locals.var_exp_rho_rv = 0.0;
            let (assign16210_body42_e23565, assign16210_body42_e23565_d_n0, assign16210_body42_e23565_d_n2, assign16210_body42_e23565_d_n6, assign16210_body42_e23565_d_n7, assign16210_body42_e23565_d_n10, assign16210_body42_e23565_d_n11, assign16210_body42_e23565_d_n12, assign16210_body42_e23565_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body42_e23560: f64 = (locals.var_chi + 1.0);
        let assign16210_body42_e23561: f64 = (locals.var_exp_bvbsvds * assign16210_body42_e23560);
        let assign16210_body42_e23562: f64 = (locals.var_exp_rho - assign16210_body42_e23561);
        let assign16210_body42_e23563: f64 = (locals.var_cnst1soi * assign16210_body42_e23562);
        (assign16210_body42_e23563, ((locals.var_cnst1soi_dn0 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn12 - ((locals.var_exp_bvbsvds_dn12 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign16210_body42_e23562) + (locals.var_cnst1soi * (locals.var_exp_rho_dn17 - ((locals.var_exp_bvbsvds_dn17 * assign16210_body42_e23560) + (locals.var_exp_bvbsvds * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn10, locals.var_fsl1_dn11, locals.var_fsl1_dn12, locals.var_fsl1_dn17,)
    }
};
            locals.var_fsl1 = assign16210_body42_e23565;
            locals.var_fsl1_dn0 = assign16210_body42_e23565_d_n0;
            locals.var_fsl1_dn2 = assign16210_body42_e23565_d_n2;
            locals.var_fsl1_dn6 = assign16210_body42_e23565_d_n6;
            locals.var_fsl1_dn7 = assign16210_body42_e23565_d_n7;
            locals.var_fsl1_dn10 = assign16210_body42_e23565_d_n10;
            locals.var_fsl1_dn11 = assign16210_body42_e23565_d_n11;
            locals.var_fsl1_dn12 = assign16210_body42_e23565_d_n12;
            locals.var_fsl1_dn17 = assign16210_body42_e23565_d_n17;
            locals.var_fsl1_rv = 0.0;
            let (assign16210_body43_e23584, assign16210_body43_e23584_d_n0, assign16210_body43_e23584_d_n2, assign16210_body43_e23584_d_n6, assign16210_body43_e23584_d_n7, assign16210_body43_e23584_d_n10, assign16210_body43_e23584_d_n11, assign16210_body43_e23584_d_n12, assign16210_body43_e23584_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body43_e23578: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign16210_body43_e23581: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign16210_body43_e23582: f64 = (assign16210_body43_e23578 * assign16210_body43_e23581);
        (assign16210_body43_e23582, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn12 - locals.var_exp_bvbsvds_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign16210_body43_e23581) + (assign16210_body43_e23578 * (locals.var_exp_rho_dn17 - locals.var_exp_bvbsvds_dn17))),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn11, locals.var_fsl1_dpsl_dn12, locals.var_fsl1_dpsl_dn17,)
    }
};
            locals.var_fsl1_dpsl = assign16210_body43_e23584;
            locals.var_fsl1_dpsl_dn0 = assign16210_body43_e23584_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign16210_body43_e23584_d_n2;
            locals.var_fsl1_dpsl_dn6 = assign16210_body43_e23584_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign16210_body43_e23584_d_n7;
            locals.var_fsl1_dpsl_dn10 = assign16210_body43_e23584_d_n10;
            locals.var_fsl1_dpsl_dn11 = assign16210_body43_e23584_d_n11;
            locals.var_fsl1_dpsl_dn12 = assign16210_body43_e23584_d_n12;
            locals.var_fsl1_dpsl_dn17 = assign16210_body43_e23584_d_n17;
            locals.var_fsl1_dpsl_rv = 0.0;
            let (assign16210_body44_e23602, assign16210_body44_e23602_d_n0, assign16210_body44_e23602_d_n2, assign16210_body44_e23602_d_n6, assign16210_body44_e23602_d_n7, assign16210_body44_e23602_d_n10, assign16210_body44_e23602_d_n11, assign16210_body44_e23602_d_n12, assign16210_body44_e23602_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body44_e23597: f64 = (locals.var_fb * locals.var_fb);
        let assign16210_body44_e23599: f64 = (assign16210_body44_e23597 + locals.var_fsl1);
        let assign16210_body44_e23600: f64 = (assign16210_body44_e23599).sqrt();
        (assign16210_body44_e23600, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fsl1_dn0) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fsl1_dn2) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fsl1_dn6) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fsl1_dn7) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fsl1_dn10) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fsl1_dn11) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fsl1_dn12) / (2.0 * assign16210_body44_e23600)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fsl1_dn17) / (2.0 * assign16210_body44_e23600)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16210_body44_e23602;
            locals.var_fsl2_dn0 = assign16210_body44_e23602_d_n0;
            locals.var_fsl2_dn2 = assign16210_body44_e23602_d_n2;
            locals.var_fsl2_dn6 = assign16210_body44_e23602_d_n6;
            locals.var_fsl2_dn7 = assign16210_body44_e23602_d_n7;
            locals.var_fsl2_dn10 = assign16210_body44_e23602_d_n10;
            locals.var_fsl2_dn11 = assign16210_body44_e23602_d_n11;
            locals.var_fsl2_dn12 = assign16210_body44_e23602_d_n12;
            locals.var_fsl2_dn17 = assign16210_body44_e23602_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16210_body45_e23625, assign16210_body45_e23625_d_n0, assign16210_body45_e23625_d_n2, assign16210_body45_e23625_d_n6, assign16210_body45_e23625_d_n7, assign16210_body45_e23625_d_n10, assign16210_body45_e23625_d_n11, assign16210_body45_e23625_d_n12, assign16210_body45_e23625_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard487 == 0.0)) {
        let assign16210_body45_e23616: f64 = (2.0 * locals.var_fb_dpss);
        let assign16210_body45_e23618: f64 = (assign16210_body45_e23616 * locals.var_fb);
        let assign16210_body45_e23620: f64 = (assign16210_body45_e23618 + locals.var_fsl1_dpsl);
        let assign16210_body45_e23621: f64 = (0.5 * assign16210_body45_e23620);
        let assign16210_body45_e23623: f64 = (assign16210_body45_e23621 / locals.var_fsl2);
        (assign16210_body45_e23623, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn0)) + locals.var_fsl1_dpsl_dn0)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn0)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn2)) + locals.var_fsl1_dpsl_dn2)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn2)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn6)) + locals.var_fsl1_dpsl_dn6)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn6)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn7)) + locals.var_fsl1_dpsl_dn7)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn7)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn10)) + locals.var_fsl1_dpsl_dn10)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn10)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn11)) + locals.var_fsl1_dpsl_dn11)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn11)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn12)) + locals.var_fsl1_dpsl_dn12)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn12)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign16210_body45_e23616 * locals.var_fb_dn17)) + locals.var_fsl1_dpsl_dn17)) * locals.var_fsl2) - (assign16210_body45_e23621 * locals.var_fsl2_dn17)) / (locals.var_fsl2 * locals.var_fsl2)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16210_body45_e23625;
            locals.var_fsl2_dpsl_dn0 = assign16210_body45_e23625_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16210_body45_e23625_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16210_body45_e23625_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16210_body45_e23625_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16210_body45_e23625_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16210_body45_e23625_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16210_body45_e23625_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16210_body45_e23625_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign16210_body46_e23643, assign16210_body46_e23643_d_n0, assign16210_body46_e23643_d_n2, assign16210_body46_e23643_d_n6, assign16210_body46_e23643_d_n7, assign16210_body46_e23643_d_n10, assign16210_body46_e23643_d_n11, assign16210_body46_e23643_d_n12, assign16210_body46_e23643_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body46_e23631: f64 = (-locals.var_vgp);
        let assign16210_body46_e23633: f64 = (assign16210_body46_e23631 + locals.var_phi_sl_soi);
        let assign16210_body46_e23636: f64 = (locals.var_fac1 * locals.var_fsl2);
        let assign16210_body46_e23637: f64 = (assign16210_body46_e23633 + assign16210_body46_e23636);
        let assign16210_body46_e23640: f64 = (locals.var_c_fox_inv * locals.var_qhs);
        let assign16210_body46_e23641: f64 = (assign16210_body46_e23637 - assign16210_body46_e23640);
        (assign16210_body46_e23641, ((((-locals.var_vgp_dn0) + locals.var_phi_sl_soi_dn0) + ((locals.var_fac1_dn0 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn0))) - ((locals.var_c_fox_inv_dn0 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn0))), ((((-locals.var_vgp_dn2) + locals.var_phi_sl_soi_dn2) + ((locals.var_fac1_dn2 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn2))) - ((locals.var_c_fox_inv_dn2 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn2))), ((((-locals.var_vgp_dn6) + locals.var_phi_sl_soi_dn6) + ((locals.var_fac1_dn6 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn6))) - ((locals.var_c_fox_inv_dn6 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn6))), ((((-locals.var_vgp_dn7) + locals.var_phi_sl_soi_dn7) + ((locals.var_fac1_dn7 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn7))) - ((locals.var_c_fox_inv_dn7 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn7))), ((((-locals.var_vgp_dn10) + locals.var_phi_sl_soi_dn10) + ((locals.var_fac1_dn10 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn10))) - ((locals.var_c_fox_inv_dn10 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn10))), ((((-locals.var_vgp_dn11) + locals.var_phi_sl_soi_dn11) + ((locals.var_fac1_dn11 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn11))) - ((locals.var_c_fox_inv_dn11 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn11))), ((((-locals.var_vgp_dn12) + locals.var_phi_sl_soi_dn12) + ((locals.var_fac1_dn12 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn12))) - ((locals.var_c_fox_inv_dn12 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn12))), ((((-locals.var_vgp_dn17) + locals.var_phi_sl_soi_dn17) + ((locals.var_fac1_dn17 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn17))) - ((locals.var_c_fox_inv_dn17 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn17))),)
    } else {
        (locals.var_fsl, locals.var_fsl_dn0, locals.var_fsl_dn2, locals.var_fsl_dn6, locals.var_fsl_dn7, locals.var_fsl_dn10, locals.var_fsl_dn11, locals.var_fsl_dn12, locals.var_fsl_dn17,)
    }
};
            locals.var_fsl = assign16210_body46_e23643;
            locals.var_fsl_dn0 = assign16210_body46_e23643_d_n0;
            locals.var_fsl_dn2 = assign16210_body46_e23643_d_n2;
            locals.var_fsl_dn6 = assign16210_body46_e23643_d_n6;
            locals.var_fsl_dn7 = assign16210_body46_e23643_d_n7;
            locals.var_fsl_dn10 = assign16210_body46_e23643_d_n10;
            locals.var_fsl_dn11 = assign16210_body46_e23643_d_n11;
            locals.var_fsl_dn12 = assign16210_body46_e23643_d_n12;
            locals.var_fsl_dn17 = assign16210_body46_e23643_d_n17;
            locals.var_fsl_rv = 0.0;
            let (assign16210_body47_e23654, assign16210_body47_e23654_d_n0, assign16210_body47_e23654_d_n2, assign16210_body47_e23654_d_n6, assign16210_body47_e23654_d_n7, assign16210_body47_e23654_d_n10, assign16210_body47_e23654_d_n11, assign16210_body47_e23654_d_n12, assign16210_body47_e23654_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body47_e23651: f64 = (locals.var_fac1 * locals.var_fsl2_dpsl);
        let assign16210_body47_e23652: f64 = (1.0 + assign16210_body47_e23651);
        (assign16210_body47_e23652, ((locals.var_fac1_dn0 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn0)), ((locals.var_fac1_dn2 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn2)), ((locals.var_fac1_dn6 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn6)), ((locals.var_fac1_dn7 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn7)), ((locals.var_fac1_dn10 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn10)), ((locals.var_fac1_dn11 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn11)), ((locals.var_fac1_dn12 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn12)), ((locals.var_fac1_dn17 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn17)),)
    } else {
        (locals.var_fsl_dpsl, locals.var_fsl_dpsl_dn0, locals.var_fsl_dpsl_dn2, locals.var_fsl_dpsl_dn6, locals.var_fsl_dpsl_dn7, locals.var_fsl_dpsl_dn10, locals.var_fsl_dpsl_dn11, locals.var_fsl_dpsl_dn12, locals.var_fsl_dpsl_dn17,)
    }
};
            locals.var_fsl_dpsl = assign16210_body47_e23654;
            locals.var_fsl_dpsl_dn0 = assign16210_body47_e23654_d_n0;
            locals.var_fsl_dpsl_dn2 = assign16210_body47_e23654_d_n2;
            locals.var_fsl_dpsl_dn6 = assign16210_body47_e23654_d_n6;
            locals.var_fsl_dpsl_dn7 = assign16210_body47_e23654_d_n7;
            locals.var_fsl_dpsl_dn10 = assign16210_body47_e23654_d_n10;
            locals.var_fsl_dpsl_dn11 = assign16210_body47_e23654_d_n11;
            locals.var_fsl_dpsl_dn12 = assign16210_body47_e23654_d_n12;
            locals.var_fsl_dpsl_dn17 = assign16210_body47_e23654_d_n17;
            locals.var_fsl_dpsl_rv = 0.0;
            let assign16210_body48_e23661: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_lp_sl > 3.0)) { 1.0 } else { 0.0 };
            locals.var_guard488 = assign16210_body48_e23661;
            locals.var_guard488_rv = 0.0;
            let (assign16210_body49_e23672,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16210_body49_e23670: f64 = (locals.var_lp_sl_max + 1.0);
        (assign16210_body49_e23670,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign16210_body49_e23672;
            locals.var_lp_sl_rv = 0.0;
            let (assign16210_body50_e23685, assign16210_body50_e23685_d_n0, assign16210_body50_e23685_d_n2, assign16210_body50_e23685_d_n6, assign16210_body50_e23685_d_n7, assign16210_body50_e23685_d_n10, assign16210_body50_e23685_d_n11, assign16210_body50_e23685_d_n12, assign16210_body50_e23685_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16210_body50_e23681: f64 = (-locals.var_fsl);
        let assign16210_body50_e23683: f64 = (assign16210_body50_e23681 / locals.var_fsl_dpsl);
        (assign16210_body50_e23683, ((((-locals.var_fsl_dn0) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn0)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn2) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn2)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn6) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn6)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn7) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn7)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn10) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn10)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn11) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn11)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn12) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn12)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn17) * locals.var_fsl_dpsl) - (assign16210_body50_e23681 * locals.var_fsl_dpsl_dn17)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn12, locals.var_dpsl_dn17,)
    }
};
            locals.var_dpsl = assign16210_body50_e23685;
            locals.var_dpsl_dn0 = assign16210_body50_e23685_d_n0;
            locals.var_dpsl_dn2 = assign16210_body50_e23685_d_n2;
            locals.var_dpsl_dn6 = assign16210_body50_e23685_d_n6;
            locals.var_dpsl_dn7 = assign16210_body50_e23685_d_n7;
            locals.var_dpsl_dn10 = assign16210_body50_e23685_d_n10;
            locals.var_dpsl_dn11 = assign16210_body50_e23685_d_n11;
            locals.var_dpsl_dn12 = assign16210_body50_e23685_d_n12;
            locals.var_dpsl_dn17 = assign16210_body50_e23685_d_n17;
            locals.var_dpsl_rv = 0.0;
            let (assign16210_body51_e23708, assign16210_body51_e23708_d_n0, assign16210_body51_e23708_d_n2, assign16210_body51_e23708_d_n6, assign16210_body51_e23708_d_n7, assign16210_body51_e23708_d_n10, assign16210_body51_e23708_d_n11, assign16210_body51_e23708_d_n12, assign16210_body51_e23708_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16210_body51_e23695: f64 = (0.5 * 0.1);
        let assign16210_body51_e23699: f64 = (locals.var_phi_sl_soi).abs();
        let (assign16210_body51_e23704, assign16210_body51_e23704_d_n0, assign16210_body51_e23704_d_n2, assign16210_body51_e23704_d_n6, assign16210_body51_e23704_d_n7, assign16210_body51_e23704_d_n10, assign16210_body51_e23704_d_n11, assign16210_body51_e23704_d_n12, assign16210_body51_e23704_d_n17,) = {
            if (1.0 >= assign16210_body51_e23699) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign16210_body51_e23703: f64 = (locals.var_phi_sl_soi).abs();
                (assign16210_body51_e23703, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn0 } else { (-locals.var_phi_sl_soi_dn0) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn2 } else { (-locals.var_phi_sl_soi_dn2) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn6 } else { (-locals.var_phi_sl_soi_dn6) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn7 } else { (-locals.var_phi_sl_soi_dn7) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn10 } else { (-locals.var_phi_sl_soi_dn10) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn11 } else { (-locals.var_phi_sl_soi_dn11) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn12 } else { (-locals.var_phi_sl_soi_dn12) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn17 } else { (-locals.var_phi_sl_soi_dn17) },)
            }
        };
        let assign16210_body51_e23705: f64 = (1.0 + assign16210_body51_e23704);
        let assign16210_body51_e23706: f64 = (assign16210_body51_e23695 * assign16210_body51_e23705);
        (assign16210_body51_e23706, (assign16210_body51_e23695 * assign16210_body51_e23704_d_n0), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n2), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n6), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n7), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n10), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n11), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n12), (assign16210_body51_e23695 * assign16210_body51_e23704_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign16210_body51_e23708;
            locals.var_dplim_dn0 = assign16210_body51_e23708_d_n0;
            locals.var_dplim_dn2 = assign16210_body51_e23708_d_n2;
            locals.var_dplim_dn6 = assign16210_body51_e23708_d_n6;
            locals.var_dplim_dn7 = assign16210_body51_e23708_d_n7;
            locals.var_dplim_dn10 = assign16210_body51_e23708_d_n10;
            locals.var_dplim_dn11 = assign16210_body51_e23708_d_n11;
            locals.var_dplim_dn12 = assign16210_body51_e23708_d_n12;
            locals.var_dplim_dn17 = assign16210_body51_e23708_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign16210_body52_e23710: f64 = (locals.var_dpsl).abs();
            let assign16210_body52_e23712: f64 = if assign16210_body52_e23710 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard489 = assign16210_body52_e23712;
            locals.var_guard489_rv = 0.0;
            let (assign16210_body53_e23732, assign16210_body53_e23732_d_n0, assign16210_body53_e23732_d_n2, assign16210_body53_e23732_d_n6, assign16210_body53_e23732_d_n7, assign16210_body53_e23732_d_n10, assign16210_body53_e23732_d_n11, assign16210_body53_e23732_d_n12, assign16210_body53_e23732_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard489 != 0.0)) {
        let (assign16210_body53_e23729,) = {
            if (locals.var_dpsl >= 0.0) {
                (1.0,)
            } else {
                let assign16210_body53_e23728: f64 = (-1.0);
                (assign16210_body53_e23728,)
            }
        };
        let assign16210_body53_e23730: f64 = (locals.var_dplim * assign16210_body53_e23729);
        (assign16210_body53_e23730, (locals.var_dplim_dn0 * assign16210_body53_e23729), (locals.var_dplim_dn2 * assign16210_body53_e23729), (locals.var_dplim_dn6 * assign16210_body53_e23729), (locals.var_dplim_dn7 * assign16210_body53_e23729), (locals.var_dplim_dn10 * assign16210_body53_e23729), (locals.var_dplim_dn11 * assign16210_body53_e23729), (locals.var_dplim_dn12 * assign16210_body53_e23729), (locals.var_dplim_dn17 * assign16210_body53_e23729),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn12, locals.var_dpsl_dn17,)
    }
};
            locals.var_dpsl = assign16210_body53_e23732;
            locals.var_dpsl_dn0 = assign16210_body53_e23732_d_n0;
            locals.var_dpsl_dn2 = assign16210_body53_e23732_d_n2;
            locals.var_dpsl_dn6 = assign16210_body53_e23732_d_n6;
            locals.var_dpsl_dn7 = assign16210_body53_e23732_d_n7;
            locals.var_dpsl_dn10 = assign16210_body53_e23732_d_n10;
            locals.var_dpsl_dn11 = assign16210_body53_e23732_d_n11;
            locals.var_dpsl_dn12 = assign16210_body53_e23732_d_n12;
            locals.var_dpsl_dn17 = assign16210_body53_e23732_d_n17;
            locals.var_dpsl_rv = 0.0;
            let (assign16210_body54_e23744, assign16210_body54_e23744_d_n0, assign16210_body54_e23744_d_n2, assign16210_body54_e23744_d_n6, assign16210_body54_e23744_d_n7, assign16210_body54_e23744_d_n10, assign16210_body54_e23744_d_n11, assign16210_body54_e23744_d_n12, assign16210_body54_e23744_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16210_body54_e23742: f64 = (locals.var_phi_sl_soi + locals.var_dpsl);
        (assign16210_body54_e23742, (locals.var_phi_sl_soi_dn0 + locals.var_dpsl_dn0), (locals.var_phi_sl_soi_dn2 + locals.var_dpsl_dn2), (locals.var_phi_sl_soi_dn6 + locals.var_dpsl_dn6), (locals.var_phi_sl_soi_dn7 + locals.var_dpsl_dn7), (locals.var_phi_sl_soi_dn10 + locals.var_dpsl_dn10), (locals.var_phi_sl_soi_dn11 + locals.var_dpsl_dn11), (locals.var_phi_sl_soi_dn12 + locals.var_dpsl_dn12), (locals.var_phi_sl_soi_dn17 + locals.var_dpsl_dn17),)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
            locals.var_phi_sl_soi = assign16210_body54_e23744;
            locals.var_phi_sl_soi_dn0 = assign16210_body54_e23744_d_n0;
            locals.var_phi_sl_soi_dn2 = assign16210_body54_e23744_d_n2;
            locals.var_phi_sl_soi_dn6 = assign16210_body54_e23744_d_n6;
            locals.var_phi_sl_soi_dn7 = assign16210_body54_e23744_d_n7;
            locals.var_phi_sl_soi_dn10 = assign16210_body54_e23744_d_n10;
            locals.var_phi_sl_soi_dn11 = assign16210_body54_e23744_d_n11;
            locals.var_phi_sl_soi_dn12 = assign16210_body54_e23744_d_n12;
            locals.var_phi_sl_soi_dn17 = assign16210_body54_e23744_d_n17;
            locals.var_phi_sl_soi_rv = 0.0;
            let assign16210_body55_e23746: f64 = (locals.var_dpsl).abs();
            let assign16210_body55_e23750: f64 = (locals.var_fsl).abs();
            let assign16210_body55_e23753: f64 = if ((assign16210_body55_e23746 <= 5e-12) && (assign16210_body55_e23750 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard490 = assign16210_body55_e23753;
            locals.var_guard490_rv = 0.0;
            let (assign16210_body56_e23765,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard488 == 0.0)) && (locals.var_guard490 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign16210_body56_e23765;
            locals.var_flg_conv_rv = 0.0;
            let (assign16210_body57_e23774,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16210_body57_e23772: f64 = (locals.var_lp_sl + 1.0);
        (assign16210_body57_e23772,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign16210_body57_e23774;
            locals.var_lp_sl_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_58(
        locals: &mut StampLocals,
    ) {
        let (assign16220_e23783,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16220_e23781: f64 = (locals.var_lp_sl - 1.0);
        (assign16220_e23781,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign16220_e23783;
        locals.var_lp_sl_rv = 0.0;

        let (assign16230_e23790, assign16230_e23790_d_n0, assign16230_e23790_d_n2, assign16230_e23790_d_n6, assign16230_e23790_d_n7, assign16230_e23790_d_n10, assign16230_e23790_d_n11, assign16230_e23790_d_n12, assign16230_e23790_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    } else {
        (locals.var_q_depsl, locals.var_q_depsl_dn0, locals.var_q_depsl_dn2, locals.var_q_depsl_dn6, locals.var_q_depsl_dn7, locals.var_q_depsl_dn10, locals.var_q_depsl_dn11, locals.var_q_depsl_dn12, locals.var_q_depsl_dn17,)
    }
};
        locals.var_q_depsl = assign16230_e23790;
        locals.var_q_depsl_dn0 = assign16230_e23790_d_n0;
        locals.var_q_depsl_dn2 = assign16230_e23790_d_n2;
        locals.var_q_depsl_dn6 = assign16230_e23790_d_n6;
        locals.var_q_depsl_dn7 = assign16230_e23790_d_n7;
        locals.var_q_depsl_dn10 = assign16230_e23790_d_n10;
        locals.var_q_depsl_dn11 = assign16230_e23790_d_n11;
        locals.var_q_depsl_dn12 = assign16230_e23790_d_n12;
        locals.var_q_depsl_dn17 = assign16230_e23790_d_n17;
        locals.var_q_depsl_rv = 0.0;

        let (assign16240_e23797, assign16240_e23797_d_n0, assign16240_e23797_d_n2, assign16240_e23797_d_n6, assign16240_e23797_d_n7, assign16240_e23797_d_n10, assign16240_e23797_d_n11, assign16240_e23797_d_n12, assign16240_e23797_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_q_depsl, locals.var_q_depsl_dn0, locals.var_q_depsl_dn2, locals.var_q_depsl_dn6, locals.var_q_depsl_dn7, locals.var_q_depsl_dn10, locals.var_q_depsl_dn11, locals.var_q_depsl_dn12, locals.var_q_depsl_dn17,)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
        locals.var_q_depl = assign16240_e23797;
        locals.var_q_depl_dn0 = assign16240_e23797_d_n0;
        locals.var_q_depl_dn2 = assign16240_e23797_d_n2;
        locals.var_q_depl_dn6 = assign16240_e23797_d_n6;
        locals.var_q_depl_dn7 = assign16240_e23797_d_n7;
        locals.var_q_depl_dn10 = assign16240_e23797_d_n10;
        locals.var_q_depl_dn11 = assign16240_e23797_d_n11;
        locals.var_q_depl_dn12 = assign16240_e23797_d_n12;
        locals.var_q_depl_dn17 = assign16240_e23797_d_n17;
        locals.var_q_depl_rv = 0.0;

        let (assign16250_e23804, assign16250_e23804_d_n0, assign16250_e23804_d_n2, assign16250_e23804_d_n6, assign16250_e23804_d_n7, assign16250_e23804_d_n10, assign16250_e23804_d_n11, assign16250_e23804_d_n12, assign16250_e23804_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16250_e23804;
        locals.var_psl_dn0 = assign16250_e23804_d_n0;
        locals.var_psl_dn2 = assign16250_e23804_d_n2;
        locals.var_psl_dn6 = assign16250_e23804_d_n6;
        locals.var_psl_dn7 = assign16250_e23804_d_n7;
        locals.var_psl_dn10 = assign16250_e23804_d_n10;
        locals.var_psl_dn11 = assign16250_e23804_d_n11;
        locals.var_psl_dn12 = assign16250_e23804_d_n12;
        locals.var_psl_dn17 = assign16250_e23804_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16270_e23820, assign16270_e23820_d_n0, assign16270_e23820_d_n2, assign16270_e23820_d_n6, assign16270_e23820_d_n7, assign16270_e23820_d_n10, assign16270_e23820_d_n11, assign16270_e23820_d_n12, assign16270_e23820_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16270_e23818: f64 = (locals.var_q_depsl / locals.var_cnst0soi);
        (assign16270_e23818, (((locals.var_q_depsl_dn0 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn0)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn2 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn2)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn6 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn6)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn7 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn7)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn10 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn10)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn11 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn11)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn12 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn12)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn17 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn17)) / (locals.var_cnst0soi * locals.var_cnst0soi)),)
    } else {
        (locals.var_q_depsl_soi_o_cnst0soi, locals.var_q_depsl_soi_o_cnst0soi_dn0, locals.var_q_depsl_soi_o_cnst0soi_dn2, locals.var_q_depsl_soi_o_cnst0soi_dn6, locals.var_q_depsl_soi_o_cnst0soi_dn7, locals.var_q_depsl_soi_o_cnst0soi_dn10, locals.var_q_depsl_soi_o_cnst0soi_dn11, locals.var_q_depsl_soi_o_cnst0soi_dn12, locals.var_q_depsl_soi_o_cnst0soi_dn17,)
    }
};
        locals.var_q_depsl_soi_o_cnst0soi = assign16270_e23820;
        locals.var_q_depsl_soi_o_cnst0soi_dn0 = assign16270_e23820_d_n0;
        locals.var_q_depsl_soi_o_cnst0soi_dn2 = assign16270_e23820_d_n2;
        locals.var_q_depsl_soi_o_cnst0soi_dn6 = assign16270_e23820_d_n6;
        locals.var_q_depsl_soi_o_cnst0soi_dn7 = assign16270_e23820_d_n7;
        locals.var_q_depsl_soi_o_cnst0soi_dn10 = assign16270_e23820_d_n10;
        locals.var_q_depsl_soi_o_cnst0soi_dn11 = assign16270_e23820_d_n11;
        locals.var_q_depsl_soi_o_cnst0soi_dn12 = assign16270_e23820_d_n12;
        locals.var_q_depsl_soi_o_cnst0soi_dn17 = assign16270_e23820_d_n17;
        locals.var_q_depsl_soi_o_cnst0soi_rv = 0.0;

        let (assign16280_e23831, assign16280_e23831_d_n0, assign16280_e23831_d_n2, assign16280_e23831_d_n6, assign16280_e23831_d_n7, assign16280_e23831_d_n10, assign16280_e23831_d_n11, assign16280_e23831_d_n12, assign16280_e23831_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16280_e23828: f64 = (10.0 * 2.220446049250313e-16);
        let assign16280_e23829: f64 = (locals.var_q_depsl_soi_o_cnst0soi + assign16280_e23828);
        (assign16280_e23829, locals.var_q_depsl_soi_o_cnst0soi_dn0, locals.var_q_depsl_soi_o_cnst0soi_dn2, locals.var_q_depsl_soi_o_cnst0soi_dn6, locals.var_q_depsl_soi_o_cnst0soi_dn7, locals.var_q_depsl_soi_o_cnst0soi_dn10, locals.var_q_depsl_soi_o_cnst0soi_dn11, locals.var_q_depsl_soi_o_cnst0soi_dn12, locals.var_q_depsl_soi_o_cnst0soi_dn17,)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn12, locals.var_xilp12_dn17,)
    }
};
        locals.var_xilp12 = assign16280_e23831;
        locals.var_xilp12_dn0 = assign16280_e23831_d_n0;
        locals.var_xilp12_dn2 = assign16280_e23831_d_n2;
        locals.var_xilp12_dn6 = assign16280_e23831_d_n6;
        locals.var_xilp12_dn7 = assign16280_e23831_d_n7;
        locals.var_xilp12_dn10 = assign16280_e23831_d_n10;
        locals.var_xilp12_dn11 = assign16280_e23831_d_n11;
        locals.var_xilp12_dn12 = assign16280_e23831_d_n12;
        locals.var_xilp12_dn17 = assign16280_e23831_d_n17;
        locals.var_xilp12_rv = 0.0;

        let (assign16290_e23842, assign16290_e23842_d_n0, assign16290_e23842_d_n2, assign16290_e23842_d_n6, assign16290_e23842_d_n7, assign16290_e23842_d_n10, assign16290_e23842_d_n11, assign16290_e23842_d_n12, assign16290_e23842_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16290_e23839: f64 = (locals.var_fsl2 + locals.var_xilp12);
        let assign16290_e23840: f64 = (1.0 / assign16290_e23839);
        (assign16290_e23840, (-((locals.var_fsl2_dn0 + locals.var_xilp12_dn0) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn2 + locals.var_xilp12_dn2) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn6 + locals.var_xilp12_dn6) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn7 + locals.var_xilp12_dn7) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn10 + locals.var_xilp12_dn10) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn11 + locals.var_xilp12_dn11) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn12 + locals.var_xilp12_dn12) / (assign16290_e23839 * assign16290_e23839))), (-((locals.var_fsl2_dn17 + locals.var_xilp12_dn17) / (assign16290_e23839 * assign16290_e23839))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16290_e23842;
        locals.var_t1_dn0 = assign16290_e23842_d_n0;
        locals.var_t1_dn2 = assign16290_e23842_d_n2;
        locals.var_t1_dn6 = assign16290_e23842_d_n6;
        locals.var_t1_dn7 = assign16290_e23842_d_n7;
        locals.var_t1_dn10 = assign16290_e23842_d_n10;
        locals.var_t1_dn11 = assign16290_e23842_d_n11;
        locals.var_t1_dn12 = assign16290_e23842_d_n12;
        locals.var_t1_dn17 = assign16290_e23842_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16300_e23853, assign16300_e23853_d_n0, assign16300_e23853_d_n2, assign16300_e23853_d_n6, assign16300_e23853_d_n7, assign16300_e23853_d_n10, assign16300_e23853_d_n11, assign16300_e23853_d_n12, assign16300_e23853_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16300_e23849: f64 = (locals.var_cnst0soi * locals.var_fsl1);
        let assign16300_e23851: f64 = (assign16300_e23849 * locals.var_t1);
        (assign16300_e23851, ((((locals.var_cnst0soi_dn0 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn0)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn2)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn6)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn7)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn10)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn11)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn12)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn17)) * locals.var_t1) + (assign16300_e23849 * locals.var_t1_dn17)),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
        locals.var_q_nl = assign16300_e23853;
        locals.var_q_nl_dn0 = assign16300_e23853_d_n0;
        locals.var_q_nl_dn2 = assign16300_e23853_d_n2;
        locals.var_q_nl_dn6 = assign16300_e23853_d_n6;
        locals.var_q_nl_dn7 = assign16300_e23853_d_n7;
        locals.var_q_nl_dn10 = assign16300_e23853_d_n10;
        locals.var_q_nl_dn11 = assign16300_e23853_d_n11;
        locals.var_q_nl_dn12 = assign16300_e23853_d_n12;
        locals.var_q_nl_dn17 = assign16300_e23853_d_n17;
        locals.var_q_nl_rv = 0.0;

        let (assign16310_e23861, assign16310_e23861_d_n0, assign16310_e23861_d_n2, assign16310_e23861_d_n6, assign16310_e23861_d_n7, assign16310_e23861_d_n10, assign16310_e23861_d_n11, assign16310_e23861_d_n12, assign16310_e23861_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16310_e23859: f64 = (-locals.var_q_nl);
        (assign16310_e23859, (-locals.var_q_nl_dn0), (-locals.var_q_nl_dn2), (-locals.var_q_nl_dn6), (-locals.var_q_nl_dn7), (-locals.var_q_nl_dn10), (-locals.var_q_nl_dn11), (-locals.var_q_nl_dn12), (-locals.var_q_nl_dn17),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
        locals.var_q_nl = assign16310_e23861;
        locals.var_q_nl_dn0 = assign16310_e23861_d_n0;
        locals.var_q_nl_dn2 = assign16310_e23861_d_n2;
        locals.var_q_nl_dn6 = assign16310_e23861_d_n6;
        locals.var_q_nl_dn7 = assign16310_e23861_d_n7;
        locals.var_q_nl_dn10 = assign16310_e23861_d_n10;
        locals.var_q_nl_dn11 = assign16310_e23861_d_n11;
        locals.var_q_nl_dn12 = assign16310_e23861_d_n12;
        locals.var_q_nl_dn17 = assign16310_e23861_d_n17;
        locals.var_q_nl_rv = 0.0;

        let (assign16320_e23870, assign16320_e23870_d_n0, assign16320_e23870_d_n2, assign16320_e23870_d_n6, assign16320_e23870_d_n7, assign16320_e23870_d_n10, assign16320_e23870_d_n11, assign16320_e23870_d_n12, assign16320_e23870_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16320_e23868: f64 = (locals.var_psl - locals.var_ps0);
        (assign16320_e23868, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn12 - locals.var_ps0_dn12), (locals.var_psl_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16320_e23870;
        locals.var_pds_dn0 = assign16320_e23870_d_n0;
        locals.var_pds_dn2 = assign16320_e23870_d_n2;
        locals.var_pds_dn6 = assign16320_e23870_d_n6;
        locals.var_pds_dn7 = assign16320_e23870_d_n7;
        locals.var_pds_dn10 = assign16320_e23870_d_n10;
        locals.var_pds_dn11 = assign16320_e23870_d_n11;
        locals.var_pds_dn12 = assign16320_e23870_d_n12;
        locals.var_pds_dn17 = assign16320_e23870_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign16330_e23877, assign16330_e23877_d_n0, assign16330_e23877_d_n2, assign16330_e23877_d_n6, assign16330_e23877_d_n7, assign16330_e23877_d_n10, assign16330_e23877_d_n11, assign16330_e23877_d_n12, assign16330_e23877_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign16330_e23877;
        locals.var_vds_dn0 = assign16330_e23877_d_n0;
        locals.var_vds_dn2 = assign16330_e23877_d_n2;
        locals.var_vds_dn6 = assign16330_e23877_d_n6;
        locals.var_vds_dn7 = assign16330_e23877_d_n7;
        locals.var_vds_dn10 = assign16330_e23877_d_n10;
        locals.var_vds_dn11 = assign16330_e23877_d_n11;
        locals.var_vds_dn12 = assign16330_e23877_d_n12;
        locals.var_vds_dn17 = assign16330_e23877_d_n17;
        locals.var_vds_rv = 0.0;

        let (assign16340_e23886, assign16340_e23886_d_n0, assign16340_e23886_d_n2, assign16340_e23886_d_n6, assign16340_e23886_d_n7, assign16340_e23886_d_n10, assign16340_e23886_d_n11, assign16340_e23886_d_n12, assign16340_e23886_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16340_e23884: f64 = (locals.var_beta / locals.var_xi0);
        (assign16340_e23884, (-((locals.var_beta * locals.var_xi0_dn0) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn2) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn6) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn7) / (locals.var_xi0 * locals.var_xi0))), (((locals.var_beta_dn10 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn10)) / (locals.var_xi0 * locals.var_xi0)), (-((locals.var_beta * locals.var_xi0_dn11) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn12) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn17) / (locals.var_xi0 * locals.var_xi0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16340_e23886;
        locals.var_t1_dn0 = assign16340_e23886_d_n0;
        locals.var_t1_dn2 = assign16340_e23886_d_n2;
        locals.var_t1_dn6 = assign16340_e23886_d_n6;
        locals.var_t1_dn7 = assign16340_e23886_d_n7;
        locals.var_t1_dn10 = assign16340_e23886_d_n10;
        locals.var_t1_dn11 = assign16340_e23886_d_n11;
        locals.var_t1_dn12 = assign16340_e23886_d_n12;
        locals.var_t1_dn17 = assign16340_e23886_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16350_e23895, assign16350_e23895_d_n0, assign16350_e23895_d_n2, assign16350_e23895_d_n6, assign16350_e23895_d_n7, assign16350_e23895_d_n10, assign16350_e23895_d_n11, assign16350_e23895_d_n12, assign16350_e23895_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16350_e23893: f64 = (locals.var_t1 * locals.var_pds);
        (assign16350_e23893, ((locals.var_t1_dn0 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn0)), ((locals.var_t1_dn2 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn2)), ((locals.var_t1_dn6 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn6)), ((locals.var_t1_dn7 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn7)), ((locals.var_t1_dn10 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn10)), ((locals.var_t1_dn11 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn11)), ((locals.var_t1_dn12 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn12)), ((locals.var_t1_dn17 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn17)),)
    } else {
        (locals.var_eta, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12, locals.var_eta_dn17,)
    }
};
        locals.var_eta = assign16350_e23895;
        locals.var_eta_dn0 = assign16350_e23895_d_n0;
        locals.var_eta_dn2 = assign16350_e23895_d_n2;
        locals.var_eta_dn6 = assign16350_e23895_d_n6;
        locals.var_eta_dn7 = assign16350_e23895_d_n7;
        locals.var_eta_dn10 = assign16350_e23895_d_n10;
        locals.var_eta_dn11 = assign16350_e23895_d_n11;
        locals.var_eta_dn12 = assign16350_e23895_d_n12;
        locals.var_eta_dn17 = assign16350_e23895_d_n17;
        locals.var_eta_rv = 0.0;

        let (assign16360_e23904, assign16360_e23904_d_n0, assign16360_e23904_d_n2, assign16360_e23904_d_n6, assign16360_e23904_d_n7, assign16360_e23904_d_n10, assign16360_e23904_d_n11, assign16360_e23904_d_n12, assign16360_e23904_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16360_e23902: f64 = (locals.var_eta + 1.0);
        (assign16360_e23902, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12, locals.var_eta_dn17,)
    } else {
        (locals.var_eta1, locals.var_eta1_dn0, locals.var_eta1_dn2, locals.var_eta1_dn6, locals.var_eta1_dn7, locals.var_eta1_dn10, locals.var_eta1_dn11, locals.var_eta1_dn12, locals.var_eta1_dn17,)
    }
};
        locals.var_eta1 = assign16360_e23904;
        locals.var_eta1_dn0 = assign16360_e23904_d_n0;
        locals.var_eta1_dn2 = assign16360_e23904_d_n2;
        locals.var_eta1_dn6 = assign16360_e23904_d_n6;
        locals.var_eta1_dn7 = assign16360_e23904_d_n7;
        locals.var_eta1_dn10 = assign16360_e23904_d_n10;
        locals.var_eta1_dn11 = assign16360_e23904_d_n11;
        locals.var_eta1_dn12 = assign16360_e23904_d_n12;
        locals.var_eta1_dn17 = assign16360_e23904_d_n17;
        locals.var_eta1_rv = 0.0;

        let (assign16370_e23912, assign16370_e23912_d_n0, assign16370_e23912_d_n2, assign16370_e23912_d_n6, assign16370_e23912_d_n7, assign16370_e23912_d_n10, assign16370_e23912_d_n11, assign16370_e23912_d_n12, assign16370_e23912_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16370_e23910: f64 = (locals.var_eta1).sqrt();
        (assign16370_e23910, (locals.var_eta1_dn0 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn2 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn6 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn7 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn10 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn11 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn12 / (2.0 * assign16370_e23910)), (locals.var_eta1_dn17 / (2.0 * assign16370_e23910)),)
    } else {
        (locals.var_eta1p12, locals.var_eta1p12_dn0, locals.var_eta1p12_dn2, locals.var_eta1p12_dn6, locals.var_eta1p12_dn7, locals.var_eta1p12_dn10, locals.var_eta1p12_dn11, locals.var_eta1p12_dn12, locals.var_eta1p12_dn17,)
    }
};
        locals.var_eta1p12 = assign16370_e23912;
        locals.var_eta1p12_dn0 = assign16370_e23912_d_n0;
        locals.var_eta1p12_dn2 = assign16370_e23912_d_n2;
        locals.var_eta1p12_dn6 = assign16370_e23912_d_n6;
        locals.var_eta1p12_dn7 = assign16370_e23912_d_n7;
        locals.var_eta1p12_dn10 = assign16370_e23912_d_n10;
        locals.var_eta1p12_dn11 = assign16370_e23912_d_n11;
        locals.var_eta1p12_dn12 = assign16370_e23912_d_n12;
        locals.var_eta1p12_dn17 = assign16370_e23912_d_n17;
        locals.var_eta1p12_rv = 0.0;

        let (assign16380_e23923, assign16380_e23923_d_n0, assign16380_e23923_d_n2, assign16380_e23923_d_n6, assign16380_e23923_d_n7, assign16380_e23923_d_n10, assign16380_e23923_d_n11, assign16380_e23923_d_n12, assign16380_e23923_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16380_e23920: f64 = (locals.var_eta1p12 + 1.0);
        let assign16380_e23921: f64 = (1.0 / assign16380_e23920);
        (assign16380_e23921, (-(locals.var_eta1p12_dn0 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn2 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn6 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn7 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn10 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn11 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn12 / (assign16380_e23920 * assign16380_e23920))), (-(locals.var_eta1p12_dn17 / (assign16380_e23920 * assign16380_e23920))),)
    } else {
        (locals.var_zeta12, locals.var_zeta12_dn0, locals.var_zeta12_dn2, locals.var_zeta12_dn6, locals.var_zeta12_dn7, locals.var_zeta12_dn10, locals.var_zeta12_dn11, locals.var_zeta12_dn12, locals.var_zeta12_dn17,)
    }
};
        locals.var_zeta12 = assign16380_e23923;
        locals.var_zeta12_dn0 = assign16380_e23923_d_n0;
        locals.var_zeta12_dn2 = assign16380_e23923_d_n2;
        locals.var_zeta12_dn6 = assign16380_e23923_d_n6;
        locals.var_zeta12_dn7 = assign16380_e23923_d_n7;
        locals.var_zeta12_dn10 = assign16380_e23923_d_n10;
        locals.var_zeta12_dn11 = assign16380_e23923_d_n11;
        locals.var_zeta12_dn12 = assign16380_e23923_d_n12;
        locals.var_zeta12_dn17 = assign16380_e23923_d_n17;
        locals.var_zeta12_rv = 0.0;

        let (assign16390_e23932, assign16390_e23932_d_n0, assign16390_e23932_d_n2, assign16390_e23932_d_n6, assign16390_e23932_d_n7, assign16390_e23932_d_n10, assign16390_e23932_d_n11, assign16390_e23932_d_n12, assign16390_e23932_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16390_e23930: f64 = (locals.var_zeta12 / locals.var_xi0p12);
        (assign16390_e23930, (((locals.var_zeta12_dn0 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn0)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn2 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn2)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn6 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn6)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn7 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn7)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn10 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn10)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn11 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn11)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn12 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn12)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn17 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn17)) / (locals.var_xi0p12 * locals.var_xi0p12)),)
    } else {
        (locals.var_f00, locals.var_f00_dn0, locals.var_f00_dn2, locals.var_f00_dn6, locals.var_f00_dn7, locals.var_f00_dn10, locals.var_f00_dn11, locals.var_f00_dn12, locals.var_f00_dn17,)
    }
};
        locals.var_f00 = assign16390_e23932;
        locals.var_f00_dn0 = assign16390_e23932_d_n0;
        locals.var_f00_dn2 = assign16390_e23932_d_n2;
        locals.var_f00_dn6 = assign16390_e23932_d_n6;
        locals.var_f00_dn7 = assign16390_e23932_d_n7;
        locals.var_f00_dn10 = assign16390_e23932_d_n10;
        locals.var_f00_dn11 = assign16390_e23932_d_n11;
        locals.var_f00_dn12 = assign16390_e23932_d_n12;
        locals.var_f00_dn17 = assign16390_e23932_d_n17;
        locals.var_f00_rv = 0.0;

        let (assign16400_e23943, assign16400_e23943_d_n0, assign16400_e23943_d_n2, assign16400_e23943_d_n6, assign16400_e23943_d_n7, assign16400_e23943_d_n10, assign16400_e23943_d_n11, assign16400_e23943_d_n12, assign16400_e23943_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16400_e23940: f64 = (locals.var_q_deps0_soi_o_cnst0soi + locals.var_q_depsl_soi_o_cnst0soi);
        let assign16400_e23941: f64 = (0.5 * assign16400_e23940);
        (assign16400_e23941, (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn0 + locals.var_q_depsl_soi_o_cnst0soi_dn0)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn2 + locals.var_q_depsl_soi_o_cnst0soi_dn2)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn6 + locals.var_q_depsl_soi_o_cnst0soi_dn6)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn7 + locals.var_q_depsl_soi_o_cnst0soi_dn7)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn10 + locals.var_q_depsl_soi_o_cnst0soi_dn10)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn11 + locals.var_q_depsl_soi_o_cnst0soi_dn11)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn12 + locals.var_q_depsl_soi_o_cnst0soi_dn12)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn17 + locals.var_q_depsl_soi_o_cnst0soi_dn17)),)
    } else {
        (locals.var_f10, locals.var_f10_dn0, locals.var_f10_dn2, locals.var_f10_dn6, locals.var_f10_dn7, locals.var_f10_dn10, locals.var_f10_dn11, locals.var_f10_dn12, locals.var_f10_dn17,)
    }
};
        locals.var_f10 = assign16400_e23943;
        locals.var_f10_dn0 = assign16400_e23943_d_n0;
        locals.var_f10_dn2 = assign16400_e23943_d_n2;
        locals.var_f10_dn6 = assign16400_e23943_d_n6;
        locals.var_f10_dn7 = assign16400_e23943_d_n7;
        locals.var_f10_dn10 = assign16400_e23943_d_n10;
        locals.var_f10_dn11 = assign16400_e23943_d_n11;
        locals.var_f10_dn12 = assign16400_e23943_d_n12;
        locals.var_f10_dn17 = assign16400_e23943_d_n17;
        locals.var_f10_rv = 0.0;

        let (assign16410_e23960, assign16410_e23960_d_n0, assign16410_e23960_d_n2, assign16410_e23960_d_n6, assign16410_e23960_d_n7, assign16410_e23960_d_n10, assign16410_e23960_d_n11, assign16410_e23960_d_n12, assign16410_e23960_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16410_e23950: f64 = (locals.var_vgp + locals.var_beta_inv);
        let assign16410_e23954: f64 = (2.0 * locals.var_ps0);
        let assign16410_e23956: f64 = (assign16410_e23954 + locals.var_pds);
        let assign16410_e23957: f64 = (0.5 * assign16410_e23956);
        let assign16410_e23958: f64 = (assign16410_e23950 - assign16410_e23957);
        (assign16410_e23958, (locals.var_vgp_dn0 - (0.5 * ((2.0 * locals.var_ps0_dn0) + locals.var_pds_dn0))), (locals.var_vgp_dn2 - (0.5 * ((2.0 * locals.var_ps0_dn2) + locals.var_pds_dn2))), (locals.var_vgp_dn6 - (0.5 * ((2.0 * locals.var_ps0_dn6) + locals.var_pds_dn6))), (locals.var_vgp_dn7 - (0.5 * ((2.0 * locals.var_ps0_dn7) + locals.var_pds_dn7))), ((locals.var_vgp_dn10 + locals.var_beta_inv_dn10) - (0.5 * ((2.0 * locals.var_ps0_dn10) + locals.var_pds_dn10))), (locals.var_vgp_dn11 - (0.5 * ((2.0 * locals.var_ps0_dn11) + locals.var_pds_dn11))), (locals.var_vgp_dn12 - (0.5 * ((2.0 * locals.var_ps0_dn12) + locals.var_pds_dn12))), (locals.var_vgp_dn17 - (0.5 * ((2.0 * locals.var_ps0_dn17) + locals.var_pds_dn17))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16410_e23960;
        locals.var_t1_dn0 = assign16410_e23960_d_n0;
        locals.var_t1_dn2 = assign16410_e23960_d_n2;
        locals.var_t1_dn6 = assign16410_e23960_d_n6;
        locals.var_t1_dn7 = assign16410_e23960_d_n7;
        locals.var_t1_dn10 = assign16410_e23960_d_n10;
        locals.var_t1_dn11 = assign16410_e23960_d_n11;
        locals.var_t1_dn12 = assign16410_e23960_d_n12;
        locals.var_t1_dn17 = assign16410_e23960_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16420_e23970, assign16420_e23970_d_n0, assign16420_e23970_d_n2, assign16420_e23970_d_n6, assign16420_e23970_d_n7, assign16420_e23970_d_n10, assign16420_e23970_d_n11, assign16420_e23970_d_n12, assign16420_e23970_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16420_e23966: f64 = (-locals.var_f10);
        let assign16420_e23968: f64 = (assign16420_e23966 + locals.var_f00);
        (assign16420_e23968, ((-locals.var_f10_dn0) + locals.var_f00_dn0), ((-locals.var_f10_dn2) + locals.var_f00_dn2), ((-locals.var_f10_dn6) + locals.var_f00_dn6), ((-locals.var_f10_dn7) + locals.var_f00_dn7), ((-locals.var_f10_dn10) + locals.var_f00_dn10), ((-locals.var_f10_dn11) + locals.var_f00_dn11), ((-locals.var_f10_dn12) + locals.var_f00_dn12), ((-locals.var_f10_dn17) + locals.var_f00_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign16420_e23970;
        locals.var_t2_dn0 = assign16420_e23970_d_n0;
        locals.var_t2_dn2 = assign16420_e23970_d_n2;
        locals.var_t2_dn6 = assign16420_e23970_d_n6;
        locals.var_t2_dn7 = assign16420_e23970_d_n7;
        locals.var_t2_dn10 = assign16420_e23970_d_n10;
        locals.var_t2_dn11 = assign16420_e23970_d_n11;
        locals.var_t2_dn12 = assign16420_e23970_d_n12;
        locals.var_t2_dn17 = assign16420_e23970_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign16430_e23979, assign16430_e23979_d_n0, assign16430_e23979_d_n2, assign16430_e23979_d_n6, assign16430_e23979_d_n7, assign16430_e23979_d_n10, assign16430_e23979_d_n11, assign16430_e23979_d_n12, assign16430_e23979_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16430_e23977: f64 = (locals.var_beta * locals.var_c_fox);
        (assign16430_e23977, (locals.var_beta * locals.var_c_fox_dn0), (locals.var_beta * locals.var_c_fox_dn2), (locals.var_beta * locals.var_c_fox_dn6), (locals.var_beta * locals.var_c_fox_dn7), ((locals.var_beta_dn10 * locals.var_c_fox) + (locals.var_beta * locals.var_c_fox_dn10)), (locals.var_beta * locals.var_c_fox_dn11), (locals.var_beta * locals.var_c_fox_dn12), (locals.var_beta * locals.var_c_fox_dn17),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign16430_e23979;
        locals.var_t3_dn0 = assign16430_e23979_d_n0;
        locals.var_t3_dn2 = assign16430_e23979_d_n2;
        locals.var_t3_dn6 = assign16430_e23979_d_n6;
        locals.var_t3_dn7 = assign16430_e23979_d_n7;
        locals.var_t3_dn10 = assign16430_e23979_d_n10;
        locals.var_t3_dn11 = assign16430_e23979_d_n11;
        locals.var_t3_dn12 = assign16430_e23979_d_n12;
        locals.var_t3_dn17 = assign16430_e23979_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign16440_e23988, assign16440_e23988_d_n0, assign16440_e23988_d_n2, assign16440_e23988_d_n6, assign16440_e23988_d_n7, assign16440_e23988_d_n10, assign16440_e23988_d_n11, assign16440_e23988_d_n12, assign16440_e23988_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16440_e23986: f64 = (locals.var_beta * locals.var_cnst0soi);
        (assign16440_e23986, (locals.var_beta * locals.var_cnst0soi_dn0), (locals.var_beta * locals.var_cnst0soi_dn2), (locals.var_beta * locals.var_cnst0soi_dn6), (locals.var_beta * locals.var_cnst0soi_dn7), ((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)), (locals.var_beta * locals.var_cnst0soi_dn11), (locals.var_beta * locals.var_cnst0soi_dn12), (locals.var_beta * locals.var_cnst0soi_dn17),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign16440_e23988;
        locals.var_t4_dn0 = assign16440_e23988_d_n0;
        locals.var_t4_dn2 = assign16440_e23988_d_n2;
        locals.var_t4_dn6 = assign16440_e23988_d_n6;
        locals.var_t4_dn7 = assign16440_e23988_d_n7;
        locals.var_t4_dn10 = assign16440_e23988_d_n10;
        locals.var_t4_dn11 = assign16440_e23988_d_n11;
        locals.var_t4_dn12 = assign16440_e23988_d_n12;
        locals.var_t4_dn17 = assign16440_e23988_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign16450_e24001, assign16450_e24001_d_n0, assign16450_e24001_d_n2, assign16450_e24001_d_n6, assign16450_e24001_d_n7, assign16450_e24001_d_n10, assign16450_e24001_d_n11, assign16450_e24001_d_n12, assign16450_e24001_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16450_e23995: f64 = (locals.var_t3 * locals.var_t1);
        let assign16450_e23998: f64 = (locals.var_t4 * locals.var_t2);
        let assign16450_e23999: f64 = (assign16450_e23995 + assign16450_e23998);
        (assign16450_e23999, (((locals.var_t3_dn0 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0))), (((locals.var_t3_dn2 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2))), (((locals.var_t3_dn6 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6))), (((locals.var_t3_dn7 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7))), (((locals.var_t3_dn10 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10))), (((locals.var_t3_dn11 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn11))), (((locals.var_t3_dn12 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn12)) + ((locals.var_t4_dn12 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn12))), (((locals.var_t3_dn17 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn17)) + ((locals.var_t4_dn17 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn17))),)
    } else {
        (locals.var_fdd, locals.var_fdd_dn0, locals.var_fdd_dn2, locals.var_fdd_dn6, locals.var_fdd_dn7, locals.var_fdd_dn10, locals.var_fdd_dn11, locals.var_fdd_dn12, locals.var_fdd_dn17,)
    }
};
        locals.var_fdd = assign16450_e24001;
        locals.var_fdd_dn0 = assign16450_e24001_d_n0;
        locals.var_fdd_dn2 = assign16450_e24001_d_n2;
        locals.var_fdd_dn6 = assign16450_e24001_d_n6;
        locals.var_fdd_dn7 = assign16450_e24001_d_n7;
        locals.var_fdd_dn10 = assign16450_e24001_d_n10;
        locals.var_fdd_dn11 = assign16450_e24001_d_n11;
        locals.var_fdd_dn12 = assign16450_e24001_d_n12;
        locals.var_fdd_dn17 = assign16450_e24001_d_n17;
        locals.var_fdd_rv = 0.0;

        let (assign16460_e24012, assign16460_e24012_d_n0, assign16460_e24012_d_n2, assign16460_e24012_d_n6, assign16460_e24012_d_n7, assign16460_e24012_d_n10, assign16460_e24012_d_n11, assign16460_e24012_d_n12, assign16460_e24012_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16460_e24008: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign16460_e24010: f64 = (assign16460_e24008 / 2.0);
        (assign16460_e24010, ((locals.var_q_depl_dn0 + locals.var_q_dep0_dn0) / 2.0), ((locals.var_q_depl_dn2 + locals.var_q_dep0_dn2) / 2.0), ((locals.var_q_depl_dn6 + locals.var_q_dep0_dn6) / 2.0), ((locals.var_q_depl_dn7 + locals.var_q_dep0_dn7) / 2.0), ((locals.var_q_depl_dn10 + locals.var_q_dep0_dn10) / 2.0), ((locals.var_q_depl_dn11 + locals.var_q_dep0_dn11) / 2.0), ((locals.var_q_depl_dn12 + locals.var_q_dep0_dn12) / 2.0), ((locals.var_q_depl_dn17 + locals.var_q_dep0_dn17) / 2.0),)
    } else {
        (locals.var_ab, locals.var_ab_dn0, locals.var_ab_dn2, locals.var_ab_dn6, locals.var_ab_dn7, locals.var_ab_dn10, locals.var_ab_dn11, locals.var_ab_dn12, locals.var_ab_dn17,)
    }
};
        locals.var_ab = assign16460_e24012;
        locals.var_ab_dn0 = assign16460_e24012_d_n0;
        locals.var_ab_dn2 = assign16460_e24012_d_n2;
        locals.var_ab_dn6 = assign16460_e24012_d_n6;
        locals.var_ab_dn7 = assign16460_e24012_d_n7;
        locals.var_ab_dn10 = assign16460_e24012_d_n10;
        locals.var_ab_dn11 = assign16460_e24012_d_n11;
        locals.var_ab_dn12 = assign16460_e24012_d_n12;
        locals.var_ab_dn17 = assign16460_e24012_d_n17;
        locals.var_ab_rv = 0.0;

        let (assign16470_e24024, assign16470_e24024_d_n0, assign16470_e24024_d_n2, assign16470_e24024_d_n6, assign16470_e24024_d_n7, assign16470_e24024_d_n10, assign16470_e24024_d_n11, assign16470_e24024_d_n12, assign16470_e24024_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16470_e24019: f64 = (locals.var_q_nl + locals.var_q_n0);
        let assign16470_e24020: f64 = (-assign16470_e24019);
        let assign16470_e24022: f64 = (assign16470_e24020 / 2.0);
        (assign16470_e24022, ((-(locals.var_q_nl_dn0 + locals.var_q_n0_dn0)) / 2.0), ((-(locals.var_q_nl_dn2 + locals.var_q_n0_dn2)) / 2.0), ((-(locals.var_q_nl_dn6 + locals.var_q_n0_dn6)) / 2.0), ((-(locals.var_q_nl_dn7 + locals.var_q_n0_dn7)) / 2.0), ((-(locals.var_q_nl_dn10 + locals.var_q_n0_dn10)) / 2.0), ((-(locals.var_q_nl_dn11 + locals.var_q_n0_dn11)) / 2.0), ((-(locals.var_q_nl_dn12 + locals.var_q_n0_dn12)) / 2.0), ((-(locals.var_q_nl_dn17 + locals.var_q_n0_dn17)) / 2.0),)
    } else {
        (locals.var_ai, locals.var_ai_dn0, locals.var_ai_dn2, locals.var_ai_dn6, locals.var_ai_dn7, locals.var_ai_dn10, locals.var_ai_dn11, locals.var_ai_dn12, locals.var_ai_dn17,)
    }
};
        locals.var_ai = assign16470_e24024;
        locals.var_ai_dn0 = assign16470_e24024_d_n0;
        locals.var_ai_dn2 = assign16470_e24024_d_n2;
        locals.var_ai_dn6 = assign16470_e24024_d_n6;
        locals.var_ai_dn7 = assign16470_e24024_d_n7;
        locals.var_ai_dn10 = assign16470_e24024_d_n10;
        locals.var_ai_dn11 = assign16470_e24024_d_n11;
        locals.var_ai_dn12 = assign16470_e24024_d_n12;
        locals.var_ai_dn17 = assign16470_e24024_d_n17;
        locals.var_ai_rv = 0.0;

        let (assign16480_e24033, assign16480_e24033_d_n0, assign16480_e24033_d_n2, assign16480_e24033_d_n6, assign16480_e24033_d_n7, assign16480_e24033_d_n10, assign16480_e24033_d_n11, assign16480_e24033_d_n12, assign16480_e24033_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16480_e24031: f64 = (locals.var_q_depl - locals.var_q_dep0);
        (assign16480_e24031, (locals.var_q_depl_dn0 - locals.var_q_dep0_dn0), (locals.var_q_depl_dn2 - locals.var_q_dep0_dn2), (locals.var_q_depl_dn6 - locals.var_q_dep0_dn6), (locals.var_q_depl_dn7 - locals.var_q_dep0_dn7), (locals.var_q_depl_dn10 - locals.var_q_dep0_dn10), (locals.var_q_depl_dn11 - locals.var_q_dep0_dn11), (locals.var_q_depl_dn12 - locals.var_q_dep0_dn12), (locals.var_q_depl_dn17 - locals.var_q_dep0_dn17),)
    } else {
        (locals.var_db, locals.var_db_dn0, locals.var_db_dn2, locals.var_db_dn6, locals.var_db_dn7, locals.var_db_dn10, locals.var_db_dn11, locals.var_db_dn12, locals.var_db_dn17,)
    }
};
        locals.var_db = assign16480_e24033;
        locals.var_db_dn0 = assign16480_e24033_d_n0;
        locals.var_db_dn2 = assign16480_e24033_d_n2;
        locals.var_db_dn6 = assign16480_e24033_d_n6;
        locals.var_db_dn7 = assign16480_e24033_d_n7;
        locals.var_db_dn10 = assign16480_e24033_d_n10;
        locals.var_db_dn11 = assign16480_e24033_d_n11;
        locals.var_db_dn12 = assign16480_e24033_d_n12;
        locals.var_db_dn17 = assign16480_e24033_d_n17;
        locals.var_db_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        let (assign16490_e24043, assign16490_e24043_d_n0, assign16490_e24043_d_n2, assign16490_e24043_d_n6, assign16490_e24043_d_n7, assign16490_e24043_d_n10, assign16490_e24043_d_n11, assign16490_e24043_d_n12, assign16490_e24043_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16490_e24040: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign16490_e24041: f64 = (-assign16490_e24040);
        (assign16490_e24041, (-(locals.var_q_nl_dn0 - locals.var_q_n0_dn0)), (-(locals.var_q_nl_dn2 - locals.var_q_n0_dn2)), (-(locals.var_q_nl_dn6 - locals.var_q_n0_dn6)), (-(locals.var_q_nl_dn7 - locals.var_q_n0_dn7)), (-(locals.var_q_nl_dn10 - locals.var_q_n0_dn10)), (-(locals.var_q_nl_dn11 - locals.var_q_n0_dn11)), (-(locals.var_q_nl_dn12 - locals.var_q_n0_dn12)), (-(locals.var_q_nl_dn17 - locals.var_q_n0_dn17)),)
    } else {
        (locals.var_di, locals.var_di_dn0, locals.var_di_dn2, locals.var_di_dn6, locals.var_di_dn7, locals.var_di_dn10, locals.var_di_dn11, locals.var_di_dn12, locals.var_di_dn17,)
    }
};
        locals.var_di = assign16490_e24043;
        locals.var_di_dn0 = assign16490_e24043_d_n0;
        locals.var_di_dn2 = assign16490_e24043_d_n2;
        locals.var_di_dn6 = assign16490_e24043_d_n6;
        locals.var_di_dn7 = assign16490_e24043_d_n7;
        locals.var_di_dn10 = assign16490_e24043_d_n10;
        locals.var_di_dn11 = assign16490_e24043_d_n11;
        locals.var_di_dn12 = assign16490_e24043_d_n12;
        locals.var_di_dn17 = assign16490_e24043_d_n17;
        locals.var_di_rv = 0.0;

        let (assign16500_e24052, assign16500_e24052_d_n0, assign16500_e24052_d_n2, assign16500_e24052_d_n6, assign16500_e24052_d_n7, assign16500_e24052_d_n10, assign16500_e24052_d_n11, assign16500_e24052_d_n12, assign16500_e24052_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16500_e24050: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        (assign16500_e24050, ((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)),)
    } else {
        (locals.var_c2, locals.var_c2_dn0, locals.var_c2_dn2, locals.var_c2_dn6, locals.var_c2_dn7, locals.var_c2_dn10, locals.var_c2_dn11, locals.var_c2_dn12, locals.var_c2_dn17,)
    }
};
        locals.var_c2 = assign16500_e24052;
        locals.var_c2_dn0 = assign16500_e24052_d_n0;
        locals.var_c2_dn2 = assign16500_e24052_d_n2;
        locals.var_c2_dn6 = assign16500_e24052_d_n6;
        locals.var_c2_dn7 = assign16500_e24052_d_n7;
        locals.var_c2_dn10 = assign16500_e24052_d_n10;
        locals.var_c2_dn11 = assign16500_e24052_d_n11;
        locals.var_c2_dn12 = assign16500_e24052_d_n12;
        locals.var_c2_dn17 = assign16500_e24052_d_n17;
        locals.var_c2_rv = 0.0;

        let assign16510_e24055: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard491 = assign16510_e24055;
        locals.var_guard491_rv = 0.0;

        let (assign16520_e24080, assign16520_e24080_d_n0, assign16520_e24080_d_n2, assign16520_e24080_d_n6, assign16520_e24080_d_n7, assign16520_e24080_d_n10, assign16520_e24080_d_n11, assign16520_e24080_d_n12, assign16520_e24080_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard491 != 0.0)) {
        let assign16520_e24064: f64 = (locals.var_ai * locals.var_beta);
        let assign16520_e24066: f64 = (assign16520_e24064 * locals.var_pds);
        let assign16520_e24068: f64 = (assign16520_e24066 - locals.var_di);
        let assign16520_e24071: f64 = (locals.var_db * locals.var_db);
        let assign16520_e24073: f64 = (assign16520_e24071 * locals.var_db);
        let assign16520_e24075: f64 = (assign16520_e24073 / locals.var_c2);
        let assign16520_e24077: f64 = (assign16520_e24075 / 6.0);
        let assign16520_e24078: f64 = (assign16520_e24068 - assign16520_e24077);
        (assign16520_e24078, (((((locals.var_ai_dn0 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn0)) - locals.var_di_dn0) - ((((((((locals.var_db_dn0 * locals.var_db) + (locals.var_db * locals.var_db_dn0)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn0)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn2 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn2)) - locals.var_di_dn2) - ((((((((locals.var_db_dn2 * locals.var_db) + (locals.var_db * locals.var_db_dn2)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn2)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn6 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn6)) - locals.var_di_dn6) - ((((((((locals.var_db_dn6 * locals.var_db) + (locals.var_db * locals.var_db_dn6)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn6)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn7 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn7)) - locals.var_di_dn7) - ((((((((locals.var_db_dn7 * locals.var_db) + (locals.var_db * locals.var_db_dn7)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn7)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((locals.var_ai_dn10 * locals.var_beta) + (locals.var_ai * locals.var_beta_dn10)) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn10)) - locals.var_di_dn10) - ((((((((locals.var_db_dn10 * locals.var_db) + (locals.var_db * locals.var_db_dn10)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn10)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn11 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn11)) - locals.var_di_dn11) - ((((((((locals.var_db_dn11 * locals.var_db) + (locals.var_db * locals.var_db_dn11)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn11)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn12 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn12)) - locals.var_di_dn12) - ((((((((locals.var_db_dn12 * locals.var_db) + (locals.var_db * locals.var_db_dn12)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn12)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn17 * locals.var_beta) * locals.var_pds) + (assign16520_e24064 * locals.var_pds_dn17)) - locals.var_di_dn17) - ((((((((locals.var_db_dn17 * locals.var_db) + (locals.var_db * locals.var_db_dn17)) * locals.var_db) + (assign16520_e24071 * locals.var_db_dn17)) * locals.var_c2) - (assign16520_e24073 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16520_e24080;
        locals.var_idd_dn0 = assign16520_e24080_d_n0;
        locals.var_idd_dn2 = assign16520_e24080_d_n2;
        locals.var_idd_dn6 = assign16520_e24080_d_n6;
        locals.var_idd_dn7 = assign16520_e24080_d_n7;
        locals.var_idd_dn10 = assign16520_e24080_d_n10;
        locals.var_idd_dn11 = assign16520_e24080_d_n11;
        locals.var_idd_dn12 = assign16520_e24080_d_n12;
        locals.var_idd_dn17 = assign16520_e24080_d_n17;
        locals.var_idd_rv = 0.0;

        let (assign16530_e24092, assign16530_e24092_d_n0, assign16530_e24092_d_n2, assign16530_e24092_d_n6, assign16530_e24092_d_n7, assign16530_e24092_d_n10, assign16530_e24092_d_n11, assign16530_e24092_d_n12, assign16530_e24092_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard491 == 0.0)) {
        let assign16530_e24090: f64 = (locals.var_pds * locals.var_fdd);
        (assign16530_e24090, ((locals.var_pds_dn0 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn0)), ((locals.var_pds_dn2 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn2)), ((locals.var_pds_dn6 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn6)), ((locals.var_pds_dn7 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn7)), ((locals.var_pds_dn10 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn10)), ((locals.var_pds_dn11 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn11)), ((locals.var_pds_dn12 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn12)), ((locals.var_pds_dn17 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn17)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16530_e24092;
        locals.var_idd_dn0 = assign16530_e24092_d_n0;
        locals.var_idd_dn2 = assign16530_e24092_d_n2;
        locals.var_idd_dn6 = assign16530_e24092_d_n6;
        locals.var_idd_dn7 = assign16530_e24092_d_n7;
        locals.var_idd_dn10 = assign16530_e24092_d_n10;
        locals.var_idd_dn11 = assign16530_e24092_d_n11;
        locals.var_idd_dn12 = assign16530_e24092_d_n12;
        locals.var_idd_dn17 = assign16530_e24092_d_n17;
        locals.var_idd_rv = 0.0;

        let assign16540_e24099: f64 = if ((locals.var_flg_info >= 1.0) && (locals.var_idd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard492 = assign16540_e24099;
        locals.var_guard492_rv = 0.0;

        let (assign16550_e24108, assign16550_e24108_d_n0, assign16550_e24108_d_n2, assign16550_e24108_d_n6, assign16550_e24108_d_n7, assign16550_e24108_d_n10, assign16550_e24108_d_n11, assign16550_e24108_d_n12, assign16550_e24108_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard492 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16550_e24108;
        locals.var_idd_dn0 = assign16550_e24108_d_n0;
        locals.var_idd_dn2 = assign16550_e24108_d_n2;
        locals.var_idd_dn6 = assign16550_e24108_d_n6;
        locals.var_idd_dn7 = assign16550_e24108_d_n7;
        locals.var_idd_dn10 = assign16550_e24108_d_n10;
        locals.var_idd_dn11 = assign16550_e24108_d_n11;
        locals.var_idd_dn12 = assign16550_e24108_d_n12;
        locals.var_idd_dn17 = assign16550_e24108_d_n17;
        locals.var_idd_rv = 0.0;

        let assign16560_e24111: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard493 = assign16560_e24111;
        locals.var_guard493_rv = 0.0;

        let assign16570_e24113: f64 = (locals.var_pds).abs();
        let assign16570_e24115: f64 = if assign16570_e24113 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard494 = assign16570_e24115;
        locals.var_guard494_rv = 0.0;

        let (assign16580_e24172, assign16580_e24172_d_n0, assign16580_e24172_d_n2, assign16580_e24172_d_n6, assign16580_e24172_d_n7, assign16580_e24172_d_n10, assign16580_e24172_d_n11, assign16580_e24172_d_n12, assign16580_e24172_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard493 != 0.0)) && (locals.var_guard494 != 0.0)) {
        let assign16580_e24127: f64 = (locals.var_ai * locals.var_beta);
        let assign16580_e24129: f64 = (assign16580_e24127 * locals.var_pds);
        let assign16580_e24131: f64 = (assign16580_e24129 - locals.var_di);
        let assign16580_e24132: f64 = (locals.var_ab * assign16580_e24131);
        let assign16580_e24136: f64 = (2.0 * locals.var_ab);
        let assign16580_e24137: f64 = (locals.var_ai - assign16580_e24136);
        let assign16580_e24140: f64 = (locals.var_c_fox / locals.var_beta);
        let assign16580_e24144: f64 = (2.0 * locals.var_ab);
        let assign16580_e24146: f64 = (assign16580_e24144 * locals.var_ab);
        let assign16580_e24148: f64 = (assign16580_e24146 / locals.var_c2);
        let assign16580_e24149: f64 = (1.0 - assign16580_e24148);
        let assign16580_e24152: f64 = (locals.var_db * locals.var_db);
        let assign16580_e24154: f64 = (assign16580_e24152 / locals.var_c2);
        let assign16580_e24156: f64 = (assign16580_e24154 / 10.0);
        let assign16580_e24157: f64 = (assign16580_e24149 + assign16580_e24156);
        let assign16580_e24158: f64 = (assign16580_e24140 * assign16580_e24157);
        let assign16580_e24159: f64 = (assign16580_e24137 + assign16580_e24158);
        let assign16580_e24161: f64 = (assign16580_e24159 * locals.var_db);
        let assign16580_e24163: f64 = (assign16580_e24161 * locals.var_db);
        let assign16580_e24165: f64 = (assign16580_e24163 * locals.var_db);
        let assign16580_e24167: f64 = (assign16580_e24165 / locals.var_c2);
        let assign16580_e24169: f64 = (assign16580_e24167 / 6.0);
        let assign16580_e24170: f64 = (assign16580_e24132 + assign16580_e24169);
        (assign16580_e24170, (((locals.var_ab_dn0 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn0 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn0)) - locals.var_di_dn0))) + ((((((((((((locals.var_ai_dn0 - (2.0 * locals.var_ab_dn0)) + (((locals.var_c_fox_dn0 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn0) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn0)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn0 * locals.var_db) + (locals.var_db * locals.var_db_dn0)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn0)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn0)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn0)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn2 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn2 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn2)) - locals.var_di_dn2))) + ((((((((((((locals.var_ai_dn2 - (2.0 * locals.var_ab_dn2)) + (((locals.var_c_fox_dn2 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn2) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn2)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn2 * locals.var_db) + (locals.var_db * locals.var_db_dn2)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn2)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn2)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn2)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn6 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn6 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn6)) - locals.var_di_dn6))) + ((((((((((((locals.var_ai_dn6 - (2.0 * locals.var_ab_dn6)) + (((locals.var_c_fox_dn6 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn6) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn6)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn6 * locals.var_db) + (locals.var_db * locals.var_db_dn6)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn6)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn6)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn6)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn7 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn7 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn7)) - locals.var_di_dn7))) + ((((((((((((locals.var_ai_dn7 - (2.0 * locals.var_ab_dn7)) + (((locals.var_c_fox_dn7 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn7) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn7)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn7 * locals.var_db) + (locals.var_db * locals.var_db_dn7)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn7)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn7)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn7)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn10 * assign16580_e24131) + (locals.var_ab * (((((locals.var_ai_dn10 * locals.var_beta) + (locals.var_ai * locals.var_beta_dn10)) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn10)) - locals.var_di_dn10))) + ((((((((((((locals.var_ai_dn10 - (2.0 * locals.var_ab_dn10)) + (((((locals.var_c_fox_dn10 * locals.var_beta) - (locals.var_c_fox * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn10) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn10)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn10 * locals.var_db) + (locals.var_db * locals.var_db_dn10)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn10)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn10)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn10)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn11 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn11 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn11)) - locals.var_di_dn11))) + ((((((((((((locals.var_ai_dn11 - (2.0 * locals.var_ab_dn11)) + (((locals.var_c_fox_dn11 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn11) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn11)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn11 * locals.var_db) + (locals.var_db * locals.var_db_dn11)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn11)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn11)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn11)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn12 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn12 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn12)) - locals.var_di_dn12))) + ((((((((((((locals.var_ai_dn12 - (2.0 * locals.var_ab_dn12)) + (((locals.var_c_fox_dn12 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn12) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn12)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn12 * locals.var_db) + (locals.var_db * locals.var_db_dn12)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn12)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn12)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn12)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn17 * assign16580_e24131) + (locals.var_ab * ((((locals.var_ai_dn17 * locals.var_beta) * locals.var_pds) + (assign16580_e24127 * locals.var_pds_dn17)) - locals.var_di_dn17))) + ((((((((((((locals.var_ai_dn17 - (2.0 * locals.var_ab_dn17)) + (((locals.var_c_fox_dn17 / locals.var_beta) * assign16580_e24157) + (assign16580_e24140 * ((-((((((2.0 * locals.var_ab_dn17) * locals.var_ab) + (assign16580_e24144 * locals.var_ab_dn17)) * locals.var_c2) - (assign16580_e24146 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn17 * locals.var_db) + (locals.var_db * locals.var_db_dn17)) * locals.var_c2) - (assign16580_e24152 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16580_e24159 * locals.var_db_dn17)) * locals.var_db) + (assign16580_e24161 * locals.var_db_dn17)) * locals.var_db) + (assign16580_e24163 * locals.var_db_dn17)) * locals.var_c2) - (assign16580_e24165 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16580_e24172;
        locals.var_qbu_dn0 = assign16580_e24172_d_n0;
        locals.var_qbu_dn2 = assign16580_e24172_d_n2;
        locals.var_qbu_dn6 = assign16580_e24172_d_n6;
        locals.var_qbu_dn7 = assign16580_e24172_d_n7;
        locals.var_qbu_dn10 = assign16580_e24172_d_n10;
        locals.var_qbu_dn11 = assign16580_e24172_d_n11;
        locals.var_qbu_dn12 = assign16580_e24172_d_n12;
        locals.var_qbu_dn17 = assign16580_e24172_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16590_e24185, assign16590_e24185_d_n0, assign16590_e24185_d_n2, assign16590_e24185_d_n6, assign16590_e24185_d_n7, assign16590_e24185_d_n10, assign16590_e24185_d_n11, assign16590_e24185_d_n12, assign16590_e24185_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard493 != 0.0)) && (locals.var_guard494 != 0.0)) {
        let assign16590_e24183: f64 = (locals.var_qbu / locals.var_idd);
        (assign16590_e24183, (((locals.var_qbu_dn0 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn0)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn2 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn2)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn6 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn6)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn7 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn7)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn10 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn10)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn11 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn11)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn12 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn12)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn17 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn17)) / (locals.var_idd * locals.var_idd)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16590_e24185;
        locals.var_qbu_dn0 = assign16590_e24185_d_n0;
        locals.var_qbu_dn2 = assign16590_e24185_d_n2;
        locals.var_qbu_dn6 = assign16590_e24185_d_n6;
        locals.var_qbu_dn7 = assign16590_e24185_d_n7;
        locals.var_qbu_dn10 = assign16590_e24185_d_n10;
        locals.var_qbu_dn11 = assign16590_e24185_d_n11;
        locals.var_qbu_dn12 = assign16590_e24185_d_n12;
        locals.var_qbu_dn17 = assign16590_e24185_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16600_e24197, assign16600_e24197_d_n0, assign16600_e24197_d_n2, assign16600_e24197_d_n6, assign16600_e24197_d_n7, assign16600_e24197_d_n10, assign16600_e24197_d_n11, assign16600_e24197_d_n12, assign16600_e24197_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard493 != 0.0)) && (locals.var_guard494 == 0.0)) {
        (locals.var_ab, locals.var_ab_dn0, locals.var_ab_dn2, locals.var_ab_dn6, locals.var_ab_dn7, locals.var_ab_dn10, locals.var_ab_dn11, locals.var_ab_dn12, locals.var_ab_dn17,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16600_e24197;
        locals.var_qbu_dn0 = assign16600_e24197_d_n0;
        locals.var_qbu_dn2 = assign16600_e24197_d_n2;
        locals.var_qbu_dn6 = assign16600_e24197_d_n6;
        locals.var_qbu_dn7 = assign16600_e24197_d_n7;
        locals.var_qbu_dn10 = assign16600_e24197_d_n10;
        locals.var_qbu_dn11 = assign16600_e24197_d_n11;
        locals.var_qbu_dn12 = assign16600_e24197_d_n12;
        locals.var_qbu_dn17 = assign16600_e24197_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16610_e24211, assign16610_e24211_d_n0, assign16610_e24211_d_n2, assign16610_e24211_d_n6, assign16610_e24211_d_n7, assign16610_e24211_d_n10, assign16610_e24211_d_n11, assign16610_e24211_d_n12, assign16610_e24211_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard493 == 0.0)) {
        let assign16610_e24208: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign16610_e24209: f64 = (0.5 * assign16610_e24208);
        (assign16610_e24209, (0.5 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (0.5 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (0.5 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (0.5 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (0.5 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (0.5 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (0.5 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (0.5 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16610_e24211;
        locals.var_qbu_dn0 = assign16610_e24211_d_n0;
        locals.var_qbu_dn2 = assign16610_e24211_d_n2;
        locals.var_qbu_dn6 = assign16610_e24211_d_n6;
        locals.var_qbu_dn7 = assign16610_e24211_d_n7;
        locals.var_qbu_dn10 = assign16610_e24211_d_n10;
        locals.var_qbu_dn11 = assign16610_e24211_d_n11;
        locals.var_qbu_dn12 = assign16610_e24211_d_n12;
        locals.var_qbu_dn17 = assign16610_e24211_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16620_e24220, assign16620_e24220_d_n0, assign16620_e24220_d_n2, assign16620_e24220_d_n6, assign16620_e24220_d_n7, assign16620_e24220_d_n10, assign16620_e24220_d_n11, assign16620_e24220_d_n12, assign16620_e24220_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16620_e24218: f64 = (2.0 * locals.var_fac1);
        (assign16620_e24218, (2.0 * locals.var_fac1_dn0), (2.0 * locals.var_fac1_dn2), (2.0 * locals.var_fac1_dn6), (2.0 * locals.var_fac1_dn7), (2.0 * locals.var_fac1_dn10), (2.0 * locals.var_fac1_dn11), (2.0 * locals.var_fac1_dn12), (2.0 * locals.var_fac1_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16620_e24220;
        locals.var_t1_dn0 = assign16620_e24220_d_n0;
        locals.var_t1_dn2 = assign16620_e24220_d_n2;
        locals.var_t1_dn6 = assign16620_e24220_d_n6;
        locals.var_t1_dn7 = assign16620_e24220_d_n7;
        locals.var_t1_dn10 = assign16620_e24220_d_n10;
        locals.var_t1_dn11 = assign16620_e24220_d_n11;
        locals.var_t1_dn12 = assign16620_e24220_d_n12;
        locals.var_t1_dn17 = assign16620_e24220_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16630_e24231, assign16630_e24231_d_n0, assign16630_e24231_d_n2, assign16630_e24231_d_n6, assign16630_e24231_d_n7, assign16630_e24231_d_n10, assign16630_e24231_d_n11, assign16630_e24231_d_n12, assign16630_e24231_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16630_e24228: f64 = (locals.var_f10 - locals.var_xi0p12);
        let assign16630_e24229: f64 = (locals.var_t1 * assign16630_e24228);
        (assign16630_e24229, ((locals.var_t1_dn0 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn0 - locals.var_xi0p12_dn0))), ((locals.var_t1_dn2 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn2 - locals.var_xi0p12_dn2))), ((locals.var_t1_dn6 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn6 - locals.var_xi0p12_dn6))), ((locals.var_t1_dn7 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn7 - locals.var_xi0p12_dn7))), ((locals.var_t1_dn10 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn10 - locals.var_xi0p12_dn10))), ((locals.var_t1_dn11 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn11 - locals.var_xi0p12_dn11))), ((locals.var_t1_dn12 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn12 - locals.var_xi0p12_dn12))), ((locals.var_t1_dn17 * assign16630_e24228) + (locals.var_t1 * (locals.var_f10_dn17 - locals.var_xi0p12_dn17))),)
    } else {
        (locals.var_dtpds, locals.var_dtpds_dn0, locals.var_dtpds_dn2, locals.var_dtpds_dn6, locals.var_dtpds_dn7, locals.var_dtpds_dn10, locals.var_dtpds_dn11, locals.var_dtpds_dn12, locals.var_dtpds_dn17,)
    }
};
        locals.var_dtpds = assign16630_e24231;
        locals.var_dtpds_dn0 = assign16630_e24231_d_n0;
        locals.var_dtpds_dn2 = assign16630_e24231_d_n2;
        locals.var_dtpds_dn6 = assign16630_e24231_d_n6;
        locals.var_dtpds_dn7 = assign16630_e24231_d_n7;
        locals.var_dtpds_dn10 = assign16630_e24231_d_n10;
        locals.var_dtpds_dn11 = assign16630_e24231_d_n11;
        locals.var_dtpds_dn12 = assign16630_e24231_d_n12;
        locals.var_dtpds_dn17 = assign16630_e24231_d_n17;
        locals.var_dtpds_rv = 0.0;

        let (assign16640_e24240, assign16640_e24240_d_n0, assign16640_e24240_d_n2, assign16640_e24240_d_n6, assign16640_e24240_d_n7, assign16640_e24240_d_n10, assign16640_e24240_d_n11, assign16640_e24240_d_n12, assign16640_e24240_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16640_e24238: f64 = (locals.var_pds + locals.var_dtpds);
        (assign16640_e24238, (locals.var_pds_dn0 + locals.var_dtpds_dn0), (locals.var_pds_dn2 + locals.var_dtpds_dn2), (locals.var_pds_dn6 + locals.var_dtpds_dn6), (locals.var_pds_dn7 + locals.var_dtpds_dn7), (locals.var_pds_dn10 + locals.var_dtpds_dn10), (locals.var_pds_dn11 + locals.var_dtpds_dn11), (locals.var_pds_dn12 + locals.var_dtpds_dn12), (locals.var_pds_dn17 + locals.var_dtpds_dn17),)
    } else {
        (locals.var_achi, locals.var_achi_dn0, locals.var_achi_dn2, locals.var_achi_dn6, locals.var_achi_dn7, locals.var_achi_dn10, locals.var_achi_dn11, locals.var_achi_dn12, locals.var_achi_dn17,)
    }
};
        locals.var_achi = assign16640_e24240;
        locals.var_achi_dn0 = assign16640_e24240_d_n0;
        locals.var_achi_dn2 = assign16640_e24240_d_n2;
        locals.var_achi_dn6 = assign16640_e24240_d_n6;
        locals.var_achi_dn7 = assign16640_e24240_d_n7;
        locals.var_achi_dn10 = assign16640_e24240_d_n10;
        locals.var_achi_dn11 = assign16640_e24240_d_n11;
        locals.var_achi_dn12 = assign16640_e24240_d_n12;
        locals.var_achi_dn17 = assign16640_e24240_d_n17;
        locals.var_achi_rv = 0.0;

        let (assign16650_e24249, assign16650_e24249_d_n0, assign16650_e24249_d_n2, assign16650_e24249_d_n6, assign16650_e24249_d_n7, assign16650_e24249_d_n10, assign16650_e24249_d_n11, assign16650_e24249_d_n12, assign16650_e24249_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16650_e24247: f64 = (1.0 / locals.var_vgvt);
        (assign16650_e24247, (-(locals.var_vgvt_dn0 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn2 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn6 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn7 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn10 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn11 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn12 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn17 / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16650_e24249;
        locals.var_t1_dn0 = assign16650_e24249_d_n0;
        locals.var_t1_dn2 = assign16650_e24249_d_n2;
        locals.var_t1_dn6 = assign16650_e24249_d_n6;
        locals.var_t1_dn7 = assign16650_e24249_d_n7;
        locals.var_t1_dn10 = assign16650_e24249_d_n10;
        locals.var_t1_dn11 = assign16650_e24249_d_n11;
        locals.var_t1_dn12 = assign16650_e24249_d_n12;
        locals.var_t1_dn17 = assign16650_e24249_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16660_e24258, assign16660_e24258_d_n0, assign16660_e24258_d_n2, assign16660_e24258_d_n6, assign16660_e24258_d_n7, assign16660_e24258_d_n10, assign16660_e24258_d_n11, assign16660_e24258_d_n12, assign16660_e24258_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16660_e24256: f64 = (locals.var_achi * locals.var_t1);
        (assign16660_e24256, ((locals.var_achi_dn0 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn0)), ((locals.var_achi_dn2 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn2)), ((locals.var_achi_dn6 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn6)), ((locals.var_achi_dn7 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn7)), ((locals.var_achi_dn10 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn10)), ((locals.var_achi_dn11 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn11)), ((locals.var_achi_dn12 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn12)), ((locals.var_achi_dn17 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign16660_e24258;
        locals.var_t2_dn0 = assign16660_e24258_d_n0;
        locals.var_t2_dn2 = assign16660_e24258_d_n2;
        locals.var_t2_dn6 = assign16660_e24258_d_n6;
        locals.var_t2_dn7 = assign16660_e24258_d_n7;
        locals.var_t2_dn10 = assign16660_e24258_d_n10;
        locals.var_t2_dn11 = assign16660_e24258_d_n11;
        locals.var_t2_dn12 = assign16660_e24258_d_n12;
        locals.var_t2_dn17 = assign16660_e24258_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign16670_e24267, assign16670_e24267_d_n0, assign16670_e24267_d_n2, assign16670_e24267_d_n6, assign16670_e24267_d_n7, assign16670_e24267_d_n10, assign16670_e24267_d_n11, assign16670_e24267_d_n12, assign16670_e24267_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16670_e24265: f64 = (1.0 - locals.var_t2);
        (assign16670_e24265, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn12), (-locals.var_t2_dn17),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign16670_e24267;
        locals.var_t3_dn0 = assign16670_e24267_d_n0;
        locals.var_t3_dn2 = assign16670_e24267_d_n2;
        locals.var_t3_dn6 = assign16670_e24267_d_n6;
        locals.var_t3_dn7 = assign16670_e24267_d_n7;
        locals.var_t3_dn10 = assign16670_e24267_d_n10;
        locals.var_t3_dn11 = assign16670_e24267_d_n11;
        locals.var_t3_dn12 = assign16670_e24267_d_n12;
        locals.var_t3_dn17 = assign16670_e24267_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign16680_e24276, assign16680_e24276_d_n0, assign16680_e24276_d_n2, assign16680_e24276_d_n6, assign16680_e24276_d_n7, assign16680_e24276_d_n10, assign16680_e24276_d_n11, assign16680_e24276_d_n12, assign16680_e24276_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16680_e24274: f64 = (1.0 - locals.var_t3);
        (assign16680_e24274, (-locals.var_t3_dn0), (-locals.var_t3_dn2), (-locals.var_t3_dn6), (-locals.var_t3_dn7), (-locals.var_t3_dn10), (-locals.var_t3_dn11), (-locals.var_t3_dn12), (-locals.var_t3_dn17),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign16680_e24276;
        locals.var_tx_dn0 = assign16680_e24276_d_n0;
        locals.var_tx_dn2 = assign16680_e24276_d_n2;
        locals.var_tx_dn6 = assign16680_e24276_d_n6;
        locals.var_tx_dn7 = assign16680_e24276_d_n7;
        locals.var_tx_dn10 = assign16680_e24276_d_n10;
        locals.var_tx_dn11 = assign16680_e24276_d_n11;
        locals.var_tx_dn12 = assign16680_e24276_d_n12;
        locals.var_tx_dn17 = assign16680_e24276_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign16690_e24285, assign16690_e24285_d_n0, assign16690_e24285_d_n2, assign16690_e24285_d_n6, assign16690_e24285_d_n7, assign16690_e24285_d_n10, assign16690_e24285_d_n11, assign16690_e24285_d_n12, assign16690_e24285_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16690_e24283: f64 = (locals.var_tx * locals.var_tx);
        (assign16690_e24283, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn12 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn12)), ((locals.var_tx_dn17 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign16690_e24285;
        locals.var_x2_dn0 = assign16690_e24285_d_n0;
        locals.var_x2_dn2 = assign16690_e24285_d_n2;
        locals.var_x2_dn6 = assign16690_e24285_d_n6;
        locals.var_x2_dn7 = assign16690_e24285_d_n7;
        locals.var_x2_dn10 = assign16690_e24285_d_n10;
        locals.var_x2_dn11 = assign16690_e24285_d_n11;
        locals.var_x2_dn12 = assign16690_e24285_d_n12;
        locals.var_x2_dn17 = assign16690_e24285_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign16700_e24294, assign16700_e24294_d_n0, assign16700_e24294_d_n2, assign16700_e24294_d_n6, assign16700_e24294_d_n7, assign16700_e24294_d_n10, assign16700_e24294_d_n11, assign16700_e24294_d_n12, assign16700_e24294_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16700_e24292: f64 = 1.0;
        (assign16700_e24292, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign16700_e24294;
        locals.var_xmax2_dn0 = assign16700_e24294_d_n0;
        locals.var_xmax2_dn2 = assign16700_e24294_d_n2;
        locals.var_xmax2_dn6 = assign16700_e24294_d_n6;
        locals.var_xmax2_dn7 = assign16700_e24294_d_n7;
        locals.var_xmax2_dn10 = assign16700_e24294_d_n10;
        locals.var_xmax2_dn11 = assign16700_e24294_d_n11;
        locals.var_xmax2_dn12 = assign16700_e24294_d_n12;
        locals.var_xmax2_dn17 = assign16700_e24294_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign16710_e24301, assign16710_e24301_d_n0, assign16710_e24301_d_n2, assign16710_e24301_d_n6, assign16710_e24301_d_n7, assign16710_e24301_d_n10, assign16710_e24301_d_n11, assign16710_e24301_d_n12, assign16710_e24301_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16710_e24301;
        locals.var_xp_dn0 = assign16710_e24301_d_n0;
        locals.var_xp_dn2 = assign16710_e24301_d_n2;
        locals.var_xp_dn6 = assign16710_e24301_d_n6;
        locals.var_xp_dn7 = assign16710_e24301_d_n7;
        locals.var_xp_dn10 = assign16710_e24301_d_n10;
        locals.var_xp_dn11 = assign16710_e24301_d_n11;
        locals.var_xp_dn12 = assign16710_e24301_d_n12;
        locals.var_xp_dn17 = assign16710_e24301_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16720_e24308, assign16720_e24308_d_n0, assign16720_e24308_d_n2, assign16720_e24308_d_n6, assign16720_e24308_d_n7, assign16720_e24308_d_n10, assign16720_e24308_d_n11, assign16720_e24308_d_n12, assign16720_e24308_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16720_e24308;
        locals.var_xmp_dn0 = assign16720_e24308_d_n0;
        locals.var_xmp_dn2 = assign16720_e24308_d_n2;
        locals.var_xmp_dn6 = assign16720_e24308_d_n6;
        locals.var_xmp_dn7 = assign16720_e24308_d_n7;
        locals.var_xmp_dn10 = assign16720_e24308_d_n10;
        locals.var_xmp_dn11 = assign16720_e24308_d_n11;
        locals.var_xmp_dn12 = assign16720_e24308_d_n12;
        locals.var_xmp_dn17 = assign16720_e24308_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16730_e24315,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign16730_e24315;
        locals.var_m0_rv = 0.0;

        let (assign16740_e24322,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16740_e24322;
        locals.var_mm_rv = 0.0;

        let (assign16750_e24329, assign16750_e24329_d_n0, assign16750_e24329_d_n2, assign16750_e24329_d_n6, assign16750_e24329_d_n7, assign16750_e24329_d_n10, assign16750_e24329_d_n11, assign16750_e24329_d_n12, assign16750_e24329_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign16750_e24329;
        locals.var_arg_dn0 = assign16750_e24329_d_n0;
        locals.var_arg_dn2 = assign16750_e24329_d_n2;
        locals.var_arg_dn6 = assign16750_e24329_d_n6;
        locals.var_arg_dn7 = assign16750_e24329_d_n7;
        locals.var_arg_dn10 = assign16750_e24329_d_n10;
        locals.var_arg_dn11 = assign16750_e24329_d_n11;
        locals.var_arg_dn12 = assign16750_e24329_d_n12;
        locals.var_arg_dn17 = assign16750_e24329_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign16760_e24336, assign16760_e24336_d_n0, assign16760_e24336_d_n2, assign16760_e24336_d_n6, assign16760_e24336_d_n7, assign16760_e24336_d_n10, assign16760_e24336_d_n11, assign16760_e24336_d_n12, assign16760_e24336_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16760_e24336;
        locals.var_dnm_dn0 = assign16760_e24336_d_n0;
        locals.var_dnm_dn2 = assign16760_e24336_d_n2;
        locals.var_dnm_dn6 = assign16760_e24336_d_n6;
        locals.var_dnm_dn7 = assign16760_e24336_d_n7;
        locals.var_dnm_dn10 = assign16760_e24336_d_n10;
        locals.var_dnm_dn11 = assign16760_e24336_d_n11;
        locals.var_dnm_dn12 = assign16760_e24336_d_n12;
        locals.var_dnm_dn17 = assign16760_e24336_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign16770_e24345, assign16770_e24345_d_n0, assign16770_e24345_d_n2, assign16770_e24345_d_n6, assign16770_e24345_d_n7, assign16770_e24345_d_n10, assign16770_e24345_d_n11, assign16770_e24345_d_n12, assign16770_e24345_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16770_e24343: f64 = (locals.var_xp * locals.var_x2);
        (assign16770_e24343, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16770_e24345;
        locals.var_xp_dn0 = assign16770_e24345_d_n0;
        locals.var_xp_dn2 = assign16770_e24345_d_n2;
        locals.var_xp_dn6 = assign16770_e24345_d_n6;
        locals.var_xp_dn7 = assign16770_e24345_d_n7;
        locals.var_xp_dn10 = assign16770_e24345_d_n10;
        locals.var_xp_dn11 = assign16770_e24345_d_n11;
        locals.var_xp_dn12 = assign16770_e24345_d_n12;
        locals.var_xp_dn17 = assign16770_e24345_d_n17;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        locals: &mut StampLocals,
    ) {
        let (assign16780_e24354, assign16780_e24354_d_n0, assign16780_e24354_d_n2, assign16780_e24354_d_n6, assign16780_e24354_d_n7, assign16780_e24354_d_n10, assign16780_e24354_d_n11, assign16780_e24354_d_n12, assign16780_e24354_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16780_e24352: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16780_e24352, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16780_e24354;
        locals.var_xmp_dn0 = assign16780_e24354_d_n0;
        locals.var_xmp_dn2 = assign16780_e24354_d_n2;
        locals.var_xmp_dn6 = assign16780_e24354_d_n6;
        locals.var_xmp_dn7 = assign16780_e24354_d_n7;
        locals.var_xmp_dn10 = assign16780_e24354_d_n10;
        locals.var_xmp_dn11 = assign16780_e24354_d_n11;
        locals.var_xmp_dn12 = assign16780_e24354_d_n12;
        locals.var_xmp_dn17 = assign16780_e24354_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16790_e24363, assign16790_e24363_d_n0, assign16790_e24363_d_n2, assign16790_e24363_d_n6, assign16790_e24363_d_n7, assign16790_e24363_d_n10, assign16790_e24363_d_n11, assign16790_e24363_d_n12, assign16790_e24363_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16790_e24361: f64 = (locals.var_xp * locals.var_x2);
        (assign16790_e24361, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16790_e24363;
        locals.var_xp_dn0 = assign16790_e24363_d_n0;
        locals.var_xp_dn2 = assign16790_e24363_d_n2;
        locals.var_xp_dn6 = assign16790_e24363_d_n6;
        locals.var_xp_dn7 = assign16790_e24363_d_n7;
        locals.var_xp_dn10 = assign16790_e24363_d_n10;
        locals.var_xp_dn11 = assign16790_e24363_d_n11;
        locals.var_xp_dn12 = assign16790_e24363_d_n12;
        locals.var_xp_dn17 = assign16790_e24363_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16800_e24372, assign16800_e24372_d_n0, assign16800_e24372_d_n2, assign16800_e24372_d_n6, assign16800_e24372_d_n7, assign16800_e24372_d_n10, assign16800_e24372_d_n11, assign16800_e24372_d_n12, assign16800_e24372_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16800_e24370: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16800_e24370, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16800_e24372;
        locals.var_xmp_dn0 = assign16800_e24372_d_n0;
        locals.var_xmp_dn2 = assign16800_e24372_d_n2;
        locals.var_xmp_dn6 = assign16800_e24372_d_n6;
        locals.var_xmp_dn7 = assign16800_e24372_d_n7;
        locals.var_xmp_dn10 = assign16800_e24372_d_n10;
        locals.var_xmp_dn11 = assign16800_e24372_d_n11;
        locals.var_xmp_dn12 = assign16800_e24372_d_n12;
        locals.var_xmp_dn17 = assign16800_e24372_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16810_e24381, assign16810_e24381_d_n0, assign16810_e24381_d_n2, assign16810_e24381_d_n6, assign16810_e24381_d_n7, assign16810_e24381_d_n10, assign16810_e24381_d_n11, assign16810_e24381_d_n12, assign16810_e24381_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16810_e24379: f64 = (locals.var_xp * locals.var_x2);
        (assign16810_e24379, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16810_e24381;
        locals.var_xp_dn0 = assign16810_e24381_d_n0;
        locals.var_xp_dn2 = assign16810_e24381_d_n2;
        locals.var_xp_dn6 = assign16810_e24381_d_n6;
        locals.var_xp_dn7 = assign16810_e24381_d_n7;
        locals.var_xp_dn10 = assign16810_e24381_d_n10;
        locals.var_xp_dn11 = assign16810_e24381_d_n11;
        locals.var_xp_dn12 = assign16810_e24381_d_n12;
        locals.var_xp_dn17 = assign16810_e24381_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16820_e24390, assign16820_e24390_d_n0, assign16820_e24390_d_n2, assign16820_e24390_d_n6, assign16820_e24390_d_n7, assign16820_e24390_d_n10, assign16820_e24390_d_n11, assign16820_e24390_d_n12, assign16820_e24390_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16820_e24388: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16820_e24388, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16820_e24390;
        locals.var_xmp_dn0 = assign16820_e24390_d_n0;
        locals.var_xmp_dn2 = assign16820_e24390_d_n2;
        locals.var_xmp_dn6 = assign16820_e24390_d_n6;
        locals.var_xmp_dn7 = assign16820_e24390_d_n7;
        locals.var_xmp_dn10 = assign16820_e24390_d_n10;
        locals.var_xmp_dn11 = assign16820_e24390_d_n11;
        locals.var_xmp_dn12 = assign16820_e24390_d_n12;
        locals.var_xmp_dn17 = assign16820_e24390_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16830_e24399, assign16830_e24399_d_n0, assign16830_e24399_d_n2, assign16830_e24399_d_n6, assign16830_e24399_d_n7, assign16830_e24399_d_n10, assign16830_e24399_d_n11, assign16830_e24399_d_n12, assign16830_e24399_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16830_e24397: f64 = (locals.var_xp * locals.var_x2);
        (assign16830_e24397, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16830_e24399;
        locals.var_xp_dn0 = assign16830_e24399_d_n0;
        locals.var_xp_dn2 = assign16830_e24399_d_n2;
        locals.var_xp_dn6 = assign16830_e24399_d_n6;
        locals.var_xp_dn7 = assign16830_e24399_d_n7;
        locals.var_xp_dn10 = assign16830_e24399_d_n10;
        locals.var_xp_dn11 = assign16830_e24399_d_n11;
        locals.var_xp_dn12 = assign16830_e24399_d_n12;
        locals.var_xp_dn17 = assign16830_e24399_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16840_e24408, assign16840_e24408_d_n0, assign16840_e24408_d_n2, assign16840_e24408_d_n6, assign16840_e24408_d_n7, assign16840_e24408_d_n10, assign16840_e24408_d_n11, assign16840_e24408_d_n12, assign16840_e24408_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16840_e24406: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16840_e24406, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16840_e24408;
        locals.var_xmp_dn0 = assign16840_e24408_d_n0;
        locals.var_xmp_dn2 = assign16840_e24408_d_n2;
        locals.var_xmp_dn6 = assign16840_e24408_d_n6;
        locals.var_xmp_dn7 = assign16840_e24408_d_n7;
        locals.var_xmp_dn10 = assign16840_e24408_d_n10;
        locals.var_xmp_dn11 = assign16840_e24408_d_n11;
        locals.var_xmp_dn12 = assign16840_e24408_d_n12;
        locals.var_xmp_dn17 = assign16840_e24408_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16850_e24417, assign16850_e24417_d_n0, assign16850_e24417_d_n2, assign16850_e24417_d_n6, assign16850_e24417_d_n7, assign16850_e24417_d_n10, assign16850_e24417_d_n11, assign16850_e24417_d_n12, assign16850_e24417_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16850_e24415: f64 = (locals.var_xp + locals.var_xmp);
        (assign16850_e24415, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign16850_e24417;
        locals.var_arg_dn0 = assign16850_e24417_d_n0;
        locals.var_arg_dn2 = assign16850_e24417_d_n2;
        locals.var_arg_dn6 = assign16850_e24417_d_n6;
        locals.var_arg_dn7 = assign16850_e24417_d_n7;
        locals.var_arg_dn10 = assign16850_e24417_d_n10;
        locals.var_arg_dn11 = assign16850_e24417_d_n11;
        locals.var_arg_dn12 = assign16850_e24417_d_n12;
        locals.var_arg_dn17 = assign16850_e24417_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign16860_e24424, assign16860_e24424_d_n0, assign16860_e24424_d_n2, assign16860_e24424_d_n6, assign16860_e24424_d_n7, assign16860_e24424_d_n10, assign16860_e24424_d_n11, assign16860_e24424_d_n12, assign16860_e24424_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16860_e24424;
        locals.var_dnm_dn0 = assign16860_e24424_d_n0;
        locals.var_dnm_dn2 = assign16860_e24424_d_n2;
        locals.var_dnm_dn6 = assign16860_e24424_d_n6;
        locals.var_dnm_dn7 = assign16860_e24424_d_n7;
        locals.var_dnm_dn10 = assign16860_e24424_d_n10;
        locals.var_dnm_dn11 = assign16860_e24424_d_n11;
        locals.var_dnm_dn12 = assign16860_e24424_d_n12;
        locals.var_dnm_dn17 = assign16860_e24424_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign16870_e24439: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard495 = assign16870_e24439;
        locals.var_guard495_rv = 0.0;

        let assign16880_e24442: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard496 = assign16880_e24442;
        locals.var_guard496_rv = 0.0;

        let (assign16890_e24453,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16890_e24453;
        locals.var_mm_rv = 0.0;

        let assign16900_e24456: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard497 = assign16900_e24456;
        locals.var_guard497_rv = 0.0;

        let (assign16910_e24470,) = {
    if (((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 == 0.0)) && (locals.var_guard497 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16910_e24470;
        locals.var_mm_rv = 0.0;

        let assign16920_e24473: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign16920_e24473;
        locals.var_guard498_rv = 0.0;

        let (assign16930_e24490,) = {
    if ((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 == 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard498 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16930_e24490;
        locals.var_mm_rv = 0.0;

        let assign16940_e24493: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard499 = assign16940_e24493;
        locals.var_guard499_rv = 0.0;

        let (assign16950_e24513,) = {
    if (((((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_guard496 == 0.0)) && (locals.var_guard497 == 0.0)) && (locals.var_guard498 == 0.0)) && (locals.var_guard499 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16950_e24513;
        locals.var_mm_rv = 0.0;

        let (assign16960_e24522,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign16960_e24522;
        locals.var_m0_rv = 0.0;

        let mut assign16970_loop_guard: usize = 0;
        while {
            let assign16970_cond_e24532: f64 = if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign16970_cond_e24532 != 0.0
        } {
            assign16970_loop_guard += 1;
            assert!(assign16970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign16970_body0_e24542, assign16970_body0_e24542_d_n0, assign16970_body0_e24542_d_n2, assign16970_body0_e24542_d_n6, assign16970_body0_e24542_d_n7, assign16970_body0_e24542_d_n10, assign16970_body0_e24542_d_n11, assign16970_body0_e24542_d_n12, assign16970_body0_e24542_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) {
        let assign16970_body0_e24540: f64 = (locals.var_dnm).sqrt();
        (assign16970_body0_e24540, (locals.var_dnm_dn0 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn2 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn6 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn7 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn10 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn11 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn12 / (2.0 * assign16970_body0_e24540)), (locals.var_dnm_dn17 / (2.0 * assign16970_body0_e24540)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign16970_body0_e24542;
            locals.var_dnm_dn0 = assign16970_body0_e24542_d_n0;
            locals.var_dnm_dn2 = assign16970_body0_e24542_d_n2;
            locals.var_dnm_dn6 = assign16970_body0_e24542_d_n6;
            locals.var_dnm_dn7 = assign16970_body0_e24542_d_n7;
            locals.var_dnm_dn10 = assign16970_body0_e24542_d_n10;
            locals.var_dnm_dn11 = assign16970_body0_e24542_d_n11;
            locals.var_dnm_dn12 = assign16970_body0_e24542_d_n12;
            locals.var_dnm_dn17 = assign16970_body0_e24542_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign16970_body1_e24553,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 != 0.0)) {
        let assign16970_body1_e24551: f64 = (locals.var_m0 + 1.0);
        (assign16970_body1_e24551,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign16970_body1_e24553;
            locals.var_m0_rv = 0.0;
        }

        let (assign16980_e24569, assign16980_e24569_d_n0, assign16980_e24569_d_n2, assign16980_e24569_d_n6, assign16980_e24569_d_n7, assign16980_e24569_d_n10, assign16980_e24569_d_n11, assign16980_e24569_d_n12, assign16980_e24569_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard495 == 0.0)) {
        let assign16980_e24565: f64 = (2.0 * 4.0);
        let assign16980_e24566: f64 = (1.0 / assign16980_e24565);
        let assign16980_e24567: f64 = (locals.var_dnm).powf(assign16980_e24566);
        (assign16980_e24567, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn0)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn2)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn6)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn7)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn10)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn11)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn12)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign16980_e24566) as f64).is_finite() && ((assign16980_e24566) as f64).fract() == 0.0 { if assign16980_e24566 == 0.0 { 0.0 } else { (assign16980_e24566 * ((locals.var_dnm).powf(assign16980_e24566 - 1.0) * locals.var_dnm_dn17)) } } else { (assign16980_e24567 * (assign16980_e24566 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16980_e24569;
        locals.var_dnm_dn0 = assign16980_e24569_d_n0;
        locals.var_dnm_dn2 = assign16980_e24569_d_n2;
        locals.var_dnm_dn6 = assign16980_e24569_d_n6;
        locals.var_dnm_dn7 = assign16980_e24569_d_n7;
        locals.var_dnm_dn10 = assign16980_e24569_d_n10;
        locals.var_dnm_dn11 = assign16980_e24569_d_n11;
        locals.var_dnm_dn12 = assign16980_e24569_d_n12;
        locals.var_dnm_dn17 = assign16980_e24569_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign16990_e24578, assign16990_e24578_d_n0, assign16990_e24578_d_n2, assign16990_e24578_d_n6, assign16990_e24578_d_n7, assign16990_e24578_d_n10, assign16990_e24578_d_n11, assign16990_e24578_d_n12, assign16990_e24578_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign16990_e24576: f64 = (1.0 / locals.var_dnm);
        (assign16990_e24576, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16990_e24578;
        locals.var_dnm_dn0 = assign16990_e24578_d_n0;
        locals.var_dnm_dn2 = assign16990_e24578_d_n2;
        locals.var_dnm_dn6 = assign16990_e24578_d_n6;
        locals.var_dnm_dn7 = assign16990_e24578_d_n7;
        locals.var_dnm_dn10 = assign16990_e24578_d_n10;
        locals.var_dnm_dn11 = assign16990_e24578_d_n11;
        locals.var_dnm_dn12 = assign16990_e24578_d_n12;
        locals.var_dnm_dn17 = assign16990_e24578_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign17000_e24589, assign17000_e24589_d_n0, assign17000_e24589_d_n2, assign17000_e24589_d_n6, assign17000_e24589_d_n7, assign17000_e24589_d_n10, assign17000_e24589_d_n11, assign17000_e24589_d_n12, assign17000_e24589_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17000_e24585: f64 = locals.var_tx;
        let assign17000_e24587: f64 = (assign17000_e24585 * locals.var_dnm);
        (assign17000_e24587, ((locals.var_tx_dn0 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn2)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn7)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn11)), ((locals.var_tx_dn12 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn12)), ((locals.var_tx_dn17 * locals.var_dnm) + (assign17000_e24585 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign17000_e24589;
        locals.var_ty_dn0 = assign17000_e24589_d_n0;
        locals.var_ty_dn2 = assign17000_e24589_d_n2;
        locals.var_ty_dn6 = assign17000_e24589_d_n6;
        locals.var_ty_dn7 = assign17000_e24589_d_n7;
        locals.var_ty_dn10 = assign17000_e24589_d_n10;
        locals.var_ty_dn11 = assign17000_e24589_d_n11;
        locals.var_ty_dn12 = assign17000_e24589_d_n12;
        locals.var_ty_dn17 = assign17000_e24589_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign17010_e24598, assign17010_e24598_d_n0, assign17010_e24598_d_n2, assign17010_e24598_d_n6, assign17010_e24598_d_n7, assign17010_e24598_d_n10, assign17010_e24598_d_n11, assign17010_e24598_d_n12, assign17010_e24598_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17010_e24596: f64 = (1.0 - locals.var_ty);
        (assign17010_e24596, (-locals.var_ty_dn0), (-locals.var_ty_dn2), (-locals.var_ty_dn6), (-locals.var_ty_dn7), (-locals.var_ty_dn10), (-locals.var_ty_dn11), (-locals.var_ty_dn12), (-locals.var_ty_dn17),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign17010_e24598;
        locals.var_alpha_dn0 = assign17010_e24598_d_n0;
        locals.var_alpha_dn2 = assign17010_e24598_d_n2;
        locals.var_alpha_dn6 = assign17010_e24598_d_n6;
        locals.var_alpha_dn7 = assign17010_e24598_d_n7;
        locals.var_alpha_dn10 = assign17010_e24598_d_n10;
        locals.var_alpha_dn11 = assign17010_e24598_d_n11;
        locals.var_alpha_dn12 = assign17010_e24598_d_n12;
        locals.var_alpha_dn17 = assign17010_e24598_d_n17;
        locals.var_alpha_rv = 0.0;

        let (assign17020_e24611, assign17020_e24611_d_n0, assign17020_e24611_d_n2, assign17020_e24611_d_n6, assign17020_e24611_d_n7, assign17020_e24611_d_n10, assign17020_e24611_d_n11, assign17020_e24611_d_n12, assign17020_e24611_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17020_e24607: f64 = (1.0 + locals.var_alpha);
        let assign17020_e24608: f64 = (locals.var_alpha * assign17020_e24607);
        let assign17020_e24609: f64 = (1.0 + assign17020_e24608);
        (assign17020_e24609, ((locals.var_alpha_dn0 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign17020_e24607) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign17020_e24611;
        locals.var_qinm_dn0 = assign17020_e24611_d_n0;
        locals.var_qinm_dn2 = assign17020_e24611_d_n2;
        locals.var_qinm_dn6 = assign17020_e24611_d_n6;
        locals.var_qinm_dn7 = assign17020_e24611_d_n7;
        locals.var_qinm_dn10 = assign17020_e24611_d_n10;
        locals.var_qinm_dn11 = assign17020_e24611_d_n11;
        locals.var_qinm_dn12 = assign17020_e24611_d_n12;
        locals.var_qinm_dn17 = assign17020_e24611_d_n17;
        locals.var_qinm_rv = 0.0;

        let (assign17030_e24631, assign17030_e24631_d_n0, assign17030_e24631_d_n2, assign17030_e24631_d_n6, assign17030_e24631_d_n7, assign17030_e24631_d_n10, assign17030_e24631_d_n11, assign17030_e24631_d_n12, assign17030_e24631_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17030_e24618: f64 = (1.0 + locals.var_alpha);
        let assign17030_e24621: f64 = (10.0 * 2.220446049250313e-16);
        let (assign17030_e24629, assign17030_e24629_d_n0, assign17030_e24629_d_n2, assign17030_e24629_d_n6, assign17030_e24629_d_n7, assign17030_e24629_d_n10, assign17030_e24629_d_n11, assign17030_e24629_d_n12, assign17030_e24629_d_n17,) = {
            if (assign17030_e24618 >= assign17030_e24621) {
                let assign17030_e24625: f64 = (1.0 + locals.var_alpha);
                (assign17030_e24625, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign17030_e24628: f64 = (10.0 * 2.220446049250313e-16);
                (assign17030_e24628, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign17030_e24629, assign17030_e24629_d_n0, assign17030_e24629_d_n2, assign17030_e24629_d_n6, assign17030_e24629_d_n7, assign17030_e24629_d_n10, assign17030_e24629_d_n11, assign17030_e24629_d_n12, assign17030_e24629_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign17030_e24631;
        locals.var_qidn_dn0 = assign17030_e24631_d_n0;
        locals.var_qidn_dn2 = assign17030_e24631_d_n2;
        locals.var_qidn_dn6 = assign17030_e24631_d_n6;
        locals.var_qidn_dn7 = assign17030_e24631_d_n7;
        locals.var_qidn_dn10 = assign17030_e24631_d_n10;
        locals.var_qidn_dn11 = assign17030_e24631_d_n11;
        locals.var_qidn_dn12 = assign17030_e24631_d_n12;
        locals.var_qidn_dn17 = assign17030_e24631_d_n17;
        locals.var_qidn_rv = 0.0;

        let (assign17040_e24644, assign17040_e24644_d_n0, assign17040_e24644_d_n2, assign17040_e24644_d_n6, assign17040_e24644_d_n7, assign17040_e24644_d_n10, assign17040_e24644_d_n11, assign17040_e24644_d_n12, assign17040_e24644_d_n17,) = {
    if ((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign17040_e24638: f64 = (0.6666666666666667 * locals.var_vgvt);
        let assign17040_e24640: f64 = (assign17040_e24638 * locals.var_qinm);
        let assign17040_e24642: f64 = (assign17040_e24640 / locals.var_qidn);
        (assign17040_e24642, ((((((0.6666666666666667 * locals.var_vgvt_dn0) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn0)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn0)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn2) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn2)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn2)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn6) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn6)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn6)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn7) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn7)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn7)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn10) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn10)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn10)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn11) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn11)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn11)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn12) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn12)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn12)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn17) * locals.var_qinm) + (assign17040_e24638 * locals.var_qinm_dn17)) * locals.var_qidn) - (assign17040_e24640 * locals.var_qidn_dn17)) / (locals.var_qidn * locals.var_qidn)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign17040_e24644;
        locals.var_t1_dn0 = assign17040_e24644_d_n0;
        locals.var_t1_dn2 = assign17040_e24644_d_n2;
        locals.var_t1_dn6 = assign17040_e24644_d_n6;
        locals.var_t1_dn7 = assign17040_e24644_d_n7;
        locals.var_t1_dn10 = assign17040_e24644_d_n10;
        locals.var_t1_dn11 = assign17040_e24644_d_n11;
        locals.var_t1_dn12 = assign17040_e24644_d_n12;
        locals.var_t1_dn17 = assign17040_e24644_d_n17;
        locals.var_t1_rv = 0.0;

        let assign17050_e24647: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign17050_e24647;
        locals.var_guard500_rv = 0.0;

        let assign17060_e24649: f64 = (locals.var_pds).abs();
        let assign17060_e24651: f64 = if assign17060_e24649 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign17060_e24651;
        locals.var_guard501_rv = 0.0;

        let (assign17070_e24704, assign17070_e24704_d_n0, assign17070_e24704_d_n2, assign17070_e24704_d_n6, assign17070_e24704_d_n7, assign17070_e24704_d_n10, assign17070_e24704_d_n11, assign17070_e24704_d_n12, assign17070_e24704_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 != 0.0)) {
        let assign17070_e24662: f64 = (locals.var_ai * locals.var_ai);
        let assign17070_e24665: f64 = (locals.var_di * locals.var_di);
        let assign17070_e24667: f64 = (assign17070_e24665 / 12.0);
        let assign17070_e24668: f64 = (assign17070_e24662 + assign17070_e24667);
        let assign17070_e24670: f64 = (assign17070_e24668 * locals.var_beta);
        let assign17070_e24672: f64 = (assign17070_e24670 * locals.var_pds);
        let assign17070_e24675: f64 = (locals.var_ai * locals.var_di);
        let assign17070_e24676: f64 = (assign17070_e24672 - assign17070_e24675);
        let assign17070_e24679: f64 = (2.0 * locals.var_ai);
        let assign17070_e24682: f64 = (locals.var_c_fox / locals.var_beta);
        let assign17070_e24684: f64 = (assign17070_e24682 * locals.var_db);
        let assign17070_e24686: f64 = (assign17070_e24684 * locals.var_db);
        let assign17070_e24688: f64 = (assign17070_e24686 / locals.var_c2);
        let assign17070_e24690: f64 = (assign17070_e24688 / 5.0);
        let assign17070_e24691: f64 = (assign17070_e24679 + assign17070_e24690);
        let assign17070_e24693: f64 = (assign17070_e24691 * locals.var_db);
        let assign17070_e24695: f64 = (assign17070_e24693 * locals.var_db);
        let assign17070_e24697: f64 = (assign17070_e24695 * locals.var_db);
        let assign17070_e24699: f64 = (assign17070_e24697 / locals.var_c2);
        let assign17070_e24701: f64 = (assign17070_e24699 / 6.0);
        let assign17070_e24702: f64 = (assign17070_e24676 - assign17070_e24701);
        (assign17070_e24702, ((((((((locals.var_ai_dn0 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn0)) + (((locals.var_di_dn0 * locals.var_di) + (locals.var_di * locals.var_di_dn0)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn0)) - ((locals.var_ai_dn0 * locals.var_di) + (locals.var_ai * locals.var_di_dn0))) - ((((((((((((2.0 * locals.var_ai_dn0) + (((((((((locals.var_c_fox_dn0 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn0)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn0)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn0)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn0)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn0)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn2 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn2)) + (((locals.var_di_dn2 * locals.var_di) + (locals.var_di * locals.var_di_dn2)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn2)) - ((locals.var_ai_dn2 * locals.var_di) + (locals.var_ai * locals.var_di_dn2))) - ((((((((((((2.0 * locals.var_ai_dn2) + (((((((((locals.var_c_fox_dn2 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn2)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn2)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn2)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn2)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn2)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn6 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn6)) + (((locals.var_di_dn6 * locals.var_di) + (locals.var_di * locals.var_di_dn6)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn6)) - ((locals.var_ai_dn6 * locals.var_di) + (locals.var_ai * locals.var_di_dn6))) - ((((((((((((2.0 * locals.var_ai_dn6) + (((((((((locals.var_c_fox_dn6 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn6)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn6)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn6)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn6)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn6)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn7 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn7)) + (((locals.var_di_dn7 * locals.var_di) + (locals.var_di * locals.var_di_dn7)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn7)) - ((locals.var_ai_dn7 * locals.var_di) + (locals.var_ai * locals.var_di_dn7))) - ((((((((((((2.0 * locals.var_ai_dn7) + (((((((((locals.var_c_fox_dn7 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn7)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn7)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn7)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn7)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn7)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((((((locals.var_ai_dn10 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn10)) + (((locals.var_di_dn10 * locals.var_di) + (locals.var_di * locals.var_di_dn10)) / 12.0)) * locals.var_beta) + (assign17070_e24668 * locals.var_beta_dn10)) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn10)) - ((locals.var_ai_dn10 * locals.var_di) + (locals.var_ai * locals.var_di_dn10))) - ((((((((((((2.0 * locals.var_ai_dn10) + (((((((((((locals.var_c_fox_dn10 * locals.var_beta) - (locals.var_c_fox * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn10)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn10)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn10)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn10)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn10)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn11 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn11)) + (((locals.var_di_dn11 * locals.var_di) + (locals.var_di * locals.var_di_dn11)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn11)) - ((locals.var_ai_dn11 * locals.var_di) + (locals.var_ai * locals.var_di_dn11))) - ((((((((((((2.0 * locals.var_ai_dn11) + (((((((((locals.var_c_fox_dn11 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn11)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn11)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn11)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn11)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn11)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn12 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn12)) + (((locals.var_di_dn12 * locals.var_di) + (locals.var_di * locals.var_di_dn12)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn12)) - ((locals.var_ai_dn12 * locals.var_di) + (locals.var_ai * locals.var_di_dn12))) - ((((((((((((2.0 * locals.var_ai_dn12) + (((((((((locals.var_c_fox_dn12 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn12)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn12)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn12)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn12)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn12)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn17 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn17)) + (((locals.var_di_dn17 * locals.var_di) + (locals.var_di * locals.var_di_dn17)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17070_e24670 * locals.var_pds_dn17)) - ((locals.var_ai_dn17 * locals.var_di) + (locals.var_ai * locals.var_di_dn17))) - ((((((((((((2.0 * locals.var_ai_dn17) + (((((((((locals.var_c_fox_dn17 / locals.var_beta) * locals.var_db) + (assign17070_e24682 * locals.var_db_dn17)) * locals.var_db) + (assign17070_e24684 * locals.var_db_dn17)) * locals.var_c2) - (assign17070_e24686 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17070_e24691 * locals.var_db_dn17)) * locals.var_db) + (assign17070_e24693 * locals.var_db_dn17)) * locals.var_db) + (assign17070_e24695 * locals.var_db_dn17)) * locals.var_c2) - (assign17070_e24697 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17070_e24704;
        locals.var_qiu_dn0 = assign17070_e24704_d_n0;
        locals.var_qiu_dn2 = assign17070_e24704_d_n2;
        locals.var_qiu_dn6 = assign17070_e24704_d_n6;
        locals.var_qiu_dn7 = assign17070_e24704_d_n7;
        locals.var_qiu_dn10 = assign17070_e24704_d_n10;
        locals.var_qiu_dn11 = assign17070_e24704_d_n11;
        locals.var_qiu_dn12 = assign17070_e24704_d_n12;
        locals.var_qiu_dn17 = assign17070_e24704_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign17080_e24717, assign17080_e24717_d_n0, assign17080_e24717_d_n2, assign17080_e24717_d_n6, assign17080_e24717_d_n7, assign17080_e24717_d_n10, assign17080_e24717_d_n11, assign17080_e24717_d_n12, assign17080_e24717_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 != 0.0)) {
        let assign17080_e24715: f64 = (locals.var_qiu / locals.var_idd);
        (assign17080_e24715, (((locals.var_qiu_dn0 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn0)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn2 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn2)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn6 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn6)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn7 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn7)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn10 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn10)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn11 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn11)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn12 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn12)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn17 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn17)) / (locals.var_idd * locals.var_idd)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17080_e24717;
        locals.var_qiu_dn0 = assign17080_e24717_d_n0;
        locals.var_qiu_dn2 = assign17080_e24717_d_n2;
        locals.var_qiu_dn6 = assign17080_e24717_d_n6;
        locals.var_qiu_dn7 = assign17080_e24717_d_n7;
        locals.var_qiu_dn10 = assign17080_e24717_d_n10;
        locals.var_qiu_dn11 = assign17080_e24717_d_n11;
        locals.var_qiu_dn12 = assign17080_e24717_d_n12;
        locals.var_qiu_dn17 = assign17080_e24717_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign17090_e24729, assign17090_e24729_d_n0, assign17090_e24729_d_n2, assign17090_e24729_d_n6, assign17090_e24729_d_n7, assign17090_e24729_d_n10, assign17090_e24729_d_n11, assign17090_e24729_d_n12, assign17090_e24729_d_n17,) = {
    if ((((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard500 != 0.0)) && (locals.var_guard501 == 0.0)) {
        (locals.var_ai, locals.var_ai_dn0, locals.var_ai_dn2, locals.var_ai_dn6, locals.var_ai_dn7, locals.var_ai_dn10, locals.var_ai_dn11, locals.var_ai_dn12, locals.var_ai_dn17,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17090_e24729;
        locals.var_qiu_dn0 = assign17090_e24729_d_n0;
        locals.var_qiu_dn2 = assign17090_e24729_d_n2;
        locals.var_qiu_dn6 = assign17090_e24729_d_n6;
        locals.var_qiu_dn7 = assign17090_e24729_d_n7;
        locals.var_qiu_dn10 = assign17090_e24729_d_n10;
        locals.var_qiu_dn11 = assign17090_e24729_d_n11;
        locals.var_qiu_dn12 = assign17090_e24729_d_n12;
        locals.var_qiu_dn17 = assign17090_e24729_d_n17;
        locals.var_qiu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17100_e24744, assign17100_e24744_d_n0, assign17100_e24744_d_n2, assign17100_e24744_d_n6, assign17100_e24744_d_n7, assign17100_e24744_d_n10, assign17100_e24744_d_n11, assign17100_e24744_d_n12, assign17100_e24744_d_n17,) = {
    if (((locals.var_guard109 == 0.0) && (locals.var_guard461 != 0.0)) && (locals.var_guard500 == 0.0)) {
        let assign17100_e24738: f64 = (-0.5);
        let assign17100_e24741: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign17100_e24742: f64 = (assign17100_e24738 * assign17100_e24741);
        (assign17100_e24742, (assign17100_e24738 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign17100_e24738 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign17100_e24738 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign17100_e24738 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign17100_e24738 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign17100_e24738 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign17100_e24738 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign17100_e24738 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17100_e24744;
        locals.var_qiu_dn0 = assign17100_e24744_d_n0;
        locals.var_qiu_dn2 = assign17100_e24744_d_n2;
        locals.var_qiu_dn6 = assign17100_e24744_d_n6;
        locals.var_qiu_dn7 = assign17100_e24744_d_n7;
        locals.var_qiu_dn10 = assign17100_e24744_d_n10;
        locals.var_qiu_dn11 = assign17100_e24744_d_n11;
        locals.var_qiu_dn12 = assign17100_e24744_d_n12;
        locals.var_qiu_dn17 = assign17100_e24744_d_n17;
        locals.var_qiu_rv = 0.0;

        let assign17140_e24758: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard505 = assign17140_e24758;
        locals.var_guard505_rv = 0.0;

        let (assign17150_e24764, assign17150_e24764_d_n0, assign17150_e24764_d_n2, assign17150_e24764_d_n6, assign17150_e24764_d_n7, assign17150_e24764_d_n10, assign17150_e24764_d_n11, assign17150_e24764_d_n12, assign17150_e24764_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17150_e24762: f64 = (0.5 + locals.var_alpha);
        (assign17150_e24762, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    } else {
        (locals.var_qdnm, locals.var_qdnm_dn0, locals.var_qdnm_dn2, locals.var_qdnm_dn6, locals.var_qdnm_dn7, locals.var_qdnm_dn10, locals.var_qdnm_dn11, locals.var_qdnm_dn12, locals.var_qdnm_dn17,)
    }
};
        locals.var_qdnm = assign17150_e24764;
        locals.var_qdnm_dn0 = assign17150_e24764_d_n0;
        locals.var_qdnm_dn2 = assign17150_e24764_d_n2;
        locals.var_qdnm_dn6 = assign17150_e24764_d_n6;
        locals.var_qdnm_dn7 = assign17150_e24764_d_n7;
        locals.var_qdnm_dn10 = assign17150_e24764_d_n10;
        locals.var_qdnm_dn11 = assign17150_e24764_d_n11;
        locals.var_qdnm_dn12 = assign17150_e24764_d_n12;
        locals.var_qdnm_dn17 = assign17150_e24764_d_n17;
        locals.var_qdnm_rv = 0.0;

        let (assign17160_e24770, assign17160_e24770_d_n0, assign17160_e24770_d_n2, assign17160_e24770_d_n6, assign17160_e24770_d_n7, assign17160_e24770_d_n10, assign17160_e24770_d_n11, assign17160_e24770_d_n12, assign17160_e24770_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17160_e24768: f64 = (locals.var_qidn * locals.var_qinm);
        (assign17160_e24768, ((locals.var_qidn_dn0 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn0)), ((locals.var_qidn_dn2 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn2)), ((locals.var_qidn_dn6 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn6)), ((locals.var_qidn_dn7 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn7)), ((locals.var_qidn_dn10 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn10)), ((locals.var_qidn_dn11 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn11)), ((locals.var_qidn_dn12 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn12)), ((locals.var_qidn_dn17 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn17)),)
    } else {
        (locals.var_qddn, locals.var_qddn_dn0, locals.var_qddn_dn2, locals.var_qddn_dn6, locals.var_qddn_dn7, locals.var_qddn_dn10, locals.var_qddn_dn11, locals.var_qddn_dn12, locals.var_qddn_dn17,)
    }
};
        locals.var_qddn = assign17160_e24770;
        locals.var_qddn_dn0 = assign17160_e24770_d_n0;
        locals.var_qddn_dn2 = assign17160_e24770_d_n2;
        locals.var_qddn_dn6 = assign17160_e24770_d_n6;
        locals.var_qddn_dn7 = assign17160_e24770_d_n7;
        locals.var_qddn_dn10 = assign17160_e24770_d_n10;
        locals.var_qddn_dn11 = assign17160_e24770_d_n11;
        locals.var_qddn_dn12 = assign17160_e24770_d_n12;
        locals.var_qddn_dn17 = assign17160_e24770_d_n17;
        locals.var_qddn_rv = 0.0;

        let (assign17170_e24778, assign17170_e24778_d_n0, assign17170_e24778_d_n2, assign17170_e24778_d_n6, assign17170_e24778_d_n7, assign17170_e24778_d_n10, assign17170_e24778_d_n11, assign17170_e24778_d_n12, assign17170_e24778_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17170_e24774: f64 = (0.4 * locals.var_qdnm);
        let assign17170_e24776: f64 = (assign17170_e24774 / locals.var_qddn);
        (assign17170_e24776, ((((0.4 * locals.var_qdnm_dn0) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn0)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn2) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn2)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn6) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn6)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn7) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn7)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn10) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn10)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn11) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn11)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn12) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn12)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn17) * locals.var_qddn) - (assign17170_e24774 * locals.var_qddn_dn17)) / (locals.var_qddn * locals.var_qddn)),)
    } else {
        (locals.var_quot, locals.var_quot_dn0, locals.var_quot_dn2, locals.var_quot_dn6, locals.var_quot_dn7, locals.var_quot_dn10, locals.var_quot_dn11, locals.var_quot_dn12, locals.var_quot_dn17,)
    }
};
        locals.var_quot = assign17170_e24778;
        locals.var_quot_dn0 = assign17170_e24778_d_n0;
        locals.var_quot_dn2 = assign17170_e24778_d_n2;
        locals.var_quot_dn6 = assign17170_e24778_d_n6;
        locals.var_quot_dn7 = assign17170_e24778_d_n7;
        locals.var_quot_dn10 = assign17170_e24778_d_n10;
        locals.var_quot_dn11 = assign17170_e24778_d_n11;
        locals.var_quot_dn12 = assign17170_e24778_d_n12;
        locals.var_quot_dn17 = assign17170_e24778_d_n17;
        locals.var_quot_rv = 0.0;

        let (assign17180_e24784, assign17180_e24784_d_n0, assign17180_e24784_d_n2, assign17180_e24784_d_n6, assign17180_e24784_d_n7, assign17180_e24784_d_n10, assign17180_e24784_d_n11, assign17180_e24784_d_n12, assign17180_e24784_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17180_e24782: f64 = (0.6 - locals.var_quot);
        (assign17180_e24782, (-locals.var_quot_dn0), (-locals.var_quot_dn2), (-locals.var_quot_dn6), (-locals.var_quot_dn7), (-locals.var_quot_dn10), (-locals.var_quot_dn11), (-locals.var_quot_dn12), (-locals.var_quot_dn17),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17180_e24784;
        locals.var_qdrat_dn0 = assign17180_e24784_d_n0;
        locals.var_qdrat_dn2 = assign17180_e24784_d_n2;
        locals.var_qdrat_dn6 = assign17180_e24784_d_n6;
        locals.var_qdrat_dn7 = assign17180_e24784_d_n7;
        locals.var_qdrat_dn10 = assign17180_e24784_d_n10;
        locals.var_qdrat_dn11 = assign17180_e24784_d_n11;
        locals.var_qdrat_dn12 = assign17180_e24784_d_n12;
        locals.var_qdrat_dn17 = assign17180_e24784_d_n17;
        locals.var_qdrat_rv = 0.0;

        let assign17190_e24788: f64 = (0.5 + 1e-8);
        let assign17190_e24789: f64 = if locals.var_qdrat > assign17190_e24788 { 1.0 } else { 0.0 };
        locals.var_guard506 = assign17190_e24789;
        locals.var_guard506_rv = 0.0;

        let (assign17210_e24798, assign17210_e24798_d_n0, assign17210_e24798_d_n2, assign17210_e24798_d_n6, assign17210_e24798_d_n7, assign17210_e24798_d_n10, assign17210_e24798_d_n11, assign17210_e24798_d_n12, assign17210_e24798_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard506 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17210_e24798;
        locals.var_qdrat_dn0 = assign17210_e24798_d_n0;
        locals.var_qdrat_dn2 = assign17210_e24798_d_n2;
        locals.var_qdrat_dn6 = assign17210_e24798_d_n6;
        locals.var_qdrat_dn7 = assign17210_e24798_d_n7;
        locals.var_qdrat_dn10 = assign17210_e24798_d_n10;
        locals.var_qdrat_dn11 = assign17210_e24798_d_n11;
        locals.var_qdrat_dn12 = assign17210_e24798_d_n12;
        locals.var_qdrat_dn17 = assign17210_e24798_d_n17;
        locals.var_qdrat_rv = 0.0;

        let (assign17220_e24802, assign17220_e24802_d_n0, assign17220_e24802_d_n2, assign17220_e24802_d_n6, assign17220_e24802_d_n7, assign17220_e24802_d_n10, assign17220_e24802_d_n11, assign17220_e24802_d_n12, assign17220_e24802_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    } else {
        (locals.var_qdrat_noi, locals.var_qdrat_noi_dn0, locals.var_qdrat_noi_dn2, locals.var_qdrat_noi_dn6, locals.var_qdrat_noi_dn7, locals.var_qdrat_noi_dn10, locals.var_qdrat_noi_dn11, locals.var_qdrat_noi_dn12, locals.var_qdrat_noi_dn17,)
    }
};
        locals.var_qdrat_noi = assign17220_e24802;
        locals.var_qdrat_noi_dn0 = assign17220_e24802_d_n0;
        locals.var_qdrat_noi_dn2 = assign17220_e24802_d_n2;
        locals.var_qdrat_noi_dn6 = assign17220_e24802_d_n6;
        locals.var_qdrat_noi_dn7 = assign17220_e24802_d_n7;
        locals.var_qdrat_noi_dn10 = assign17220_e24802_d_n10;
        locals.var_qdrat_noi_dn11 = assign17220_e24802_d_n11;
        locals.var_qdrat_noi_dn12 = assign17220_e24802_d_n12;
        locals.var_qdrat_noi_dn17 = assign17220_e24802_d_n17;
        locals.var_qdrat_noi_rv = 0.0;

        let (assign17230_e24806, assign17230_e24806_d_n0, assign17230_e24806_d_n2, assign17230_e24806_d_n6, assign17230_e24806_d_n7, assign17230_e24806_d_n10, assign17230_e24806_d_n11, assign17230_e24806_d_n12, assign17230_e24806_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17230_e24806;
        locals.var_qdrat_dn0 = assign17230_e24806_d_n0;
        locals.var_qdrat_dn2 = assign17230_e24806_d_n2;
        locals.var_qdrat_dn6 = assign17230_e24806_d_n6;
        locals.var_qdrat_dn7 = assign17230_e24806_d_n7;
        locals.var_qdrat_dn10 = assign17230_e24806_d_n10;
        locals.var_qdrat_dn11 = assign17230_e24806_d_n11;
        locals.var_qdrat_dn12 = assign17230_e24806_d_n12;
        locals.var_qdrat_dn17 = assign17230_e24806_d_n17;
        locals.var_qdrat_rv = 0.0;

        let assign17240_e24809: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard508 = assign17240_e24809;
        locals.var_guard508_rv = 0.0;

        let assign17250_e24813: f64 = (10.0 * 2.220446049250313e-16);
        let assign17250_e24818: f64 = (10.0 * 2.220446049250313e-16);
        let assign17250_e24820: f64 = if ((p.p190 < assign17250_e24813) && (p.p191 < assign17250_e24818)) { 1.0 } else { 0.0 };
        locals.var_guard524 = assign17250_e24820;
        locals.var_guard524_rv = 0.0;

        let (assign17260_e24828, assign17260_e24828_d_n0, assign17260_e24828_d_n2, assign17260_e24828_d_n6, assign17260_e24828_d_n7, assign17260_e24828_d_n10, assign17260_e24828_d_n11, assign17260_e24828_d_n12, assign17260_e24828_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17260_e24828;
        locals.var_lred_dn0 = assign17260_e24828_d_n0;
        locals.var_lred_dn2 = assign17260_e24828_d_n2;
        locals.var_lred_dn6 = assign17260_e24828_d_n6;
        locals.var_lred_dn7 = assign17260_e24828_d_n7;
        locals.var_lred_dn10 = assign17260_e24828_d_n10;
        locals.var_lred_dn11 = assign17260_e24828_d_n11;
        locals.var_lred_dn12 = assign17260_e24828_d_n12;
        locals.var_lred_dn17 = assign17260_e24828_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17270_e24836, assign17270_e24836_d_n0, assign17270_e24836_d_n2, assign17270_e24836_d_n6, assign17270_e24836_d_n7, assign17270_e24836_d_n10, assign17270_e24836_d_n11, assign17270_e24836_d_n12, assign17270_e24836_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17270_e24836;
        locals.var_psdl_dn0 = assign17270_e24836_d_n0;
        locals.var_psdl_dn2 = assign17270_e24836_d_n2;
        locals.var_psdl_dn6 = assign17270_e24836_d_n6;
        locals.var_psdl_dn7 = assign17270_e24836_d_n7;
        locals.var_psdl_dn10 = assign17270_e24836_d_n10;
        locals.var_psdl_dn11 = assign17270_e24836_d_n11;
        locals.var_psdl_dn12 = assign17270_e24836_d_n12;
        locals.var_psdl_dn17 = assign17270_e24836_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign17280_e24840: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17280_e24843: f64 = (10.0 * 2.220446049250313e-16);
        let assign17280_e24844: f64 = (assign17280_e24840 - assign17280_e24843);
        let assign17280_e24845: f64 = if locals.var_psdl > assign17280_e24844 { 1.0 } else { 0.0 };
        locals.var_guard525 = assign17280_e24845;
        locals.var_guard525_rv = 0.0;

        let (assign17290_e24861, assign17290_e24861_d_n0, assign17290_e24861_d_n2, assign17290_e24861_d_n6, assign17290_e24861_d_n7, assign17290_e24861_d_n10, assign17290_e24861_d_n11, assign17290_e24861_d_n12, assign17290_e24861_d_n17,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 != 0.0)) && (locals.var_guard525 != 0.0)) {
        let assign17290_e24855: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17290_e24858: f64 = (10.0 * 2.220446049250313e-16);
        let assign17290_e24859: f64 = (assign17290_e24855 - assign17290_e24858);
        (assign17290_e24859, (locals.var_ps0_dn0 + locals.var_vdsz_dn0), (locals.var_ps0_dn2 + locals.var_vdsz_dn2), (locals.var_ps0_dn6 + locals.var_vdsz_dn6), (locals.var_ps0_dn7 + locals.var_vdsz_dn7), (locals.var_ps0_dn10 + locals.var_vdsz_dn10), (locals.var_ps0_dn11 + locals.var_vdsz_dn11), (locals.var_ps0_dn12 + locals.var_vdsz_dn12), (locals.var_ps0_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17290_e24861;
        locals.var_psdl_dn0 = assign17290_e24861_d_n0;
        locals.var_psdl_dn2 = assign17290_e24861_d_n2;
        locals.var_psdl_dn6 = assign17290_e24861_d_n6;
        locals.var_psdl_dn7 = assign17290_e24861_d_n7;
        locals.var_psdl_dn10 = assign17290_e24861_d_n10;
        locals.var_psdl_dn11 = assign17290_e24861_d_n11;
        locals.var_psdl_dn12 = assign17290_e24861_d_n12;
        locals.var_psdl_dn17 = assign17290_e24861_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign17300_e24875,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let (assign17300_e24873,) = {
            if (p.p43 == 1.0) {
                (p.p237,)
            } else {
                (locals.var_wdsoi_0,)
            }
        };
        (assign17300_e24873,)
    } else {
        (locals.var_wd,)
    }
};
        locals.var_wd = assign17300_e24875;
        locals.var_wd_rv = 0.0;

        let (assign17310_e24886,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17310_e24884: f64 = (1.0 / locals.var_wd);
        (assign17310_e24884,)
    } else {
        (locals.var_t0__blk509,)
    }
};
        locals.var_t0__blk509 = assign17310_e24886;
        locals.var_t0__blk509_rv = 0.0;

        let (assign17320_e24897, assign17320_e24897_d_n0, assign17320_e24897_d_n2, assign17320_e24897_d_n6, assign17320_e24897_d_n7, assign17320_e24897_d_n10, assign17320_e24897_d_n11, assign17320_e24897_d_n12, assign17320_e24897_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17320_e24895: f64 = (locals.var_qn0 * locals.var_t0__blk509);
        (assign17320_e24895, (locals.var_qn0_dn0 * locals.var_t0__blk509), (locals.var_qn0_dn2 * locals.var_t0__blk509), (locals.var_qn0_dn6 * locals.var_t0__blk509), (locals.var_qn0_dn7 * locals.var_t0__blk509), (locals.var_qn0_dn10 * locals.var_t0__blk509), (locals.var_qn0_dn11 * locals.var_t0__blk509), (locals.var_qn0_dn12 * locals.var_t0__blk509), (locals.var_qn0_dn17 * locals.var_t0__blk509),)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17320_e24897;
        locals.var_t1__blk510_dn0 = assign17320_e24897_d_n0;
        locals.var_t1__blk510_dn2 = assign17320_e24897_d_n2;
        locals.var_t1__blk510_dn6 = assign17320_e24897_d_n6;
        locals.var_t1__blk510_dn7 = assign17320_e24897_d_n7;
        locals.var_t1__blk510_dn10 = assign17320_e24897_d_n10;
        locals.var_t1__blk510_dn11 = assign17320_e24897_d_n11;
        locals.var_t1__blk510_dn12 = assign17320_e24897_d_n12;
        locals.var_t1__blk510_dn17 = assign17320_e24897_d_n17;
        locals.var_t1__blk510_rv = 0.0;

        let (assign17330_e24908, assign17330_e24908_d_n0, assign17330_e24908_d_n2, assign17330_e24908_d_n6, assign17330_e24908_d_n7, assign17330_e24908_d_n10, assign17330_e24908_d_n11, assign17330_e24908_d_n12, assign17330_e24908_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17330_e24906: f64 = (p.p191 * locals.var_t1__blk510);
        (assign17330_e24906, (p.p191 * locals.var_t1__blk510_dn0), (p.p191 * locals.var_t1__blk510_dn2), (p.p191 * locals.var_t1__blk510_dn6), (p.p191 * locals.var_t1__blk510_dn7), (p.p191 * locals.var_t1__blk510_dn10), (p.p191 * locals.var_t1__blk510_dn11), (p.p191 * locals.var_t1__blk510_dn12), (p.p191 * locals.var_t1__blk510_dn17),)
    } else {
        (locals.var_t2__blk511, locals.var_t2__blk511_dn0, locals.var_t2__blk511_dn2, locals.var_t2__blk511_dn6, locals.var_t2__blk511_dn7, locals.var_t2__blk511_dn10, locals.var_t2__blk511_dn11, locals.var_t2__blk511_dn12, locals.var_t2__blk511_dn17,)
    }
};
        locals.var_t2__blk511 = assign17330_e24908;
        locals.var_t2__blk511_dn0 = assign17330_e24908_d_n0;
        locals.var_t2__blk511_dn2 = assign17330_e24908_d_n2;
        locals.var_t2__blk511_dn6 = assign17330_e24908_d_n6;
        locals.var_t2__blk511_dn7 = assign17330_e24908_d_n7;
        locals.var_t2__blk511_dn10 = assign17330_e24908_d_n10;
        locals.var_t2__blk511_dn11 = assign17330_e24908_d_n11;
        locals.var_t2__blk511_dn12 = assign17330_e24908_d_n12;
        locals.var_t2__blk511_dn17 = assign17330_e24908_d_n17;
        locals.var_t2__blk511_rv = 0.0;

        let (assign17340_e24921, assign17340_e24921_d_n0, assign17340_e24921_d_n2, assign17340_e24921_d_n6, assign17340_e24921_d_n7, assign17340_e24921_d_n10, assign17340_e24921_d_n11, assign17340_e24921_d_n12, assign17340_e24921_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17340_e24917: f64 = (locals.var_uc_clm2 * locals.var_q_nsub);
        let assign17340_e24919: f64 = (assign17340_e24917 + locals.var_t2__blk511);
        (assign17340_e24919, (((locals.var_uc_clm2_dn0 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn0)) + locals.var_t2__blk511_dn0), (((locals.var_uc_clm2_dn2 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn2)) + locals.var_t2__blk511_dn2), (((locals.var_uc_clm2_dn6 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn6)) + locals.var_t2__blk511_dn6), (((locals.var_uc_clm2_dn7 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn7)) + locals.var_t2__blk511_dn7), (((locals.var_uc_clm2_dn10 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn10)) + locals.var_t2__blk511_dn10), (((locals.var_uc_clm2_dn11 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn11)) + locals.var_t2__blk511_dn11), (((locals.var_uc_clm2_dn12 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn12)) + locals.var_t2__blk511_dn12), (((locals.var_uc_clm2_dn17 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn17)) + locals.var_t2__blk511_dn17),)
    } else {
        (locals.var_t5__blk514, locals.var_t5__blk514_dn0, locals.var_t5__blk514_dn2, locals.var_t5__blk514_dn6, locals.var_t5__blk514_dn7, locals.var_t5__blk514_dn10, locals.var_t5__blk514_dn11, locals.var_t5__blk514_dn12, locals.var_t5__blk514_dn17,)
    }
};
        locals.var_t5__blk514 = assign17340_e24921;
        locals.var_t5__blk514_dn0 = assign17340_e24921_d_n0;
        locals.var_t5__blk514_dn2 = assign17340_e24921_d_n2;
        locals.var_t5__blk514_dn6 = assign17340_e24921_d_n6;
        locals.var_t5__blk514_dn7 = assign17340_e24921_d_n7;
        locals.var_t5__blk514_dn10 = assign17340_e24921_d_n10;
        locals.var_t5__blk514_dn11 = assign17340_e24921_d_n11;
        locals.var_t5__blk514_dn12 = assign17340_e24921_d_n12;
        locals.var_t5__blk514_dn17 = assign17340_e24921_d_n17;
        locals.var_t5__blk514_rv = 0.0;

        let (assign17350_e24932, assign17350_e24932_d_n0, assign17350_e24932_d_n2, assign17350_e24932_d_n6, assign17350_e24932_d_n7, assign17350_e24932_d_n10, assign17350_e24932_d_n11, assign17350_e24932_d_n12, assign17350_e24932_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17350_e24930: f64 = (1.0 / locals.var_t5__blk514);
        (assign17350_e24930, (-(locals.var_t5__blk514_dn0 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn2 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn6 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn7 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn10 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn11 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn12 / (locals.var_t5__blk514 * locals.var_t5__blk514))), (-(locals.var_t5__blk514_dn17 / (locals.var_t5__blk514 * locals.var_t5__blk514))),)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17350_e24932;
        locals.var_t1__blk510_dn0 = assign17350_e24932_d_n0;
        locals.var_t1__blk510_dn2 = assign17350_e24932_d_n2;
        locals.var_t1__blk510_dn6 = assign17350_e24932_d_n6;
        locals.var_t1__blk510_dn7 = assign17350_e24932_d_n7;
        locals.var_t1__blk510_dn10 = assign17350_e24932_d_n10;
        locals.var_t1__blk510_dn11 = assign17350_e24932_d_n11;
        locals.var_t1__blk510_dn12 = assign17350_e24932_d_n12;
        locals.var_t1__blk510_dn17 = assign17350_e24932_d_n17;
        locals.var_t1__blk510_rv = 0.0;

        let (assign17360_e24943, assign17360_e24943_d_n0, assign17360_e24943_d_n2, assign17360_e24943_d_n6, assign17360_e24943_d_n7, assign17360_e24943_d_n10, assign17360_e24943_d_n11, assign17360_e24943_d_n12, assign17360_e24943_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17360_e24941: f64 = (1.034943e-10 * locals.var_t1__blk510);
        (assign17360_e24941, (1.034943e-10 * locals.var_t1__blk510_dn0), (1.034943e-10 * locals.var_t1__blk510_dn2), (1.034943e-10 * locals.var_t1__blk510_dn6), (1.034943e-10 * locals.var_t1__blk510_dn7), (1.034943e-10 * locals.var_t1__blk510_dn10), (1.034943e-10 * locals.var_t1__blk510_dn11), (1.034943e-10 * locals.var_t1__blk510_dn12), (1.034943e-10 * locals.var_t1__blk510_dn17),)
    } else {
        (locals.var_t4__blk513, locals.var_t4__blk513_dn0, locals.var_t4__blk513_dn2, locals.var_t4__blk513_dn6, locals.var_t4__blk513_dn7, locals.var_t4__blk513_dn10, locals.var_t4__blk513_dn11, locals.var_t4__blk513_dn12, locals.var_t4__blk513_dn17,)
    }
};
        locals.var_t4__blk513 = assign17360_e24943;
        locals.var_t4__blk513_dn0 = assign17360_e24943_d_n0;
        locals.var_t4__blk513_dn2 = assign17360_e24943_d_n2;
        locals.var_t4__blk513_dn6 = assign17360_e24943_d_n6;
        locals.var_t4__blk513_dn7 = assign17360_e24943_d_n7;
        locals.var_t4__blk513_dn10 = assign17360_e24943_d_n10;
        locals.var_t4__blk513_dn11 = assign17360_e24943_d_n11;
        locals.var_t4__blk513_dn12 = assign17360_e24943_d_n12;
        locals.var_t4__blk513_dn17 = assign17360_e24943_d_n17;
        locals.var_t4__blk513_rv = 0.0;

        let (assign17370_e24954, assign17370_e24954_d_n0, assign17370_e24954_d_n2, assign17370_e24954_d_n6, assign17370_e24954_d_n7, assign17370_e24954_d_n10, assign17370_e24954_d_n11, assign17370_e24954_d_n12, assign17370_e24954_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17370_e24952: f64 = (1.0 - p.p189);
        (assign17370_e24952, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17370_e24954;
        locals.var_t1__blk510_dn0 = assign17370_e24954_d_n0;
        locals.var_t1__blk510_dn2 = assign17370_e24954_d_n2;
        locals.var_t1__blk510_dn6 = assign17370_e24954_d_n6;
        locals.var_t1__blk510_dn7 = assign17370_e24954_d_n7;
        locals.var_t1__blk510_dn10 = assign17370_e24954_d_n10;
        locals.var_t1__blk510_dn11 = assign17370_e24954_d_n11;
        locals.var_t1__blk510_dn12 = assign17370_e24954_d_n12;
        locals.var_t1__blk510_dn17 = assign17370_e24954_d_n17;
        locals.var_t1__blk510_rv = 0.0;

        let (assign17380_e24971, assign17380_e24971_d_n0, assign17380_e24971_d_n2, assign17380_e24971_d_n6, assign17380_e24971_d_n7, assign17380_e24971_d_n10, assign17380_e24971_d_n11, assign17380_e24971_d_n12, assign17380_e24971_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17380_e24964: f64 = (locals.var_vds + locals.var_ps0);
        let assign17380_e24965: f64 = (p.p189 * assign17380_e24964);
        let assign17380_e24968: f64 = (locals.var_t1__blk510 * locals.var_psl);
        let assign17380_e24969: f64 = (assign17380_e24965 + assign17380_e24968);
        (assign17380_e24969, ((p.p189 * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + ((locals.var_t1__blk510_dn0 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn0))), ((p.p189 * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + ((locals.var_t1__blk510_dn2 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn2))), ((p.p189 * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + ((locals.var_t1__blk510_dn6 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn6))), ((p.p189 * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + ((locals.var_t1__blk510_dn7 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn7))), ((p.p189 * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + ((locals.var_t1__blk510_dn10 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn10))), ((p.p189 * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + ((locals.var_t1__blk510_dn11 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn11))), ((p.p189 * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + ((locals.var_t1__blk510_dn12 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn12))), ((p.p189 * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + ((locals.var_t1__blk510_dn17 * locals.var_psl) + (locals.var_t1__blk510 * locals.var_psl_dn17))),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17380_e24971;
        locals.var_psdl_dn0 = assign17380_e24971_d_n0;
        locals.var_psdl_dn2 = assign17380_e24971_d_n2;
        locals.var_psdl_dn6 = assign17380_e24971_d_n6;
        locals.var_psdl_dn7 = assign17380_e24971_d_n7;
        locals.var_psdl_dn10 = assign17380_e24971_d_n10;
        locals.var_psdl_dn11 = assign17380_e24971_d_n11;
        locals.var_psdl_dn12 = assign17380_e24971_d_n12;
        locals.var_psdl_dn17 = assign17380_e24971_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign17390_e24975: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17390_e24978: f64 = (10.0 * 2.220446049250313e-16);
        let assign17390_e24979: f64 = (assign17390_e24975 - assign17390_e24978);
        let assign17390_e24980: f64 = if locals.var_psdl > assign17390_e24979 { 1.0 } else { 0.0 };
        locals.var_guard526 = assign17390_e24980;
        locals.var_guard526_rv = 0.0;

        let (assign17400_e24997, assign17400_e24997_d_n0, assign17400_e24997_d_n2, assign17400_e24997_d_n6, assign17400_e24997_d_n7, assign17400_e24997_d_n10, assign17400_e24997_d_n11, assign17400_e24997_d_n12, assign17400_e24997_d_n17,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign17400_e24991: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17400_e24994: f64 = (10.0 * 2.220446049250313e-16);
        let assign17400_e24995: f64 = (assign17400_e24991 - assign17400_e24994);
        (assign17400_e24995, (locals.var_ps0_dn0 + locals.var_vdsz_dn0), (locals.var_ps0_dn2 + locals.var_vdsz_dn2), (locals.var_ps0_dn6 + locals.var_vdsz_dn6), (locals.var_ps0_dn7 + locals.var_vdsz_dn7), (locals.var_ps0_dn10 + locals.var_vdsz_dn10), (locals.var_ps0_dn11 + locals.var_vdsz_dn11), (locals.var_ps0_dn12 + locals.var_vdsz_dn12), (locals.var_ps0_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17400_e24997;
        locals.var_psdl_dn0 = assign17400_e24997_d_n0;
        locals.var_psdl_dn2 = assign17400_e24997_d_n2;
        locals.var_psdl_dn6 = assign17400_e24997_d_n6;
        locals.var_psdl_dn7 = assign17400_e24997_d_n7;
        locals.var_psdl_dn10 = assign17400_e24997_d_n10;
        locals.var_psdl_dn11 = assign17400_e24997_d_n11;
        locals.var_psdl_dn12 = assign17400_e24997_d_n12;
        locals.var_psdl_dn17 = assign17400_e24997_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign17410_e25008, assign17410_e25008_d_n0, assign17410_e25008_d_n2, assign17410_e25008_d_n6, assign17410_e25008_d_n7, assign17410_e25008_d_n10, assign17410_e25008_d_n11, assign17410_e25008_d_n12, assign17410_e25008_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17410_e25006: f64 = (locals.var_psdl - locals.var_psl);
        (assign17410_e25006, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn11 - locals.var_psl_dn11), (locals.var_psdl_dn12 - locals.var_psl_dn12), (locals.var_psdl_dn17 - locals.var_psl_dn17),)
    } else {
        (locals.var_t6w__blk516, locals.var_t6w__blk516_dn0, locals.var_t6w__blk516_dn2, locals.var_t6w__blk516_dn6, locals.var_t6w__blk516_dn7, locals.var_t6w__blk516_dn10, locals.var_t6w__blk516_dn11, locals.var_t6w__blk516_dn12, locals.var_t6w__blk516_dn17,)
    }
};
        locals.var_t6w__blk516 = assign17410_e25008;
        locals.var_t6w__blk516_dn0 = assign17410_e25008_d_n0;
        locals.var_t6w__blk516_dn2 = assign17410_e25008_d_n2;
        locals.var_t6w__blk516_dn6 = assign17410_e25008_d_n6;
        locals.var_t6w__blk516_dn7 = assign17410_e25008_d_n7;
        locals.var_t6w__blk516_dn10 = assign17410_e25008_d_n10;
        locals.var_t6w__blk516_dn11 = assign17410_e25008_d_n11;
        locals.var_t6w__blk516_dn12 = assign17410_e25008_d_n12;
        locals.var_t6w__blk516_dn17 = assign17410_e25008_d_n17;
        locals.var_t6w__blk516_rv = 0.0;

        let (assign17420_e25026, assign17420_e25026_d_n0, assign17420_e25026_d_n2, assign17420_e25026_d_n6, assign17420_e25026_d_n7, assign17420_e25026_d_n10, assign17420_e25026_d_n11, assign17420_e25026_d_n12, assign17420_e25026_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17420_e25017: f64 = (locals.var_t6w__blk516 * locals.var_t6w__blk516);
        let assign17420_e25020: f64 = (4.0 * 0.001);
        let assign17420_e25022: f64 = (assign17420_e25020 * 0.001);
        let assign17420_e25023: f64 = (assign17420_e25017 + assign17420_e25022);
        let assign17420_e25024: f64 = (assign17420_e25023).sqrt();
        (assign17420_e25024, (((locals.var_t6w__blk516_dn0 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn0)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn2 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn2)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn6 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn6)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn7 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn7)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn10 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn10)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn11 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn11)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn12 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn12)) / (2.0 * assign17420_e25024)), (((locals.var_t6w__blk516_dn17 * locals.var_t6w__blk516) + (locals.var_t6w__blk516 * locals.var_t6w__blk516_dn17)) / (2.0 * assign17420_e25024)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17420_e25026;
        locals.var_tmf1_dn0 = assign17420_e25026_d_n0;
        locals.var_tmf1_dn2 = assign17420_e25026_d_n2;
        locals.var_tmf1_dn6 = assign17420_e25026_d_n6;
        locals.var_tmf1_dn7 = assign17420_e25026_d_n7;
        locals.var_tmf1_dn10 = assign17420_e25026_d_n10;
        locals.var_tmf1_dn11 = assign17420_e25026_d_n11;
        locals.var_tmf1_dn12 = assign17420_e25026_d_n12;
        locals.var_tmf1_dn17 = assign17420_e25026_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign17430_e25043, assign17430_e25043_d_n0, assign17430_e25043_d_n2, assign17430_e25043_d_n6, assign17430_e25043_d_n7, assign17430_e25043_d_n10, assign17430_e25043_d_n11, assign17430_e25043_d_n12, assign17430_e25043_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17430_e25036: f64 = (locals.var_t6w__blk516 + locals.var_tmf1);
        let assign17430_e25037: f64 = (0.5 * assign17430_e25036);
        let assign17430_e25040: f64 = (1e-10 * 0.001);
        let assign17430_e25041: f64 = (assign17430_e25037 + assign17430_e25040);
        (assign17430_e25041, (0.5 * (locals.var_t6w__blk516_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w__blk516_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w__blk516_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w__blk516_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w__blk516_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w__blk516_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w__blk516_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w__blk516_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk515, locals.var_t6__blk515_dn0, locals.var_t6__blk515_dn2, locals.var_t6__blk515_dn6, locals.var_t6__blk515_dn7, locals.var_t6__blk515_dn10, locals.var_t6__blk515_dn11, locals.var_t6__blk515_dn12, locals.var_t6__blk515_dn17,)
    }
};
        locals.var_t6__blk515 = assign17430_e25043;
        locals.var_t6__blk515_dn0 = assign17430_e25043_d_n0;
        locals.var_t6__blk515_dn2 = assign17430_e25043_d_n2;
        locals.var_t6__blk515_dn6 = assign17430_e25043_d_n6;
        locals.var_t6__blk515_dn7 = assign17430_e25043_d_n7;
        locals.var_t6__blk515_dn10 = assign17430_e25043_d_n10;
        locals.var_t6__blk515_dn11 = assign17430_e25043_d_n11;
        locals.var_t6__blk515_dn12 = assign17430_e25043_d_n12;
        locals.var_t6__blk515_dn17 = assign17430_e25043_d_n17;
        locals.var_t6__blk515_rv = 0.0;

        let assign17440_e25046: f64 = if locals.var_t6__blk515 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign17440_e25046;
        locals.var_guard527_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17450_e25057, assign17450_e25057_d_n0, assign17450_e25057_d_n2, assign17450_e25057_d_n6, assign17450_e25057_d_n7, assign17450_e25057_d_n10, assign17450_e25057_d_n11, assign17450_e25057_d_n12, assign17450_e25057_d_n17,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) && (locals.var_guard527 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk515, locals.var_t6__blk515_dn0, locals.var_t6__blk515_dn2, locals.var_t6__blk515_dn6, locals.var_t6__blk515_dn7, locals.var_t6__blk515_dn10, locals.var_t6__blk515_dn11, locals.var_t6__blk515_dn12, locals.var_t6__blk515_dn17,)
    }
};
        locals.var_t6__blk515 = assign17450_e25057;
        locals.var_t6__blk515_dn0 = assign17450_e25057_d_n0;
        locals.var_t6__blk515_dn2 = assign17450_e25057_d_n2;
        locals.var_t6__blk515_dn6 = assign17450_e25057_d_n6;
        locals.var_t6__blk515_dn7 = assign17450_e25057_d_n7;
        locals.var_t6__blk515_dn10 = assign17450_e25057_d_n10;
        locals.var_t6__blk515_dn11 = assign17450_e25057_d_n11;
        locals.var_t6__blk515_dn12 = assign17450_e25057_d_n12;
        locals.var_t6__blk515_dn17 = assign17450_e25057_d_n17;
        locals.var_t6__blk515_rv = 0.0;

        let (assign17460_e25068, assign17460_e25068_d_n0, assign17460_e25068_d_n2, assign17460_e25068_d_n6, assign17460_e25068_d_n7, assign17460_e25068_d_n10, assign17460_e25068_d_n11, assign17460_e25068_d_n12, assign17460_e25068_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17460_e25066: f64 = (locals.var_beta * locals.var_qn0);
        (assign17460_e25066, (locals.var_beta * locals.var_qn0_dn0), (locals.var_beta * locals.var_qn0_dn2), (locals.var_beta * locals.var_qn0_dn6), (locals.var_beta * locals.var_qn0_dn7), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), (locals.var_beta * locals.var_qn0_dn11), (locals.var_beta * locals.var_qn0_dn12), (locals.var_beta * locals.var_qn0_dn17),)
    } else {
        (locals.var_t3__blk512, locals.var_t3__blk512_dn0, locals.var_t3__blk512_dn2, locals.var_t3__blk512_dn6, locals.var_t3__blk512_dn7, locals.var_t3__blk512_dn10, locals.var_t3__blk512_dn11, locals.var_t3__blk512_dn12, locals.var_t3__blk512_dn17,)
    }
};
        locals.var_t3__blk512 = assign17460_e25068;
        locals.var_t3__blk512_dn0 = assign17460_e25068_d_n0;
        locals.var_t3__blk512_dn2 = assign17460_e25068_d_n2;
        locals.var_t3__blk512_dn6 = assign17460_e25068_d_n6;
        locals.var_t3__blk512_dn7 = assign17460_e25068_d_n7;
        locals.var_t3__blk512_dn10 = assign17460_e25068_d_n10;
        locals.var_t3__blk512_dn11 = assign17460_e25068_d_n11;
        locals.var_t3__blk512_dn12 = assign17460_e25068_d_n12;
        locals.var_t3__blk512_dn17 = assign17460_e25068_d_n17;
        locals.var_t3__blk512_rv = 0.0;

        let (assign17470_e25079, assign17470_e25079_d_n0, assign17470_e25079_d_n2, assign17470_e25079_d_n6, assign17470_e25079_d_n7, assign17470_e25079_d_n10, assign17470_e25079_d_n11, assign17470_e25079_d_n12, assign17470_e25079_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17470_e25077: f64 = (1.0 / locals.var_t3__blk512);
        (assign17470_e25077, (-(locals.var_t3__blk512_dn0 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn2 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn6 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn7 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn10 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn11 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn12 / (locals.var_t3__blk512 * locals.var_t3__blk512))), (-(locals.var_t3__blk512_dn17 / (locals.var_t3__blk512 * locals.var_t3__blk512))),)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17470_e25079;
        locals.var_t1__blk510_dn0 = assign17470_e25079_d_n0;
        locals.var_t1__blk510_dn2 = assign17470_e25079_d_n2;
        locals.var_t1__blk510_dn6 = assign17470_e25079_d_n6;
        locals.var_t1__blk510_dn7 = assign17470_e25079_d_n7;
        locals.var_t1__blk510_dn10 = assign17470_e25079_d_n10;
        locals.var_t1__blk510_dn11 = assign17470_e25079_d_n11;
        locals.var_t1__blk510_dn12 = assign17470_e25079_d_n12;
        locals.var_t1__blk510_dn17 = assign17470_e25079_d_n17;
        locals.var_t1__blk510_rv = 0.0;

        let (assign17480_e25090, assign17480_e25090_d_n0, assign17480_e25090_d_n2, assign17480_e25090_d_n6, assign17480_e25090_d_n7, assign17480_e25090_d_n10, assign17480_e25090_d_n11, assign17480_e25090_d_n12, assign17480_e25090_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17480_e25088: f64 = (locals.var_idd * locals.var_t1__blk510);
        (assign17480_e25088, ((locals.var_idd_dn0 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn0)), ((locals.var_idd_dn2 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn2)), ((locals.var_idd_dn6 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn6)), ((locals.var_idd_dn7 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn7)), ((locals.var_idd_dn10 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn10)), ((locals.var_idd_dn11 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn11)), ((locals.var_idd_dn12 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn12)), ((locals.var_idd_dn17 * locals.var_t1__blk510) + (locals.var_idd * locals.var_t1__blk510_dn17)),)
    } else {
        (locals.var_t5__blk514, locals.var_t5__blk514_dn0, locals.var_t5__blk514_dn2, locals.var_t5__blk514_dn6, locals.var_t5__blk514_dn7, locals.var_t5__blk514_dn10, locals.var_t5__blk514_dn11, locals.var_t5__blk514_dn12, locals.var_t5__blk514_dn17,)
    }
};
        locals.var_t5__blk514 = assign17480_e25090;
        locals.var_t5__blk514_dn0 = assign17480_e25090_d_n0;
        locals.var_t5__blk514_dn2 = assign17480_e25090_d_n2;
        locals.var_t5__blk514_dn6 = assign17480_e25090_d_n6;
        locals.var_t5__blk514_dn7 = assign17480_e25090_d_n7;
        locals.var_t5__blk514_dn10 = assign17480_e25090_d_n10;
        locals.var_t5__blk514_dn11 = assign17480_e25090_d_n11;
        locals.var_t5__blk514_dn12 = assign17480_e25090_d_n12;
        locals.var_t5__blk514_dn17 = assign17480_e25090_d_n17;
        locals.var_t5__blk514_rv = 0.0;

        let assign17490_e25093: f64 = if locals.var_t5__blk514 < locals.var_beta_inv { 1.0 } else { 0.0 };
        locals.var_guard528 = assign17490_e25093;
        locals.var_guard528_rv = 0.0;

        let (assign17500_e25104, assign17500_e25104_d_n0, assign17500_e25104_d_n2, assign17500_e25104_d_n6, assign17500_e25104_d_n7, assign17500_e25104_d_n10, assign17500_e25104_d_n11, assign17500_e25104_d_n12, assign17500_e25104_d_n17,) = {
    if ((((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) && (locals.var_guard528 != 0.0)) {
        (locals.var_beta_inv, 0.0, 0.0, 0.0, 0.0, locals.var_beta_inv_dn10, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk514, locals.var_t5__blk514_dn0, locals.var_t5__blk514_dn2, locals.var_t5__blk514_dn6, locals.var_t5__blk514_dn7, locals.var_t5__blk514_dn10, locals.var_t5__blk514_dn11, locals.var_t5__blk514_dn12, locals.var_t5__blk514_dn17,)
    }
};
        locals.var_t5__blk514 = assign17500_e25104;
        locals.var_t5__blk514_dn0 = assign17500_e25104_d_n0;
        locals.var_t5__blk514_dn2 = assign17500_e25104_d_n2;
        locals.var_t5__blk514_dn6 = assign17500_e25104_d_n6;
        locals.var_t5__blk514_dn7 = assign17500_e25104_d_n7;
        locals.var_t5__blk514_dn10 = assign17500_e25104_d_n10;
        locals.var_t5__blk514_dn11 = assign17500_e25104_d_n11;
        locals.var_t5__blk514_dn12 = assign17500_e25104_d_n12;
        locals.var_t5__blk514_dn17 = assign17500_e25104_d_n17;
        locals.var_t5__blk514_rv = 0.0;

        let (assign17510_e25115, assign17510_e25115_d_n0, assign17510_e25115_d_n2, assign17510_e25115_d_n6, assign17510_e25115_d_n7, assign17510_e25115_d_n10, assign17510_e25115_d_n11, assign17510_e25115_d_n12, assign17510_e25115_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17510_e25113: f64 = (locals.var_q_nsub / 1.034943e-10);
        (assign17510_e25113, (locals.var_q_nsub_dn0 / 1.034943e-10), (locals.var_q_nsub_dn2 / 1.034943e-10), (locals.var_q_nsub_dn6 / 1.034943e-10), (locals.var_q_nsub_dn7 / 1.034943e-10), (locals.var_q_nsub_dn10 / 1.034943e-10), (locals.var_q_nsub_dn11 / 1.034943e-10), (locals.var_q_nsub_dn12 / 1.034943e-10), (locals.var_q_nsub_dn17 / 1.034943e-10),)
    } else {
        (locals.var_t10__blk520, locals.var_t10__blk520_dn0, locals.var_t10__blk520_dn2, locals.var_t10__blk520_dn6, locals.var_t10__blk520_dn7, locals.var_t10__blk520_dn10, locals.var_t10__blk520_dn11, locals.var_t10__blk520_dn12, locals.var_t10__blk520_dn17,)
    }
};
        locals.var_t10__blk520 = assign17510_e25115;
        locals.var_t10__blk520_dn0 = assign17510_e25115_d_n0;
        locals.var_t10__blk520_dn2 = assign17510_e25115_d_n2;
        locals.var_t10__blk520_dn6 = assign17510_e25115_d_n6;
        locals.var_t10__blk520_dn7 = assign17510_e25115_d_n7;
        locals.var_t10__blk520_dn10 = assign17510_e25115_d_n10;
        locals.var_t10__blk520_dn11 = assign17510_e25115_d_n11;
        locals.var_t10__blk520_dn12 = assign17510_e25115_d_n12;
        locals.var_t10__blk520_dn17 = assign17510_e25115_d_n17;
        locals.var_t10__blk520_rv = 0.0;

        let (assign17520_e25126, assign17520_e25126_d_n0, assign17520_e25126_d_n2, assign17520_e25126_d_n6, assign17520_e25126_d_n7, assign17520_e25126_d_n10, assign17520_e25126_d_n11, assign17520_e25126_d_n12, assign17520_e25126_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17520_e25124: f64 = (100000.0 * 10000.0);
        (assign17520_e25124, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk510, locals.var_t1__blk510_dn0, locals.var_t1__blk510_dn2, locals.var_t1__blk510_dn6, locals.var_t1__blk510_dn7, locals.var_t1__blk510_dn10, locals.var_t1__blk510_dn11, locals.var_t1__blk510_dn12, locals.var_t1__blk510_dn17,)
    }
};
        locals.var_t1__blk510 = assign17520_e25126;
        locals.var_t1__blk510_dn0 = assign17520_e25126_d_n0;
        locals.var_t1__blk510_dn2 = assign17520_e25126_d_n2;
        locals.var_t1__blk510_dn6 = assign17520_e25126_d_n6;
        locals.var_t1__blk510_dn7 = assign17520_e25126_d_n7;
        locals.var_t1__blk510_dn10 = assign17520_e25126_d_n10;
        locals.var_t1__blk510_dn11 = assign17520_e25126_d_n11;
        locals.var_t1__blk510_dn12 = assign17520_e25126_d_n12;
        locals.var_t1__blk510_dn17 = assign17520_e25126_d_n17;
        locals.var_t1__blk510_rv = 0.0;

        let (assign17530_e25137, assign17530_e25137_d_n0, assign17530_e25137_d_n2, assign17530_e25137_d_n6, assign17530_e25137_d_n7, assign17530_e25137_d_n10, assign17530_e25137_d_n11, assign17530_e25137_d_n12, assign17530_e25137_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17530_e25135: f64 = (1.0 / locals.var_leff);
        (assign17530_e25135, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk511, locals.var_t2__blk511_dn0, locals.var_t2__blk511_dn2, locals.var_t2__blk511_dn6, locals.var_t2__blk511_dn7, locals.var_t2__blk511_dn10, locals.var_t2__blk511_dn11, locals.var_t2__blk511_dn12, locals.var_t2__blk511_dn17,)
    }
};
        locals.var_t2__blk511 = assign17530_e25137;
        locals.var_t2__blk511_dn0 = assign17530_e25137_d_n0;
        locals.var_t2__blk511_dn2 = assign17530_e25137_d_n2;
        locals.var_t2__blk511_dn6 = assign17530_e25137_d_n6;
        locals.var_t2__blk511_dn7 = assign17530_e25137_d_n7;
        locals.var_t2__blk511_dn10 = assign17530_e25137_d_n10;
        locals.var_t2__blk511_dn11 = assign17530_e25137_d_n11;
        locals.var_t2__blk511_dn12 = assign17530_e25137_d_n12;
        locals.var_t2__blk511_dn17 = assign17530_e25137_d_n17;
        locals.var_t2__blk511_rv = 0.0;

        let (assign17540_e25162, assign17540_e25162_d_n0, assign17540_e25162_d_n2, assign17540_e25162_d_n6, assign17540_e25162_d_n7, assign17540_e25162_d_n10, assign17540_e25162_d_n11, assign17540_e25162_d_n12, assign17540_e25162_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17540_e25146: f64 = (2.0 * locals.var_t5__blk514);
        let assign17540_e25149: f64 = (2.0 * locals.var_t10__blk520);
        let assign17540_e25151: f64 = (assign17540_e25149 * locals.var_t6__blk515);
        let assign17540_e25153: f64 = (assign17540_e25151 * locals.var_t4__blk513);
        let assign17540_e25154: f64 = (assign17540_e25146 + assign17540_e25153);
        let assign17540_e25157: f64 = (locals.var_t1__blk510 * locals.var_t4__blk513);
        let assign17540_e25158: f64 = (assign17540_e25154 + assign17540_e25157);
        let assign17540_e25160: f64 = (assign17540_e25158 * locals.var_t2__blk511);
        (assign17540_e25160, (((((2.0 * locals.var_t5__blk514_dn0) + (((((2.0 * locals.var_t10__blk520_dn0) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn0)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn0))) + ((locals.var_t1__blk510_dn0 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn0))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn0)), (((((2.0 * locals.var_t5__blk514_dn2) + (((((2.0 * locals.var_t10__blk520_dn2) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn2)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn2))) + ((locals.var_t1__blk510_dn2 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn2))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn2)), (((((2.0 * locals.var_t5__blk514_dn6) + (((((2.0 * locals.var_t10__blk520_dn6) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn6)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn6))) + ((locals.var_t1__blk510_dn6 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn6))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn6)), (((((2.0 * locals.var_t5__blk514_dn7) + (((((2.0 * locals.var_t10__blk520_dn7) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn7)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn7))) + ((locals.var_t1__blk510_dn7 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn7))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn7)), (((((2.0 * locals.var_t5__blk514_dn10) + (((((2.0 * locals.var_t10__blk520_dn10) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn10)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn10))) + ((locals.var_t1__blk510_dn10 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn10))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn10)), (((((2.0 * locals.var_t5__blk514_dn11) + (((((2.0 * locals.var_t10__blk520_dn11) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn11)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn11))) + ((locals.var_t1__blk510_dn11 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn11))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn11)), (((((2.0 * locals.var_t5__blk514_dn12) + (((((2.0 * locals.var_t10__blk520_dn12) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn12)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn12))) + ((locals.var_t1__blk510_dn12 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn12))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn12)), (((((2.0 * locals.var_t5__blk514_dn17) + (((((2.0 * locals.var_t10__blk520_dn17) * locals.var_t6__blk515) + (assign17540_e25149 * locals.var_t6__blk515_dn17)) * locals.var_t4__blk513) + (assign17540_e25151 * locals.var_t4__blk513_dn17))) + ((locals.var_t1__blk510_dn17 * locals.var_t4__blk513) + (locals.var_t1__blk510 * locals.var_t4__blk513_dn17))) * locals.var_t2__blk511) + (assign17540_e25158 * locals.var_t2__blk511_dn17)),)
    } else {
        (locals.var_t11w, locals.var_t11w_dn0, locals.var_t11w_dn2, locals.var_t11w_dn6, locals.var_t11w_dn7, locals.var_t11w_dn10, locals.var_t11w_dn11, locals.var_t11w_dn12, locals.var_t11w_dn17,)
    }
};
        locals.var_t11w = assign17540_e25162;
        locals.var_t11w_dn0 = assign17540_e25162_d_n0;
        locals.var_t11w_dn2 = assign17540_e25162_d_n2;
        locals.var_t11w_dn6 = assign17540_e25162_d_n6;
        locals.var_t11w_dn7 = assign17540_e25162_d_n7;
        locals.var_t11w_dn10 = assign17540_e25162_d_n10;
        locals.var_t11w_dn11 = assign17540_e25162_d_n11;
        locals.var_t11w_dn12 = assign17540_e25162_d_n12;
        locals.var_t11w_dn17 = assign17540_e25162_d_n17;
        locals.var_t11w_rv = 0.0;

        let (assign17550_e25173, assign17550_e25173_d_n0, assign17550_e25173_d_n2, assign17550_e25173_d_n6, assign17550_e25173_d_n7, assign17550_e25173_d_n10, assign17550_e25173_d_n11, assign17550_e25173_d_n12, assign17550_e25173_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17550_e25171: f64 = (locals.var_t11w * locals.var_t4__blk513);
        (assign17550_e25171, ((locals.var_t11w_dn0 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn0)), ((locals.var_t11w_dn2 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn2)), ((locals.var_t11w_dn6 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn6)), ((locals.var_t11w_dn7 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn7)), ((locals.var_t11w_dn10 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn10)), ((locals.var_t11w_dn11 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn11)), ((locals.var_t11w_dn12 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn12)), ((locals.var_t11w_dn17 * locals.var_t4__blk513) + (locals.var_t11w * locals.var_t4__blk513_dn17)),)
    } else {
        (locals.var_t7__blk517, locals.var_t7__blk517_dn0, locals.var_t7__blk517_dn2, locals.var_t7__blk517_dn6, locals.var_t7__blk517_dn7, locals.var_t7__blk517_dn10, locals.var_t7__blk517_dn11, locals.var_t7__blk517_dn12, locals.var_t7__blk517_dn17,)
    }
};
        locals.var_t7__blk517 = assign17550_e25173;
        locals.var_t7__blk517_dn0 = assign17550_e25173_d_n0;
        locals.var_t7__blk517_dn2 = assign17550_e25173_d_n2;
        locals.var_t7__blk517_dn6 = assign17550_e25173_d_n6;
        locals.var_t7__blk517_dn7 = assign17550_e25173_d_n7;
        locals.var_t7__blk517_dn10 = assign17550_e25173_d_n10;
        locals.var_t7__blk517_dn11 = assign17550_e25173_d_n11;
        locals.var_t7__blk517_dn12 = assign17550_e25173_d_n12;
        locals.var_t7__blk517_dn17 = assign17550_e25173_d_n17;
        locals.var_t7__blk517_rv = 0.0;

        let (assign17560_e25190, assign17560_e25190_d_n0, assign17560_e25190_d_n2, assign17560_e25190_d_n6, assign17560_e25190_d_n7, assign17560_e25190_d_n10, assign17560_e25190_d_n11, assign17560_e25190_d_n12, assign17560_e25190_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17560_e25183: f64 = (2.0 * locals.var_t10__blk520);
        let assign17560_e25185: f64 = (assign17560_e25183 * locals.var_t6__blk515);
        let assign17560_e25187: f64 = (assign17560_e25185 + locals.var_t1__blk510);
        let assign17560_e25188: f64 = (4.0 * assign17560_e25187);
        (assign17560_e25188, (4.0 * ((((2.0 * locals.var_t10__blk520_dn0) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn0)) + locals.var_t1__blk510_dn0)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn2) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn2)) + locals.var_t1__blk510_dn2)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn6) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn6)) + locals.var_t1__blk510_dn6)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn7) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn7)) + locals.var_t1__blk510_dn7)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn10) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn10)) + locals.var_t1__blk510_dn10)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn11) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn11)) + locals.var_t1__blk510_dn11)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn12) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn12)) + locals.var_t1__blk510_dn12)), (4.0 * ((((2.0 * locals.var_t10__blk520_dn17) * locals.var_t6__blk515) + (assign17560_e25183 * locals.var_t6__blk515_dn17)) + locals.var_t1__blk510_dn17)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12, locals.var_t11_dn17,)
    }
};
        locals.var_t11 = assign17560_e25190;
        locals.var_t11_dn0 = assign17560_e25190_d_n0;
        locals.var_t11_dn2 = assign17560_e25190_d_n2;
        locals.var_t11_dn6 = assign17560_e25190_d_n6;
        locals.var_t11_dn7 = assign17560_e25190_d_n7;
        locals.var_t11_dn10 = assign17560_e25190_d_n10;
        locals.var_t11_dn11 = assign17560_e25190_d_n11;
        locals.var_t11_dn12 = assign17560_e25190_d_n12;
        locals.var_t11_dn17 = assign17560_e25190_d_n17;
        locals.var_t11_rv = 0.0;

        let (assign17570_e25203, assign17570_e25203_d_n0, assign17570_e25203_d_n2, assign17570_e25203_d_n6, assign17570_e25203_d_n7, assign17570_e25203_d_n10, assign17570_e25203_d_n11, assign17570_e25203_d_n12, assign17570_e25203_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17570_e25199: f64 = (locals.var_t11 * locals.var_t4__blk513);
        let assign17570_e25201: f64 = (assign17570_e25199 * locals.var_t4__blk513);
        (assign17570_e25201, ((((locals.var_t11_dn0 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn0)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn0)), ((((locals.var_t11_dn2 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn2)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn2)), ((((locals.var_t11_dn6 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn6)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn6)), ((((locals.var_t11_dn7 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn7)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn7)), ((((locals.var_t11_dn10 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn10)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn10)), ((((locals.var_t11_dn11 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn11)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn11)), ((((locals.var_t11_dn12 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn12)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn12)), ((((locals.var_t11_dn17 * locals.var_t4__blk513) + (locals.var_t11 * locals.var_t4__blk513_dn17)) * locals.var_t4__blk513) + (assign17570_e25199 * locals.var_t4__blk513_dn17)),)
    } else {
        (locals.var_t8__blk518, locals.var_t8__blk518_dn0, locals.var_t8__blk518_dn2, locals.var_t8__blk518_dn6, locals.var_t8__blk518_dn7, locals.var_t8__blk518_dn10, locals.var_t8__blk518_dn11, locals.var_t8__blk518_dn12, locals.var_t8__blk518_dn17,)
    }
};
        locals.var_t8__blk518 = assign17570_e25203;
        locals.var_t8__blk518_dn0 = assign17570_e25203_d_n0;
        locals.var_t8__blk518_dn2 = assign17570_e25203_d_n2;
        locals.var_t8__blk518_dn6 = assign17570_e25203_d_n6;
        locals.var_t8__blk518_dn7 = assign17570_e25203_d_n7;
        locals.var_t8__blk518_dn10 = assign17570_e25203_d_n10;
        locals.var_t8__blk518_dn11 = assign17570_e25203_d_n11;
        locals.var_t8__blk518_dn12 = assign17570_e25203_d_n12;
        locals.var_t8__blk518_dn17 = assign17570_e25203_d_n17;
        locals.var_t8__blk518_rv = 0.0;

        let (assign17580_e25217, assign17580_e25217_d_n0, assign17580_e25217_d_n2, assign17580_e25217_d_n6, assign17580_e25217_d_n7, assign17580_e25217_d_n10, assign17580_e25217_d_n11, assign17580_e25217_d_n12, assign17580_e25217_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17580_e25212: f64 = (locals.var_t7__blk517 * locals.var_t7__blk517);
        let assign17580_e25214: f64 = (assign17580_e25212 + locals.var_t8__blk518);
        let assign17580_e25215: f64 = (assign17580_e25214).sqrt();
        (assign17580_e25215, ((((locals.var_t7__blk517_dn0 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn0)) + locals.var_t8__blk518_dn0) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn2 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn2)) + locals.var_t8__blk518_dn2) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn6 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn6)) + locals.var_t8__blk518_dn6) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn7 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn7)) + locals.var_t8__blk518_dn7) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn10 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn10)) + locals.var_t8__blk518_dn10) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn11 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn11)) + locals.var_t8__blk518_dn11) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn12 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn12)) + locals.var_t8__blk518_dn12) / (2.0 * assign17580_e25215)), ((((locals.var_t7__blk517_dn17 * locals.var_t7__blk517) + (locals.var_t7__blk517 * locals.var_t7__blk517_dn17)) + locals.var_t8__blk518_dn17) / (2.0 * assign17580_e25215)),)
    } else {
        (locals.var_t9__blk519, locals.var_t9__blk519_dn0, locals.var_t9__blk519_dn2, locals.var_t9__blk519_dn6, locals.var_t9__blk519_dn7, locals.var_t9__blk519_dn10, locals.var_t9__blk519_dn11, locals.var_t9__blk519_dn12, locals.var_t9__blk519_dn17,)
    }
};
        locals.var_t9__blk519 = assign17580_e25217;
        locals.var_t9__blk519_dn0 = assign17580_e25217_d_n0;
        locals.var_t9__blk519_dn2 = assign17580_e25217_d_n2;
        locals.var_t9__blk519_dn6 = assign17580_e25217_d_n6;
        locals.var_t9__blk519_dn7 = assign17580_e25217_d_n7;
        locals.var_t9__blk519_dn10 = assign17580_e25217_d_n10;
        locals.var_t9__blk519_dn11 = assign17580_e25217_d_n11;
        locals.var_t9__blk519_dn12 = assign17580_e25217_d_n12;
        locals.var_t9__blk519_dn17 = assign17580_e25217_d_n17;
        locals.var_t9__blk519_rv = 0.0;

        let (assign17590_e25233, assign17590_e25233_d_n0, assign17590_e25233_d_n2, assign17590_e25233_d_n6, assign17590_e25233_d_n7, assign17590_e25233_d_n10, assign17590_e25233_d_n11, assign17590_e25233_d_n12, assign17590_e25233_d_n17,) = {
    if (((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) && (locals.var_guard524 == 0.0)) {
        let assign17590_e25227: f64 = (-locals.var_t7__blk517);
        let assign17590_e25229: f64 = (assign17590_e25227 + locals.var_t9__blk519);
        let assign17590_e25230: f64 = (0.5 * assign17590_e25229);
        let assign17590_e25231: f64 = (locals.var_fmdvds * assign17590_e25230);
        (assign17590_e25231, ((locals.var_fmdvds_dn0 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn0) + locals.var_t9__blk519_dn0)))), ((locals.var_fmdvds_dn2 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn2) + locals.var_t9__blk519_dn2)))), ((locals.var_fmdvds_dn6 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn6) + locals.var_t9__blk519_dn6)))), ((locals.var_fmdvds_dn7 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn7) + locals.var_t9__blk519_dn7)))), ((locals.var_fmdvds_dn10 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn10) + locals.var_t9__blk519_dn10)))), ((locals.var_fmdvds_dn11 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn11) + locals.var_t9__blk519_dn11)))), ((locals.var_fmdvds_dn12 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn12) + locals.var_t9__blk519_dn12)))), ((locals.var_fmdvds_dn17 * assign17590_e25230) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk517_dn17) + locals.var_t9__blk519_dn17)))),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17590_e25233;
        locals.var_lred_dn0 = assign17590_e25233_d_n0;
        locals.var_lred_dn2 = assign17590_e25233_d_n2;
        locals.var_lred_dn6 = assign17590_e25233_d_n6;
        locals.var_lred_dn7 = assign17590_e25233_d_n7;
        locals.var_lred_dn10 = assign17590_e25233_d_n10;
        locals.var_lred_dn11 = assign17590_e25233_d_n11;
        locals.var_lred_dn12 = assign17590_e25233_d_n12;
        locals.var_lred_dn17 = assign17590_e25233_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17600_e25241, assign17600_e25241_d_n0, assign17600_e25241_d_n2, assign17600_e25241_d_n6, assign17600_e25241_d_n7, assign17600_e25241_d_n10, assign17600_e25241_d_n11, assign17600_e25241_d_n12, assign17600_e25241_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard508 != 0.0)) {
        let assign17600_e25239: f64 = (locals.var_lred * locals.var_clmmod);
        (assign17600_e25239, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn12 * locals.var_clmmod), (locals.var_lred_dn17 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17600_e25241;
        locals.var_lred_dn0 = assign17600_e25241_d_n0;
        locals.var_lred_dn2 = assign17600_e25241_d_n2;
        locals.var_lred_dn6 = assign17600_e25241_d_n6;
        locals.var_lred_dn7 = assign17600_e25241_d_n7;
        locals.var_lred_dn10 = assign17600_e25241_d_n10;
        locals.var_lred_dn11 = assign17600_e25241_d_n11;
        locals.var_lred_dn12 = assign17600_e25241_d_n12;
        locals.var_lred_dn17 = assign17600_e25241_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17610_e25247, assign17610_e25247_d_n0, assign17610_e25247_d_n2, assign17610_e25247_d_n6, assign17610_e25247_d_n7, assign17610_e25247_d_n10, assign17610_e25247_d_n11, assign17610_e25247_d_n12, assign17610_e25247_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17610_e25245: f64 = (locals.var_leff - locals.var_lred);
        (assign17610_e25245, (-locals.var_lred_dn0), (-locals.var_lred_dn2), (-locals.var_lred_dn6), (-locals.var_lred_dn7), (-locals.var_lred_dn10), (-locals.var_lred_dn11), (-locals.var_lred_dn12), (-locals.var_lred_dn17),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    }
};
        locals.var_lch = assign17610_e25247;
        locals.var_lch_dn0 = assign17610_e25247_d_n0;
        locals.var_lch_dn2 = assign17610_e25247_d_n2;
        locals.var_lch_dn6 = assign17610_e25247_d_n6;
        locals.var_lch_dn7 = assign17610_e25247_d_n7;
        locals.var_lch_dn10 = assign17610_e25247_d_n10;
        locals.var_lch_dn11 = assign17610_e25247_d_n11;
        locals.var_lch_dn12 = assign17610_e25247_d_n12;
        locals.var_lch_dn17 = assign17610_e25247_d_n17;
        locals.var_lch_rv = 0.0;

        let assign17630_e25256: f64 = if locals.var_lch < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard529 = assign17630_e25256;
        locals.var_guard529_rv = 0.0;

        let (assign17640_e25262, assign17640_e25262_d_n0, assign17640_e25262_d_n2, assign17640_e25262_d_n6, assign17640_e25262_d_n7, assign17640_e25262_d_n10, assign17640_e25262_d_n11, assign17640_e25262_d_n12, assign17640_e25262_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard529 != 0.0)) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    }
};
        locals.var_lch = assign17640_e25262;
        locals.var_lch_dn0 = assign17640_e25262_d_n0;
        locals.var_lch_dn2 = assign17640_e25262_d_n2;
        locals.var_lch_dn6 = assign17640_e25262_d_n6;
        locals.var_lch_dn7 = assign17640_e25262_d_n7;
        locals.var_lch_dn10 = assign17640_e25262_d_n10;
        locals.var_lch_dn11 = assign17640_e25262_d_n11;
        locals.var_lch_dn12 = assign17640_e25262_d_n12;
        locals.var_lch_dn17 = assign17640_e25262_d_n17;
        locals.var_lch_rv = 0.0;

        let (assign17650_e25269, assign17650_e25269_d_n0, assign17650_e25269_d_n2, assign17650_e25269_d_n6, assign17650_e25269_d_n7, assign17650_e25269_d_n10, assign17650_e25269_d_n11, assign17650_e25269_d_n12, assign17650_e25269_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17650_e25265: f64 = (-locals.var_weffcv_nf);
        let assign17650_e25267: f64 = (assign17650_e25265 * locals.var_leff_cv);
        (assign17650_e25267, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign17650_e25269;
        locals.var_t1_dn0 = assign17650_e25269_d_n0;
        locals.var_t1_dn2 = assign17650_e25269_d_n2;
        locals.var_t1_dn6 = assign17650_e25269_d_n6;
        locals.var_t1_dn7 = assign17650_e25269_d_n7;
        locals.var_t1_dn10 = assign17650_e25269_d_n10;
        locals.var_t1_dn11 = assign17650_e25269_d_n11;
        locals.var_t1_dn12 = assign17650_e25269_d_n12;
        locals.var_t1_dn17 = assign17650_e25269_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign17660_e25275, assign17660_e25275_d_n0, assign17660_e25275_d_n2, assign17660_e25275_d_n6, assign17660_e25275_d_n7, assign17660_e25275_d_n10, assign17660_e25275_d_n11, assign17660_e25275_d_n12, assign17660_e25275_d_n13, assign17660_e25275_d_n15, assign17660_e25275_d_n16, assign17660_e25275_d_n17, assign17660_e25275_d_n18,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17660_e25273: f64 = (locals.var_t1 * locals.var_qbu);
        (assign17660_e25273, ((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0)), ((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2)), ((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6)), ((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7)), ((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10)), ((locals.var_t1_dn11 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn11)), ((locals.var_t1_dn12 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t1_dn17 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign17660_e25275;
        locals.var_qb_dn0 = assign17660_e25275_d_n0;
        locals.var_qb_dn2 = assign17660_e25275_d_n2;
        locals.var_qb_dn6 = assign17660_e25275_d_n6;
        locals.var_qb_dn7 = assign17660_e25275_d_n7;
        locals.var_qb_dn10 = assign17660_e25275_d_n10;
        locals.var_qb_dn11 = assign17660_e25275_d_n11;
        locals.var_qb_dn12 = assign17660_e25275_d_n12;
        locals.var_qb_dn13 = assign17660_e25275_d_n13;
        locals.var_qb_dn15 = assign17660_e25275_d_n15;
        locals.var_qb_dn16 = assign17660_e25275_d_n16;
        locals.var_qb_dn17 = assign17660_e25275_d_n17;
        locals.var_qb_dn18 = assign17660_e25275_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign17670_e25281, assign17670_e25281_d_n0, assign17670_e25281_d_n2, assign17670_e25281_d_n6, assign17670_e25281_d_n7, assign17670_e25281_d_n10, assign17670_e25281_d_n11, assign17670_e25281_d_n12, assign17670_e25281_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17670_e25279: f64 = (locals.var_t1 * locals.var_qiu);
        (assign17670_e25279, ((locals.var_t1_dn0 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn0)), ((locals.var_t1_dn2 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn2)), ((locals.var_t1_dn6 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn6)), ((locals.var_t1_dn7 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn7)), ((locals.var_t1_dn10 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn10)), ((locals.var_t1_dn11 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn11)), ((locals.var_t1_dn12 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn12)), ((locals.var_t1_dn17 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn17)),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign17670_e25281;
        locals.var_qi_dn0 = assign17670_e25281_d_n0;
        locals.var_qi_dn2 = assign17670_e25281_d_n2;
        locals.var_qi_dn6 = assign17670_e25281_d_n6;
        locals.var_qi_dn7 = assign17670_e25281_d_n7;
        locals.var_qi_dn10 = assign17670_e25281_d_n10;
        locals.var_qi_dn11 = assign17670_e25281_d_n11;
        locals.var_qi_dn12 = assign17670_e25281_d_n12;
        locals.var_qi_dn17 = assign17670_e25281_d_n17;
        locals.var_qi_rv = 0.0;

        let (assign17680_e25287, assign17680_e25287_d_n0, assign17680_e25287_d_n2, assign17680_e25287_d_n6, assign17680_e25287_d_n7, assign17680_e25287_d_n10, assign17680_e25287_d_n11, assign17680_e25287_d_n12, assign17680_e25287_d_n13, assign17680_e25287_d_n15, assign17680_e25287_d_n16, assign17680_e25287_d_n17, assign17680_e25287_d_n18,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17680_e25285: f64 = (locals.var_qi * locals.var_qdrat);
        (assign17680_e25285, ((locals.var_qi_dn0 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn0)), ((locals.var_qi_dn2 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn2)), ((locals.var_qi_dn6 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn6)), ((locals.var_qi_dn7 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn7)), ((locals.var_qi_dn10 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn10)), ((locals.var_qi_dn11 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn11)), ((locals.var_qi_dn12 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn12)), 0.0, 0.0, 0.0, ((locals.var_qi_dn17 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn17)), 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign17680_e25287;
        locals.var_qd_dn0 = assign17680_e25287_d_n0;
        locals.var_qd_dn2 = assign17680_e25287_d_n2;
        locals.var_qd_dn6 = assign17680_e25287_d_n6;
        locals.var_qd_dn7 = assign17680_e25287_d_n7;
        locals.var_qd_dn10 = assign17680_e25287_d_n10;
        locals.var_qd_dn11 = assign17680_e25287_d_n11;
        locals.var_qd_dn12 = assign17680_e25287_d_n12;
        locals.var_qd_dn13 = assign17680_e25287_d_n13;
        locals.var_qd_dn15 = assign17680_e25287_d_n15;
        locals.var_qd_dn16 = assign17680_e25287_d_n16;
        locals.var_qd_dn17 = assign17680_e25287_d_n17;
        locals.var_qd_dn18 = assign17680_e25287_d_n18;
        locals.var_qd_rv = 0.0;

        let assign17690_e25290: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard530 = assign17690_e25290;
        locals.var_guard530_rv = 0.0;

        let (assign17700_e25298, assign17700_e25298_d_n0, assign17700_e25298_d_n2, assign17700_e25298_d_n6, assign17700_e25298_d_n7, assign17700_e25298_d_n10, assign17700_e25298_d_n11, assign17700_e25298_d_n12, assign17700_e25298_d_n13, assign17700_e25298_d_n15, assign17700_e25298_d_n16, assign17700_e25298_d_n17, assign17700_e25298_d_n18,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign17700_e25296: f64 = (locals.var_qb * 0.5);
        (assign17700_e25296, (locals.var_qb_dn0 * 0.5), (locals.var_qb_dn2 * 0.5), (locals.var_qb_dn6 * 0.5), (locals.var_qb_dn7 * 0.5), (locals.var_qb_dn10 * 0.5), (locals.var_qb_dn11 * 0.5), (locals.var_qb_dn12 * 0.5), (locals.var_qb_dn13 * 0.5), (locals.var_qb_dn15 * 0.5), (locals.var_qb_dn16 * 0.5), (locals.var_qb_dn17 * 0.5), (locals.var_qb_dn18 * 0.5),)
    } else {
        (locals.var_qd_fb, locals.var_qd_fb_dn0, locals.var_qd_fb_dn2, locals.var_qd_fb_dn6, locals.var_qd_fb_dn7, locals.var_qd_fb_dn10, locals.var_qd_fb_dn11, locals.var_qd_fb_dn12, locals.var_qd_fb_dn13, locals.var_qd_fb_dn15, locals.var_qd_fb_dn16, locals.var_qd_fb_dn17, locals.var_qd_fb_dn18,)
    }
};
        locals.var_qd_fb = assign17700_e25298;
        locals.var_qd_fb_dn0 = assign17700_e25298_d_n0;
        locals.var_qd_fb_dn2 = assign17700_e25298_d_n2;
        locals.var_qd_fb_dn6 = assign17700_e25298_d_n6;
        locals.var_qd_fb_dn7 = assign17700_e25298_d_n7;
        locals.var_qd_fb_dn10 = assign17700_e25298_d_n10;
        locals.var_qd_fb_dn11 = assign17700_e25298_d_n11;
        locals.var_qd_fb_dn12 = assign17700_e25298_d_n12;
        locals.var_qd_fb_dn13 = assign17700_e25298_d_n13;
        locals.var_qd_fb_dn15 = assign17700_e25298_d_n15;
        locals.var_qd_fb_dn16 = assign17700_e25298_d_n16;
        locals.var_qd_fb_dn17 = assign17700_e25298_d_n17;
        locals.var_qd_fb_dn18 = assign17700_e25298_d_n18;
        locals.var_qd_fb_rv = 0.0;

        let (assign17710_e25308, assign17710_e25308_d_n0, assign17710_e25308_d_n2, assign17710_e25308_d_n6, assign17710_e25308_d_n7, assign17710_e25308_d_n10, assign17710_e25308_d_n11, assign17710_e25308_d_n12, assign17710_e25308_d_n13, assign17710_e25308_d_n15, assign17710_e25308_d_n16, assign17710_e25308_d_n17, assign17710_e25308_d_n18,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign17710_e25305: f64 = (1.0 - 0.5);
        let assign17710_e25306: f64 = (locals.var_qb * assign17710_e25305);
        (assign17710_e25306, (locals.var_qb_dn0 * assign17710_e25305), (locals.var_qb_dn2 * assign17710_e25305), (locals.var_qb_dn6 * assign17710_e25305), (locals.var_qb_dn7 * assign17710_e25305), (locals.var_qb_dn10 * assign17710_e25305), (locals.var_qb_dn11 * assign17710_e25305), (locals.var_qb_dn12 * assign17710_e25305), (locals.var_qb_dn13 * assign17710_e25305), (locals.var_qb_dn15 * assign17710_e25305), (locals.var_qb_dn16 * assign17710_e25305), (locals.var_qb_dn17 * assign17710_e25305), (locals.var_qb_dn18 * assign17710_e25305),)
    } else {
        (locals.var_qs_fb, locals.var_qs_fb_dn0, locals.var_qs_fb_dn2, locals.var_qs_fb_dn6, locals.var_qs_fb_dn7, locals.var_qs_fb_dn10, locals.var_qs_fb_dn11, locals.var_qs_fb_dn12, locals.var_qs_fb_dn13, locals.var_qs_fb_dn15, locals.var_qs_fb_dn16, locals.var_qs_fb_dn17, locals.var_qs_fb_dn18,)
    }
};
        locals.var_qs_fb = assign17710_e25308;
        locals.var_qs_fb_dn0 = assign17710_e25308_d_n0;
        locals.var_qs_fb_dn2 = assign17710_e25308_d_n2;
        locals.var_qs_fb_dn6 = assign17710_e25308_d_n6;
        locals.var_qs_fb_dn7 = assign17710_e25308_d_n7;
        locals.var_qs_fb_dn10 = assign17710_e25308_d_n10;
        locals.var_qs_fb_dn11 = assign17710_e25308_d_n11;
        locals.var_qs_fb_dn12 = assign17710_e25308_d_n12;
        locals.var_qs_fb_dn13 = assign17710_e25308_d_n13;
        locals.var_qs_fb_dn15 = assign17710_e25308_d_n15;
        locals.var_qs_fb_dn16 = assign17710_e25308_d_n16;
        locals.var_qs_fb_dn17 = assign17710_e25308_d_n17;
        locals.var_qs_fb_dn18 = assign17710_e25308_d_n18;
        locals.var_qs_fb_rv = 0.0;

        let (assign17720_e25322, assign17720_e25322_d_n0, assign17720_e25322_d_n2, assign17720_e25322_d_n6, assign17720_e25322_d_n7, assign17720_e25322_d_n10, assign17720_e25322_d_n11, assign17720_e25322_d_n12, assign17720_e25322_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard530 != 0.0)) {
        let assign17720_e25315: f64 = (locals.var_q_s0_bulk + locals.var_q_sl_bulk);
        let assign17720_e25316: f64 = (0.5 * assign17720_e25315);
        let assign17720_e25318: f64 = (assign17720_e25316 * locals.var_leff_cv);
        let assign17720_e25320: f64 = (assign17720_e25318 * locals.var_weffcv_nf);
        (assign17720_e25320, (((0.5 * (locals.var_q_s0_bulk_dn0 + locals.var_q_sl_bulk_dn0)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn2 + locals.var_q_sl_bulk_dn2)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn6 + locals.var_q_sl_bulk_dn6)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn7 + locals.var_q_sl_bulk_dn7)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn10 + locals.var_q_sl_bulk_dn10)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn11 + locals.var_q_sl_bulk_dn11)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn12 + locals.var_q_sl_bulk_dn12)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn17 + locals.var_q_sl_bulk_dn17)) * locals.var_leff_cv) * locals.var_weffcv_nf),)
    } else {
        (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12, locals.var_qsub_dn17,)
    }
};
        locals.var_qsub = assign17720_e25322;
        locals.var_qsub_dn0 = assign17720_e25322_d_n0;
        locals.var_qsub_dn2 = assign17720_e25322_d_n2;
        locals.var_qsub_dn6 = assign17720_e25322_d_n6;
        locals.var_qsub_dn7 = assign17720_e25322_d_n7;
        locals.var_qsub_dn10 = assign17720_e25322_d_n10;
        locals.var_qsub_dn11 = assign17720_e25322_d_n11;
        locals.var_qsub_dn12 = assign17720_e25322_d_n12;
        locals.var_qsub_dn17 = assign17720_e25322_d_n17;
        locals.var_qsub_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17730_e25330, assign17730_e25330_d_n0, assign17730_e25330_d_n2, assign17730_e25330_d_n6, assign17730_e25330_d_n7, assign17730_e25330_d_n10, assign17730_e25330_d_n11, assign17730_e25330_d_n12, assign17730_e25330_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17730_e25326: f64 = (locals.var_vds - locals.var_pds);
        let assign17730_e25328: f64 = (assign17730_e25326 / 2.0);
        (assign17730_e25328, ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0), ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0), ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0), ((locals.var_vds_dn7 - locals.var_pds_dn7) / 2.0), ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0), ((locals.var_vds_dn11 - locals.var_pds_dn11) / 2.0), ((locals.var_vds_dn12 - locals.var_pds_dn12) / 2.0), ((locals.var_vds_dn17 - locals.var_pds_dn17) / 2.0),)
    } else {
        (locals.var_t1__blk531, locals.var_t1__blk531_dn0, locals.var_t1__blk531_dn2, locals.var_t1__blk531_dn6, locals.var_t1__blk531_dn7, locals.var_t1__blk531_dn10, locals.var_t1__blk531_dn11, locals.var_t1__blk531_dn12, locals.var_t1__blk531_dn17,)
    }
};
        locals.var_t1__blk531 = assign17730_e25330;
        locals.var_t1__blk531_dn0 = assign17730_e25330_d_n0;
        locals.var_t1__blk531_dn2 = assign17730_e25330_d_n2;
        locals.var_t1__blk531_dn6 = assign17730_e25330_d_n6;
        locals.var_t1__blk531_dn7 = assign17730_e25330_d_n7;
        locals.var_t1__blk531_dn10 = assign17730_e25330_d_n10;
        locals.var_t1__blk531_dn11 = assign17730_e25330_d_n11;
        locals.var_t1__blk531_dn12 = assign17730_e25330_d_n12;
        locals.var_t1__blk531_dn17 = assign17730_e25330_d_n17;
        locals.var_t1__blk531_rv = 0.0;

        let (assign17740_e25338, assign17740_e25338_d_n0, assign17740_e25338_d_n2, assign17740_e25338_d_n6, assign17740_e25338_d_n7, assign17740_e25338_d_n10, assign17740_e25338_d_n11, assign17740_e25338_d_n12, assign17740_e25338_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17740_e25334: f64 = (2.0 * locals.var_t1__blk531);
        let assign17740_e25336: f64 = (assign17740_e25334 / p.p227);
        (assign17740_e25336, ((2.0 * locals.var_t1__blk531_dn0) / p.p227), ((2.0 * locals.var_t1__blk531_dn2) / p.p227), ((2.0 * locals.var_t1__blk531_dn6) / p.p227), ((2.0 * locals.var_t1__blk531_dn7) / p.p227), ((2.0 * locals.var_t1__blk531_dn10) / p.p227), ((2.0 * locals.var_t1__blk531_dn11) / p.p227), ((2.0 * locals.var_t1__blk531_dn12) / p.p227), ((2.0 * locals.var_t1__blk531_dn17) / p.p227),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17740_e25338;
        locals.var_tmf1_dn0 = assign17740_e25338_d_n0;
        locals.var_tmf1_dn2 = assign17740_e25338_d_n2;
        locals.var_tmf1_dn6 = assign17740_e25338_d_n6;
        locals.var_tmf1_dn7 = assign17740_e25338_d_n7;
        locals.var_tmf1_dn10 = assign17740_e25338_d_n10;
        locals.var_tmf1_dn11 = assign17740_e25338_d_n11;
        locals.var_tmf1_dn12 = assign17740_e25338_d_n12;
        locals.var_tmf1_dn17 = assign17740_e25338_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign17750_e25378, assign17750_e25378_d_n0, assign17750_e25378_d_n2, assign17750_e25378_d_n6, assign17750_e25378_d_n7, assign17750_e25378_d_n10, assign17750_e25378_d_n11, assign17750_e25378_d_n12, assign17750_e25378_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17750_e25344: f64 = (1.0 / 2.0);
        let assign17750_e25348: f64 = (1.0 / 6.0);
        let assign17750_e25352: f64 = (1.0 / 24.0);
        let assign17750_e25356: f64 = (1.0 / 120.0);
        let assign17750_e25360: f64 = (1.0 / 720.0);
        let assign17750_e25364: f64 = (1.0 / 5040.0);
        let assign17750_e25365: f64 = (locals.var_tmf1 * assign17750_e25364);
        let assign17750_e25366: f64 = (assign17750_e25360 + assign17750_e25365);
        let assign17750_e25367: f64 = (locals.var_tmf1 * assign17750_e25366);
        let assign17750_e25368: f64 = (assign17750_e25356 + assign17750_e25367);
        let assign17750_e25369: f64 = (locals.var_tmf1 * assign17750_e25368);
        let assign17750_e25370: f64 = (assign17750_e25352 + assign17750_e25369);
        let assign17750_e25371: f64 = (locals.var_tmf1 * assign17750_e25370);
        let assign17750_e25372: f64 = (assign17750_e25348 + assign17750_e25371);
        let assign17750_e25373: f64 = (locals.var_tmf1 * assign17750_e25372);
        let assign17750_e25374: f64 = (assign17750_e25344 + assign17750_e25373);
        let assign17750_e25375: f64 = (locals.var_tmf1 * assign17750_e25374);
        let assign17750_e25376: f64 = (1.0 + assign17750_e25375);
        (assign17750_e25376, ((locals.var_tmf1_dn0 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn2 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn6 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn7 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn10 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn11 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn12 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign17750_e25364))))))))))), ((locals.var_tmf1_dn17 * assign17750_e25374) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17750_e25372) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17750_e25370) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17750_e25368) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17750_e25366) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign17750_e25364))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign17750_e25378;
        locals.var_tmf2_dn0 = assign17750_e25378_d_n0;
        locals.var_tmf2_dn2 = assign17750_e25378_d_n2;
        locals.var_tmf2_dn6 = assign17750_e25378_d_n6;
        locals.var_tmf2_dn7 = assign17750_e25378_d_n7;
        locals.var_tmf2_dn10 = assign17750_e25378_d_n10;
        locals.var_tmf2_dn11 = assign17750_e25378_d_n11;
        locals.var_tmf2_dn12 = assign17750_e25378_d_n12;
        locals.var_tmf2_dn17 = assign17750_e25378_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign17760_e25384, assign17760_e25384_d_n0, assign17760_e25384_d_n2, assign17760_e25384_d_n6, assign17760_e25384_d_n7, assign17760_e25384_d_n10, assign17760_e25384_d_n11, assign17760_e25384_d_n12, assign17760_e25384_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17760_e25382: f64 = (p.p227 / locals.var_tmf2);
        (assign17760_e25382, (-((p.p227 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn12, locals.var_pzadd_dn17,)
    }
};
        locals.var_pzadd = assign17760_e25384;
        locals.var_pzadd_dn0 = assign17760_e25384_d_n0;
        locals.var_pzadd_dn2 = assign17760_e25384_d_n2;
        locals.var_pzadd_dn6 = assign17760_e25384_d_n6;
        locals.var_pzadd_dn7 = assign17760_e25384_d_n7;
        locals.var_pzadd_dn10 = assign17760_e25384_d_n10;
        locals.var_pzadd_dn11 = assign17760_e25384_d_n11;
        locals.var_pzadd_dn12 = assign17760_e25384_d_n12;
        locals.var_pzadd_dn17 = assign17760_e25384_d_n17;
        locals.var_pzadd_rv = 0.0;

        let assign17770_e25388: f64 = (10.0 * 2.220446049250313e-16);
        let assign17770_e25389: f64 = if locals.var_pzadd < assign17770_e25388 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign17770_e25389;
        locals.var_guard532_rv = 0.0;

        let (assign17780_e25397, assign17780_e25397_d_n0, assign17780_e25397_d_n2, assign17780_e25397_d_n6, assign17780_e25397_d_n7, assign17780_e25397_d_n10, assign17780_e25397_d_n11, assign17780_e25397_d_n12, assign17780_e25397_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard532 != 0.0)) {
        let assign17780_e25395: f64 = (10.0 * 2.220446049250313e-16);
        (assign17780_e25395, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn12, locals.var_pzadd_dn17,)
    }
};
        locals.var_pzadd = assign17780_e25397;
        locals.var_pzadd_dn0 = assign17780_e25397_d_n0;
        locals.var_pzadd_dn2 = assign17780_e25397_d_n2;
        locals.var_pzadd_dn6 = assign17780_e25397_d_n6;
        locals.var_pzadd_dn7 = assign17780_e25397_d_n7;
        locals.var_pzadd_dn10 = assign17780_e25397_d_n10;
        locals.var_pzadd_dn11 = assign17780_e25397_d_n11;
        locals.var_pzadd_dn12 = assign17780_e25397_d_n12;
        locals.var_pzadd_dn17 = assign17780_e25397_d_n17;
        locals.var_pzadd_rv = 0.0;

        let (assign17790_e25403, assign17790_e25403_d_n0, assign17790_e25403_d_n2, assign17790_e25403_d_n6, assign17790_e25403_d_n7, assign17790_e25403_d_n10, assign17790_e25403_d_n11, assign17790_e25403_d_n12, assign17790_e25403_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17790_e25401: f64 = (locals.var_ps0 + locals.var_pzadd);
        (assign17790_e25401, (locals.var_ps0_dn0 + locals.var_pzadd_dn0), (locals.var_ps0_dn2 + locals.var_pzadd_dn2), (locals.var_ps0_dn6 + locals.var_pzadd_dn6), (locals.var_ps0_dn7 + locals.var_pzadd_dn7), (locals.var_ps0_dn10 + locals.var_pzadd_dn10), (locals.var_ps0_dn11 + locals.var_pzadd_dn11), (locals.var_ps0_dn12 + locals.var_pzadd_dn12), (locals.var_ps0_dn17 + locals.var_pzadd_dn17),)
    } else {
        (locals.var_ps0z, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn10, locals.var_ps0z_dn11, locals.var_ps0z_dn12, locals.var_ps0z_dn17,)
    }
};
        locals.var_ps0z = assign17790_e25403;
        locals.var_ps0z_dn0 = assign17790_e25403_d_n0;
        locals.var_ps0z_dn2 = assign17790_e25403_d_n2;
        locals.var_ps0z_dn6 = assign17790_e25403_d_n6;
        locals.var_ps0z_dn7 = assign17790_e25403_d_n7;
        locals.var_ps0z_dn10 = assign17790_e25403_d_n10;
        locals.var_ps0z_dn11 = assign17790_e25403_d_n11;
        locals.var_ps0z_dn12 = assign17790_e25403_d_n12;
        locals.var_ps0z_dn17 = assign17790_e25403_d_n17;
        locals.var_ps0z_rv = 0.0;

        let (assign17800_e25409,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17800_e25407: f64 = (1.034943e-10 / 100.0);
        (assign17800_e25407,)
    } else {
        (locals.var_cgs_esi,)
    }
};
        locals.var_cgs_esi = assign17800_e25409;
        locals.var_cgs_esi_rv = 0.0;

        let (assign17810_e25415, assign17810_e25415_d_n0, assign17810_e25415_d_n2, assign17810_e25415_d_n6, assign17810_e25415_d_n7, assign17810_e25415_d_n10, assign17810_e25415_d_n11, assign17810_e25415_d_n12, assign17810_e25415_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17810_e25413: f64 = (locals.var_qbu / 10000.0);
        (assign17810_e25413, (locals.var_qbu_dn0 / 10000.0), (locals.var_qbu_dn2 / 10000.0), (locals.var_qbu_dn6 / 10000.0), (locals.var_qbu_dn7 / 10000.0), (locals.var_qbu_dn10 / 10000.0), (locals.var_qbu_dn11 / 10000.0), (locals.var_qbu_dn12 / 10000.0), (locals.var_qbu_dn17 / 10000.0),)
    } else {
        (locals.var_cgs_qbu, locals.var_cgs_qbu_dn0, locals.var_cgs_qbu_dn2, locals.var_cgs_qbu_dn6, locals.var_cgs_qbu_dn7, locals.var_cgs_qbu_dn10, locals.var_cgs_qbu_dn11, locals.var_cgs_qbu_dn12, locals.var_cgs_qbu_dn17,)
    }
};
        locals.var_cgs_qbu = assign17810_e25415;
        locals.var_cgs_qbu_dn0 = assign17810_e25415_d_n0;
        locals.var_cgs_qbu_dn2 = assign17810_e25415_d_n2;
        locals.var_cgs_qbu_dn6 = assign17810_e25415_d_n6;
        locals.var_cgs_qbu_dn7 = assign17810_e25415_d_n7;
        locals.var_cgs_qbu_dn10 = assign17810_e25415_d_n10;
        locals.var_cgs_qbu_dn11 = assign17810_e25415_d_n11;
        locals.var_cgs_qbu_dn12 = assign17810_e25415_d_n12;
        locals.var_cgs_qbu_dn17 = assign17810_e25415_d_n17;
        locals.var_cgs_qbu_rv = 0.0;

        let (assign17820_e25421, assign17820_e25421_d_n0, assign17820_e25421_d_n2, assign17820_e25421_d_n6, assign17820_e25421_d_n7, assign17820_e25421_d_n10, assign17820_e25421_d_n11, assign17820_e25421_d_n12, assign17820_e25421_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17820_e25419: f64 = (locals.var_qiu / 10000.0);
        (assign17820_e25419, (locals.var_qiu_dn0 / 10000.0), (locals.var_qiu_dn2 / 10000.0), (locals.var_qiu_dn6 / 10000.0), (locals.var_qiu_dn7 / 10000.0), (locals.var_qiu_dn10 / 10000.0), (locals.var_qiu_dn11 / 10000.0), (locals.var_qiu_dn12 / 10000.0), (locals.var_qiu_dn17 / 10000.0),)
    } else {
        (locals.var_cgs_qiu, locals.var_cgs_qiu_dn0, locals.var_cgs_qiu_dn2, locals.var_cgs_qiu_dn6, locals.var_cgs_qiu_dn7, locals.var_cgs_qiu_dn10, locals.var_cgs_qiu_dn11, locals.var_cgs_qiu_dn12, locals.var_cgs_qiu_dn17,)
    }
};
        locals.var_cgs_qiu = assign17820_e25421;
        locals.var_cgs_qiu_dn0 = assign17820_e25421_d_n0;
        locals.var_cgs_qiu_dn2 = assign17820_e25421_d_n2;
        locals.var_cgs_qiu_dn6 = assign17820_e25421_d_n6;
        locals.var_cgs_qiu_dn7 = assign17820_e25421_d_n7;
        locals.var_cgs_qiu_dn10 = assign17820_e25421_d_n10;
        locals.var_cgs_qiu_dn11 = assign17820_e25421_d_n11;
        locals.var_cgs_qiu_dn12 = assign17820_e25421_d_n12;
        locals.var_cgs_qiu_dn17 = assign17820_e25421_d_n17;
        locals.var_cgs_qiu_rv = 0.0;

        let (assign17830_e25427, assign17830_e25427_d_n0, assign17830_e25427_d_n2, assign17830_e25427_d_n6, assign17830_e25427_d_n7, assign17830_e25427_d_n10, assign17830_e25427_d_n11, assign17830_e25427_d_n12, assign17830_e25427_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17830_e25425: f64 = (p.p92 / locals.var_cgs_esi);
        (assign17830_e25425, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk533, locals.var_t1__blk533_dn0, locals.var_t1__blk533_dn2, locals.var_t1__blk533_dn6, locals.var_t1__blk533_dn7, locals.var_t1__blk533_dn10, locals.var_t1__blk533_dn11, locals.var_t1__blk533_dn12, locals.var_t1__blk533_dn17,)
    }
};
        locals.var_t1__blk533 = assign17830_e25427;
        locals.var_t1__blk533_dn0 = assign17830_e25427_d_n0;
        locals.var_t1__blk533_dn2 = assign17830_e25427_d_n2;
        locals.var_t1__blk533_dn6 = assign17830_e25427_d_n6;
        locals.var_t1__blk533_dn7 = assign17830_e25427_d_n7;
        locals.var_t1__blk533_dn10 = assign17830_e25427_d_n10;
        locals.var_t1__blk533_dn11 = assign17830_e25427_d_n11;
        locals.var_t1__blk533_dn12 = assign17830_e25427_d_n12;
        locals.var_t1__blk533_dn17 = assign17830_e25427_d_n17;
        locals.var_t1__blk533_rv = 0.0;

        let (assign17840_e25433,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17840_e25431: f64 = (p.p93 / locals.var_cgs_esi);
        (assign17840_e25431,)
    } else {
        (locals.var_t2__blk534,)
    }
};
        locals.var_t2__blk534 = assign17840_e25433;
        locals.var_t2__blk534_rv = 0.0;

        let (assign17850_e25437, assign17850_e25437_d_n0, assign17850_e25437_d_n2, assign17850_e25437_d_n6, assign17850_e25437_d_n7, assign17850_e25437_d_n10, assign17850_e25437_d_n11, assign17850_e25437_d_n12, assign17850_e25437_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (p.p94, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk535, locals.var_t0__blk535_dn0, locals.var_t0__blk535_dn2, locals.var_t0__blk535_dn6, locals.var_t0__blk535_dn7, locals.var_t0__blk535_dn10, locals.var_t0__blk535_dn11, locals.var_t0__blk535_dn12, locals.var_t0__blk535_dn17,)
    }
};
        locals.var_t0__blk535 = assign17850_e25437;
        locals.var_t0__blk535_dn0 = assign17850_e25437_d_n0;
        locals.var_t0__blk535_dn2 = assign17850_e25437_d_n2;
        locals.var_t0__blk535_dn6 = assign17850_e25437_d_n6;
        locals.var_t0__blk535_dn7 = assign17850_e25437_d_n7;
        locals.var_t0__blk535_dn10 = assign17850_e25437_d_n10;
        locals.var_t0__blk535_dn11 = assign17850_e25437_d_n11;
        locals.var_t0__blk535_dn12 = assign17850_e25437_d_n12;
        locals.var_t0__blk535_dn17 = assign17850_e25437_d_n17;
        locals.var_t0__blk535_rv = 0.0;

        let (assign17860_e25447, assign17860_e25447_d_n0, assign17860_e25447_d_n2, assign17860_e25447_d_n6, assign17860_e25447_d_n7, assign17860_e25447_d_n10, assign17860_e25447_d_n11, assign17860_e25447_d_n12, assign17860_e25447_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17860_e25442: f64 = (locals.var_psl - locals.var_ps0);
        let assign17860_e25444: f64 = (assign17860_e25442 * locals.var_t0__blk535);
        let assign17860_e25445: f64 = (1.0 + assign17860_e25444);
        (assign17860_e25445, (((locals.var_psl_dn0 - locals.var_ps0_dn0) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn0)), (((locals.var_psl_dn2 - locals.var_ps0_dn2) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn2)), (((locals.var_psl_dn6 - locals.var_ps0_dn6) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn6)), (((locals.var_psl_dn7 - locals.var_ps0_dn7) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn7)), (((locals.var_psl_dn10 - locals.var_ps0_dn10) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn10)), (((locals.var_psl_dn11 - locals.var_ps0_dn11) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn11)), (((locals.var_psl_dn12 - locals.var_ps0_dn12) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn12)), (((locals.var_psl_dn17 - locals.var_ps0_dn17) * locals.var_t0__blk535) + (assign17860_e25442 * locals.var_t0__blk535_dn17)),)
    } else {
        (locals.var_t4__blk536, locals.var_t4__blk536_dn0, locals.var_t4__blk536_dn2, locals.var_t4__blk536_dn6, locals.var_t4__blk536_dn7, locals.var_t4__blk536_dn10, locals.var_t4__blk536_dn11, locals.var_t4__blk536_dn12, locals.var_t4__blk536_dn17,)
    }
};
        locals.var_t4__blk536 = assign17860_e25447;
        locals.var_t4__blk536_dn0 = assign17860_e25447_d_n0;
        locals.var_t4__blk536_dn2 = assign17860_e25447_d_n2;
        locals.var_t4__blk536_dn6 = assign17860_e25447_d_n6;
        locals.var_t4__blk536_dn7 = assign17860_e25447_d_n7;
        locals.var_t4__blk536_dn10 = assign17860_e25447_d_n10;
        locals.var_t4__blk536_dn11 = assign17860_e25447_d_n11;
        locals.var_t4__blk536_dn12 = assign17860_e25447_d_n12;
        locals.var_t4__blk536_dn17 = assign17860_e25447_d_n17;
        locals.var_t4__blk536_rv = 0.0;

        let (assign17870_e25457, assign17870_e25457_d_n0, assign17870_e25457_d_n2, assign17870_e25457_d_n6, assign17870_e25457_d_n7, assign17870_e25457_d_n10, assign17870_e25457_d_n11, assign17870_e25457_d_n12, assign17870_e25457_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17870_e25451: f64 = (locals.var_t1__blk533 * locals.var_cgs_qbu);
        let assign17870_e25454: f64 = (locals.var_t2__blk534 * locals.var_cgs_qiu);
        let assign17870_e25455: f64 = (assign17870_e25451 + assign17870_e25454);
        (assign17870_e25455, (((locals.var_t1__blk533_dn0 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn0)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn0)), (((locals.var_t1__blk533_dn2 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn2)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn2)), (((locals.var_t1__blk533_dn6 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn6)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn6)), (((locals.var_t1__blk533_dn7 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn7)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn7)), (((locals.var_t1__blk533_dn10 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn10)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn10)), (((locals.var_t1__blk533_dn11 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn11)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn11)), (((locals.var_t1__blk533_dn12 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn12)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn12)), (((locals.var_t1__blk533_dn17 * locals.var_cgs_qbu) + (locals.var_t1__blk533 * locals.var_cgs_qbu_dn17)) + (locals.var_t2__blk534 * locals.var_cgs_qiu_dn17)),)
    } else {
        (locals.var_t5__blk537, locals.var_t5__blk537_dn0, locals.var_t5__blk537_dn2, locals.var_t5__blk537_dn6, locals.var_t5__blk537_dn7, locals.var_t5__blk537_dn10, locals.var_t5__blk537_dn11, locals.var_t5__blk537_dn12, locals.var_t5__blk537_dn17,)
    }
};
        locals.var_t5__blk537 = assign17870_e25457;
        locals.var_t5__blk537_dn0 = assign17870_e25457_d_n0;
        locals.var_t5__blk537_dn2 = assign17870_e25457_d_n2;
        locals.var_t5__blk537_dn6 = assign17870_e25457_d_n6;
        locals.var_t5__blk537_dn7 = assign17870_e25457_d_n7;
        locals.var_t5__blk537_dn10 = assign17870_e25457_d_n10;
        locals.var_t5__blk537_dn11 = assign17870_e25457_d_n11;
        locals.var_t5__blk537_dn12 = assign17870_e25457_d_n12;
        locals.var_t5__blk537_dn17 = assign17870_e25457_d_n17;
        locals.var_t5__blk537_rv = 0.0;

        let (assign17880_e25463, assign17880_e25463_d_n0, assign17880_e25463_d_n2, assign17880_e25463_d_n6, assign17880_e25463_d_n7, assign17880_e25463_d_n10, assign17880_e25463_d_n11, assign17880_e25463_d_n12, assign17880_e25463_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17880_e25461: f64 = (locals.var_t5__blk537 / locals.var_t4__blk536);
        (assign17880_e25461, (((locals.var_t5__blk537_dn0 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn0)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn2 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn2)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn6 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn6)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn7 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn7)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn10 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn10)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn11 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn11)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn12 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn12)) / (locals.var_t4__blk536 * locals.var_t4__blk536)), (((locals.var_t5__blk537_dn17 * locals.var_t4__blk536) - (locals.var_t5__blk537 * locals.var_t4__blk536_dn17)) / (locals.var_t4__blk536 * locals.var_t4__blk536)),)
    } else {
        (locals.var_t3__blk538, locals.var_t3__blk538_dn0, locals.var_t3__blk538_dn2, locals.var_t3__blk538_dn6, locals.var_t3__blk538_dn7, locals.var_t3__blk538_dn10, locals.var_t3__blk538_dn11, locals.var_t3__blk538_dn12, locals.var_t3__blk538_dn17,)
    }
};
        locals.var_t3__blk538 = assign17880_e25463;
        locals.var_t3__blk538_dn0 = assign17880_e25463_d_n0;
        locals.var_t3__blk538_dn2 = assign17880_e25463_d_n2;
        locals.var_t3__blk538_dn6 = assign17880_e25463_d_n6;
        locals.var_t3__blk538_dn7 = assign17880_e25463_d_n7;
        locals.var_t3__blk538_dn10 = assign17880_e25463_d_n10;
        locals.var_t3__blk538_dn11 = assign17880_e25463_d_n11;
        locals.var_t3__blk538_dn12 = assign17880_e25463_d_n12;
        locals.var_t3__blk538_dn17 = assign17880_e25463_d_n17;
        locals.var_t3__blk538_rv = 0.0;

        let (assign17890_e25467, assign17890_e25467_d_n0, assign17890_e25467_d_n2, assign17890_e25467_d_n6, assign17890_e25467_d_n7, assign17890_e25467_d_n10, assign17890_e25467_d_n11, assign17890_e25467_d_n12, assign17890_e25467_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        (locals.var_t3__blk538, locals.var_t3__blk538_dn0, locals.var_t3__blk538_dn2, locals.var_t3__blk538_dn6, locals.var_t3__blk538_dn7, locals.var_t3__blk538_dn10, locals.var_t3__blk538_dn11, locals.var_t3__blk538_dn12, locals.var_t3__blk538_dn17,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn12, locals.var_eeff_dn17,)
    }
};
        locals.var_eeff = assign17890_e25467;
        locals.var_eeff_dn0 = assign17890_e25467_d_n0;
        locals.var_eeff_dn2 = assign17890_e25467_d_n2;
        locals.var_eeff_dn6 = assign17890_e25467_d_n6;
        locals.var_eeff_dn7 = assign17890_e25467_d_n7;
        locals.var_eeff_dn10 = assign17890_e25467_d_n10;
        locals.var_eeff_dn11 = assign17890_e25467_d_n11;
        locals.var_eeff_dn12 = assign17890_e25467_d_n12;
        locals.var_eeff_dn17 = assign17890_e25467_d_n17;
        locals.var_eeff_rv = 0.0;

        let (assign17900_e25480, assign17900_e25480_d_n0, assign17900_e25480_d_n2, assign17900_e25480_d_n6, assign17900_e25480_d_n7, assign17900_e25480_d_n10, assign17900_e25480_d_n11, assign17900_e25480_d_n12, assign17900_e25480_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17900_e25471: f64 = (locals.var_eeff * locals.var_eeff);
        let assign17900_e25474: f64 = (4.0 * 3000.0);
        let assign17900_e25476: f64 = (assign17900_e25474 * 3000.0);
        let assign17900_e25477: f64 = (assign17900_e25471 + assign17900_e25476);
        let assign17900_e25478: f64 = (assign17900_e25477).sqrt();
        (assign17900_e25478, (((locals.var_eeff_dn0 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn0)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn2 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn2)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn6 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn6)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn7 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn7)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn10 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn10)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn11 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn11)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn12 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn12)) / (2.0 * assign17900_e25478)), (((locals.var_eeff_dn17 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn17)) / (2.0 * assign17900_e25478)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17900_e25480;
        locals.var_tmf1_dn0 = assign17900_e25480_d_n0;
        locals.var_tmf1_dn2 = assign17900_e25480_d_n2;
        locals.var_tmf1_dn6 = assign17900_e25480_d_n6;
        locals.var_tmf1_dn7 = assign17900_e25480_d_n7;
        locals.var_tmf1_dn10 = assign17900_e25480_d_n10;
        locals.var_tmf1_dn11 = assign17900_e25480_d_n11;
        locals.var_tmf1_dn12 = assign17900_e25480_d_n12;
        locals.var_tmf1_dn17 = assign17900_e25480_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign17910_e25492, assign17910_e25492_d_n0, assign17910_e25492_d_n2, assign17910_e25492_d_n6, assign17910_e25492_d_n7, assign17910_e25492_d_n10, assign17910_e25492_d_n11, assign17910_e25492_d_n12, assign17910_e25492_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17910_e25485: f64 = (locals.var_eeff + locals.var_tmf1);
        let assign17910_e25486: f64 = (0.5 * assign17910_e25485);
        let assign17910_e25489: f64 = (1e-10 * 3000.0);
        let assign17910_e25490: f64 = (assign17910_e25486 + assign17910_e25489);
        (assign17910_e25490, (0.5 * (locals.var_eeff_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_eeff_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_eeff_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_eeff_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_eeff_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_eeff_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_eeff_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_eeff_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t0__blk535, locals.var_t0__blk535_dn0, locals.var_t0__blk535_dn2, locals.var_t0__blk535_dn6, locals.var_t0__blk535_dn7, locals.var_t0__blk535_dn10, locals.var_t0__blk535_dn11, locals.var_t0__blk535_dn12, locals.var_t0__blk535_dn17,)
    }
};
        locals.var_t0__blk535 = assign17910_e25492;
        locals.var_t0__blk535_dn0 = assign17910_e25492_d_n0;
        locals.var_t0__blk535_dn2 = assign17910_e25492_d_n2;
        locals.var_t0__blk535_dn6 = assign17910_e25492_d_n6;
        locals.var_t0__blk535_dn7 = assign17910_e25492_d_n7;
        locals.var_t0__blk535_dn10 = assign17910_e25492_d_n10;
        locals.var_t0__blk535_dn11 = assign17910_e25492_d_n11;
        locals.var_t0__blk535_dn12 = assign17910_e25492_d_n12;
        locals.var_t0__blk535_dn17 = assign17910_e25492_d_n17;
        locals.var_t0__blk535_rv = 0.0;

        let assign17920_e25495: f64 = if locals.var_t0__blk535 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign17920_e25495;
        locals.var_guard545_rv = 0.0;

        let (assign17930_e25501, assign17930_e25501_d_n0, assign17930_e25501_d_n2, assign17930_e25501_d_n6, assign17930_e25501_d_n7, assign17930_e25501_d_n10, assign17930_e25501_d_n11, assign17930_e25501_d_n12, assign17930_e25501_d_n17,) = {
    if ((locals.var_guard505 != 0.0) && (locals.var_guard545 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk535, locals.var_t0__blk535_dn0, locals.var_t0__blk535_dn2, locals.var_t0__blk535_dn6, locals.var_t0__blk535_dn7, locals.var_t0__blk535_dn10, locals.var_t0__blk535_dn11, locals.var_t0__blk535_dn12, locals.var_t0__blk535_dn17,)
    }
};
        locals.var_t0__blk535 = assign17930_e25501;
        locals.var_t0__blk535_dn0 = assign17930_e25501_d_n0;
        locals.var_t0__blk535_dn2 = assign17930_e25501_d_n2;
        locals.var_t0__blk535_dn6 = assign17930_e25501_d_n6;
        locals.var_t0__blk535_dn7 = assign17930_e25501_d_n7;
        locals.var_t0__blk535_dn10 = assign17930_e25501_d_n10;
        locals.var_t0__blk535_dn11 = assign17930_e25501_d_n11;
        locals.var_t0__blk535_dn12 = assign17930_e25501_d_n12;
        locals.var_t0__blk535_dn17 = assign17930_e25501_d_n17;
        locals.var_t0__blk535_rv = 0.0;

        let (assign17940_e25509, assign17940_e25509_d_n0, assign17940_e25509_d_n2, assign17940_e25509_d_n6, assign17940_e25509_d_n7, assign17940_e25509_d_n10, assign17940_e25509_d_n11, assign17940_e25509_d_n12, assign17940_e25509_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17940_e25506: f64 = (p.p97 - 1.0);
        let assign17940_e25507: f64 = (locals.var_t0__blk535).powf(assign17940_e25506);
        (assign17940_e25507, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn0)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn0 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn2)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn2 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn6)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn6 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn7)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn7 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn10)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn10 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn11)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn11 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn12)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn12 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17940_e25506) as f64).is_finite() && ((assign17940_e25506) as f64).fract() == 0.0 { if assign17940_e25506 == 0.0 { 0.0 } else { (assign17940_e25506 * ((locals.var_t0__blk535).powf(assign17940_e25506 - 1.0) * locals.var_t0__blk535_dn17)) } } else { (assign17940_e25507 * (assign17940_e25506 * (locals.var_t0__blk535_dn17 / locals.var_t0__blk535))) },)
    } else {
        (locals.var_t5__blk537, locals.var_t5__blk537_dn0, locals.var_t5__blk537_dn2, locals.var_t5__blk537_dn6, locals.var_t5__blk537_dn7, locals.var_t5__blk537_dn10, locals.var_t5__blk537_dn11, locals.var_t5__blk537_dn12, locals.var_t5__blk537_dn17,)
    }
};
        locals.var_t5__blk537 = assign17940_e25509;
        locals.var_t5__blk537_dn0 = assign17940_e25509_d_n0;
        locals.var_t5__blk537_dn2 = assign17940_e25509_d_n2;
        locals.var_t5__blk537_dn6 = assign17940_e25509_d_n6;
        locals.var_t5__blk537_dn7 = assign17940_e25509_d_n7;
        locals.var_t5__blk537_dn10 = assign17940_e25509_d_n10;
        locals.var_t5__blk537_dn11 = assign17940_e25509_d_n11;
        locals.var_t5__blk537_dn12 = assign17940_e25509_d_n12;
        locals.var_t5__blk537_dn17 = assign17940_e25509_d_n17;
        locals.var_t5__blk537_rv = 0.0;

        let (assign17950_e25515, assign17950_e25515_d_n0, assign17950_e25515_d_n2, assign17950_e25515_d_n6, assign17950_e25515_d_n7, assign17950_e25515_d_n10, assign17950_e25515_d_n11, assign17950_e25515_d_n12, assign17950_e25515_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17950_e25513: f64 = (locals.var_t5__blk537 * locals.var_t0__blk535);
        (assign17950_e25513, ((locals.var_t5__blk537_dn0 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn0)), ((locals.var_t5__blk537_dn2 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn2)), ((locals.var_t5__blk537_dn6 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn6)), ((locals.var_t5__blk537_dn7 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn7)), ((locals.var_t5__blk537_dn10 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn10)), ((locals.var_t5__blk537_dn11 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn11)), ((locals.var_t5__blk537_dn12 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn12)), ((locals.var_t5__blk537_dn17 * locals.var_t0__blk535) + (locals.var_t5__blk537 * locals.var_t0__blk535_dn17)),)
    } else {
        (locals.var_t8__blk539, locals.var_t8__blk539_dn0, locals.var_t8__blk539_dn2, locals.var_t8__blk539_dn6, locals.var_t8__blk539_dn7, locals.var_t8__blk539_dn10, locals.var_t8__blk539_dn11, locals.var_t8__blk539_dn12, locals.var_t8__blk539_dn17,)
    }
};
        locals.var_t8__blk539 = assign17950_e25515;
        locals.var_t8__blk539_dn0 = assign17950_e25515_d_n0;
        locals.var_t8__blk539_dn2 = assign17950_e25515_d_n2;
        locals.var_t8__blk539_dn6 = assign17950_e25515_d_n6;
        locals.var_t8__blk539_dn7 = assign17950_e25515_d_n7;
        locals.var_t8__blk539_dn10 = assign17950_e25515_d_n10;
        locals.var_t8__blk539_dn11 = assign17950_e25515_d_n11;
        locals.var_t8__blk539_dn12 = assign17950_e25515_d_n12;
        locals.var_t8__blk539_dn17 = assign17950_e25515_d_n17;
        locals.var_t8__blk539_rv = 0.0;

        let (assign17960_e25523, assign17960_e25523_d_n0, assign17960_e25523_d_n2, assign17960_e25523_d_n6, assign17960_e25523_d_n7, assign17960_e25523_d_n10, assign17960_e25523_d_n11, assign17960_e25523_d_n12, assign17960_e25523_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17960_e25520: f64 = (locals.var_muesr - 1.0);
        let assign17960_e25521: f64 = (locals.var_t0__blk535).powf(assign17960_e25520);
        (assign17960_e25521, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn0)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn0 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn2)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn2 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn6)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn6 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn7)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn7 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn10)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn10 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn11)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn11 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn12)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn12 / locals.var_t0__blk535))) }, if 0.0 == 0.0 && ((assign17960_e25520) as f64).is_finite() && ((assign17960_e25520) as f64).fract() == 0.0 { if assign17960_e25520 == 0.0 { 0.0 } else { (assign17960_e25520 * ((locals.var_t0__blk535).powf(assign17960_e25520 - 1.0) * locals.var_t0__blk535_dn17)) } } else { (assign17960_e25521 * (assign17960_e25520 * (locals.var_t0__blk535_dn17 / locals.var_t0__blk535))) },)
    } else {
        (locals.var_t7__blk540, locals.var_t7__blk540_dn0, locals.var_t7__blk540_dn2, locals.var_t7__blk540_dn6, locals.var_t7__blk540_dn7, locals.var_t7__blk540_dn10, locals.var_t7__blk540_dn11, locals.var_t7__blk540_dn12, locals.var_t7__blk540_dn17,)
    }
};
        locals.var_t7__blk540 = assign17960_e25523;
        locals.var_t7__blk540_dn0 = assign17960_e25523_d_n0;
        locals.var_t7__blk540_dn2 = assign17960_e25523_d_n2;
        locals.var_t7__blk540_dn6 = assign17960_e25523_d_n6;
        locals.var_t7__blk540_dn7 = assign17960_e25523_d_n7;
        locals.var_t7__blk540_dn10 = assign17960_e25523_d_n10;
        locals.var_t7__blk540_dn11 = assign17960_e25523_d_n11;
        locals.var_t7__blk540_dn12 = assign17960_e25523_d_n12;
        locals.var_t7__blk540_dn17 = assign17960_e25523_d_n17;
        locals.var_t7__blk540_rv = 0.0;

        let (assign17970_e25529, assign17970_e25529_d_n0, assign17970_e25529_d_n2, assign17970_e25529_d_n6, assign17970_e25529_d_n7, assign17970_e25529_d_n10, assign17970_e25529_d_n11, assign17970_e25529_d_n12, assign17970_e25529_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17970_e25527: f64 = (locals.var_t7__blk540 * locals.var_t0__blk535);
        (assign17970_e25527, ((locals.var_t7__blk540_dn0 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn0)), ((locals.var_t7__blk540_dn2 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn2)), ((locals.var_t7__blk540_dn6 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn6)), ((locals.var_t7__blk540_dn7 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn7)), ((locals.var_t7__blk540_dn10 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn10)), ((locals.var_t7__blk540_dn11 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn11)), ((locals.var_t7__blk540_dn12 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn12)), ((locals.var_t7__blk540_dn17 * locals.var_t0__blk535) + (locals.var_t7__blk540 * locals.var_t0__blk535_dn17)),)
    } else {
        (locals.var_t6__blk541, locals.var_t6__blk541_dn0, locals.var_t6__blk541_dn2, locals.var_t6__blk541_dn6, locals.var_t6__blk541_dn7, locals.var_t6__blk541_dn10, locals.var_t6__blk541_dn11, locals.var_t6__blk541_dn12, locals.var_t6__blk541_dn17,)
    }
};
        locals.var_t6__blk541 = assign17970_e25529;
        locals.var_t6__blk541_dn0 = assign17970_e25529_d_n0;
        locals.var_t6__blk541_dn2 = assign17970_e25529_d_n2;
        locals.var_t6__blk541_dn6 = assign17970_e25529_d_n6;
        locals.var_t6__blk541_dn7 = assign17970_e25529_d_n7;
        locals.var_t6__blk541_dn10 = assign17970_e25529_d_n10;
        locals.var_t6__blk541_dn11 = assign17970_e25529_d_n11;
        locals.var_t6__blk541_dn12 = assign17970_e25529_d_n12;
        locals.var_t6__blk541_dn17 = assign17970_e25529_d_n17;
        locals.var_t6__blk541_rv = 0.0;

        let (assign17980_e25535, assign17980_e25535_d_n0, assign17980_e25535_d_n2, assign17980_e25535_d_n6, assign17980_e25535_d_n7, assign17980_e25535_d_n10, assign17980_e25535_d_n11, assign17980_e25535_d_n12, assign17980_e25535_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17980_e25533: f64 = (locals.var_cgs_qiu / 1.6021918e-19);
        (assign17980_e25533, (locals.var_cgs_qiu_dn0 / 1.6021918e-19), (locals.var_cgs_qiu_dn2 / 1.6021918e-19), (locals.var_cgs_qiu_dn6 / 1.6021918e-19), (locals.var_cgs_qiu_dn7 / 1.6021918e-19), (locals.var_cgs_qiu_dn10 / 1.6021918e-19), (locals.var_cgs_qiu_dn11 / 1.6021918e-19), (locals.var_cgs_qiu_dn12 / 1.6021918e-19), (locals.var_cgs_qiu_dn17 / 1.6021918e-19),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn12, locals.var_rns_dn17,)
    }
};
        locals.var_rns = assign17980_e25535;
        locals.var_rns_dn0 = assign17980_e25535_d_n0;
        locals.var_rns_dn2 = assign17980_e25535_d_n2;
        locals.var_rns_dn6 = assign17980_e25535_d_n6;
        locals.var_rns_dn7 = assign17980_e25535_d_n7;
        locals.var_rns_dn10 = assign17980_e25535_d_n10;
        locals.var_rns_dn11 = assign17980_e25535_d_n11;
        locals.var_rns_dn12 = assign17980_e25535_d_n12;
        locals.var_rns_dn17 = assign17980_e25535_d_n17;
        locals.var_rns_rv = 0.0;

        let (assign17990_e25555, assign17990_e25555_d_n0, assign17990_e25555_d_n2, assign17990_e25555_d_n6, assign17990_e25555_d_n7, assign17990_e25555_d_n10, assign17990_e25555_d_n11, assign17990_e25555_d_n12, assign17990_e25555_d_n17,) = {
    if (locals.var_guard505 != 0.0) {
        let assign17990_e25541: f64 = (p.p96 * locals.var_rns);
        let assign17990_e25543: f64 = (assign17990_e25541 / 100000000000.0);
        let assign17990_e25544: f64 = (p.p95 + assign17990_e25543);
        let assign17990_e25545: f64 = (1.0 / assign17990_e25544);
        let assign17990_e25548: f64 = (locals.var_cgs_mphn0 * locals.var_t8__blk539);
        let assign17990_e25549: f64 = (assign17990_e25545 + assign17990_e25548);
        let assign17990_e25552: f64 = (locals.var_t6__blk541 / p.p106);
        let assign17990_e25553: f64 = (assign17990_e25549 + assign17990_e25552);
        (assign17990_e25553, (((-(((p.p96 * locals.var_rns_dn0) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn0)) + (locals.var_t6__blk541_dn0 / p.p106)), (((-(((p.p96 * locals.var_rns_dn2) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn2)) + (locals.var_t6__blk541_dn2 / p.p106)), (((-(((p.p96 * locals.var_rns_dn6) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn6)) + (locals.var_t6__blk541_dn6 / p.p106)), (((-(((p.p96 * locals.var_rns_dn7) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn7)) + (locals.var_t6__blk541_dn7 / p.p106)), (((-(((p.p96 * locals.var_rns_dn10) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + ((locals.var_cgs_mphn0_dn10 * locals.var_t8__blk539) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn10))) + (locals.var_t6__blk541_dn10 / p.p106)), (((-(((p.p96 * locals.var_rns_dn11) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn11)) + (locals.var_t6__blk541_dn11 / p.p106)), (((-(((p.p96 * locals.var_rns_dn12) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn12)) + (locals.var_t6__blk541_dn12 / p.p106)), (((-(((p.p96 * locals.var_rns_dn17) / 100000000000.0) / (assign17990_e25544 * assign17990_e25544))) + (locals.var_cgs_mphn0 * locals.var_t8__blk539_dn17)) + (locals.var_t6__blk541_dn17 / p.p106)),)
    } else {
        (locals.var_t1__blk533, locals.var_t1__blk533_dn0, locals.var_t1__blk533_dn2, locals.var_t1__blk533_dn6, locals.var_t1__blk533_dn7, locals.var_t1__blk533_dn10, locals.var_t1__blk533_dn11, locals.var_t1__blk533_dn12, locals.var_t1__blk533_dn17,)
    }
};
        locals.var_t1__blk533 = assign17990_e25555;
        locals.var_t1__blk533_dn0 = assign17990_e25555_d_n0;
        locals.var_t1__blk533_dn2 = assign17990_e25555_d_n2;
        locals.var_t1__blk533_dn6 = assign17990_e25555_d_n6;
        locals.var_t1__blk533_dn7 = assign17990_e25555_d_n7;
        locals.var_t1__blk533_dn10 = assign17990_e25555_d_n10;
        locals.var_t1__blk533_dn11 = assign17990_e25555_d_n11;
        locals.var_t1__blk533_dn12 = assign17990_e25555_d_n12;
        locals.var_t1__blk533_dn17 = assign17990_e25555_d_n17;
        locals.var_t1__blk533_rv = 0.0;

    }
}
