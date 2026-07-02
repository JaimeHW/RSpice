#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_48(
        locals: &mut StampLocals,
    ) {
        let (assign13910_e19659, assign13910_e19659_d_n0, assign13910_e19659_d_n2, assign13910_e19659_d_n6, assign13910_e19659_d_n7, assign13910_e19659_d_n10, assign13910_e19659_d_n11, assign13910_e19659_d_n12, assign13910_e19659_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign13910_e19657: f64 = (1e-5 * 1e-5);
        (assign13910_e19657, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13910_e19659;
        locals.var_xmax2_dn0 = assign13910_e19659_d_n0;
        locals.var_xmax2_dn2 = assign13910_e19659_d_n2;
        locals.var_xmax2_dn6 = assign13910_e19659_d_n6;
        locals.var_xmax2_dn7 = assign13910_e19659_d_n7;
        locals.var_xmax2_dn10 = assign13910_e19659_d_n10;
        locals.var_xmax2_dn11 = assign13910_e19659_d_n11;
        locals.var_xmax2_dn12 = assign13910_e19659_d_n12;
        locals.var_xmax2_dn17 = assign13910_e19659_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign13920_e19668, assign13920_e19668_d_n0, assign13920_e19668_d_n2, assign13920_e19668_d_n6, assign13920_e19668_d_n7, assign13920_e19668_d_n10, assign13920_e19668_d_n11, assign13920_e19668_d_n12, assign13920_e19668_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13920_e19668;
        locals.var_xp_dn0 = assign13920_e19668_d_n0;
        locals.var_xp_dn2 = assign13920_e19668_d_n2;
        locals.var_xp_dn6 = assign13920_e19668_d_n6;
        locals.var_xp_dn7 = assign13920_e19668_d_n7;
        locals.var_xp_dn10 = assign13920_e19668_d_n10;
        locals.var_xp_dn11 = assign13920_e19668_d_n11;
        locals.var_xp_dn12 = assign13920_e19668_d_n12;
        locals.var_xp_dn17 = assign13920_e19668_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13930_e19677, assign13930_e19677_d_n0, assign13930_e19677_d_n2, assign13930_e19677_d_n6, assign13930_e19677_d_n7, assign13930_e19677_d_n10, assign13930_e19677_d_n11, assign13930_e19677_d_n12, assign13930_e19677_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13930_e19677;
        locals.var_xmp_dn0 = assign13930_e19677_d_n0;
        locals.var_xmp_dn2 = assign13930_e19677_d_n2;
        locals.var_xmp_dn6 = assign13930_e19677_d_n6;
        locals.var_xmp_dn7 = assign13930_e19677_d_n7;
        locals.var_xmp_dn10 = assign13930_e19677_d_n10;
        locals.var_xmp_dn11 = assign13930_e19677_d_n11;
        locals.var_xmp_dn12 = assign13930_e19677_d_n12;
        locals.var_xmp_dn17 = assign13930_e19677_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign13940_e19686,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13940_e19686;
        locals.var_m0_rv = 0.0;

        let (assign13950_e19695,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13950_e19695;
        locals.var_mm_rv = 0.0;

        let (assign13960_e19704, assign13960_e19704_d_n0, assign13960_e19704_d_n2, assign13960_e19704_d_n6, assign13960_e19704_d_n7, assign13960_e19704_d_n10, assign13960_e19704_d_n11, assign13960_e19704_d_n12, assign13960_e19704_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13960_e19704;
        locals.var_arg_dn0 = assign13960_e19704_d_n0;
        locals.var_arg_dn2 = assign13960_e19704_d_n2;
        locals.var_arg_dn6 = assign13960_e19704_d_n6;
        locals.var_arg_dn7 = assign13960_e19704_d_n7;
        locals.var_arg_dn10 = assign13960_e19704_d_n10;
        locals.var_arg_dn11 = assign13960_e19704_d_n11;
        locals.var_arg_dn12 = assign13960_e19704_d_n12;
        locals.var_arg_dn17 = assign13960_e19704_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign13970_e19713, assign13970_e19713_d_n0, assign13970_e19713_d_n2, assign13970_e19713_d_n6, assign13970_e19713_d_n7, assign13970_e19713_d_n10, assign13970_e19713_d_n11, assign13970_e19713_d_n12, assign13970_e19713_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13970_e19713;
        locals.var_dnm_dn0 = assign13970_e19713_d_n0;
        locals.var_dnm_dn2 = assign13970_e19713_d_n2;
        locals.var_dnm_dn6 = assign13970_e19713_d_n6;
        locals.var_dnm_dn7 = assign13970_e19713_d_n7;
        locals.var_dnm_dn10 = assign13970_e19713_d_n10;
        locals.var_dnm_dn11 = assign13970_e19713_d_n11;
        locals.var_dnm_dn12 = assign13970_e19713_d_n12;
        locals.var_dnm_dn17 = assign13970_e19713_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign13980_e19724, assign13980_e19724_d_n0, assign13980_e19724_d_n2, assign13980_e19724_d_n6, assign13980_e19724_d_n7, assign13980_e19724_d_n10, assign13980_e19724_d_n11, assign13980_e19724_d_n12, assign13980_e19724_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign13980_e19722: f64 = (locals.var_xp * locals.var_x2);
        (assign13980_e19722, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13980_e19724;
        locals.var_xp_dn0 = assign13980_e19724_d_n0;
        locals.var_xp_dn2 = assign13980_e19724_d_n2;
        locals.var_xp_dn6 = assign13980_e19724_d_n6;
        locals.var_xp_dn7 = assign13980_e19724_d_n7;
        locals.var_xp_dn10 = assign13980_e19724_d_n10;
        locals.var_xp_dn11 = assign13980_e19724_d_n11;
        locals.var_xp_dn12 = assign13980_e19724_d_n12;
        locals.var_xp_dn17 = assign13980_e19724_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign13990_e19735, assign13990_e19735_d_n0, assign13990_e19735_d_n2, assign13990_e19735_d_n6, assign13990_e19735_d_n7, assign13990_e19735_d_n10, assign13990_e19735_d_n11, assign13990_e19735_d_n12, assign13990_e19735_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign13990_e19733: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13990_e19733, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13990_e19735;
        locals.var_xmp_dn0 = assign13990_e19735_d_n0;
        locals.var_xmp_dn2 = assign13990_e19735_d_n2;
        locals.var_xmp_dn6 = assign13990_e19735_d_n6;
        locals.var_xmp_dn7 = assign13990_e19735_d_n7;
        locals.var_xmp_dn10 = assign13990_e19735_d_n10;
        locals.var_xmp_dn11 = assign13990_e19735_d_n11;
        locals.var_xmp_dn12 = assign13990_e19735_d_n12;
        locals.var_xmp_dn17 = assign13990_e19735_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign14000_e19746, assign14000_e19746_d_n0, assign14000_e19746_d_n2, assign14000_e19746_d_n6, assign14000_e19746_d_n7, assign14000_e19746_d_n10, assign14000_e19746_d_n11, assign14000_e19746_d_n12, assign14000_e19746_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14000_e19744: f64 = (locals.var_xp * locals.var_x2);
        (assign14000_e19744, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign14000_e19746;
        locals.var_xp_dn0 = assign14000_e19746_d_n0;
        locals.var_xp_dn2 = assign14000_e19746_d_n2;
        locals.var_xp_dn6 = assign14000_e19746_d_n6;
        locals.var_xp_dn7 = assign14000_e19746_d_n7;
        locals.var_xp_dn10 = assign14000_e19746_d_n10;
        locals.var_xp_dn11 = assign14000_e19746_d_n11;
        locals.var_xp_dn12 = assign14000_e19746_d_n12;
        locals.var_xp_dn17 = assign14000_e19746_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign14010_e19757, assign14010_e19757_d_n0, assign14010_e19757_d_n2, assign14010_e19757_d_n6, assign14010_e19757_d_n7, assign14010_e19757_d_n10, assign14010_e19757_d_n11, assign14010_e19757_d_n12, assign14010_e19757_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14010_e19755: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign14010_e19755, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign14010_e19757;
        locals.var_xmp_dn0 = assign14010_e19757_d_n0;
        locals.var_xmp_dn2 = assign14010_e19757_d_n2;
        locals.var_xmp_dn6 = assign14010_e19757_d_n6;
        locals.var_xmp_dn7 = assign14010_e19757_d_n7;
        locals.var_xmp_dn10 = assign14010_e19757_d_n10;
        locals.var_xmp_dn11 = assign14010_e19757_d_n11;
        locals.var_xmp_dn12 = assign14010_e19757_d_n12;
        locals.var_xmp_dn17 = assign14010_e19757_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign14020_e19768, assign14020_e19768_d_n0, assign14020_e19768_d_n2, assign14020_e19768_d_n6, assign14020_e19768_d_n7, assign14020_e19768_d_n10, assign14020_e19768_d_n11, assign14020_e19768_d_n12, assign14020_e19768_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14020_e19766: f64 = (locals.var_xp + locals.var_xmp);
        (assign14020_e19766, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign14020_e19768;
        locals.var_arg_dn0 = assign14020_e19768_d_n0;
        locals.var_arg_dn2 = assign14020_e19768_d_n2;
        locals.var_arg_dn6 = assign14020_e19768_d_n6;
        locals.var_arg_dn7 = assign14020_e19768_d_n7;
        locals.var_arg_dn10 = assign14020_e19768_d_n10;
        locals.var_arg_dn11 = assign14020_e19768_d_n11;
        locals.var_arg_dn12 = assign14020_e19768_d_n12;
        locals.var_arg_dn17 = assign14020_e19768_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign14030_e19777, assign14030_e19777_d_n0, assign14030_e19777_d_n2, assign14030_e19777_d_n6, assign14030_e19777_d_n7, assign14030_e19777_d_n10, assign14030_e19777_d_n11, assign14030_e19777_d_n12, assign14030_e19777_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14030_e19777;
        locals.var_dnm_dn0 = assign14030_e19777_d_n0;
        locals.var_dnm_dn2 = assign14030_e19777_d_n2;
        locals.var_dnm_dn6 = assign14030_e19777_d_n6;
        locals.var_dnm_dn7 = assign14030_e19777_d_n7;
        locals.var_dnm_dn10 = assign14030_e19777_d_n10;
        locals.var_dnm_dn11 = assign14030_e19777_d_n11;
        locals.var_dnm_dn12 = assign14030_e19777_d_n12;
        locals.var_dnm_dn17 = assign14030_e19777_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign14040_e19792: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard432 = assign14040_e19792;
        locals.var_guard432_rv = 0.0;

        let assign14050_e19795: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign14050_e19795;
        locals.var_guard433_rv = 0.0;

        let (assign14060_e19808,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_guard433 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14060_e19808;
        locals.var_mm_rv = 0.0;

        let assign14070_e19811: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign14070_e19811;
        locals.var_guard434_rv = 0.0;

        let (assign14080_e19827,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14080_e19827;
        locals.var_mm_rv = 0.0;

        let assign14090_e19830: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign14090_e19830;
        locals.var_guard435_rv = 0.0;

        let (assign14100_e19849,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14100_e19849;
        locals.var_mm_rv = 0.0;

        let assign14110_e19852: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign14110_e19852;
        locals.var_guard436_rv = 0.0;

        let (assign14120_e19874,) = {
    if ((((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 == 0.0)) && (locals.var_guard436 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14120_e19874;
        locals.var_mm_rv = 0.0;

        let (assign14130_e19885,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign14130_e19885;
        locals.var_m0_rv = 0.0;

        let mut assign14140_loop_guard: usize = 0;
        while {
            let assign14140_cond_e19897: f64 = if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign14140_cond_e19897 != 0.0
        } {
            assign14140_loop_guard += 1;
            assert!(assign14140_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign14140_body0_e19909, assign14140_body0_e19909_d_n0, assign14140_body0_e19909_d_n2, assign14140_body0_e19909_d_n6, assign14140_body0_e19909_d_n7, assign14140_body0_e19909_d_n10, assign14140_body0_e19909_d_n11, assign14140_body0_e19909_d_n12, assign14140_body0_e19909_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) {
        let assign14140_body0_e19907: f64 = (locals.var_dnm).sqrt();
        (assign14140_body0_e19907, (locals.var_dnm_dn0 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn2 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn6 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn7 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn10 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn11 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn12 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn17 / (2.0 * assign14140_body0_e19907)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign14140_body0_e19909;
            locals.var_dnm_dn0 = assign14140_body0_e19909_d_n0;
            locals.var_dnm_dn2 = assign14140_body0_e19909_d_n2;
            locals.var_dnm_dn6 = assign14140_body0_e19909_d_n6;
            locals.var_dnm_dn7 = assign14140_body0_e19909_d_n7;
            locals.var_dnm_dn10 = assign14140_body0_e19909_d_n10;
            locals.var_dnm_dn11 = assign14140_body0_e19909_d_n11;
            locals.var_dnm_dn12 = assign14140_body0_e19909_d_n12;
            locals.var_dnm_dn17 = assign14140_body0_e19909_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign14140_body1_e19922,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) {
        let assign14140_body1_e19920: f64 = (locals.var_m0 + 1.0);
        (assign14140_body1_e19920,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign14140_body1_e19922;
            locals.var_m0_rv = 0.0;
        }

        let (assign14150_e19940, assign14150_e19940_d_n0, assign14150_e19940_d_n2, assign14150_e19940_d_n6, assign14150_e19940_d_n7, assign14150_e19940_d_n10, assign14150_e19940_d_n11, assign14150_e19940_d_n12, assign14150_e19940_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 == 0.0)) {
        let assign14150_e19936: f64 = (2.0 * 2.0);
        let assign14150_e19937: f64 = (1.0 / assign14150_e19936);
        let assign14150_e19938: f64 = (locals.var_dnm).powf(assign14150_e19937);
        (assign14150_e19938, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn0)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn2)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn6)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn7)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn10)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn11)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn12)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn17)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14150_e19940;
        locals.var_dnm_dn0 = assign14150_e19940_d_n0;
        locals.var_dnm_dn2 = assign14150_e19940_d_n2;
        locals.var_dnm_dn6 = assign14150_e19940_d_n6;
        locals.var_dnm_dn7 = assign14150_e19940_d_n7;
        locals.var_dnm_dn10 = assign14150_e19940_d_n10;
        locals.var_dnm_dn11 = assign14150_e19940_d_n11;
        locals.var_dnm_dn12 = assign14150_e19940_d_n12;
        locals.var_dnm_dn17 = assign14150_e19940_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign14160_e19951, assign14160_e19951_d_n0, assign14160_e19951_d_n2, assign14160_e19951_d_n6, assign14160_e19951_d_n7, assign14160_e19951_d_n10, assign14160_e19951_d_n11, assign14160_e19951_d_n12, assign14160_e19951_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14160_e19949: f64 = (1.0 / locals.var_dnm);
        (assign14160_e19949, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14160_e19951;
        locals.var_dnm_dn0 = assign14160_e19951_d_n0;
        locals.var_dnm_dn2 = assign14160_e19951_d_n2;
        locals.var_dnm_dn6 = assign14160_e19951_d_n6;
        locals.var_dnm_dn7 = assign14160_e19951_d_n7;
        locals.var_dnm_dn10 = assign14160_e19951_d_n10;
        locals.var_dnm_dn11 = assign14160_e19951_d_n11;
        locals.var_dnm_dn12 = assign14160_e19951_d_n12;
        locals.var_dnm_dn17 = assign14160_e19951_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign14170_e19964, assign14170_e19964_d_n0, assign14170_e19964_d_n2, assign14170_e19964_d_n6, assign14170_e19964_d_n7, assign14170_e19964_d_n10, assign14170_e19964_d_n11, assign14170_e19964_d_n12, assign14170_e19964_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14170_e19960: f64 = (locals.var_tmf1 * 1e-5);
        let assign14170_e19962: f64 = (assign14170_e19960 * locals.var_dnm);
        (assign14170_e19962, (((locals.var_tmf1_dn0 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign14170_e19964;
        locals.var_tmf0_dn0 = assign14170_e19964_d_n0;
        locals.var_tmf0_dn2 = assign14170_e19964_d_n2;
        locals.var_tmf0_dn6 = assign14170_e19964_d_n6;
        locals.var_tmf0_dn7 = assign14170_e19964_d_n7;
        locals.var_tmf0_dn10 = assign14170_e19964_d_n10;
        locals.var_tmf0_dn11 = assign14170_e19964_d_n11;
        locals.var_tmf0_dn12 = assign14170_e19964_d_n12;
        locals.var_tmf0_dn17 = assign14170_e19964_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign14180_e19977, assign14180_e19977_d_n0, assign14180_e19977_d_n2, assign14180_e19977_d_n6, assign14180_e19977_d_n7, assign14180_e19977_d_n10, assign14180_e19977_d_n11, assign14180_e19977_d_n12, assign14180_e19977_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14180_e19973: f64 = 1e-5;
        let assign14180_e19975: f64 = (assign14180_e19973 - locals.var_tmf0);
        (assign14180_e19975, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14180_e19977;
        locals.var_rrr_eta_dn0 = assign14180_e19977_d_n0;
        locals.var_rrr_eta_dn2 = assign14180_e19977_d_n2;
        locals.var_rrr_eta_dn6 = assign14180_e19977_d_n6;
        locals.var_rrr_eta_dn7 = assign14180_e19977_d_n7;
        locals.var_rrr_eta_dn10 = assign14180_e19977_d_n10;
        locals.var_rrr_eta_dn11 = assign14180_e19977_d_n11;
        locals.var_rrr_eta_dn12 = assign14180_e19977_d_n12;
        locals.var_rrr_eta_dn17 = assign14180_e19977_d_n17;
        locals.var_rrr_eta_rv = 0.0;

        let (assign14190_e19987, assign14190_e19987_d_n0, assign14190_e19987_d_n2, assign14190_e19987_d_n6, assign14190_e19987_d_n7, assign14190_e19987_d_n10, assign14190_e19987_d_n11, assign14190_e19987_d_n12, assign14190_e19987_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14190_e19987;
        locals.var_rrr_eta_dn0 = assign14190_e19987_d_n0;
        locals.var_rrr_eta_dn2 = assign14190_e19987_d_n2;
        locals.var_rrr_eta_dn6 = assign14190_e19987_d_n6;
        locals.var_rrr_eta_dn7 = assign14190_e19987_d_n7;
        locals.var_rrr_eta_dn10 = assign14190_e19987_d_n10;
        locals.var_rrr_eta_dn11 = assign14190_e19987_d_n11;
        locals.var_rrr_eta_dn12 = assign14190_e19987_d_n12;
        locals.var_rrr_eta_dn17 = assign14190_e19987_d_n17;
        locals.var_rrr_eta_rv = 0.0;

        let (assign14200_e19994, assign14200_e19994_d_n0, assign14200_e19994_d_n2, assign14200_e19994_d_n6, assign14200_e19994_d_n7, assign14200_e19994_d_n10, assign14200_e19994_d_n11, assign14200_e19994_d_n12, assign14200_e19994_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign14200_e19994;
        locals.var_alpha_dn0 = assign14200_e19994_d_n0;
        locals.var_alpha_dn2 = assign14200_e19994_d_n2;
        locals.var_alpha_dn6 = assign14200_e19994_d_n6;
        locals.var_alpha_dn7 = assign14200_e19994_d_n7;
        locals.var_alpha_dn10 = assign14200_e19994_d_n10;
        locals.var_alpha_dn11 = assign14200_e19994_d_n11;
        locals.var_alpha_dn12 = assign14200_e19994_d_n12;
        locals.var_alpha_dn17 = assign14200_e19994_d_n17;
        locals.var_alpha_rv = 0.0;

        let (assign14210_e20007, assign14210_e20007_d_n0, assign14210_e20007_d_n2, assign14210_e20007_d_n6, assign14210_e20007_d_n7, assign14210_e20007_d_n10, assign14210_e20007_d_n11, assign14210_e20007_d_n12, assign14210_e20007_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign14210_e20003: f64 = (1.0 + locals.var_alpha);
        let assign14210_e20004: f64 = (locals.var_alpha * assign14210_e20003);
        let assign14210_e20005: f64 = (1.0 + assign14210_e20004);
        (assign14210_e20005, ((locals.var_alpha_dn0 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign14210_e20007;
        locals.var_qinm_dn0 = assign14210_e20007_d_n0;
        locals.var_qinm_dn2 = assign14210_e20007_d_n2;
        locals.var_qinm_dn6 = assign14210_e20007_d_n6;
        locals.var_qinm_dn7 = assign14210_e20007_d_n7;
        locals.var_qinm_dn10 = assign14210_e20007_d_n10;
        locals.var_qinm_dn11 = assign14210_e20007_d_n11;
        locals.var_qinm_dn12 = assign14210_e20007_d_n12;
        locals.var_qinm_dn17 = assign14210_e20007_d_n17;
        locals.var_qinm_rv = 0.0;

        let (assign14220_e20027, assign14220_e20027_d_n0, assign14220_e20027_d_n2, assign14220_e20027_d_n6, assign14220_e20027_d_n7, assign14220_e20027_d_n10, assign14220_e20027_d_n11, assign14220_e20027_d_n12, assign14220_e20027_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign14220_e20014: f64 = (1.0 + locals.var_alpha);
        let assign14220_e20017: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14220_e20025, assign14220_e20025_d_n0, assign14220_e20025_d_n2, assign14220_e20025_d_n6, assign14220_e20025_d_n7, assign14220_e20025_d_n10, assign14220_e20025_d_n11, assign14220_e20025_d_n12, assign14220_e20025_d_n17,) = {
            if (assign14220_e20014 >= assign14220_e20017) {
                let assign14220_e20021: f64 = (1.0 + locals.var_alpha);
                (assign14220_e20021, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign14220_e20024: f64 = (10.0 * 2.220446049250313e-16);
                (assign14220_e20024, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14220_e20025, assign14220_e20025_d_n0, assign14220_e20025_d_n2, assign14220_e20025_d_n6, assign14220_e20025_d_n7, assign14220_e20025_d_n10, assign14220_e20025_d_n11, assign14220_e20025_d_n12, assign14220_e20025_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign14220_e20027;
        locals.var_qidn_dn0 = assign14220_e20027_d_n0;
        locals.var_qidn_dn2 = assign14220_e20027_d_n2;
        locals.var_qidn_dn6 = assign14220_e20027_d_n6;
        locals.var_qidn_dn7 = assign14220_e20027_d_n7;
        locals.var_qidn_dn10 = assign14220_e20027_d_n10;
        locals.var_qidn_dn11 = assign14220_e20027_d_n11;
        locals.var_qidn_dn12 = assign14220_e20027_d_n12;
        locals.var_qidn_dn17 = assign14220_e20027_d_n17;
        locals.var_qidn_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14230_e20039, assign14230_e20039_d_n0, assign14230_e20039_d_n2, assign14230_e20039_d_n6, assign14230_e20039_d_n7, assign14230_e20039_d_n10, assign14230_e20039_d_n11, assign14230_e20039_d_n12, assign14230_e20039_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign14230_e20033: f64 = (-0.5);
        let assign14230_e20036: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign14230_e20037: f64 = (assign14230_e20033 * assign14230_e20036);
        (assign14230_e20037, (assign14230_e20033 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign14230_e20033 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign14230_e20033 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign14230_e20033 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign14230_e20033 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign14230_e20033 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign14230_e20033 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign14230_e20033 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign14230_e20039;
        locals.var_qiu_dn0 = assign14230_e20039_d_n0;
        locals.var_qiu_dn2 = assign14230_e20039_d_n2;
        locals.var_qiu_dn6 = assign14230_e20039_d_n6;
        locals.var_qiu_dn7 = assign14230_e20039_d_n7;
        locals.var_qiu_dn10 = assign14230_e20039_d_n10;
        locals.var_qiu_dn11 = assign14230_e20039_d_n11;
        locals.var_qiu_dn12 = assign14230_e20039_d_n12;
        locals.var_qiu_dn17 = assign14230_e20039_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign14300_e20072, assign14300_e20072_d_n0, assign14300_e20072_d_n2, assign14300_e20072_d_n6, assign14300_e20072_d_n7, assign14300_e20072_d_n10, assign14300_e20072_d_n11, assign14300_e20072_d_n12, assign14300_e20072_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    } else {
        (locals.var_vbcs_cl, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    }
};
        locals.var_vbcs_cl = assign14300_e20072;
        locals.var_vbcs_cl_dn0 = assign14300_e20072_d_n0;
        locals.var_vbcs_cl_dn2 = assign14300_e20072_d_n2;
        locals.var_vbcs_cl_dn6 = assign14300_e20072_d_n6;
        locals.var_vbcs_cl_dn7 = assign14300_e20072_d_n7;
        locals.var_vbcs_cl_dn10 = assign14300_e20072_d_n10;
        locals.var_vbcs_cl_dn11 = assign14300_e20072_d_n11;
        locals.var_vbcs_cl_dn12 = assign14300_e20072_d_n12;
        locals.var_vbcs_cl_dn17 = assign14300_e20072_d_n17;
        locals.var_vbcs_cl_rv = 0.0;

        let assign14310_e20075: f64 = if locals.var_wdsoi_ini < p.p237 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign14310_e20075;
        locals.var_guard443_rv = 0.0;

        let (assign14320_e20082,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard443 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14320_e20082;
        locals.var_flg_depmode_rv = 0.0;

        let (assign14330_e20090,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard443 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14330_e20090;
        locals.var_flg_depmode_rv = 0.0;

        let (assign14340_e20101, assign14340_e20101_d_n0, assign14340_e20101_d_n2, assign14340_e20101_d_n6, assign14340_e20101_d_n7, assign14340_e20101_d_n10, assign14340_e20101_d_n11, assign14340_e20101_d_n12, assign14340_e20101_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign14340_e20095: f64 = (locals.var_vfb - locals.var_dvth);
        let assign14340_e20097: f64 = (assign14340_e20095 + locals.var_dppg);
        let assign14340_e20099: f64 = (assign14340_e20097 + locals.var_vbcs_cl);
        (assign14340_e20099, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbcs_cl_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbcs_cl_dn2), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbcs_cl_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbcs_cl_dn7), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbcs_cl_dn10), (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbcs_cl_dn11), (((-locals.var_dvth_dn12) + locals.var_dppg_dn12) + locals.var_vbcs_cl_dn12), (((-locals.var_dvth_dn17) + locals.var_dppg_dn17) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12, locals.var_vgs_fb_dn17,)
    }
};
        locals.var_vgs_fb = assign14340_e20101;
        locals.var_vgs_fb_dn0 = assign14340_e20101_d_n0;
        locals.var_vgs_fb_dn2 = assign14340_e20101_d_n2;
        locals.var_vgs_fb_dn6 = assign14340_e20101_d_n6;
        locals.var_vgs_fb_dn7 = assign14340_e20101_d_n7;
        locals.var_vgs_fb_dn10 = assign14340_e20101_d_n10;
        locals.var_vgs_fb_dn11 = assign14340_e20101_d_n11;
        locals.var_vgs_fb_dn12 = assign14340_e20101_d_n12;
        locals.var_vgs_fb_dn17 = assign14340_e20101_d_n17;
        locals.var_vgs_fb_rv = 0.0;

        let assign14350_e20104: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard444 = assign14350_e20104;
        locals.var_guard444_rv = 0.0;

        let (assign14360_e20112,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14360_e20110: f64 = (-1.0);
        (assign14360_e20110,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign14360_e20112;
        locals.var_flg_zone_rv = 0.0;

        let (assign14370_e20127, assign14370_e20127_d_n0, assign14370_e20127_d_n2, assign14370_e20127_d_n6, assign14370_e20127_d_n7, assign14370_e20127_d_n10, assign14370_e20127_d_n11, assign14370_e20127_d_n12, assign14370_e20127_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14370_e20119: f64 = (2.0 * locals.var_beta_inv);
        let assign14370_e20121: f64 = (-locals.var_vgs_min);
        let assign14370_e20123: f64 = (assign14370_e20121 / locals.var_fac1);
        let assign14370_e20124: f64 = (assign14370_e20123).ln();
        let assign14370_e20125: f64 = (assign14370_e20119 * assign14370_e20124);
        (assign14370_e20125, (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (((2.0 * locals.var_beta_inv_dn10) * assign14370_e20124) + (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123))), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign14370_e20127;
        locals.var_ps0_min_dn0 = assign14370_e20127_d_n0;
        locals.var_ps0_min_dn2 = assign14370_e20127_d_n2;
        locals.var_ps0_min_dn6 = assign14370_e20127_d_n6;
        locals.var_ps0_min_dn7 = assign14370_e20127_d_n7;
        locals.var_ps0_min_dn10 = assign14370_e20127_d_n10;
        locals.var_ps0_min_dn11 = assign14370_e20127_d_n11;
        locals.var_ps0_min_dn12 = assign14370_e20127_d_n12;
        locals.var_ps0_min_dn17 = assign14370_e20127_d_n17;
        locals.var_ps0_min_rv = 0.0;

        let (assign14380_e20138, assign14380_e20138_d_n0, assign14380_e20138_d_n2, assign14380_e20138_d_n6, assign14380_e20138_d_n7, assign14380_e20138_d_n10, assign14380_e20138_d_n11, assign14380_e20138_d_n12, assign14380_e20138_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14380_e20135: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14380_e20136: f64 = (locals.var_beta * assign14380_e20135);
        (assign14380_e20136, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14380_e20135) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14380_e20138;
        locals.var_tx_dn0 = assign14380_e20138_d_n0;
        locals.var_tx_dn2 = assign14380_e20138_d_n2;
        locals.var_tx_dn6 = assign14380_e20138_d_n6;
        locals.var_tx_dn7 = assign14380_e20138_d_n7;
        locals.var_tx_dn10 = assign14380_e20138_d_n10;
        locals.var_tx_dn11 = assign14380_e20138_d_n11;
        locals.var_tx_dn12 = assign14380_e20138_d_n12;
        locals.var_tx_dn17 = assign14380_e20138_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14390_e20149, assign14390_e20149_d_n0, assign14390_e20149_d_n2, assign14390_e20149_d_n6, assign14390_e20149_d_n7, assign14390_e20149_d_n10, assign14390_e20149_d_n11, assign14390_e20149_d_n12, assign14390_e20149_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14390_e20146: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign14390_e20147: f64 = (1.0 / assign14390_e20146);
        (assign14390_e20147, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign14390_e20146 * assign14390_e20146))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign14390_e20146 * assign14390_e20146))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14390_e20149;
        locals.var_t1_dn0 = assign14390_e20149_d_n0;
        locals.var_t1_dn2 = assign14390_e20149_d_n2;
        locals.var_t1_dn6 = assign14390_e20149_d_n6;
        locals.var_t1_dn7 = assign14390_e20149_d_n7;
        locals.var_t1_dn10 = assign14390_e20149_d_n10;
        locals.var_t1_dn11 = assign14390_e20149_d_n11;
        locals.var_t1_dn12 = assign14390_e20149_d_n12;
        locals.var_t1_dn17 = assign14390_e20149_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14400_e20158, assign14400_e20158_d_n0, assign14400_e20158_d_n2, assign14400_e20158_d_n6, assign14400_e20158_d_n7, assign14400_e20158_d_n10, assign14400_e20158_d_n11, assign14400_e20158_d_n12, assign14400_e20158_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14400_e20156: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign14400_e20156, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14400_e20158;
        locals.var_ty_dn0 = assign14400_e20158_d_n0;
        locals.var_ty_dn2 = assign14400_e20158_d_n2;
        locals.var_ty_dn6 = assign14400_e20158_d_n6;
        locals.var_ty_dn7 = assign14400_e20158_d_n7;
        locals.var_ty_dn10 = assign14400_e20158_d_n10;
        locals.var_ty_dn11 = assign14400_e20158_d_n11;
        locals.var_ty_dn12 = assign14400_e20158_d_n12;
        locals.var_ty_dn17 = assign14400_e20158_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign14410_e20171, assign14410_e20171_d_n0, assign14410_e20171_d_n2, assign14410_e20171_d_n6, assign14410_e20171_d_n7, assign14410_e20171_d_n10, assign14410_e20171_d_n11, assign14410_e20171_d_n12, assign14410_e20171_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14410_e20166: f64 = (3.0 * 1.414213562373095);
        let assign14410_e20168: f64 = (assign14410_e20166 * locals.var_ty);
        let assign14410_e20169: f64 = (2.0 + assign14410_e20168);
        (assign14410_e20169, (assign14410_e20166 * locals.var_ty_dn0), (assign14410_e20166 * locals.var_ty_dn2), (assign14410_e20166 * locals.var_ty_dn6), (assign14410_e20166 * locals.var_ty_dn7), (assign14410_e20166 * locals.var_ty_dn10), (assign14410_e20166 * locals.var_ty_dn11), (assign14410_e20166 * locals.var_ty_dn12), (assign14410_e20166 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign14410_e20171;
        locals.var_ac41_dn0 = assign14410_e20171_d_n0;
        locals.var_ac41_dn2 = assign14410_e20171_d_n2;
        locals.var_ac41_dn6 = assign14410_e20171_d_n6;
        locals.var_ac41_dn7 = assign14410_e20171_d_n7;
        locals.var_ac41_dn10 = assign14410_e20171_d_n10;
        locals.var_ac41_dn11 = assign14410_e20171_d_n11;
        locals.var_ac41_dn12 = assign14410_e20171_d_n12;
        locals.var_ac41_dn17 = assign14410_e20171_d_n17;
        locals.var_ac41_rv = 0.0;

        let (assign14420_e20184, assign14420_e20184_d_n0, assign14420_e20184_d_n2, assign14420_e20184_d_n6, assign14420_e20184_d_n7, assign14420_e20184_d_n10, assign14420_e20184_d_n11, assign14420_e20184_d_n12, assign14420_e20184_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14420_e20178: f64 = (8.0 * locals.var_ac41);
        let assign14420_e20180: f64 = (assign14420_e20178 * locals.var_ac41);
        let assign14420_e20182: f64 = (assign14420_e20180 * locals.var_ac41);
        (assign14420_e20182, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign14420_e20184;
        locals.var_ac4_dn0 = assign14420_e20184_d_n0;
        locals.var_ac4_dn2 = assign14420_e20184_d_n2;
        locals.var_ac4_dn6 = assign14420_e20184_d_n6;
        locals.var_ac4_dn7 = assign14420_e20184_d_n7;
        locals.var_ac4_dn10 = assign14420_e20184_d_n10;
        locals.var_ac4_dn11 = assign14420_e20184_d_n11;
        locals.var_ac4_dn12 = assign14420_e20184_d_n12;
        locals.var_ac4_dn17 = assign14420_e20184_d_n17;
        locals.var_ac4_rv = 0.0;

        let (assign14430_e20193, assign14430_e20193_d_n0, assign14430_e20193_d_n2, assign14430_e20193_d_n6, assign14430_e20193_d_n7, assign14430_e20193_d_n10, assign14430_e20193_d_n11, assign14430_e20193_d_n12, assign14430_e20193_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14430_e20191: f64 = (locals.var_tx - 2.0);
        (assign14430_e20191, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14430_e20193;
        locals.var_t4_dn0 = assign14430_e20193_d_n0;
        locals.var_t4_dn2 = assign14430_e20193_d_n2;
        locals.var_t4_dn6 = assign14430_e20193_d_n6;
        locals.var_t4_dn7 = assign14430_e20193_d_n7;
        locals.var_t4_dn10 = assign14430_e20193_d_n10;
        locals.var_t4_dn11 = assign14430_e20193_d_n11;
        locals.var_t4_dn12 = assign14430_e20193_d_n12;
        locals.var_t4_dn17 = assign14430_e20193_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14440_e20204, assign14440_e20204_d_n0, assign14440_e20204_d_n2, assign14440_e20204_d_n6, assign14440_e20204_d_n7, assign14440_e20204_d_n10, assign14440_e20204_d_n11, assign14440_e20204_d_n12, assign14440_e20204_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14440_e20200: f64 = (9.0 * locals.var_ty);
        let assign14440_e20202: f64 = (assign14440_e20200 * locals.var_t4);
        (assign14440_e20202, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14440_e20204;
        locals.var_t5_dn0 = assign14440_e20204_d_n0;
        locals.var_t5_dn2 = assign14440_e20204_d_n2;
        locals.var_t5_dn6 = assign14440_e20204_d_n6;
        locals.var_t5_dn7 = assign14440_e20204_d_n7;
        locals.var_t5_dn10 = assign14440_e20204_d_n10;
        locals.var_t5_dn11 = assign14440_e20204_d_n11;
        locals.var_t5_dn12 = assign14440_e20204_d_n12;
        locals.var_t5_dn17 = assign14440_e20204_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign14450_e20215, assign14450_e20215_d_n0, assign14450_e20215_d_n2, assign14450_e20215_d_n6, assign14450_e20215_d_n7, assign14450_e20215_d_n10, assign14450_e20215_d_n11, assign14450_e20215_d_n12, assign14450_e20215_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14450_e20211: f64 = (7.0 * 1.414213562373095);
        let assign14450_e20213: f64 = (assign14450_e20211 - locals.var_t5);
        (assign14450_e20213, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign14450_e20215;
        locals.var_ac31_dn0 = assign14450_e20215_d_n0;
        locals.var_ac31_dn2 = assign14450_e20215_d_n2;
        locals.var_ac31_dn6 = assign14450_e20215_d_n6;
        locals.var_ac31_dn7 = assign14450_e20215_d_n7;
        locals.var_ac31_dn10 = assign14450_e20215_d_n10;
        locals.var_ac31_dn11 = assign14450_e20215_d_n11;
        locals.var_ac31_dn12 = assign14450_e20215_d_n12;
        locals.var_ac31_dn17 = assign14450_e20215_d_n17;
        locals.var_ac31_rv = 0.0;

        let (assign14460_e20224, assign14460_e20224_d_n0, assign14460_e20224_d_n2, assign14460_e20224_d_n6, assign14460_e20224_d_n7, assign14460_e20224_d_n10, assign14460_e20224_d_n11, assign14460_e20224_d_n12, assign14460_e20224_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14460_e20222: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign14460_e20222, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign14460_e20224;
        locals.var_ac3_dn0 = assign14460_e20224_d_n0;
        locals.var_ac3_dn2 = assign14460_e20224_d_n2;
        locals.var_ac3_dn6 = assign14460_e20224_d_n6;
        locals.var_ac3_dn7 = assign14460_e20224_d_n7;
        locals.var_ac3_dn10 = assign14460_e20224_d_n10;
        locals.var_ac3_dn11 = assign14460_e20224_d_n11;
        locals.var_ac3_dn12 = assign14460_e20224_d_n12;
        locals.var_ac3_dn17 = assign14460_e20224_d_n17;
        locals.var_ac3_rv = 0.0;

        let assign14470_e20228: f64 = (locals.var_ac3 * 1e-8);
        let assign14470_e20229: f64 = if locals.var_ac4 < assign14470_e20228 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign14470_e20229;
        locals.var_guard445_rv = 0.0;

        let (assign14480_e20251, assign14480_e20251_d_n0, assign14480_e20251_d_n2, assign14480_e20251_d_n6, assign14480_e20251_d_n7, assign14480_e20251_d_n10, assign14480_e20251_d_n11, assign14480_e20251_d_n12, assign14480_e20251_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14480_e20237: f64 = (-7.0);
        let assign14480_e20239: f64 = (assign14480_e20237 * 1.414213562373095);
        let assign14480_e20241: f64 = (assign14480_e20239 + locals.var_ac31);
        let assign14480_e20244: f64 = (0.5 * locals.var_ac4);
        let assign14480_e20246: f64 = (assign14480_e20244 / locals.var_ac31);
        let assign14480_e20247: f64 = (assign14480_e20241 + assign14480_e20246);
        let assign14480_e20249: f64 = (assign14480_e20247 + locals.var_t5);
        (assign14480_e20249, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14480_e20251;
        locals.var_ac1_dn0 = assign14480_e20251_d_n0;
        locals.var_ac1_dn2 = assign14480_e20251_d_n2;
        locals.var_ac1_dn6 = assign14480_e20251_d_n6;
        locals.var_ac1_dn7 = assign14480_e20251_d_n7;
        locals.var_ac1_dn10 = assign14480_e20251_d_n10;
        locals.var_ac1_dn11 = assign14480_e20251_d_n11;
        locals.var_ac1_dn12 = assign14480_e20251_d_n12;
        locals.var_ac1_dn17 = assign14480_e20251_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign14490_e20264, assign14490_e20264_d_n0, assign14490_e20264_d_n2, assign14490_e20264_d_n6, assign14490_e20264_d_n7, assign14490_e20264_d_n10, assign14490_e20264_d_n11, assign14490_e20264_d_n12, assign14490_e20264_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard445 == 0.0)) {
        let assign14490_e20261: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign14490_e20262: f64 = (assign14490_e20261).sqrt();
        (assign14490_e20262, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign14490_e20262)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign14490_e20264;
        locals.var_ac2_dn0 = assign14490_e20264_d_n0;
        locals.var_ac2_dn2 = assign14490_e20264_d_n2;
        locals.var_ac2_dn6 = assign14490_e20264_d_n6;
        locals.var_ac2_dn7 = assign14490_e20264_d_n7;
        locals.var_ac2_dn10 = assign14490_e20264_d_n10;
        locals.var_ac2_dn11 = assign14490_e20264_d_n11;
        locals.var_ac2_dn12 = assign14490_e20264_d_n12;
        locals.var_ac2_dn17 = assign14490_e20264_d_n17;
        locals.var_ac2_rv = 0.0;

        let (assign14500_e20281, assign14500_e20281_d_n0, assign14500_e20281_d_n2, assign14500_e20281_d_n6, assign14500_e20281_d_n7, assign14500_e20281_d_n10, assign14500_e20281_d_n11, assign14500_e20281_d_n12, assign14500_e20281_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard445 == 0.0)) {
        let assign14500_e20273: f64 = (-7.0);
        let assign14500_e20275: f64 = (assign14500_e20273 * 1.414213562373095);
        let assign14500_e20277: f64 = (assign14500_e20275 + locals.var_ac2);
        let assign14500_e20279: f64 = (assign14500_e20277 + locals.var_t5);
        (assign14500_e20279, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14500_e20281;
        locals.var_ac1_dn0 = assign14500_e20281_d_n0;
        locals.var_ac1_dn2 = assign14500_e20281_d_n2;
        locals.var_ac1_dn6 = assign14500_e20281_d_n6;
        locals.var_ac1_dn7 = assign14500_e20281_d_n7;
        locals.var_ac1_dn10 = assign14500_e20281_d_n10;
        locals.var_ac1_dn11 = assign14500_e20281_d_n11;
        locals.var_ac1_dn12 = assign14500_e20281_d_n12;
        locals.var_ac1_dn17 = assign14500_e20281_d_n17;
        locals.var_ac1_rv = 0.0;

        let (assign14510_e20290, assign14510_e20290_d_n0, assign14510_e20290_d_n2, assign14510_e20290_d_n6, assign14510_e20290_d_n7, assign14510_e20290_d_n10, assign14510_e20290_d_n11, assign14510_e20290_d_n12, assign14510_e20290_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14510_e20288: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign14510_e20288, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign14510_e20290;
        locals.var_acd_dn0 = assign14510_e20290_d_n0;
        locals.var_acd_dn2 = assign14510_e20290_d_n2;
        locals.var_acd_dn6 = assign14510_e20290_d_n6;
        locals.var_acd_dn7 = assign14510_e20290_d_n7;
        locals.var_acd_dn10 = assign14510_e20290_d_n10;
        locals.var_acd_dn11 = assign14510_e20290_d_n11;
        locals.var_acd_dn12 = assign14510_e20290_d_n12;
        locals.var_acd_dn17 = assign14510_e20290_d_n17;
        locals.var_acd_rv = 0.0;

        let (assign14520_e20314, assign14520_e20314_d_n0, assign14520_e20314_d_n2, assign14520_e20314_d_n6, assign14520_e20314_d_n7, assign14520_e20314_d_n10, assign14520_e20314_d_n11, assign14520_e20314_d_n12, assign14520_e20314_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14520_e20296: f64 = (-4.0);
        let assign14520_e20298: f64 = (assign14520_e20296 * 1.414213562373095);
        let assign14520_e20301: f64 = (12.0 * locals.var_ty);
        let assign14520_e20302: f64 = (assign14520_e20298 - assign14520_e20301);
        let assign14520_e20305: f64 = (2.0 * locals.var_acd);
        let assign14520_e20306: f64 = (assign14520_e20302 + assign14520_e20305);
        let assign14520_e20309: f64 = (1.414213562373095 * locals.var_acd);
        let assign14520_e20311: f64 = (assign14520_e20309 * locals.var_acd);
        let assign14520_e20312: f64 = (assign14520_e20306 + assign14520_e20311);
        (assign14520_e20312, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign14520_e20314;
        locals.var_acn_dn0 = assign14520_e20314_d_n0;
        locals.var_acn_dn2 = assign14520_e20314_d_n2;
        locals.var_acn_dn6 = assign14520_e20314_d_n6;
        locals.var_acn_dn7 = assign14520_e20314_d_n7;
        locals.var_acn_dn10 = assign14520_e20314_d_n10;
        locals.var_acn_dn11 = assign14520_e20314_d_n11;
        locals.var_acn_dn12 = assign14520_e20314_d_n12;
        locals.var_acn_dn17 = assign14520_e20314_d_n17;
        locals.var_acn_rv = 0.0;

        let (assign14530_e20323, assign14530_e20323_d_n0, assign14530_e20323_d_n2, assign14530_e20323_d_n6, assign14530_e20323_d_n7, assign14530_e20323_d_n10, assign14530_e20323_d_n11, assign14530_e20323_d_n12, assign14530_e20323_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14530_e20321: f64 = (1.0 / locals.var_acd);
        (assign14530_e20321, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14530_e20323;
        locals.var_t1_dn0 = assign14530_e20323_d_n0;
        locals.var_t1_dn2 = assign14530_e20323_d_n2;
        locals.var_t1_dn6 = assign14530_e20323_d_n6;
        locals.var_t1_dn7 = assign14530_e20323_d_n7;
        locals.var_t1_dn10 = assign14530_e20323_d_n10;
        locals.var_t1_dn11 = assign14530_e20323_d_n11;
        locals.var_t1_dn12 = assign14530_e20323_d_n12;
        locals.var_t1_dn17 = assign14530_e20323_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14540_e20332, assign14540_e20332_d_n0, assign14540_e20332_d_n2, assign14540_e20332_d_n6, assign14540_e20332_d_n7, assign14540_e20332_d_n10, assign14540_e20332_d_n11, assign14540_e20332_d_n12, assign14540_e20332_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14540_e20330: f64 = (locals.var_acn * locals.var_t1);
        (assign14540_e20330, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14540_e20332;
        locals.var_chi_dn0 = assign14540_e20332_d_n0;
        locals.var_chi_dn2 = assign14540_e20332_d_n2;
        locals.var_chi_dn6 = assign14540_e20332_d_n6;
        locals.var_chi_dn7 = assign14540_e20332_d_n7;
        locals.var_chi_dn10 = assign14540_e20332_d_n10;
        locals.var_chi_dn11 = assign14540_e20332_d_n11;
        locals.var_chi_dn12 = assign14540_e20332_d_n12;
        locals.var_chi_dn17 = assign14540_e20332_d_n17;
        locals.var_chi_rv = 0.0;

        let (assign14550_e20343, assign14550_e20343_d_n0, assign14550_e20343_d_n2, assign14550_e20343_d_n6, assign14550_e20343_d_n7, assign14550_e20343_d_n10, assign14550_e20343_d_n11, assign14550_e20343_d_n12, assign14550_e20343_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14550_e20339: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign14550_e20341: f64 = (assign14550_e20339 + locals.var_vbcs_cl);
        (assign14550_e20341, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign14550_e20343;
        locals.var_psa_dn0 = assign14550_e20343_d_n0;
        locals.var_psa_dn2 = assign14550_e20343_d_n2;
        locals.var_psa_dn6 = assign14550_e20343_d_n6;
        locals.var_psa_dn7 = assign14550_e20343_d_n7;
        locals.var_psa_dn10 = assign14550_e20343_d_n10;
        locals.var_psa_dn11 = assign14550_e20343_d_n11;
        locals.var_psa_dn12 = assign14550_e20343_d_n12;
        locals.var_psa_dn17 = assign14550_e20343_d_n17;
        locals.var_psa_rv = 0.0;

        let (assign14560_e20352, assign14560_e20352_d_n0, assign14560_e20352_d_n2, assign14560_e20352_d_n6, assign14560_e20352_d_n7, assign14560_e20352_d_n10, assign14560_e20352_d_n11, assign14560_e20352_d_n12, assign14560_e20352_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14560_e20350: f64 = (locals.var_psa - locals.var_vbcs_cl);
        (assign14560_e20350, (locals.var_psa_dn0 - locals.var_vbcs_cl_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14560_e20352;
        locals.var_t1_dn0 = assign14560_e20352_d_n0;
        locals.var_t1_dn2 = assign14560_e20352_d_n2;
        locals.var_t1_dn6 = assign14560_e20352_d_n6;
        locals.var_t1_dn7 = assign14560_e20352_d_n7;
        locals.var_t1_dn10 = assign14560_e20352_d_n10;
        locals.var_t1_dn11 = assign14560_e20352_d_n11;
        locals.var_t1_dn12 = assign14560_e20352_d_n12;
        locals.var_t1_dn17 = assign14560_e20352_d_n17;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        locals: &mut StampLocals,
    ) {
        let (assign14570_e20361, assign14570_e20361_d_n0, assign14570_e20361_d_n2, assign14570_e20361_d_n6, assign14570_e20361_d_n7, assign14570_e20361_d_n10, assign14570_e20361_d_n11, assign14570_e20361_d_n12, assign14570_e20361_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14570_e20359: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign14570_e20359, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14570_e20361;
        locals.var_t2_dn0 = assign14570_e20361_d_n0;
        locals.var_t2_dn2 = assign14570_e20361_d_n2;
        locals.var_t2_dn6 = assign14570_e20361_d_n6;
        locals.var_t2_dn7 = assign14570_e20361_d_n7;
        locals.var_t2_dn10 = assign14570_e20361_d_n10;
        locals.var_t2_dn11 = assign14570_e20361_d_n11;
        locals.var_t2_dn12 = assign14570_e20361_d_n12;
        locals.var_t2_dn17 = assign14570_e20361_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14580_e20373, assign14580_e20373_d_n0, assign14580_e20373_d_n2, assign14580_e20373_d_n6, assign14580_e20373_d_n7, assign14580_e20373_d_n10, assign14580_e20373_d_n11, assign14580_e20373_d_n12, assign14580_e20373_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14580_e20369: f64 = (locals.var_t2 * locals.var_t2);
        let assign14580_e20370: f64 = (1.0 + assign14580_e20369);
        let assign14580_e20371: f64 = (assign14580_e20370).sqrt();
        (assign14580_e20371, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign14580_e20371)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14580_e20373;
        locals.var_t3_dn0 = assign14580_e20373_d_n0;
        locals.var_t3_dn2 = assign14580_e20373_d_n2;
        locals.var_t3_dn6 = assign14580_e20373_d_n6;
        locals.var_t3_dn7 = assign14580_e20373_d_n7;
        locals.var_t3_dn10 = assign14580_e20373_d_n10;
        locals.var_t3_dn11 = assign14580_e20373_d_n11;
        locals.var_t3_dn12 = assign14580_e20373_d_n12;
        locals.var_t3_dn17 = assign14580_e20373_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign14590_e20384, assign14590_e20384_d_n0, assign14590_e20384_d_n2, assign14590_e20384_d_n6, assign14590_e20384_d_n7, assign14590_e20384_d_n10, assign14590_e20384_d_n11, assign14590_e20384_d_n12, assign14590_e20384_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14590_e20380: f64 = (locals.var_t1 / locals.var_t3);
        let assign14590_e20382: f64 = (assign14590_e20380 + locals.var_vbcs_cl);
        (assign14590_e20382, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14590_e20384;
        locals.var_ps0_dn0 = assign14590_e20384_d_n0;
        locals.var_ps0_dn2 = assign14590_e20384_d_n2;
        locals.var_ps0_dn6 = assign14590_e20384_d_n6;
        locals.var_ps0_dn7 = assign14590_e20384_d_n7;
        locals.var_ps0_dn10 = assign14590_e20384_d_n10;
        locals.var_ps0_dn11 = assign14590_e20384_d_n11;
        locals.var_ps0_dn12 = assign14590_e20384_d_n12;
        locals.var_ps0_dn17 = assign14590_e20384_d_n17;
        locals.var_ps0_rv = 0.0;

        let assign14600_e20387: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign14600_e20387;
        locals.var_guard446_rv = 0.0;

        let (assign14610_e20397, assign14610_e20397_d_n0, assign14610_e20397_d_n2, assign14610_e20397_d_n6, assign14610_e20397_d_n7, assign14610_e20397_d_n10, assign14610_e20397_d_n11, assign14610_e20397_d_n12, assign14610_e20397_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14610_e20397;
        locals.var_phi_s0_soi_dn0 = assign14610_e20397_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14610_e20397_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14610_e20397_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14610_e20397_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14610_e20397_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14610_e20397_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14610_e20397_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14610_e20397_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign14620_e20407, assign14620_e20407_d_n0, assign14620_e20407_d_n2, assign14620_e20407_d_n6, assign14620_e20407_d_n7, assign14620_e20407_d_n10, assign14620_e20407_d_n11, assign14620_e20407_d_n12, assign14620_e20407_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14620_e20407;
        locals.var_ps0_ini_dn0 = assign14620_e20407_d_n0;
        locals.var_ps0_ini_dn2 = assign14620_e20407_d_n2;
        locals.var_ps0_ini_dn6 = assign14620_e20407_d_n6;
        locals.var_ps0_ini_dn7 = assign14620_e20407_d_n7;
        locals.var_ps0_ini_dn10 = assign14620_e20407_d_n10;
        locals.var_ps0_ini_dn11 = assign14620_e20407_d_n11;
        locals.var_ps0_ini_dn12 = assign14620_e20407_d_n12;
        locals.var_ps0_ini_dn17 = assign14620_e20407_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14630_e20432, assign14630_e20432_d_n0, assign14630_e20432_d_n2, assign14630_e20432_d_n6, assign14630_e20432_d_n7, assign14630_e20432_d_n10, assign14630_e20432_d_n11, assign14630_e20432_d_n12, assign14630_e20432_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14630_e20421: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14630_e20422: f64 = (locals.var_beta * assign14630_e20421);
        let assign14630_e20424: f64 = (assign14630_e20422 - 1.0);
        let assign14630_e20425: f64 = (4.0 * assign14630_e20424);
        let assign14630_e20428: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign14630_e20429: f64 = (assign14630_e20425 / assign14630_e20428);
        let assign14630_e20430: f64 = (1.0 + assign14630_e20429);
        (assign14630_e20430, ((((4.0 * (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * ((locals.var_beta_dn10 * assign14630_e20421) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10)))) * assign14630_e20428) - (assign14630_e20425 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14630_e20432;
        locals.var_tx_dn0 = assign14630_e20432_d_n0;
        locals.var_tx_dn2 = assign14630_e20432_d_n2;
        locals.var_tx_dn6 = assign14630_e20432_d_n6;
        locals.var_tx_dn7 = assign14630_e20432_d_n7;
        locals.var_tx_dn10 = assign14630_e20432_d_n10;
        locals.var_tx_dn11 = assign14630_e20432_d_n11;
        locals.var_tx_dn12 = assign14630_e20432_d_n12;
        locals.var_tx_dn17 = assign14630_e20432_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14640_e20452, assign14640_e20452_d_n0, assign14640_e20452_d_n2, assign14640_e20452_d_n6, assign14640_e20452_d_n7, assign14640_e20452_d_n10, assign14640_e20452_d_n11, assign14640_e20452_d_n12, assign14640_e20452_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14640_e20444: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14640_e20450, assign14640_e20450_d_n0, assign14640_e20450_d_n2, assign14640_e20450_d_n6, assign14640_e20450_d_n7, assign14640_e20450_d_n10, assign14640_e20450_d_n11, assign14640_e20450_d_n12, assign14640_e20450_d_n17,) = {
            if (locals.var_tx >= assign14640_e20444) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign14640_e20449: f64 = (10.0 * 2.220446049250313e-16);
                (assign14640_e20449, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14640_e20450, assign14640_e20450_d_n0, assign14640_e20450_d_n2, assign14640_e20450_d_n6, assign14640_e20450_d_n7, assign14640_e20450_d_n10, assign14640_e20450_d_n11, assign14640_e20450_d_n12, assign14640_e20450_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14640_e20452;
        locals.var_tx_dn0 = assign14640_e20452_d_n0;
        locals.var_tx_dn2 = assign14640_e20452_d_n2;
        locals.var_tx_dn6 = assign14640_e20452_d_n6;
        locals.var_tx_dn7 = assign14640_e20452_d_n7;
        locals.var_tx_dn10 = assign14640_e20452_d_n10;
        locals.var_tx_dn11 = assign14640_e20452_d_n11;
        locals.var_tx_dn12 = assign14640_e20452_d_n12;
        locals.var_tx_dn17 = assign14640_e20452_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14650_e20474, assign14650_e20474_d_n0, assign14650_e20474_d_n2, assign14650_e20474_d_n6, assign14650_e20474_d_n7, assign14650_e20474_d_n10, assign14650_e20474_d_n11, assign14650_e20474_d_n12, assign14650_e20474_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14650_e20464: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign14650_e20466: f64 = (assign14650_e20464 * 0.5);
        let assign14650_e20469: f64 = (locals.var_tx).sqrt();
        let assign14650_e20470: f64 = (1.0 - assign14650_e20469);
        let assign14650_e20471: f64 = (assign14650_e20466 * assign14650_e20470);
        let assign14650_e20472: f64 = (locals.var_vgp + assign14650_e20471);
        (assign14650_e20472, (locals.var_vgp_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn0 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn2 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn6 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn7 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn10 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn11 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn12 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn17 / (2.0 * assign14650_e20469)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14650_e20474;
        locals.var_ps0_inia_dn0 = assign14650_e20474_d_n0;
        locals.var_ps0_inia_dn2 = assign14650_e20474_d_n2;
        locals.var_ps0_inia_dn6 = assign14650_e20474_d_n6;
        locals.var_ps0_inia_dn7 = assign14650_e20474_d_n7;
        locals.var_ps0_inia_dn10 = assign14650_e20474_d_n10;
        locals.var_ps0_inia_dn11 = assign14650_e20474_d_n11;
        locals.var_ps0_inia_dn12 = assign14650_e20474_d_n12;
        locals.var_ps0_inia_dn17 = assign14650_e20474_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign14660_e20489, assign14660_e20489_d_n0, assign14660_e20489_d_n2, assign14660_e20489_d_n6, assign14660_e20489_d_n7, assign14660_e20489_d_n10, assign14660_e20489_d_n11, assign14660_e20489_d_n12, assign14660_e20489_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14660_e20486: f64 = (locals.var_ps0_inia - locals.var_vbcs_cl);
        let assign14660_e20487: f64 = (locals.var_beta * assign14660_e20486);
        (assign14660_e20487, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14660_e20486) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14660_e20489;
        locals.var_chi_dn0 = assign14660_e20489_d_n0;
        locals.var_chi_dn2 = assign14660_e20489_d_n2;
        locals.var_chi_dn6 = assign14660_e20489_d_n6;
        locals.var_chi_dn7 = assign14660_e20489_d_n7;
        locals.var_chi_dn10 = assign14660_e20489_d_n10;
        locals.var_chi_dn11 = assign14660_e20489_d_n11;
        locals.var_chi_dn12 = assign14660_e20489_d_n12;
        locals.var_chi_dn17 = assign14660_e20489_d_n17;
        locals.var_chi_rv = 0.0;

        let assign14670_e20492: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard447 = assign14670_e20492;
        locals.var_guard447_rv = 0.0;

        let (assign14680_e20509, assign14680_e20509_d_n0, assign14680_e20509_d_n2, assign14680_e20509_d_n6, assign14680_e20509_d_n7, assign14680_e20509_d_n10, assign14680_e20509_d_n11, assign14680_e20509_d_n12, assign14680_e20509_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14680_e20506: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14680_e20507: f64 = (locals.var_beta * assign14680_e20506);
        (assign14680_e20507, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14680_e20506) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14680_e20509;
        locals.var_ty_dn0 = assign14680_e20509_d_n0;
        locals.var_ty_dn2 = assign14680_e20509_d_n2;
        locals.var_ty_dn6 = assign14680_e20509_d_n6;
        locals.var_ty_dn7 = assign14680_e20509_d_n7;
        locals.var_ty_dn10 = assign14680_e20509_d_n10;
        locals.var_ty_dn11 = assign14680_e20509_d_n11;
        locals.var_ty_dn12 = assign14680_e20509_d_n12;
        locals.var_ty_dn17 = assign14680_e20509_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign14690_e20530, assign14690_e20530_d_n0, assign14690_e20530_d_n2, assign14690_e20530_d_n6, assign14690_e20530_d_n7, assign14690_e20530_d_n10, assign14690_e20530_d_n11, assign14690_e20530_d_n12, assign14690_e20530_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14690_e20523: f64 = (1.414213562373095 / 108.0);
        let assign14690_e20525: f64 = (assign14690_e20523 * locals.var_beta);
        let assign14690_e20527: f64 = (assign14690_e20525 * locals.var_fac1);
        let assign14690_e20528: f64 = (1.0 / assign14690_e20527);
        (assign14690_e20528, (-((assign14690_e20525 * locals.var_fac1_dn0) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn2) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn6) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn7) / (assign14690_e20527 * assign14690_e20527))), (-((((assign14690_e20523 * locals.var_beta_dn10) * locals.var_fac1) + (assign14690_e20525 * locals.var_fac1_dn10)) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn11) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn12) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn17) / (assign14690_e20527 * assign14690_e20527))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14690_e20530;
        locals.var_t1_dn0 = assign14690_e20530_d_n0;
        locals.var_t1_dn2 = assign14690_e20530_d_n2;
        locals.var_t1_dn6 = assign14690_e20530_d_n6;
        locals.var_t1_dn7 = assign14690_e20530_d_n7;
        locals.var_t1_dn10 = assign14690_e20530_d_n10;
        locals.var_t1_dn11 = assign14690_e20530_d_n11;
        locals.var_t1_dn12 = assign14690_e20530_d_n12;
        locals.var_t1_dn17 = assign14690_e20530_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14700_e20547, assign14700_e20547_d_n0, assign14700_e20547_d_n2, assign14700_e20547_d_n6, assign14700_e20547_d_n7, assign14700_e20547_d_n10, assign14700_e20547_d_n11, assign14700_e20547_d_n12, assign14700_e20547_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14700_e20544: f64 = (3.0 * locals.var_t1);
        let assign14700_e20545: f64 = (81.0 + assign14700_e20544);
        (assign14700_e20545, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14700_e20547;
        locals.var_t2_dn0 = assign14700_e20547_d_n0;
        locals.var_t2_dn2 = assign14700_e20547_d_n2;
        locals.var_t2_dn6 = assign14700_e20547_d_n6;
        locals.var_t2_dn7 = assign14700_e20547_d_n7;
        locals.var_t2_dn10 = assign14700_e20547_d_n10;
        locals.var_t2_dn11 = assign14700_e20547_d_n11;
        locals.var_t2_dn12 = assign14700_e20547_d_n12;
        locals.var_t2_dn17 = assign14700_e20547_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14710_e20571, assign14710_e20571_d_n0, assign14710_e20571_d_n2, assign14710_e20571_d_n6, assign14710_e20571_d_n7, assign14710_e20571_d_n10, assign14710_e20571_d_n11, assign14710_e20571_d_n12, assign14710_e20571_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14710_e20559: f64 = (-2916.0);
        let assign14710_e20562: f64 = (81.0 * locals.var_t1);
        let assign14710_e20563: f64 = (assign14710_e20559 - assign14710_e20562);
        let assign14710_e20566: f64 = (27.0 * locals.var_t1);
        let assign14710_e20568: f64 = (assign14710_e20566 * locals.var_ty);
        let assign14710_e20569: f64 = (assign14710_e20563 + assign14710_e20568);
        (assign14710_e20569, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14710_e20571;
        locals.var_t3_dn0 = assign14710_e20571_d_n0;
        locals.var_t3_dn2 = assign14710_e20571_d_n2;
        locals.var_t3_dn6 = assign14710_e20571_d_n6;
        locals.var_t3_dn7 = assign14710_e20571_d_n7;
        locals.var_t3_dn10 = assign14710_e20571_d_n10;
        locals.var_t3_dn11 = assign14710_e20571_d_n11;
        locals.var_t3_dn12 = assign14710_e20571_d_n12;
        locals.var_t3_dn17 = assign14710_e20571_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign14720_e20596, assign14720_e20596_d_n0, assign14720_e20596_d_n2, assign14720_e20596_d_n6, assign14720_e20596_d_n7, assign14720_e20596_d_n10, assign14720_e20596_d_n11, assign14720_e20596_d_n12, assign14720_e20596_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14720_e20586: f64 = (54.0 + locals.var_t1);
        let assign14720_e20587: f64 = (81.0 * assign14720_e20586);
        let assign14720_e20588: f64 = (1458.0 - assign14720_e20587);
        let assign14720_e20591: f64 = (27.0 * locals.var_t1);
        let assign14720_e20593: f64 = (assign14720_e20591 * locals.var_ty);
        let assign14720_e20594: f64 = (assign14720_e20588 + assign14720_e20593);
        (assign14720_e20594, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14720_e20596;
        locals.var_t4_dn0 = assign14720_e20596_d_n0;
        locals.var_t4_dn2 = assign14720_e20596_d_n2;
        locals.var_t4_dn6 = assign14720_e20596_d_n6;
        locals.var_t4_dn7 = assign14720_e20596_d_n7;
        locals.var_t4_dn10 = assign14720_e20596_d_n10;
        locals.var_t4_dn11 = assign14720_e20596_d_n11;
        locals.var_t4_dn12 = assign14720_e20596_d_n12;
        locals.var_t4_dn17 = assign14720_e20596_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14730_e20611, assign14730_e20611_d_n0, assign14730_e20611_d_n2, assign14730_e20611_d_n6, assign14730_e20611_d_n7, assign14730_e20611_d_n10, assign14730_e20611_d_n11, assign14730_e20611_d_n12, assign14730_e20611_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14730_e20609: f64 = (locals.var_t4 * locals.var_t4);
        (assign14730_e20609, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14730_e20611;
        locals.var_t4_dn0 = assign14730_e20611_d_n0;
        locals.var_t4_dn2 = assign14730_e20611_d_n2;
        locals.var_t4_dn6 = assign14730_e20611_d_n6;
        locals.var_t4_dn7 = assign14730_e20611_d_n7;
        locals.var_t4_dn10 = assign14730_e20611_d_n10;
        locals.var_t4_dn11 = assign14730_e20611_d_n11;
        locals.var_t4_dn12 = assign14730_e20611_d_n12;
        locals.var_t4_dn17 = assign14730_e20611_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign14740_e20637, assign14740_e20637_d_n0, assign14740_e20637_d_n2, assign14740_e20637_d_n6, assign14740_e20637_d_n7, assign14740_e20637_d_n10, assign14740_e20637_d_n11, assign14740_e20637_d_n12, assign14740_e20637_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14740_e20625: f64 = (4.0 * locals.var_t2);
        let assign14740_e20627: f64 = (assign14740_e20625 * locals.var_t2);
        let assign14740_e20629: f64 = (assign14740_e20627 * locals.var_t2);
        let assign14740_e20631: f64 = (assign14740_e20629 + locals.var_t4);
        let assign14740_e20632: f64 = (assign14740_e20631).sqrt();
        let assign14740_e20633: f64 = (locals.var_t3 + assign14740_e20632);
        let assign14740_e20635: f64 = (assign14740_e20633).powf(0.3333333333333333);
        (assign14740_e20635, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn0)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn0)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn2)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn2)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn6)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn6)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn7)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn7)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn10)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn10)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn11)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn11)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn12)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn12)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn17)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn17)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14740_e20632))) / assign14740_e20633))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14740_e20637;
        locals.var_t5_dn0 = assign14740_e20637_d_n0;
        locals.var_t5_dn2 = assign14740_e20637_d_n2;
        locals.var_t5_dn6 = assign14740_e20637_d_n6;
        locals.var_t5_dn7 = assign14740_e20637_d_n7;
        locals.var_t5_dn10 = assign14740_e20637_d_n10;
        locals.var_t5_dn11 = assign14740_e20637_d_n11;
        locals.var_t5_dn12 = assign14740_e20637_d_n12;
        locals.var_t5_dn17 = assign14740_e20637_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign14750_e20666, assign14750_e20666_d_n0, assign14750_e20666_d_n2, assign14750_e20666_d_n6, assign14750_e20666_d_n7, assign14750_e20666_d_n10, assign14750_e20666_d_n11, assign14750_e20666_d_n12, assign14750_e20666_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14750_e20651: f64 = (1.259921049894873 * locals.var_t2);
        let assign14750_e20654: f64 = (3.0 * locals.var_t5);
        let assign14750_e20655: f64 = (assign14750_e20651 / assign14750_e20654);
        let assign14750_e20656: f64 = (3.0 - assign14750_e20655);
        let assign14750_e20660: f64 = (3.0 * 1.259921049894873);
        let assign14750_e20661: f64 = (1.0 / assign14750_e20660);
        let assign14750_e20663: f64 = (assign14750_e20661 * locals.var_t5);
        let assign14750_e20664: f64 = (assign14750_e20656 + assign14750_e20663);
        (assign14750_e20664, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn0))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn2))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn6))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn7))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn10))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn11))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn12))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn17))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14750_e20666;
        locals.var_tx_dn0 = assign14750_e20666_d_n0;
        locals.var_tx_dn2 = assign14750_e20666_d_n2;
        locals.var_tx_dn6 = assign14750_e20666_d_n6;
        locals.var_tx_dn7 = assign14750_e20666_d_n7;
        locals.var_tx_dn10 = assign14750_e20666_d_n10;
        locals.var_tx_dn11 = assign14750_e20666_d_n11;
        locals.var_tx_dn12 = assign14750_e20666_d_n12;
        locals.var_tx_dn17 = assign14750_e20666_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign14760_e20683, assign14760_e20683_d_n0, assign14760_e20683_d_n2, assign14760_e20683_d_n6, assign14760_e20683_d_n7, assign14760_e20683_d_n10, assign14760_e20683_d_n11, assign14760_e20683_d_n12, assign14760_e20683_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14760_e20679: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign14760_e20681: f64 = (assign14760_e20679 + locals.var_vbcs_cl);
        (assign14760_e20681, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14760_e20683;
        locals.var_ps0_inia_dn0 = assign14760_e20683_d_n0;
        locals.var_ps0_inia_dn2 = assign14760_e20683_d_n2;
        locals.var_ps0_inia_dn6 = assign14760_e20683_d_n6;
        locals.var_ps0_inia_dn7 = assign14760_e20683_d_n7;
        locals.var_ps0_inia_dn10 = assign14760_e20683_d_n10;
        locals.var_ps0_inia_dn11 = assign14760_e20683_d_n11;
        locals.var_ps0_inia_dn12 = assign14760_e20683_d_n12;
        locals.var_ps0_inia_dn17 = assign14760_e20683_d_n17;
        locals.var_ps0_inia_rv = 0.0;

        let (assign14770_e20696, assign14770_e20696_d_n0, assign14770_e20696_d_n2, assign14770_e20696_d_n6, assign14770_e20696_d_n7, assign14770_e20696_d_n10, assign14770_e20696_d_n11, assign14770_e20696_d_n12, assign14770_e20696_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14770_e20696;
        locals.var_ps0_ini_dn0 = assign14770_e20696_d_n0;
        locals.var_ps0_ini_dn2 = assign14770_e20696_d_n2;
        locals.var_ps0_ini_dn6 = assign14770_e20696_d_n6;
        locals.var_ps0_ini_dn7 = assign14770_e20696_d_n7;
        locals.var_ps0_ini_dn10 = assign14770_e20696_d_n10;
        locals.var_ps0_ini_dn11 = assign14770_e20696_d_n11;
        locals.var_ps0_ini_dn12 = assign14770_e20696_d_n12;
        locals.var_ps0_ini_dn17 = assign14770_e20696_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let assign14780_e20699: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard448 = assign14780_e20699;
        locals.var_guard448_rv = 0.0;

        let (assign14790_e20715, assign14790_e20715_d_n0, assign14790_e20715_d_n2, assign14790_e20715_d_n6, assign14790_e20715_d_n7, assign14790_e20715_d_n10, assign14790_e20715_d_n11, assign14790_e20715_d_n12, assign14790_e20715_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14790_e20715;
        locals.var_ps0_ini_dn0 = assign14790_e20715_d_n0;
        locals.var_ps0_ini_dn2 = assign14790_e20715_d_n2;
        locals.var_ps0_ini_dn6 = assign14790_e20715_d_n6;
        locals.var_ps0_ini_dn7 = assign14790_e20715_d_n7;
        locals.var_ps0_ini_dn10 = assign14790_e20715_d_n10;
        locals.var_ps0_ini_dn11 = assign14790_e20715_d_n11;
        locals.var_ps0_ini_dn12 = assign14790_e20715_d_n12;
        locals.var_ps0_ini_dn17 = assign14790_e20715_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14800_e20736, assign14800_e20736_d_n0, assign14800_e20736_d_n2, assign14800_e20736_d_n6, assign14800_e20736_d_n7, assign14800_e20736_d_n10, assign14800_e20736_d_n11, assign14800_e20736_d_n12, assign14800_e20736_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14800_e20732: f64 = (1.0 / locals.var_cnst1soi);
        let assign14800_e20734: f64 = (assign14800_e20732 / locals.var_cnstc_foxi);
        (assign14800_e20734, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14800_e20736;
        locals.var_t1_dn0 = assign14800_e20736_d_n0;
        locals.var_t1_dn2 = assign14800_e20736_d_n2;
        locals.var_t1_dn6 = assign14800_e20736_d_n6;
        locals.var_t1_dn7 = assign14800_e20736_d_n7;
        locals.var_t1_dn10 = assign14800_e20736_d_n10;
        locals.var_t1_dn11 = assign14800_e20736_d_n11;
        locals.var_t1_dn12 = assign14800_e20736_d_n12;
        locals.var_t1_dn17 = assign14800_e20736_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign14810_e20757, assign14810_e20757_d_n0, assign14810_e20757_d_n2, assign14810_e20757_d_n6, assign14810_e20757_d_n7, assign14810_e20757_d_n10, assign14810_e20757_d_n11, assign14810_e20757_d_n12, assign14810_e20757_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14810_e20753: f64 = (locals.var_t1 * locals.var_vgp);
        let assign14810_e20755: f64 = (assign14810_e20753 * locals.var_vgp);
        (assign14810_e20755, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn2)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn7)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn11)), ((((locals.var_t1_dn12 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn12)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn12)), ((((locals.var_t1_dn17 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn17)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14810_e20757;
        locals.var_t2_dn0 = assign14810_e20757_d_n0;
        locals.var_t2_dn2 = assign14810_e20757_d_n2;
        locals.var_t2_dn6 = assign14810_e20757_d_n6;
        locals.var_t2_dn7 = assign14810_e20757_d_n7;
        locals.var_t2_dn10 = assign14810_e20757_d_n10;
        locals.var_t2_dn11 = assign14810_e20757_d_n11;
        locals.var_t2_dn12 = assign14810_e20757_d_n12;
        locals.var_t2_dn17 = assign14810_e20757_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign14820_e20778, assign14820_e20778_d_n0, assign14820_e20778_d_n2, assign14820_e20778_d_n6, assign14820_e20778_d_n7, assign14820_e20778_d_n10, assign14820_e20778_d_n11, assign14820_e20778_d_n12, assign14820_e20778_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14820_e20775: f64 = (2.0 / locals.var_vgp);
        let assign14820_e20776: f64 = (locals.var_beta + assign14820_e20775);
        (assign14820_e20776, (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn12) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn17) / (locals.var_vgp * locals.var_vgp))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14820_e20778;
        locals.var_t3_dn0 = assign14820_e20778_d_n0;
        locals.var_t3_dn2 = assign14820_e20778_d_n2;
        locals.var_t3_dn6 = assign14820_e20778_d_n6;
        locals.var_t3_dn7 = assign14820_e20778_d_n7;
        locals.var_t3_dn10 = assign14820_e20778_d_n10;
        locals.var_t3_dn11 = assign14820_e20778_d_n11;
        locals.var_t3_dn12 = assign14820_e20778_d_n12;
        locals.var_t3_dn17 = assign14820_e20778_d_n17;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign14830_e20798, assign14830_e20798_d_n0, assign14830_e20798_d_n2, assign14830_e20798_d_n6, assign14830_e20798_d_n7, assign14830_e20798_d_n10, assign14830_e20798_d_n11, assign14830_e20798_d_n12, assign14830_e20798_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14830_e20794: f64 = (locals.var_t2).ln();
        let assign14830_e20796: f64 = (assign14830_e20794 / locals.var_t3);
        (assign14830_e20796, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign14830_e20798;
        locals.var_ps0_inib_dn0 = assign14830_e20798_d_n0;
        locals.var_ps0_inib_dn2 = assign14830_e20798_d_n2;
        locals.var_ps0_inib_dn6 = assign14830_e20798_d_n6;
        locals.var_ps0_inib_dn7 = assign14830_e20798_d_n7;
        locals.var_ps0_inib_dn10 = assign14830_e20798_d_n10;
        locals.var_ps0_inib_dn11 = assign14830_e20798_d_n11;
        locals.var_ps0_inib_dn12 = assign14830_e20798_d_n12;
        locals.var_ps0_inib_dn17 = assign14830_e20798_d_n17;
        locals.var_ps0_inib_rv = 0.0;

        let (assign14840_e20819, assign14840_e20819_d_n0, assign14840_e20819_d_n2, assign14840_e20819_d_n6, assign14840_e20819_d_n7, assign14840_e20819_d_n10, assign14840_e20819_d_n11, assign14840_e20819_d_n12, assign14840_e20819_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14840_e20815: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign14840_e20817: f64 = (assign14840_e20815 - 0.0008);
        (assign14840_e20817, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign14840_e20819;
        locals.var_tmf1_dn0 = assign14840_e20819_d_n0;
        locals.var_tmf1_dn2 = assign14840_e20819_d_n2;
        locals.var_tmf1_dn6 = assign14840_e20819_d_n6;
        locals.var_tmf1_dn7 = assign14840_e20819_d_n7;
        locals.var_tmf1_dn10 = assign14840_e20819_d_n10;
        locals.var_tmf1_dn11 = assign14840_e20819_d_n11;
        locals.var_tmf1_dn12 = assign14840_e20819_d_n12;
        locals.var_tmf1_dn17 = assign14840_e20819_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign14850_e20840, assign14850_e20840_d_n0, assign14850_e20840_d_n2, assign14850_e20840_d_n6, assign14850_e20840_d_n7, assign14850_e20840_d_n10, assign14850_e20840_d_n11, assign14850_e20840_d_n12, assign14850_e20840_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14850_e20836: f64 = (4.0 * locals.var_ps0_inib);
        let assign14850_e20838: f64 = (assign14850_e20836 * 0.0008);
        (assign14850_e20838, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14850_e20840;
        locals.var_tmf2_dn0 = assign14850_e20840_d_n0;
        locals.var_tmf2_dn2 = assign14850_e20840_d_n2;
        locals.var_tmf2_dn6 = assign14850_e20840_d_n6;
        locals.var_tmf2_dn7 = assign14850_e20840_d_n7;
        locals.var_tmf2_dn10 = assign14850_e20840_d_n10;
        locals.var_tmf2_dn11 = assign14850_e20840_d_n11;
        locals.var_tmf2_dn12 = assign14850_e20840_d_n12;
        locals.var_tmf2_dn17 = assign14850_e20840_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14860_e20863, assign14860_e20863_d_n0, assign14860_e20863_d_n2, assign14860_e20863_d_n6, assign14860_e20863_d_n7, assign14860_e20863_d_n10, assign14860_e20863_d_n11, assign14860_e20863_d_n12, assign14860_e20863_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let (assign14860_e20861, assign14860_e20861_d_n0, assign14860_e20861_d_n2, assign14860_e20861_d_n6, assign14860_e20861_d_n7, assign14860_e20861_d_n10, assign14860_e20861_d_n11, assign14860_e20861_d_n12, assign14860_e20861_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign14860_e20860: f64 = (-locals.var_tmf2);
                (assign14860_e20860, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign14860_e20861, assign14860_e20861_d_n0, assign14860_e20861_d_n2, assign14860_e20861_d_n6, assign14860_e20861_d_n7, assign14860_e20861_d_n10, assign14860_e20861_d_n11, assign14860_e20861_d_n12, assign14860_e20861_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14860_e20863;
        locals.var_tmf2_dn0 = assign14860_e20863_d_n0;
        locals.var_tmf2_dn2 = assign14860_e20863_d_n2;
        locals.var_tmf2_dn6 = assign14860_e20863_d_n6;
        locals.var_tmf2_dn7 = assign14860_e20863_d_n7;
        locals.var_tmf2_dn10 = assign14860_e20863_d_n10;
        locals.var_tmf2_dn11 = assign14860_e20863_d_n11;
        locals.var_tmf2_dn12 = assign14860_e20863_d_n12;
        locals.var_tmf2_dn17 = assign14860_e20863_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14870_e20885, assign14870_e20885_d_n0, assign14870_e20885_d_n2, assign14870_e20885_d_n6, assign14870_e20885_d_n7, assign14870_e20885_d_n10, assign14870_e20885_d_n11, assign14870_e20885_d_n12, assign14870_e20885_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14870_e20880: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14870_e20882: f64 = (assign14870_e20880 + locals.var_tmf2);
        let assign14870_e20883: f64 = (assign14870_e20882).sqrt();
        (assign14870_e20883, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign14870_e20883)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14870_e20885;
        locals.var_tmf2_dn0 = assign14870_e20885_d_n0;
        locals.var_tmf2_dn2 = assign14870_e20885_d_n2;
        locals.var_tmf2_dn6 = assign14870_e20885_d_n6;
        locals.var_tmf2_dn7 = assign14870_e20885_d_n7;
        locals.var_tmf2_dn10 = assign14870_e20885_d_n10;
        locals.var_tmf2_dn11 = assign14870_e20885_d_n11;
        locals.var_tmf2_dn12 = assign14870_e20885_d_n12;
        locals.var_tmf2_dn17 = assign14870_e20885_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign14880_e20908, assign14880_e20908_d_n0, assign14880_e20908_d_n2, assign14880_e20908_d_n6, assign14880_e20908_d_n7, assign14880_e20908_d_n10, assign14880_e20908_d_n11, assign14880_e20908_d_n12, assign14880_e20908_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14880_e20904: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14880_e20905: f64 = (0.5 * assign14880_e20904);
        let assign14880_e20906: f64 = (locals.var_ps0_inib - assign14880_e20905);
        (assign14880_e20906, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14880_e20908;
        locals.var_ps0_ini_dn0 = assign14880_e20908_d_n0;
        locals.var_ps0_ini_dn2 = assign14880_e20908_d_n2;
        locals.var_ps0_ini_dn6 = assign14880_e20908_d_n6;
        locals.var_ps0_ini_dn7 = assign14880_e20908_d_n7;
        locals.var_ps0_ini_dn10 = assign14880_e20908_d_n10;
        locals.var_ps0_ini_dn11 = assign14880_e20908_d_n11;
        locals.var_ps0_ini_dn12 = assign14880_e20908_d_n12;
        locals.var_ps0_ini_dn17 = assign14880_e20908_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14890_e20923, assign14890_e20923_d_n0, assign14890_e20923_d_n2, assign14890_e20923_d_n6, assign14890_e20923_d_n7, assign14890_e20923_d_n10, assign14890_e20923_d_n11, assign14890_e20923_d_n12, assign14890_e20923_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14890_e20920: f64 = (5e-12 / 2.0);
        let assign14890_e20921: f64 = (locals.var_vbcs_cl + assign14890_e20920);
        (assign14890_e20921, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14890_e20923;
        locals.var_tx_dn0 = assign14890_e20923_d_n0;
        locals.var_tx_dn2 = assign14890_e20923_d_n2;
        locals.var_tx_dn6 = assign14890_e20923_d_n6;
        locals.var_tx_dn7 = assign14890_e20923_d_n7;
        locals.var_tx_dn10 = assign14890_e20923_d_n10;
        locals.var_tx_dn11 = assign14890_e20923_d_n11;
        locals.var_tx_dn12 = assign14890_e20923_d_n12;
        locals.var_tx_dn17 = assign14890_e20923_d_n17;
        locals.var_tx_rv = 0.0;

        let assign14900_e20926: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard449 = assign14900_e20926;
        locals.var_guard449_rv = 0.0;

        let (assign14910_e20939, assign14910_e20939_d_n0, assign14910_e20939_d_n2, assign14910_e20939_d_n6, assign14910_e20939_d_n7, assign14910_e20939_d_n10, assign14910_e20939_d_n11, assign14910_e20939_d_n12, assign14910_e20939_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard449 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14910_e20939;
        locals.var_ps0_ini_dn0 = assign14910_e20939_d_n0;
        locals.var_ps0_ini_dn2 = assign14910_e20939_d_n2;
        locals.var_ps0_ini_dn6 = assign14910_e20939_d_n6;
        locals.var_ps0_ini_dn7 = assign14910_e20939_d_n7;
        locals.var_ps0_ini_dn10 = assign14910_e20939_d_n10;
        locals.var_ps0_ini_dn11 = assign14910_e20939_d_n11;
        locals.var_ps0_ini_dn12 = assign14910_e20939_d_n12;
        locals.var_ps0_ini_dn17 = assign14910_e20939_d_n17;
        locals.var_ps0_ini_rv = 0.0;

        let (assign14920_e20947, assign14920_e20947_d_n0, assign14920_e20947_d_n2, assign14920_e20947_d_n6, assign14920_e20947_d_n7, assign14920_e20947_d_n10, assign14920_e20947_d_n11, assign14920_e20947_d_n12, assign14920_e20947_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14920_e20947;
        locals.var_ps0_dn0 = assign14920_e20947_d_n0;
        locals.var_ps0_dn2 = assign14920_e20947_d_n2;
        locals.var_ps0_dn6 = assign14920_e20947_d_n6;
        locals.var_ps0_dn7 = assign14920_e20947_d_n7;
        locals.var_ps0_dn10 = assign14920_e20947_d_n10;
        locals.var_ps0_dn11 = assign14920_e20947_d_n11;
        locals.var_ps0_dn12 = assign14920_e20947_d_n12;
        locals.var_ps0_dn17 = assign14920_e20947_d_n17;
        locals.var_ps0_rv = 0.0;

        let (assign14930_e20955, assign14930_e20955_d_n0, assign14930_e20955_d_n2, assign14930_e20955_d_n6, assign14930_e20955_d_n7, assign14930_e20955_d_n10, assign14930_e20955_d_n11, assign14930_e20955_d_n12, assign14930_e20955_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign14930_e20955;
        locals.var_psl_lim_dn0 = assign14930_e20955_d_n0;
        locals.var_psl_lim_dn2 = assign14930_e20955_d_n2;
        locals.var_psl_lim_dn6 = assign14930_e20955_d_n6;
        locals.var_psl_lim_dn7 = assign14930_e20955_d_n7;
        locals.var_psl_lim_dn10 = assign14930_e20955_d_n10;
        locals.var_psl_lim_dn11 = assign14930_e20955_d_n11;
        locals.var_psl_lim_dn12 = assign14930_e20955_d_n12;
        locals.var_psl_lim_dn17 = assign14930_e20955_d_n17;
        locals.var_psl_lim_rv = 0.0;

        let assign14940_e20962: f64 = if ((p.p25 == 1.0) && (p.p26 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard450 = assign14940_e20962;
        locals.var_guard450_rv = 0.0;

        let (assign14950_e20973, assign14950_e20973_d_n0, assign14950_e20973_d_n2, assign14950_e20973_d_n6, assign14950_e20973_d_n7, assign14950_e20973_d_n10, assign14950_e20973_d_n11, assign14950_e20973_d_n12, assign14950_e20973_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard450 != 0.0)) {
        let assign14950_e20969: f64 = (1e-9 / 0.0001);
        let assign14950_e20971: f64 = (assign14950_e20969 * (nv17 - 0.0));
        (assign14950_e20971, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign14950_e20969,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign14950_e20973;
        locals.var_qhs_dn0 = assign14950_e20973_d_n0;
        locals.var_qhs_dn2 = assign14950_e20973_d_n2;
        locals.var_qhs_dn6 = assign14950_e20973_d_n6;
        locals.var_qhs_dn7 = assign14950_e20973_d_n7;
        locals.var_qhs_dn10 = assign14950_e20973_d_n10;
        locals.var_qhs_dn11 = assign14950_e20973_d_n11;
        locals.var_qhs_dn12 = assign14950_e20973_d_n12;
        locals.var_qhs_dn17 = assign14950_e20973_d_n17;
        locals.var_qhs_rv = 0.0;

        let (assign14960_e20981, assign14960_e20981_d_n0, assign14960_e20981_d_n2, assign14960_e20981_d_n6, assign14960_e20981_d_n7, assign14960_e20981_d_n10, assign14960_e20981_d_n11, assign14960_e20981_d_n12, assign14960_e20981_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard450 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign14960_e20981;
        locals.var_qhs_dn0 = assign14960_e20981_d_n0;
        locals.var_qhs_dn2 = assign14960_e20981_d_n2;
        locals.var_qhs_dn6 = assign14960_e20981_d_n6;
        locals.var_qhs_dn7 = assign14960_e20981_d_n7;
        locals.var_qhs_dn10 = assign14960_e20981_d_n10;
        locals.var_qhs_dn11 = assign14960_e20981_d_n11;
        locals.var_qhs_dn12 = assign14960_e20981_d_n12;
        locals.var_qhs_dn17 = assign14960_e20981_d_n17;
        locals.var_qhs_rv = 0.0;

        let (assign14980_e20994, assign14980_e20994_d_n0, assign14980_e20994_d_n2, assign14980_e20994_d_n6, assign14980_e20994_d_n7, assign14980_e20994_d_n10, assign14980_e20994_d_n11, assign14980_e20994_d_n12, assign14980_e20994_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign14980_e20991: f64 = (locals.var_beta * locals.var_vbcs_cl);
        let assign14980_e20992: f64 = (assign14980_e20991).exp();
        (assign14980_e20992, (assign14980_e20992 * (locals.var_beta * locals.var_vbcs_cl_dn0)), (assign14980_e20992 * (locals.var_beta * locals.var_vbcs_cl_dn2)), (assign14980_e20992 * (locals.var_beta * locals.var_vbcs_cl_dn6)), (assign14980_e20992 * (locals.var_beta * locals.var_vbcs_cl_dn7)), (assign14980_e20992 * ((locals.var_beta_dn10 * locals.var_vbcs_cl) + (locals.var_beta * locals.var_vbcs_cl_dn10))), (assign14980_e20992 * (locals.var_beta * locals.var_vbcs_cl_dn11)), (assign14980_e20992 * (locals.var_beta * locals.var_vbcs_cl_dn12)), (assign14980_e20992 * (locals.var_beta * locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12, locals.var_exp_bvbs_dn17,)
    }
};
        locals.var_exp_bvbs = assign14980_e20994;
        locals.var_exp_bvbs_dn0 = assign14980_e20994_d_n0;
        locals.var_exp_bvbs_dn2 = assign14980_e20994_d_n2;
        locals.var_exp_bvbs_dn6 = assign14980_e20994_d_n6;
        locals.var_exp_bvbs_dn7 = assign14980_e20994_d_n7;
        locals.var_exp_bvbs_dn10 = assign14980_e20994_d_n10;
        locals.var_exp_bvbs_dn11 = assign14980_e20994_d_n11;
        locals.var_exp_bvbs_dn12 = assign14980_e20994_d_n12;
        locals.var_exp_bvbs_dn17 = assign14980_e20994_d_n17;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign14990_e21001, assign14990_e21001_d_n0, assign14990_e21001_d_n2, assign14990_e21001_d_n6, assign14990_e21001_d_n7, assign14990_e21001_d_n10, assign14990_e21001_d_n11, assign14990_e21001_d_n12, assign14990_e21001_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign14990_e20999: f64 = (locals.var_cnst1soi * locals.var_exp_bvbs);
        (assign14990_e20999, ((locals.var_cnst1soi_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1soi_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1soi_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1soi_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1soi_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1soi_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1soi_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn12)), ((locals.var_cnst1soi_dn17 * locals.var_exp_bvbs) + (locals.var_cnst1soi * locals.var_exp_bvbs_dn17)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn12, locals.var_cfs1_dn17,)
    }
};
        locals.var_cfs1 = assign14990_e21001;
        locals.var_cfs1_dn0 = assign14990_e21001_d_n0;
        locals.var_cfs1_dn2 = assign14990_e21001_d_n2;
        locals.var_cfs1_dn6 = assign14990_e21001_d_n6;
        locals.var_cfs1_dn7 = assign14990_e21001_d_n7;
        locals.var_cfs1_dn10 = assign14990_e21001_d_n10;
        locals.var_cfs1_dn11 = assign14990_e21001_d_n11;
        locals.var_cfs1_dn12 = assign14990_e21001_d_n12;
        locals.var_cfs1_dn17 = assign14990_e21001_d_n17;
        locals.var_cfs1_rv = 0.0;

        let (assign15000_e21006,) = {
    if (locals.var_guard113 == 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign15000_e21006;
        locals.var_flg_conv_rv = 0.0;

        let (assign15010_e21011, assign15010_e21011_d_n0, assign15010_e21011_d_n2, assign15010_e21011_d_n6, assign15010_e21011_d_n7, assign15010_e21011_d_n10, assign15010_e21011_d_n11, assign15010_e21011_d_n12, assign15010_e21011_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign15010_e21011;
        locals.var_phi_s0_soi_dn0 = assign15010_e21011_d_n0;
        locals.var_phi_s0_soi_dn2 = assign15010_e21011_d_n2;
        locals.var_phi_s0_soi_dn6 = assign15010_e21011_d_n6;
        locals.var_phi_s0_soi_dn7 = assign15010_e21011_d_n7;
        locals.var_phi_s0_soi_dn10 = assign15010_e21011_d_n10;
        locals.var_phi_s0_soi_dn11 = assign15010_e21011_d_n11;
        locals.var_phi_s0_soi_dn12 = assign15010_e21011_d_n12;
        locals.var_phi_s0_soi_dn17 = assign15010_e21011_d_n17;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign15020_e21024, assign15020_e21024_d_n0, assign15020_e21024_d_n2, assign15020_e21024_d_n6, assign15020_e21024_d_n7, assign15020_e21024_d_n10, assign15020_e21024_d_n11, assign15020_e21024_d_n12, assign15020_e21024_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15020_e21016: f64 = (locals.var_q_nsub * p.p237);
        let assign15020_e21018: f64 = (assign15020_e21016 * p.p237);
        let assign15020_e21020: f64 = (assign15020_e21018 / 2.0);
        let assign15020_e21022: f64 = (assign15020_e21020 / 1.034943e-10);
        (assign15020_e21022, ((((locals.var_q_nsub_dn0 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn12 * p.p237) * p.p237) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn17 * p.p237) * p.p237) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn12, locals.var_dphi_sb_dn17,)
    }
};
        locals.var_dphi_sb = assign15020_e21024;
        locals.var_dphi_sb_dn0 = assign15020_e21024_d_n0;
        locals.var_dphi_sb_dn2 = assign15020_e21024_d_n2;
        locals.var_dphi_sb_dn6 = assign15020_e21024_d_n6;
        locals.var_dphi_sb_dn7 = assign15020_e21024_d_n7;
        locals.var_dphi_sb_dn10 = assign15020_e21024_d_n10;
        locals.var_dphi_sb_dn11 = assign15020_e21024_d_n11;
        locals.var_dphi_sb_dn12 = assign15020_e21024_d_n12;
        locals.var_dphi_sb_dn17 = assign15020_e21024_d_n17;
        locals.var_dphi_sb_rv = 0.0;

        let (assign15030_e21034, assign15030_e21034_d_n0, assign15030_e21034_d_n2, assign15030_e21034_d_n6, assign15030_e21034_d_n7, assign15030_e21034_d_n10, assign15030_e21034_d_n11, assign15030_e21034_d_n12, assign15030_e21034_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15030_e21029: f64 = (2.0 * locals.var_beta);
        let assign15030_e21031: f64 = (assign15030_e21029 * locals.var_dphi_sb);
        let assign15030_e21032: f64 = (assign15030_e21031).sqrt();
        (assign15030_e21032, ((assign15030_e21029 * locals.var_dphi_sb_dn0) / (2.0 * assign15030_e21032)), ((assign15030_e21029 * locals.var_dphi_sb_dn2) / (2.0 * assign15030_e21032)), ((assign15030_e21029 * locals.var_dphi_sb_dn6) / (2.0 * assign15030_e21032)), ((assign15030_e21029 * locals.var_dphi_sb_dn7) / (2.0 * assign15030_e21032)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign15030_e21029 * locals.var_dphi_sb_dn10)) / (2.0 * assign15030_e21032)), ((assign15030_e21029 * locals.var_dphi_sb_dn11) / (2.0 * assign15030_e21032)), ((assign15030_e21029 * locals.var_dphi_sb_dn12) / (2.0 * assign15030_e21032)), ((assign15030_e21029 * locals.var_dphi_sb_dn17) / (2.0 * assign15030_e21032)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign15030_e21034;
        locals.var_t0_dn0 = assign15030_e21034_d_n0;
        locals.var_t0_dn2 = assign15030_e21034_d_n2;
        locals.var_t0_dn6 = assign15030_e21034_d_n6;
        locals.var_t0_dn7 = assign15030_e21034_d_n7;
        locals.var_t0_dn10 = assign15030_e21034_d_n10;
        locals.var_t0_dn11 = assign15030_e21034_d_n11;
        locals.var_t0_dn12 = assign15030_e21034_d_n12;
        locals.var_t0_dn17 = assign15030_e21034_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign15040_e21046, assign15040_e21046_d_n0, assign15040_e21046_d_n2, assign15040_e21046_d_n6, assign15040_e21046_d_n7, assign15040_e21046_d_n10, assign15040_e21046_d_n11, assign15040_e21046_d_n12, assign15040_e21046_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15040_e21038: f64 = (locals.var_t0).exp();
        let assign15040_e21040: f64 = (-locals.var_t0);
        let assign15040_e21041: f64 = (assign15040_e21040).exp();
        let assign15040_e21042: f64 = (assign15040_e21038 + assign15040_e21041);
        let assign15040_e21044: f64 = (assign15040_e21042 / 2.0);
        (assign15040_e21044, (((assign15040_e21038 * locals.var_t0_dn0) + (assign15040_e21041 * (-locals.var_t0_dn0))) / 2.0), (((assign15040_e21038 * locals.var_t0_dn2) + (assign15040_e21041 * (-locals.var_t0_dn2))) / 2.0), (((assign15040_e21038 * locals.var_t0_dn6) + (assign15040_e21041 * (-locals.var_t0_dn6))) / 2.0), (((assign15040_e21038 * locals.var_t0_dn7) + (assign15040_e21041 * (-locals.var_t0_dn7))) / 2.0), (((assign15040_e21038 * locals.var_t0_dn10) + (assign15040_e21041 * (-locals.var_t0_dn10))) / 2.0), (((assign15040_e21038 * locals.var_t0_dn11) + (assign15040_e21041 * (-locals.var_t0_dn11))) / 2.0), (((assign15040_e21038 * locals.var_t0_dn12) + (assign15040_e21041 * (-locals.var_t0_dn12))) / 2.0), (((assign15040_e21038 * locals.var_t0_dn17) + (assign15040_e21041 * (-locals.var_t0_dn17))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15040_e21046;
        locals.var_t1_dn0 = assign15040_e21046_d_n0;
        locals.var_t1_dn2 = assign15040_e21046_d_n2;
        locals.var_t1_dn6 = assign15040_e21046_d_n6;
        locals.var_t1_dn7 = assign15040_e21046_d_n7;
        locals.var_t1_dn10 = assign15040_e21046_d_n10;
        locals.var_t1_dn11 = assign15040_e21046_d_n11;
        locals.var_t1_dn12 = assign15040_e21046_d_n12;
        locals.var_t1_dn17 = assign15040_e21046_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15050_e21054, assign15050_e21054_d_n0, assign15050_e21054_d_n2, assign15050_e21054_d_n6, assign15050_e21054_d_n7, assign15050_e21054_d_n10, assign15050_e21054_d_n11, assign15050_e21054_d_n12, assign15050_e21054_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15050_e21050: f64 = (locals.var_t1).ln();
        let assign15050_e21052: f64 = (assign15050_e21050 / locals.var_dphi_sb);
        (assign15050_e21052, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign15050_e21050 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign15050_e21050 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign15050_e21050 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign15050_e21050 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign15050_e21050 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign15050_e21050 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn12 / locals.var_t1) * locals.var_dphi_sb) - (assign15050_e21050 * locals.var_dphi_sb_dn12)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn17 / locals.var_t1) * locals.var_dphi_sb) - (assign15050_e21050 * locals.var_dphi_sb_dn17)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn12, locals.var_c_sb_dn17,)
    }
};
        locals.var_c_sb = assign15050_e21054;
        locals.var_c_sb_dn0 = assign15050_e21054_d_n0;
        locals.var_c_sb_dn2 = assign15050_e21054_d_n2;
        locals.var_c_sb_dn6 = assign15050_e21054_d_n6;
        locals.var_c_sb_dn7 = assign15050_e21054_d_n7;
        locals.var_c_sb_dn10 = assign15050_e21054_d_n10;
        locals.var_c_sb_dn11 = assign15050_e21054_d_n11;
        locals.var_c_sb_dn12 = assign15050_e21054_d_n12;
        locals.var_c_sb_dn17 = assign15050_e21054_d_n17;
        locals.var_c_sb_rv = 0.0;

        let (assign15060_e21059,) = {
    if (locals.var_guard113 == 0.0) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign15060_e21059;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign15070_loop_guard: usize = 0;
        while {
            let assign15070_cond_e21065: f64 = (locals.var_lp_s0_max + 1.0);
            let assign15070_cond_e21067: f64 = if ((locals.var_guard113 == 0.0) && (locals.var_lp_s0 <= assign15070_cond_e21065)) { 1.0 } else { 0.0 };
            assign15070_cond_e21067 != 0.0
        } {
            assign15070_loop_guard += 1;
            assert!(assign15070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15070_body0_e21074, assign15070_body0_e21074_d_n0, assign15070_body0_e21074_d_n2, assign15070_body0_e21074_d_n6, assign15070_body0_e21074_d_n7, assign15070_body0_e21074_d_n10, assign15070_body0_e21074_d_n11, assign15070_body0_e21074_d_n12, assign15070_body0_e21074_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15070_body0_e21072: f64 = (locals.var_phi_s0_soi - locals.var_vbcs_cl);
        (assign15070_body0_e21072, (locals.var_phi_s0_soi_dn0 - locals.var_vbcs_cl_dn0), (locals.var_phi_s0_soi_dn2 - locals.var_vbcs_cl_dn2), (locals.var_phi_s0_soi_dn6 - locals.var_vbcs_cl_dn6), (locals.var_phi_s0_soi_dn7 - locals.var_vbcs_cl_dn7), (locals.var_phi_s0_soi_dn10 - locals.var_vbcs_cl_dn10), (locals.var_phi_s0_soi_dn11 - locals.var_vbcs_cl_dn11), (locals.var_phi_s0_soi_dn12 - locals.var_vbcs_cl_dn12), (locals.var_phi_s0_soi_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_phi_soi0, locals.var_phi_soi0_dn0, locals.var_phi_soi0_dn2, locals.var_phi_soi0_dn6, locals.var_phi_soi0_dn7, locals.var_phi_soi0_dn10, locals.var_phi_soi0_dn11, locals.var_phi_soi0_dn12, locals.var_phi_soi0_dn17,)
    }
};
            locals.var_phi_soi0 = assign15070_body0_e21074;
            locals.var_phi_soi0_dn0 = assign15070_body0_e21074_d_n0;
            locals.var_phi_soi0_dn2 = assign15070_body0_e21074_d_n2;
            locals.var_phi_soi0_dn6 = assign15070_body0_e21074_d_n6;
            locals.var_phi_soi0_dn7 = assign15070_body0_e21074_d_n7;
            locals.var_phi_soi0_dn10 = assign15070_body0_e21074_d_n10;
            locals.var_phi_soi0_dn11 = assign15070_body0_e21074_d_n11;
            locals.var_phi_soi0_dn12 = assign15070_body0_e21074_d_n12;
            locals.var_phi_soi0_dn17 = assign15070_body0_e21074_d_n17;
            locals.var_phi_soi0_rv = 0.0;
            let (assign15070_body1_e21081, assign15070_body1_e21081_d_n0, assign15070_body1_e21081_d_n2, assign15070_body1_e21081_d_n6, assign15070_body1_e21081_d_n7, assign15070_body1_e21081_d_n10, assign15070_body1_e21081_d_n11, assign15070_body1_e21081_d_n12, assign15070_body1_e21081_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15070_body1_e21079: f64 = (locals.var_beta * locals.var_phi_soi0);
        (assign15070_body1_e21079, (locals.var_beta * locals.var_phi_soi0_dn0), (locals.var_beta * locals.var_phi_soi0_dn2), (locals.var_beta * locals.var_phi_soi0_dn6), (locals.var_beta * locals.var_phi_soi0_dn7), ((locals.var_beta_dn10 * locals.var_phi_soi0) + (locals.var_beta * locals.var_phi_soi0_dn10)), (locals.var_beta * locals.var_phi_soi0_dn11), (locals.var_beta * locals.var_phi_soi0_dn12), (locals.var_beta * locals.var_phi_soi0_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign15070_body1_e21081;
            locals.var_chi_dn0 = assign15070_body1_e21081_d_n0;
            locals.var_chi_dn2 = assign15070_body1_e21081_d_n2;
            locals.var_chi_dn6 = assign15070_body1_e21081_d_n6;
            locals.var_chi_dn7 = assign15070_body1_e21081_d_n7;
            locals.var_chi_dn10 = assign15070_body1_e21081_d_n10;
            locals.var_chi_dn11 = assign15070_body1_e21081_d_n11;
            locals.var_chi_dn12 = assign15070_body1_e21081_d_n12;
            locals.var_chi_dn17 = assign15070_body1_e21081_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign15070_body2_e21090, assign15070_body2_e21090_d_n0, assign15070_body2_e21090_d_n2, assign15070_body2_e21090_d_n6, assign15070_body2_e21090_d_n7, assign15070_body2_e21090_d_n10, assign15070_body2_e21090_d_n11, assign15070_body2_e21090_d_n12, assign15070_body2_e21090_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15070_body2_e21087: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        let assign15070_body2_e21088: f64 = (locals.var_c_sb * assign15070_body2_e21087);
        (assign15070_body2_e21088, ((locals.var_c_sb_dn0 * assign15070_body2_e21087) + (locals.var_c_sb * (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign15070_body2_e21087) + (locals.var_c_sb * (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign15070_body2_e21087) + (locals.var_c_sb * (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign15070_body2_e21087) + (locals.var_c_sb * (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign15070_body2_e21087) + (locals.var_c_sb * (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign15070_body2_e21087) + (locals.var_c_sb * (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign15070_body2_e21087) + (locals.var_c_sb * (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign15070_body2_e21087) + (locals.var_c_sb * (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign15070_body2_e21090;
            locals.var_ty_dn0 = assign15070_body2_e21090_d_n0;
            locals.var_ty_dn2 = assign15070_body2_e21090_d_n2;
            locals.var_ty_dn6 = assign15070_body2_e21090_d_n6;
            locals.var_ty_dn7 = assign15070_body2_e21090_d_n7;
            locals.var_ty_dn10 = assign15070_body2_e21090_d_n10;
            locals.var_ty_dn11 = assign15070_body2_e21090_d_n11;
            locals.var_ty_dn12 = assign15070_body2_e21090_d_n12;
            locals.var_ty_dn17 = assign15070_body2_e21090_d_n17;
            locals.var_ty_rv = 0.0;
            let assign15070_body3_e21093: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard451 = assign15070_body3_e21093;
            locals.var_guard451_rv = 0.0;
            let (assign15070_body4_e21101, assign15070_body4_e21101_d_n0, assign15070_body4_e21101_d_n2, assign15070_body4_e21101_d_n6, assign15070_body4_e21101_d_n7, assign15070_body4_e21101_d_n10, assign15070_body4_e21101_d_n11, assign15070_body4_e21101_d_n12, assign15070_body4_e21101_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard451 != 0.0)) {
        let assign15070_body4_e21099: f64 = (locals.var_ty).exp();
        (assign15070_body4_e21099, (assign15070_body4_e21099 * locals.var_ty_dn0), (assign15070_body4_e21099 * locals.var_ty_dn2), (assign15070_body4_e21099 * locals.var_ty_dn6), (assign15070_body4_e21099 * locals.var_ty_dn7), (assign15070_body4_e21099 * locals.var_ty_dn10), (assign15070_body4_e21099 * locals.var_ty_dn11), (assign15070_body4_e21099 * locals.var_ty_dn12), (assign15070_body4_e21099 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15070_body4_e21101;
            locals.var_t1_dn0 = assign15070_body4_e21101_d_n0;
            locals.var_t1_dn2 = assign15070_body4_e21101_d_n2;
            locals.var_t1_dn6 = assign15070_body4_e21101_d_n6;
            locals.var_t1_dn7 = assign15070_body4_e21101_d_n7;
            locals.var_t1_dn10 = assign15070_body4_e21101_d_n10;
            locals.var_t1_dn11 = assign15070_body4_e21101_d_n11;
            locals.var_t1_dn12 = assign15070_body4_e21101_d_n12;
            locals.var_t1_dn17 = assign15070_body4_e21101_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15070_body5_e21112, assign15070_body5_e21112_d_n0, assign15070_body5_e21112_d_n2, assign15070_body5_e21112_d_n6, assign15070_body5_e21112_d_n7, assign15070_body5_e21112_d_n10, assign15070_body5_e21112_d_n11, assign15070_body5_e21112_d_n12, assign15070_body5_e21112_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard451 != 0.0)) {
        let assign15070_body5_e21107: f64 = (-locals.var_c_sb);
        let assign15070_body5_e21109: f64 = (assign15070_body5_e21107 * locals.var_dphi_sb);
        let assign15070_body5_e21110: f64 = (assign15070_body5_e21109).exp();
        (assign15070_body5_e21110, (assign15070_body5_e21110 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign15070_body5_e21107 * locals.var_dphi_sb_dn0))), (assign15070_body5_e21110 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign15070_body5_e21107 * locals.var_dphi_sb_dn2))), (assign15070_body5_e21110 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign15070_body5_e21107 * locals.var_dphi_sb_dn6))), (assign15070_body5_e21110 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign15070_body5_e21107 * locals.var_dphi_sb_dn7))), (assign15070_body5_e21110 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign15070_body5_e21107 * locals.var_dphi_sb_dn10))), (assign15070_body5_e21110 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign15070_body5_e21107 * locals.var_dphi_sb_dn11))), (assign15070_body5_e21110 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign15070_body5_e21107 * locals.var_dphi_sb_dn12))), (assign15070_body5_e21110 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign15070_body5_e21107 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15070_body5_e21112;
            locals.var_t0_dn0 = assign15070_body5_e21112_d_n0;
            locals.var_t0_dn2 = assign15070_body5_e21112_d_n2;
            locals.var_t0_dn6 = assign15070_body5_e21112_d_n6;
            locals.var_t0_dn7 = assign15070_body5_e21112_d_n7;
            locals.var_t0_dn10 = assign15070_body5_e21112_d_n10;
            locals.var_t0_dn11 = assign15070_body5_e21112_d_n11;
            locals.var_t0_dn12 = assign15070_body5_e21112_d_n12;
            locals.var_t0_dn17 = assign15070_body5_e21112_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15070_body6_e21121, assign15070_body6_e21121_d_n0, assign15070_body6_e21121_d_n2, assign15070_body6_e21121_d_n6, assign15070_body6_e21121_d_n7, assign15070_body6_e21121_d_n10, assign15070_body6_e21121_d_n11, assign15070_body6_e21121_d_n12, assign15070_body6_e21121_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard451 != 0.0)) {
        let assign15070_body6_e21119: f64 = (locals.var_t1 - locals.var_t0);
        (assign15070_body6_e21119, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign15070_body6_e21121;
            locals.var_t2_dn0 = assign15070_body6_e21121_d_n0;
            locals.var_t2_dn2 = assign15070_body6_e21121_d_n2;
            locals.var_t2_dn6 = assign15070_body6_e21121_d_n6;
            locals.var_t2_dn7 = assign15070_body6_e21121_d_n7;
            locals.var_t2_dn10 = assign15070_body6_e21121_d_n10;
            locals.var_t2_dn11 = assign15070_body6_e21121_d_n11;
            locals.var_t2_dn12 = assign15070_body6_e21121_d_n12;
            locals.var_t2_dn17 = assign15070_body6_e21121_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign15070_body7_e21133, assign15070_body7_e21133_d_n0, assign15070_body7_e21133_d_n2, assign15070_body7_e21133_d_n6, assign15070_body7_e21133_d_n7, assign15070_body7_e21133_d_n10, assign15070_body7_e21133_d_n11, assign15070_body7_e21133_d_n12, assign15070_body7_e21133_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard451 != 0.0)) {
        let assign15070_body7_e21128: f64 = (1.0 + locals.var_t2);
        let assign15070_body7_e21129: f64 = (assign15070_body7_e21128).ln();
        let assign15070_body7_e21131: f64 = (assign15070_body7_e21129 / locals.var_c_sb);
        (assign15070_body7_e21131, ((((locals.var_t2_dn0 / assign15070_body7_e21128) * locals.var_c_sb) - (assign15070_body7_e21129 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign15070_body7_e21128) * locals.var_c_sb) - (assign15070_body7_e21129 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign15070_body7_e21128) * locals.var_c_sb) - (assign15070_body7_e21129 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign15070_body7_e21128) * locals.var_c_sb) - (assign15070_body7_e21129 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign15070_body7_e21128) * locals.var_c_sb) - (assign15070_body7_e21129 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign15070_body7_e21128) * locals.var_c_sb) - (assign15070_body7_e21129 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign15070_body7_e21128) * locals.var_c_sb) - (assign15070_body7_e21129 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign15070_body7_e21128) * locals.var_c_sb) - (assign15070_body7_e21129 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign15070_body7_e21133;
            locals.var_phi_soib_dn0 = assign15070_body7_e21133_d_n0;
            locals.var_phi_soib_dn2 = assign15070_body7_e21133_d_n2;
            locals.var_phi_soib_dn6 = assign15070_body7_e21133_d_n6;
            locals.var_phi_soib_dn7 = assign15070_body7_e21133_d_n7;
            locals.var_phi_soib_dn10 = assign15070_body7_e21133_d_n10;
            locals.var_phi_soib_dn11 = assign15070_body7_e21133_d_n11;
            locals.var_phi_soib_dn12 = assign15070_body7_e21133_d_n12;
            locals.var_phi_soib_dn17 = assign15070_body7_e21133_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign15070_body8_e21144, assign15070_body8_e21144_d_n0, assign15070_body8_e21144_d_n2, assign15070_body8_e21144_d_n6, assign15070_body8_e21144_d_n7, assign15070_body8_e21144_d_n10, assign15070_body8_e21144_d_n11, assign15070_body8_e21144_d_n12, assign15070_body8_e21144_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard451 != 0.0)) {
        let assign15070_body8_e21141: f64 = (1.0 + locals.var_t2);
        let assign15070_body8_e21142: f64 = (locals.var_t1 / assign15070_body8_e21141);
        (assign15070_body8_e21142, (((locals.var_t1_dn0 * assign15070_body8_e21141) - (locals.var_t1 * locals.var_t2_dn0)) / (assign15070_body8_e21141 * assign15070_body8_e21141)), (((locals.var_t1_dn2 * assign15070_body8_e21141) - (locals.var_t1 * locals.var_t2_dn2)) / (assign15070_body8_e21141 * assign15070_body8_e21141)), (((locals.var_t1_dn6 * assign15070_body8_e21141) - (locals.var_t1 * locals.var_t2_dn6)) / (assign15070_body8_e21141 * assign15070_body8_e21141)), (((locals.var_t1_dn7 * assign15070_body8_e21141) - (locals.var_t1 * locals.var_t2_dn7)) / (assign15070_body8_e21141 * assign15070_body8_e21141)), (((locals.var_t1_dn10 * assign15070_body8_e21141) - (locals.var_t1 * locals.var_t2_dn10)) / (assign15070_body8_e21141 * assign15070_body8_e21141)), (((locals.var_t1_dn11 * assign15070_body8_e21141) - (locals.var_t1 * locals.var_t2_dn11)) / (assign15070_body8_e21141 * assign15070_body8_e21141)), (((locals.var_t1_dn12 * assign15070_body8_e21141) - (locals.var_t1 * locals.var_t2_dn12)) / (assign15070_body8_e21141 * assign15070_body8_e21141)), (((locals.var_t1_dn17 * assign15070_body8_e21141) - (locals.var_t1 * locals.var_t2_dn17)) / (assign15070_body8_e21141 * assign15070_body8_e21141)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign15070_body8_e21144;
            locals.var_phi_soib_dpss_dn0 = assign15070_body8_e21144_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign15070_body8_e21144_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign15070_body8_e21144_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign15070_body8_e21144_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign15070_body8_e21144_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign15070_body8_e21144_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign15070_body8_e21144_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign15070_body8_e21144_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign15070_body9_e21154, assign15070_body9_e21154_d_n0, assign15070_body9_e21154_d_n2, assign15070_body9_e21154_d_n6, assign15070_body9_e21154_d_n7, assign15070_body9_e21154_d_n10, assign15070_body9_e21154_d_n11, assign15070_body9_e21154_d_n12, assign15070_body9_e21154_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard451 == 0.0)) {
        let assign15070_body9_e21152: f64 = (locals.var_phi_soi0 - locals.var_dphi_sb);
        (assign15070_body9_e21152, (locals.var_phi_soi0_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soi0_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soi0_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soi0_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soi0_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soi0_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soi0_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soi0_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign15070_body9_e21154;
            locals.var_phi_soib_dn0 = assign15070_body9_e21154_d_n0;
            locals.var_phi_soib_dn2 = assign15070_body9_e21154_d_n2;
            locals.var_phi_soib_dn6 = assign15070_body9_e21154_d_n6;
            locals.var_phi_soib_dn7 = assign15070_body9_e21154_d_n7;
            locals.var_phi_soib_dn10 = assign15070_body9_e21154_d_n10;
            locals.var_phi_soib_dn11 = assign15070_body9_e21154_d_n11;
            locals.var_phi_soib_dn12 = assign15070_body9_e21154_d_n12;
            locals.var_phi_soib_dn17 = assign15070_body9_e21154_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign15070_body10_e21162, assign15070_body10_e21162_d_n0, assign15070_body10_e21162_d_n2, assign15070_body10_e21162_d_n6, assign15070_body10_e21162_d_n7, assign15070_body10_e21162_d_n10, assign15070_body10_e21162_d_n11, assign15070_body10_e21162_d_n12, assign15070_body10_e21162_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard451 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign15070_body10_e21162;
            locals.var_phi_soib_dpss_dn0 = assign15070_body10_e21162_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign15070_body10_e21162_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign15070_body10_e21162_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign15070_body10_e21162_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign15070_body10_e21162_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign15070_body10_e21162_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign15070_body10_e21162_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign15070_body10_e21162_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign15070_body11_e21169, assign15070_body11_e21169_d_n0, assign15070_body11_e21169_d_n2, assign15070_body11_e21169_d_n6, assign15070_body11_e21169_d_n7, assign15070_body11_e21169_d_n10, assign15070_body11_e21169_d_n11, assign15070_body11_e21169_d_n12, assign15070_body11_e21169_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15070_body11_e21167: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign15070_body11_e21167, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign15070_body11_e21169;
            locals.var_chib_dn0 = assign15070_body11_e21169_d_n0;
            locals.var_chib_dn2 = assign15070_body11_e21169_d_n2;
            locals.var_chib_dn6 = assign15070_body11_e21169_d_n6;
            locals.var_chib_dn7 = assign15070_body11_e21169_d_n7;
            locals.var_chib_dn10 = assign15070_body11_e21169_d_n10;
            locals.var_chib_dn11 = assign15070_body11_e21169_d_n11;
            locals.var_chib_dn12 = assign15070_body11_e21169_d_n12;
            locals.var_chib_dn17 = assign15070_body11_e21169_d_n17;
            locals.var_chib_rv = 0.0;
            let assign15070_body12_e21171: f64 = (locals.var_chi).abs();
            let assign15070_body12_e21173: f64 = if assign15070_body12_e21171 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard452 = assign15070_body12_e21173;
            locals.var_guard452_rv = 0.0;
            let (assign15070_body13_e21187, assign15070_body13_e21187_d_n0, assign15070_body13_e21187_d_n2, assign15070_body13_e21187_d_n6, assign15070_body13_e21187_d_n7, assign15070_body13_e21187_d_n10, assign15070_body13_e21187_d_n11, assign15070_body13_e21187_d_n12, assign15070_body13_e21187_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard452 != 0.0)) {
        let assign15070_body13_e21181: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign15070_body13_e21182: f64 = (1.0 - assign15070_body13_e21181);
        let assign15070_body13_e21184: f64 = (assign15070_body13_e21182 / 2.0);
        let assign15070_body13_e21185: f64 = (assign15070_body13_e21184).sqrt();
        (assign15070_body13_e21185, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign15070_body13_e21185)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign15070_body13_e21185)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign15070_body13_e21185)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign15070_body13_e21185)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign15070_body13_e21185)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign15070_body13_e21185)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign15070_body13_e21185)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign15070_body13_e21185)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15070_body13_e21187;
            locals.var_t0_dn0 = assign15070_body13_e21187_d_n0;
            locals.var_t0_dn2 = assign15070_body13_e21187_d_n2;
            locals.var_t0_dn6 = assign15070_body13_e21187_d_n6;
            locals.var_t0_dn7 = assign15070_body13_e21187_d_n7;
            locals.var_t0_dn10 = assign15070_body13_e21187_d_n10;
            locals.var_t0_dn11 = assign15070_body13_e21187_d_n11;
            locals.var_t0_dn12 = assign15070_body13_e21187_d_n12;
            locals.var_t0_dn17 = assign15070_body13_e21187_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15070_body14_e21196, assign15070_body14_e21196_d_n0, assign15070_body14_e21196_d_n2, assign15070_body14_e21196_d_n6, assign15070_body14_e21196_d_n7, assign15070_body14_e21196_d_n10, assign15070_body14_e21196_d_n11, assign15070_body14_e21196_d_n12, assign15070_body14_e21196_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard452 != 0.0)) {
        let assign15070_body14_e21194: f64 = (locals.var_chi * locals.var_t0);
        (assign15070_body14_e21194, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15070_body14_e21196;
            locals.var_fb_dn0 = assign15070_body14_e21196_d_n0;
            locals.var_fb_dn2 = assign15070_body14_e21196_d_n2;
            locals.var_fb_dn6 = assign15070_body14_e21196_d_n6;
            locals.var_fb_dn7 = assign15070_body14_e21196_d_n7;
            locals.var_fb_dn10 = assign15070_body14_e21196_d_n10;
            locals.var_fb_dn11 = assign15070_body14_e21196_d_n11;
            locals.var_fb_dn12 = assign15070_body14_e21196_d_n12;
            locals.var_fb_dn17 = assign15070_body14_e21196_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15070_body15_e21205, assign15070_body15_e21205_d_n0, assign15070_body15_e21205_d_n2, assign15070_body15_e21205_d_n6, assign15070_body15_e21205_d_n7, assign15070_body15_e21205_d_n10, assign15070_body15_e21205_d_n11, assign15070_body15_e21205_d_n12, assign15070_body15_e21205_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard452 != 0.0)) {
        let assign15070_body15_e21203: f64 = (locals.var_beta * locals.var_t0);
        (assign15070_body15_e21203, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15070_body15_e21205;
            locals.var_fb_dpss_dn0 = assign15070_body15_e21205_d_n0;
            locals.var_fb_dpss_dn2 = assign15070_body15_e21205_d_n2;
            locals.var_fb_dpss_dn6 = assign15070_body15_e21205_d_n6;
            locals.var_fb_dpss_dn7 = assign15070_body15_e21205_d_n7;
            locals.var_fb_dpss_dn10 = assign15070_body15_e21205_d_n10;
            locals.var_fb_dpss_dn11 = assign15070_body15_e21205_d_n11;
            locals.var_fb_dpss_dn12 = assign15070_body15_e21205_d_n12;
            locals.var_fb_dpss_dn17 = assign15070_body15_e21205_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15070_body16_e21208: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard453 = assign15070_body16_e21208;
            locals.var_guard453_rv = 0.0;
            let (assign15070_body17_e21218, assign15070_body17_e21218_d_n0, assign15070_body17_e21218_d_n2, assign15070_body17_e21218_d_n6, assign15070_body17_e21218_d_n7, assign15070_body17_e21218_d_n10, assign15070_body17_e21218_d_n11, assign15070_body17_e21218_d_n12, assign15070_body17_e21218_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 != 0.0)) && (locals.var_guard453 != 0.0)) {
        let assign15070_body17_e21216: f64 = (-locals.var_fb);
        (assign15070_body17_e21216, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15070_body17_e21218;
            locals.var_fb_dn0 = assign15070_body17_e21218_d_n0;
            locals.var_fb_dn2 = assign15070_body17_e21218_d_n2;
            locals.var_fb_dn6 = assign15070_body17_e21218_d_n6;
            locals.var_fb_dn7 = assign15070_body17_e21218_d_n7;
            locals.var_fb_dn10 = assign15070_body17_e21218_d_n10;
            locals.var_fb_dn11 = assign15070_body17_e21218_d_n11;
            locals.var_fb_dn12 = assign15070_body17_e21218_d_n12;
            locals.var_fb_dn17 = assign15070_body17_e21218_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15070_body18_e21228, assign15070_body18_e21228_d_n0, assign15070_body18_e21228_d_n2, assign15070_body18_e21228_d_n6, assign15070_body18_e21228_d_n7, assign15070_body18_e21228_d_n10, assign15070_body18_e21228_d_n11, assign15070_body18_e21228_d_n12, assign15070_body18_e21228_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 != 0.0)) && (locals.var_guard453 != 0.0)) {
        let assign15070_body18_e21226: f64 = (-locals.var_fb_dpss);
        (assign15070_body18_e21226, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15070_body18_e21228;
            locals.var_fb_dpss_dn0 = assign15070_body18_e21228_d_n0;
            locals.var_fb_dpss_dn2 = assign15070_body18_e21228_d_n2;
            locals.var_fb_dpss_dn6 = assign15070_body18_e21228_d_n6;
            locals.var_fb_dpss_dn7 = assign15070_body18_e21228_d_n7;
            locals.var_fb_dpss_dn10 = assign15070_body18_e21228_d_n10;
            locals.var_fb_dpss_dn11 = assign15070_body18_e21228_d_n11;
            locals.var_fb_dpss_dn12 = assign15070_body18_e21228_d_n12;
            locals.var_fb_dpss_dn17 = assign15070_body18_e21228_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15070_body19_e21230: f64 = (locals.var_chi).abs();
            let assign15070_body19_e21232: f64 = if assign15070_body19_e21230 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard454 = assign15070_body19_e21232;
            locals.var_guard454_rv = 0.0;
            let (assign15070_body20_e21264, assign15070_body20_e21264_d_n0, assign15070_body20_e21264_d_n2, assign15070_body20_e21264_d_n6, assign15070_body20_e21264_d_n7, assign15070_body20_e21264_d_n10, assign15070_body20_e21264_d_n11, assign15070_body20_e21264_d_n12, assign15070_body20_e21264_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 != 0.0)) {
        let assign15070_body20_e21242: f64 = (locals.var_chi * locals.var_chi);
        let assign15070_body20_e21244: f64 = (assign15070_body20_e21242 / 2.0);
        let assign15070_body20_e21248: f64 = (locals.var_chi / 3.0);
        let assign15070_body20_e21252: f64 = (locals.var_chi / 4.0);
        let assign15070_body20_e21256: f64 = (locals.var_chi / 5.0);
        let assign15070_body20_e21257: f64 = (1.0 - assign15070_body20_e21256);
        let assign15070_body20_e21258: f64 = (assign15070_body20_e21252 * assign15070_body20_e21257);
        let assign15070_body20_e21259: f64 = (1.0 - assign15070_body20_e21258);
        let assign15070_body20_e21260: f64 = (assign15070_body20_e21248 * assign15070_body20_e21259);
        let assign15070_body20_e21261: f64 = (1.0 - assign15070_body20_e21260);
        let assign15070_body20_e21262: f64 = (assign15070_body20_e21244 * assign15070_body20_e21261);
        (assign15070_body20_e21262, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign15070_body20_e21261) + (assign15070_body20_e21244 * (-(((locals.var_chi_dn0 / 3.0) * assign15070_body20_e21259) + (assign15070_body20_e21248 * (-(((locals.var_chi_dn0 / 4.0) * assign15070_body20_e21257) + (assign15070_body20_e21252 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign15070_body20_e21261) + (assign15070_body20_e21244 * (-(((locals.var_chi_dn2 / 3.0) * assign15070_body20_e21259) + (assign15070_body20_e21248 * (-(((locals.var_chi_dn2 / 4.0) * assign15070_body20_e21257) + (assign15070_body20_e21252 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign15070_body20_e21261) + (assign15070_body20_e21244 * (-(((locals.var_chi_dn6 / 3.0) * assign15070_body20_e21259) + (assign15070_body20_e21248 * (-(((locals.var_chi_dn6 / 4.0) * assign15070_body20_e21257) + (assign15070_body20_e21252 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign15070_body20_e21261) + (assign15070_body20_e21244 * (-(((locals.var_chi_dn7 / 3.0) * assign15070_body20_e21259) + (assign15070_body20_e21248 * (-(((locals.var_chi_dn7 / 4.0) * assign15070_body20_e21257) + (assign15070_body20_e21252 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign15070_body20_e21261) + (assign15070_body20_e21244 * (-(((locals.var_chi_dn10 / 3.0) * assign15070_body20_e21259) + (assign15070_body20_e21248 * (-(((locals.var_chi_dn10 / 4.0) * assign15070_body20_e21257) + (assign15070_body20_e21252 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign15070_body20_e21261) + (assign15070_body20_e21244 * (-(((locals.var_chi_dn11 / 3.0) * assign15070_body20_e21259) + (assign15070_body20_e21248 * (-(((locals.var_chi_dn11 / 4.0) * assign15070_body20_e21257) + (assign15070_body20_e21252 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign15070_body20_e21261) + (assign15070_body20_e21244 * (-(((locals.var_chi_dn12 / 3.0) * assign15070_body20_e21259) + (assign15070_body20_e21248 * (-(((locals.var_chi_dn12 / 4.0) * assign15070_body20_e21257) + (assign15070_body20_e21252 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign15070_body20_e21261) + (assign15070_body20_e21244 * (-(((locals.var_chi_dn17 / 3.0) * assign15070_body20_e21259) + (assign15070_body20_e21248 * (-(((locals.var_chi_dn17 / 4.0) * assign15070_body20_e21257) + (assign15070_body20_e21252 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15070_body20_e21264;
            locals.var_t0_dn0 = assign15070_body20_e21264_d_n0;
            locals.var_t0_dn2 = assign15070_body20_e21264_d_n2;
            locals.var_t0_dn6 = assign15070_body20_e21264_d_n6;
            locals.var_t0_dn7 = assign15070_body20_e21264_d_n7;
            locals.var_t0_dn10 = assign15070_body20_e21264_d_n10;
            locals.var_t0_dn11 = assign15070_body20_e21264_d_n11;
            locals.var_t0_dn12 = assign15070_body20_e21264_d_n12;
            locals.var_t0_dn17 = assign15070_body20_e21264_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15070_body21_e21292, assign15070_body21_e21292_d_n0, assign15070_body21_e21292_d_n2, assign15070_body21_e21292_d_n6, assign15070_body21_e21292_d_n7, assign15070_body21_e21292_d_n10, assign15070_body21_e21292_d_n11, assign15070_body21_e21292_d_n12, assign15070_body21_e21292_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 != 0.0)) {
        let assign15070_body21_e21276: f64 = (locals.var_chi / 2.0);
        let assign15070_body21_e21280: f64 = (locals.var_chi / 3.0);
        let assign15070_body21_e21284: f64 = (locals.var_chi / 4.0);
        let assign15070_body21_e21285: f64 = (1.0 - assign15070_body21_e21284);
        let assign15070_body21_e21286: f64 = (assign15070_body21_e21280 * assign15070_body21_e21285);
        let assign15070_body21_e21287: f64 = (1.0 - assign15070_body21_e21286);
        let assign15070_body21_e21288: f64 = (assign15070_body21_e21276 * assign15070_body21_e21287);
        let assign15070_body21_e21289: f64 = (1.0 - assign15070_body21_e21288);
        let assign15070_body21_e21290: f64 = (locals.var_chi * assign15070_body21_e21289);
        (assign15070_body21_e21290, ((locals.var_chi_dn0 * assign15070_body21_e21289) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign15070_body21_e21287) + (assign15070_body21_e21276 * (-(((locals.var_chi_dn0 / 3.0) * assign15070_body21_e21285) + (assign15070_body21_e21280 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign15070_body21_e21289) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign15070_body21_e21287) + (assign15070_body21_e21276 * (-(((locals.var_chi_dn2 / 3.0) * assign15070_body21_e21285) + (assign15070_body21_e21280 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign15070_body21_e21289) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign15070_body21_e21287) + (assign15070_body21_e21276 * (-(((locals.var_chi_dn6 / 3.0) * assign15070_body21_e21285) + (assign15070_body21_e21280 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign15070_body21_e21289) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign15070_body21_e21287) + (assign15070_body21_e21276 * (-(((locals.var_chi_dn7 / 3.0) * assign15070_body21_e21285) + (assign15070_body21_e21280 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign15070_body21_e21289) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign15070_body21_e21287) + (assign15070_body21_e21276 * (-(((locals.var_chi_dn10 / 3.0) * assign15070_body21_e21285) + (assign15070_body21_e21280 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign15070_body21_e21289) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign15070_body21_e21287) + (assign15070_body21_e21276 * (-(((locals.var_chi_dn11 / 3.0) * assign15070_body21_e21285) + (assign15070_body21_e21280 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign15070_body21_e21289) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign15070_body21_e21287) + (assign15070_body21_e21276 * (-(((locals.var_chi_dn12 / 3.0) * assign15070_body21_e21285) + (assign15070_body21_e21280 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign15070_body21_e21289) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign15070_body21_e21287) + (assign15070_body21_e21276 * (-(((locals.var_chi_dn17 / 3.0) * assign15070_body21_e21285) + (assign15070_body21_e21280 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15070_body21_e21292;
            locals.var_t1_dn0 = assign15070_body21_e21292_d_n0;
            locals.var_t1_dn2 = assign15070_body21_e21292_d_n2;
            locals.var_t1_dn6 = assign15070_body21_e21292_d_n6;
            locals.var_t1_dn7 = assign15070_body21_e21292_d_n7;
            locals.var_t1_dn10 = assign15070_body21_e21292_d_n10;
            locals.var_t1_dn11 = assign15070_body21_e21292_d_n11;
            locals.var_t1_dn12 = assign15070_body21_e21292_d_n12;
            locals.var_t1_dn17 = assign15070_body21_e21292_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15070_body22_e21324, assign15070_body22_e21324_d_n0, assign15070_body22_e21324_d_n2, assign15070_body22_e21324_d_n6, assign15070_body22_e21324_d_n7, assign15070_body22_e21324_d_n10, assign15070_body22_e21324_d_n11, assign15070_body22_e21324_d_n12, assign15070_body22_e21324_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 != 0.0)) {
        let assign15070_body22_e21302: f64 = (locals.var_chib * locals.var_chib);
        let assign15070_body22_e21304: f64 = (assign15070_body22_e21302 / 2.0);
        let assign15070_body22_e21308: f64 = (locals.var_chib / 3.0);
        let assign15070_body22_e21312: f64 = (locals.var_chib / 4.0);
        let assign15070_body22_e21316: f64 = (locals.var_chib / 5.0);
        let assign15070_body22_e21317: f64 = (1.0 - assign15070_body22_e21316);
        let assign15070_body22_e21318: f64 = (assign15070_body22_e21312 * assign15070_body22_e21317);
        let assign15070_body22_e21319: f64 = (1.0 - assign15070_body22_e21318);
        let assign15070_body22_e21320: f64 = (assign15070_body22_e21308 * assign15070_body22_e21319);
        let assign15070_body22_e21321: f64 = (1.0 - assign15070_body22_e21320);
        let assign15070_body22_e21322: f64 = (assign15070_body22_e21304 * assign15070_body22_e21321);
        (assign15070_body22_e21322, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign15070_body22_e21321) + (assign15070_body22_e21304 * (-(((locals.var_chib_dn0 / 3.0) * assign15070_body22_e21319) + (assign15070_body22_e21308 * (-(((locals.var_chib_dn0 / 4.0) * assign15070_body22_e21317) + (assign15070_body22_e21312 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign15070_body22_e21321) + (assign15070_body22_e21304 * (-(((locals.var_chib_dn2 / 3.0) * assign15070_body22_e21319) + (assign15070_body22_e21308 * (-(((locals.var_chib_dn2 / 4.0) * assign15070_body22_e21317) + (assign15070_body22_e21312 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign15070_body22_e21321) + (assign15070_body22_e21304 * (-(((locals.var_chib_dn6 / 3.0) * assign15070_body22_e21319) + (assign15070_body22_e21308 * (-(((locals.var_chib_dn6 / 4.0) * assign15070_body22_e21317) + (assign15070_body22_e21312 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign15070_body22_e21321) + (assign15070_body22_e21304 * (-(((locals.var_chib_dn7 / 3.0) * assign15070_body22_e21319) + (assign15070_body22_e21308 * (-(((locals.var_chib_dn7 / 4.0) * assign15070_body22_e21317) + (assign15070_body22_e21312 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign15070_body22_e21321) + (assign15070_body22_e21304 * (-(((locals.var_chib_dn10 / 3.0) * assign15070_body22_e21319) + (assign15070_body22_e21308 * (-(((locals.var_chib_dn10 / 4.0) * assign15070_body22_e21317) + (assign15070_body22_e21312 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign15070_body22_e21321) + (assign15070_body22_e21304 * (-(((locals.var_chib_dn11 / 3.0) * assign15070_body22_e21319) + (assign15070_body22_e21308 * (-(((locals.var_chib_dn11 / 4.0) * assign15070_body22_e21317) + (assign15070_body22_e21312 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign15070_body22_e21321) + (assign15070_body22_e21304 * (-(((locals.var_chib_dn12 / 3.0) * assign15070_body22_e21319) + (assign15070_body22_e21308 * (-(((locals.var_chib_dn12 / 4.0) * assign15070_body22_e21317) + (assign15070_body22_e21312 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign15070_body22_e21321) + (assign15070_body22_e21304 * (-(((locals.var_chib_dn17 / 3.0) * assign15070_body22_e21319) + (assign15070_body22_e21308 * (-(((locals.var_chib_dn17 / 4.0) * assign15070_body22_e21317) + (assign15070_body22_e21312 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign15070_body22_e21324;
            locals.var_t2_dn0 = assign15070_body22_e21324_d_n0;
            locals.var_t2_dn2 = assign15070_body22_e21324_d_n2;
            locals.var_t2_dn6 = assign15070_body22_e21324_d_n6;
            locals.var_t2_dn7 = assign15070_body22_e21324_d_n7;
            locals.var_t2_dn10 = assign15070_body22_e21324_d_n10;
            locals.var_t2_dn11 = assign15070_body22_e21324_d_n11;
            locals.var_t2_dn12 = assign15070_body22_e21324_d_n12;
            locals.var_t2_dn17 = assign15070_body22_e21324_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign15070_body23_e21352, assign15070_body23_e21352_d_n0, assign15070_body23_e21352_d_n2, assign15070_body23_e21352_d_n6, assign15070_body23_e21352_d_n7, assign15070_body23_e21352_d_n10, assign15070_body23_e21352_d_n11, assign15070_body23_e21352_d_n12, assign15070_body23_e21352_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 != 0.0)) {
        let assign15070_body23_e21336: f64 = (locals.var_chib / 2.0);
        let assign15070_body23_e21340: f64 = (locals.var_chib / 3.0);
        let assign15070_body23_e21344: f64 = (locals.var_chib / 4.0);
        let assign15070_body23_e21345: f64 = (1.0 - assign15070_body23_e21344);
        let assign15070_body23_e21346: f64 = (assign15070_body23_e21340 * assign15070_body23_e21345);
        let assign15070_body23_e21347: f64 = (1.0 - assign15070_body23_e21346);
        let assign15070_body23_e21348: f64 = (assign15070_body23_e21336 * assign15070_body23_e21347);
        let assign15070_body23_e21349: f64 = (1.0 - assign15070_body23_e21348);
        let assign15070_body23_e21350: f64 = (locals.var_chib * assign15070_body23_e21349);
        (assign15070_body23_e21350, ((locals.var_chib_dn0 * assign15070_body23_e21349) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign15070_body23_e21347) + (assign15070_body23_e21336 * (-(((locals.var_chib_dn0 / 3.0) * assign15070_body23_e21345) + (assign15070_body23_e21340 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign15070_body23_e21349) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign15070_body23_e21347) + (assign15070_body23_e21336 * (-(((locals.var_chib_dn2 / 3.0) * assign15070_body23_e21345) + (assign15070_body23_e21340 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign15070_body23_e21349) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign15070_body23_e21347) + (assign15070_body23_e21336 * (-(((locals.var_chib_dn6 / 3.0) * assign15070_body23_e21345) + (assign15070_body23_e21340 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign15070_body23_e21349) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign15070_body23_e21347) + (assign15070_body23_e21336 * (-(((locals.var_chib_dn7 / 3.0) * assign15070_body23_e21345) + (assign15070_body23_e21340 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign15070_body23_e21349) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign15070_body23_e21347) + (assign15070_body23_e21336 * (-(((locals.var_chib_dn10 / 3.0) * assign15070_body23_e21345) + (assign15070_body23_e21340 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign15070_body23_e21349) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign15070_body23_e21347) + (assign15070_body23_e21336 * (-(((locals.var_chib_dn11 / 3.0) * assign15070_body23_e21345) + (assign15070_body23_e21340 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign15070_body23_e21349) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign15070_body23_e21347) + (assign15070_body23_e21336 * (-(((locals.var_chib_dn12 / 3.0) * assign15070_body23_e21345) + (assign15070_body23_e21340 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign15070_body23_e21349) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign15070_body23_e21347) + (assign15070_body23_e21336 * (-(((locals.var_chib_dn17 / 3.0) * assign15070_body23_e21345) + (assign15070_body23_e21340 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign15070_body23_e21352;
            locals.var_t3_dn0 = assign15070_body23_e21352_d_n0;
            locals.var_t3_dn2 = assign15070_body23_e21352_d_n2;
            locals.var_t3_dn6 = assign15070_body23_e21352_d_n6;
            locals.var_t3_dn7 = assign15070_body23_e21352_d_n7;
            locals.var_t3_dn10 = assign15070_body23_e21352_d_n10;
            locals.var_t3_dn11 = assign15070_body23_e21352_d_n11;
            locals.var_t3_dn12 = assign15070_body23_e21352_d_n12;
            locals.var_t3_dn17 = assign15070_body23_e21352_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign15070_body24_e21365, assign15070_body24_e21365_d_n0, assign15070_body24_e21365_d_n2, assign15070_body24_e21365_d_n6, assign15070_body24_e21365_d_n7, assign15070_body24_e21365_d_n10, assign15070_body24_e21365_d_n11, assign15070_body24_e21365_d_n12, assign15070_body24_e21365_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 != 0.0)) {
        let assign15070_body24_e21362: f64 = (locals.var_t0 - locals.var_t2);
        let assign15070_body24_e21363: f64 = (assign15070_body24_e21362).sqrt();
        (assign15070_body24_e21363, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign15070_body24_e21363)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign15070_body24_e21363)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign15070_body24_e21363)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign15070_body24_e21363)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign15070_body24_e21363)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign15070_body24_e21363)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign15070_body24_e21363)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign15070_body24_e21363)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15070_body24_e21365;
            locals.var_fb_dn0 = assign15070_body24_e21365_d_n0;
            locals.var_fb_dn2 = assign15070_body24_e21365_d_n2;
            locals.var_fb_dn6 = assign15070_body24_e21365_d_n6;
            locals.var_fb_dn7 = assign15070_body24_e21365_d_n7;
            locals.var_fb_dn10 = assign15070_body24_e21365_d_n10;
            locals.var_fb_dn11 = assign15070_body24_e21365_d_n11;
            locals.var_fb_dn12 = assign15070_body24_e21365_d_n12;
            locals.var_fb_dn17 = assign15070_body24_e21365_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15070_body25_e21385, assign15070_body25_e21385_d_n0, assign15070_body25_e21385_d_n2, assign15070_body25_e21385_d_n6, assign15070_body25_e21385_d_n7, assign15070_body25_e21385_d_n10, assign15070_body25_e21385_d_n11, assign15070_body25_e21385_d_n12, assign15070_body25_e21385_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 != 0.0)) {
        let assign15070_body25_e21375: f64 = (locals.var_beta * 0.5);
        let assign15070_body25_e21379: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign15070_body25_e21380: f64 = (locals.var_t1 - assign15070_body25_e21379);
        let assign15070_body25_e21381: f64 = (assign15070_body25_e21375 * assign15070_body25_e21380);
        let assign15070_body25_e21383: f64 = (assign15070_body25_e21381 / locals.var_fb);
        (assign15070_body25_e21383, ((((assign15070_body25_e21375 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign15070_body25_e21381 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body25_e21375 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign15070_body25_e21381 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body25_e21375 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign15070_body25_e21381 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body25_e21375 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign15070_body25_e21381 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign15070_body25_e21380) + (assign15070_body25_e21375 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign15070_body25_e21381 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body25_e21375 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign15070_body25_e21381 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body25_e21375 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign15070_body25_e21381 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body25_e21375 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign15070_body25_e21381 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15070_body25_e21385;
            locals.var_fb_dpss_dn0 = assign15070_body25_e21385_d_n0;
            locals.var_fb_dpss_dn2 = assign15070_body25_e21385_d_n2;
            locals.var_fb_dpss_dn6 = assign15070_body25_e21385_d_n6;
            locals.var_fb_dpss_dn7 = assign15070_body25_e21385_d_n7;
            locals.var_fb_dpss_dn10 = assign15070_body25_e21385_d_n10;
            locals.var_fb_dpss_dn11 = assign15070_body25_e21385_d_n11;
            locals.var_fb_dpss_dn12 = assign15070_body25_e21385_d_n12;
            locals.var_fb_dpss_dn17 = assign15070_body25_e21385_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign15070_body26_e21398, assign15070_body26_e21398_d_n0, assign15070_body26_e21398_d_n2, assign15070_body26_e21398_d_n6, assign15070_body26_e21398_d_n7, assign15070_body26_e21398_d_n10, assign15070_body26_e21398_d_n11, assign15070_body26_e21398_d_n12, assign15070_body26_e21398_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 == 0.0)) {
        let assign15070_body26_e21395: f64 = (-locals.var_chi);
        let assign15070_body26_e21396: f64 = (assign15070_body26_e21395).exp();
        (assign15070_body26_e21396, (assign15070_body26_e21396 * (-locals.var_chi_dn0)), (assign15070_body26_e21396 * (-locals.var_chi_dn2)), (assign15070_body26_e21396 * (-locals.var_chi_dn6)), (assign15070_body26_e21396 * (-locals.var_chi_dn7)), (assign15070_body26_e21396 * (-locals.var_chi_dn10)), (assign15070_body26_e21396 * (-locals.var_chi_dn11)), (assign15070_body26_e21396 * (-locals.var_chi_dn12)), (assign15070_body26_e21396 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign15070_body26_e21398;
            locals.var_t0_dn0 = assign15070_body26_e21398_d_n0;
            locals.var_t0_dn2 = assign15070_body26_e21398_d_n2;
            locals.var_t0_dn6 = assign15070_body26_e21398_d_n6;
            locals.var_t0_dn7 = assign15070_body26_e21398_d_n7;
            locals.var_t0_dn10 = assign15070_body26_e21398_d_n10;
            locals.var_t0_dn11 = assign15070_body26_e21398_d_n11;
            locals.var_t0_dn12 = assign15070_body26_e21398_d_n12;
            locals.var_t0_dn17 = assign15070_body26_e21398_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign15070_body27_e21411, assign15070_body27_e21411_d_n0, assign15070_body27_e21411_d_n2, assign15070_body27_e21411_d_n6, assign15070_body27_e21411_d_n7, assign15070_body27_e21411_d_n10, assign15070_body27_e21411_d_n11, assign15070_body27_e21411_d_n12, assign15070_body27_e21411_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 == 0.0)) {
        let assign15070_body27_e21408: f64 = (-locals.var_chib);
        let assign15070_body27_e21409: f64 = (assign15070_body27_e21408).exp();
        (assign15070_body27_e21409, (assign15070_body27_e21409 * (-locals.var_chib_dn0)), (assign15070_body27_e21409 * (-locals.var_chib_dn2)), (assign15070_body27_e21409 * (-locals.var_chib_dn6)), (assign15070_body27_e21409 * (-locals.var_chib_dn7)), (assign15070_body27_e21409 * (-locals.var_chib_dn10)), (assign15070_body27_e21409 * (-locals.var_chib_dn11)), (assign15070_body27_e21409 * (-locals.var_chib_dn12)), (assign15070_body27_e21409 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign15070_body27_e21411;
            locals.var_t1_dn0 = assign15070_body27_e21411_d_n0;
            locals.var_t1_dn2 = assign15070_body27_e21411_d_n2;
            locals.var_t1_dn6 = assign15070_body27_e21411_d_n6;
            locals.var_t1_dn7 = assign15070_body27_e21411_d_n7;
            locals.var_t1_dn10 = assign15070_body27_e21411_d_n10;
            locals.var_t1_dn11 = assign15070_body27_e21411_d_n11;
            locals.var_t1_dn12 = assign15070_body27_e21411_d_n12;
            locals.var_t1_dn17 = assign15070_body27_e21411_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign15070_body28_e21429, assign15070_body28_e21429_d_n0, assign15070_body28_e21429_d_n2, assign15070_body28_e21429_d_n6, assign15070_body28_e21429_d_n7, assign15070_body28_e21429_d_n10, assign15070_body28_e21429_d_n11, assign15070_body28_e21429_d_n12, assign15070_body28_e21429_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 == 0.0)) {
        let assign15070_body28_e21422: f64 = (locals.var_chi - locals.var_chib);
        let assign15070_body28_e21425: f64 = (locals.var_t0 - locals.var_t1);
        let assign15070_body28_e21426: f64 = (assign15070_body28_e21422 + assign15070_body28_e21425);
        let assign15070_body28_e21427: f64 = (assign15070_body28_e21426).sqrt();
        (assign15070_body28_e21427, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign15070_body28_e21427)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign15070_body28_e21427)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign15070_body28_e21427)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign15070_body28_e21427)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign15070_body28_e21427)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign15070_body28_e21427)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign15070_body28_e21427)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign15070_body28_e21427)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign15070_body28_e21429;
            locals.var_fb_dn0 = assign15070_body28_e21429_d_n0;
            locals.var_fb_dn2 = assign15070_body28_e21429_d_n2;
            locals.var_fb_dn6 = assign15070_body28_e21429_d_n6;
            locals.var_fb_dn7 = assign15070_body28_e21429_d_n7;
            locals.var_fb_dn10 = assign15070_body28_e21429_d_n10;
            locals.var_fb_dn11 = assign15070_body28_e21429_d_n11;
            locals.var_fb_dn12 = assign15070_body28_e21429_d_n12;
            locals.var_fb_dn17 = assign15070_body28_e21429_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign15070_body29_e21454, assign15070_body29_e21454_d_n0, assign15070_body29_e21454_d_n2, assign15070_body29_e21454_d_n6, assign15070_body29_e21454_d_n7, assign15070_body29_e21454_d_n10, assign15070_body29_e21454_d_n11, assign15070_body29_e21454_d_n12, assign15070_body29_e21454_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard452 == 0.0)) && (locals.var_guard454 == 0.0)) {
        let assign15070_body29_e21440: f64 = (locals.var_beta * 0.5);
        let assign15070_body29_e21443: f64 = (1.0 - locals.var_t0);
        let assign15070_body29_e21447: f64 = (1.0 - locals.var_t1);
        let assign15070_body29_e21448: f64 = (locals.var_phi_soib_dpss * assign15070_body29_e21447);
        let assign15070_body29_e21449: f64 = (assign15070_body29_e21443 - assign15070_body29_e21448);
        let assign15070_body29_e21450: f64 = (assign15070_body29_e21440 * assign15070_body29_e21449);
        let assign15070_body29_e21452: f64 = (assign15070_body29_e21450 / locals.var_fb);
        (assign15070_body29_e21452, ((((assign15070_body29_e21440 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign15070_body29_e21447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign15070_body29_e21450 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body29_e21440 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign15070_body29_e21447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign15070_body29_e21450 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body29_e21440 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign15070_body29_e21447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign15070_body29_e21450 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body29_e21440 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign15070_body29_e21447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign15070_body29_e21450 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign15070_body29_e21449) + (assign15070_body29_e21440 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign15070_body29_e21447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign15070_body29_e21450 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body29_e21440 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign15070_body29_e21447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign15070_body29_e21450 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body29_e21440 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign15070_body29_e21447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign15070_body29_e21450 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign15070_body29_e21440 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign15070_body29_e21447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign15070_body29_e21450 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign15070_body29_e21454;
            locals.var_fb_dpss_dn0 = assign15070_body29_e21454_d_n0;
            locals.var_fb_dpss_dn2 = assign15070_body29_e21454_d_n2;
            locals.var_fb_dpss_dn6 = assign15070_body29_e21454_d_n6;
            locals.var_fb_dpss_dn7 = assign15070_body29_e21454_d_n7;
            locals.var_fb_dpss_dn10 = assign15070_body29_e21454_d_n10;
            locals.var_fb_dpss_dn11 = assign15070_body29_e21454_d_n11;
            locals.var_fb_dpss_dn12 = assign15070_body29_e21454_d_n12;
            locals.var_fb_dpss_dn17 = assign15070_body29_e21454_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign15070_body30_e21461: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard455 = assign15070_body30_e21461;
            locals.var_guard455_rv = 0.0;
            let (assign15070_body31_e21469,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard455 != 0.0)) {
        let assign15070_body31_e21467: f64 = (-1.0);
        (assign15070_body31_e21467,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign15070_body31_e21469;
            locals.var_flg_zone_rv = 0.0;
            let assign15070_body32_e21472: f64 = (-1.0);
            let assign15070_body32_e21473: f64 = if locals.var_flg_zone == assign15070_body32_e21472 { 1.0 } else { 0.0 };
            locals.var_guard456 = assign15070_body32_e21473;
            locals.var_guard456_rv = 0.0;
            let (assign15070_body33_e21480, assign15070_body33_e21480_d_n0, assign15070_body33_e21480_d_n2, assign15070_body33_e21480_d_n6, assign15070_body33_e21480_d_n7, assign15070_body33_e21480_d_n10, assign15070_body33_e21480_d_n11, assign15070_body33_e21480_d_n12, assign15070_body33_e21480_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard456 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign15070_body33_e21480;
            locals.var_wdsoi_dn0 = assign15070_body33_e21480_d_n0;
            locals.var_wdsoi_dn2 = assign15070_body33_e21480_d_n2;
            locals.var_wdsoi_dn6 = assign15070_body33_e21480_d_n6;
            locals.var_wdsoi_dn7 = assign15070_body33_e21480_d_n7;
            locals.var_wdsoi_dn10 = assign15070_body33_e21480_d_n10;
            locals.var_wdsoi_dn11 = assign15070_body33_e21480_d_n11;
            locals.var_wdsoi_dn12 = assign15070_body33_e21480_d_n12;
            locals.var_wdsoi_dn17 = assign15070_body33_e21480_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign15070_body34_e21490, assign15070_body34_e21490_d_n0, assign15070_body34_e21490_d_n2, assign15070_body34_e21490_d_n6, assign15070_body34_e21490_d_n7, assign15070_body34_e21490_d_n10, assign15070_body34_e21490_d_n11, assign15070_body34_e21490_d_n12, assign15070_body34_e21490_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard456 == 0.0)) {
        let assign15070_body34_e21488: f64 = (locals.var_c_w_soi * locals.var_fb);
        (assign15070_body34_e21488, ((locals.var_c_w_soi_dn0 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn0)), ((locals.var_c_w_soi_dn2 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn2)), ((locals.var_c_w_soi_dn6 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn6)), ((locals.var_c_w_soi_dn7 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn7)), ((locals.var_c_w_soi_dn10 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn10)), ((locals.var_c_w_soi_dn11 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn11)), ((locals.var_c_w_soi_dn12 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn12)), ((locals.var_c_w_soi_dn17 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn17)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign15070_body34_e21490;
            locals.var_wdsoi_dn0 = assign15070_body34_e21490_d_n0;
            locals.var_wdsoi_dn2 = assign15070_body34_e21490_d_n2;
            locals.var_wdsoi_dn6 = assign15070_body34_e21490_d_n6;
            locals.var_wdsoi_dn7 = assign15070_body34_e21490_d_n7;
            locals.var_wdsoi_dn10 = assign15070_body34_e21490_d_n10;
            locals.var_wdsoi_dn11 = assign15070_body34_e21490_d_n11;
            locals.var_wdsoi_dn12 = assign15070_body34_e21490_d_n12;
            locals.var_wdsoi_dn17 = assign15070_body34_e21490_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let assign15070_body35_e21494: f64 = (p.p237 * 1.01);
            let assign15070_body35_e21495: f64 = if locals.var_wdsoi < assign15070_body35_e21494 { 1.0 } else { 0.0 };
            locals.var_guard457 = assign15070_body35_e21495;
            locals.var_guard457_rv = 0.0;
            let (assign15070_body36_e21502,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard457 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
            locals.var_flg_depmode = assign15070_body36_e21502;
            locals.var_flg_depmode_rv = 0.0;
            let (assign15070_body37_e21510,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard457 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
            locals.var_flg_depmode = assign15070_body37_e21510;
            locals.var_flg_depmode_rv = 0.0;
            let (assign15070_body38_e21517, assign15070_body38_e21517_d_n0, assign15070_body38_e21517_d_n2, assign15070_body38_e21517_d_n6, assign15070_body38_e21517_d_n7, assign15070_body38_e21517_d_n10, assign15070_body38_e21517_d_n11, assign15070_body38_e21517_d_n12, assign15070_body38_e21517_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15070_body38_e21515: f64 = (locals.var_q_nsub * locals.var_wdsoi);
        (assign15070_body38_e21515, ((locals.var_q_nsub_dn0 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn0)), ((locals.var_q_nsub_dn2 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn2)), ((locals.var_q_nsub_dn6 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn6)), ((locals.var_q_nsub_dn7 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn7)), ((locals.var_q_nsub_dn10 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn10)), ((locals.var_q_nsub_dn11 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn11)), ((locals.var_q_nsub_dn12 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn12)), ((locals.var_q_nsub_dn17 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn17)),)
    } else {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    }
};
            locals.var_q_dep_soi = assign15070_body38_e21517;
            locals.var_q_dep_soi_dn0 = assign15070_body38_e21517_d_n0;
            locals.var_q_dep_soi_dn2 = assign15070_body38_e21517_d_n2;
            locals.var_q_dep_soi_dn6 = assign15070_body38_e21517_d_n6;
            locals.var_q_dep_soi_dn7 = assign15070_body38_e21517_d_n7;
            locals.var_q_dep_soi_dn10 = assign15070_body38_e21517_d_n10;
            locals.var_q_dep_soi_dn11 = assign15070_body38_e21517_d_n11;
            locals.var_q_dep_soi_dn12 = assign15070_body38_e21517_d_n12;
            locals.var_q_dep_soi_dn17 = assign15070_body38_e21517_d_n17;
            locals.var_q_dep_soi_rv = 0.0;
            let assign15070_body39_e21520: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard458 = assign15070_body39_e21520;
            locals.var_guard458_rv = 0.0;
            let (assign15070_body40_e21528, assign15070_body40_e21528_d_n0, assign15070_body40_e21528_d_n2, assign15070_body40_e21528_d_n6, assign15070_body40_e21528_d_n7, assign15070_body40_e21528_d_n10, assign15070_body40_e21528_d_n11, assign15070_body40_e21528_d_n12, assign15070_body40_e21528_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard458 != 0.0)) {
        let assign15070_body40_e21526: f64 = (-locals.var_fb);
        (assign15070_body40_e21526, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15070_body40_e21528;
            locals.var_fs02_dn0 = assign15070_body40_e21528_d_n0;
            locals.var_fs02_dn2 = assign15070_body40_e21528_d_n2;
            locals.var_fs02_dn6 = assign15070_body40_e21528_d_n6;
            locals.var_fs02_dn7 = assign15070_body40_e21528_d_n7;
            locals.var_fs02_dn10 = assign15070_body40_e21528_d_n10;
            locals.var_fs02_dn11 = assign15070_body40_e21528_d_n11;
            locals.var_fs02_dn12 = assign15070_body40_e21528_d_n12;
            locals.var_fs02_dn17 = assign15070_body40_e21528_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15070_body41_e21536, assign15070_body41_e21536_d_n0, assign15070_body41_e21536_d_n2, assign15070_body41_e21536_d_n6, assign15070_body41_e21536_d_n7, assign15070_body41_e21536_d_n10, assign15070_body41_e21536_d_n11, assign15070_body41_e21536_d_n12, assign15070_body41_e21536_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard458 != 0.0)) {
        let assign15070_body41_e21534: f64 = (-locals.var_fb_dpss);
        (assign15070_body41_e21534, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15070_body41_e21536;
            locals.var_fs02_dps0_dn0 = assign15070_body41_e21536_d_n0;
            locals.var_fs02_dps0_dn2 = assign15070_body41_e21536_d_n2;
            locals.var_fs02_dps0_dn6 = assign15070_body41_e21536_d_n6;
            locals.var_fs02_dps0_dn7 = assign15070_body41_e21536_d_n7;
            locals.var_fs02_dps0_dn10 = assign15070_body41_e21536_d_n10;
            locals.var_fs02_dps0_dn11 = assign15070_body41_e21536_d_n11;
            locals.var_fs02_dps0_dn12 = assign15070_body41_e21536_d_n12;
            locals.var_fs02_dps0_dn17 = assign15070_body41_e21536_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign15070_body42_e21539: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard459 = assign15070_body42_e21539;
            locals.var_guard459_rv = 0.0;
            let (assign15070_body43_e21549, assign15070_body43_e21549_d_n0, assign15070_body43_e21549_d_n2, assign15070_body43_e21549_d_n6, assign15070_body43_e21549_d_n7, assign15070_body43_e21549_d_n10, assign15070_body43_e21549_d_n11, assign15070_body43_e21549_d_n12, assign15070_body43_e21549_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15070_body43_e21549;
            locals.var_fs02_dn0 = assign15070_body43_e21549_d_n0;
            locals.var_fs02_dn2 = assign15070_body43_e21549_d_n2;
            locals.var_fs02_dn6 = assign15070_body43_e21549_d_n6;
            locals.var_fs02_dn7 = assign15070_body43_e21549_d_n7;
            locals.var_fs02_dn10 = assign15070_body43_e21549_d_n10;
            locals.var_fs02_dn11 = assign15070_body43_e21549_d_n11;
            locals.var_fs02_dn12 = assign15070_body43_e21549_d_n12;
            locals.var_fs02_dn17 = assign15070_body43_e21549_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15070_body44_e21559, assign15070_body44_e21559_d_n0, assign15070_body44_e21559_d_n2, assign15070_body44_e21559_d_n6, assign15070_body44_e21559_d_n7, assign15070_body44_e21559_d_n10, assign15070_body44_e21559_d_n11, assign15070_body44_e21559_d_n12, assign15070_body44_e21559_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15070_body44_e21559;
            locals.var_fs02_dps0_dn0 = assign15070_body44_e21559_d_n0;
            locals.var_fs02_dps0_dn2 = assign15070_body44_e21559_d_n2;
            locals.var_fs02_dps0_dn6 = assign15070_body44_e21559_d_n6;
            locals.var_fs02_dps0_dn7 = assign15070_body44_e21559_d_n7;
            locals.var_fs02_dps0_dn10 = assign15070_body44_e21559_d_n10;
            locals.var_fs02_dps0_dn11 = assign15070_body44_e21559_d_n11;
            locals.var_fs02_dps0_dn12 = assign15070_body44_e21559_d_n12;
            locals.var_fs02_dps0_dn17 = assign15070_body44_e21559_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let assign15070_body45_e21562: f64 = if locals.var_chi < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard460 = assign15070_body45_e21562;
            locals.var_guard460_rv = 0.0;
            let (assign15070_body46_e21576, assign15070_body46_e21576_d_n0, assign15070_body46_e21576_d_n2, assign15070_body46_e21576_d_n6, assign15070_body46_e21576_d_n7, assign15070_body46_e21576_d_n10, assign15070_body46_e21576_d_n11, assign15070_body46_e21576_d_n12, assign15070_body46_e21576_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 != 0.0)) {
        let assign15070_body46_e21574: f64 = (locals.var_chi).exp();
        (assign15070_body46_e21574, (assign15070_body46_e21574 * locals.var_chi_dn0), (assign15070_body46_e21574 * locals.var_chi_dn2), (assign15070_body46_e21574 * locals.var_chi_dn6), (assign15070_body46_e21574 * locals.var_chi_dn7), (assign15070_body46_e21574 * locals.var_chi_dn10), (assign15070_body46_e21574 * locals.var_chi_dn11), (assign15070_body46_e21574 * locals.var_chi_dn12), (assign15070_body46_e21574 * locals.var_chi_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign15070_body46_e21576;
            locals.var_exp_chi_dn0 = assign15070_body46_e21576_d_n0;
            locals.var_exp_chi_dn2 = assign15070_body46_e21576_d_n2;
            locals.var_exp_chi_dn6 = assign15070_body46_e21576_d_n6;
            locals.var_exp_chi_dn7 = assign15070_body46_e21576_d_n7;
            locals.var_exp_chi_dn10 = assign15070_body46_e21576_d_n10;
            locals.var_exp_chi_dn11 = assign15070_body46_e21576_d_n11;
            locals.var_exp_chi_dn12 = assign15070_body46_e21576_d_n12;
            locals.var_exp_chi_dn17 = assign15070_body46_e21576_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign15070_body47_e21595, assign15070_body47_e21595_d_n0, assign15070_body47_e21595_d_n2, assign15070_body47_e21595_d_n6, assign15070_body47_e21595_d_n7, assign15070_body47_e21595_d_n10, assign15070_body47_e21595_d_n11, assign15070_body47_e21595_d_n12, assign15070_body47_e21595_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 != 0.0)) {
        let assign15070_body47_e21591: f64 = (locals.var_chi + 1.0);
        let assign15070_body47_e21592: f64 = (locals.var_exp_chi - assign15070_body47_e21591);
        let assign15070_body47_e21593: f64 = (locals.var_cfs1 * assign15070_body47_e21592);
        (assign15070_body47_e21593, ((locals.var_cfs1_dn0 * assign15070_body47_e21592) + (locals.var_cfs1 * (locals.var_exp_chi_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign15070_body47_e21592) + (locals.var_cfs1 * (locals.var_exp_chi_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn6 * assign15070_body47_e21592) + (locals.var_cfs1 * (locals.var_exp_chi_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign15070_body47_e21592) + (locals.var_cfs1 * (locals.var_exp_chi_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn10 * assign15070_body47_e21592) + (locals.var_cfs1 * (locals.var_exp_chi_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign15070_body47_e21592) + (locals.var_cfs1 * (locals.var_exp_chi_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn12 * assign15070_body47_e21592) + (locals.var_cfs1 * (locals.var_exp_chi_dn12 - locals.var_chi_dn12))), ((locals.var_cfs1_dn17 * assign15070_body47_e21592) + (locals.var_cfs1 * (locals.var_exp_chi_dn17 - locals.var_chi_dn17))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign15070_body47_e21595;
            locals.var_fs01_dn0 = assign15070_body47_e21595_d_n0;
            locals.var_fs01_dn2 = assign15070_body47_e21595_d_n2;
            locals.var_fs01_dn6 = assign15070_body47_e21595_d_n6;
            locals.var_fs01_dn7 = assign15070_body47_e21595_d_n7;
            locals.var_fs01_dn10 = assign15070_body47_e21595_d_n10;
            locals.var_fs01_dn11 = assign15070_body47_e21595_d_n11;
            locals.var_fs01_dn12 = assign15070_body47_e21595_d_n12;
            locals.var_fs01_dn17 = assign15070_body47_e21595_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign15070_body48_e21614, assign15070_body48_e21614_d_n0, assign15070_body48_e21614_d_n2, assign15070_body48_e21614_d_n6, assign15070_body48_e21614_d_n7, assign15070_body48_e21614_d_n10, assign15070_body48_e21614_d_n11, assign15070_body48_e21614_d_n12, assign15070_body48_e21614_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 != 0.0)) {
        let assign15070_body48_e21608: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign15070_body48_e21611: f64 = (locals.var_exp_chi - 1.0);
        let assign15070_body48_e21612: f64 = (assign15070_body48_e21608 * assign15070_body48_e21611);
        (assign15070_body48_e21612, (((locals.var_cfs1_dn0 * locals.var_beta) * assign15070_body48_e21611) + (assign15070_body48_e21608 * locals.var_exp_chi_dn0)), (((locals.var_cfs1_dn2 * locals.var_beta) * assign15070_body48_e21611) + (assign15070_body48_e21608 * locals.var_exp_chi_dn2)), (((locals.var_cfs1_dn6 * locals.var_beta) * assign15070_body48_e21611) + (assign15070_body48_e21608 * locals.var_exp_chi_dn6)), (((locals.var_cfs1_dn7 * locals.var_beta) * assign15070_body48_e21611) + (assign15070_body48_e21608 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * assign15070_body48_e21611) + (assign15070_body48_e21608 * locals.var_exp_chi_dn10)), (((locals.var_cfs1_dn11 * locals.var_beta) * assign15070_body48_e21611) + (assign15070_body48_e21608 * locals.var_exp_chi_dn11)), (((locals.var_cfs1_dn12 * locals.var_beta) * assign15070_body48_e21611) + (assign15070_body48_e21608 * locals.var_exp_chi_dn12)), (((locals.var_cfs1_dn17 * locals.var_beta) * assign15070_body48_e21611) + (assign15070_body48_e21608 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign15070_body48_e21614;
            locals.var_fs01_dps0_dn0 = assign15070_body48_e21614_d_n0;
            locals.var_fs01_dps0_dn2 = assign15070_body48_e21614_d_n2;
            locals.var_fs01_dps0_dn6 = assign15070_body48_e21614_d_n6;
            locals.var_fs01_dps0_dn7 = assign15070_body48_e21614_d_n7;
            locals.var_fs01_dps0_dn10 = assign15070_body48_e21614_d_n10;
            locals.var_fs01_dps0_dn11 = assign15070_body48_e21614_d_n11;
            locals.var_fs01_dps0_dn12 = assign15070_body48_e21614_d_n12;
            locals.var_fs01_dps0_dn17 = assign15070_body48_e21614_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign15070_body49_e21631, assign15070_body49_e21631_d_n0, assign15070_body49_e21631_d_n2, assign15070_body49_e21631_d_n6, assign15070_body49_e21631_d_n7, assign15070_body49_e21631_d_n10, assign15070_body49_e21631_d_n11, assign15070_body49_e21631_d_n12, assign15070_body49_e21631_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign15070_body49_e21628: f64 = (locals.var_beta * locals.var_phi_s0_soi);
        let assign15070_body49_e21629: f64 = (assign15070_body49_e21628).exp();
        (assign15070_body49_e21629, (assign15070_body49_e21629 * (locals.var_beta * locals.var_phi_s0_soi_dn0)), (assign15070_body49_e21629 * (locals.var_beta * locals.var_phi_s0_soi_dn2)), (assign15070_body49_e21629 * (locals.var_beta * locals.var_phi_s0_soi_dn6)), (assign15070_body49_e21629 * (locals.var_beta * locals.var_phi_s0_soi_dn7)), (assign15070_body49_e21629 * ((locals.var_beta_dn10 * locals.var_phi_s0_soi) + (locals.var_beta * locals.var_phi_s0_soi_dn10))), (assign15070_body49_e21629 * (locals.var_beta * locals.var_phi_s0_soi_dn11)), (assign15070_body49_e21629 * (locals.var_beta * locals.var_phi_s0_soi_dn12)), (assign15070_body49_e21629 * (locals.var_beta * locals.var_phi_s0_soi_dn17)),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn12, locals.var_exp_bps0_dn17,)
    }
};
            locals.var_exp_bps0 = assign15070_body49_e21631;
            locals.var_exp_bps0_dn0 = assign15070_body49_e21631_d_n0;
            locals.var_exp_bps0_dn2 = assign15070_body49_e21631_d_n2;
            locals.var_exp_bps0_dn6 = assign15070_body49_e21631_d_n6;
            locals.var_exp_bps0_dn7 = assign15070_body49_e21631_d_n7;
            locals.var_exp_bps0_dn10 = assign15070_body49_e21631_d_n10;
            locals.var_exp_bps0_dn11 = assign15070_body49_e21631_d_n11;
            locals.var_exp_bps0_dn12 = assign15070_body49_e21631_d_n12;
            locals.var_exp_bps0_dn17 = assign15070_body49_e21631_d_n17;
            locals.var_exp_bps0_rv = 0.0;
            let (assign15070_body50_e21653, assign15070_body50_e21653_d_n0, assign15070_body50_e21653_d_n2, assign15070_body50_e21653_d_n6, assign15070_body50_e21653_d_n7, assign15070_body50_e21653_d_n10, assign15070_body50_e21653_d_n11, assign15070_body50_e21653_d_n12, assign15070_body50_e21653_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign15070_body50_e21648: f64 = (locals.var_chi + 1.0);
        let assign15070_body50_e21649: f64 = (locals.var_exp_bvbs * assign15070_body50_e21648);
        let assign15070_body50_e21650: f64 = (locals.var_exp_bps0 - assign15070_body50_e21649);
        let assign15070_body50_e21651: f64 = (locals.var_cnst1soi * assign15070_body50_e21650);
        (assign15070_body50_e21651, ((locals.var_cnst1soi_dn0 * assign15070_body50_e21650) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign15070_body50_e21648) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign15070_body50_e21650) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign15070_body50_e21648) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign15070_body50_e21650) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign15070_body50_e21648) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign15070_body50_e21650) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign15070_body50_e21648) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign15070_body50_e21650) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign15070_body50_e21648) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign15070_body50_e21650) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign15070_body50_e21648) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign15070_body50_e21650) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn12 - ((locals.var_exp_bvbs_dn12 * assign15070_body50_e21648) + (locals.var_exp_bvbs * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign15070_body50_e21650) + (locals.var_cnst1soi * (locals.var_exp_bps0_dn17 - ((locals.var_exp_bvbs_dn17 * assign15070_body50_e21648) + (locals.var_exp_bvbs * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, locals.var_fs01_dn17,)
    }
};
            locals.var_fs01 = assign15070_body50_e21653;
            locals.var_fs01_dn0 = assign15070_body50_e21653_d_n0;
            locals.var_fs01_dn2 = assign15070_body50_e21653_d_n2;
            locals.var_fs01_dn6 = assign15070_body50_e21653_d_n6;
            locals.var_fs01_dn7 = assign15070_body50_e21653_d_n7;
            locals.var_fs01_dn10 = assign15070_body50_e21653_d_n10;
            locals.var_fs01_dn11 = assign15070_body50_e21653_d_n11;
            locals.var_fs01_dn12 = assign15070_body50_e21653_d_n12;
            locals.var_fs01_dn17 = assign15070_body50_e21653_d_n17;
            locals.var_fs01_rv = 0.0;
            let (assign15070_body51_e21673, assign15070_body51_e21673_d_n0, assign15070_body51_e21673_d_n2, assign15070_body51_e21673_d_n6, assign15070_body51_e21673_d_n7, assign15070_body51_e21673_d_n10, assign15070_body51_e21673_d_n11, assign15070_body51_e21673_d_n12, assign15070_body51_e21673_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 == 0.0)) && (locals.var_guard460 == 0.0)) {
        let assign15070_body51_e21667: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign15070_body51_e21670: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign15070_body51_e21671: f64 = (assign15070_body51_e21667 * assign15070_body51_e21670);
        (assign15070_body51_e21671, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign15070_body51_e21670) + (assign15070_body51_e21667 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign15070_body51_e21670) + (assign15070_body51_e21667 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign15070_body51_e21670) + (assign15070_body51_e21667 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign15070_body51_e21670) + (assign15070_body51_e21667 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign15070_body51_e21670) + (assign15070_body51_e21667 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign15070_body51_e21670) + (assign15070_body51_e21667 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign15070_body51_e21670) + (assign15070_body51_e21667 * (locals.var_exp_bps0_dn12 - locals.var_exp_bvbs_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign15070_body51_e21670) + (assign15070_body51_e21667 * (locals.var_exp_bps0_dn17 - locals.var_exp_bvbs_dn17))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, locals.var_fs01_dps0_dn17,)
    }
};
            locals.var_fs01_dps0 = assign15070_body51_e21673;
            locals.var_fs01_dps0_dn0 = assign15070_body51_e21673_d_n0;
            locals.var_fs01_dps0_dn2 = assign15070_body51_e21673_d_n2;
            locals.var_fs01_dps0_dn6 = assign15070_body51_e21673_d_n6;
            locals.var_fs01_dps0_dn7 = assign15070_body51_e21673_d_n7;
            locals.var_fs01_dps0_dn10 = assign15070_body51_e21673_d_n10;
            locals.var_fs01_dps0_dn11 = assign15070_body51_e21673_d_n11;
            locals.var_fs01_dps0_dn12 = assign15070_body51_e21673_d_n12;
            locals.var_fs01_dps0_dn17 = assign15070_body51_e21673_d_n17;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign15070_body52_e21689, assign15070_body52_e21689_d_n0, assign15070_body52_e21689_d_n2, assign15070_body52_e21689_d_n6, assign15070_body52_e21689_d_n7, assign15070_body52_e21689_d_n10, assign15070_body52_e21689_d_n11, assign15070_body52_e21689_d_n12, assign15070_body52_e21689_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 == 0.0)) {
        let assign15070_body52_e21684: f64 = (locals.var_fb * locals.var_fb);
        let assign15070_body52_e21686: f64 = (assign15070_body52_e21684 + locals.var_fs01);
        let assign15070_body52_e21687: f64 = (assign15070_body52_e21686).sqrt();
        (assign15070_body52_e21687, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign15070_body52_e21687)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign15070_body52_e21687)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign15070_body52_e21687)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign15070_body52_e21687)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign15070_body52_e21687)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign15070_body52_e21687)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign15070_body52_e21687)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fs01_dn17) / (2.0 * assign15070_body52_e21687)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, locals.var_fs02_dn17,)
    }
};
            locals.var_fs02 = assign15070_body52_e21689;
            locals.var_fs02_dn0 = assign15070_body52_e21689_d_n0;
            locals.var_fs02_dn2 = assign15070_body52_e21689_d_n2;
            locals.var_fs02_dn6 = assign15070_body52_e21689_d_n6;
            locals.var_fs02_dn7 = assign15070_body52_e21689_d_n7;
            locals.var_fs02_dn10 = assign15070_body52_e21689_d_n10;
            locals.var_fs02_dn11 = assign15070_body52_e21689_d_n11;
            locals.var_fs02_dn12 = assign15070_body52_e21689_d_n12;
            locals.var_fs02_dn17 = assign15070_body52_e21689_d_n17;
            locals.var_fs02_rv = 0.0;
            let (assign15070_body53_e21710, assign15070_body53_e21710_d_n0, assign15070_body53_e21710_d_n2, assign15070_body53_e21710_d_n6, assign15070_body53_e21710_d_n7, assign15070_body53_e21710_d_n10, assign15070_body53_e21710_d_n11, assign15070_body53_e21710_d_n12, assign15070_body53_e21710_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard458 == 0.0)) && (locals.var_guard459 == 0.0)) {
        let assign15070_body53_e21701: f64 = (2.0 * locals.var_fb_dpss);
        let assign15070_body53_e21703: f64 = (assign15070_body53_e21701 * locals.var_fb);
        let assign15070_body53_e21705: f64 = (assign15070_body53_e21703 + locals.var_fs01_dps0);
        let assign15070_body53_e21706: f64 = (0.5 * assign15070_body53_e21705);
        let assign15070_body53_e21708: f64 = (assign15070_body53_e21706 / locals.var_fs02);
        (assign15070_body53_e21708, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign15070_body53_e21701 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign15070_body53_e21706 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign15070_body53_e21701 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign15070_body53_e21706 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign15070_body53_e21701 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign15070_body53_e21706 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign15070_body53_e21701 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign15070_body53_e21706 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign15070_body53_e21701 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign15070_body53_e21706 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign15070_body53_e21701 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign15070_body53_e21706 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign15070_body53_e21701 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12)) * locals.var_fs02) - (assign15070_body53_e21706 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign15070_body53_e21701 * locals.var_fb_dn17)) + locals.var_fs01_dps0_dn17)) * locals.var_fs02) - (assign15070_body53_e21706 * locals.var_fs02_dn17)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, locals.var_fs02_dps0_dn17,)
    }
};
            locals.var_fs02_dps0 = assign15070_body53_e21710;
            locals.var_fs02_dps0_dn0 = assign15070_body53_e21710_d_n0;
            locals.var_fs02_dps0_dn2 = assign15070_body53_e21710_d_n2;
            locals.var_fs02_dps0_dn6 = assign15070_body53_e21710_d_n6;
            locals.var_fs02_dps0_dn7 = assign15070_body53_e21710_d_n7;
            locals.var_fs02_dps0_dn10 = assign15070_body53_e21710_d_n10;
            locals.var_fs02_dps0_dn11 = assign15070_body53_e21710_d_n11;
            locals.var_fs02_dps0_dn12 = assign15070_body53_e21710_d_n12;
            locals.var_fs02_dps0_dn17 = assign15070_body53_e21710_d_n17;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign15070_body54_e21726, assign15070_body54_e21726_d_n0, assign15070_body54_e21726_d_n2, assign15070_body54_e21726_d_n6, assign15070_body54_e21726_d_n7, assign15070_body54_e21726_d_n10, assign15070_body54_e21726_d_n11, assign15070_body54_e21726_d_n12, assign15070_body54_e21726_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15070_body54_e21714: f64 = (-locals.var_vgp);
        let assign15070_body54_e21716: f64 = (assign15070_body54_e21714 + locals.var_phi_s0_soi);
        let assign15070_body54_e21719: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign15070_body54_e21720: f64 = (assign15070_body54_e21716 + assign15070_body54_e21719);
        let assign15070_body54_e21723: f64 = (locals.var_c_fox_inv * locals.var_qhs);
        let assign15070_body54_e21724: f64 = (assign15070_body54_e21720 - assign15070_body54_e21723);
        (assign15070_body54_e21724, ((((-locals.var_vgp_dn0) + locals.var_phi_s0_soi_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))) - ((locals.var_c_fox_inv_dn0 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn0))), ((((-locals.var_vgp_dn2) + locals.var_phi_s0_soi_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))) - ((locals.var_c_fox_inv_dn2 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn2))), ((((-locals.var_vgp_dn6) + locals.var_phi_s0_soi_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))) - ((locals.var_c_fox_inv_dn6 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn6))), ((((-locals.var_vgp_dn7) + locals.var_phi_s0_soi_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))) - ((locals.var_c_fox_inv_dn7 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn7))), ((((-locals.var_vgp_dn10) + locals.var_phi_s0_soi_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))) - ((locals.var_c_fox_inv_dn10 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn10))), ((((-locals.var_vgp_dn11) + locals.var_phi_s0_soi_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))) - ((locals.var_c_fox_inv_dn11 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn11))), ((((-locals.var_vgp_dn12) + locals.var_phi_s0_soi_dn12) + ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))) - ((locals.var_c_fox_inv_dn12 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn12))), ((((-locals.var_vgp_dn17) + locals.var_phi_s0_soi_dn17) + ((locals.var_fac1_dn17 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn17))) - ((locals.var_c_fox_inv_dn17 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn17))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12, locals.var_fs0_dn17,)
    }
};
            locals.var_fs0 = assign15070_body54_e21726;
            locals.var_fs0_dn0 = assign15070_body54_e21726_d_n0;
            locals.var_fs0_dn2 = assign15070_body54_e21726_d_n2;
            locals.var_fs0_dn6 = assign15070_body54_e21726_d_n6;
            locals.var_fs0_dn7 = assign15070_body54_e21726_d_n7;
            locals.var_fs0_dn10 = assign15070_body54_e21726_d_n10;
            locals.var_fs0_dn11 = assign15070_body54_e21726_d_n11;
            locals.var_fs0_dn12 = assign15070_body54_e21726_d_n12;
            locals.var_fs0_dn17 = assign15070_body54_e21726_d_n17;
            locals.var_fs0_rv = 0.0;
            let (assign15070_body55_e21735, assign15070_body55_e21735_d_n0, assign15070_body55_e21735_d_n2, assign15070_body55_e21735_d_n6, assign15070_body55_e21735_d_n7, assign15070_body55_e21735_d_n10, assign15070_body55_e21735_d_n11, assign15070_body55_e21735_d_n12, assign15070_body55_e21735_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15070_body55_e21732: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign15070_body55_e21733: f64 = (1.0 + assign15070_body55_e21732);
        (assign15070_body55_e21733, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12)), ((locals.var_fac1_dn17 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn17)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12, locals.var_fs0_dps0_dn17,)
    }
};
            locals.var_fs0_dps0 = assign15070_body55_e21735;
            locals.var_fs0_dps0_dn0 = assign15070_body55_e21735_d_n0;
            locals.var_fs0_dps0_dn2 = assign15070_body55_e21735_d_n2;
            locals.var_fs0_dps0_dn6 = assign15070_body55_e21735_d_n6;
            locals.var_fs0_dps0_dn7 = assign15070_body55_e21735_d_n7;
            locals.var_fs0_dps0_dn10 = assign15070_body55_e21735_d_n10;
            locals.var_fs0_dps0_dn11 = assign15070_body55_e21735_d_n11;
            locals.var_fs0_dps0_dn12 = assign15070_body55_e21735_d_n12;
            locals.var_fs0_dps0_dn17 = assign15070_body55_e21735_d_n17;
            locals.var_fs0_dps0_rv = 0.0;
            let assign15070_body56_e21738: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard461 = assign15070_body56_e21738;
            locals.var_guard461_rv = 0.0;
            let (assign15070_body57_e21747,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard461 != 0.0)) {
        let assign15070_body57_e21745: f64 = (locals.var_lp_s0_max + 1.0);
        (assign15070_body57_e21745,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign15070_body57_e21747;
            locals.var_lp_s0_rv = 0.0;
            let (assign15070_body58_e21758, assign15070_body58_e21758_d_n0, assign15070_body58_e21758_d_n2, assign15070_body58_e21758_d_n6, assign15070_body58_e21758_d_n7, assign15070_body58_e21758_d_n10, assign15070_body58_e21758_d_n11, assign15070_body58_e21758_d_n12, assign15070_body58_e21758_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard461 == 0.0)) {
        let assign15070_body58_e21754: f64 = (-locals.var_fs0);
        let assign15070_body58_e21756: f64 = (assign15070_body58_e21754 / locals.var_fs0_dps0);
        (assign15070_body58_e21756, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign15070_body58_e21754 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign15070_body58_e21754 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign15070_body58_e21754 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign15070_body58_e21754 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign15070_body58_e21754 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign15070_body58_e21754 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign15070_body58_e21754 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn17) * locals.var_fs0_dps0) - (assign15070_body58_e21754 * locals.var_fs0_dps0_dn17)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign15070_body58_e21758;
            locals.var_dps0_dn0 = assign15070_body58_e21758_d_n0;
            locals.var_dps0_dn2 = assign15070_body58_e21758_d_n2;
            locals.var_dps0_dn6 = assign15070_body58_e21758_d_n6;
            locals.var_dps0_dn7 = assign15070_body58_e21758_d_n7;
            locals.var_dps0_dn10 = assign15070_body58_e21758_d_n10;
            locals.var_dps0_dn11 = assign15070_body58_e21758_d_n11;
            locals.var_dps0_dn12 = assign15070_body58_e21758_d_n12;
            locals.var_dps0_dn17 = assign15070_body58_e21758_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign15070_body59_e21779, assign15070_body59_e21779_d_n0, assign15070_body59_e21779_d_n2, assign15070_body59_e21779_d_n6, assign15070_body59_e21779_d_n7, assign15070_body59_e21779_d_n10, assign15070_body59_e21779_d_n11, assign15070_body59_e21779_d_n12, assign15070_body59_e21779_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard461 == 0.0)) {
        let assign15070_body59_e21766: f64 = (0.5 * 0.1);
        let assign15070_body59_e21770: f64 = (locals.var_phi_s0_soi).abs();
        let (assign15070_body59_e21775, assign15070_body59_e21775_d_n0, assign15070_body59_e21775_d_n2, assign15070_body59_e21775_d_n6, assign15070_body59_e21775_d_n7, assign15070_body59_e21775_d_n10, assign15070_body59_e21775_d_n11, assign15070_body59_e21775_d_n12, assign15070_body59_e21775_d_n17,) = {
            if (1.0 >= assign15070_body59_e21770) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15070_body59_e21774: f64 = (locals.var_phi_s0_soi).abs();
                (assign15070_body59_e21774, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn0 } else { (-locals.var_phi_s0_soi_dn0) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn2 } else { (-locals.var_phi_s0_soi_dn2) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn6 } else { (-locals.var_phi_s0_soi_dn6) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn7 } else { (-locals.var_phi_s0_soi_dn7) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn10 } else { (-locals.var_phi_s0_soi_dn10) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn11 } else { (-locals.var_phi_s0_soi_dn11) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn12 } else { (-locals.var_phi_s0_soi_dn12) }, if locals.var_phi_s0_soi >= 0.0 { locals.var_phi_s0_soi_dn17 } else { (-locals.var_phi_s0_soi_dn17) },)
            }
        };
        let assign15070_body59_e21776: f64 = (1.0 + assign15070_body59_e21775);
        let assign15070_body59_e21777: f64 = (assign15070_body59_e21766 * assign15070_body59_e21776);
        (assign15070_body59_e21777, (assign15070_body59_e21766 * assign15070_body59_e21775_d_n0), (assign15070_body59_e21766 * assign15070_body59_e21775_d_n2), (assign15070_body59_e21766 * assign15070_body59_e21775_d_n6), (assign15070_body59_e21766 * assign15070_body59_e21775_d_n7), (assign15070_body59_e21766 * assign15070_body59_e21775_d_n10), (assign15070_body59_e21766 * assign15070_body59_e21775_d_n11), (assign15070_body59_e21766 * assign15070_body59_e21775_d_n12), (assign15070_body59_e21766 * assign15070_body59_e21775_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign15070_body59_e21779;
            locals.var_dplim_dn0 = assign15070_body59_e21779_d_n0;
            locals.var_dplim_dn2 = assign15070_body59_e21779_d_n2;
            locals.var_dplim_dn6 = assign15070_body59_e21779_d_n6;
            locals.var_dplim_dn7 = assign15070_body59_e21779_d_n7;
            locals.var_dplim_dn10 = assign15070_body59_e21779_d_n10;
            locals.var_dplim_dn11 = assign15070_body59_e21779_d_n11;
            locals.var_dplim_dn12 = assign15070_body59_e21779_d_n12;
            locals.var_dplim_dn17 = assign15070_body59_e21779_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign15070_body60_e21781: f64 = (locals.var_dps0).abs();
            let assign15070_body60_e21783: f64 = if assign15070_body60_e21781 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard462 = assign15070_body60_e21783;
            locals.var_guard462_rv = 0.0;
            let (assign15070_body61_e21801, assign15070_body61_e21801_d_n0, assign15070_body61_e21801_d_n2, assign15070_body61_e21801_d_n6, assign15070_body61_e21801_d_n7, assign15070_body61_e21801_d_n10, assign15070_body61_e21801_d_n11, assign15070_body61_e21801_d_n12, assign15070_body61_e21801_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard461 == 0.0)) && (locals.var_guard462 != 0.0)) {
        let (assign15070_body61_e21798,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign15070_body61_e21797: f64 = (-1.0);
                (assign15070_body61_e21797,)
            }
        };
        let assign15070_body61_e21799: f64 = (locals.var_dplim * assign15070_body61_e21798);
        (assign15070_body61_e21799, (locals.var_dplim_dn0 * assign15070_body61_e21798), (locals.var_dplim_dn2 * assign15070_body61_e21798), (locals.var_dplim_dn6 * assign15070_body61_e21798), (locals.var_dplim_dn7 * assign15070_body61_e21798), (locals.var_dplim_dn10 * assign15070_body61_e21798), (locals.var_dplim_dn11 * assign15070_body61_e21798), (locals.var_dplim_dn12 * assign15070_body61_e21798), (locals.var_dplim_dn17 * assign15070_body61_e21798),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign15070_body61_e21801;
            locals.var_dps0_dn0 = assign15070_body61_e21801_d_n0;
            locals.var_dps0_dn2 = assign15070_body61_e21801_d_n2;
            locals.var_dps0_dn6 = assign15070_body61_e21801_d_n6;
            locals.var_dps0_dn7 = assign15070_body61_e21801_d_n7;
            locals.var_dps0_dn10 = assign15070_body61_e21801_d_n10;
            locals.var_dps0_dn11 = assign15070_body61_e21801_d_n11;
            locals.var_dps0_dn12 = assign15070_body61_e21801_d_n12;
            locals.var_dps0_dn17 = assign15070_body61_e21801_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign15070_body62_e21811, assign15070_body62_e21811_d_n0, assign15070_body62_e21811_d_n2, assign15070_body62_e21811_d_n6, assign15070_body62_e21811_d_n7, assign15070_body62_e21811_d_n10, assign15070_body62_e21811_d_n11, assign15070_body62_e21811_d_n12, assign15070_body62_e21811_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard461 == 0.0)) {
        let assign15070_body62_e21809: f64 = (locals.var_phi_s0_soi + locals.var_dps0);
        (assign15070_body62_e21809, (locals.var_phi_s0_soi_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_soi_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_soi_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_soi_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_soi_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_soi_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_soi_dn12 + locals.var_dps0_dn12), (locals.var_phi_s0_soi_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
            locals.var_phi_s0_soi = assign15070_body62_e21811;
            locals.var_phi_s0_soi_dn0 = assign15070_body62_e21811_d_n0;
            locals.var_phi_s0_soi_dn2 = assign15070_body62_e21811_d_n2;
            locals.var_phi_s0_soi_dn6 = assign15070_body62_e21811_d_n6;
            locals.var_phi_s0_soi_dn7 = assign15070_body62_e21811_d_n7;
            locals.var_phi_s0_soi_dn10 = assign15070_body62_e21811_d_n10;
            locals.var_phi_s0_soi_dn11 = assign15070_body62_e21811_d_n11;
            locals.var_phi_s0_soi_dn12 = assign15070_body62_e21811_d_n12;
            locals.var_phi_s0_soi_dn17 = assign15070_body62_e21811_d_n17;
            locals.var_phi_s0_soi_rv = 0.0;
            let assign15070_body63_e21813: f64 = (locals.var_dps0).abs();
            let assign15070_body63_e21817: f64 = (locals.var_fs0).abs();
            let assign15070_body63_e21820: f64 = if ((assign15070_body63_e21813 <= 5e-12) && (assign15070_body63_e21817 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard463 = assign15070_body63_e21820;
            locals.var_guard463_rv = 0.0;
            let (assign15070_body64_e21830,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard461 == 0.0)) && (locals.var_guard463 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign15070_body64_e21830;
            locals.var_flg_conv_rv = 0.0;
            let (assign15070_body65_e21837,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15070_body65_e21835: f64 = (locals.var_lp_s0 + 1.0);
        (assign15070_body65_e21835,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign15070_body65_e21837;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_53(
        locals: &mut StampLocals,
    ) {
        let (assign15080_e21844,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15080_e21842: f64 = (locals.var_lp_s0 - 1.0);
        (assign15080_e21842,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign15080_e21844;
        locals.var_lp_s0_rv = 0.0;

        let (assign15090_e21849, assign15090_e21849_d_n0, assign15090_e21849_d_n2, assign15090_e21849_d_n6, assign15090_e21849_d_n7, assign15090_e21849_d_n10, assign15090_e21849_d_n11, assign15090_e21849_d_n12, assign15090_e21849_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    } else {
        (locals.var_q_deps0, locals.var_q_deps0_dn0, locals.var_q_deps0_dn2, locals.var_q_deps0_dn6, locals.var_q_deps0_dn7, locals.var_q_deps0_dn10, locals.var_q_deps0_dn11, locals.var_q_deps0_dn12, locals.var_q_deps0_dn17,)
    }
};
        locals.var_q_deps0 = assign15090_e21849;
        locals.var_q_deps0_dn0 = assign15090_e21849_d_n0;
        locals.var_q_deps0_dn2 = assign15090_e21849_d_n2;
        locals.var_q_deps0_dn6 = assign15090_e21849_d_n6;
        locals.var_q_deps0_dn7 = assign15090_e21849_d_n7;
        locals.var_q_deps0_dn10 = assign15090_e21849_d_n10;
        locals.var_q_deps0_dn11 = assign15090_e21849_d_n11;
        locals.var_q_deps0_dn12 = assign15090_e21849_d_n12;
        locals.var_q_deps0_dn17 = assign15090_e21849_d_n17;
        locals.var_q_deps0_rv = 0.0;

        let (assign15100_e21854, assign15100_e21854_d_n0, assign15100_e21854_d_n2, assign15100_e21854_d_n6, assign15100_e21854_d_n7, assign15100_e21854_d_n10, assign15100_e21854_d_n11, assign15100_e21854_d_n12, assign15100_e21854_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        (locals.var_q_deps0, locals.var_q_deps0_dn0, locals.var_q_deps0_dn2, locals.var_q_deps0_dn6, locals.var_q_deps0_dn7, locals.var_q_deps0_dn10, locals.var_q_deps0_dn11, locals.var_q_deps0_dn12, locals.var_q_deps0_dn17,)
    } else {
        (locals.var_q_dep0, locals.var_q_dep0_dn0, locals.var_q_dep0_dn2, locals.var_q_dep0_dn6, locals.var_q_dep0_dn7, locals.var_q_dep0_dn10, locals.var_q_dep0_dn11, locals.var_q_dep0_dn12, locals.var_q_dep0_dn17,)
    }
};
        locals.var_q_dep0 = assign15100_e21854;
        locals.var_q_dep0_dn0 = assign15100_e21854_d_n0;
        locals.var_q_dep0_dn2 = assign15100_e21854_d_n2;
        locals.var_q_dep0_dn6 = assign15100_e21854_d_n6;
        locals.var_q_dep0_dn7 = assign15100_e21854_d_n7;
        locals.var_q_dep0_dn10 = assign15100_e21854_d_n10;
        locals.var_q_dep0_dn11 = assign15100_e21854_d_n11;
        locals.var_q_dep0_dn12 = assign15100_e21854_d_n12;
        locals.var_q_dep0_dn17 = assign15100_e21854_d_n17;
        locals.var_q_dep0_rv = 0.0;

        let (assign15110_e21859, assign15110_e21859_d_n0, assign15110_e21859_d_n2, assign15110_e21859_d_n6, assign15110_e21859_d_n7, assign15110_e21859_d_n10, assign15110_e21859_d_n11, assign15110_e21859_d_n12, assign15110_e21859_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign15110_e21859;
        locals.var_ps0_dn0 = assign15110_e21859_d_n0;
        locals.var_ps0_dn2 = assign15110_e21859_d_n2;
        locals.var_ps0_dn6 = assign15110_e21859_d_n6;
        locals.var_ps0_dn7 = assign15110_e21859_d_n7;
        locals.var_ps0_dn10 = assign15110_e21859_d_n10;
        locals.var_ps0_dn11 = assign15110_e21859_d_n11;
        locals.var_ps0_dn12 = assign15110_e21859_d_n12;
        locals.var_ps0_dn17 = assign15110_e21859_d_n17;
        locals.var_ps0_rv = 0.0;

        let (assign15130_e21871, assign15130_e21871_d_n0, assign15130_e21871_d_n2, assign15130_e21871_d_n6, assign15130_e21871_d_n7, assign15130_e21871_d_n10, assign15130_e21871_d_n11, assign15130_e21871_d_n12, assign15130_e21871_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15130_e21869: f64 = (locals.var_q_deps0 / locals.var_cnst0soi);
        (assign15130_e21869, (((locals.var_q_deps0_dn0 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn0)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn2 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn2)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn6 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn6)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn7 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn7)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn10 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn10)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn11 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn11)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn12 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn12)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_deps0_dn17 * locals.var_cnst0soi) - (locals.var_q_deps0 * locals.var_cnst0soi_dn17)) / (locals.var_cnst0soi * locals.var_cnst0soi)),)
    } else {
        (locals.var_q_deps0_soi_o_cnst0soi, locals.var_q_deps0_soi_o_cnst0soi_dn0, locals.var_q_deps0_soi_o_cnst0soi_dn2, locals.var_q_deps0_soi_o_cnst0soi_dn6, locals.var_q_deps0_soi_o_cnst0soi_dn7, locals.var_q_deps0_soi_o_cnst0soi_dn10, locals.var_q_deps0_soi_o_cnst0soi_dn11, locals.var_q_deps0_soi_o_cnst0soi_dn12, locals.var_q_deps0_soi_o_cnst0soi_dn17,)
    }
};
        locals.var_q_deps0_soi_o_cnst0soi = assign15130_e21871;
        locals.var_q_deps0_soi_o_cnst0soi_dn0 = assign15130_e21871_d_n0;
        locals.var_q_deps0_soi_o_cnst0soi_dn2 = assign15130_e21871_d_n2;
        locals.var_q_deps0_soi_o_cnst0soi_dn6 = assign15130_e21871_d_n6;
        locals.var_q_deps0_soi_o_cnst0soi_dn7 = assign15130_e21871_d_n7;
        locals.var_q_deps0_soi_o_cnst0soi_dn10 = assign15130_e21871_d_n10;
        locals.var_q_deps0_soi_o_cnst0soi_dn11 = assign15130_e21871_d_n11;
        locals.var_q_deps0_soi_o_cnst0soi_dn12 = assign15130_e21871_d_n12;
        locals.var_q_deps0_soi_o_cnst0soi_dn17 = assign15130_e21871_d_n17;
        locals.var_q_deps0_soi_o_cnst0soi_rv = 0.0;

        let (assign15140_e21882, assign15140_e21882_d_n0, assign15140_e21882_d_n2, assign15140_e21882_d_n6, assign15140_e21882_d_n7, assign15140_e21882_d_n10, assign15140_e21882_d_n11, assign15140_e21882_d_n12, assign15140_e21882_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15140_e21876: f64 = (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi);
        let assign15140_e21879: f64 = (10.0 * 2.220446049250313e-16);
        let assign15140_e21880: f64 = (assign15140_e21876 + assign15140_e21879);
        (assign15140_e21880, ((locals.var_q_deps0_soi_o_cnst0soi_dn0 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn0)), ((locals.var_q_deps0_soi_o_cnst0soi_dn2 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn2)), ((locals.var_q_deps0_soi_o_cnst0soi_dn6 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn6)), ((locals.var_q_deps0_soi_o_cnst0soi_dn7 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn7)), ((locals.var_q_deps0_soi_o_cnst0soi_dn10 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn10)), ((locals.var_q_deps0_soi_o_cnst0soi_dn11 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn11)), ((locals.var_q_deps0_soi_o_cnst0soi_dn12 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn12)), ((locals.var_q_deps0_soi_o_cnst0soi_dn17 * locals.var_q_deps0_soi_o_cnst0soi) + (locals.var_q_deps0_soi_o_cnst0soi * locals.var_q_deps0_soi_o_cnst0soi_dn17)),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn6, locals.var_xi0_dn7, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12, locals.var_xi0_dn17,)
    }
};
        locals.var_xi0 = assign15140_e21882;
        locals.var_xi0_dn0 = assign15140_e21882_d_n0;
        locals.var_xi0_dn2 = assign15140_e21882_d_n2;
        locals.var_xi0_dn6 = assign15140_e21882_d_n6;
        locals.var_xi0_dn7 = assign15140_e21882_d_n7;
        locals.var_xi0_dn10 = assign15140_e21882_d_n10;
        locals.var_xi0_dn11 = assign15140_e21882_d_n11;
        locals.var_xi0_dn12 = assign15140_e21882_d_n12;
        locals.var_xi0_dn17 = assign15140_e21882_d_n17;
        locals.var_xi0_rv = 0.0;

        let (assign15150_e21889, assign15150_e21889_d_n0, assign15150_e21889_d_n2, assign15150_e21889_d_n6, assign15150_e21889_d_n7, assign15150_e21889_d_n10, assign15150_e21889_d_n11, assign15150_e21889_d_n12, assign15150_e21889_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15150_e21887: f64 = (2.0 * locals.var_q_deps0_soi_o_cnst0soi);
        (assign15150_e21887, (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn0), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn2), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn6), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn7), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn10), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn11), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn12), (2.0 * locals.var_q_deps0_soi_o_cnst0soi_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15150_e21889;
        locals.var_t1_dn0 = assign15150_e21889_d_n0;
        locals.var_t1_dn2 = assign15150_e21889_d_n2;
        locals.var_t1_dn6 = assign15150_e21889_d_n6;
        locals.var_t1_dn7 = assign15150_e21889_d_n7;
        locals.var_t1_dn10 = assign15150_e21889_d_n10;
        locals.var_t1_dn11 = assign15150_e21889_d_n11;
        locals.var_t1_dn12 = assign15150_e21889_d_n12;
        locals.var_t1_dn17 = assign15150_e21889_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15160_e21898, assign15160_e21898_d_n0, assign15160_e21898_d_n2, assign15160_e21898_d_n6, assign15160_e21898_d_n7, assign15160_e21898_d_n10, assign15160_e21898_d_n11, assign15160_e21898_d_n12, assign15160_e21898_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15160_e21895: f64 = (10.0 * 2.220446049250313e-16);
        let assign15160_e21896: f64 = (locals.var_q_deps0_soi_o_cnst0soi + assign15160_e21895);
        (assign15160_e21896, locals.var_q_deps0_soi_o_cnst0soi_dn0, locals.var_q_deps0_soi_o_cnst0soi_dn2, locals.var_q_deps0_soi_o_cnst0soi_dn6, locals.var_q_deps0_soi_o_cnst0soi_dn7, locals.var_q_deps0_soi_o_cnst0soi_dn10, locals.var_q_deps0_soi_o_cnst0soi_dn11, locals.var_q_deps0_soi_o_cnst0soi_dn12, locals.var_q_deps0_soi_o_cnst0soi_dn17,)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12, locals.var_xi0p12_dn17,)
    }
};
        locals.var_xi0p12 = assign15160_e21898;
        locals.var_xi0p12_dn0 = assign15160_e21898_d_n0;
        locals.var_xi0p12_dn2 = assign15160_e21898_d_n2;
        locals.var_xi0p12_dn6 = assign15160_e21898_d_n6;
        locals.var_xi0p12_dn7 = assign15160_e21898_d_n7;
        locals.var_xi0p12_dn10 = assign15160_e21898_d_n10;
        locals.var_xi0p12_dn11 = assign15160_e21898_d_n11;
        locals.var_xi0p12_dn12 = assign15160_e21898_d_n12;
        locals.var_xi0p12_dn17 = assign15160_e21898_d_n17;
        locals.var_xi0p12_rv = 0.0;

        let (assign15170_e21905, assign15170_e21905_d_n0, assign15170_e21905_d_n2, assign15170_e21905_d_n6, assign15170_e21905_d_n7, assign15170_e21905_d_n10, assign15170_e21905_d_n11, assign15170_e21905_d_n12, assign15170_e21905_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15170_e21903: f64 = (locals.var_cnst0soi * locals.var_xi0p12);
        (assign15170_e21903, ((locals.var_cnst0soi_dn0 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_xi0p12) + (locals.var_cnst0soi * locals.var_xi0p12_dn17)),)
    } else {
        (locals.var_qb0, locals.var_qb0_dn0, locals.var_qb0_dn2, locals.var_qb0_dn6, locals.var_qb0_dn7, locals.var_qb0_dn10, locals.var_qb0_dn11, locals.var_qb0_dn12, locals.var_qb0_dn17,)
    }
};
        locals.var_qb0 = assign15170_e21905;
        locals.var_qb0_dn0 = assign15170_e21905_d_n0;
        locals.var_qb0_dn2 = assign15170_e21905_d_n2;
        locals.var_qb0_dn6 = assign15170_e21905_d_n6;
        locals.var_qb0_dn7 = assign15170_e21905_d_n7;
        locals.var_qb0_dn10 = assign15170_e21905_d_n10;
        locals.var_qb0_dn11 = assign15170_e21905_d_n11;
        locals.var_qb0_dn12 = assign15170_e21905_d_n12;
        locals.var_qb0_dn17 = assign15170_e21905_d_n17;
        locals.var_qb0_rv = 0.0;

        let (assign15180_e21914, assign15180_e21914_d_n0, assign15180_e21914_d_n2, assign15180_e21914_d_n6, assign15180_e21914_d_n7, assign15180_e21914_d_n10, assign15180_e21914_d_n11, assign15180_e21914_d_n12, assign15180_e21914_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15180_e21911: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign15180_e21912: f64 = (1.0 / assign15180_e21911);
        (assign15180_e21912, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign15180_e21911 * assign15180_e21911))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign15180_e21911 * assign15180_e21911))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign15180_e21911 * assign15180_e21911))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign15180_e21911 * assign15180_e21911))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign15180_e21911 * assign15180_e21911))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign15180_e21911 * assign15180_e21911))), (-((locals.var_fs02_dn12 + locals.var_xi0p12_dn12) / (assign15180_e21911 * assign15180_e21911))), (-((locals.var_fs02_dn17 + locals.var_xi0p12_dn17) / (assign15180_e21911 * assign15180_e21911))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign15180_e21914;
        locals.var_t1_dn0 = assign15180_e21914_d_n0;
        locals.var_t1_dn2 = assign15180_e21914_d_n2;
        locals.var_t1_dn6 = assign15180_e21914_d_n6;
        locals.var_t1_dn7 = assign15180_e21914_d_n7;
        locals.var_t1_dn10 = assign15180_e21914_d_n10;
        locals.var_t1_dn11 = assign15180_e21914_d_n11;
        locals.var_t1_dn12 = assign15180_e21914_d_n12;
        locals.var_t1_dn17 = assign15180_e21914_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign15190_e21923, assign15190_e21923_d_n0, assign15190_e21923_d_n2, assign15190_e21923_d_n6, assign15190_e21923_d_n7, assign15190_e21923_d_n10, assign15190_e21923_d_n11, assign15190_e21923_d_n12, assign15190_e21923_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15190_e21919: f64 = (locals.var_cnst0soi * locals.var_fs01);
        let assign15190_e21921: f64 = (assign15190_e21919 * locals.var_t1);
        (assign15190_e21921, ((((locals.var_cnst0soi_dn0 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn0)) * locals.var_t1) + (assign15190_e21919 * locals.var_t1_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn2)) * locals.var_t1) + (assign15190_e21919 * locals.var_t1_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn6)) * locals.var_t1) + (assign15190_e21919 * locals.var_t1_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn7)) * locals.var_t1) + (assign15190_e21919 * locals.var_t1_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn10)) * locals.var_t1) + (assign15190_e21919 * locals.var_t1_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn11)) * locals.var_t1) + (assign15190_e21919 * locals.var_t1_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn12)) * locals.var_t1) + (assign15190_e21919 * locals.var_t1_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_fs01) + (locals.var_cnst0soi * locals.var_fs01_dn17)) * locals.var_t1) + (assign15190_e21919 * locals.var_t1_dn17)),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign15190_e21923;
        locals.var_qn0_dn0 = assign15190_e21923_d_n0;
        locals.var_qn0_dn2 = assign15190_e21923_d_n2;
        locals.var_qn0_dn6 = assign15190_e21923_d_n6;
        locals.var_qn0_dn7 = assign15190_e21923_d_n7;
        locals.var_qn0_dn10 = assign15190_e21923_d_n10;
        locals.var_qn0_dn11 = assign15190_e21923_d_n11;
        locals.var_qn0_dn12 = assign15190_e21923_d_n12;
        locals.var_qn0_dn17 = assign15190_e21923_d_n17;
        locals.var_qn0_rv = 0.0;

        let (assign15200_e21929, assign15200_e21929_d_n0, assign15200_e21929_d_n2, assign15200_e21929_d_n6, assign15200_e21929_d_n7, assign15200_e21929_d_n10, assign15200_e21929_d_n11, assign15200_e21929_d_n12, assign15200_e21929_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15200_e21927: f64 = (-locals.var_qn0);
        (assign15200_e21927, (-locals.var_qn0_dn0), (-locals.var_qn0_dn2), (-locals.var_qn0_dn6), (-locals.var_qn0_dn7), (-locals.var_qn0_dn10), (-locals.var_qn0_dn11), (-locals.var_qn0_dn12), (-locals.var_qn0_dn17),)
    } else {
        (locals.var_q_n0, locals.var_q_n0_dn0, locals.var_q_n0_dn2, locals.var_q_n0_dn6, locals.var_q_n0_dn7, locals.var_q_n0_dn10, locals.var_q_n0_dn11, locals.var_q_n0_dn12, locals.var_q_n0_dn17,)
    }
};
        locals.var_q_n0 = assign15200_e21929;
        locals.var_q_n0_dn0 = assign15200_e21929_d_n0;
        locals.var_q_n0_dn2 = assign15200_e21929_d_n2;
        locals.var_q_n0_dn6 = assign15200_e21929_d_n6;
        locals.var_q_n0_dn7 = assign15200_e21929_d_n7;
        locals.var_q_n0_dn10 = assign15200_e21929_d_n10;
        locals.var_q_n0_dn11 = assign15200_e21929_d_n11;
        locals.var_q_n0_dn12 = assign15200_e21929_d_n12;
        locals.var_q_n0_dn17 = assign15200_e21929_d_n17;
        locals.var_q_n0_rv = 0.0;

        let (assign15210_e21936, assign15210_e21936_d_n0, assign15210_e21936_d_n2, assign15210_e21936_d_n6, assign15210_e21936_d_n7, assign15210_e21936_d_n10, assign15210_e21936_d_n11, assign15210_e21936_d_n12, assign15210_e21936_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        let assign15210_e21934: f64 = (locals.var_qn0 * locals.var_c_fox_inv);
        (assign15210_e21934, ((locals.var_qn0_dn0 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn2)), ((locals.var_qn0_dn6 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn7)), ((locals.var_qn0_dn10 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn10)), ((locals.var_qn0_dn11 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn11)), ((locals.var_qn0_dn12 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn12)), ((locals.var_qn0_dn17 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign15210_e21936;
        locals.var_vgvt_dn0 = assign15210_e21936_d_n0;
        locals.var_vgvt_dn2 = assign15210_e21936_d_n2;
        locals.var_vgvt_dn6 = assign15210_e21936_d_n6;
        locals.var_vgvt_dn7 = assign15210_e21936_d_n7;
        locals.var_vgvt_dn10 = assign15210_e21936_d_n10;
        locals.var_vgvt_dn11 = assign15210_e21936_d_n11;
        locals.var_vgvt_dn12 = assign15210_e21936_d_n12;
        locals.var_vgvt_dn17 = assign15210_e21936_d_n17;
        locals.var_vgvt_rv = 0.0;

        let assign15220_e21939: f64 = (-1.0);
        let assign15220_e21944: f64 = if ((locals.var_flg_zone == assign15220_e21939) || (locals.var_vgvt <= 1e-12)) { 1.0 } else { 0.0 };
        locals.var_guard464 = assign15220_e21944;
        locals.var_guard464_rv = 0.0;

        let (assign15230_e21951,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign15230_e21951;
        locals.var_flg_zone_rv = 0.0;

        let (assign15240_e21958,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign15240_e21958;
        locals.var_flg_noqi_rv = 0.0;

        let (assign15250_e21967, assign15250_e21967_d_n0, assign15250_e21967_d_n2, assign15250_e21967_d_n6, assign15250_e21967_d_n7, assign15250_e21967_d_n10, assign15250_e21967_d_n11, assign15250_e21967_d_n12, assign15250_e21967_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        let assign15250_e21965: f64 = (locals.var_vgp - locals.var_ps0);
        (assign15250_e21965, (locals.var_vgp_dn0 - locals.var_ps0_dn0), (locals.var_vgp_dn2 - locals.var_ps0_dn2), (locals.var_vgp_dn6 - locals.var_ps0_dn6), (locals.var_vgp_dn7 - locals.var_ps0_dn7), (locals.var_vgp_dn10 - locals.var_ps0_dn10), (locals.var_vgp_dn11 - locals.var_ps0_dn11), (locals.var_vgp_dn12 - locals.var_ps0_dn12), (locals.var_vgp_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign15250_e21967;
        locals.var_t2_dn0 = assign15250_e21967_d_n0;
        locals.var_t2_dn2 = assign15250_e21967_d_n2;
        locals.var_t2_dn6 = assign15250_e21967_d_n6;
        locals.var_t2_dn7 = assign15250_e21967_d_n7;
        locals.var_t2_dn10 = assign15250_e21967_d_n10;
        locals.var_t2_dn11 = assign15250_e21967_d_n11;
        locals.var_t2_dn12 = assign15250_e21967_d_n12;
        locals.var_t2_dn17 = assign15250_e21967_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign15260_e21976, assign15260_e21976_d_n0, assign15260_e21976_d_n2, assign15260_e21976_d_n6, assign15260_e21976_d_n7, assign15260_e21976_d_n10, assign15260_e21976_d_n11, assign15260_e21976_d_n12, assign15260_e21976_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        let assign15260_e21974: f64 = (locals.var_c_fox * locals.var_t2);
        (assign15260_e21974, ((locals.var_c_fox_dn0 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn0)), ((locals.var_c_fox_dn2 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn2)), ((locals.var_c_fox_dn6 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn6)), ((locals.var_c_fox_dn7 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn7)), ((locals.var_c_fox_dn10 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn10)), ((locals.var_c_fox_dn11 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn11)), ((locals.var_c_fox_dn12 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn12)), ((locals.var_c_fox_dn17 * locals.var_t2) + (locals.var_c_fox * locals.var_t2_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign15260_e21976;
        locals.var_qbu_dn0 = assign15260_e21976_d_n0;
        locals.var_qbu_dn2 = assign15260_e21976_d_n2;
        locals.var_qbu_dn6 = assign15260_e21976_d_n6;
        locals.var_qbu_dn7 = assign15260_e21976_d_n7;
        locals.var_qbu_dn10 = assign15260_e21976_d_n10;
        locals.var_qbu_dn11 = assign15260_e21976_d_n11;
        locals.var_qbu_dn12 = assign15260_e21976_d_n12;
        locals.var_qbu_dn17 = assign15260_e21976_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign15270_e21986, assign15270_e21986_d_n0, assign15270_e21986_d_n2, assign15270_e21986_d_n6, assign15270_e21986_d_n7, assign15270_e21986_d_n10, assign15270_e21986_d_n11, assign15270_e21986_d_n12, assign15270_e21986_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        let assign15270_e21982: f64 = (-locals.var_weffcv_nf);
        let assign15270_e21984: f64 = (assign15270_e21982 * locals.var_leff_cv);
        (assign15270_e21984, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign15270_e21986;
        locals.var_t0_dn0 = assign15270_e21986_d_n0;
        locals.var_t0_dn2 = assign15270_e21986_d_n2;
        locals.var_t0_dn6 = assign15270_e21986_d_n6;
        locals.var_t0_dn7 = assign15270_e21986_d_n7;
        locals.var_t0_dn10 = assign15270_e21986_d_n10;
        locals.var_t0_dn11 = assign15270_e21986_d_n11;
        locals.var_t0_dn12 = assign15270_e21986_d_n12;
        locals.var_t0_dn17 = assign15270_e21986_d_n17;
        locals.var_t0_rv = 0.0;

        let (assign15280_e21995, assign15280_e21995_d_n0, assign15280_e21995_d_n2, assign15280_e21995_d_n6, assign15280_e21995_d_n7, assign15280_e21995_d_n10, assign15280_e21995_d_n11, assign15280_e21995_d_n12, assign15280_e21995_d_n13, assign15280_e21995_d_n15, assign15280_e21995_d_n16, assign15280_e21995_d_n17, assign15280_e21995_d_n18,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        let assign15280_e21993: f64 = (locals.var_t0 * locals.var_qbu);
        (assign15280_e21993, ((locals.var_t0_dn0 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn0)), ((locals.var_t0_dn2 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn2)), ((locals.var_t0_dn6 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn6)), ((locals.var_t0_dn7 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn7)), ((locals.var_t0_dn10 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn10)), ((locals.var_t0_dn11 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn11)), ((locals.var_t0_dn12 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t0_dn17 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign15280_e21995;
        locals.var_qb_dn0 = assign15280_e21995_d_n0;
        locals.var_qb_dn2 = assign15280_e21995_d_n2;
        locals.var_qb_dn6 = assign15280_e21995_d_n6;
        locals.var_qb_dn7 = assign15280_e21995_d_n7;
        locals.var_qb_dn10 = assign15280_e21995_d_n10;
        locals.var_qb_dn11 = assign15280_e21995_d_n11;
        locals.var_qb_dn12 = assign15280_e21995_d_n12;
        locals.var_qb_dn13 = assign15280_e21995_d_n13;
        locals.var_qb_dn15 = assign15280_e21995_d_n15;
        locals.var_qb_dn16 = assign15280_e21995_d_n16;
        locals.var_qb_dn17 = assign15280_e21995_d_n17;
        locals.var_qb_dn18 = assign15280_e21995_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign15290_e22002, assign15290_e22002_d_n0, assign15290_e22002_d_n2, assign15290_e22002_d_n6, assign15290_e22002_d_n7, assign15290_e22002_d_n10, assign15290_e22002_d_n11, assign15290_e22002_d_n12, assign15290_e22002_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign15290_e22002;
        locals.var_qi_dn0 = assign15290_e22002_d_n0;
        locals.var_qi_dn2 = assign15290_e22002_d_n2;
        locals.var_qi_dn6 = assign15290_e22002_d_n6;
        locals.var_qi_dn7 = assign15290_e22002_d_n7;
        locals.var_qi_dn10 = assign15290_e22002_d_n10;
        locals.var_qi_dn11 = assign15290_e22002_d_n11;
        locals.var_qi_dn12 = assign15290_e22002_d_n12;
        locals.var_qi_dn17 = assign15290_e22002_d_n17;
        locals.var_qi_rv = 0.0;

        let (assign15300_e22009, assign15300_e22009_d_n0, assign15300_e22009_d_n2, assign15300_e22009_d_n6, assign15300_e22009_d_n7, assign15300_e22009_d_n10, assign15300_e22009_d_n11, assign15300_e22009_d_n12, assign15300_e22009_d_n13, assign15300_e22009_d_n15, assign15300_e22009_d_n16, assign15300_e22009_d_n17, assign15300_e22009_d_n18,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign15300_e22009;
        locals.var_qd_dn0 = assign15300_e22009_d_n0;
        locals.var_qd_dn2 = assign15300_e22009_d_n2;
        locals.var_qd_dn6 = assign15300_e22009_d_n6;
        locals.var_qd_dn7 = assign15300_e22009_d_n7;
        locals.var_qd_dn10 = assign15300_e22009_d_n10;
        locals.var_qd_dn11 = assign15300_e22009_d_n11;
        locals.var_qd_dn12 = assign15300_e22009_d_n12;
        locals.var_qd_dn13 = assign15300_e22009_d_n13;
        locals.var_qd_dn15 = assign15300_e22009_d_n15;
        locals.var_qd_dn16 = assign15300_e22009_d_n16;
        locals.var_qd_dn17 = assign15300_e22009_d_n17;
        locals.var_qd_dn18 = assign15300_e22009_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign15310_e22019, assign15310_e22019_d_n0, assign15310_e22019_d_n2, assign15310_e22019_d_n6, assign15310_e22019_d_n7, assign15310_e22019_d_n10, assign15310_e22019_d_n11, assign15310_e22019_d_n12, assign15310_e22019_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        let assign15310_e22015: f64 = (-locals.var_area_bt_n);
        let assign15310_e22017: f64 = (assign15310_e22015 * locals.var_qbu);
        (assign15310_e22017, (assign15310_e22015 * locals.var_qbu_dn0), (assign15310_e22015 * locals.var_qbu_dn2), (assign15310_e22015 * locals.var_qbu_dn6), (assign15310_e22015 * locals.var_qbu_dn7), (assign15310_e22015 * locals.var_qbu_dn10), (assign15310_e22015 * locals.var_qbu_dn11), (assign15310_e22015 * locals.var_qbu_dn12), (assign15310_e22015 * locals.var_qbu_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign15310_e22019;
        locals.var_t2_dn0 = assign15310_e22019_d_n0;
        locals.var_t2_dn2 = assign15310_e22019_d_n2;
        locals.var_t2_dn6 = assign15310_e22019_d_n6;
        locals.var_t2_dn7 = assign15310_e22019_d_n7;
        locals.var_t2_dn10 = assign15310_e22019_d_n10;
        locals.var_t2_dn11 = assign15310_e22019_d_n11;
        locals.var_t2_dn12 = assign15310_e22019_d_n12;
        locals.var_t2_dn17 = assign15310_e22019_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign15320_e22028, assign15320_e22028_d_n0, assign15320_e22028_d_n2, assign15320_e22028_d_n6, assign15320_e22028_d_n7, assign15320_e22028_d_n10, assign15320_e22028_d_n11, assign15320_e22028_d_n12, assign15320_e22028_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        let assign15320_e22026: f64 = (locals.var_t2 * locals.var_qdrat);
        (assign15320_e22026, ((locals.var_t2_dn0 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn0)), ((locals.var_t2_dn2 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn2)), ((locals.var_t2_dn6 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn6)), ((locals.var_t2_dn7 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn7)), ((locals.var_t2_dn10 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn10)), ((locals.var_t2_dn11 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn11)), ((locals.var_t2_dn12 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn12)), ((locals.var_t2_dn17 * locals.var_qdrat) + (locals.var_t2 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign15320_e22028;
        locals.var_qbody_bt_n_sud_dn0 = assign15320_e22028_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign15320_e22028_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign15320_e22028_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign15320_e22028_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign15320_e22028_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign15320_e22028_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign15320_e22028_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign15320_e22028_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign15330_e22037, assign15330_e22037_d_n0, assign15330_e22037_d_n2, assign15330_e22037_d_n6, assign15330_e22037_d_n7, assign15330_e22037_d_n10, assign15330_e22037_d_n11, assign15330_e22037_d_n12, assign15330_e22037_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        let assign15330_e22035: f64 = (locals.var_t2 - locals.var_qbody_bt_n_sud);
        (assign15330_e22035, (locals.var_t2_dn0 - locals.var_qbody_bt_n_sud_dn0), (locals.var_t2_dn2 - locals.var_qbody_bt_n_sud_dn2), (locals.var_t2_dn6 - locals.var_qbody_bt_n_sud_dn6), (locals.var_t2_dn7 - locals.var_qbody_bt_n_sud_dn7), (locals.var_t2_dn10 - locals.var_qbody_bt_n_sud_dn10), (locals.var_t2_dn11 - locals.var_qbody_bt_n_sud_dn11), (locals.var_t2_dn12 - locals.var_qbody_bt_n_sud_dn12), (locals.var_t2_dn17 - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign15330_e22037;
        locals.var_qbody_bt_n_sus_dn0 = assign15330_e22037_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign15330_e22037_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign15330_e22037_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign15330_e22037_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign15330_e22037_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign15330_e22037_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign15330_e22037_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign15330_e22037_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign15340_e22044, assign15340_e22044_d_n0, assign15340_e22044_d_n2, assign15340_e22044_d_n6, assign15340_e22044_d_n7, assign15340_e22044_d_n10, assign15340_e22044_d_n11, assign15340_e22044_d_n12, assign15340_e22044_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign15340_e22044;
        locals.var_qbody_bt_n_iud_dn0 = assign15340_e22044_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign15340_e22044_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign15340_e22044_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign15340_e22044_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign15340_e22044_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign15340_e22044_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign15340_e22044_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign15340_e22044_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        let (assign15350_e22051, assign15350_e22051_d_n0, assign15350_e22051_d_n2, assign15350_e22051_d_n6, assign15350_e22051_d_n7, assign15350_e22051_d_n10, assign15350_e22051_d_n11, assign15350_e22051_d_n12, assign15350_e22051_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign15350_e22051;
        locals.var_qbody_bt_n_ius_dn0 = assign15350_e22051_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign15350_e22051_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign15350_e22051_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign15350_e22051_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign15350_e22051_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign15350_e22051_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign15350_e22051_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign15350_e22051_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign15360_e22058, assign15360_e22058_d_n0, assign15360_e22058_d_n2, assign15360_e22058_d_n6, assign15360_e22058_d_n7, assign15360_e22058_d_n10, assign15360_e22058_d_n11, assign15360_e22058_d_n12, assign15360_e22058_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign15360_e22058;
        locals.var_ids_dn0 = assign15360_e22058_d_n0;
        locals.var_ids_dn2 = assign15360_e22058_d_n2;
        locals.var_ids_dn6 = assign15360_e22058_d_n6;
        locals.var_ids_dn7 = assign15360_e22058_d_n7;
        locals.var_ids_dn10 = assign15360_e22058_d_n10;
        locals.var_ids_dn11 = assign15360_e22058_d_n11;
        locals.var_ids_dn12 = assign15360_e22058_d_n12;
        locals.var_ids_dn17 = assign15360_e22058_d_n17;
        locals.var_ids_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        locals: &mut StampLocals,
    ) {
        let (assign15370_e22065, assign15370_e22065_d_n0, assign15370_e22065_d_n2, assign15370_e22065_d_n6, assign15370_e22065_d_n7, assign15370_e22065_d_n10, assign15370_e22065_d_n11, assign15370_e22065_d_n12, assign15370_e22065_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign15370_e22065;
        locals.var_vgvt_dn0 = assign15370_e22065_d_n0;
        locals.var_vgvt_dn2 = assign15370_e22065_d_n2;
        locals.var_vgvt_dn6 = assign15370_e22065_d_n6;
        locals.var_vgvt_dn7 = assign15370_e22065_d_n7;
        locals.var_vgvt_dn10 = assign15370_e22065_d_n10;
        locals.var_vgvt_dn11 = assign15370_e22065_d_n11;
        locals.var_vgvt_dn12 = assign15370_e22065_d_n12;
        locals.var_vgvt_dn17 = assign15370_e22065_d_n17;
        locals.var_vgvt_rv = 0.0;

        let (assign15380_e22072,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign15380_e22072;
        locals.var_flg_noqi_rv = 0.0;

        let (assign15390_e22079, assign15390_e22079_d_n0, assign15390_e22079_d_n2, assign15390_e22079_d_n6, assign15390_e22079_d_n7, assign15390_e22079_d_n10, assign15390_e22079_d_n11, assign15390_e22079_d_n12, assign15390_e22079_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign15390_e22079;
        locals.var_phi_sl_soi_dn0 = assign15390_e22079_d_n0;
        locals.var_phi_sl_soi_dn2 = assign15390_e22079_d_n2;
        locals.var_phi_sl_soi_dn6 = assign15390_e22079_d_n6;
        locals.var_phi_sl_soi_dn7 = assign15390_e22079_d_n7;
        locals.var_phi_sl_soi_dn10 = assign15390_e22079_d_n10;
        locals.var_phi_sl_soi_dn11 = assign15390_e22079_d_n11;
        locals.var_phi_sl_soi_dn12 = assign15390_e22079_d_n12;
        locals.var_phi_sl_soi_dn17 = assign15390_e22079_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign15400_e22086, assign15400_e22086_d_n0, assign15400_e22086_d_n2, assign15400_e22086_d_n6, assign15400_e22086_d_n7, assign15400_e22086_d_n10, assign15400_e22086_d_n11, assign15400_e22086_d_n12, assign15400_e22086_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign15400_e22086;
        locals.var_psl_dn0 = assign15400_e22086_d_n0;
        locals.var_psl_dn2 = assign15400_e22086_d_n2;
        locals.var_psl_dn6 = assign15400_e22086_d_n6;
        locals.var_psl_dn7 = assign15400_e22086_d_n7;
        locals.var_psl_dn10 = assign15400_e22086_d_n10;
        locals.var_psl_dn11 = assign15400_e22086_d_n11;
        locals.var_psl_dn12 = assign15400_e22086_d_n12;
        locals.var_psl_dn17 = assign15400_e22086_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign15410_e22093, assign15410_e22093_d_n0, assign15410_e22093_d_n2, assign15410_e22093_d_n6, assign15410_e22093_d_n7, assign15410_e22093_d_n10, assign15410_e22093_d_n11, assign15410_e22093_d_n12, assign15410_e22093_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign15410_e22093;
        locals.var_psdl_dn0 = assign15410_e22093_d_n0;
        locals.var_psdl_dn2 = assign15410_e22093_d_n2;
        locals.var_psdl_dn6 = assign15410_e22093_d_n6;
        locals.var_psdl_dn7 = assign15410_e22093_d_n7;
        locals.var_psdl_dn10 = assign15410_e22093_d_n10;
        locals.var_psdl_dn11 = assign15410_e22093_d_n11;
        locals.var_psdl_dn12 = assign15410_e22093_d_n12;
        locals.var_psdl_dn17 = assign15410_e22093_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign15420_e22100,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard464 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign15420_e22100;
        locals.var_end_of_part_1_rv = 0.0;

        let assign15430_e22103: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign15430_e22103;
        locals.var_guard465_rv = 0.0;

        let (assign15440_e22110, assign15440_e22110_d_n0, assign15440_e22110_d_n2, assign15440_e22110_d_n6, assign15440_e22110_d_n7, assign15440_e22110_d_n10, assign15440_e22110_d_n11, assign15440_e22110_d_n12, assign15440_e22110_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    }
};
        locals.var_vdsorg = assign15440_e22110;
        locals.var_vdsorg_dn0 = assign15440_e22110_d_n0;
        locals.var_vdsorg_dn2 = assign15440_e22110_d_n2;
        locals.var_vdsorg_dn6 = assign15440_e22110_d_n6;
        locals.var_vdsorg_dn7 = assign15440_e22110_d_n7;
        locals.var_vdsorg_dn10 = assign15440_e22110_d_n10;
        locals.var_vdsorg_dn11 = assign15440_e22110_d_n11;
        locals.var_vdsorg_dn12 = assign15440_e22110_d_n12;
        locals.var_vdsorg_dn17 = assign15440_e22110_d_n17;
        locals.var_vdsorg_rv = 0.0;

        let (assign15450_e22117, assign15450_e22117_d_n0, assign15450_e22117_d_n2, assign15450_e22117_d_n6, assign15450_e22117_d_n7, assign15450_e22117_d_n10, assign15450_e22117_d_n11, assign15450_e22117_d_n12, assign15450_e22117_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10__blk472, locals.var_t10__blk472_dn0, locals.var_t10__blk472_dn2, locals.var_t10__blk472_dn6, locals.var_t10__blk472_dn7, locals.var_t10__blk472_dn10, locals.var_t10__blk472_dn11, locals.var_t10__blk472_dn12, locals.var_t10__blk472_dn17,)
    }
};
        locals.var_t10__blk472 = assign15450_e22117;
        locals.var_t10__blk472_dn0 = assign15450_e22117_d_n0;
        locals.var_t10__blk472_dn2 = assign15450_e22117_d_n2;
        locals.var_t10__blk472_dn6 = assign15450_e22117_d_n6;
        locals.var_t10__blk472_dn7 = assign15450_e22117_d_n7;
        locals.var_t10__blk472_dn10 = assign15450_e22117_d_n10;
        locals.var_t10__blk472_dn11 = assign15450_e22117_d_n11;
        locals.var_t10__blk472_dn12 = assign15450_e22117_d_n12;
        locals.var_t10__blk472_dn17 = assign15450_e22117_d_n17;
        locals.var_t10__blk472_rv = 0.0;

        let (assign15460_e22128, assign15460_e22128_d_n0, assign15460_e22128_d_n2, assign15460_e22128_d_n6, assign15460_e22128_d_n7, assign15460_e22128_d_n10, assign15460_e22128_d_n11, assign15460_e22128_d_n12, assign15460_e22128_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15460_e22125: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign15460_e22126: f64 = (locals.var_qnsub_esi / assign15460_e22125);
        (assign15460_e22126, (((locals.var_qnsub_esi_dn0 * assign15460_e22125) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign15460_e22125 * assign15460_e22125)), (((locals.var_qnsub_esi_dn2 * assign15460_e22125) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign15460_e22125 * assign15460_e22125)), (((locals.var_qnsub_esi_dn6 * assign15460_e22125) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign15460_e22125 * assign15460_e22125)), (((locals.var_qnsub_esi_dn7 * assign15460_e22125) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign15460_e22125 * assign15460_e22125)), (((locals.var_qnsub_esi_dn10 * assign15460_e22125) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign15460_e22125 * assign15460_e22125)), (((locals.var_qnsub_esi_dn11 * assign15460_e22125) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign15460_e22125 * assign15460_e22125)), (((locals.var_qnsub_esi_dn12 * assign15460_e22125) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign15460_e22125 * assign15460_e22125)), (((locals.var_qnsub_esi_dn17 * assign15460_e22125) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign15460_e22125 * assign15460_e22125)),)
    } else {
        (locals.var_t2__blk467, locals.var_t2__blk467_dn0, locals.var_t2__blk467_dn2, locals.var_t2__blk467_dn6, locals.var_t2__blk467_dn7, locals.var_t2__blk467_dn10, locals.var_t2__blk467_dn11, locals.var_t2__blk467_dn12, locals.var_t2__blk467_dn17,)
    }
};
        locals.var_t2__blk467 = assign15460_e22128;
        locals.var_t2__blk467_dn0 = assign15460_e22128_d_n0;
        locals.var_t2__blk467_dn2 = assign15460_e22128_d_n2;
        locals.var_t2__blk467_dn6 = assign15460_e22128_d_n6;
        locals.var_t2__blk467_dn7 = assign15460_e22128_d_n7;
        locals.var_t2__blk467_dn10 = assign15460_e22128_d_n10;
        locals.var_t2__blk467_dn11 = assign15460_e22128_d_n11;
        locals.var_t2__blk467_dn12 = assign15460_e22128_d_n12;
        locals.var_t2__blk467_dn17 = assign15460_e22128_d_n17;
        locals.var_t2__blk467_rv = 0.0;

        let (assign15470_e22143, assign15470_e22143_d_n0, assign15470_e22143_d_n2, assign15470_e22143_d_n6, assign15470_e22143_d_n7, assign15470_e22143_d_n10, assign15470_e22143_d_n11, assign15470_e22143_d_n12, assign15470_e22143_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15470_e22136: f64 = (2.0 / locals.var_t2__blk467);
        let assign15470_e22139: f64 = (locals.var_vgp - locals.var_t10__blk472);
        let assign15470_e22140: f64 = (assign15470_e22136 * assign15470_e22139);
        let assign15470_e22141: f64 = (1.0 + assign15470_e22140);
        (assign15470_e22141, (((-((2.0 * locals.var_t2__blk467_dn0) / (locals.var_t2__blk467 * locals.var_t2__blk467))) * assign15470_e22139) + (assign15470_e22136 * (locals.var_vgp_dn0 - locals.var_t10__blk472_dn0))), (((-((2.0 * locals.var_t2__blk467_dn2) / (locals.var_t2__blk467 * locals.var_t2__blk467))) * assign15470_e22139) + (assign15470_e22136 * (locals.var_vgp_dn2 - locals.var_t10__blk472_dn2))), (((-((2.0 * locals.var_t2__blk467_dn6) / (locals.var_t2__blk467 * locals.var_t2__blk467))) * assign15470_e22139) + (assign15470_e22136 * (locals.var_vgp_dn6 - locals.var_t10__blk472_dn6))), (((-((2.0 * locals.var_t2__blk467_dn7) / (locals.var_t2__blk467 * locals.var_t2__blk467))) * assign15470_e22139) + (assign15470_e22136 * (locals.var_vgp_dn7 - locals.var_t10__blk472_dn7))), (((-((2.0 * locals.var_t2__blk467_dn10) / (locals.var_t2__blk467 * locals.var_t2__blk467))) * assign15470_e22139) + (assign15470_e22136 * (locals.var_vgp_dn10 - locals.var_t10__blk472_dn10))), (((-((2.0 * locals.var_t2__blk467_dn11) / (locals.var_t2__blk467 * locals.var_t2__blk467))) * assign15470_e22139) + (assign15470_e22136 * (locals.var_vgp_dn11 - locals.var_t10__blk472_dn11))), (((-((2.0 * locals.var_t2__blk467_dn12) / (locals.var_t2__blk467 * locals.var_t2__blk467))) * assign15470_e22139) + (assign15470_e22136 * (locals.var_vgp_dn12 - locals.var_t10__blk472_dn12))), (((-((2.0 * locals.var_t2__blk467_dn17) / (locals.var_t2__blk467 * locals.var_t2__blk467))) * assign15470_e22139) + (assign15470_e22136 * (locals.var_vgp_dn17 - locals.var_t10__blk472_dn17))),)
    } else {
        (locals.var_t4__blk469, locals.var_t4__blk469_dn0, locals.var_t4__blk469_dn2, locals.var_t4__blk469_dn6, locals.var_t4__blk469_dn7, locals.var_t4__blk469_dn10, locals.var_t4__blk469_dn11, locals.var_t4__blk469_dn12, locals.var_t4__blk469_dn17,)
    }
};
        locals.var_t4__blk469 = assign15470_e22143;
        locals.var_t4__blk469_dn0 = assign15470_e22143_d_n0;
        locals.var_t4__blk469_dn2 = assign15470_e22143_d_n2;
        locals.var_t4__blk469_dn6 = assign15470_e22143_d_n6;
        locals.var_t4__blk469_dn7 = assign15470_e22143_d_n7;
        locals.var_t4__blk469_dn10 = assign15470_e22143_d_n10;
        locals.var_t4__blk469_dn11 = assign15470_e22143_d_n11;
        locals.var_t4__blk469_dn12 = assign15470_e22143_d_n12;
        locals.var_t4__blk469_dn17 = assign15470_e22143_d_n17;
        locals.var_t4__blk469_rv = 0.0;

        let (assign15480_e22154, assign15480_e22154_d_n0, assign15480_e22154_d_n2, assign15480_e22154_d_n6, assign15480_e22154_d_n7, assign15480_e22154_d_n10, assign15480_e22154_d_n11, assign15480_e22154_d_n12, assign15480_e22154_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15480_e22151: f64 = (2.0 / locals.var_t2__blk467);
        let assign15480_e22152: f64 = (1.0 + assign15480_e22151);
        (assign15480_e22152, (-((2.0 * locals.var_t2__blk467_dn0) / (locals.var_t2__blk467 * locals.var_t2__blk467))), (-((2.0 * locals.var_t2__blk467_dn2) / (locals.var_t2__blk467 * locals.var_t2__blk467))), (-((2.0 * locals.var_t2__blk467_dn6) / (locals.var_t2__blk467 * locals.var_t2__blk467))), (-((2.0 * locals.var_t2__blk467_dn7) / (locals.var_t2__blk467 * locals.var_t2__blk467))), (-((2.0 * locals.var_t2__blk467_dn10) / (locals.var_t2__blk467 * locals.var_t2__blk467))), (-((2.0 * locals.var_t2__blk467_dn11) / (locals.var_t2__blk467 * locals.var_t2__blk467))), (-((2.0 * locals.var_t2__blk467_dn12) / (locals.var_t2__blk467 * locals.var_t2__blk467))), (-((2.0 * locals.var_t2__blk467_dn17) / (locals.var_t2__blk467 * locals.var_t2__blk467))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign15480_e22154;
        locals.var_t5_dn0 = assign15480_e22154_d_n0;
        locals.var_t5_dn2 = assign15480_e22154_d_n2;
        locals.var_t5_dn6 = assign15480_e22154_d_n6;
        locals.var_t5_dn7 = assign15480_e22154_d_n7;
        locals.var_t5_dn10 = assign15480_e22154_d_n10;
        locals.var_t5_dn11 = assign15480_e22154_d_n11;
        locals.var_t5_dn12 = assign15480_e22154_d_n12;
        locals.var_t5_dn17 = assign15480_e22154_d_n17;
        locals.var_t5_rv = 0.0;

        let assign15490_e22158: f64 = locals.var_t5;
        let assign15490_e22163: f64 = if ((locals.var_t4__blk469 < assign15490_e22158) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard473 = assign15490_e22163;
        locals.var_guard473_rv = 0.0;

        let (assign15500_e22176, assign15500_e22176_d_n0, assign15500_e22176_d_n2, assign15500_e22176_d_n6, assign15500_e22176_d_n7, assign15500_e22176_d_n10, assign15500_e22176_d_n11, assign15500_e22176_d_n12, assign15500_e22176_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15500_e22172: f64 = locals.var_t5;
        let assign15500_e22174: f64 = (assign15500_e22172 - locals.var_t4__blk469);
        (assign15500_e22174, (locals.var_t5_dn0 - locals.var_t4__blk469_dn0), (locals.var_t5_dn2 - locals.var_t4__blk469_dn2), (locals.var_t5_dn6 - locals.var_t4__blk469_dn6), (locals.var_t5_dn7 - locals.var_t4__blk469_dn7), (locals.var_t5_dn10 - locals.var_t4__blk469_dn10), (locals.var_t5_dn11 - locals.var_t4__blk469_dn11), (locals.var_t5_dn12 - locals.var_t4__blk469_dn12), (locals.var_t5_dn17 - locals.var_t4__blk469_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign15500_e22176;
        locals.var_tmf1_dn0 = assign15500_e22176_d_n0;
        locals.var_tmf1_dn2 = assign15500_e22176_d_n2;
        locals.var_tmf1_dn6 = assign15500_e22176_d_n6;
        locals.var_tmf1_dn7 = assign15500_e22176_d_n7;
        locals.var_tmf1_dn10 = assign15500_e22176_d_n10;
        locals.var_tmf1_dn11 = assign15500_e22176_d_n11;
        locals.var_tmf1_dn12 = assign15500_e22176_d_n12;
        locals.var_tmf1_dn17 = assign15500_e22176_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign15510_e22187, assign15510_e22187_d_n0, assign15510_e22187_d_n2, assign15510_e22187_d_n6, assign15510_e22187_d_n7, assign15510_e22187_d_n10, assign15510_e22187_d_n11, assign15510_e22187_d_n12, assign15510_e22187_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15510_e22185: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign15510_e22185, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign15510_e22187;
        locals.var_x2_dn0 = assign15510_e22187_d_n0;
        locals.var_x2_dn2 = assign15510_e22187_d_n2;
        locals.var_x2_dn6 = assign15510_e22187_d_n6;
        locals.var_x2_dn7 = assign15510_e22187_d_n7;
        locals.var_x2_dn10 = assign15510_e22187_d_n10;
        locals.var_x2_dn11 = assign15510_e22187_d_n11;
        locals.var_x2_dn12 = assign15510_e22187_d_n12;
        locals.var_x2_dn17 = assign15510_e22187_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign15520_e22198, assign15520_e22198_d_n0, assign15520_e22198_d_n2, assign15520_e22198_d_n6, assign15520_e22198_d_n7, assign15520_e22198_d_n10, assign15520_e22198_d_n11, assign15520_e22198_d_n12, assign15520_e22198_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15520_e22196: f64 = (locals.var_t5 * locals.var_t5);
        (assign15520_e22196, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn12 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn12)), ((locals.var_t5_dn17 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign15520_e22198;
        locals.var_xmax2_dn0 = assign15520_e22198_d_n0;
        locals.var_xmax2_dn2 = assign15520_e22198_d_n2;
        locals.var_xmax2_dn6 = assign15520_e22198_d_n6;
        locals.var_xmax2_dn7 = assign15520_e22198_d_n7;
        locals.var_xmax2_dn10 = assign15520_e22198_d_n10;
        locals.var_xmax2_dn11 = assign15520_e22198_d_n11;
        locals.var_xmax2_dn12 = assign15520_e22198_d_n12;
        locals.var_xmax2_dn17 = assign15520_e22198_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign15530_e22207, assign15530_e22207_d_n0, assign15530_e22207_d_n2, assign15530_e22207_d_n6, assign15530_e22207_d_n7, assign15530_e22207_d_n10, assign15530_e22207_d_n11, assign15530_e22207_d_n12, assign15530_e22207_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15530_e22207;
        locals.var_xp_dn0 = assign15530_e22207_d_n0;
        locals.var_xp_dn2 = assign15530_e22207_d_n2;
        locals.var_xp_dn6 = assign15530_e22207_d_n6;
        locals.var_xp_dn7 = assign15530_e22207_d_n7;
        locals.var_xp_dn10 = assign15530_e22207_d_n10;
        locals.var_xp_dn11 = assign15530_e22207_d_n11;
        locals.var_xp_dn12 = assign15530_e22207_d_n12;
        locals.var_xp_dn17 = assign15530_e22207_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15540_e22216, assign15540_e22216_d_n0, assign15540_e22216_d_n2, assign15540_e22216_d_n6, assign15540_e22216_d_n7, assign15540_e22216_d_n10, assign15540_e22216_d_n11, assign15540_e22216_d_n12, assign15540_e22216_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15540_e22216;
        locals.var_xmp_dn0 = assign15540_e22216_d_n0;
        locals.var_xmp_dn2 = assign15540_e22216_d_n2;
        locals.var_xmp_dn6 = assign15540_e22216_d_n6;
        locals.var_xmp_dn7 = assign15540_e22216_d_n7;
        locals.var_xmp_dn10 = assign15540_e22216_d_n10;
        locals.var_xmp_dn11 = assign15540_e22216_d_n11;
        locals.var_xmp_dn12 = assign15540_e22216_d_n12;
        locals.var_xmp_dn17 = assign15540_e22216_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15550_e22225,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15550_e22225;
        locals.var_m0_rv = 0.0;

        let (assign15560_e22234,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15560_e22234;
        locals.var_mm_rv = 0.0;

        let (assign15570_e22243, assign15570_e22243_d_n0, assign15570_e22243_d_n2, assign15570_e22243_d_n6, assign15570_e22243_d_n7, assign15570_e22243_d_n10, assign15570_e22243_d_n11, assign15570_e22243_d_n12, assign15570_e22243_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign15570_e22243;
        locals.var_arg_dn0 = assign15570_e22243_d_n0;
        locals.var_arg_dn2 = assign15570_e22243_d_n2;
        locals.var_arg_dn6 = assign15570_e22243_d_n6;
        locals.var_arg_dn7 = assign15570_e22243_d_n7;
        locals.var_arg_dn10 = assign15570_e22243_d_n10;
        locals.var_arg_dn11 = assign15570_e22243_d_n11;
        locals.var_arg_dn12 = assign15570_e22243_d_n12;
        locals.var_arg_dn17 = assign15570_e22243_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign15580_e22252, assign15580_e22252_d_n0, assign15580_e22252_d_n2, assign15580_e22252_d_n6, assign15580_e22252_d_n7, assign15580_e22252_d_n10, assign15580_e22252_d_n11, assign15580_e22252_d_n12, assign15580_e22252_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15580_e22252;
        locals.var_dnm_dn0 = assign15580_e22252_d_n0;
        locals.var_dnm_dn2 = assign15580_e22252_d_n2;
        locals.var_dnm_dn6 = assign15580_e22252_d_n6;
        locals.var_dnm_dn7 = assign15580_e22252_d_n7;
        locals.var_dnm_dn10 = assign15580_e22252_d_n10;
        locals.var_dnm_dn11 = assign15580_e22252_d_n11;
        locals.var_dnm_dn12 = assign15580_e22252_d_n12;
        locals.var_dnm_dn17 = assign15580_e22252_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15590_e22263, assign15590_e22263_d_n0, assign15590_e22263_d_n2, assign15590_e22263_d_n6, assign15590_e22263_d_n7, assign15590_e22263_d_n10, assign15590_e22263_d_n11, assign15590_e22263_d_n12, assign15590_e22263_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15590_e22261: f64 = (locals.var_xp * locals.var_x2);
        (assign15590_e22261, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15590_e22263;
        locals.var_xp_dn0 = assign15590_e22263_d_n0;
        locals.var_xp_dn2 = assign15590_e22263_d_n2;
        locals.var_xp_dn6 = assign15590_e22263_d_n6;
        locals.var_xp_dn7 = assign15590_e22263_d_n7;
        locals.var_xp_dn10 = assign15590_e22263_d_n10;
        locals.var_xp_dn11 = assign15590_e22263_d_n11;
        locals.var_xp_dn12 = assign15590_e22263_d_n12;
        locals.var_xp_dn17 = assign15590_e22263_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15600_e22274, assign15600_e22274_d_n0, assign15600_e22274_d_n2, assign15600_e22274_d_n6, assign15600_e22274_d_n7, assign15600_e22274_d_n10, assign15600_e22274_d_n11, assign15600_e22274_d_n12, assign15600_e22274_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15600_e22272: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15600_e22272, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15600_e22274;
        locals.var_xmp_dn0 = assign15600_e22274_d_n0;
        locals.var_xmp_dn2 = assign15600_e22274_d_n2;
        locals.var_xmp_dn6 = assign15600_e22274_d_n6;
        locals.var_xmp_dn7 = assign15600_e22274_d_n7;
        locals.var_xmp_dn10 = assign15600_e22274_d_n10;
        locals.var_xmp_dn11 = assign15600_e22274_d_n11;
        locals.var_xmp_dn12 = assign15600_e22274_d_n12;
        locals.var_xmp_dn17 = assign15600_e22274_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15610_e22285, assign15610_e22285_d_n0, assign15610_e22285_d_n2, assign15610_e22285_d_n6, assign15610_e22285_d_n7, assign15610_e22285_d_n10, assign15610_e22285_d_n11, assign15610_e22285_d_n12, assign15610_e22285_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15610_e22283: f64 = (locals.var_xp * locals.var_x2);
        (assign15610_e22283, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15610_e22285;
        locals.var_xp_dn0 = assign15610_e22285_d_n0;
        locals.var_xp_dn2 = assign15610_e22285_d_n2;
        locals.var_xp_dn6 = assign15610_e22285_d_n6;
        locals.var_xp_dn7 = assign15610_e22285_d_n7;
        locals.var_xp_dn10 = assign15610_e22285_d_n10;
        locals.var_xp_dn11 = assign15610_e22285_d_n11;
        locals.var_xp_dn12 = assign15610_e22285_d_n12;
        locals.var_xp_dn17 = assign15610_e22285_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15620_e22296, assign15620_e22296_d_n0, assign15620_e22296_d_n2, assign15620_e22296_d_n6, assign15620_e22296_d_n7, assign15620_e22296_d_n10, assign15620_e22296_d_n11, assign15620_e22296_d_n12, assign15620_e22296_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15620_e22294: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15620_e22294, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15620_e22296;
        locals.var_xmp_dn0 = assign15620_e22296_d_n0;
        locals.var_xmp_dn2 = assign15620_e22296_d_n2;
        locals.var_xmp_dn6 = assign15620_e22296_d_n6;
        locals.var_xmp_dn7 = assign15620_e22296_d_n7;
        locals.var_xmp_dn10 = assign15620_e22296_d_n10;
        locals.var_xmp_dn11 = assign15620_e22296_d_n11;
        locals.var_xmp_dn12 = assign15620_e22296_d_n12;
        locals.var_xmp_dn17 = assign15620_e22296_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15630_e22307, assign15630_e22307_d_n0, assign15630_e22307_d_n2, assign15630_e22307_d_n6, assign15630_e22307_d_n7, assign15630_e22307_d_n10, assign15630_e22307_d_n11, assign15630_e22307_d_n12, assign15630_e22307_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15630_e22305: f64 = (locals.var_xp * locals.var_x2);
        (assign15630_e22305, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15630_e22307;
        locals.var_xp_dn0 = assign15630_e22307_d_n0;
        locals.var_xp_dn2 = assign15630_e22307_d_n2;
        locals.var_xp_dn6 = assign15630_e22307_d_n6;
        locals.var_xp_dn7 = assign15630_e22307_d_n7;
        locals.var_xp_dn10 = assign15630_e22307_d_n10;
        locals.var_xp_dn11 = assign15630_e22307_d_n11;
        locals.var_xp_dn12 = assign15630_e22307_d_n12;
        locals.var_xp_dn17 = assign15630_e22307_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15640_e22318, assign15640_e22318_d_n0, assign15640_e22318_d_n2, assign15640_e22318_d_n6, assign15640_e22318_d_n7, assign15640_e22318_d_n10, assign15640_e22318_d_n11, assign15640_e22318_d_n12, assign15640_e22318_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15640_e22316: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15640_e22316, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15640_e22318;
        locals.var_xmp_dn0 = assign15640_e22318_d_n0;
        locals.var_xmp_dn2 = assign15640_e22318_d_n2;
        locals.var_xmp_dn6 = assign15640_e22318_d_n6;
        locals.var_xmp_dn7 = assign15640_e22318_d_n7;
        locals.var_xmp_dn10 = assign15640_e22318_d_n10;
        locals.var_xmp_dn11 = assign15640_e22318_d_n11;
        locals.var_xmp_dn12 = assign15640_e22318_d_n12;
        locals.var_xmp_dn17 = assign15640_e22318_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign15650_e22329, assign15650_e22329_d_n0, assign15650_e22329_d_n2, assign15650_e22329_d_n6, assign15650_e22329_d_n7, assign15650_e22329_d_n10, assign15650_e22329_d_n11, assign15650_e22329_d_n12, assign15650_e22329_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15650_e22327: f64 = (locals.var_xp * locals.var_x2);
        (assign15650_e22327, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign15650_e22329;
        locals.var_xp_dn0 = assign15650_e22329_d_n0;
        locals.var_xp_dn2 = assign15650_e22329_d_n2;
        locals.var_xp_dn6 = assign15650_e22329_d_n6;
        locals.var_xp_dn7 = assign15650_e22329_d_n7;
        locals.var_xp_dn10 = assign15650_e22329_d_n10;
        locals.var_xp_dn11 = assign15650_e22329_d_n11;
        locals.var_xp_dn12 = assign15650_e22329_d_n12;
        locals.var_xp_dn17 = assign15650_e22329_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign15660_e22340, assign15660_e22340_d_n0, assign15660_e22340_d_n2, assign15660_e22340_d_n6, assign15660_e22340_d_n7, assign15660_e22340_d_n10, assign15660_e22340_d_n11, assign15660_e22340_d_n12, assign15660_e22340_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15660_e22338: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15660_e22338, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign15660_e22340;
        locals.var_xmp_dn0 = assign15660_e22340_d_n0;
        locals.var_xmp_dn2 = assign15660_e22340_d_n2;
        locals.var_xmp_dn6 = assign15660_e22340_d_n6;
        locals.var_xmp_dn7 = assign15660_e22340_d_n7;
        locals.var_xmp_dn10 = assign15660_e22340_d_n10;
        locals.var_xmp_dn11 = assign15660_e22340_d_n11;
        locals.var_xmp_dn12 = assign15660_e22340_d_n12;
        locals.var_xmp_dn17 = assign15660_e22340_d_n17;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign15670_e22351, assign15670_e22351_d_n0, assign15670_e22351_d_n2, assign15670_e22351_d_n6, assign15670_e22351_d_n7, assign15670_e22351_d_n10, assign15670_e22351_d_n11, assign15670_e22351_d_n12, assign15670_e22351_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15670_e22349: f64 = (locals.var_xp + locals.var_xmp);
        (assign15670_e22349, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign15670_e22351;
        locals.var_arg_dn0 = assign15670_e22351_d_n0;
        locals.var_arg_dn2 = assign15670_e22351_d_n2;
        locals.var_arg_dn6 = assign15670_e22351_d_n6;
        locals.var_arg_dn7 = assign15670_e22351_d_n7;
        locals.var_arg_dn10 = assign15670_e22351_d_n10;
        locals.var_arg_dn11 = assign15670_e22351_d_n11;
        locals.var_arg_dn12 = assign15670_e22351_d_n12;
        locals.var_arg_dn17 = assign15670_e22351_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign15680_e22360, assign15680_e22360_d_n0, assign15680_e22360_d_n2, assign15680_e22360_d_n6, assign15680_e22360_d_n7, assign15680_e22360_d_n10, assign15680_e22360_d_n11, assign15680_e22360_d_n12, assign15680_e22360_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15680_e22360;
        locals.var_dnm_dn0 = assign15680_e22360_d_n0;
        locals.var_dnm_dn2 = assign15680_e22360_d_n2;
        locals.var_dnm_dn6 = assign15680_e22360_d_n6;
        locals.var_dnm_dn7 = assign15680_e22360_d_n7;
        locals.var_dnm_dn10 = assign15680_e22360_d_n10;
        locals.var_dnm_dn11 = assign15680_e22360_d_n11;
        locals.var_dnm_dn12 = assign15680_e22360_d_n12;
        locals.var_dnm_dn17 = assign15680_e22360_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign15690_e22375: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard474 = assign15690_e22375;
        locals.var_guard474_rv = 0.0;

        let assign15700_e22378: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign15700_e22378;
        locals.var_guard475_rv = 0.0;

        let (assign15710_e22391,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15710_e22391;
        locals.var_mm_rv = 0.0;

        let assign15720_e22394: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign15720_e22394;
        locals.var_guard476_rv = 0.0;

        let (assign15730_e22410,) = {
    if ((((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 == 0.0)) && (locals.var_guard476 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15730_e22410;
        locals.var_mm_rv = 0.0;

        let assign15740_e22413: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign15740_e22413;
        locals.var_guard477_rv = 0.0;

        let (assign15750_e22432,) = {
    if (((((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 == 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard477 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15750_e22432;
        locals.var_mm_rv = 0.0;

        let assign15760_e22435: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign15760_e22435;
        locals.var_guard478_rv = 0.0;

        let (assign15770_e22457,) = {
    if ((((((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 == 0.0)) && (locals.var_guard476 == 0.0)) && (locals.var_guard477 == 0.0)) && (locals.var_guard478 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15770_e22457;
        locals.var_mm_rv = 0.0;

        let (assign15780_e22468,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15780_e22468;
        locals.var_m0_rv = 0.0;

        let mut assign15790_loop_guard: usize = 0;
        while {
            let assign15790_cond_e22480: f64 = if (((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign15790_cond_e22480 != 0.0
        } {
            assign15790_loop_guard += 1;
            assert!(assign15790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15790_body0_e22492, assign15790_body0_e22492_d_n0, assign15790_body0_e22492_d_n2, assign15790_body0_e22492_d_n6, assign15790_body0_e22492_d_n7, assign15790_body0_e22492_d_n10, assign15790_body0_e22492_d_n11, assign15790_body0_e22492_d_n12, assign15790_body0_e22492_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign15790_body0_e22490: f64 = (locals.var_dnm).sqrt();
        (assign15790_body0_e22490, (locals.var_dnm_dn0 / (2.0 * assign15790_body0_e22490)), (locals.var_dnm_dn2 / (2.0 * assign15790_body0_e22490)), (locals.var_dnm_dn6 / (2.0 * assign15790_body0_e22490)), (locals.var_dnm_dn7 / (2.0 * assign15790_body0_e22490)), (locals.var_dnm_dn10 / (2.0 * assign15790_body0_e22490)), (locals.var_dnm_dn11 / (2.0 * assign15790_body0_e22490)), (locals.var_dnm_dn12 / (2.0 * assign15790_body0_e22490)), (locals.var_dnm_dn17 / (2.0 * assign15790_body0_e22490)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign15790_body0_e22492;
            locals.var_dnm_dn0 = assign15790_body0_e22492_d_n0;
            locals.var_dnm_dn2 = assign15790_body0_e22492_d_n2;
            locals.var_dnm_dn6 = assign15790_body0_e22492_d_n6;
            locals.var_dnm_dn7 = assign15790_body0_e22492_d_n7;
            locals.var_dnm_dn10 = assign15790_body0_e22492_d_n10;
            locals.var_dnm_dn11 = assign15790_body0_e22492_d_n11;
            locals.var_dnm_dn12 = assign15790_body0_e22492_d_n12;
            locals.var_dnm_dn17 = assign15790_body0_e22492_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign15790_body1_e22505,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign15790_body1_e22503: f64 = (locals.var_m0 + 1.0);
        (assign15790_body1_e22503,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign15790_body1_e22505;
            locals.var_m0_rv = 0.0;
        }

        let (assign15800_e22523, assign15800_e22523_d_n0, assign15800_e22523_d_n2, assign15800_e22523_d_n6, assign15800_e22523_d_n7, assign15800_e22523_d_n10, assign15800_e22523_d_n11, assign15800_e22523_d_n12, assign15800_e22523_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) && (locals.var_guard474 == 0.0)) {
        let assign15800_e22519: f64 = (2.0 * 4.0);
        let assign15800_e22520: f64 = (1.0 / assign15800_e22519);
        let assign15800_e22521: f64 = (locals.var_dnm).powf(assign15800_e22520);
        (assign15800_e22521, if 0.0 == 0.0 && ((assign15800_e22520) as f64).is_finite() && ((assign15800_e22520) as f64).fract() == 0.0 { if assign15800_e22520 == 0.0 { 0.0 } else { (assign15800_e22520 * ((locals.var_dnm).powf(assign15800_e22520 - 1.0) * locals.var_dnm_dn0)) } } else { (assign15800_e22521 * (assign15800_e22520 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15800_e22520) as f64).is_finite() && ((assign15800_e22520) as f64).fract() == 0.0 { if assign15800_e22520 == 0.0 { 0.0 } else { (assign15800_e22520 * ((locals.var_dnm).powf(assign15800_e22520 - 1.0) * locals.var_dnm_dn2)) } } else { (assign15800_e22521 * (assign15800_e22520 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15800_e22520) as f64).is_finite() && ((assign15800_e22520) as f64).fract() == 0.0 { if assign15800_e22520 == 0.0 { 0.0 } else { (assign15800_e22520 * ((locals.var_dnm).powf(assign15800_e22520 - 1.0) * locals.var_dnm_dn6)) } } else { (assign15800_e22521 * (assign15800_e22520 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15800_e22520) as f64).is_finite() && ((assign15800_e22520) as f64).fract() == 0.0 { if assign15800_e22520 == 0.0 { 0.0 } else { (assign15800_e22520 * ((locals.var_dnm).powf(assign15800_e22520 - 1.0) * locals.var_dnm_dn7)) } } else { (assign15800_e22521 * (assign15800_e22520 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15800_e22520) as f64).is_finite() && ((assign15800_e22520) as f64).fract() == 0.0 { if assign15800_e22520 == 0.0 { 0.0 } else { (assign15800_e22520 * ((locals.var_dnm).powf(assign15800_e22520 - 1.0) * locals.var_dnm_dn10)) } } else { (assign15800_e22521 * (assign15800_e22520 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15800_e22520) as f64).is_finite() && ((assign15800_e22520) as f64).fract() == 0.0 { if assign15800_e22520 == 0.0 { 0.0 } else { (assign15800_e22520 * ((locals.var_dnm).powf(assign15800_e22520 - 1.0) * locals.var_dnm_dn11)) } } else { (assign15800_e22521 * (assign15800_e22520 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15800_e22520) as f64).is_finite() && ((assign15800_e22520) as f64).fract() == 0.0 { if assign15800_e22520 == 0.0 { 0.0 } else { (assign15800_e22520 * ((locals.var_dnm).powf(assign15800_e22520 - 1.0) * locals.var_dnm_dn12)) } } else { (assign15800_e22521 * (assign15800_e22520 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15800_e22520) as f64).is_finite() && ((assign15800_e22520) as f64).fract() == 0.0 { if assign15800_e22520 == 0.0 { 0.0 } else { (assign15800_e22520 * ((locals.var_dnm).powf(assign15800_e22520 - 1.0) * locals.var_dnm_dn17)) } } else { (assign15800_e22521 * (assign15800_e22520 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15800_e22523;
        locals.var_dnm_dn0 = assign15800_e22523_d_n0;
        locals.var_dnm_dn2 = assign15800_e22523_d_n2;
        locals.var_dnm_dn6 = assign15800_e22523_d_n6;
        locals.var_dnm_dn7 = assign15800_e22523_d_n7;
        locals.var_dnm_dn10 = assign15800_e22523_d_n10;
        locals.var_dnm_dn11 = assign15800_e22523_d_n11;
        locals.var_dnm_dn12 = assign15800_e22523_d_n12;
        locals.var_dnm_dn17 = assign15800_e22523_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15810_e22534, assign15810_e22534_d_n0, assign15810_e22534_d_n2, assign15810_e22534_d_n6, assign15810_e22534_d_n7, assign15810_e22534_d_n10, assign15810_e22534_d_n11, assign15810_e22534_d_n12, assign15810_e22534_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15810_e22532: f64 = (1.0 / locals.var_dnm);
        (assign15810_e22532, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign15810_e22534;
        locals.var_dnm_dn0 = assign15810_e22534_d_n0;
        locals.var_dnm_dn2 = assign15810_e22534_d_n2;
        locals.var_dnm_dn6 = assign15810_e22534_d_n6;
        locals.var_dnm_dn7 = assign15810_e22534_d_n7;
        locals.var_dnm_dn10 = assign15810_e22534_d_n10;
        locals.var_dnm_dn11 = assign15810_e22534_d_n11;
        locals.var_dnm_dn12 = assign15810_e22534_d_n12;
        locals.var_dnm_dn17 = assign15810_e22534_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign15820_e22547, assign15820_e22547_d_n0, assign15820_e22547_d_n2, assign15820_e22547_d_n6, assign15820_e22547_d_n7, assign15820_e22547_d_n10, assign15820_e22547_d_n11, assign15820_e22547_d_n12, assign15820_e22547_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15820_e22543: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign15820_e22545: f64 = (assign15820_e22543 * locals.var_dnm);
        (assign15820_e22545, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign15820_e22543 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign15820_e22543 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign15820_e22543 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign15820_e22543 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign15820_e22543 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign15820_e22543 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn12)) * locals.var_dnm) + (assign15820_e22543 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn17)) * locals.var_dnm) + (assign15820_e22543 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign15820_e22547;
        locals.var_tmf0_dn0 = assign15820_e22547_d_n0;
        locals.var_tmf0_dn2 = assign15820_e22547_d_n2;
        locals.var_tmf0_dn6 = assign15820_e22547_d_n6;
        locals.var_tmf0_dn7 = assign15820_e22547_d_n7;
        locals.var_tmf0_dn10 = assign15820_e22547_d_n10;
        locals.var_tmf0_dn11 = assign15820_e22547_d_n11;
        locals.var_tmf0_dn12 = assign15820_e22547_d_n12;
        locals.var_tmf0_dn17 = assign15820_e22547_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign15830_e22560, assign15830_e22560_d_n0, assign15830_e22560_d_n2, assign15830_e22560_d_n6, assign15830_e22560_d_n7, assign15830_e22560_d_n10, assign15830_e22560_d_n11, assign15830_e22560_d_n12, assign15830_e22560_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 != 0.0)) {
        let assign15830_e22556: f64 = locals.var_t5;
        let assign15830_e22558: f64 = (assign15830_e22556 - locals.var_tmf0);
        (assign15830_e22558, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn12 - locals.var_tmf0_dn12), (locals.var_t5_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t4__blk469, locals.var_t4__blk469_dn0, locals.var_t4__blk469_dn2, locals.var_t4__blk469_dn6, locals.var_t4__blk469_dn7, locals.var_t4__blk469_dn10, locals.var_t4__blk469_dn11, locals.var_t4__blk469_dn12, locals.var_t4__blk469_dn17,)
    }
};
        locals.var_t4__blk469 = assign15830_e22560;
        locals.var_t4__blk469_dn0 = assign15830_e22560_d_n0;
        locals.var_t4__blk469_dn2 = assign15830_e22560_d_n2;
        locals.var_t4__blk469_dn6 = assign15830_e22560_d_n6;
        locals.var_t4__blk469_dn7 = assign15830_e22560_d_n7;
        locals.var_t4__blk469_dn10 = assign15830_e22560_d_n10;
        locals.var_t4__blk469_dn11 = assign15830_e22560_d_n11;
        locals.var_t4__blk469_dn12 = assign15830_e22560_d_n12;
        locals.var_t4__blk469_dn17 = assign15830_e22560_d_n17;
        locals.var_t4__blk469_rv = 0.0;

        let (assign15840_e22570, assign15840_e22570_d_n0, assign15840_e22570_d_n2, assign15840_e22570_d_n6, assign15840_e22570_d_n7, assign15840_e22570_d_n10, assign15840_e22570_d_n11, assign15840_e22570_d_n12, assign15840_e22570_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard473 == 0.0)) {
        (locals.var_t4__blk469, locals.var_t4__blk469_dn0, locals.var_t4__blk469_dn2, locals.var_t4__blk469_dn6, locals.var_t4__blk469_dn7, locals.var_t4__blk469_dn10, locals.var_t4__blk469_dn11, locals.var_t4__blk469_dn12, locals.var_t4__blk469_dn17,)
    } else {
        (locals.var_t4__blk469, locals.var_t4__blk469_dn0, locals.var_t4__blk469_dn2, locals.var_t4__blk469_dn6, locals.var_t4__blk469_dn7, locals.var_t4__blk469_dn10, locals.var_t4__blk469_dn11, locals.var_t4__blk469_dn12, locals.var_t4__blk469_dn17,)
    }
};
        locals.var_t4__blk469 = assign15840_e22570;
        locals.var_t4__blk469_dn0 = assign15840_e22570_d_n0;
        locals.var_t4__blk469_dn2 = assign15840_e22570_d_n2;
        locals.var_t4__blk469_dn6 = assign15840_e22570_d_n6;
        locals.var_t4__blk469_dn7 = assign15840_e22570_d_n7;
        locals.var_t4__blk469_dn10 = assign15840_e22570_d_n10;
        locals.var_t4__blk469_dn11 = assign15840_e22570_d_n11;
        locals.var_t4__blk469_dn12 = assign15840_e22570_d_n12;
        locals.var_t4__blk469_dn17 = assign15840_e22570_d_n17;
        locals.var_t4__blk469_rv = 0.0;

        let (assign15850_e22578, assign15850_e22578_d_n0, assign15850_e22578_d_n2, assign15850_e22578_d_n6, assign15850_e22578_d_n7, assign15850_e22578_d_n10, assign15850_e22578_d_n11, assign15850_e22578_d_n12, assign15850_e22578_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15850_e22576: f64 = (locals.var_t4__blk469).sqrt();
        (assign15850_e22576, (locals.var_t4__blk469_dn0 / (2.0 * assign15850_e22576)), (locals.var_t4__blk469_dn2 / (2.0 * assign15850_e22576)), (locals.var_t4__blk469_dn6 / (2.0 * assign15850_e22576)), (locals.var_t4__blk469_dn7 / (2.0 * assign15850_e22576)), (locals.var_t4__blk469_dn10 / (2.0 * assign15850_e22576)), (locals.var_t4__blk469_dn11 / (2.0 * assign15850_e22576)), (locals.var_t4__blk469_dn12 / (2.0 * assign15850_e22576)), (locals.var_t4__blk469_dn17 / (2.0 * assign15850_e22576)),)
    } else {
        (locals.var_t3__blk468, locals.var_t3__blk468_dn0, locals.var_t3__blk468_dn2, locals.var_t3__blk468_dn6, locals.var_t3__blk468_dn7, locals.var_t3__blk468_dn10, locals.var_t3__blk468_dn11, locals.var_t3__blk468_dn12, locals.var_t3__blk468_dn17,)
    }
};
        locals.var_t3__blk468 = assign15850_e22578;
        locals.var_t3__blk468_dn0 = assign15850_e22578_d_n0;
        locals.var_t3__blk468_dn2 = assign15850_e22578_d_n2;
        locals.var_t3__blk468_dn6 = assign15850_e22578_d_n6;
        locals.var_t3__blk468_dn7 = assign15850_e22578_d_n7;
        locals.var_t3__blk468_dn10 = assign15850_e22578_d_n10;
        locals.var_t3__blk468_dn11 = assign15850_e22578_d_n11;
        locals.var_t3__blk468_dn12 = assign15850_e22578_d_n12;
        locals.var_t3__blk468_dn17 = assign15850_e22578_d_n17;
        locals.var_t3__blk468_rv = 0.0;

        let (assign15860_e22591, assign15860_e22591_d_n0, assign15860_e22591_d_n2, assign15860_e22591_d_n6, assign15860_e22591_d_n7, assign15860_e22591_d_n10, assign15860_e22591_d_n11, assign15860_e22591_d_n12, assign15860_e22591_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15860_e22587: f64 = (1.0 - locals.var_t3__blk468);
        let assign15860_e22588: f64 = (locals.var_t2__blk467 * assign15860_e22587);
        let assign15860_e22589: f64 = (locals.var_vgp + assign15860_e22588);
        (assign15860_e22589, (locals.var_vgp_dn0 + ((locals.var_t2__blk467_dn0 * assign15860_e22587) + (locals.var_t2__blk467 * (-locals.var_t3__blk468_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2__blk467_dn2 * assign15860_e22587) + (locals.var_t2__blk467 * (-locals.var_t3__blk468_dn2)))), (locals.var_vgp_dn6 + ((locals.var_t2__blk467_dn6 * assign15860_e22587) + (locals.var_t2__blk467 * (-locals.var_t3__blk468_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2__blk467_dn7 * assign15860_e22587) + (locals.var_t2__blk467 * (-locals.var_t3__blk468_dn7)))), (locals.var_vgp_dn10 + ((locals.var_t2__blk467_dn10 * assign15860_e22587) + (locals.var_t2__blk467 * (-locals.var_t3__blk468_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2__blk467_dn11 * assign15860_e22587) + (locals.var_t2__blk467 * (-locals.var_t3__blk468_dn11)))), (locals.var_vgp_dn12 + ((locals.var_t2__blk467_dn12 * assign15860_e22587) + (locals.var_t2__blk467 * (-locals.var_t3__blk468_dn12)))), (locals.var_vgp_dn17 + ((locals.var_t2__blk467_dn17 * assign15860_e22587) + (locals.var_t2__blk467 * (-locals.var_t3__blk468_dn17)))),)
    } else {
        (locals.var_t10__blk472, locals.var_t10__blk472_dn0, locals.var_t10__blk472_dn2, locals.var_t10__blk472_dn6, locals.var_t10__blk472_dn7, locals.var_t10__blk472_dn10, locals.var_t10__blk472_dn11, locals.var_t10__blk472_dn12, locals.var_t10__blk472_dn17,)
    }
};
        locals.var_t10__blk472 = assign15860_e22591;
        locals.var_t10__blk472_dn0 = assign15860_e22591_d_n0;
        locals.var_t10__blk472_dn2 = assign15860_e22591_d_n2;
        locals.var_t10__blk472_dn6 = assign15860_e22591_d_n6;
        locals.var_t10__blk472_dn7 = assign15860_e22591_d_n7;
        locals.var_t10__blk472_dn10 = assign15860_e22591_d_n10;
        locals.var_t10__blk472_dn11 = assign15860_e22591_d_n11;
        locals.var_t10__blk472_dn12 = assign15860_e22591_d_n12;
        locals.var_t10__blk472_dn17 = assign15860_e22591_d_n17;
        locals.var_t10__blk472_rv = 0.0;

        let (assign15870_e22607, assign15870_e22607_d_n0, assign15870_e22607_d_n2, assign15870_e22607_d_n6, assign15870_e22607_d_n7, assign15870_e22607_d_n10, assign15870_e22607_d_n11, assign15870_e22607_d_n12, assign15870_e22607_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15870_e22598: f64 = (locals.var_t10__blk472 * locals.var_t10__blk472);
        let assign15870_e22601: f64 = (4.0 * 0.01);
        let assign15870_e22603: f64 = (assign15870_e22601 * 0.01);
        let assign15870_e22604: f64 = (assign15870_e22598 + assign15870_e22603);
        let assign15870_e22605: f64 = (assign15870_e22604).sqrt();
        (assign15870_e22605, (((locals.var_t10__blk472_dn0 * locals.var_t10__blk472) + (locals.var_t10__blk472 * locals.var_t10__blk472_dn0)) / (2.0 * assign15870_e22605)), (((locals.var_t10__blk472_dn2 * locals.var_t10__blk472) + (locals.var_t10__blk472 * locals.var_t10__blk472_dn2)) / (2.0 * assign15870_e22605)), (((locals.var_t10__blk472_dn6 * locals.var_t10__blk472) + (locals.var_t10__blk472 * locals.var_t10__blk472_dn6)) / (2.0 * assign15870_e22605)), (((locals.var_t10__blk472_dn7 * locals.var_t10__blk472) + (locals.var_t10__blk472 * locals.var_t10__blk472_dn7)) / (2.0 * assign15870_e22605)), (((locals.var_t10__blk472_dn10 * locals.var_t10__blk472) + (locals.var_t10__blk472 * locals.var_t10__blk472_dn10)) / (2.0 * assign15870_e22605)), (((locals.var_t10__blk472_dn11 * locals.var_t10__blk472) + (locals.var_t10__blk472 * locals.var_t10__blk472_dn11)) / (2.0 * assign15870_e22605)), (((locals.var_t10__blk472_dn12 * locals.var_t10__blk472) + (locals.var_t10__blk472 * locals.var_t10__blk472_dn12)) / (2.0 * assign15870_e22605)), (((locals.var_t10__blk472_dn17 * locals.var_t10__blk472) + (locals.var_t10__blk472 * locals.var_t10__blk472_dn17)) / (2.0 * assign15870_e22605)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign15870_e22607;
        locals.var_tmf1_dn0 = assign15870_e22607_d_n0;
        locals.var_tmf1_dn2 = assign15870_e22607_d_n2;
        locals.var_tmf1_dn6 = assign15870_e22607_d_n6;
        locals.var_tmf1_dn7 = assign15870_e22607_d_n7;
        locals.var_tmf1_dn10 = assign15870_e22607_d_n10;
        locals.var_tmf1_dn11 = assign15870_e22607_d_n11;
        locals.var_tmf1_dn12 = assign15870_e22607_d_n12;
        locals.var_tmf1_dn17 = assign15870_e22607_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign15880_e22622, assign15880_e22622_d_n0, assign15880_e22622_d_n2, assign15880_e22622_d_n6, assign15880_e22622_d_n7, assign15880_e22622_d_n10, assign15880_e22622_d_n11, assign15880_e22622_d_n12, assign15880_e22622_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15880_e22615: f64 = (locals.var_t10__blk472 + locals.var_tmf1);
        let assign15880_e22616: f64 = (0.5 * assign15880_e22615);
        let assign15880_e22619: f64 = (1e-10 * 0.01);
        let assign15880_e22620: f64 = (assign15880_e22616 + assign15880_e22619);
        (assign15880_e22620, (0.5 * (locals.var_t10__blk472_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t10__blk472_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t10__blk472_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t10__blk472_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t10__blk472_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t10__blk472_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t10__blk472_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t10__blk472_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t10__blk472, locals.var_t10__blk472_dn0, locals.var_t10__blk472_dn2, locals.var_t10__blk472_dn6, locals.var_t10__blk472_dn7, locals.var_t10__blk472_dn10, locals.var_t10__blk472_dn11, locals.var_t10__blk472_dn12, locals.var_t10__blk472_dn17,)
    }
};
        locals.var_t10__blk472 = assign15880_e22622;
        locals.var_t10__blk472_dn0 = assign15880_e22622_d_n0;
        locals.var_t10__blk472_dn2 = assign15880_e22622_d_n2;
        locals.var_t10__blk472_dn6 = assign15880_e22622_d_n6;
        locals.var_t10__blk472_dn7 = assign15880_e22622_d_n7;
        locals.var_t10__blk472_dn10 = assign15880_e22622_d_n10;
        locals.var_t10__blk472_dn11 = assign15880_e22622_d_n11;
        locals.var_t10__blk472_dn12 = assign15880_e22622_d_n12;
        locals.var_t10__blk472_dn17 = assign15880_e22622_d_n17;
        locals.var_t10__blk472_rv = 0.0;

        let assign15890_e22625: f64 = if locals.var_t10__blk472 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign15890_e22625;
        locals.var_guard479_rv = 0.0;

        let (assign15900_e22634, assign15900_e22634_d_n0, assign15900_e22634_d_n2, assign15900_e22634_d_n6, assign15900_e22634_d_n7, assign15900_e22634_d_n10, assign15900_e22634_d_n11, assign15900_e22634_d_n12, assign15900_e22634_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard479 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10__blk472, locals.var_t10__blk472_dn0, locals.var_t10__blk472_dn2, locals.var_t10__blk472_dn6, locals.var_t10__blk472_dn7, locals.var_t10__blk472_dn10, locals.var_t10__blk472_dn11, locals.var_t10__blk472_dn12, locals.var_t10__blk472_dn17,)
    }
};
        locals.var_t10__blk472 = assign15900_e22634;
        locals.var_t10__blk472_dn0 = assign15900_e22634_d_n0;
        locals.var_t10__blk472_dn2 = assign15900_e22634_d_n2;
        locals.var_t10__blk472_dn6 = assign15900_e22634_d_n6;
        locals.var_t10__blk472_dn7 = assign15900_e22634_d_n7;
        locals.var_t10__blk472_dn10 = assign15900_e22634_d_n10;
        locals.var_t10__blk472_dn11 = assign15900_e22634_d_n11;
        locals.var_t10__blk472_dn12 = assign15900_e22634_d_n12;
        locals.var_t10__blk472_dn17 = assign15900_e22634_d_n17;
        locals.var_t10__blk472_rv = 0.0;

        let (assign15920_e22650, assign15920_e22650_d_n0, assign15920_e22650_d_n2, assign15920_e22650_d_n6, assign15920_e22650_d_n7, assign15920_e22650_d_n10, assign15920_e22650_d_n11, assign15920_e22650_d_n12, assign15920_e22650_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15920_e22648: f64 = (locals.var_vds / locals.var_t10__blk472);
        (assign15920_e22648, (((locals.var_vds_dn0 * locals.var_t10__blk472) - (locals.var_vds * locals.var_t10__blk472_dn0)) / (locals.var_t10__blk472 * locals.var_t10__blk472)), (((locals.var_vds_dn2 * locals.var_t10__blk472) - (locals.var_vds * locals.var_t10__blk472_dn2)) / (locals.var_t10__blk472 * locals.var_t10__blk472)), (((locals.var_vds_dn6 * locals.var_t10__blk472) - (locals.var_vds * locals.var_t10__blk472_dn6)) / (locals.var_t10__blk472 * locals.var_t10__blk472)), (((locals.var_vds_dn7 * locals.var_t10__blk472) - (locals.var_vds * locals.var_t10__blk472_dn7)) / (locals.var_t10__blk472 * locals.var_t10__blk472)), (((locals.var_vds_dn10 * locals.var_t10__blk472) - (locals.var_vds * locals.var_t10__blk472_dn10)) / (locals.var_t10__blk472 * locals.var_t10__blk472)), (((locals.var_vds_dn11 * locals.var_t10__blk472) - (locals.var_vds * locals.var_t10__blk472_dn11)) / (locals.var_t10__blk472 * locals.var_t10__blk472)), (((locals.var_vds_dn12 * locals.var_t10__blk472) - (locals.var_vds * locals.var_t10__blk472_dn12)) / (locals.var_t10__blk472 * locals.var_t10__blk472)), (((locals.var_vds_dn17 * locals.var_t10__blk472) - (locals.var_vds * locals.var_t10__blk472_dn17)) / (locals.var_t10__blk472 * locals.var_t10__blk472)),)
    } else {
        (locals.var_t1__blk466, locals.var_t1__blk466_dn0, locals.var_t1__blk466_dn2, locals.var_t1__blk466_dn6, locals.var_t1__blk466_dn7, locals.var_t1__blk466_dn10, locals.var_t1__blk466_dn11, locals.var_t1__blk466_dn12, locals.var_t1__blk466_dn17,)
    }
};
        locals.var_t1__blk466 = assign15920_e22650;
        locals.var_t1__blk466_dn0 = assign15920_e22650_d_n0;
        locals.var_t1__blk466_dn2 = assign15920_e22650_d_n2;
        locals.var_t1__blk466_dn6 = assign15920_e22650_d_n6;
        locals.var_t1__blk466_dn7 = assign15920_e22650_d_n7;
        locals.var_t1__blk466_dn10 = assign15920_e22650_d_n10;
        locals.var_t1__blk466_dn11 = assign15920_e22650_d_n11;
        locals.var_t1__blk466_dn12 = assign15920_e22650_d_n12;
        locals.var_t1__blk466_dn17 = assign15920_e22650_d_n17;
        locals.var_t1__blk466_rv = 0.0;

        let (assign15930_e22661, assign15930_e22661_d_n0, assign15930_e22661_d_n2, assign15930_e22661_d_n6, assign15930_e22661_d_n7, assign15930_e22661_d_n10, assign15930_e22661_d_n11, assign15930_e22661_d_n12, assign15930_e22661_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15930_e22658: f64 = (locals.var_ddlte - 1.0);
        let assign15930_e22659: f64 = (locals.var_t1__blk466).powf(assign15930_e22658);
        (assign15930_e22659, if 0.0 == 0.0 && ((assign15930_e22658) as f64).is_finite() && ((assign15930_e22658) as f64).fract() == 0.0 { if assign15930_e22658 == 0.0 { 0.0 } else { (assign15930_e22658 * ((locals.var_t1__blk466).powf(assign15930_e22658 - 1.0) * locals.var_t1__blk466_dn0)) } } else { (assign15930_e22659 * (assign15930_e22658 * (locals.var_t1__blk466_dn0 / locals.var_t1__blk466))) }, if 0.0 == 0.0 && ((assign15930_e22658) as f64).is_finite() && ((assign15930_e22658) as f64).fract() == 0.0 { if assign15930_e22658 == 0.0 { 0.0 } else { (assign15930_e22658 * ((locals.var_t1__blk466).powf(assign15930_e22658 - 1.0) * locals.var_t1__blk466_dn2)) } } else { (assign15930_e22659 * (assign15930_e22658 * (locals.var_t1__blk466_dn2 / locals.var_t1__blk466))) }, if 0.0 == 0.0 && ((assign15930_e22658) as f64).is_finite() && ((assign15930_e22658) as f64).fract() == 0.0 { if assign15930_e22658 == 0.0 { 0.0 } else { (assign15930_e22658 * ((locals.var_t1__blk466).powf(assign15930_e22658 - 1.0) * locals.var_t1__blk466_dn6)) } } else { (assign15930_e22659 * (assign15930_e22658 * (locals.var_t1__blk466_dn6 / locals.var_t1__blk466))) }, if 0.0 == 0.0 && ((assign15930_e22658) as f64).is_finite() && ((assign15930_e22658) as f64).fract() == 0.0 { if assign15930_e22658 == 0.0 { 0.0 } else { (assign15930_e22658 * ((locals.var_t1__blk466).powf(assign15930_e22658 - 1.0) * locals.var_t1__blk466_dn7)) } } else { (assign15930_e22659 * (assign15930_e22658 * (locals.var_t1__blk466_dn7 / locals.var_t1__blk466))) }, if 0.0 == 0.0 && ((assign15930_e22658) as f64).is_finite() && ((assign15930_e22658) as f64).fract() == 0.0 { if assign15930_e22658 == 0.0 { 0.0 } else { (assign15930_e22658 * ((locals.var_t1__blk466).powf(assign15930_e22658 - 1.0) * locals.var_t1__blk466_dn10)) } } else { (assign15930_e22659 * (assign15930_e22658 * (locals.var_t1__blk466_dn10 / locals.var_t1__blk466))) }, if 0.0 == 0.0 && ((assign15930_e22658) as f64).is_finite() && ((assign15930_e22658) as f64).fract() == 0.0 { if assign15930_e22658 == 0.0 { 0.0 } else { (assign15930_e22658 * ((locals.var_t1__blk466).powf(assign15930_e22658 - 1.0) * locals.var_t1__blk466_dn11)) } } else { (assign15930_e22659 * (assign15930_e22658 * (locals.var_t1__blk466_dn11 / locals.var_t1__blk466))) }, if 0.0 == 0.0 && ((assign15930_e22658) as f64).is_finite() && ((assign15930_e22658) as f64).fract() == 0.0 { if assign15930_e22658 == 0.0 { 0.0 } else { (assign15930_e22658 * ((locals.var_t1__blk466).powf(assign15930_e22658 - 1.0) * locals.var_t1__blk466_dn12)) } } else { (assign15930_e22659 * (assign15930_e22658 * (locals.var_t1__blk466_dn12 / locals.var_t1__blk466))) }, if 0.0 == 0.0 && ((assign15930_e22658) as f64).is_finite() && ((assign15930_e22658) as f64).fract() == 0.0 { if assign15930_e22658 == 0.0 { 0.0 } else { (assign15930_e22658 * ((locals.var_t1__blk466).powf(assign15930_e22658 - 1.0) * locals.var_t1__blk466_dn17)) } } else { (assign15930_e22659 * (assign15930_e22658 * (locals.var_t1__blk466_dn17 / locals.var_t1__blk466))) },)
    } else {
        (locals.var_t2__blk467, locals.var_t2__blk467_dn0, locals.var_t2__blk467_dn2, locals.var_t2__blk467_dn6, locals.var_t2__blk467_dn7, locals.var_t2__blk467_dn10, locals.var_t2__blk467_dn11, locals.var_t2__blk467_dn12, locals.var_t2__blk467_dn17,)
    }
};
        locals.var_t2__blk467 = assign15930_e22661;
        locals.var_t2__blk467_dn0 = assign15930_e22661_d_n0;
        locals.var_t2__blk467_dn2 = assign15930_e22661_d_n2;
        locals.var_t2__blk467_dn6 = assign15930_e22661_d_n6;
        locals.var_t2__blk467_dn7 = assign15930_e22661_d_n7;
        locals.var_t2__blk467_dn10 = assign15930_e22661_d_n10;
        locals.var_t2__blk467_dn11 = assign15930_e22661_d_n11;
        locals.var_t2__blk467_dn12 = assign15930_e22661_d_n12;
        locals.var_t2__blk467_dn17 = assign15930_e22661_d_n17;
        locals.var_t2__blk467_rv = 0.0;

        let (assign15940_e22670, assign15940_e22670_d_n0, assign15940_e22670_d_n2, assign15940_e22670_d_n6, assign15940_e22670_d_n7, assign15940_e22670_d_n10, assign15940_e22670_d_n11, assign15940_e22670_d_n12, assign15940_e22670_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15940_e22668: f64 = (locals.var_t2__blk467 * locals.var_t1__blk466);
        (assign15940_e22668, ((locals.var_t2__blk467_dn0 * locals.var_t1__blk466) + (locals.var_t2__blk467 * locals.var_t1__blk466_dn0)), ((locals.var_t2__blk467_dn2 * locals.var_t1__blk466) + (locals.var_t2__blk467 * locals.var_t1__blk466_dn2)), ((locals.var_t2__blk467_dn6 * locals.var_t1__blk466) + (locals.var_t2__blk467 * locals.var_t1__blk466_dn6)), ((locals.var_t2__blk467_dn7 * locals.var_t1__blk466) + (locals.var_t2__blk467 * locals.var_t1__blk466_dn7)), ((locals.var_t2__blk467_dn10 * locals.var_t1__blk466) + (locals.var_t2__blk467 * locals.var_t1__blk466_dn10)), ((locals.var_t2__blk467_dn11 * locals.var_t1__blk466) + (locals.var_t2__blk467 * locals.var_t1__blk466_dn11)), ((locals.var_t2__blk467_dn12 * locals.var_t1__blk466) + (locals.var_t2__blk467 * locals.var_t1__blk466_dn12)), ((locals.var_t2__blk467_dn17 * locals.var_t1__blk466) + (locals.var_t2__blk467 * locals.var_t1__blk466_dn17)),)
    } else {
        (locals.var_t7__blk471, locals.var_t7__blk471_dn0, locals.var_t7__blk471_dn2, locals.var_t7__blk471_dn6, locals.var_t7__blk471_dn7, locals.var_t7__blk471_dn10, locals.var_t7__blk471_dn11, locals.var_t7__blk471_dn12, locals.var_t7__blk471_dn17,)
    }
};
        locals.var_t7__blk471 = assign15940_e22670;
        locals.var_t7__blk471_dn0 = assign15940_e22670_d_n0;
        locals.var_t7__blk471_dn2 = assign15940_e22670_d_n2;
        locals.var_t7__blk471_dn6 = assign15940_e22670_d_n6;
        locals.var_t7__blk471_dn7 = assign15940_e22670_d_n7;
        locals.var_t7__blk471_dn10 = assign15940_e22670_d_n10;
        locals.var_t7__blk471_dn11 = assign15940_e22670_d_n11;
        locals.var_t7__blk471_dn12 = assign15940_e22670_d_n12;
        locals.var_t7__blk471_dn17 = assign15940_e22670_d_n17;
        locals.var_t7__blk471_rv = 0.0;

        let (assign15950_e22679, assign15950_e22679_d_n0, assign15950_e22679_d_n2, assign15950_e22679_d_n6, assign15950_e22679_d_n7, assign15950_e22679_d_n10, assign15950_e22679_d_n11, assign15950_e22679_d_n12, assign15950_e22679_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15950_e22677: f64 = (1.0 + locals.var_t7__blk471);
        (assign15950_e22677, locals.var_t7__blk471_dn0, locals.var_t7__blk471_dn2, locals.var_t7__blk471_dn6, locals.var_t7__blk471_dn7, locals.var_t7__blk471_dn10, locals.var_t7__blk471_dn11, locals.var_t7__blk471_dn12, locals.var_t7__blk471_dn17,)
    } else {
        (locals.var_t3__blk468, locals.var_t3__blk468_dn0, locals.var_t3__blk468_dn2, locals.var_t3__blk468_dn6, locals.var_t3__blk468_dn7, locals.var_t3__blk468_dn10, locals.var_t3__blk468_dn11, locals.var_t3__blk468_dn12, locals.var_t3__blk468_dn17,)
    }
};
        locals.var_t3__blk468 = assign15950_e22679;
        locals.var_t3__blk468_dn0 = assign15950_e22679_d_n0;
        locals.var_t3__blk468_dn2 = assign15950_e22679_d_n2;
        locals.var_t3__blk468_dn6 = assign15950_e22679_d_n6;
        locals.var_t3__blk468_dn7 = assign15950_e22679_d_n7;
        locals.var_t3__blk468_dn10 = assign15950_e22679_d_n10;
        locals.var_t3__blk468_dn11 = assign15950_e22679_d_n11;
        locals.var_t3__blk468_dn12 = assign15950_e22679_d_n12;
        locals.var_t3__blk468_dn17 = assign15950_e22679_d_n17;
        locals.var_t3__blk468_rv = 0.0;

        let (assign15960_e22692, assign15960_e22692_d_n0, assign15960_e22692_d_n2, assign15960_e22692_d_n6, assign15960_e22692_d_n7, assign15960_e22692_d_n10, assign15960_e22692_d_n11, assign15960_e22692_d_n12, assign15960_e22692_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15960_e22687: f64 = (1.0 / locals.var_ddlte);
        let assign15960_e22689: f64 = (assign15960_e22687 - 1.0);
        let assign15960_e22690: f64 = (locals.var_t3__blk468).powf(assign15960_e22689);
        (assign15960_e22690, if 0.0 == 0.0 && ((assign15960_e22689) as f64).is_finite() && ((assign15960_e22689) as f64).fract() == 0.0 { if assign15960_e22689 == 0.0 { 0.0 } else { (assign15960_e22689 * ((locals.var_t3__blk468).powf(assign15960_e22689 - 1.0) * locals.var_t3__blk468_dn0)) } } else { (assign15960_e22690 * (assign15960_e22689 * (locals.var_t3__blk468_dn0 / locals.var_t3__blk468))) }, if 0.0 == 0.0 && ((assign15960_e22689) as f64).is_finite() && ((assign15960_e22689) as f64).fract() == 0.0 { if assign15960_e22689 == 0.0 { 0.0 } else { (assign15960_e22689 * ((locals.var_t3__blk468).powf(assign15960_e22689 - 1.0) * locals.var_t3__blk468_dn2)) } } else { (assign15960_e22690 * (assign15960_e22689 * (locals.var_t3__blk468_dn2 / locals.var_t3__blk468))) }, if 0.0 == 0.0 && ((assign15960_e22689) as f64).is_finite() && ((assign15960_e22689) as f64).fract() == 0.0 { if assign15960_e22689 == 0.0 { 0.0 } else { (assign15960_e22689 * ((locals.var_t3__blk468).powf(assign15960_e22689 - 1.0) * locals.var_t3__blk468_dn6)) } } else { (assign15960_e22690 * (assign15960_e22689 * (locals.var_t3__blk468_dn6 / locals.var_t3__blk468))) }, if 0.0 == 0.0 && ((assign15960_e22689) as f64).is_finite() && ((assign15960_e22689) as f64).fract() == 0.0 { if assign15960_e22689 == 0.0 { 0.0 } else { (assign15960_e22689 * ((locals.var_t3__blk468).powf(assign15960_e22689 - 1.0) * locals.var_t3__blk468_dn7)) } } else { (assign15960_e22690 * (assign15960_e22689 * (locals.var_t3__blk468_dn7 / locals.var_t3__blk468))) }, if 0.0 == 0.0 && ((assign15960_e22689) as f64).is_finite() && ((assign15960_e22689) as f64).fract() == 0.0 { if assign15960_e22689 == 0.0 { 0.0 } else { (assign15960_e22689 * ((locals.var_t3__blk468).powf(assign15960_e22689 - 1.0) * locals.var_t3__blk468_dn10)) } } else { (assign15960_e22690 * (assign15960_e22689 * (locals.var_t3__blk468_dn10 / locals.var_t3__blk468))) }, if 0.0 == 0.0 && ((assign15960_e22689) as f64).is_finite() && ((assign15960_e22689) as f64).fract() == 0.0 { if assign15960_e22689 == 0.0 { 0.0 } else { (assign15960_e22689 * ((locals.var_t3__blk468).powf(assign15960_e22689 - 1.0) * locals.var_t3__blk468_dn11)) } } else { (assign15960_e22690 * (assign15960_e22689 * (locals.var_t3__blk468_dn11 / locals.var_t3__blk468))) }, if 0.0 == 0.0 && ((assign15960_e22689) as f64).is_finite() && ((assign15960_e22689) as f64).fract() == 0.0 { if assign15960_e22689 == 0.0 { 0.0 } else { (assign15960_e22689 * ((locals.var_t3__blk468).powf(assign15960_e22689 - 1.0) * locals.var_t3__blk468_dn12)) } } else { (assign15960_e22690 * (assign15960_e22689 * (locals.var_t3__blk468_dn12 / locals.var_t3__blk468))) }, if 0.0 == 0.0 && ((assign15960_e22689) as f64).is_finite() && ((assign15960_e22689) as f64).fract() == 0.0 { if assign15960_e22689 == 0.0 { 0.0 } else { (assign15960_e22689 * ((locals.var_t3__blk468).powf(assign15960_e22689 - 1.0) * locals.var_t3__blk468_dn17)) } } else { (assign15960_e22690 * (assign15960_e22689 * (locals.var_t3__blk468_dn17 / locals.var_t3__blk468))) },)
    } else {
        (locals.var_t4__blk469, locals.var_t4__blk469_dn0, locals.var_t4__blk469_dn2, locals.var_t4__blk469_dn6, locals.var_t4__blk469_dn7, locals.var_t4__blk469_dn10, locals.var_t4__blk469_dn11, locals.var_t4__blk469_dn12, locals.var_t4__blk469_dn17,)
    }
};
        locals.var_t4__blk469 = assign15960_e22692;
        locals.var_t4__blk469_dn0 = assign15960_e22692_d_n0;
        locals.var_t4__blk469_dn2 = assign15960_e22692_d_n2;
        locals.var_t4__blk469_dn6 = assign15960_e22692_d_n6;
        locals.var_t4__blk469_dn7 = assign15960_e22692_d_n7;
        locals.var_t4__blk469_dn10 = assign15960_e22692_d_n10;
        locals.var_t4__blk469_dn11 = assign15960_e22692_d_n11;
        locals.var_t4__blk469_dn12 = assign15960_e22692_d_n12;
        locals.var_t4__blk469_dn17 = assign15960_e22692_d_n17;
        locals.var_t4__blk469_rv = 0.0;

        let (assign15970_e22701, assign15970_e22701_d_n0, assign15970_e22701_d_n2, assign15970_e22701_d_n6, assign15970_e22701_d_n7, assign15970_e22701_d_n10, assign15970_e22701_d_n11, assign15970_e22701_d_n12, assign15970_e22701_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15970_e22699: f64 = (locals.var_t4__blk469 * locals.var_t3__blk468);
        (assign15970_e22699, ((locals.var_t4__blk469_dn0 * locals.var_t3__blk468) + (locals.var_t4__blk469 * locals.var_t3__blk468_dn0)), ((locals.var_t4__blk469_dn2 * locals.var_t3__blk468) + (locals.var_t4__blk469 * locals.var_t3__blk468_dn2)), ((locals.var_t4__blk469_dn6 * locals.var_t3__blk468) + (locals.var_t4__blk469 * locals.var_t3__blk468_dn6)), ((locals.var_t4__blk469_dn7 * locals.var_t3__blk468) + (locals.var_t4__blk469 * locals.var_t3__blk468_dn7)), ((locals.var_t4__blk469_dn10 * locals.var_t3__blk468) + (locals.var_t4__blk469 * locals.var_t3__blk468_dn10)), ((locals.var_t4__blk469_dn11 * locals.var_t3__blk468) + (locals.var_t4__blk469 * locals.var_t3__blk468_dn11)), ((locals.var_t4__blk469_dn12 * locals.var_t3__blk468) + (locals.var_t4__blk469 * locals.var_t3__blk468_dn12)), ((locals.var_t4__blk469_dn17 * locals.var_t3__blk468) + (locals.var_t4__blk469 * locals.var_t3__blk468_dn17)),)
    } else {
        (locals.var_t6__blk470, locals.var_t6__blk470_dn0, locals.var_t6__blk470_dn2, locals.var_t6__blk470_dn6, locals.var_t6__blk470_dn7, locals.var_t6__blk470_dn10, locals.var_t6__blk470_dn11, locals.var_t6__blk470_dn12, locals.var_t6__blk470_dn17,)
    }
};
        locals.var_t6__blk470 = assign15970_e22701;
        locals.var_t6__blk470_dn0 = assign15970_e22701_d_n0;
        locals.var_t6__blk470_dn2 = assign15970_e22701_d_n2;
        locals.var_t6__blk470_dn6 = assign15970_e22701_d_n6;
        locals.var_t6__blk470_dn7 = assign15970_e22701_d_n7;
        locals.var_t6__blk470_dn10 = assign15970_e22701_d_n10;
        locals.var_t6__blk470_dn11 = assign15970_e22701_d_n11;
        locals.var_t6__blk470_dn12 = assign15970_e22701_d_n12;
        locals.var_t6__blk470_dn17 = assign15970_e22701_d_n17;
        locals.var_t6__blk470_rv = 0.0;

        let (assign15980_e22710, assign15980_e22710_d_n0, assign15980_e22710_d_n2, assign15980_e22710_d_n6, assign15980_e22710_d_n7, assign15980_e22710_d_n10, assign15980_e22710_d_n11, assign15980_e22710_d_n12, assign15980_e22710_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign15980_e22708: f64 = (locals.var_vds / locals.var_t6__blk470);
        (assign15980_e22708, (((locals.var_vds_dn0 * locals.var_t6__blk470) - (locals.var_vds * locals.var_t6__blk470_dn0)) / (locals.var_t6__blk470 * locals.var_t6__blk470)), (((locals.var_vds_dn2 * locals.var_t6__blk470) - (locals.var_vds * locals.var_t6__blk470_dn2)) / (locals.var_t6__blk470 * locals.var_t6__blk470)), (((locals.var_vds_dn6 * locals.var_t6__blk470) - (locals.var_vds * locals.var_t6__blk470_dn6)) / (locals.var_t6__blk470 * locals.var_t6__blk470)), (((locals.var_vds_dn7 * locals.var_t6__blk470) - (locals.var_vds * locals.var_t6__blk470_dn7)) / (locals.var_t6__blk470 * locals.var_t6__blk470)), (((locals.var_vds_dn10 * locals.var_t6__blk470) - (locals.var_vds * locals.var_t6__blk470_dn10)) / (locals.var_t6__blk470 * locals.var_t6__blk470)), (((locals.var_vds_dn11 * locals.var_t6__blk470) - (locals.var_vds * locals.var_t6__blk470_dn11)) / (locals.var_t6__blk470 * locals.var_t6__blk470)), (((locals.var_vds_dn12 * locals.var_t6__blk470) - (locals.var_vds * locals.var_t6__blk470_dn12)) / (locals.var_t6__blk470 * locals.var_t6__blk470)), (((locals.var_vds_dn17 * locals.var_t6__blk470) - (locals.var_vds * locals.var_t6__blk470_dn17)) / (locals.var_t6__blk470 * locals.var_t6__blk470)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    }
};
        locals.var_vdseff = assign15980_e22710;
        locals.var_vdseff_dn0 = assign15980_e22710_d_n0;
        locals.var_vdseff_dn2 = assign15980_e22710_d_n2;
        locals.var_vdseff_dn6 = assign15980_e22710_d_n6;
        locals.var_vdseff_dn7 = assign15980_e22710_d_n7;
        locals.var_vdseff_dn10 = assign15980_e22710_d_n10;
        locals.var_vdseff_dn11 = assign15980_e22710_d_n11;
        locals.var_vdseff_dn12 = assign15980_e22710_d_n12;
        locals.var_vdseff_dn17 = assign15980_e22710_d_n17;
        locals.var_vdseff_rv = 0.0;

        let (assign15990_e22717, assign15990_e22717_d_n0, assign15990_e22717_d_n2, assign15990_e22717_d_n6, assign15990_e22717_d_n7, assign15990_e22717_d_n10, assign15990_e22717_d_n11, assign15990_e22717_d_n12, assign15990_e22717_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign15990_e22717;
        locals.var_vds_dn0 = assign15990_e22717_d_n0;
        locals.var_vds_dn2 = assign15990_e22717_d_n2;
        locals.var_vds_dn6 = assign15990_e22717_d_n6;
        locals.var_vds_dn7 = assign15990_e22717_d_n7;
        locals.var_vds_dn10 = assign15990_e22717_d_n10;
        locals.var_vds_dn11 = assign15990_e22717_d_n11;
        locals.var_vds_dn12 = assign15990_e22717_d_n12;
        locals.var_vds_dn17 = assign15990_e22717_d_n17;
        locals.var_vds_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
        let (assign16000_e22729, assign16000_e22729_d_n0, assign16000_e22729_d_n2, assign16000_e22729_d_n6, assign16000_e22729_d_n7, assign16000_e22729_d_n10, assign16000_e22729_d_n11, assign16000_e22729_d_n12, assign16000_e22729_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16000_e22725: f64 = (locals.var_vbcs_cl - locals.var_vds);
        let assign16000_e22726: f64 = (locals.var_beta * assign16000_e22725);
        let assign16000_e22727: f64 = (assign16000_e22726).exp();
        (assign16000_e22727, (assign16000_e22727 * (locals.var_beta * (locals.var_vbcs_cl_dn0 - locals.var_vds_dn0))), (assign16000_e22727 * (locals.var_beta * (locals.var_vbcs_cl_dn2 - locals.var_vds_dn2))), (assign16000_e22727 * (locals.var_beta * (locals.var_vbcs_cl_dn6 - locals.var_vds_dn6))), (assign16000_e22727 * (locals.var_beta * (locals.var_vbcs_cl_dn7 - locals.var_vds_dn7))), (assign16000_e22727 * ((locals.var_beta_dn10 * assign16000_e22725) + (locals.var_beta * (locals.var_vbcs_cl_dn10 - locals.var_vds_dn10)))), (assign16000_e22727 * (locals.var_beta * (locals.var_vbcs_cl_dn11 - locals.var_vds_dn11))), (assign16000_e22727 * (locals.var_beta * (locals.var_vbcs_cl_dn12 - locals.var_vds_dn12))), (assign16000_e22727 * (locals.var_beta * (locals.var_vbcs_cl_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn12, locals.var_exp_bvbsvds_dn17,)
    }
};
        locals.var_exp_bvbsvds = assign16000_e22729;
        locals.var_exp_bvbsvds_dn0 = assign16000_e22729_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign16000_e22729_d_n2;
        locals.var_exp_bvbsvds_dn6 = assign16000_e22729_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign16000_e22729_d_n7;
        locals.var_exp_bvbsvds_dn10 = assign16000_e22729_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign16000_e22729_d_n11;
        locals.var_exp_bvbsvds_dn12 = assign16000_e22729_d_n12;
        locals.var_exp_bvbsvds_dn17 = assign16000_e22729_d_n17;
        locals.var_exp_bvbsvds_rv = 0.0;

        let assign16010_e22732: f64 = if locals.var_vds <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard480 = assign16010_e22732;
        locals.var_guard480_rv = 0.0;

        let (assign16020_e22741, assign16020_e22741_d_n0, assign16020_e22741_d_n2, assign16020_e22741_d_n6, assign16020_e22741_d_n7, assign16020_e22741_d_n10, assign16020_e22741_d_n11, assign16020_e22741_d_n12, assign16020_e22741_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16020_e22741;
        locals.var_pds_dn0 = assign16020_e22741_d_n0;
        locals.var_pds_dn2 = assign16020_e22741_d_n2;
        locals.var_pds_dn6 = assign16020_e22741_d_n6;
        locals.var_pds_dn7 = assign16020_e22741_d_n7;
        locals.var_pds_dn10 = assign16020_e22741_d_n10;
        locals.var_pds_dn11 = assign16020_e22741_d_n11;
        locals.var_pds_dn12 = assign16020_e22741_d_n12;
        locals.var_pds_dn17 = assign16020_e22741_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign16030_e22750, assign16030_e22750_d_n0, assign16030_e22750_d_n2, assign16030_e22750_d_n6, assign16030_e22750_d_n7, assign16030_e22750_d_n10, assign16030_e22750_d_n11, assign16030_e22750_d_n12, assign16030_e22750_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16030_e22750;
        locals.var_psl_dn0 = assign16030_e22750_d_n0;
        locals.var_psl_dn2 = assign16030_e22750_d_n2;
        locals.var_psl_dn6 = assign16030_e22750_d_n6;
        locals.var_psl_dn7 = assign16030_e22750_d_n7;
        locals.var_psl_dn10 = assign16030_e22750_d_n10;
        locals.var_psl_dn11 = assign16030_e22750_d_n11;
        locals.var_psl_dn12 = assign16030_e22750_d_n12;
        locals.var_psl_dn17 = assign16030_e22750_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16040_e22759,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign16040_e22759;
        locals.var_flg_conv_rv = 0.0;

        let assign16050_e22762: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard481 = assign16050_e22762;
        locals.var_guard481_rv = 0.0;

        let (assign16060_e22774, assign16060_e22774_d_n0, assign16060_e22774_d_n2, assign16060_e22774_d_n6, assign16060_e22774_d_n7, assign16060_e22774_d_n10, assign16060_e22774_d_n11, assign16060_e22774_d_n12, assign16060_e22774_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard481 != 0.0)) {
        (locals.var_pssl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign16060_e22774;
        locals.var_phi_sl_soi_dn0 = assign16060_e22774_d_n0;
        locals.var_phi_sl_soi_dn2 = assign16060_e22774_d_n2;
        locals.var_phi_sl_soi_dn6 = assign16060_e22774_d_n6;
        locals.var_phi_sl_soi_dn7 = assign16060_e22774_d_n7;
        locals.var_phi_sl_soi_dn10 = assign16060_e22774_d_n10;
        locals.var_phi_sl_soi_dn11 = assign16060_e22774_d_n11;
        locals.var_phi_sl_soi_dn12 = assign16060_e22774_d_n12;
        locals.var_phi_sl_soi_dn17 = assign16060_e22774_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign16070_e22788, assign16070_e22788_d_n0, assign16070_e22788_d_n2, assign16070_e22788_d_n6, assign16070_e22788_d_n7, assign16070_e22788_d_n10, assign16070_e22788_d_n11, assign16070_e22788_d_n12, assign16070_e22788_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard481 != 0.0)) {
        let assign16070_e22786: f64 = (locals.var_pssl_ini - locals.var_ps0);
        (assign16070_e22786, (-locals.var_ps0_dn0), (-locals.var_ps0_dn2), (-locals.var_ps0_dn6), (-locals.var_ps0_dn7), (-locals.var_ps0_dn10), (-locals.var_ps0_dn11), (-locals.var_ps0_dn12), (-locals.var_ps0_dn17),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16070_e22788;
        locals.var_pds_ini_dn0 = assign16070_e22788_d_n0;
        locals.var_pds_ini_dn2 = assign16070_e22788_d_n2;
        locals.var_pds_ini_dn6 = assign16070_e22788_d_n6;
        locals.var_pds_ini_dn7 = assign16070_e22788_d_n7;
        locals.var_pds_ini_dn10 = assign16070_e22788_d_n10;
        locals.var_pds_ini_dn11 = assign16070_e22788_d_n11;
        locals.var_pds_ini_dn12 = assign16070_e22788_d_n12;
        locals.var_pds_ini_dn17 = assign16070_e22788_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16080_e22791: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard482 = assign16080_e22791;
        locals.var_guard482_rv = 0.0;

        let (assign16090_e22812, assign16090_e22812_d_n0, assign16090_e22812_d_n2, assign16090_e22812_d_n6, assign16090_e22812_d_n7, assign16090_e22812_d_n10, assign16090_e22812_d_n11, assign16090_e22812_d_n12, assign16090_e22812_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16090_e22803: f64 = (locals.var_psl_lim - locals.var_ps0);
        let (assign16090_e22810, assign16090_e22810_d_n0, assign16090_e22810_d_n2, assign16090_e22810_d_n6, assign16090_e22810_d_n7, assign16090_e22810_d_n10, assign16090_e22810_d_n11, assign16090_e22810_d_n12, assign16090_e22810_d_n17,) = {
            if (assign16090_e22803 >= 0.0) {
                let assign16090_e22808: f64 = (locals.var_psl_lim - locals.var_ps0);
                (assign16090_e22808, (locals.var_psl_lim_dn0 - locals.var_ps0_dn0), (locals.var_psl_lim_dn2 - locals.var_ps0_dn2), (locals.var_psl_lim_dn6 - locals.var_ps0_dn6), (locals.var_psl_lim_dn7 - locals.var_ps0_dn7), (locals.var_psl_lim_dn10 - locals.var_ps0_dn10), (locals.var_psl_lim_dn11 - locals.var_ps0_dn11), (locals.var_psl_lim_dn12 - locals.var_ps0_dn12), (locals.var_psl_lim_dn17 - locals.var_ps0_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign16090_e22810, assign16090_e22810_d_n0, assign16090_e22810_d_n2, assign16090_e22810_d_n6, assign16090_e22810_d_n7, assign16090_e22810_d_n10, assign16090_e22810_d_n11, assign16090_e22810_d_n12, assign16090_e22810_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign16090_e22812;
        locals.var_pds_max_dn0 = assign16090_e22812_d_n0;
        locals.var_pds_max_dn2 = assign16090_e22812_d_n2;
        locals.var_pds_max_dn6 = assign16090_e22812_d_n6;
        locals.var_pds_max_dn7 = assign16090_e22812_d_n7;
        locals.var_pds_max_dn10 = assign16090_e22812_d_n10;
        locals.var_pds_max_dn11 = assign16090_e22812_d_n11;
        locals.var_pds_max_dn12 = assign16090_e22812_d_n12;
        locals.var_pds_max_dn17 = assign16090_e22812_d_n17;
        locals.var_pds_max_rv = 0.0;

        let (assign16100_e22832, assign16100_e22832_d_n0, assign16100_e22832_d_n2, assign16100_e22832_d_n6, assign16100_e22832_d_n7, assign16100_e22832_d_n10, assign16100_e22832_d_n11, assign16100_e22832_d_n12, assign16100_e22832_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16100_e22824: f64 = (1.0 + 0.3);
        let assign16100_e22826: f64 = (assign16100_e22824 * locals.var_pds_max);
        let assign16100_e22828: f64 = (assign16100_e22826 - locals.var_vds);
        let assign16100_e22830: f64 = (assign16100_e22828 - 0.03);
        (assign16100_e22830, ((assign16100_e22824 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign16100_e22824 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign16100_e22824 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign16100_e22824 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign16100_e22824 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign16100_e22824 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign16100_e22824 * locals.var_pds_max_dn12) - locals.var_vds_dn12), ((assign16100_e22824 * locals.var_pds_max_dn17) - locals.var_vds_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign16100_e22832;
        locals.var_tmf1_dn0 = assign16100_e22832_d_n0;
        locals.var_tmf1_dn2 = assign16100_e22832_d_n2;
        locals.var_tmf1_dn6 = assign16100_e22832_d_n6;
        locals.var_tmf1_dn7 = assign16100_e22832_d_n7;
        locals.var_tmf1_dn10 = assign16100_e22832_d_n10;
        locals.var_tmf1_dn11 = assign16100_e22832_d_n11;
        locals.var_tmf1_dn12 = assign16100_e22832_d_n12;
        locals.var_tmf1_dn17 = assign16100_e22832_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign16110_e22852, assign16110_e22852_d_n0, assign16110_e22852_d_n2, assign16110_e22852_d_n6, assign16110_e22852_d_n7, assign16110_e22852_d_n10, assign16110_e22852_d_n11, assign16110_e22852_d_n12, assign16110_e22852_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16110_e22845: f64 = (1.0 + 0.3);
        let assign16110_e22847: f64 = (assign16110_e22845 * locals.var_pds_max);
        let assign16110_e22848: f64 = (4.0 * assign16110_e22847);
        let assign16110_e22850: f64 = (assign16110_e22848 * 0.03);
        (assign16110_e22850, ((4.0 * (assign16110_e22845 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign16110_e22845 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign16110_e22845 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign16110_e22845 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign16110_e22845 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign16110_e22845 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign16110_e22845 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign16110_e22845 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16110_e22852;
        locals.var_tmf2_dn0 = assign16110_e22852_d_n0;
        locals.var_tmf2_dn2 = assign16110_e22852_d_n2;
        locals.var_tmf2_dn6 = assign16110_e22852_d_n6;
        locals.var_tmf2_dn7 = assign16110_e22852_d_n7;
        locals.var_tmf2_dn10 = assign16110_e22852_d_n10;
        locals.var_tmf2_dn11 = assign16110_e22852_d_n11;
        locals.var_tmf2_dn12 = assign16110_e22852_d_n12;
        locals.var_tmf2_dn17 = assign16110_e22852_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16120_e22870, assign16120_e22870_d_n0, assign16120_e22870_d_n2, assign16120_e22870_d_n6, assign16120_e22870_d_n7, assign16120_e22870_d_n10, assign16120_e22870_d_n11, assign16120_e22870_d_n12, assign16120_e22870_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard482 != 0.0)) {
        let (assign16120_e22868, assign16120_e22868_d_n0, assign16120_e22868_d_n2, assign16120_e22868_d_n6, assign16120_e22868_d_n7, assign16120_e22868_d_n10, assign16120_e22868_d_n11, assign16120_e22868_d_n12, assign16120_e22868_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign16120_e22867: f64 = (-locals.var_tmf2);
                (assign16120_e22867, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign16120_e22868, assign16120_e22868_d_n0, assign16120_e22868_d_n2, assign16120_e22868_d_n6, assign16120_e22868_d_n7, assign16120_e22868_d_n10, assign16120_e22868_d_n11, assign16120_e22868_d_n12, assign16120_e22868_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16120_e22870;
        locals.var_tmf2_dn0 = assign16120_e22870_d_n0;
        locals.var_tmf2_dn2 = assign16120_e22870_d_n2;
        locals.var_tmf2_dn6 = assign16120_e22870_d_n6;
        locals.var_tmf2_dn7 = assign16120_e22870_d_n7;
        locals.var_tmf2_dn10 = assign16120_e22870_d_n10;
        locals.var_tmf2_dn11 = assign16120_e22870_d_n11;
        locals.var_tmf2_dn12 = assign16120_e22870_d_n12;
        locals.var_tmf2_dn17 = assign16120_e22870_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16130_e22887, assign16130_e22887_d_n0, assign16130_e22887_d_n2, assign16130_e22887_d_n6, assign16130_e22887_d_n7, assign16130_e22887_d_n10, assign16130_e22887_d_n11, assign16130_e22887_d_n12, assign16130_e22887_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16130_e22882: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign16130_e22884: f64 = (assign16130_e22882 + locals.var_tmf2);
        let assign16130_e22885: f64 = (assign16130_e22884).sqrt();
        (assign16130_e22885, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign16130_e22885)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign16130_e22885)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign16130_e22885)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign16130_e22885)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign16130_e22885)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign16130_e22885)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign16130_e22885)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign16130_e22885)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign16130_e22887;
        locals.var_tmf2_dn0 = assign16130_e22887_d_n0;
        locals.var_tmf2_dn2 = assign16130_e22887_d_n2;
        locals.var_tmf2_dn6 = assign16130_e22887_d_n6;
        locals.var_tmf2_dn7 = assign16130_e22887_d_n7;
        locals.var_tmf2_dn10 = assign16130_e22887_d_n10;
        locals.var_tmf2_dn11 = assign16130_e22887_d_n11;
        locals.var_tmf2_dn12 = assign16130_e22887_d_n12;
        locals.var_tmf2_dn17 = assign16130_e22887_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign16140_e22909, assign16140_e22909_d_n0, assign16140_e22909_d_n2, assign16140_e22909_d_n6, assign16140_e22909_d_n7, assign16140_e22909_d_n10, assign16140_e22909_d_n11, assign16140_e22909_d_n12, assign16140_e22909_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard482 != 0.0)) {
        let assign16140_e22899: f64 = (1.0 + 0.3);
        let assign16140_e22901: f64 = (assign16140_e22899 * locals.var_pds_max);
        let assign16140_e22905: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign16140_e22906: f64 = (0.5 * assign16140_e22905);
        let assign16140_e22907: f64 = (assign16140_e22901 - assign16140_e22906);
        (assign16140_e22907, ((assign16140_e22899 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign16140_e22899 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign16140_e22899 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign16140_e22899 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign16140_e22899 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign16140_e22899 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign16140_e22899 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign16140_e22899 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16140_e22909;
        locals.var_pds_ini_dn0 = assign16140_e22909_d_n0;
        locals.var_pds_ini_dn2 = assign16140_e22909_d_n2;
        locals.var_pds_ini_dn6 = assign16140_e22909_d_n6;
        locals.var_pds_ini_dn7 = assign16140_e22909_d_n7;
        locals.var_pds_ini_dn10 = assign16140_e22909_d_n10;
        locals.var_pds_ini_dn11 = assign16140_e22909_d_n11;
        locals.var_pds_ini_dn12 = assign16140_e22909_d_n12;
        locals.var_pds_ini_dn17 = assign16140_e22909_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign16150_e22926, assign16150_e22926_d_n0, assign16150_e22926_d_n2, assign16150_e22926_d_n6, assign16150_e22926_d_n7, assign16150_e22926_d_n10, assign16150_e22926_d_n11, assign16150_e22926_d_n12, assign16150_e22926_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard482 != 0.0)) {
        let (assign16150_e22924, assign16150_e22924_d_n0, assign16150_e22924_d_n2, assign16150_e22924_d_n6, assign16150_e22924_d_n7, assign16150_e22924_d_n10, assign16150_e22924_d_n11, assign16150_e22924_d_n12, assign16150_e22924_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign16150_e22924, assign16150_e22924_d_n0, assign16150_e22924_d_n2, assign16150_e22924_d_n6, assign16150_e22924_d_n7, assign16150_e22924_d_n10, assign16150_e22924_d_n11, assign16150_e22924_d_n12, assign16150_e22924_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16150_e22926;
        locals.var_pds_ini_dn0 = assign16150_e22926_d_n0;
        locals.var_pds_ini_dn2 = assign16150_e22926_d_n2;
        locals.var_pds_ini_dn6 = assign16150_e22926_d_n6;
        locals.var_pds_ini_dn7 = assign16150_e22926_d_n7;
        locals.var_pds_ini_dn10 = assign16150_e22926_d_n10;
        locals.var_pds_ini_dn11 = assign16150_e22926_d_n11;
        locals.var_pds_ini_dn12 = assign16150_e22926_d_n12;
        locals.var_pds_ini_dn17 = assign16150_e22926_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16160_e22929: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard483 = assign16160_e22929;
        locals.var_guard483_rv = 0.0;

        let (assign16170_e22941, assign16170_e22941_d_n0, assign16170_e22941_d_n2, assign16170_e22941_d_n6, assign16170_e22941_d_n7, assign16170_e22941_d_n10, assign16170_e22941_d_n11, assign16170_e22941_d_n12, assign16170_e22941_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard483 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16170_e22941;
        locals.var_pds_ini_dn0 = assign16170_e22941_d_n0;
        locals.var_pds_ini_dn2 = assign16170_e22941_d_n2;
        locals.var_pds_ini_dn6 = assign16170_e22941_d_n6;
        locals.var_pds_ini_dn7 = assign16170_e22941_d_n7;
        locals.var_pds_ini_dn10 = assign16170_e22941_d_n10;
        locals.var_pds_ini_dn11 = assign16170_e22941_d_n11;
        locals.var_pds_ini_dn12 = assign16170_e22941_d_n12;
        locals.var_pds_ini_dn17 = assign16170_e22941_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let assign16180_e22944: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard484 = assign16180_e22944;
        locals.var_guard484_rv = 0.0;

        let (assign16190_e22959, assign16190_e22959_d_n0, assign16190_e22959_d_n2, assign16190_e22959_d_n6, assign16190_e22959_d_n7, assign16190_e22959_d_n10, assign16190_e22959_d_n11, assign16190_e22959_d_n12, assign16190_e22959_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) && (locals.var_guard483 == 0.0)) && (locals.var_guard484 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign16190_e22959;
        locals.var_pds_ini_dn0 = assign16190_e22959_d_n0;
        locals.var_pds_ini_dn2 = assign16190_e22959_d_n2;
        locals.var_pds_ini_dn6 = assign16190_e22959_d_n6;
        locals.var_pds_ini_dn7 = assign16190_e22959_d_n7;
        locals.var_pds_ini_dn10 = assign16190_e22959_d_n10;
        locals.var_pds_ini_dn11 = assign16190_e22959_d_n11;
        locals.var_pds_ini_dn12 = assign16190_e22959_d_n12;
        locals.var_pds_ini_dn17 = assign16190_e22959_d_n17;
        locals.var_pds_ini_rv = 0.0;

        let (assign16200_e22969, assign16200_e22969_d_n0, assign16200_e22969_d_n2, assign16200_e22969_d_n6, assign16200_e22969_d_n7, assign16200_e22969_d_n10, assign16200_e22969_d_n11, assign16200_e22969_d_n12, assign16200_e22969_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16200_e22969;
        locals.var_pds_dn0 = assign16200_e22969_d_n0;
        locals.var_pds_dn2 = assign16200_e22969_d_n2;
        locals.var_pds_dn6 = assign16200_e22969_d_n6;
        locals.var_pds_dn7 = assign16200_e22969_d_n7;
        locals.var_pds_dn10 = assign16200_e22969_d_n10;
        locals.var_pds_dn11 = assign16200_e22969_d_n11;
        locals.var_pds_dn12 = assign16200_e22969_d_n12;
        locals.var_pds_dn17 = assign16200_e22969_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign16210_e22981, assign16210_e22981_d_n0, assign16210_e22981_d_n2, assign16210_e22981_d_n6, assign16210_e22981_d_n7, assign16210_e22981_d_n10, assign16210_e22981_d_n11, assign16210_e22981_d_n12, assign16210_e22981_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) {
        let assign16210_e22979: f64 = (locals.var_ps0 + locals.var_pds);
        (assign16210_e22979, (locals.var_ps0_dn0 + locals.var_pds_dn0), (locals.var_ps0_dn2 + locals.var_pds_dn2), (locals.var_ps0_dn6 + locals.var_pds_dn6), (locals.var_ps0_dn7 + locals.var_pds_dn7), (locals.var_ps0_dn10 + locals.var_pds_dn10), (locals.var_ps0_dn11 + locals.var_pds_dn11), (locals.var_ps0_dn12 + locals.var_pds_dn12), (locals.var_ps0_dn17 + locals.var_pds_dn17),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16210_e22981;
        locals.var_psl_dn0 = assign16210_e22981_d_n0;
        locals.var_psl_dn2 = assign16210_e22981_d_n2;
        locals.var_psl_dn6 = assign16210_e22981_d_n6;
        locals.var_psl_dn7 = assign16210_e22981_d_n7;
        locals.var_psl_dn10 = assign16210_e22981_d_n10;
        locals.var_psl_dn11 = assign16210_e22981_d_n11;
        locals.var_psl_dn12 = assign16210_e22981_d_n12;
        locals.var_psl_dn17 = assign16210_e22981_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16220_e22991,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard480 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign16220_e22991;
        locals.var_flg_conv_rv = 0.0;

        let (assign16230_e22998, assign16230_e22998_d_n0, assign16230_e22998_d_n2, assign16230_e22998_d_n6, assign16230_e22998_d_n7, assign16230_e22998_d_n10, assign16230_e22998_d_n11, assign16230_e22998_d_n12, assign16230_e22998_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign16230_e22998;
        locals.var_phi_sl_soi_dn0 = assign16230_e22998_d_n0;
        locals.var_phi_sl_soi_dn2 = assign16230_e22998_d_n2;
        locals.var_phi_sl_soi_dn6 = assign16230_e22998_d_n6;
        locals.var_phi_sl_soi_dn7 = assign16230_e22998_d_n7;
        locals.var_phi_sl_soi_dn10 = assign16230_e22998_d_n10;
        locals.var_phi_sl_soi_dn11 = assign16230_e22998_d_n11;
        locals.var_phi_sl_soi_dn12 = assign16230_e22998_d_n12;
        locals.var_phi_sl_soi_dn17 = assign16230_e22998_d_n17;
        locals.var_phi_sl_soi_rv = 0.0;

        let (assign16240_e23005,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign16240_e23005;
        locals.var_lp_sl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        locals: &mut StampLocals,
    ) {
        let mut assign16250_loop_guard: usize = 0;
        while {
            let assign16250_cond_e23013: f64 = (locals.var_lp_sl_max + 1.0);
            let assign16250_cond_e23015: f64 = if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_lp_sl <= assign16250_cond_e23013)) { 1.0 } else { 0.0 };
            assign16250_cond_e23015 != 0.0
        } {
            assign16250_loop_guard += 1;
            assert!(assign16250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign16250_body0_e23024, assign16250_body0_e23024_d_n0, assign16250_body0_e23024_d_n2, assign16250_body0_e23024_d_n6, assign16250_body0_e23024_d_n7, assign16250_body0_e23024_d_n10, assign16250_body0_e23024_d_n11, assign16250_body0_e23024_d_n12, assign16250_body0_e23024_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16250_body0_e23022: f64 = (locals.var_phi_sl_soi - locals.var_vbcs_cl);
        (assign16250_body0_e23022, (locals.var_phi_sl_soi_dn0 - locals.var_vbcs_cl_dn0), (locals.var_phi_sl_soi_dn2 - locals.var_vbcs_cl_dn2), (locals.var_phi_sl_soi_dn6 - locals.var_vbcs_cl_dn6), (locals.var_phi_sl_soi_dn7 - locals.var_vbcs_cl_dn7), (locals.var_phi_sl_soi_dn10 - locals.var_vbcs_cl_dn10), (locals.var_phi_sl_soi_dn11 - locals.var_vbcs_cl_dn11), (locals.var_phi_sl_soi_dn12 - locals.var_vbcs_cl_dn12), (locals.var_phi_sl_soi_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_phi_soil, locals.var_phi_soil_dn0, locals.var_phi_soil_dn2, locals.var_phi_soil_dn6, locals.var_phi_soil_dn7, locals.var_phi_soil_dn10, locals.var_phi_soil_dn11, locals.var_phi_soil_dn12, locals.var_phi_soil_dn17,)
    }
};
            locals.var_phi_soil = assign16250_body0_e23024;
            locals.var_phi_soil_dn0 = assign16250_body0_e23024_d_n0;
            locals.var_phi_soil_dn2 = assign16250_body0_e23024_d_n2;
            locals.var_phi_soil_dn6 = assign16250_body0_e23024_d_n6;
            locals.var_phi_soil_dn7 = assign16250_body0_e23024_d_n7;
            locals.var_phi_soil_dn10 = assign16250_body0_e23024_d_n10;
            locals.var_phi_soil_dn11 = assign16250_body0_e23024_d_n11;
            locals.var_phi_soil_dn12 = assign16250_body0_e23024_d_n12;
            locals.var_phi_soil_dn17 = assign16250_body0_e23024_d_n17;
            locals.var_phi_soil_rv = 0.0;
            let (assign16250_body1_e23033, assign16250_body1_e23033_d_n0, assign16250_body1_e23033_d_n2, assign16250_body1_e23033_d_n6, assign16250_body1_e23033_d_n7, assign16250_body1_e23033_d_n10, assign16250_body1_e23033_d_n11, assign16250_body1_e23033_d_n12, assign16250_body1_e23033_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16250_body1_e23031: f64 = (locals.var_beta * locals.var_phi_soil);
        (assign16250_body1_e23031, (locals.var_beta * locals.var_phi_soil_dn0), (locals.var_beta * locals.var_phi_soil_dn2), (locals.var_beta * locals.var_phi_soil_dn6), (locals.var_beta * locals.var_phi_soil_dn7), ((locals.var_beta_dn10 * locals.var_phi_soil) + (locals.var_beta * locals.var_phi_soil_dn10)), (locals.var_beta * locals.var_phi_soil_dn11), (locals.var_beta * locals.var_phi_soil_dn12), (locals.var_beta * locals.var_phi_soil_dn17),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
            locals.var_chi = assign16250_body1_e23033;
            locals.var_chi_dn0 = assign16250_body1_e23033_d_n0;
            locals.var_chi_dn2 = assign16250_body1_e23033_d_n2;
            locals.var_chi_dn6 = assign16250_body1_e23033_d_n6;
            locals.var_chi_dn7 = assign16250_body1_e23033_d_n7;
            locals.var_chi_dn10 = assign16250_body1_e23033_d_n10;
            locals.var_chi_dn11 = assign16250_body1_e23033_d_n11;
            locals.var_chi_dn12 = assign16250_body1_e23033_d_n12;
            locals.var_chi_dn17 = assign16250_body1_e23033_d_n17;
            locals.var_chi_rv = 0.0;
            let (assign16250_body2_e23044, assign16250_body2_e23044_d_n0, assign16250_body2_e23044_d_n2, assign16250_body2_e23044_d_n6, assign16250_body2_e23044_d_n7, assign16250_body2_e23044_d_n10, assign16250_body2_e23044_d_n11, assign16250_body2_e23044_d_n12, assign16250_body2_e23044_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16250_body2_e23041: f64 = (locals.var_phi_soil - locals.var_dphi_sb);
        let assign16250_body2_e23042: f64 = (locals.var_c_sb * assign16250_body2_e23041);
        (assign16250_body2_e23042, ((locals.var_c_sb_dn0 * assign16250_body2_e23041) + (locals.var_c_sb * (locals.var_phi_soil_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign16250_body2_e23041) + (locals.var_c_sb * (locals.var_phi_soil_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn6 * assign16250_body2_e23041) + (locals.var_c_sb * (locals.var_phi_soil_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign16250_body2_e23041) + (locals.var_c_sb * (locals.var_phi_soil_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn10 * assign16250_body2_e23041) + (locals.var_c_sb * (locals.var_phi_soil_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign16250_body2_e23041) + (locals.var_c_sb * (locals.var_phi_soil_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn12 * assign16250_body2_e23041) + (locals.var_c_sb * (locals.var_phi_soil_dn12 - locals.var_dphi_sb_dn12))), ((locals.var_c_sb_dn17 * assign16250_body2_e23041) + (locals.var_c_sb * (locals.var_phi_soil_dn17 - locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
            locals.var_ty = assign16250_body2_e23044;
            locals.var_ty_dn0 = assign16250_body2_e23044_d_n0;
            locals.var_ty_dn2 = assign16250_body2_e23044_d_n2;
            locals.var_ty_dn6 = assign16250_body2_e23044_d_n6;
            locals.var_ty_dn7 = assign16250_body2_e23044_d_n7;
            locals.var_ty_dn10 = assign16250_body2_e23044_d_n10;
            locals.var_ty_dn11 = assign16250_body2_e23044_d_n11;
            locals.var_ty_dn12 = assign16250_body2_e23044_d_n12;
            locals.var_ty_dn17 = assign16250_body2_e23044_d_n17;
            locals.var_ty_rv = 0.0;
            let assign16250_body3_e23047: f64 = if locals.var_ty < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard485 = assign16250_body3_e23047;
            locals.var_guard485_rv = 0.0;
            let (assign16250_body4_e23057, assign16250_body4_e23057_d_n0, assign16250_body4_e23057_d_n2, assign16250_body4_e23057_d_n6, assign16250_body4_e23057_d_n7, assign16250_body4_e23057_d_n10, assign16250_body4_e23057_d_n11, assign16250_body4_e23057_d_n12, assign16250_body4_e23057_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign16250_body4_e23055: f64 = (locals.var_ty).exp();
        (assign16250_body4_e23055, (assign16250_body4_e23055 * locals.var_ty_dn0), (assign16250_body4_e23055 * locals.var_ty_dn2), (assign16250_body4_e23055 * locals.var_ty_dn6), (assign16250_body4_e23055 * locals.var_ty_dn7), (assign16250_body4_e23055 * locals.var_ty_dn10), (assign16250_body4_e23055 * locals.var_ty_dn11), (assign16250_body4_e23055 * locals.var_ty_dn12), (assign16250_body4_e23055 * locals.var_ty_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16250_body4_e23057;
            locals.var_t1_dn0 = assign16250_body4_e23057_d_n0;
            locals.var_t1_dn2 = assign16250_body4_e23057_d_n2;
            locals.var_t1_dn6 = assign16250_body4_e23057_d_n6;
            locals.var_t1_dn7 = assign16250_body4_e23057_d_n7;
            locals.var_t1_dn10 = assign16250_body4_e23057_d_n10;
            locals.var_t1_dn11 = assign16250_body4_e23057_d_n11;
            locals.var_t1_dn12 = assign16250_body4_e23057_d_n12;
            locals.var_t1_dn17 = assign16250_body4_e23057_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16250_body5_e23070, assign16250_body5_e23070_d_n0, assign16250_body5_e23070_d_n2, assign16250_body5_e23070_d_n6, assign16250_body5_e23070_d_n7, assign16250_body5_e23070_d_n10, assign16250_body5_e23070_d_n11, assign16250_body5_e23070_d_n12, assign16250_body5_e23070_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign16250_body5_e23065: f64 = (-locals.var_c_sb);
        let assign16250_body5_e23067: f64 = (assign16250_body5_e23065 * locals.var_dphi_sb);
        let assign16250_body5_e23068: f64 = (assign16250_body5_e23067).exp();
        (assign16250_body5_e23068, (assign16250_body5_e23068 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign16250_body5_e23065 * locals.var_dphi_sb_dn0))), (assign16250_body5_e23068 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign16250_body5_e23065 * locals.var_dphi_sb_dn2))), (assign16250_body5_e23068 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign16250_body5_e23065 * locals.var_dphi_sb_dn6))), (assign16250_body5_e23068 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign16250_body5_e23065 * locals.var_dphi_sb_dn7))), (assign16250_body5_e23068 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign16250_body5_e23065 * locals.var_dphi_sb_dn10))), (assign16250_body5_e23068 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign16250_body5_e23065 * locals.var_dphi_sb_dn11))), (assign16250_body5_e23068 * (((-locals.var_c_sb_dn12) * locals.var_dphi_sb) + (assign16250_body5_e23065 * locals.var_dphi_sb_dn12))), (assign16250_body5_e23068 * (((-locals.var_c_sb_dn17) * locals.var_dphi_sb) + (assign16250_body5_e23065 * locals.var_dphi_sb_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16250_body5_e23070;
            locals.var_t0_dn0 = assign16250_body5_e23070_d_n0;
            locals.var_t0_dn2 = assign16250_body5_e23070_d_n2;
            locals.var_t0_dn6 = assign16250_body5_e23070_d_n6;
            locals.var_t0_dn7 = assign16250_body5_e23070_d_n7;
            locals.var_t0_dn10 = assign16250_body5_e23070_d_n10;
            locals.var_t0_dn11 = assign16250_body5_e23070_d_n11;
            locals.var_t0_dn12 = assign16250_body5_e23070_d_n12;
            locals.var_t0_dn17 = assign16250_body5_e23070_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16250_body6_e23081, assign16250_body6_e23081_d_n0, assign16250_body6_e23081_d_n2, assign16250_body6_e23081_d_n6, assign16250_body6_e23081_d_n7, assign16250_body6_e23081_d_n10, assign16250_body6_e23081_d_n11, assign16250_body6_e23081_d_n12, assign16250_body6_e23081_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign16250_body6_e23079: f64 = (locals.var_t1 - locals.var_t0);
        (assign16250_body6_e23079, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn12 - locals.var_t0_dn12), (locals.var_t1_dn17 - locals.var_t0_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign16250_body6_e23081;
            locals.var_t2_dn0 = assign16250_body6_e23081_d_n0;
            locals.var_t2_dn2 = assign16250_body6_e23081_d_n2;
            locals.var_t2_dn6 = assign16250_body6_e23081_d_n6;
            locals.var_t2_dn7 = assign16250_body6_e23081_d_n7;
            locals.var_t2_dn10 = assign16250_body6_e23081_d_n10;
            locals.var_t2_dn11 = assign16250_body6_e23081_d_n11;
            locals.var_t2_dn12 = assign16250_body6_e23081_d_n12;
            locals.var_t2_dn17 = assign16250_body6_e23081_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign16250_body7_e23095, assign16250_body7_e23095_d_n0, assign16250_body7_e23095_d_n2, assign16250_body7_e23095_d_n6, assign16250_body7_e23095_d_n7, assign16250_body7_e23095_d_n10, assign16250_body7_e23095_d_n11, assign16250_body7_e23095_d_n12, assign16250_body7_e23095_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign16250_body7_e23090: f64 = (1.0 + locals.var_t2);
        let assign16250_body7_e23091: f64 = (assign16250_body7_e23090).ln();
        let assign16250_body7_e23093: f64 = (assign16250_body7_e23091 / locals.var_c_sb);
        (assign16250_body7_e23093, ((((locals.var_t2_dn0 / assign16250_body7_e23090) * locals.var_c_sb) - (assign16250_body7_e23091 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign16250_body7_e23090) * locals.var_c_sb) - (assign16250_body7_e23091 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign16250_body7_e23090) * locals.var_c_sb) - (assign16250_body7_e23091 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign16250_body7_e23090) * locals.var_c_sb) - (assign16250_body7_e23091 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign16250_body7_e23090) * locals.var_c_sb) - (assign16250_body7_e23091 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign16250_body7_e23090) * locals.var_c_sb) - (assign16250_body7_e23091 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn12 / assign16250_body7_e23090) * locals.var_c_sb) - (assign16250_body7_e23091 * locals.var_c_sb_dn12)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn17 / assign16250_body7_e23090) * locals.var_c_sb) - (assign16250_body7_e23091 * locals.var_c_sb_dn17)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign16250_body7_e23095;
            locals.var_phi_soib_dn0 = assign16250_body7_e23095_d_n0;
            locals.var_phi_soib_dn2 = assign16250_body7_e23095_d_n2;
            locals.var_phi_soib_dn6 = assign16250_body7_e23095_d_n6;
            locals.var_phi_soib_dn7 = assign16250_body7_e23095_d_n7;
            locals.var_phi_soib_dn10 = assign16250_body7_e23095_d_n10;
            locals.var_phi_soib_dn11 = assign16250_body7_e23095_d_n11;
            locals.var_phi_soib_dn12 = assign16250_body7_e23095_d_n12;
            locals.var_phi_soib_dn17 = assign16250_body7_e23095_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign16250_body8_e23108, assign16250_body8_e23108_d_n0, assign16250_body8_e23108_d_n2, assign16250_body8_e23108_d_n6, assign16250_body8_e23108_d_n7, assign16250_body8_e23108_d_n10, assign16250_body8_e23108_d_n11, assign16250_body8_e23108_d_n12, assign16250_body8_e23108_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard485 != 0.0)) {
        let assign16250_body8_e23105: f64 = (1.0 + locals.var_t2);
        let assign16250_body8_e23106: f64 = (locals.var_t1 / assign16250_body8_e23105);
        (assign16250_body8_e23106, (((locals.var_t1_dn0 * assign16250_body8_e23105) - (locals.var_t1 * locals.var_t2_dn0)) / (assign16250_body8_e23105 * assign16250_body8_e23105)), (((locals.var_t1_dn2 * assign16250_body8_e23105) - (locals.var_t1 * locals.var_t2_dn2)) / (assign16250_body8_e23105 * assign16250_body8_e23105)), (((locals.var_t1_dn6 * assign16250_body8_e23105) - (locals.var_t1 * locals.var_t2_dn6)) / (assign16250_body8_e23105 * assign16250_body8_e23105)), (((locals.var_t1_dn7 * assign16250_body8_e23105) - (locals.var_t1 * locals.var_t2_dn7)) / (assign16250_body8_e23105 * assign16250_body8_e23105)), (((locals.var_t1_dn10 * assign16250_body8_e23105) - (locals.var_t1 * locals.var_t2_dn10)) / (assign16250_body8_e23105 * assign16250_body8_e23105)), (((locals.var_t1_dn11 * assign16250_body8_e23105) - (locals.var_t1 * locals.var_t2_dn11)) / (assign16250_body8_e23105 * assign16250_body8_e23105)), (((locals.var_t1_dn12 * assign16250_body8_e23105) - (locals.var_t1 * locals.var_t2_dn12)) / (assign16250_body8_e23105 * assign16250_body8_e23105)), (((locals.var_t1_dn17 * assign16250_body8_e23105) - (locals.var_t1 * locals.var_t2_dn17)) / (assign16250_body8_e23105 * assign16250_body8_e23105)),)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign16250_body8_e23108;
            locals.var_phi_soib_dpss_dn0 = assign16250_body8_e23108_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign16250_body8_e23108_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign16250_body8_e23108_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign16250_body8_e23108_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign16250_body8_e23108_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign16250_body8_e23108_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign16250_body8_e23108_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign16250_body8_e23108_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign16250_body9_e23120, assign16250_body9_e23120_d_n0, assign16250_body9_e23120_d_n2, assign16250_body9_e23120_d_n6, assign16250_body9_e23120_d_n7, assign16250_body9_e23120_d_n10, assign16250_body9_e23120_d_n11, assign16250_body9_e23120_d_n12, assign16250_body9_e23120_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard485 == 0.0)) {
        let assign16250_body9_e23118: f64 = (locals.var_phi_soil - locals.var_dphi_sb);
        (assign16250_body9_e23118, (locals.var_phi_soil_dn0 - locals.var_dphi_sb_dn0), (locals.var_phi_soil_dn2 - locals.var_dphi_sb_dn2), (locals.var_phi_soil_dn6 - locals.var_dphi_sb_dn6), (locals.var_phi_soil_dn7 - locals.var_dphi_sb_dn7), (locals.var_phi_soil_dn10 - locals.var_dphi_sb_dn10), (locals.var_phi_soil_dn11 - locals.var_dphi_sb_dn11), (locals.var_phi_soil_dn12 - locals.var_dphi_sb_dn12), (locals.var_phi_soil_dn17 - locals.var_dphi_sb_dn17),)
    } else {
        (locals.var_phi_soib, locals.var_phi_soib_dn0, locals.var_phi_soib_dn2, locals.var_phi_soib_dn6, locals.var_phi_soib_dn7, locals.var_phi_soib_dn10, locals.var_phi_soib_dn11, locals.var_phi_soib_dn12, locals.var_phi_soib_dn17,)
    }
};
            locals.var_phi_soib = assign16250_body9_e23120;
            locals.var_phi_soib_dn0 = assign16250_body9_e23120_d_n0;
            locals.var_phi_soib_dn2 = assign16250_body9_e23120_d_n2;
            locals.var_phi_soib_dn6 = assign16250_body9_e23120_d_n6;
            locals.var_phi_soib_dn7 = assign16250_body9_e23120_d_n7;
            locals.var_phi_soib_dn10 = assign16250_body9_e23120_d_n10;
            locals.var_phi_soib_dn11 = assign16250_body9_e23120_d_n11;
            locals.var_phi_soib_dn12 = assign16250_body9_e23120_d_n12;
            locals.var_phi_soib_dn17 = assign16250_body9_e23120_d_n17;
            locals.var_phi_soib_rv = 0.0;
            let (assign16250_body10_e23130, assign16250_body10_e23130_d_n0, assign16250_body10_e23130_d_n2, assign16250_body10_e23130_d_n6, assign16250_body10_e23130_d_n7, assign16250_body10_e23130_d_n10, assign16250_body10_e23130_d_n11, assign16250_body10_e23130_d_n12, assign16250_body10_e23130_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard485 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_soib_dpss, locals.var_phi_soib_dpss_dn0, locals.var_phi_soib_dpss_dn2, locals.var_phi_soib_dpss_dn6, locals.var_phi_soib_dpss_dn7, locals.var_phi_soib_dpss_dn10, locals.var_phi_soib_dpss_dn11, locals.var_phi_soib_dpss_dn12, locals.var_phi_soib_dpss_dn17,)
    }
};
            locals.var_phi_soib_dpss = assign16250_body10_e23130;
            locals.var_phi_soib_dpss_dn0 = assign16250_body10_e23130_d_n0;
            locals.var_phi_soib_dpss_dn2 = assign16250_body10_e23130_d_n2;
            locals.var_phi_soib_dpss_dn6 = assign16250_body10_e23130_d_n6;
            locals.var_phi_soib_dpss_dn7 = assign16250_body10_e23130_d_n7;
            locals.var_phi_soib_dpss_dn10 = assign16250_body10_e23130_d_n10;
            locals.var_phi_soib_dpss_dn11 = assign16250_body10_e23130_d_n11;
            locals.var_phi_soib_dpss_dn12 = assign16250_body10_e23130_d_n12;
            locals.var_phi_soib_dpss_dn17 = assign16250_body10_e23130_d_n17;
            locals.var_phi_soib_dpss_rv = 0.0;
            let (assign16250_body11_e23139, assign16250_body11_e23139_d_n0, assign16250_body11_e23139_d_n2, assign16250_body11_e23139_d_n6, assign16250_body11_e23139_d_n7, assign16250_body11_e23139_d_n10, assign16250_body11_e23139_d_n11, assign16250_body11_e23139_d_n12, assign16250_body11_e23139_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16250_body11_e23137: f64 = (locals.var_beta * locals.var_phi_soib);
        (assign16250_body11_e23137, (locals.var_beta * locals.var_phi_soib_dn0), (locals.var_beta * locals.var_phi_soib_dn2), (locals.var_beta * locals.var_phi_soib_dn6), (locals.var_beta * locals.var_phi_soib_dn7), ((locals.var_beta_dn10 * locals.var_phi_soib) + (locals.var_beta * locals.var_phi_soib_dn10)), (locals.var_beta * locals.var_phi_soib_dn11), (locals.var_beta * locals.var_phi_soib_dn12), (locals.var_beta * locals.var_phi_soib_dn17),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn12, locals.var_chib_dn17,)
    }
};
            locals.var_chib = assign16250_body11_e23139;
            locals.var_chib_dn0 = assign16250_body11_e23139_d_n0;
            locals.var_chib_dn2 = assign16250_body11_e23139_d_n2;
            locals.var_chib_dn6 = assign16250_body11_e23139_d_n6;
            locals.var_chib_dn7 = assign16250_body11_e23139_d_n7;
            locals.var_chib_dn10 = assign16250_body11_e23139_d_n10;
            locals.var_chib_dn11 = assign16250_body11_e23139_d_n11;
            locals.var_chib_dn12 = assign16250_body11_e23139_d_n12;
            locals.var_chib_dn17 = assign16250_body11_e23139_d_n17;
            locals.var_chib_rv = 0.0;
            let assign16250_body12_e23141: f64 = (locals.var_chi).abs();
            let assign16250_body12_e23143: f64 = if assign16250_body12_e23141 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard486 = assign16250_body12_e23143;
            locals.var_guard486_rv = 0.0;
            let (assign16250_body13_e23159, assign16250_body13_e23159_d_n0, assign16250_body13_e23159_d_n2, assign16250_body13_e23159_d_n6, assign16250_body13_e23159_d_n7, assign16250_body13_e23159_d_n10, assign16250_body13_e23159_d_n11, assign16250_body13_e23159_d_n12, assign16250_body13_e23159_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16250_body13_e23153: f64 = (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss);
        let assign16250_body13_e23154: f64 = (1.0 - assign16250_body13_e23153);
        let assign16250_body13_e23156: f64 = (assign16250_body13_e23154 / 2.0);
        let assign16250_body13_e23157: f64 = (assign16250_body13_e23156).sqrt();
        (assign16250_body13_e23157, (((-((locals.var_phi_soib_dpss_dn0 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn0))) / 2.0) / (2.0 * assign16250_body13_e23157)), (((-((locals.var_phi_soib_dpss_dn2 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn2))) / 2.0) / (2.0 * assign16250_body13_e23157)), (((-((locals.var_phi_soib_dpss_dn6 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn6))) / 2.0) / (2.0 * assign16250_body13_e23157)), (((-((locals.var_phi_soib_dpss_dn7 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn7))) / 2.0) / (2.0 * assign16250_body13_e23157)), (((-((locals.var_phi_soib_dpss_dn10 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn10))) / 2.0) / (2.0 * assign16250_body13_e23157)), (((-((locals.var_phi_soib_dpss_dn11 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn11))) / 2.0) / (2.0 * assign16250_body13_e23157)), (((-((locals.var_phi_soib_dpss_dn12 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn12))) / 2.0) / (2.0 * assign16250_body13_e23157)), (((-((locals.var_phi_soib_dpss_dn17 * locals.var_phi_soib_dpss) + (locals.var_phi_soib_dpss * locals.var_phi_soib_dpss_dn17))) / 2.0) / (2.0 * assign16250_body13_e23157)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16250_body13_e23159;
            locals.var_t0_dn0 = assign16250_body13_e23159_d_n0;
            locals.var_t0_dn2 = assign16250_body13_e23159_d_n2;
            locals.var_t0_dn6 = assign16250_body13_e23159_d_n6;
            locals.var_t0_dn7 = assign16250_body13_e23159_d_n7;
            locals.var_t0_dn10 = assign16250_body13_e23159_d_n10;
            locals.var_t0_dn11 = assign16250_body13_e23159_d_n11;
            locals.var_t0_dn12 = assign16250_body13_e23159_d_n12;
            locals.var_t0_dn17 = assign16250_body13_e23159_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16250_body14_e23170, assign16250_body14_e23170_d_n0, assign16250_body14_e23170_d_n2, assign16250_body14_e23170_d_n6, assign16250_body14_e23170_d_n7, assign16250_body14_e23170_d_n10, assign16250_body14_e23170_d_n11, assign16250_body14_e23170_d_n12, assign16250_body14_e23170_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16250_body14_e23168: f64 = (locals.var_chi * locals.var_t0);
        (assign16250_body14_e23168, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn12 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn12)), ((locals.var_chi_dn17 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn17)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16250_body14_e23170;
            locals.var_fb_dn0 = assign16250_body14_e23170_d_n0;
            locals.var_fb_dn2 = assign16250_body14_e23170_d_n2;
            locals.var_fb_dn6 = assign16250_body14_e23170_d_n6;
            locals.var_fb_dn7 = assign16250_body14_e23170_d_n7;
            locals.var_fb_dn10 = assign16250_body14_e23170_d_n10;
            locals.var_fb_dn11 = assign16250_body14_e23170_d_n11;
            locals.var_fb_dn12 = assign16250_body14_e23170_d_n12;
            locals.var_fb_dn17 = assign16250_body14_e23170_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16250_body15_e23181, assign16250_body15_e23181_d_n0, assign16250_body15_e23181_d_n2, assign16250_body15_e23181_d_n6, assign16250_body15_e23181_d_n7, assign16250_body15_e23181_d_n10, assign16250_body15_e23181_d_n11, assign16250_body15_e23181_d_n12, assign16250_body15_e23181_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 != 0.0)) {
        let assign16250_body15_e23179: f64 = (locals.var_beta * locals.var_t0);
        (assign16250_body15_e23179, (locals.var_beta * locals.var_t0_dn0), (locals.var_beta * locals.var_t0_dn2), (locals.var_beta * locals.var_t0_dn6), (locals.var_beta * locals.var_t0_dn7), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), (locals.var_beta * locals.var_t0_dn11), (locals.var_beta * locals.var_t0_dn12), (locals.var_beta * locals.var_t0_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16250_body15_e23181;
            locals.var_fb_dpss_dn0 = assign16250_body15_e23181_d_n0;
            locals.var_fb_dpss_dn2 = assign16250_body15_e23181_d_n2;
            locals.var_fb_dpss_dn6 = assign16250_body15_e23181_d_n6;
            locals.var_fb_dpss_dn7 = assign16250_body15_e23181_d_n7;
            locals.var_fb_dpss_dn10 = assign16250_body15_e23181_d_n10;
            locals.var_fb_dpss_dn11 = assign16250_body15_e23181_d_n11;
            locals.var_fb_dpss_dn12 = assign16250_body15_e23181_d_n12;
            locals.var_fb_dpss_dn17 = assign16250_body15_e23181_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16250_body16_e23184: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard487 = assign16250_body16_e23184;
            locals.var_guard487_rv = 0.0;
            let (assign16250_body17_e23196, assign16250_body17_e23196_d_n0, assign16250_body17_e23196_d_n2, assign16250_body17_e23196_d_n6, assign16250_body17_e23196_d_n7, assign16250_body17_e23196_d_n10, assign16250_body17_e23196_d_n11, assign16250_body17_e23196_d_n12, assign16250_body17_e23196_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 != 0.0)) {
        let assign16250_body17_e23194: f64 = (-locals.var_fb);
        (assign16250_body17_e23194, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16250_body17_e23196;
            locals.var_fb_dn0 = assign16250_body17_e23196_d_n0;
            locals.var_fb_dn2 = assign16250_body17_e23196_d_n2;
            locals.var_fb_dn6 = assign16250_body17_e23196_d_n6;
            locals.var_fb_dn7 = assign16250_body17_e23196_d_n7;
            locals.var_fb_dn10 = assign16250_body17_e23196_d_n10;
            locals.var_fb_dn11 = assign16250_body17_e23196_d_n11;
            locals.var_fb_dn12 = assign16250_body17_e23196_d_n12;
            locals.var_fb_dn17 = assign16250_body17_e23196_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16250_body18_e23208, assign16250_body18_e23208_d_n0, assign16250_body18_e23208_d_n2, assign16250_body18_e23208_d_n6, assign16250_body18_e23208_d_n7, assign16250_body18_e23208_d_n10, assign16250_body18_e23208_d_n11, assign16250_body18_e23208_d_n12, assign16250_body18_e23208_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 != 0.0)) && (locals.var_guard487 != 0.0)) {
        let assign16250_body18_e23206: f64 = (-locals.var_fb_dpss);
        (assign16250_body18_e23206, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16250_body18_e23208;
            locals.var_fb_dpss_dn0 = assign16250_body18_e23208_d_n0;
            locals.var_fb_dpss_dn2 = assign16250_body18_e23208_d_n2;
            locals.var_fb_dpss_dn6 = assign16250_body18_e23208_d_n6;
            locals.var_fb_dpss_dn7 = assign16250_body18_e23208_d_n7;
            locals.var_fb_dpss_dn10 = assign16250_body18_e23208_d_n10;
            locals.var_fb_dpss_dn11 = assign16250_body18_e23208_d_n11;
            locals.var_fb_dpss_dn12 = assign16250_body18_e23208_d_n12;
            locals.var_fb_dpss_dn17 = assign16250_body18_e23208_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16250_body19_e23210: f64 = (locals.var_chi).abs();
            let assign16250_body19_e23212: f64 = if assign16250_body19_e23210 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard488 = assign16250_body19_e23212;
            locals.var_guard488_rv = 0.0;
            let (assign16250_body20_e23246, assign16250_body20_e23246_d_n0, assign16250_body20_e23246_d_n2, assign16250_body20_e23246_d_n6, assign16250_body20_e23246_d_n7, assign16250_body20_e23246_d_n10, assign16250_body20_e23246_d_n11, assign16250_body20_e23246_d_n12, assign16250_body20_e23246_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16250_body20_e23224: f64 = (locals.var_chi * locals.var_chi);
        let assign16250_body20_e23226: f64 = (assign16250_body20_e23224 / 2.0);
        let assign16250_body20_e23230: f64 = (locals.var_chi / 3.0);
        let assign16250_body20_e23234: f64 = (locals.var_chi / 4.0);
        let assign16250_body20_e23238: f64 = (locals.var_chi / 5.0);
        let assign16250_body20_e23239: f64 = (1.0 - assign16250_body20_e23238);
        let assign16250_body20_e23240: f64 = (assign16250_body20_e23234 * assign16250_body20_e23239);
        let assign16250_body20_e23241: f64 = (1.0 - assign16250_body20_e23240);
        let assign16250_body20_e23242: f64 = (assign16250_body20_e23230 * assign16250_body20_e23241);
        let assign16250_body20_e23243: f64 = (1.0 - assign16250_body20_e23242);
        let assign16250_body20_e23244: f64 = (assign16250_body20_e23226 * assign16250_body20_e23243);
        (assign16250_body20_e23244, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign16250_body20_e23243) + (assign16250_body20_e23226 * (-(((locals.var_chi_dn0 / 3.0) * assign16250_body20_e23241) + (assign16250_body20_e23230 * (-(((locals.var_chi_dn0 / 4.0) * assign16250_body20_e23239) + (assign16250_body20_e23234 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign16250_body20_e23243) + (assign16250_body20_e23226 * (-(((locals.var_chi_dn2 / 3.0) * assign16250_body20_e23241) + (assign16250_body20_e23230 * (-(((locals.var_chi_dn2 / 4.0) * assign16250_body20_e23239) + (assign16250_body20_e23234 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign16250_body20_e23243) + (assign16250_body20_e23226 * (-(((locals.var_chi_dn6 / 3.0) * assign16250_body20_e23241) + (assign16250_body20_e23230 * (-(((locals.var_chi_dn6 / 4.0) * assign16250_body20_e23239) + (assign16250_body20_e23234 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign16250_body20_e23243) + (assign16250_body20_e23226 * (-(((locals.var_chi_dn7 / 3.0) * assign16250_body20_e23241) + (assign16250_body20_e23230 * (-(((locals.var_chi_dn7 / 4.0) * assign16250_body20_e23239) + (assign16250_body20_e23234 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign16250_body20_e23243) + (assign16250_body20_e23226 * (-(((locals.var_chi_dn10 / 3.0) * assign16250_body20_e23241) + (assign16250_body20_e23230 * (-(((locals.var_chi_dn10 / 4.0) * assign16250_body20_e23239) + (assign16250_body20_e23234 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign16250_body20_e23243) + (assign16250_body20_e23226 * (-(((locals.var_chi_dn11 / 3.0) * assign16250_body20_e23241) + (assign16250_body20_e23230 * (-(((locals.var_chi_dn11 / 4.0) * assign16250_body20_e23239) + (assign16250_body20_e23234 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) / 2.0) * assign16250_body20_e23243) + (assign16250_body20_e23226 * (-(((locals.var_chi_dn12 / 3.0) * assign16250_body20_e23241) + (assign16250_body20_e23230 * (-(((locals.var_chi_dn12 / 4.0) * assign16250_body20_e23239) + (assign16250_body20_e23234 * (-(locals.var_chi_dn12 / 5.0)))))))))), (((((locals.var_chi_dn17 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn17)) / 2.0) * assign16250_body20_e23243) + (assign16250_body20_e23226 * (-(((locals.var_chi_dn17 / 3.0) * assign16250_body20_e23241) + (assign16250_body20_e23230 * (-(((locals.var_chi_dn17 / 4.0) * assign16250_body20_e23239) + (assign16250_body20_e23234 * (-(locals.var_chi_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16250_body20_e23246;
            locals.var_t0_dn0 = assign16250_body20_e23246_d_n0;
            locals.var_t0_dn2 = assign16250_body20_e23246_d_n2;
            locals.var_t0_dn6 = assign16250_body20_e23246_d_n6;
            locals.var_t0_dn7 = assign16250_body20_e23246_d_n7;
            locals.var_t0_dn10 = assign16250_body20_e23246_d_n10;
            locals.var_t0_dn11 = assign16250_body20_e23246_d_n11;
            locals.var_t0_dn12 = assign16250_body20_e23246_d_n12;
            locals.var_t0_dn17 = assign16250_body20_e23246_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16250_body21_e23276, assign16250_body21_e23276_d_n0, assign16250_body21_e23276_d_n2, assign16250_body21_e23276_d_n6, assign16250_body21_e23276_d_n7, assign16250_body21_e23276_d_n10, assign16250_body21_e23276_d_n11, assign16250_body21_e23276_d_n12, assign16250_body21_e23276_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16250_body21_e23260: f64 = (locals.var_chi / 2.0);
        let assign16250_body21_e23264: f64 = (locals.var_chi / 3.0);
        let assign16250_body21_e23268: f64 = (locals.var_chi / 4.0);
        let assign16250_body21_e23269: f64 = (1.0 - assign16250_body21_e23268);
        let assign16250_body21_e23270: f64 = (assign16250_body21_e23264 * assign16250_body21_e23269);
        let assign16250_body21_e23271: f64 = (1.0 - assign16250_body21_e23270);
        let assign16250_body21_e23272: f64 = (assign16250_body21_e23260 * assign16250_body21_e23271);
        let assign16250_body21_e23273: f64 = (1.0 - assign16250_body21_e23272);
        let assign16250_body21_e23274: f64 = (locals.var_chi * assign16250_body21_e23273);
        (assign16250_body21_e23274, ((locals.var_chi_dn0 * assign16250_body21_e23273) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign16250_body21_e23271) + (assign16250_body21_e23260 * (-(((locals.var_chi_dn0 / 3.0) * assign16250_body21_e23269) + (assign16250_body21_e23264 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign16250_body21_e23273) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign16250_body21_e23271) + (assign16250_body21_e23260 * (-(((locals.var_chi_dn2 / 3.0) * assign16250_body21_e23269) + (assign16250_body21_e23264 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn6 * assign16250_body21_e23273) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign16250_body21_e23271) + (assign16250_body21_e23260 * (-(((locals.var_chi_dn6 / 3.0) * assign16250_body21_e23269) + (assign16250_body21_e23264 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign16250_body21_e23273) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign16250_body21_e23271) + (assign16250_body21_e23260 * (-(((locals.var_chi_dn7 / 3.0) * assign16250_body21_e23269) + (assign16250_body21_e23264 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn10 * assign16250_body21_e23273) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign16250_body21_e23271) + (assign16250_body21_e23260 * (-(((locals.var_chi_dn10 / 3.0) * assign16250_body21_e23269) + (assign16250_body21_e23264 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign16250_body21_e23273) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign16250_body21_e23271) + (assign16250_body21_e23260 * (-(((locals.var_chi_dn11 / 3.0) * assign16250_body21_e23269) + (assign16250_body21_e23264 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn12 * assign16250_body21_e23273) + (locals.var_chi * (-(((locals.var_chi_dn12 / 2.0) * assign16250_body21_e23271) + (assign16250_body21_e23260 * (-(((locals.var_chi_dn12 / 3.0) * assign16250_body21_e23269) + (assign16250_body21_e23264 * (-(locals.var_chi_dn12 / 4.0)))))))))), ((locals.var_chi_dn17 * assign16250_body21_e23273) + (locals.var_chi * (-(((locals.var_chi_dn17 / 2.0) * assign16250_body21_e23271) + (assign16250_body21_e23260 * (-(((locals.var_chi_dn17 / 3.0) * assign16250_body21_e23269) + (assign16250_body21_e23264 * (-(locals.var_chi_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16250_body21_e23276;
            locals.var_t1_dn0 = assign16250_body21_e23276_d_n0;
            locals.var_t1_dn2 = assign16250_body21_e23276_d_n2;
            locals.var_t1_dn6 = assign16250_body21_e23276_d_n6;
            locals.var_t1_dn7 = assign16250_body21_e23276_d_n7;
            locals.var_t1_dn10 = assign16250_body21_e23276_d_n10;
            locals.var_t1_dn11 = assign16250_body21_e23276_d_n11;
            locals.var_t1_dn12 = assign16250_body21_e23276_d_n12;
            locals.var_t1_dn17 = assign16250_body21_e23276_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16250_body22_e23310, assign16250_body22_e23310_d_n0, assign16250_body22_e23310_d_n2, assign16250_body22_e23310_d_n6, assign16250_body22_e23310_d_n7, assign16250_body22_e23310_d_n10, assign16250_body22_e23310_d_n11, assign16250_body22_e23310_d_n12, assign16250_body22_e23310_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16250_body22_e23288: f64 = (locals.var_chib * locals.var_chib);
        let assign16250_body22_e23290: f64 = (assign16250_body22_e23288 / 2.0);
        let assign16250_body22_e23294: f64 = (locals.var_chib / 3.0);
        let assign16250_body22_e23298: f64 = (locals.var_chib / 4.0);
        let assign16250_body22_e23302: f64 = (locals.var_chib / 5.0);
        let assign16250_body22_e23303: f64 = (1.0 - assign16250_body22_e23302);
        let assign16250_body22_e23304: f64 = (assign16250_body22_e23298 * assign16250_body22_e23303);
        let assign16250_body22_e23305: f64 = (1.0 - assign16250_body22_e23304);
        let assign16250_body22_e23306: f64 = (assign16250_body22_e23294 * assign16250_body22_e23305);
        let assign16250_body22_e23307: f64 = (1.0 - assign16250_body22_e23306);
        let assign16250_body22_e23308: f64 = (assign16250_body22_e23290 * assign16250_body22_e23307);
        (assign16250_body22_e23308, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign16250_body22_e23307) + (assign16250_body22_e23290 * (-(((locals.var_chib_dn0 / 3.0) * assign16250_body22_e23305) + (assign16250_body22_e23294 * (-(((locals.var_chib_dn0 / 4.0) * assign16250_body22_e23303) + (assign16250_body22_e23298 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign16250_body22_e23307) + (assign16250_body22_e23290 * (-(((locals.var_chib_dn2 / 3.0) * assign16250_body22_e23305) + (assign16250_body22_e23294 * (-(((locals.var_chib_dn2 / 4.0) * assign16250_body22_e23303) + (assign16250_body22_e23298 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign16250_body22_e23307) + (assign16250_body22_e23290 * (-(((locals.var_chib_dn6 / 3.0) * assign16250_body22_e23305) + (assign16250_body22_e23294 * (-(((locals.var_chib_dn6 / 4.0) * assign16250_body22_e23303) + (assign16250_body22_e23298 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign16250_body22_e23307) + (assign16250_body22_e23290 * (-(((locals.var_chib_dn7 / 3.0) * assign16250_body22_e23305) + (assign16250_body22_e23294 * (-(((locals.var_chib_dn7 / 4.0) * assign16250_body22_e23303) + (assign16250_body22_e23298 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign16250_body22_e23307) + (assign16250_body22_e23290 * (-(((locals.var_chib_dn10 / 3.0) * assign16250_body22_e23305) + (assign16250_body22_e23294 * (-(((locals.var_chib_dn10 / 4.0) * assign16250_body22_e23303) + (assign16250_body22_e23298 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign16250_body22_e23307) + (assign16250_body22_e23290 * (-(((locals.var_chib_dn11 / 3.0) * assign16250_body22_e23305) + (assign16250_body22_e23294 * (-(((locals.var_chib_dn11 / 4.0) * assign16250_body22_e23303) + (assign16250_body22_e23298 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn12 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn12)) / 2.0) * assign16250_body22_e23307) + (assign16250_body22_e23290 * (-(((locals.var_chib_dn12 / 3.0) * assign16250_body22_e23305) + (assign16250_body22_e23294 * (-(((locals.var_chib_dn12 / 4.0) * assign16250_body22_e23303) + (assign16250_body22_e23298 * (-(locals.var_chib_dn12 / 5.0)))))))))), (((((locals.var_chib_dn17 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn17)) / 2.0) * assign16250_body22_e23307) + (assign16250_body22_e23290 * (-(((locals.var_chib_dn17 / 3.0) * assign16250_body22_e23305) + (assign16250_body22_e23294 * (-(((locals.var_chib_dn17 / 4.0) * assign16250_body22_e23303) + (assign16250_body22_e23298 * (-(locals.var_chib_dn17 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
            locals.var_t2 = assign16250_body22_e23310;
            locals.var_t2_dn0 = assign16250_body22_e23310_d_n0;
            locals.var_t2_dn2 = assign16250_body22_e23310_d_n2;
            locals.var_t2_dn6 = assign16250_body22_e23310_d_n6;
            locals.var_t2_dn7 = assign16250_body22_e23310_d_n7;
            locals.var_t2_dn10 = assign16250_body22_e23310_d_n10;
            locals.var_t2_dn11 = assign16250_body22_e23310_d_n11;
            locals.var_t2_dn12 = assign16250_body22_e23310_d_n12;
            locals.var_t2_dn17 = assign16250_body22_e23310_d_n17;
            locals.var_t2_rv = 0.0;
            let (assign16250_body23_e23340, assign16250_body23_e23340_d_n0, assign16250_body23_e23340_d_n2, assign16250_body23_e23340_d_n6, assign16250_body23_e23340_d_n7, assign16250_body23_e23340_d_n10, assign16250_body23_e23340_d_n11, assign16250_body23_e23340_d_n12, assign16250_body23_e23340_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16250_body23_e23324: f64 = (locals.var_chib / 2.0);
        let assign16250_body23_e23328: f64 = (locals.var_chib / 3.0);
        let assign16250_body23_e23332: f64 = (locals.var_chib / 4.0);
        let assign16250_body23_e23333: f64 = (1.0 - assign16250_body23_e23332);
        let assign16250_body23_e23334: f64 = (assign16250_body23_e23328 * assign16250_body23_e23333);
        let assign16250_body23_e23335: f64 = (1.0 - assign16250_body23_e23334);
        let assign16250_body23_e23336: f64 = (assign16250_body23_e23324 * assign16250_body23_e23335);
        let assign16250_body23_e23337: f64 = (1.0 - assign16250_body23_e23336);
        let assign16250_body23_e23338: f64 = (locals.var_chib * assign16250_body23_e23337);
        (assign16250_body23_e23338, ((locals.var_chib_dn0 * assign16250_body23_e23337) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign16250_body23_e23335) + (assign16250_body23_e23324 * (-(((locals.var_chib_dn0 / 3.0) * assign16250_body23_e23333) + (assign16250_body23_e23328 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign16250_body23_e23337) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign16250_body23_e23335) + (assign16250_body23_e23324 * (-(((locals.var_chib_dn2 / 3.0) * assign16250_body23_e23333) + (assign16250_body23_e23328 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn6 * assign16250_body23_e23337) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign16250_body23_e23335) + (assign16250_body23_e23324 * (-(((locals.var_chib_dn6 / 3.0) * assign16250_body23_e23333) + (assign16250_body23_e23328 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign16250_body23_e23337) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign16250_body23_e23335) + (assign16250_body23_e23324 * (-(((locals.var_chib_dn7 / 3.0) * assign16250_body23_e23333) + (assign16250_body23_e23328 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn10 * assign16250_body23_e23337) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign16250_body23_e23335) + (assign16250_body23_e23324 * (-(((locals.var_chib_dn10 / 3.0) * assign16250_body23_e23333) + (assign16250_body23_e23328 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign16250_body23_e23337) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign16250_body23_e23335) + (assign16250_body23_e23324 * (-(((locals.var_chib_dn11 / 3.0) * assign16250_body23_e23333) + (assign16250_body23_e23328 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn12 * assign16250_body23_e23337) + (locals.var_chib * (-(((locals.var_chib_dn12 / 2.0) * assign16250_body23_e23335) + (assign16250_body23_e23324 * (-(((locals.var_chib_dn12 / 3.0) * assign16250_body23_e23333) + (assign16250_body23_e23328 * (-(locals.var_chib_dn12 / 4.0)))))))))), ((locals.var_chib_dn17 * assign16250_body23_e23337) + (locals.var_chib * (-(((locals.var_chib_dn17 / 2.0) * assign16250_body23_e23335) + (assign16250_body23_e23324 * (-(((locals.var_chib_dn17 / 3.0) * assign16250_body23_e23333) + (assign16250_body23_e23328 * (-(locals.var_chib_dn17 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
            locals.var_t3 = assign16250_body23_e23340;
            locals.var_t3_dn0 = assign16250_body23_e23340_d_n0;
            locals.var_t3_dn2 = assign16250_body23_e23340_d_n2;
            locals.var_t3_dn6 = assign16250_body23_e23340_d_n6;
            locals.var_t3_dn7 = assign16250_body23_e23340_d_n7;
            locals.var_t3_dn10 = assign16250_body23_e23340_d_n10;
            locals.var_t3_dn11 = assign16250_body23_e23340_d_n11;
            locals.var_t3_dn12 = assign16250_body23_e23340_d_n12;
            locals.var_t3_dn17 = assign16250_body23_e23340_d_n17;
            locals.var_t3_rv = 0.0;
            let (assign16250_body24_e23355, assign16250_body24_e23355_d_n0, assign16250_body24_e23355_d_n2, assign16250_body24_e23355_d_n6, assign16250_body24_e23355_d_n7, assign16250_body24_e23355_d_n10, assign16250_body24_e23355_d_n11, assign16250_body24_e23355_d_n12, assign16250_body24_e23355_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16250_body24_e23352: f64 = (locals.var_t0 - locals.var_t2);
        let assign16250_body24_e23353: f64 = (assign16250_body24_e23352).sqrt();
        (assign16250_body24_e23353, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign16250_body24_e23353)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign16250_body24_e23353)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign16250_body24_e23353)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign16250_body24_e23353)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign16250_body24_e23353)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign16250_body24_e23353)), ((locals.var_t0_dn12 - locals.var_t2_dn12) / (2.0 * assign16250_body24_e23353)), ((locals.var_t0_dn17 - locals.var_t2_dn17) / (2.0 * assign16250_body24_e23353)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16250_body24_e23355;
            locals.var_fb_dn0 = assign16250_body24_e23355_d_n0;
            locals.var_fb_dn2 = assign16250_body24_e23355_d_n2;
            locals.var_fb_dn6 = assign16250_body24_e23355_d_n6;
            locals.var_fb_dn7 = assign16250_body24_e23355_d_n7;
            locals.var_fb_dn10 = assign16250_body24_e23355_d_n10;
            locals.var_fb_dn11 = assign16250_body24_e23355_d_n11;
            locals.var_fb_dn12 = assign16250_body24_e23355_d_n12;
            locals.var_fb_dn17 = assign16250_body24_e23355_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16250_body25_e23377, assign16250_body25_e23377_d_n0, assign16250_body25_e23377_d_n2, assign16250_body25_e23377_d_n6, assign16250_body25_e23377_d_n7, assign16250_body25_e23377_d_n10, assign16250_body25_e23377_d_n11, assign16250_body25_e23377_d_n12, assign16250_body25_e23377_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 != 0.0)) {
        let assign16250_body25_e23367: f64 = (locals.var_beta * 0.5);
        let assign16250_body25_e23371: f64 = (locals.var_phi_soib_dpss * locals.var_t3);
        let assign16250_body25_e23372: f64 = (locals.var_t1 - assign16250_body25_e23371);
        let assign16250_body25_e23373: f64 = (assign16250_body25_e23367 * assign16250_body25_e23372);
        let assign16250_body25_e23375: f64 = (assign16250_body25_e23373 / locals.var_fb);
        (assign16250_body25_e23375, ((((assign16250_body25_e23367 * (locals.var_t1_dn0 - ((locals.var_phi_soib_dpss_dn0 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn0)))) * locals.var_fb) - (assign16250_body25_e23373 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body25_e23367 * (locals.var_t1_dn2 - ((locals.var_phi_soib_dpss_dn2 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn2)))) * locals.var_fb) - (assign16250_body25_e23373 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body25_e23367 * (locals.var_t1_dn6 - ((locals.var_phi_soib_dpss_dn6 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn6)))) * locals.var_fb) - (assign16250_body25_e23373 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body25_e23367 * (locals.var_t1_dn7 - ((locals.var_phi_soib_dpss_dn7 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn7)))) * locals.var_fb) - (assign16250_body25_e23373 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign16250_body25_e23372) + (assign16250_body25_e23367 * (locals.var_t1_dn10 - ((locals.var_phi_soib_dpss_dn10 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign16250_body25_e23373 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body25_e23367 * (locals.var_t1_dn11 - ((locals.var_phi_soib_dpss_dn11 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn11)))) * locals.var_fb) - (assign16250_body25_e23373 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body25_e23367 * (locals.var_t1_dn12 - ((locals.var_phi_soib_dpss_dn12 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn12)))) * locals.var_fb) - (assign16250_body25_e23373 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body25_e23367 * (locals.var_t1_dn17 - ((locals.var_phi_soib_dpss_dn17 * locals.var_t3) + (locals.var_phi_soib_dpss * locals.var_t3_dn17)))) * locals.var_fb) - (assign16250_body25_e23373 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16250_body25_e23377;
            locals.var_fb_dpss_dn0 = assign16250_body25_e23377_d_n0;
            locals.var_fb_dpss_dn2 = assign16250_body25_e23377_d_n2;
            locals.var_fb_dpss_dn6 = assign16250_body25_e23377_d_n6;
            locals.var_fb_dpss_dn7 = assign16250_body25_e23377_d_n7;
            locals.var_fb_dpss_dn10 = assign16250_body25_e23377_d_n10;
            locals.var_fb_dpss_dn11 = assign16250_body25_e23377_d_n11;
            locals.var_fb_dpss_dn12 = assign16250_body25_e23377_d_n12;
            locals.var_fb_dpss_dn17 = assign16250_body25_e23377_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let (assign16250_body26_e23392, assign16250_body26_e23392_d_n0, assign16250_body26_e23392_d_n2, assign16250_body26_e23392_d_n6, assign16250_body26_e23392_d_n7, assign16250_body26_e23392_d_n10, assign16250_body26_e23392_d_n11, assign16250_body26_e23392_d_n12, assign16250_body26_e23392_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16250_body26_e23389: f64 = (-locals.var_chi);
        let assign16250_body26_e23390: f64 = (assign16250_body26_e23389).exp();
        (assign16250_body26_e23390, (assign16250_body26_e23390 * (-locals.var_chi_dn0)), (assign16250_body26_e23390 * (-locals.var_chi_dn2)), (assign16250_body26_e23390 * (-locals.var_chi_dn6)), (assign16250_body26_e23390 * (-locals.var_chi_dn7)), (assign16250_body26_e23390 * (-locals.var_chi_dn10)), (assign16250_body26_e23390 * (-locals.var_chi_dn11)), (assign16250_body26_e23390 * (-locals.var_chi_dn12)), (assign16250_body26_e23390 * (-locals.var_chi_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign16250_body26_e23392;
            locals.var_t0_dn0 = assign16250_body26_e23392_d_n0;
            locals.var_t0_dn2 = assign16250_body26_e23392_d_n2;
            locals.var_t0_dn6 = assign16250_body26_e23392_d_n6;
            locals.var_t0_dn7 = assign16250_body26_e23392_d_n7;
            locals.var_t0_dn10 = assign16250_body26_e23392_d_n10;
            locals.var_t0_dn11 = assign16250_body26_e23392_d_n11;
            locals.var_t0_dn12 = assign16250_body26_e23392_d_n12;
            locals.var_t0_dn17 = assign16250_body26_e23392_d_n17;
            locals.var_t0_rv = 0.0;
            let (assign16250_body27_e23407, assign16250_body27_e23407_d_n0, assign16250_body27_e23407_d_n2, assign16250_body27_e23407_d_n6, assign16250_body27_e23407_d_n7, assign16250_body27_e23407_d_n10, assign16250_body27_e23407_d_n11, assign16250_body27_e23407_d_n12, assign16250_body27_e23407_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16250_body27_e23404: f64 = (-locals.var_chib);
        let assign16250_body27_e23405: f64 = (assign16250_body27_e23404).exp();
        (assign16250_body27_e23405, (assign16250_body27_e23405 * (-locals.var_chib_dn0)), (assign16250_body27_e23405 * (-locals.var_chib_dn2)), (assign16250_body27_e23405 * (-locals.var_chib_dn6)), (assign16250_body27_e23405 * (-locals.var_chib_dn7)), (assign16250_body27_e23405 * (-locals.var_chib_dn10)), (assign16250_body27_e23405 * (-locals.var_chib_dn11)), (assign16250_body27_e23405 * (-locals.var_chib_dn12)), (assign16250_body27_e23405 * (-locals.var_chib_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign16250_body27_e23407;
            locals.var_t1_dn0 = assign16250_body27_e23407_d_n0;
            locals.var_t1_dn2 = assign16250_body27_e23407_d_n2;
            locals.var_t1_dn6 = assign16250_body27_e23407_d_n6;
            locals.var_t1_dn7 = assign16250_body27_e23407_d_n7;
            locals.var_t1_dn10 = assign16250_body27_e23407_d_n10;
            locals.var_t1_dn11 = assign16250_body27_e23407_d_n11;
            locals.var_t1_dn12 = assign16250_body27_e23407_d_n12;
            locals.var_t1_dn17 = assign16250_body27_e23407_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign16250_body28_e23427, assign16250_body28_e23427_d_n0, assign16250_body28_e23427_d_n2, assign16250_body28_e23427_d_n6, assign16250_body28_e23427_d_n7, assign16250_body28_e23427_d_n10, assign16250_body28_e23427_d_n11, assign16250_body28_e23427_d_n12, assign16250_body28_e23427_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16250_body28_e23420: f64 = (locals.var_chi - locals.var_chib);
        let assign16250_body28_e23423: f64 = (locals.var_t0 - locals.var_t1);
        let assign16250_body28_e23424: f64 = (assign16250_body28_e23420 + assign16250_body28_e23423);
        let assign16250_body28_e23425: f64 = (assign16250_body28_e23424).sqrt();
        (assign16250_body28_e23425, (((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign16250_body28_e23425)), (((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign16250_body28_e23425)), (((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign16250_body28_e23425)), (((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign16250_body28_e23425)), (((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign16250_body28_e23425)), (((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign16250_body28_e23425)), (((locals.var_chi_dn12 - locals.var_chib_dn12) + (locals.var_t0_dn12 - locals.var_t1_dn12)) / (2.0 * assign16250_body28_e23425)), (((locals.var_chi_dn17 - locals.var_chib_dn17) + (locals.var_t0_dn17 - locals.var_t1_dn17)) / (2.0 * assign16250_body28_e23425)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    }
};
            locals.var_fb = assign16250_body28_e23427;
            locals.var_fb_dn0 = assign16250_body28_e23427_d_n0;
            locals.var_fb_dn2 = assign16250_body28_e23427_d_n2;
            locals.var_fb_dn6 = assign16250_body28_e23427_d_n6;
            locals.var_fb_dn7 = assign16250_body28_e23427_d_n7;
            locals.var_fb_dn10 = assign16250_body28_e23427_d_n10;
            locals.var_fb_dn11 = assign16250_body28_e23427_d_n11;
            locals.var_fb_dn12 = assign16250_body28_e23427_d_n12;
            locals.var_fb_dn17 = assign16250_body28_e23427_d_n17;
            locals.var_fb_rv = 0.0;
            let (assign16250_body29_e23454, assign16250_body29_e23454_d_n0, assign16250_body29_e23454_d_n2, assign16250_body29_e23454_d_n6, assign16250_body29_e23454_d_n7, assign16250_body29_e23454_d_n10, assign16250_body29_e23454_d_n11, assign16250_body29_e23454_d_n12, assign16250_body29_e23454_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard486 == 0.0)) && (locals.var_guard488 == 0.0)) {
        let assign16250_body29_e23440: f64 = (locals.var_beta * 0.5);
        let assign16250_body29_e23443: f64 = (1.0 - locals.var_t0);
        let assign16250_body29_e23447: f64 = (1.0 - locals.var_t1);
        let assign16250_body29_e23448: f64 = (locals.var_phi_soib_dpss * assign16250_body29_e23447);
        let assign16250_body29_e23449: f64 = (assign16250_body29_e23443 - assign16250_body29_e23448);
        let assign16250_body29_e23450: f64 = (assign16250_body29_e23440 * assign16250_body29_e23449);
        let assign16250_body29_e23452: f64 = (assign16250_body29_e23450 / locals.var_fb);
        (assign16250_body29_e23452, ((((assign16250_body29_e23440 * ((-locals.var_t0_dn0) - ((locals.var_phi_soib_dpss_dn0 * assign16250_body29_e23447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn0))))) * locals.var_fb) - (assign16250_body29_e23450 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body29_e23440 * ((-locals.var_t0_dn2) - ((locals.var_phi_soib_dpss_dn2 * assign16250_body29_e23447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn2))))) * locals.var_fb) - (assign16250_body29_e23450 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body29_e23440 * ((-locals.var_t0_dn6) - ((locals.var_phi_soib_dpss_dn6 * assign16250_body29_e23447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn6))))) * locals.var_fb) - (assign16250_body29_e23450 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body29_e23440 * ((-locals.var_t0_dn7) - ((locals.var_phi_soib_dpss_dn7 * assign16250_body29_e23447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn7))))) * locals.var_fb) - (assign16250_body29_e23450 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign16250_body29_e23449) + (assign16250_body29_e23440 * ((-locals.var_t0_dn10) - ((locals.var_phi_soib_dpss_dn10 * assign16250_body29_e23447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign16250_body29_e23450 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body29_e23440 * ((-locals.var_t0_dn11) - ((locals.var_phi_soib_dpss_dn11 * assign16250_body29_e23447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn11))))) * locals.var_fb) - (assign16250_body29_e23450 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body29_e23440 * ((-locals.var_t0_dn12) - ((locals.var_phi_soib_dpss_dn12 * assign16250_body29_e23447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn12))))) * locals.var_fb) - (assign16250_body29_e23450 * locals.var_fb_dn12)) / (locals.var_fb * locals.var_fb)), ((((assign16250_body29_e23440 * ((-locals.var_t0_dn17) - ((locals.var_phi_soib_dpss_dn17 * assign16250_body29_e23447) + (locals.var_phi_soib_dpss * (-locals.var_t1_dn17))))) * locals.var_fb) - (assign16250_body29_e23450 * locals.var_fb_dn17)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    }
};
            locals.var_fb_dpss = assign16250_body29_e23454;
            locals.var_fb_dpss_dn0 = assign16250_body29_e23454_d_n0;
            locals.var_fb_dpss_dn2 = assign16250_body29_e23454_d_n2;
            locals.var_fb_dpss_dn6 = assign16250_body29_e23454_d_n6;
            locals.var_fb_dpss_dn7 = assign16250_body29_e23454_d_n7;
            locals.var_fb_dpss_dn10 = assign16250_body29_e23454_d_n10;
            locals.var_fb_dpss_dn11 = assign16250_body29_e23454_d_n11;
            locals.var_fb_dpss_dn12 = assign16250_body29_e23454_d_n12;
            locals.var_fb_dpss_dn17 = assign16250_body29_e23454_d_n17;
            locals.var_fb_dpss_rv = 0.0;
            let assign16250_body30_e23457: f64 = (-1.0);
            let assign16250_body30_e23458: f64 = if locals.var_flg_zone == assign16250_body30_e23457 { 1.0 } else { 0.0 };
            locals.var_guard489 = assign16250_body30_e23458;
            locals.var_guard489_rv = 0.0;
            let (assign16250_body31_e23467, assign16250_body31_e23467_d_n0, assign16250_body31_e23467_d_n2, assign16250_body31_e23467_d_n6, assign16250_body31_e23467_d_n7, assign16250_body31_e23467_d_n10, assign16250_body31_e23467_d_n11, assign16250_body31_e23467_d_n12, assign16250_body31_e23467_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard489 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign16250_body31_e23467;
            locals.var_wdsoi_dn0 = assign16250_body31_e23467_d_n0;
            locals.var_wdsoi_dn2 = assign16250_body31_e23467_d_n2;
            locals.var_wdsoi_dn6 = assign16250_body31_e23467_d_n6;
            locals.var_wdsoi_dn7 = assign16250_body31_e23467_d_n7;
            locals.var_wdsoi_dn10 = assign16250_body31_e23467_d_n10;
            locals.var_wdsoi_dn11 = assign16250_body31_e23467_d_n11;
            locals.var_wdsoi_dn12 = assign16250_body31_e23467_d_n12;
            locals.var_wdsoi_dn17 = assign16250_body31_e23467_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign16250_body32_e23479, assign16250_body32_e23479_d_n0, assign16250_body32_e23479_d_n2, assign16250_body32_e23479_d_n6, assign16250_body32_e23479_d_n7, assign16250_body32_e23479_d_n10, assign16250_body32_e23479_d_n11, assign16250_body32_e23479_d_n12, assign16250_body32_e23479_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard489 == 0.0)) {
        let assign16250_body32_e23477: f64 = (locals.var_c_w_soi * locals.var_fb);
        (assign16250_body32_e23477, ((locals.var_c_w_soi_dn0 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn0)), ((locals.var_c_w_soi_dn2 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn2)), ((locals.var_c_w_soi_dn6 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn6)), ((locals.var_c_w_soi_dn7 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn7)), ((locals.var_c_w_soi_dn10 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn10)), ((locals.var_c_w_soi_dn11 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn11)), ((locals.var_c_w_soi_dn12 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn12)), ((locals.var_c_w_soi_dn17 * locals.var_fb) + (locals.var_c_w_soi * locals.var_fb_dn17)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
            locals.var_wdsoi = assign16250_body32_e23479;
            locals.var_wdsoi_dn0 = assign16250_body32_e23479_d_n0;
            locals.var_wdsoi_dn2 = assign16250_body32_e23479_d_n2;
            locals.var_wdsoi_dn6 = assign16250_body32_e23479_d_n6;
            locals.var_wdsoi_dn7 = assign16250_body32_e23479_d_n7;
            locals.var_wdsoi_dn10 = assign16250_body32_e23479_d_n10;
            locals.var_wdsoi_dn11 = assign16250_body32_e23479_d_n11;
            locals.var_wdsoi_dn12 = assign16250_body32_e23479_d_n12;
            locals.var_wdsoi_dn17 = assign16250_body32_e23479_d_n17;
            locals.var_wdsoi_rv = 0.0;
            let (assign16250_body33_e23488, assign16250_body33_e23488_d_n0, assign16250_body33_e23488_d_n2, assign16250_body33_e23488_d_n6, assign16250_body33_e23488_d_n7, assign16250_body33_e23488_d_n10, assign16250_body33_e23488_d_n11, assign16250_body33_e23488_d_n12, assign16250_body33_e23488_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16250_body33_e23486: f64 = (locals.var_q_nsub * locals.var_wdsoi);
        (assign16250_body33_e23486, ((locals.var_q_nsub_dn0 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn0)), ((locals.var_q_nsub_dn2 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn2)), ((locals.var_q_nsub_dn6 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn6)), ((locals.var_q_nsub_dn7 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn7)), ((locals.var_q_nsub_dn10 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn10)), ((locals.var_q_nsub_dn11 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn11)), ((locals.var_q_nsub_dn12 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn12)), ((locals.var_q_nsub_dn17 * locals.var_wdsoi) + (locals.var_q_nsub * locals.var_wdsoi_dn17)),)
    } else {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    }
};
            locals.var_q_dep_soi = assign16250_body33_e23488;
            locals.var_q_dep_soi_dn0 = assign16250_body33_e23488_d_n0;
            locals.var_q_dep_soi_dn2 = assign16250_body33_e23488_d_n2;
            locals.var_q_dep_soi_dn6 = assign16250_body33_e23488_d_n6;
            locals.var_q_dep_soi_dn7 = assign16250_body33_e23488_d_n7;
            locals.var_q_dep_soi_dn10 = assign16250_body33_e23488_d_n10;
            locals.var_q_dep_soi_dn11 = assign16250_body33_e23488_d_n11;
            locals.var_q_dep_soi_dn12 = assign16250_body33_e23488_d_n12;
            locals.var_q_dep_soi_dn17 = assign16250_body33_e23488_d_n17;
            locals.var_q_dep_soi_rv = 0.0;
            let assign16250_body34_e23491: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard490 = assign16250_body34_e23491;
            locals.var_guard490_rv = 0.0;
            let (assign16250_body35_e23501, assign16250_body35_e23501_d_n0, assign16250_body35_e23501_d_n2, assign16250_body35_e23501_d_n6, assign16250_body35_e23501_d_n7, assign16250_body35_e23501_d_n10, assign16250_body35_e23501_d_n11, assign16250_body35_e23501_d_n12, assign16250_body35_e23501_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 != 0.0)) {
        let assign16250_body35_e23499: f64 = (-locals.var_fb);
        (assign16250_body35_e23499, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn12), (-locals.var_fb_dn17),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16250_body35_e23501;
            locals.var_fsl2_dn0 = assign16250_body35_e23501_d_n0;
            locals.var_fsl2_dn2 = assign16250_body35_e23501_d_n2;
            locals.var_fsl2_dn6 = assign16250_body35_e23501_d_n6;
            locals.var_fsl2_dn7 = assign16250_body35_e23501_d_n7;
            locals.var_fsl2_dn10 = assign16250_body35_e23501_d_n10;
            locals.var_fsl2_dn11 = assign16250_body35_e23501_d_n11;
            locals.var_fsl2_dn12 = assign16250_body35_e23501_d_n12;
            locals.var_fsl2_dn17 = assign16250_body35_e23501_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16250_body36_e23511, assign16250_body36_e23511_d_n0, assign16250_body36_e23511_d_n2, assign16250_body36_e23511_d_n6, assign16250_body36_e23511_d_n7, assign16250_body36_e23511_d_n10, assign16250_body36_e23511_d_n11, assign16250_body36_e23511_d_n12, assign16250_body36_e23511_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 != 0.0)) {
        let assign16250_body36_e23509: f64 = (-locals.var_fb_dpss);
        (assign16250_body36_e23509, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn12), (-locals.var_fb_dpss_dn17),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16250_body36_e23511;
            locals.var_fsl2_dpsl_dn0 = assign16250_body36_e23511_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16250_body36_e23511_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16250_body36_e23511_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16250_body36_e23511_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16250_body36_e23511_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16250_body36_e23511_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16250_body36_e23511_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16250_body36_e23511_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let assign16250_body37_e23514: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard491 = assign16250_body37_e23514;
            locals.var_guard491_rv = 0.0;
            let (assign16250_body38_e23526, assign16250_body38_e23526_d_n0, assign16250_body38_e23526_d_n2, assign16250_body38_e23526_d_n6, assign16250_body38_e23526_d_n7, assign16250_body38_e23526_d_n10, assign16250_body38_e23526_d_n11, assign16250_body38_e23526_d_n12, assign16250_body38_e23526_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, locals.var_fb_dn17,)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16250_body38_e23526;
            locals.var_fsl2_dn0 = assign16250_body38_e23526_d_n0;
            locals.var_fsl2_dn2 = assign16250_body38_e23526_d_n2;
            locals.var_fsl2_dn6 = assign16250_body38_e23526_d_n6;
            locals.var_fsl2_dn7 = assign16250_body38_e23526_d_n7;
            locals.var_fsl2_dn10 = assign16250_body38_e23526_d_n10;
            locals.var_fsl2_dn11 = assign16250_body38_e23526_d_n11;
            locals.var_fsl2_dn12 = assign16250_body38_e23526_d_n12;
            locals.var_fsl2_dn17 = assign16250_body38_e23526_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16250_body39_e23538, assign16250_body39_e23538_d_n0, assign16250_body39_e23538_d_n2, assign16250_body39_e23538_d_n6, assign16250_body39_e23538_d_n7, assign16250_body39_e23538_d_n10, assign16250_body39_e23538_d_n11, assign16250_body39_e23538_d_n12, assign16250_body39_e23538_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 != 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn12, locals.var_fb_dpss_dn17,)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16250_body39_e23538;
            locals.var_fsl2_dpsl_dn0 = assign16250_body39_e23538_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16250_body39_e23538_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16250_body39_e23538_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16250_body39_e23538_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16250_body39_e23538_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16250_body39_e23538_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16250_body39_e23538_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16250_body39_e23538_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign16250_body40_e23555, assign16250_body40_e23555_d_n0, assign16250_body40_e23555_d_n2, assign16250_body40_e23555_d_n6, assign16250_body40_e23555_d_n7, assign16250_body40_e23555_d_n10, assign16250_body40_e23555_d_n11, assign16250_body40_e23555_d_n12, assign16250_body40_e23555_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 == 0.0)) {
        let assign16250_body40_e23552: f64 = (locals.var_phi_sl_soi - locals.var_vds);
        let assign16250_body40_e23553: f64 = (locals.var_beta * assign16250_body40_e23552);
        (assign16250_body40_e23553, (locals.var_beta * (locals.var_phi_sl_soi_dn0 - locals.var_vds_dn0)), (locals.var_beta * (locals.var_phi_sl_soi_dn2 - locals.var_vds_dn2)), (locals.var_beta * (locals.var_phi_sl_soi_dn6 - locals.var_vds_dn6)), (locals.var_beta * (locals.var_phi_sl_soi_dn7 - locals.var_vds_dn7)), ((locals.var_beta_dn10 * assign16250_body40_e23552) + (locals.var_beta * (locals.var_phi_sl_soi_dn10 - locals.var_vds_dn10))), (locals.var_beta * (locals.var_phi_sl_soi_dn11 - locals.var_vds_dn11)), (locals.var_beta * (locals.var_phi_sl_soi_dn12 - locals.var_vds_dn12)), (locals.var_beta * (locals.var_phi_sl_soi_dn17 - locals.var_vds_dn17)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn17,)
    }
};
            locals.var_rho = assign16250_body40_e23555;
            locals.var_rho_dn0 = assign16250_body40_e23555_d_n0;
            locals.var_rho_dn2 = assign16250_body40_e23555_d_n2;
            locals.var_rho_dn6 = assign16250_body40_e23555_d_n6;
            locals.var_rho_dn7 = assign16250_body40_e23555_d_n7;
            locals.var_rho_dn10 = assign16250_body40_e23555_d_n10;
            locals.var_rho_dn11 = assign16250_body40_e23555_d_n11;
            locals.var_rho_dn12 = assign16250_body40_e23555_d_n12;
            locals.var_rho_dn17 = assign16250_body40_e23555_d_n17;
            locals.var_rho_rv = 0.0;
            let (assign16250_body41_e23569, assign16250_body41_e23569_d_n0, assign16250_body41_e23569_d_n2, assign16250_body41_e23569_d_n6, assign16250_body41_e23569_d_n7, assign16250_body41_e23569_d_n10, assign16250_body41_e23569_d_n11, assign16250_body41_e23569_d_n12, assign16250_body41_e23569_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 == 0.0)) {
        let assign16250_body41_e23567: f64 = (locals.var_rho).exp();
        (assign16250_body41_e23567, (assign16250_body41_e23567 * locals.var_rho_dn0), (assign16250_body41_e23567 * locals.var_rho_dn2), (assign16250_body41_e23567 * locals.var_rho_dn6), (assign16250_body41_e23567 * locals.var_rho_dn7), (assign16250_body41_e23567 * locals.var_rho_dn10), (assign16250_body41_e23567 * locals.var_rho_dn11), (assign16250_body41_e23567 * locals.var_rho_dn12), (assign16250_body41_e23567 * locals.var_rho_dn17),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn12, locals.var_exp_rho_dn17,)
    }
};
            locals.var_exp_rho = assign16250_body41_e23569;
            locals.var_exp_rho_dn0 = assign16250_body41_e23569_d_n0;
            locals.var_exp_rho_dn2 = assign16250_body41_e23569_d_n2;
            locals.var_exp_rho_dn6 = assign16250_body41_e23569_d_n6;
            locals.var_exp_rho_dn7 = assign16250_body41_e23569_d_n7;
            locals.var_exp_rho_dn10 = assign16250_body41_e23569_d_n10;
            locals.var_exp_rho_dn11 = assign16250_body41_e23569_d_n11;
            locals.var_exp_rho_dn12 = assign16250_body41_e23569_d_n12;
            locals.var_exp_rho_dn17 = assign16250_body41_e23569_d_n17;
            locals.var_exp_rho_rv = 0.0;
            let (assign16250_body42_e23590, assign16250_body42_e23590_d_n0, assign16250_body42_e23590_d_n2, assign16250_body42_e23590_d_n6, assign16250_body42_e23590_d_n7, assign16250_body42_e23590_d_n10, assign16250_body42_e23590_d_n11, assign16250_body42_e23590_d_n12, assign16250_body42_e23590_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 == 0.0)) {
        let assign16250_body42_e23585: f64 = (locals.var_chi + 1.0);
        let assign16250_body42_e23586: f64 = (locals.var_exp_bvbsvds * assign16250_body42_e23585);
        let assign16250_body42_e23587: f64 = (locals.var_exp_rho - assign16250_body42_e23586);
        let assign16250_body42_e23588: f64 = (locals.var_cnst1soi * assign16250_body42_e23587);
        (assign16250_body42_e23588, ((locals.var_cnst1soi_dn0 * assign16250_body42_e23587) + (locals.var_cnst1soi * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign16250_body42_e23585) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1soi_dn2 * assign16250_body42_e23587) + (locals.var_cnst1soi * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign16250_body42_e23585) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1soi_dn6 * assign16250_body42_e23587) + (locals.var_cnst1soi * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign16250_body42_e23585) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1soi_dn7 * assign16250_body42_e23587) + (locals.var_cnst1soi * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign16250_body42_e23585) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1soi_dn10 * assign16250_body42_e23587) + (locals.var_cnst1soi * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign16250_body42_e23585) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1soi_dn11 * assign16250_body42_e23587) + (locals.var_cnst1soi * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign16250_body42_e23585) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1soi_dn12 * assign16250_body42_e23587) + (locals.var_cnst1soi * (locals.var_exp_rho_dn12 - ((locals.var_exp_bvbsvds_dn12 * assign16250_body42_e23585) + (locals.var_exp_bvbsvds * locals.var_chi_dn12))))), ((locals.var_cnst1soi_dn17 * assign16250_body42_e23587) + (locals.var_cnst1soi * (locals.var_exp_rho_dn17 - ((locals.var_exp_bvbsvds_dn17 * assign16250_body42_e23585) + (locals.var_exp_bvbsvds * locals.var_chi_dn17))))),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn10, locals.var_fsl1_dn11, locals.var_fsl1_dn12, locals.var_fsl1_dn17,)
    }
};
            locals.var_fsl1 = assign16250_body42_e23590;
            locals.var_fsl1_dn0 = assign16250_body42_e23590_d_n0;
            locals.var_fsl1_dn2 = assign16250_body42_e23590_d_n2;
            locals.var_fsl1_dn6 = assign16250_body42_e23590_d_n6;
            locals.var_fsl1_dn7 = assign16250_body42_e23590_d_n7;
            locals.var_fsl1_dn10 = assign16250_body42_e23590_d_n10;
            locals.var_fsl1_dn11 = assign16250_body42_e23590_d_n11;
            locals.var_fsl1_dn12 = assign16250_body42_e23590_d_n12;
            locals.var_fsl1_dn17 = assign16250_body42_e23590_d_n17;
            locals.var_fsl1_rv = 0.0;
            let (assign16250_body43_e23609, assign16250_body43_e23609_d_n0, assign16250_body43_e23609_d_n2, assign16250_body43_e23609_d_n6, assign16250_body43_e23609_d_n7, assign16250_body43_e23609_d_n10, assign16250_body43_e23609_d_n11, assign16250_body43_e23609_d_n12, assign16250_body43_e23609_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 == 0.0)) {
        let assign16250_body43_e23603: f64 = (locals.var_cnst1soi * locals.var_beta);
        let assign16250_body43_e23606: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign16250_body43_e23607: f64 = (assign16250_body43_e23603 * assign16250_body43_e23606);
        (assign16250_body43_e23607, (((locals.var_cnst1soi_dn0 * locals.var_beta) * assign16250_body43_e23606) + (assign16250_body43_e23603 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), (((locals.var_cnst1soi_dn2 * locals.var_beta) * assign16250_body43_e23606) + (assign16250_body43_e23603 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), (((locals.var_cnst1soi_dn6 * locals.var_beta) * assign16250_body43_e23606) + (assign16250_body43_e23603 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), (((locals.var_cnst1soi_dn7 * locals.var_beta) * assign16250_body43_e23606) + (assign16250_body43_e23603 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1soi_dn10 * locals.var_beta) + (locals.var_cnst1soi * locals.var_beta_dn10)) * assign16250_body43_e23606) + (assign16250_body43_e23603 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), (((locals.var_cnst1soi_dn11 * locals.var_beta) * assign16250_body43_e23606) + (assign16250_body43_e23603 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), (((locals.var_cnst1soi_dn12 * locals.var_beta) * assign16250_body43_e23606) + (assign16250_body43_e23603 * (locals.var_exp_rho_dn12 - locals.var_exp_bvbsvds_dn12))), (((locals.var_cnst1soi_dn17 * locals.var_beta) * assign16250_body43_e23606) + (assign16250_body43_e23603 * (locals.var_exp_rho_dn17 - locals.var_exp_bvbsvds_dn17))),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn11, locals.var_fsl1_dpsl_dn12, locals.var_fsl1_dpsl_dn17,)
    }
};
            locals.var_fsl1_dpsl = assign16250_body43_e23609;
            locals.var_fsl1_dpsl_dn0 = assign16250_body43_e23609_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign16250_body43_e23609_d_n2;
            locals.var_fsl1_dpsl_dn6 = assign16250_body43_e23609_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign16250_body43_e23609_d_n7;
            locals.var_fsl1_dpsl_dn10 = assign16250_body43_e23609_d_n10;
            locals.var_fsl1_dpsl_dn11 = assign16250_body43_e23609_d_n11;
            locals.var_fsl1_dpsl_dn12 = assign16250_body43_e23609_d_n12;
            locals.var_fsl1_dpsl_dn17 = assign16250_body43_e23609_d_n17;
            locals.var_fsl1_dpsl_rv = 0.0;
            let (assign16250_body44_e23627, assign16250_body44_e23627_d_n0, assign16250_body44_e23627_d_n2, assign16250_body44_e23627_d_n6, assign16250_body44_e23627_d_n7, assign16250_body44_e23627_d_n10, assign16250_body44_e23627_d_n11, assign16250_body44_e23627_d_n12, assign16250_body44_e23627_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 == 0.0)) {
        let assign16250_body44_e23622: f64 = (locals.var_fb * locals.var_fb);
        let assign16250_body44_e23624: f64 = (assign16250_body44_e23622 + locals.var_fsl1);
        let assign16250_body44_e23625: f64 = (assign16250_body44_e23624).sqrt();
        (assign16250_body44_e23625, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fsl1_dn0) / (2.0 * assign16250_body44_e23625)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fsl1_dn2) / (2.0 * assign16250_body44_e23625)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fsl1_dn6) / (2.0 * assign16250_body44_e23625)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fsl1_dn7) / (2.0 * assign16250_body44_e23625)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fsl1_dn10) / (2.0 * assign16250_body44_e23625)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fsl1_dn11) / (2.0 * assign16250_body44_e23625)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fsl1_dn12) / (2.0 * assign16250_body44_e23625)), ((((locals.var_fb_dn17 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn17)) + locals.var_fsl1_dn17) / (2.0 * assign16250_body44_e23625)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn12, locals.var_fsl2_dn17,)
    }
};
            locals.var_fsl2 = assign16250_body44_e23627;
            locals.var_fsl2_dn0 = assign16250_body44_e23627_d_n0;
            locals.var_fsl2_dn2 = assign16250_body44_e23627_d_n2;
            locals.var_fsl2_dn6 = assign16250_body44_e23627_d_n6;
            locals.var_fsl2_dn7 = assign16250_body44_e23627_d_n7;
            locals.var_fsl2_dn10 = assign16250_body44_e23627_d_n10;
            locals.var_fsl2_dn11 = assign16250_body44_e23627_d_n11;
            locals.var_fsl2_dn12 = assign16250_body44_e23627_d_n12;
            locals.var_fsl2_dn17 = assign16250_body44_e23627_d_n17;
            locals.var_fsl2_rv = 0.0;
            let (assign16250_body45_e23650, assign16250_body45_e23650_d_n0, assign16250_body45_e23650_d_n2, assign16250_body45_e23650_d_n6, assign16250_body45_e23650_d_n7, assign16250_body45_e23650_d_n10, assign16250_body45_e23650_d_n11, assign16250_body45_e23650_d_n12, assign16250_body45_e23650_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard490 == 0.0)) && (locals.var_guard491 == 0.0)) {
        let assign16250_body45_e23641: f64 = (2.0 * locals.var_fb_dpss);
        let assign16250_body45_e23643: f64 = (assign16250_body45_e23641 * locals.var_fb);
        let assign16250_body45_e23645: f64 = (assign16250_body45_e23643 + locals.var_fsl1_dpsl);
        let assign16250_body45_e23646: f64 = (0.5 * assign16250_body45_e23645);
        let assign16250_body45_e23648: f64 = (assign16250_body45_e23646 / locals.var_fsl2);
        (assign16250_body45_e23648, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign16250_body45_e23641 * locals.var_fb_dn0)) + locals.var_fsl1_dpsl_dn0)) * locals.var_fsl2) - (assign16250_body45_e23646 * locals.var_fsl2_dn0)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign16250_body45_e23641 * locals.var_fb_dn2)) + locals.var_fsl1_dpsl_dn2)) * locals.var_fsl2) - (assign16250_body45_e23646 * locals.var_fsl2_dn2)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign16250_body45_e23641 * locals.var_fb_dn6)) + locals.var_fsl1_dpsl_dn6)) * locals.var_fsl2) - (assign16250_body45_e23646 * locals.var_fsl2_dn6)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign16250_body45_e23641 * locals.var_fb_dn7)) + locals.var_fsl1_dpsl_dn7)) * locals.var_fsl2) - (assign16250_body45_e23646 * locals.var_fsl2_dn7)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign16250_body45_e23641 * locals.var_fb_dn10)) + locals.var_fsl1_dpsl_dn10)) * locals.var_fsl2) - (assign16250_body45_e23646 * locals.var_fsl2_dn10)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign16250_body45_e23641 * locals.var_fb_dn11)) + locals.var_fsl1_dpsl_dn11)) * locals.var_fsl2) - (assign16250_body45_e23646 * locals.var_fsl2_dn11)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn12) * locals.var_fb) + (assign16250_body45_e23641 * locals.var_fb_dn12)) + locals.var_fsl1_dpsl_dn12)) * locals.var_fsl2) - (assign16250_body45_e23646 * locals.var_fsl2_dn12)) / (locals.var_fsl2 * locals.var_fsl2)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn17) * locals.var_fb) + (assign16250_body45_e23641 * locals.var_fb_dn17)) + locals.var_fsl1_dpsl_dn17)) * locals.var_fsl2) - (assign16250_body45_e23646 * locals.var_fsl2_dn17)) / (locals.var_fsl2 * locals.var_fsl2)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn12, locals.var_fsl2_dpsl_dn17,)
    }
};
            locals.var_fsl2_dpsl = assign16250_body45_e23650;
            locals.var_fsl2_dpsl_dn0 = assign16250_body45_e23650_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign16250_body45_e23650_d_n2;
            locals.var_fsl2_dpsl_dn6 = assign16250_body45_e23650_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign16250_body45_e23650_d_n7;
            locals.var_fsl2_dpsl_dn10 = assign16250_body45_e23650_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign16250_body45_e23650_d_n11;
            locals.var_fsl2_dpsl_dn12 = assign16250_body45_e23650_d_n12;
            locals.var_fsl2_dpsl_dn17 = assign16250_body45_e23650_d_n17;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign16250_body46_e23668, assign16250_body46_e23668_d_n0, assign16250_body46_e23668_d_n2, assign16250_body46_e23668_d_n6, assign16250_body46_e23668_d_n7, assign16250_body46_e23668_d_n10, assign16250_body46_e23668_d_n11, assign16250_body46_e23668_d_n12, assign16250_body46_e23668_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16250_body46_e23656: f64 = (-locals.var_vgp);
        let assign16250_body46_e23658: f64 = (assign16250_body46_e23656 + locals.var_phi_sl_soi);
        let assign16250_body46_e23661: f64 = (locals.var_fac1 * locals.var_fsl2);
        let assign16250_body46_e23662: f64 = (assign16250_body46_e23658 + assign16250_body46_e23661);
        let assign16250_body46_e23665: f64 = (locals.var_c_fox_inv * locals.var_qhs);
        let assign16250_body46_e23666: f64 = (assign16250_body46_e23662 - assign16250_body46_e23665);
        (assign16250_body46_e23666, ((((-locals.var_vgp_dn0) + locals.var_phi_sl_soi_dn0) + ((locals.var_fac1_dn0 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn0))) - ((locals.var_c_fox_inv_dn0 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn0))), ((((-locals.var_vgp_dn2) + locals.var_phi_sl_soi_dn2) + ((locals.var_fac1_dn2 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn2))) - ((locals.var_c_fox_inv_dn2 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn2))), ((((-locals.var_vgp_dn6) + locals.var_phi_sl_soi_dn6) + ((locals.var_fac1_dn6 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn6))) - ((locals.var_c_fox_inv_dn6 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn6))), ((((-locals.var_vgp_dn7) + locals.var_phi_sl_soi_dn7) + ((locals.var_fac1_dn7 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn7))) - ((locals.var_c_fox_inv_dn7 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn7))), ((((-locals.var_vgp_dn10) + locals.var_phi_sl_soi_dn10) + ((locals.var_fac1_dn10 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn10))) - ((locals.var_c_fox_inv_dn10 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn10))), ((((-locals.var_vgp_dn11) + locals.var_phi_sl_soi_dn11) + ((locals.var_fac1_dn11 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn11))) - ((locals.var_c_fox_inv_dn11 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn11))), ((((-locals.var_vgp_dn12) + locals.var_phi_sl_soi_dn12) + ((locals.var_fac1_dn12 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn12))) - ((locals.var_c_fox_inv_dn12 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn12))), ((((-locals.var_vgp_dn17) + locals.var_phi_sl_soi_dn17) + ((locals.var_fac1_dn17 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn17))) - ((locals.var_c_fox_inv_dn17 * locals.var_qhs) + (locals.var_c_fox_inv * locals.var_qhs_dn17))),)
    } else {
        (locals.var_fsl, locals.var_fsl_dn0, locals.var_fsl_dn2, locals.var_fsl_dn6, locals.var_fsl_dn7, locals.var_fsl_dn10, locals.var_fsl_dn11, locals.var_fsl_dn12, locals.var_fsl_dn17,)
    }
};
            locals.var_fsl = assign16250_body46_e23668;
            locals.var_fsl_dn0 = assign16250_body46_e23668_d_n0;
            locals.var_fsl_dn2 = assign16250_body46_e23668_d_n2;
            locals.var_fsl_dn6 = assign16250_body46_e23668_d_n6;
            locals.var_fsl_dn7 = assign16250_body46_e23668_d_n7;
            locals.var_fsl_dn10 = assign16250_body46_e23668_d_n10;
            locals.var_fsl_dn11 = assign16250_body46_e23668_d_n11;
            locals.var_fsl_dn12 = assign16250_body46_e23668_d_n12;
            locals.var_fsl_dn17 = assign16250_body46_e23668_d_n17;
            locals.var_fsl_rv = 0.0;
            let (assign16250_body47_e23679, assign16250_body47_e23679_d_n0, assign16250_body47_e23679_d_n2, assign16250_body47_e23679_d_n6, assign16250_body47_e23679_d_n7, assign16250_body47_e23679_d_n10, assign16250_body47_e23679_d_n11, assign16250_body47_e23679_d_n12, assign16250_body47_e23679_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16250_body47_e23676: f64 = (locals.var_fac1 * locals.var_fsl2_dpsl);
        let assign16250_body47_e23677: f64 = (1.0 + assign16250_body47_e23676);
        (assign16250_body47_e23677, ((locals.var_fac1_dn0 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn0)), ((locals.var_fac1_dn2 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn2)), ((locals.var_fac1_dn6 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn6)), ((locals.var_fac1_dn7 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn7)), ((locals.var_fac1_dn10 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn10)), ((locals.var_fac1_dn11 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn11)), ((locals.var_fac1_dn12 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn12)), ((locals.var_fac1_dn17 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn17)),)
    } else {
        (locals.var_fsl_dpsl, locals.var_fsl_dpsl_dn0, locals.var_fsl_dpsl_dn2, locals.var_fsl_dpsl_dn6, locals.var_fsl_dpsl_dn7, locals.var_fsl_dpsl_dn10, locals.var_fsl_dpsl_dn11, locals.var_fsl_dpsl_dn12, locals.var_fsl_dpsl_dn17,)
    }
};
            locals.var_fsl_dpsl = assign16250_body47_e23679;
            locals.var_fsl_dpsl_dn0 = assign16250_body47_e23679_d_n0;
            locals.var_fsl_dpsl_dn2 = assign16250_body47_e23679_d_n2;
            locals.var_fsl_dpsl_dn6 = assign16250_body47_e23679_d_n6;
            locals.var_fsl_dpsl_dn7 = assign16250_body47_e23679_d_n7;
            locals.var_fsl_dpsl_dn10 = assign16250_body47_e23679_d_n10;
            locals.var_fsl_dpsl_dn11 = assign16250_body47_e23679_d_n11;
            locals.var_fsl_dpsl_dn12 = assign16250_body47_e23679_d_n12;
            locals.var_fsl_dpsl_dn17 = assign16250_body47_e23679_d_n17;
            locals.var_fsl_dpsl_rv = 0.0;
            let assign16250_body48_e23686: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_lp_sl > 3.0)) { 1.0 } else { 0.0 };
            locals.var_guard492 = assign16250_body48_e23686;
            locals.var_guard492_rv = 0.0;
            let (assign16250_body49_e23697,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign16250_body49_e23695: f64 = (locals.var_lp_sl_max + 1.0);
        (assign16250_body49_e23695,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign16250_body49_e23697;
            locals.var_lp_sl_rv = 0.0;
            let (assign16250_body50_e23710, assign16250_body50_e23710_d_n0, assign16250_body50_e23710_d_n2, assign16250_body50_e23710_d_n6, assign16250_body50_e23710_d_n7, assign16250_body50_e23710_d_n10, assign16250_body50_e23710_d_n11, assign16250_body50_e23710_d_n12, assign16250_body50_e23710_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard492 == 0.0)) {
        let assign16250_body50_e23706: f64 = (-locals.var_fsl);
        let assign16250_body50_e23708: f64 = (assign16250_body50_e23706 / locals.var_fsl_dpsl);
        (assign16250_body50_e23708, ((((-locals.var_fsl_dn0) * locals.var_fsl_dpsl) - (assign16250_body50_e23706 * locals.var_fsl_dpsl_dn0)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn2) * locals.var_fsl_dpsl) - (assign16250_body50_e23706 * locals.var_fsl_dpsl_dn2)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn6) * locals.var_fsl_dpsl) - (assign16250_body50_e23706 * locals.var_fsl_dpsl_dn6)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn7) * locals.var_fsl_dpsl) - (assign16250_body50_e23706 * locals.var_fsl_dpsl_dn7)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn10) * locals.var_fsl_dpsl) - (assign16250_body50_e23706 * locals.var_fsl_dpsl_dn10)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn11) * locals.var_fsl_dpsl) - (assign16250_body50_e23706 * locals.var_fsl_dpsl_dn11)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn12) * locals.var_fsl_dpsl) - (assign16250_body50_e23706 * locals.var_fsl_dpsl_dn12)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn17) * locals.var_fsl_dpsl) - (assign16250_body50_e23706 * locals.var_fsl_dpsl_dn17)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn12, locals.var_dpsl_dn17,)
    }
};
            locals.var_dpsl = assign16250_body50_e23710;
            locals.var_dpsl_dn0 = assign16250_body50_e23710_d_n0;
            locals.var_dpsl_dn2 = assign16250_body50_e23710_d_n2;
            locals.var_dpsl_dn6 = assign16250_body50_e23710_d_n6;
            locals.var_dpsl_dn7 = assign16250_body50_e23710_d_n7;
            locals.var_dpsl_dn10 = assign16250_body50_e23710_d_n10;
            locals.var_dpsl_dn11 = assign16250_body50_e23710_d_n11;
            locals.var_dpsl_dn12 = assign16250_body50_e23710_d_n12;
            locals.var_dpsl_dn17 = assign16250_body50_e23710_d_n17;
            locals.var_dpsl_rv = 0.0;
            let (assign16250_body51_e23733, assign16250_body51_e23733_d_n0, assign16250_body51_e23733_d_n2, assign16250_body51_e23733_d_n6, assign16250_body51_e23733_d_n7, assign16250_body51_e23733_d_n10, assign16250_body51_e23733_d_n11, assign16250_body51_e23733_d_n12, assign16250_body51_e23733_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard492 == 0.0)) {
        let assign16250_body51_e23720: f64 = (0.5 * 0.1);
        let assign16250_body51_e23724: f64 = (locals.var_phi_sl_soi).abs();
        let (assign16250_body51_e23729, assign16250_body51_e23729_d_n0, assign16250_body51_e23729_d_n2, assign16250_body51_e23729_d_n6, assign16250_body51_e23729_d_n7, assign16250_body51_e23729_d_n10, assign16250_body51_e23729_d_n11, assign16250_body51_e23729_d_n12, assign16250_body51_e23729_d_n17,) = {
            if (1.0 >= assign16250_body51_e23724) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign16250_body51_e23728: f64 = (locals.var_phi_sl_soi).abs();
                (assign16250_body51_e23728, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn0 } else { (-locals.var_phi_sl_soi_dn0) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn2 } else { (-locals.var_phi_sl_soi_dn2) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn6 } else { (-locals.var_phi_sl_soi_dn6) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn7 } else { (-locals.var_phi_sl_soi_dn7) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn10 } else { (-locals.var_phi_sl_soi_dn10) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn11 } else { (-locals.var_phi_sl_soi_dn11) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn12 } else { (-locals.var_phi_sl_soi_dn12) }, if locals.var_phi_sl_soi >= 0.0 { locals.var_phi_sl_soi_dn17 } else { (-locals.var_phi_sl_soi_dn17) },)
            }
        };
        let assign16250_body51_e23730: f64 = (1.0 + assign16250_body51_e23729);
        let assign16250_body51_e23731: f64 = (assign16250_body51_e23720 * assign16250_body51_e23730);
        (assign16250_body51_e23731, (assign16250_body51_e23720 * assign16250_body51_e23729_d_n0), (assign16250_body51_e23720 * assign16250_body51_e23729_d_n2), (assign16250_body51_e23720 * assign16250_body51_e23729_d_n6), (assign16250_body51_e23720 * assign16250_body51_e23729_d_n7), (assign16250_body51_e23720 * assign16250_body51_e23729_d_n10), (assign16250_body51_e23720 * assign16250_body51_e23729_d_n11), (assign16250_body51_e23720 * assign16250_body51_e23729_d_n12), (assign16250_body51_e23720 * assign16250_body51_e23729_d_n17),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, locals.var_dplim_dn17,)
    }
};
            locals.var_dplim = assign16250_body51_e23733;
            locals.var_dplim_dn0 = assign16250_body51_e23733_d_n0;
            locals.var_dplim_dn2 = assign16250_body51_e23733_d_n2;
            locals.var_dplim_dn6 = assign16250_body51_e23733_d_n6;
            locals.var_dplim_dn7 = assign16250_body51_e23733_d_n7;
            locals.var_dplim_dn10 = assign16250_body51_e23733_d_n10;
            locals.var_dplim_dn11 = assign16250_body51_e23733_d_n11;
            locals.var_dplim_dn12 = assign16250_body51_e23733_d_n12;
            locals.var_dplim_dn17 = assign16250_body51_e23733_d_n17;
            locals.var_dplim_rv = 0.0;
            let assign16250_body52_e23735: f64 = (locals.var_dpsl).abs();
            let assign16250_body52_e23737: f64 = if assign16250_body52_e23735 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard493 = assign16250_body52_e23737;
            locals.var_guard493_rv = 0.0;
            let (assign16250_body53_e23757, assign16250_body53_e23757_d_n0, assign16250_body53_e23757_d_n2, assign16250_body53_e23757_d_n6, assign16250_body53_e23757_d_n7, assign16250_body53_e23757_d_n10, assign16250_body53_e23757_d_n11, assign16250_body53_e23757_d_n12, assign16250_body53_e23757_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard492 == 0.0)) && (locals.var_guard493 != 0.0)) {
        let (assign16250_body53_e23754,) = {
            if (locals.var_dpsl >= 0.0) {
                (1.0,)
            } else {
                let assign16250_body53_e23753: f64 = (-1.0);
                (assign16250_body53_e23753,)
            }
        };
        let assign16250_body53_e23755: f64 = (locals.var_dplim * assign16250_body53_e23754);
        (assign16250_body53_e23755, (locals.var_dplim_dn0 * assign16250_body53_e23754), (locals.var_dplim_dn2 * assign16250_body53_e23754), (locals.var_dplim_dn6 * assign16250_body53_e23754), (locals.var_dplim_dn7 * assign16250_body53_e23754), (locals.var_dplim_dn10 * assign16250_body53_e23754), (locals.var_dplim_dn11 * assign16250_body53_e23754), (locals.var_dplim_dn12 * assign16250_body53_e23754), (locals.var_dplim_dn17 * assign16250_body53_e23754),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn12, locals.var_dpsl_dn17,)
    }
};
            locals.var_dpsl = assign16250_body53_e23757;
            locals.var_dpsl_dn0 = assign16250_body53_e23757_d_n0;
            locals.var_dpsl_dn2 = assign16250_body53_e23757_d_n2;
            locals.var_dpsl_dn6 = assign16250_body53_e23757_d_n6;
            locals.var_dpsl_dn7 = assign16250_body53_e23757_d_n7;
            locals.var_dpsl_dn10 = assign16250_body53_e23757_d_n10;
            locals.var_dpsl_dn11 = assign16250_body53_e23757_d_n11;
            locals.var_dpsl_dn12 = assign16250_body53_e23757_d_n12;
            locals.var_dpsl_dn17 = assign16250_body53_e23757_d_n17;
            locals.var_dpsl_rv = 0.0;
            let (assign16250_body54_e23769, assign16250_body54_e23769_d_n0, assign16250_body54_e23769_d_n2, assign16250_body54_e23769_d_n6, assign16250_body54_e23769_d_n7, assign16250_body54_e23769_d_n10, assign16250_body54_e23769_d_n11, assign16250_body54_e23769_d_n12, assign16250_body54_e23769_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard492 == 0.0)) {
        let assign16250_body54_e23767: f64 = (locals.var_phi_sl_soi + locals.var_dpsl);
        (assign16250_body54_e23767, (locals.var_phi_sl_soi_dn0 + locals.var_dpsl_dn0), (locals.var_phi_sl_soi_dn2 + locals.var_dpsl_dn2), (locals.var_phi_sl_soi_dn6 + locals.var_dpsl_dn6), (locals.var_phi_sl_soi_dn7 + locals.var_dpsl_dn7), (locals.var_phi_sl_soi_dn10 + locals.var_dpsl_dn10), (locals.var_phi_sl_soi_dn11 + locals.var_dpsl_dn11), (locals.var_phi_sl_soi_dn12 + locals.var_dpsl_dn12), (locals.var_phi_sl_soi_dn17 + locals.var_dpsl_dn17),)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
            locals.var_phi_sl_soi = assign16250_body54_e23769;
            locals.var_phi_sl_soi_dn0 = assign16250_body54_e23769_d_n0;
            locals.var_phi_sl_soi_dn2 = assign16250_body54_e23769_d_n2;
            locals.var_phi_sl_soi_dn6 = assign16250_body54_e23769_d_n6;
            locals.var_phi_sl_soi_dn7 = assign16250_body54_e23769_d_n7;
            locals.var_phi_sl_soi_dn10 = assign16250_body54_e23769_d_n10;
            locals.var_phi_sl_soi_dn11 = assign16250_body54_e23769_d_n11;
            locals.var_phi_sl_soi_dn12 = assign16250_body54_e23769_d_n12;
            locals.var_phi_sl_soi_dn17 = assign16250_body54_e23769_d_n17;
            locals.var_phi_sl_soi_rv = 0.0;
            let assign16250_body55_e23771: f64 = (locals.var_dpsl).abs();
            let assign16250_body55_e23775: f64 = (locals.var_fsl).abs();
            let assign16250_body55_e23778: f64 = if ((assign16250_body55_e23771 <= 5e-12) && (assign16250_body55_e23775 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard494 = assign16250_body55_e23778;
            locals.var_guard494_rv = 0.0;
            let (assign16250_body56_e23790,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard492 == 0.0)) && (locals.var_guard494 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign16250_body56_e23790;
            locals.var_flg_conv_rv = 0.0;
            let (assign16250_body57_e23799,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16250_body57_e23797: f64 = (locals.var_lp_sl + 1.0);
        (assign16250_body57_e23797,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign16250_body57_e23799;
            locals.var_lp_sl_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_58(
        locals: &mut StampLocals,
    ) {
        let (assign16260_e23808,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16260_e23806: f64 = (locals.var_lp_sl - 1.0);
        (assign16260_e23806,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign16260_e23808;
        locals.var_lp_sl_rv = 0.0;

        let (assign16270_e23815, assign16270_e23815_d_n0, assign16270_e23815_d_n2, assign16270_e23815_d_n6, assign16270_e23815_d_n7, assign16270_e23815_d_n10, assign16270_e23815_d_n11, assign16270_e23815_d_n12, assign16270_e23815_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (locals.var_q_dep_soi, locals.var_q_dep_soi_dn0, locals.var_q_dep_soi_dn2, locals.var_q_dep_soi_dn6, locals.var_q_dep_soi_dn7, locals.var_q_dep_soi_dn10, locals.var_q_dep_soi_dn11, locals.var_q_dep_soi_dn12, locals.var_q_dep_soi_dn17,)
    } else {
        (locals.var_q_depsl, locals.var_q_depsl_dn0, locals.var_q_depsl_dn2, locals.var_q_depsl_dn6, locals.var_q_depsl_dn7, locals.var_q_depsl_dn10, locals.var_q_depsl_dn11, locals.var_q_depsl_dn12, locals.var_q_depsl_dn17,)
    }
};
        locals.var_q_depsl = assign16270_e23815;
        locals.var_q_depsl_dn0 = assign16270_e23815_d_n0;
        locals.var_q_depsl_dn2 = assign16270_e23815_d_n2;
        locals.var_q_depsl_dn6 = assign16270_e23815_d_n6;
        locals.var_q_depsl_dn7 = assign16270_e23815_d_n7;
        locals.var_q_depsl_dn10 = assign16270_e23815_d_n10;
        locals.var_q_depsl_dn11 = assign16270_e23815_d_n11;
        locals.var_q_depsl_dn12 = assign16270_e23815_d_n12;
        locals.var_q_depsl_dn17 = assign16270_e23815_d_n17;
        locals.var_q_depsl_rv = 0.0;

        let (assign16280_e23822, assign16280_e23822_d_n0, assign16280_e23822_d_n2, assign16280_e23822_d_n6, assign16280_e23822_d_n7, assign16280_e23822_d_n10, assign16280_e23822_d_n11, assign16280_e23822_d_n12, assign16280_e23822_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (locals.var_q_depsl, locals.var_q_depsl_dn0, locals.var_q_depsl_dn2, locals.var_q_depsl_dn6, locals.var_q_depsl_dn7, locals.var_q_depsl_dn10, locals.var_q_depsl_dn11, locals.var_q_depsl_dn12, locals.var_q_depsl_dn17,)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
        locals.var_q_depl = assign16280_e23822;
        locals.var_q_depl_dn0 = assign16280_e23822_d_n0;
        locals.var_q_depl_dn2 = assign16280_e23822_d_n2;
        locals.var_q_depl_dn6 = assign16280_e23822_d_n6;
        locals.var_q_depl_dn7 = assign16280_e23822_d_n7;
        locals.var_q_depl_dn10 = assign16280_e23822_d_n10;
        locals.var_q_depl_dn11 = assign16280_e23822_d_n11;
        locals.var_q_depl_dn12 = assign16280_e23822_d_n12;
        locals.var_q_depl_dn17 = assign16280_e23822_d_n17;
        locals.var_q_depl_rv = 0.0;

        let (assign16290_e23829, assign16290_e23829_d_n0, assign16290_e23829_d_n2, assign16290_e23829_d_n6, assign16290_e23829_d_n7, assign16290_e23829_d_n10, assign16290_e23829_d_n11, assign16290_e23829_d_n12, assign16290_e23829_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign16290_e23829;
        locals.var_psl_dn0 = assign16290_e23829_d_n0;
        locals.var_psl_dn2 = assign16290_e23829_d_n2;
        locals.var_psl_dn6 = assign16290_e23829_d_n6;
        locals.var_psl_dn7 = assign16290_e23829_d_n7;
        locals.var_psl_dn10 = assign16290_e23829_d_n10;
        locals.var_psl_dn11 = assign16290_e23829_d_n11;
        locals.var_psl_dn12 = assign16290_e23829_d_n12;
        locals.var_psl_dn17 = assign16290_e23829_d_n17;
        locals.var_psl_rv = 0.0;

        let (assign16310_e23845, assign16310_e23845_d_n0, assign16310_e23845_d_n2, assign16310_e23845_d_n6, assign16310_e23845_d_n7, assign16310_e23845_d_n10, assign16310_e23845_d_n11, assign16310_e23845_d_n12, assign16310_e23845_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16310_e23843: f64 = (locals.var_q_depsl / locals.var_cnst0soi);
        (assign16310_e23843, (((locals.var_q_depsl_dn0 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn0)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn2 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn2)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn6 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn6)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn7 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn7)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn10 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn10)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn11 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn11)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn12 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn12)) / (locals.var_cnst0soi * locals.var_cnst0soi)), (((locals.var_q_depsl_dn17 * locals.var_cnst0soi) - (locals.var_q_depsl * locals.var_cnst0soi_dn17)) / (locals.var_cnst0soi * locals.var_cnst0soi)),)
    } else {
        (locals.var_q_depsl_soi_o_cnst0soi, locals.var_q_depsl_soi_o_cnst0soi_dn0, locals.var_q_depsl_soi_o_cnst0soi_dn2, locals.var_q_depsl_soi_o_cnst0soi_dn6, locals.var_q_depsl_soi_o_cnst0soi_dn7, locals.var_q_depsl_soi_o_cnst0soi_dn10, locals.var_q_depsl_soi_o_cnst0soi_dn11, locals.var_q_depsl_soi_o_cnst0soi_dn12, locals.var_q_depsl_soi_o_cnst0soi_dn17,)
    }
};
        locals.var_q_depsl_soi_o_cnst0soi = assign16310_e23845;
        locals.var_q_depsl_soi_o_cnst0soi_dn0 = assign16310_e23845_d_n0;
        locals.var_q_depsl_soi_o_cnst0soi_dn2 = assign16310_e23845_d_n2;
        locals.var_q_depsl_soi_o_cnst0soi_dn6 = assign16310_e23845_d_n6;
        locals.var_q_depsl_soi_o_cnst0soi_dn7 = assign16310_e23845_d_n7;
        locals.var_q_depsl_soi_o_cnst0soi_dn10 = assign16310_e23845_d_n10;
        locals.var_q_depsl_soi_o_cnst0soi_dn11 = assign16310_e23845_d_n11;
        locals.var_q_depsl_soi_o_cnst0soi_dn12 = assign16310_e23845_d_n12;
        locals.var_q_depsl_soi_o_cnst0soi_dn17 = assign16310_e23845_d_n17;
        locals.var_q_depsl_soi_o_cnst0soi_rv = 0.0;

        let (assign16320_e23856, assign16320_e23856_d_n0, assign16320_e23856_d_n2, assign16320_e23856_d_n6, assign16320_e23856_d_n7, assign16320_e23856_d_n10, assign16320_e23856_d_n11, assign16320_e23856_d_n12, assign16320_e23856_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16320_e23853: f64 = (10.0 * 2.220446049250313e-16);
        let assign16320_e23854: f64 = (locals.var_q_depsl_soi_o_cnst0soi + assign16320_e23853);
        (assign16320_e23854, locals.var_q_depsl_soi_o_cnst0soi_dn0, locals.var_q_depsl_soi_o_cnst0soi_dn2, locals.var_q_depsl_soi_o_cnst0soi_dn6, locals.var_q_depsl_soi_o_cnst0soi_dn7, locals.var_q_depsl_soi_o_cnst0soi_dn10, locals.var_q_depsl_soi_o_cnst0soi_dn11, locals.var_q_depsl_soi_o_cnst0soi_dn12, locals.var_q_depsl_soi_o_cnst0soi_dn17,)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn12, locals.var_xilp12_dn17,)
    }
};
        locals.var_xilp12 = assign16320_e23856;
        locals.var_xilp12_dn0 = assign16320_e23856_d_n0;
        locals.var_xilp12_dn2 = assign16320_e23856_d_n2;
        locals.var_xilp12_dn6 = assign16320_e23856_d_n6;
        locals.var_xilp12_dn7 = assign16320_e23856_d_n7;
        locals.var_xilp12_dn10 = assign16320_e23856_d_n10;
        locals.var_xilp12_dn11 = assign16320_e23856_d_n11;
        locals.var_xilp12_dn12 = assign16320_e23856_d_n12;
        locals.var_xilp12_dn17 = assign16320_e23856_d_n17;
        locals.var_xilp12_rv = 0.0;

        let (assign16330_e23867, assign16330_e23867_d_n0, assign16330_e23867_d_n2, assign16330_e23867_d_n6, assign16330_e23867_d_n7, assign16330_e23867_d_n10, assign16330_e23867_d_n11, assign16330_e23867_d_n12, assign16330_e23867_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16330_e23864: f64 = (locals.var_fsl2 + locals.var_xilp12);
        let assign16330_e23865: f64 = (1.0 / assign16330_e23864);
        (assign16330_e23865, (-((locals.var_fsl2_dn0 + locals.var_xilp12_dn0) / (assign16330_e23864 * assign16330_e23864))), (-((locals.var_fsl2_dn2 + locals.var_xilp12_dn2) / (assign16330_e23864 * assign16330_e23864))), (-((locals.var_fsl2_dn6 + locals.var_xilp12_dn6) / (assign16330_e23864 * assign16330_e23864))), (-((locals.var_fsl2_dn7 + locals.var_xilp12_dn7) / (assign16330_e23864 * assign16330_e23864))), (-((locals.var_fsl2_dn10 + locals.var_xilp12_dn10) / (assign16330_e23864 * assign16330_e23864))), (-((locals.var_fsl2_dn11 + locals.var_xilp12_dn11) / (assign16330_e23864 * assign16330_e23864))), (-((locals.var_fsl2_dn12 + locals.var_xilp12_dn12) / (assign16330_e23864 * assign16330_e23864))), (-((locals.var_fsl2_dn17 + locals.var_xilp12_dn17) / (assign16330_e23864 * assign16330_e23864))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16330_e23867;
        locals.var_t1_dn0 = assign16330_e23867_d_n0;
        locals.var_t1_dn2 = assign16330_e23867_d_n2;
        locals.var_t1_dn6 = assign16330_e23867_d_n6;
        locals.var_t1_dn7 = assign16330_e23867_d_n7;
        locals.var_t1_dn10 = assign16330_e23867_d_n10;
        locals.var_t1_dn11 = assign16330_e23867_d_n11;
        locals.var_t1_dn12 = assign16330_e23867_d_n12;
        locals.var_t1_dn17 = assign16330_e23867_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16340_e23878, assign16340_e23878_d_n0, assign16340_e23878_d_n2, assign16340_e23878_d_n6, assign16340_e23878_d_n7, assign16340_e23878_d_n10, assign16340_e23878_d_n11, assign16340_e23878_d_n12, assign16340_e23878_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16340_e23874: f64 = (locals.var_cnst0soi * locals.var_fsl1);
        let assign16340_e23876: f64 = (assign16340_e23874 * locals.var_t1);
        (assign16340_e23876, ((((locals.var_cnst0soi_dn0 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn0)) * locals.var_t1) + (assign16340_e23874 * locals.var_t1_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn2)) * locals.var_t1) + (assign16340_e23874 * locals.var_t1_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn6)) * locals.var_t1) + (assign16340_e23874 * locals.var_t1_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn7)) * locals.var_t1) + (assign16340_e23874 * locals.var_t1_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn10)) * locals.var_t1) + (assign16340_e23874 * locals.var_t1_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn11)) * locals.var_t1) + (assign16340_e23874 * locals.var_t1_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn12)) * locals.var_t1) + (assign16340_e23874 * locals.var_t1_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_fsl1) + (locals.var_cnst0soi * locals.var_fsl1_dn17)) * locals.var_t1) + (assign16340_e23874 * locals.var_t1_dn17)),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
        locals.var_q_nl = assign16340_e23878;
        locals.var_q_nl_dn0 = assign16340_e23878_d_n0;
        locals.var_q_nl_dn2 = assign16340_e23878_d_n2;
        locals.var_q_nl_dn6 = assign16340_e23878_d_n6;
        locals.var_q_nl_dn7 = assign16340_e23878_d_n7;
        locals.var_q_nl_dn10 = assign16340_e23878_d_n10;
        locals.var_q_nl_dn11 = assign16340_e23878_d_n11;
        locals.var_q_nl_dn12 = assign16340_e23878_d_n12;
        locals.var_q_nl_dn17 = assign16340_e23878_d_n17;
        locals.var_q_nl_rv = 0.0;

        let (assign16350_e23886, assign16350_e23886_d_n0, assign16350_e23886_d_n2, assign16350_e23886_d_n6, assign16350_e23886_d_n7, assign16350_e23886_d_n10, assign16350_e23886_d_n11, assign16350_e23886_d_n12, assign16350_e23886_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16350_e23884: f64 = (-locals.var_q_nl);
        (assign16350_e23884, (-locals.var_q_nl_dn0), (-locals.var_q_nl_dn2), (-locals.var_q_nl_dn6), (-locals.var_q_nl_dn7), (-locals.var_q_nl_dn10), (-locals.var_q_nl_dn11), (-locals.var_q_nl_dn12), (-locals.var_q_nl_dn17),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
        locals.var_q_nl = assign16350_e23886;
        locals.var_q_nl_dn0 = assign16350_e23886_d_n0;
        locals.var_q_nl_dn2 = assign16350_e23886_d_n2;
        locals.var_q_nl_dn6 = assign16350_e23886_d_n6;
        locals.var_q_nl_dn7 = assign16350_e23886_d_n7;
        locals.var_q_nl_dn10 = assign16350_e23886_d_n10;
        locals.var_q_nl_dn11 = assign16350_e23886_d_n11;
        locals.var_q_nl_dn12 = assign16350_e23886_d_n12;
        locals.var_q_nl_dn17 = assign16350_e23886_d_n17;
        locals.var_q_nl_rv = 0.0;

        let (assign16360_e23895, assign16360_e23895_d_n0, assign16360_e23895_d_n2, assign16360_e23895_d_n6, assign16360_e23895_d_n7, assign16360_e23895_d_n10, assign16360_e23895_d_n11, assign16360_e23895_d_n12, assign16360_e23895_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16360_e23893: f64 = (locals.var_psl - locals.var_ps0);
        (assign16360_e23893, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn12 - locals.var_ps0_dn12), (locals.var_psl_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign16360_e23895;
        locals.var_pds_dn0 = assign16360_e23895_d_n0;
        locals.var_pds_dn2 = assign16360_e23895_d_n2;
        locals.var_pds_dn6 = assign16360_e23895_d_n6;
        locals.var_pds_dn7 = assign16360_e23895_d_n7;
        locals.var_pds_dn10 = assign16360_e23895_d_n10;
        locals.var_pds_dn11 = assign16360_e23895_d_n11;
        locals.var_pds_dn12 = assign16360_e23895_d_n12;
        locals.var_pds_dn17 = assign16360_e23895_d_n17;
        locals.var_pds_rv = 0.0;

        let (assign16370_e23902, assign16370_e23902_d_n0, assign16370_e23902_d_n2, assign16370_e23902_d_n6, assign16370_e23902_d_n7, assign16370_e23902_d_n10, assign16370_e23902_d_n11, assign16370_e23902_d_n12, assign16370_e23902_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign16370_e23902;
        locals.var_vds_dn0 = assign16370_e23902_d_n0;
        locals.var_vds_dn2 = assign16370_e23902_d_n2;
        locals.var_vds_dn6 = assign16370_e23902_d_n6;
        locals.var_vds_dn7 = assign16370_e23902_d_n7;
        locals.var_vds_dn10 = assign16370_e23902_d_n10;
        locals.var_vds_dn11 = assign16370_e23902_d_n11;
        locals.var_vds_dn12 = assign16370_e23902_d_n12;
        locals.var_vds_dn17 = assign16370_e23902_d_n17;
        locals.var_vds_rv = 0.0;

        let (assign16380_e23911, assign16380_e23911_d_n0, assign16380_e23911_d_n2, assign16380_e23911_d_n6, assign16380_e23911_d_n7, assign16380_e23911_d_n10, assign16380_e23911_d_n11, assign16380_e23911_d_n12, assign16380_e23911_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16380_e23909: f64 = (locals.var_beta / locals.var_xi0);
        (assign16380_e23909, (-((locals.var_beta * locals.var_xi0_dn0) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn2) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn6) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn7) / (locals.var_xi0 * locals.var_xi0))), (((locals.var_beta_dn10 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn10)) / (locals.var_xi0 * locals.var_xi0)), (-((locals.var_beta * locals.var_xi0_dn11) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn12) / (locals.var_xi0 * locals.var_xi0))), (-((locals.var_beta * locals.var_xi0_dn17) / (locals.var_xi0 * locals.var_xi0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16380_e23911;
        locals.var_t1_dn0 = assign16380_e23911_d_n0;
        locals.var_t1_dn2 = assign16380_e23911_d_n2;
        locals.var_t1_dn6 = assign16380_e23911_d_n6;
        locals.var_t1_dn7 = assign16380_e23911_d_n7;
        locals.var_t1_dn10 = assign16380_e23911_d_n10;
        locals.var_t1_dn11 = assign16380_e23911_d_n11;
        locals.var_t1_dn12 = assign16380_e23911_d_n12;
        locals.var_t1_dn17 = assign16380_e23911_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16390_e23920, assign16390_e23920_d_n0, assign16390_e23920_d_n2, assign16390_e23920_d_n6, assign16390_e23920_d_n7, assign16390_e23920_d_n10, assign16390_e23920_d_n11, assign16390_e23920_d_n12, assign16390_e23920_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16390_e23918: f64 = (locals.var_t1 * locals.var_pds);
        (assign16390_e23918, ((locals.var_t1_dn0 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn0)), ((locals.var_t1_dn2 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn2)), ((locals.var_t1_dn6 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn6)), ((locals.var_t1_dn7 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn7)), ((locals.var_t1_dn10 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn10)), ((locals.var_t1_dn11 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn11)), ((locals.var_t1_dn12 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn12)), ((locals.var_t1_dn17 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn17)),)
    } else {
        (locals.var_eta, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12, locals.var_eta_dn17,)
    }
};
        locals.var_eta = assign16390_e23920;
        locals.var_eta_dn0 = assign16390_e23920_d_n0;
        locals.var_eta_dn2 = assign16390_e23920_d_n2;
        locals.var_eta_dn6 = assign16390_e23920_d_n6;
        locals.var_eta_dn7 = assign16390_e23920_d_n7;
        locals.var_eta_dn10 = assign16390_e23920_d_n10;
        locals.var_eta_dn11 = assign16390_e23920_d_n11;
        locals.var_eta_dn12 = assign16390_e23920_d_n12;
        locals.var_eta_dn17 = assign16390_e23920_d_n17;
        locals.var_eta_rv = 0.0;

        let (assign16400_e23929, assign16400_e23929_d_n0, assign16400_e23929_d_n2, assign16400_e23929_d_n6, assign16400_e23929_d_n7, assign16400_e23929_d_n10, assign16400_e23929_d_n11, assign16400_e23929_d_n12, assign16400_e23929_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16400_e23927: f64 = (locals.var_eta + 1.0);
        (assign16400_e23927, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12, locals.var_eta_dn17,)
    } else {
        (locals.var_eta1, locals.var_eta1_dn0, locals.var_eta1_dn2, locals.var_eta1_dn6, locals.var_eta1_dn7, locals.var_eta1_dn10, locals.var_eta1_dn11, locals.var_eta1_dn12, locals.var_eta1_dn17,)
    }
};
        locals.var_eta1 = assign16400_e23929;
        locals.var_eta1_dn0 = assign16400_e23929_d_n0;
        locals.var_eta1_dn2 = assign16400_e23929_d_n2;
        locals.var_eta1_dn6 = assign16400_e23929_d_n6;
        locals.var_eta1_dn7 = assign16400_e23929_d_n7;
        locals.var_eta1_dn10 = assign16400_e23929_d_n10;
        locals.var_eta1_dn11 = assign16400_e23929_d_n11;
        locals.var_eta1_dn12 = assign16400_e23929_d_n12;
        locals.var_eta1_dn17 = assign16400_e23929_d_n17;
        locals.var_eta1_rv = 0.0;

        let (assign16410_e23937, assign16410_e23937_d_n0, assign16410_e23937_d_n2, assign16410_e23937_d_n6, assign16410_e23937_d_n7, assign16410_e23937_d_n10, assign16410_e23937_d_n11, assign16410_e23937_d_n12, assign16410_e23937_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16410_e23935: f64 = (locals.var_eta1).sqrt();
        (assign16410_e23935, (locals.var_eta1_dn0 / (2.0 * assign16410_e23935)), (locals.var_eta1_dn2 / (2.0 * assign16410_e23935)), (locals.var_eta1_dn6 / (2.0 * assign16410_e23935)), (locals.var_eta1_dn7 / (2.0 * assign16410_e23935)), (locals.var_eta1_dn10 / (2.0 * assign16410_e23935)), (locals.var_eta1_dn11 / (2.0 * assign16410_e23935)), (locals.var_eta1_dn12 / (2.0 * assign16410_e23935)), (locals.var_eta1_dn17 / (2.0 * assign16410_e23935)),)
    } else {
        (locals.var_eta1p12, locals.var_eta1p12_dn0, locals.var_eta1p12_dn2, locals.var_eta1p12_dn6, locals.var_eta1p12_dn7, locals.var_eta1p12_dn10, locals.var_eta1p12_dn11, locals.var_eta1p12_dn12, locals.var_eta1p12_dn17,)
    }
};
        locals.var_eta1p12 = assign16410_e23937;
        locals.var_eta1p12_dn0 = assign16410_e23937_d_n0;
        locals.var_eta1p12_dn2 = assign16410_e23937_d_n2;
        locals.var_eta1p12_dn6 = assign16410_e23937_d_n6;
        locals.var_eta1p12_dn7 = assign16410_e23937_d_n7;
        locals.var_eta1p12_dn10 = assign16410_e23937_d_n10;
        locals.var_eta1p12_dn11 = assign16410_e23937_d_n11;
        locals.var_eta1p12_dn12 = assign16410_e23937_d_n12;
        locals.var_eta1p12_dn17 = assign16410_e23937_d_n17;
        locals.var_eta1p12_rv = 0.0;

        let (assign16420_e23948, assign16420_e23948_d_n0, assign16420_e23948_d_n2, assign16420_e23948_d_n6, assign16420_e23948_d_n7, assign16420_e23948_d_n10, assign16420_e23948_d_n11, assign16420_e23948_d_n12, assign16420_e23948_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16420_e23945: f64 = (locals.var_eta1p12 + 1.0);
        let assign16420_e23946: f64 = (1.0 / assign16420_e23945);
        (assign16420_e23946, (-(locals.var_eta1p12_dn0 / (assign16420_e23945 * assign16420_e23945))), (-(locals.var_eta1p12_dn2 / (assign16420_e23945 * assign16420_e23945))), (-(locals.var_eta1p12_dn6 / (assign16420_e23945 * assign16420_e23945))), (-(locals.var_eta1p12_dn7 / (assign16420_e23945 * assign16420_e23945))), (-(locals.var_eta1p12_dn10 / (assign16420_e23945 * assign16420_e23945))), (-(locals.var_eta1p12_dn11 / (assign16420_e23945 * assign16420_e23945))), (-(locals.var_eta1p12_dn12 / (assign16420_e23945 * assign16420_e23945))), (-(locals.var_eta1p12_dn17 / (assign16420_e23945 * assign16420_e23945))),)
    } else {
        (locals.var_zeta12, locals.var_zeta12_dn0, locals.var_zeta12_dn2, locals.var_zeta12_dn6, locals.var_zeta12_dn7, locals.var_zeta12_dn10, locals.var_zeta12_dn11, locals.var_zeta12_dn12, locals.var_zeta12_dn17,)
    }
};
        locals.var_zeta12 = assign16420_e23948;
        locals.var_zeta12_dn0 = assign16420_e23948_d_n0;
        locals.var_zeta12_dn2 = assign16420_e23948_d_n2;
        locals.var_zeta12_dn6 = assign16420_e23948_d_n6;
        locals.var_zeta12_dn7 = assign16420_e23948_d_n7;
        locals.var_zeta12_dn10 = assign16420_e23948_d_n10;
        locals.var_zeta12_dn11 = assign16420_e23948_d_n11;
        locals.var_zeta12_dn12 = assign16420_e23948_d_n12;
        locals.var_zeta12_dn17 = assign16420_e23948_d_n17;
        locals.var_zeta12_rv = 0.0;

        let (assign16430_e23957, assign16430_e23957_d_n0, assign16430_e23957_d_n2, assign16430_e23957_d_n6, assign16430_e23957_d_n7, assign16430_e23957_d_n10, assign16430_e23957_d_n11, assign16430_e23957_d_n12, assign16430_e23957_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16430_e23955: f64 = (locals.var_zeta12 / locals.var_xi0p12);
        (assign16430_e23955, (((locals.var_zeta12_dn0 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn0)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn2 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn2)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn6 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn6)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn7 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn7)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn10 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn10)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn11 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn11)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn12 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn12)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn17 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn17)) / (locals.var_xi0p12 * locals.var_xi0p12)),)
    } else {
        (locals.var_f00, locals.var_f00_dn0, locals.var_f00_dn2, locals.var_f00_dn6, locals.var_f00_dn7, locals.var_f00_dn10, locals.var_f00_dn11, locals.var_f00_dn12, locals.var_f00_dn17,)
    }
};
        locals.var_f00 = assign16430_e23957;
        locals.var_f00_dn0 = assign16430_e23957_d_n0;
        locals.var_f00_dn2 = assign16430_e23957_d_n2;
        locals.var_f00_dn6 = assign16430_e23957_d_n6;
        locals.var_f00_dn7 = assign16430_e23957_d_n7;
        locals.var_f00_dn10 = assign16430_e23957_d_n10;
        locals.var_f00_dn11 = assign16430_e23957_d_n11;
        locals.var_f00_dn12 = assign16430_e23957_d_n12;
        locals.var_f00_dn17 = assign16430_e23957_d_n17;
        locals.var_f00_rv = 0.0;

        let (assign16440_e23968, assign16440_e23968_d_n0, assign16440_e23968_d_n2, assign16440_e23968_d_n6, assign16440_e23968_d_n7, assign16440_e23968_d_n10, assign16440_e23968_d_n11, assign16440_e23968_d_n12, assign16440_e23968_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16440_e23965: f64 = (locals.var_q_deps0_soi_o_cnst0soi + locals.var_q_depsl_soi_o_cnst0soi);
        let assign16440_e23966: f64 = (0.5 * assign16440_e23965);
        (assign16440_e23966, (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn0 + locals.var_q_depsl_soi_o_cnst0soi_dn0)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn2 + locals.var_q_depsl_soi_o_cnst0soi_dn2)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn6 + locals.var_q_depsl_soi_o_cnst0soi_dn6)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn7 + locals.var_q_depsl_soi_o_cnst0soi_dn7)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn10 + locals.var_q_depsl_soi_o_cnst0soi_dn10)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn11 + locals.var_q_depsl_soi_o_cnst0soi_dn11)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn12 + locals.var_q_depsl_soi_o_cnst0soi_dn12)), (0.5 * (locals.var_q_deps0_soi_o_cnst0soi_dn17 + locals.var_q_depsl_soi_o_cnst0soi_dn17)),)
    } else {
        (locals.var_f10, locals.var_f10_dn0, locals.var_f10_dn2, locals.var_f10_dn6, locals.var_f10_dn7, locals.var_f10_dn10, locals.var_f10_dn11, locals.var_f10_dn12, locals.var_f10_dn17,)
    }
};
        locals.var_f10 = assign16440_e23968;
        locals.var_f10_dn0 = assign16440_e23968_d_n0;
        locals.var_f10_dn2 = assign16440_e23968_d_n2;
        locals.var_f10_dn6 = assign16440_e23968_d_n6;
        locals.var_f10_dn7 = assign16440_e23968_d_n7;
        locals.var_f10_dn10 = assign16440_e23968_d_n10;
        locals.var_f10_dn11 = assign16440_e23968_d_n11;
        locals.var_f10_dn12 = assign16440_e23968_d_n12;
        locals.var_f10_dn17 = assign16440_e23968_d_n17;
        locals.var_f10_rv = 0.0;

        let (assign16450_e23985, assign16450_e23985_d_n0, assign16450_e23985_d_n2, assign16450_e23985_d_n6, assign16450_e23985_d_n7, assign16450_e23985_d_n10, assign16450_e23985_d_n11, assign16450_e23985_d_n12, assign16450_e23985_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16450_e23975: f64 = (locals.var_vgp + locals.var_beta_inv);
        let assign16450_e23979: f64 = (2.0 * locals.var_ps0);
        let assign16450_e23981: f64 = (assign16450_e23979 + locals.var_pds);
        let assign16450_e23982: f64 = (0.5 * assign16450_e23981);
        let assign16450_e23983: f64 = (assign16450_e23975 - assign16450_e23982);
        (assign16450_e23983, (locals.var_vgp_dn0 - (0.5 * ((2.0 * locals.var_ps0_dn0) + locals.var_pds_dn0))), (locals.var_vgp_dn2 - (0.5 * ((2.0 * locals.var_ps0_dn2) + locals.var_pds_dn2))), (locals.var_vgp_dn6 - (0.5 * ((2.0 * locals.var_ps0_dn6) + locals.var_pds_dn6))), (locals.var_vgp_dn7 - (0.5 * ((2.0 * locals.var_ps0_dn7) + locals.var_pds_dn7))), ((locals.var_vgp_dn10 + locals.var_beta_inv_dn10) - (0.5 * ((2.0 * locals.var_ps0_dn10) + locals.var_pds_dn10))), (locals.var_vgp_dn11 - (0.5 * ((2.0 * locals.var_ps0_dn11) + locals.var_pds_dn11))), (locals.var_vgp_dn12 - (0.5 * ((2.0 * locals.var_ps0_dn12) + locals.var_pds_dn12))), (locals.var_vgp_dn17 - (0.5 * ((2.0 * locals.var_ps0_dn17) + locals.var_pds_dn17))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16450_e23985;
        locals.var_t1_dn0 = assign16450_e23985_d_n0;
        locals.var_t1_dn2 = assign16450_e23985_d_n2;
        locals.var_t1_dn6 = assign16450_e23985_d_n6;
        locals.var_t1_dn7 = assign16450_e23985_d_n7;
        locals.var_t1_dn10 = assign16450_e23985_d_n10;
        locals.var_t1_dn11 = assign16450_e23985_d_n11;
        locals.var_t1_dn12 = assign16450_e23985_d_n12;
        locals.var_t1_dn17 = assign16450_e23985_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16460_e23995, assign16460_e23995_d_n0, assign16460_e23995_d_n2, assign16460_e23995_d_n6, assign16460_e23995_d_n7, assign16460_e23995_d_n10, assign16460_e23995_d_n11, assign16460_e23995_d_n12, assign16460_e23995_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16460_e23991: f64 = (-locals.var_f10);
        let assign16460_e23993: f64 = (assign16460_e23991 + locals.var_f00);
        (assign16460_e23993, ((-locals.var_f10_dn0) + locals.var_f00_dn0), ((-locals.var_f10_dn2) + locals.var_f00_dn2), ((-locals.var_f10_dn6) + locals.var_f00_dn6), ((-locals.var_f10_dn7) + locals.var_f00_dn7), ((-locals.var_f10_dn10) + locals.var_f00_dn10), ((-locals.var_f10_dn11) + locals.var_f00_dn11), ((-locals.var_f10_dn12) + locals.var_f00_dn12), ((-locals.var_f10_dn17) + locals.var_f00_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign16460_e23995;
        locals.var_t2_dn0 = assign16460_e23995_d_n0;
        locals.var_t2_dn2 = assign16460_e23995_d_n2;
        locals.var_t2_dn6 = assign16460_e23995_d_n6;
        locals.var_t2_dn7 = assign16460_e23995_d_n7;
        locals.var_t2_dn10 = assign16460_e23995_d_n10;
        locals.var_t2_dn11 = assign16460_e23995_d_n11;
        locals.var_t2_dn12 = assign16460_e23995_d_n12;
        locals.var_t2_dn17 = assign16460_e23995_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign16470_e24004, assign16470_e24004_d_n0, assign16470_e24004_d_n2, assign16470_e24004_d_n6, assign16470_e24004_d_n7, assign16470_e24004_d_n10, assign16470_e24004_d_n11, assign16470_e24004_d_n12, assign16470_e24004_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16470_e24002: f64 = (locals.var_beta * locals.var_c_fox);
        (assign16470_e24002, (locals.var_beta * locals.var_c_fox_dn0), (locals.var_beta * locals.var_c_fox_dn2), (locals.var_beta * locals.var_c_fox_dn6), (locals.var_beta * locals.var_c_fox_dn7), ((locals.var_beta_dn10 * locals.var_c_fox) + (locals.var_beta * locals.var_c_fox_dn10)), (locals.var_beta * locals.var_c_fox_dn11), (locals.var_beta * locals.var_c_fox_dn12), (locals.var_beta * locals.var_c_fox_dn17),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign16470_e24004;
        locals.var_t3_dn0 = assign16470_e24004_d_n0;
        locals.var_t3_dn2 = assign16470_e24004_d_n2;
        locals.var_t3_dn6 = assign16470_e24004_d_n6;
        locals.var_t3_dn7 = assign16470_e24004_d_n7;
        locals.var_t3_dn10 = assign16470_e24004_d_n10;
        locals.var_t3_dn11 = assign16470_e24004_d_n11;
        locals.var_t3_dn12 = assign16470_e24004_d_n12;
        locals.var_t3_dn17 = assign16470_e24004_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign16480_e24013, assign16480_e24013_d_n0, assign16480_e24013_d_n2, assign16480_e24013_d_n6, assign16480_e24013_d_n7, assign16480_e24013_d_n10, assign16480_e24013_d_n11, assign16480_e24013_d_n12, assign16480_e24013_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16480_e24011: f64 = (locals.var_beta * locals.var_cnst0soi);
        (assign16480_e24011, (locals.var_beta * locals.var_cnst0soi_dn0), (locals.var_beta * locals.var_cnst0soi_dn2), (locals.var_beta * locals.var_cnst0soi_dn6), (locals.var_beta * locals.var_cnst0soi_dn7), ((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)), (locals.var_beta * locals.var_cnst0soi_dn11), (locals.var_beta * locals.var_cnst0soi_dn12), (locals.var_beta * locals.var_cnst0soi_dn17),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign16480_e24013;
        locals.var_t4_dn0 = assign16480_e24013_d_n0;
        locals.var_t4_dn2 = assign16480_e24013_d_n2;
        locals.var_t4_dn6 = assign16480_e24013_d_n6;
        locals.var_t4_dn7 = assign16480_e24013_d_n7;
        locals.var_t4_dn10 = assign16480_e24013_d_n10;
        locals.var_t4_dn11 = assign16480_e24013_d_n11;
        locals.var_t4_dn12 = assign16480_e24013_d_n12;
        locals.var_t4_dn17 = assign16480_e24013_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign16490_e24026, assign16490_e24026_d_n0, assign16490_e24026_d_n2, assign16490_e24026_d_n6, assign16490_e24026_d_n7, assign16490_e24026_d_n10, assign16490_e24026_d_n11, assign16490_e24026_d_n12, assign16490_e24026_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16490_e24020: f64 = (locals.var_t3 * locals.var_t1);
        let assign16490_e24023: f64 = (locals.var_t4 * locals.var_t2);
        let assign16490_e24024: f64 = (assign16490_e24020 + assign16490_e24023);
        (assign16490_e24024, (((locals.var_t3_dn0 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0))), (((locals.var_t3_dn2 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2))), (((locals.var_t3_dn6 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6))), (((locals.var_t3_dn7 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7))), (((locals.var_t3_dn10 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10))), (((locals.var_t3_dn11 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn11))), (((locals.var_t3_dn12 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn12)) + ((locals.var_t4_dn12 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn12))), (((locals.var_t3_dn17 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn17)) + ((locals.var_t4_dn17 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn17))),)
    } else {
        (locals.var_fdd, locals.var_fdd_dn0, locals.var_fdd_dn2, locals.var_fdd_dn6, locals.var_fdd_dn7, locals.var_fdd_dn10, locals.var_fdd_dn11, locals.var_fdd_dn12, locals.var_fdd_dn17,)
    }
};
        locals.var_fdd = assign16490_e24026;
        locals.var_fdd_dn0 = assign16490_e24026_d_n0;
        locals.var_fdd_dn2 = assign16490_e24026_d_n2;
        locals.var_fdd_dn6 = assign16490_e24026_d_n6;
        locals.var_fdd_dn7 = assign16490_e24026_d_n7;
        locals.var_fdd_dn10 = assign16490_e24026_d_n10;
        locals.var_fdd_dn11 = assign16490_e24026_d_n11;
        locals.var_fdd_dn12 = assign16490_e24026_d_n12;
        locals.var_fdd_dn17 = assign16490_e24026_d_n17;
        locals.var_fdd_rv = 0.0;

        let (assign16500_e24037, assign16500_e24037_d_n0, assign16500_e24037_d_n2, assign16500_e24037_d_n6, assign16500_e24037_d_n7, assign16500_e24037_d_n10, assign16500_e24037_d_n11, assign16500_e24037_d_n12, assign16500_e24037_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16500_e24033: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign16500_e24035: f64 = (assign16500_e24033 / 2.0);
        (assign16500_e24035, ((locals.var_q_depl_dn0 + locals.var_q_dep0_dn0) / 2.0), ((locals.var_q_depl_dn2 + locals.var_q_dep0_dn2) / 2.0), ((locals.var_q_depl_dn6 + locals.var_q_dep0_dn6) / 2.0), ((locals.var_q_depl_dn7 + locals.var_q_dep0_dn7) / 2.0), ((locals.var_q_depl_dn10 + locals.var_q_dep0_dn10) / 2.0), ((locals.var_q_depl_dn11 + locals.var_q_dep0_dn11) / 2.0), ((locals.var_q_depl_dn12 + locals.var_q_dep0_dn12) / 2.0), ((locals.var_q_depl_dn17 + locals.var_q_dep0_dn17) / 2.0),)
    } else {
        (locals.var_ab, locals.var_ab_dn0, locals.var_ab_dn2, locals.var_ab_dn6, locals.var_ab_dn7, locals.var_ab_dn10, locals.var_ab_dn11, locals.var_ab_dn12, locals.var_ab_dn17,)
    }
};
        locals.var_ab = assign16500_e24037;
        locals.var_ab_dn0 = assign16500_e24037_d_n0;
        locals.var_ab_dn2 = assign16500_e24037_d_n2;
        locals.var_ab_dn6 = assign16500_e24037_d_n6;
        locals.var_ab_dn7 = assign16500_e24037_d_n7;
        locals.var_ab_dn10 = assign16500_e24037_d_n10;
        locals.var_ab_dn11 = assign16500_e24037_d_n11;
        locals.var_ab_dn12 = assign16500_e24037_d_n12;
        locals.var_ab_dn17 = assign16500_e24037_d_n17;
        locals.var_ab_rv = 0.0;

        let (assign16510_e24049, assign16510_e24049_d_n0, assign16510_e24049_d_n2, assign16510_e24049_d_n6, assign16510_e24049_d_n7, assign16510_e24049_d_n10, assign16510_e24049_d_n11, assign16510_e24049_d_n12, assign16510_e24049_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16510_e24044: f64 = (locals.var_q_nl + locals.var_q_n0);
        let assign16510_e24045: f64 = (-assign16510_e24044);
        let assign16510_e24047: f64 = (assign16510_e24045 / 2.0);
        (assign16510_e24047, ((-(locals.var_q_nl_dn0 + locals.var_q_n0_dn0)) / 2.0), ((-(locals.var_q_nl_dn2 + locals.var_q_n0_dn2)) / 2.0), ((-(locals.var_q_nl_dn6 + locals.var_q_n0_dn6)) / 2.0), ((-(locals.var_q_nl_dn7 + locals.var_q_n0_dn7)) / 2.0), ((-(locals.var_q_nl_dn10 + locals.var_q_n0_dn10)) / 2.0), ((-(locals.var_q_nl_dn11 + locals.var_q_n0_dn11)) / 2.0), ((-(locals.var_q_nl_dn12 + locals.var_q_n0_dn12)) / 2.0), ((-(locals.var_q_nl_dn17 + locals.var_q_n0_dn17)) / 2.0),)
    } else {
        (locals.var_ai, locals.var_ai_dn0, locals.var_ai_dn2, locals.var_ai_dn6, locals.var_ai_dn7, locals.var_ai_dn10, locals.var_ai_dn11, locals.var_ai_dn12, locals.var_ai_dn17,)
    }
};
        locals.var_ai = assign16510_e24049;
        locals.var_ai_dn0 = assign16510_e24049_d_n0;
        locals.var_ai_dn2 = assign16510_e24049_d_n2;
        locals.var_ai_dn6 = assign16510_e24049_d_n6;
        locals.var_ai_dn7 = assign16510_e24049_d_n7;
        locals.var_ai_dn10 = assign16510_e24049_d_n10;
        locals.var_ai_dn11 = assign16510_e24049_d_n11;
        locals.var_ai_dn12 = assign16510_e24049_d_n12;
        locals.var_ai_dn17 = assign16510_e24049_d_n17;
        locals.var_ai_rv = 0.0;

        let (assign16520_e24058, assign16520_e24058_d_n0, assign16520_e24058_d_n2, assign16520_e24058_d_n6, assign16520_e24058_d_n7, assign16520_e24058_d_n10, assign16520_e24058_d_n11, assign16520_e24058_d_n12, assign16520_e24058_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16520_e24056: f64 = (locals.var_q_depl - locals.var_q_dep0);
        (assign16520_e24056, (locals.var_q_depl_dn0 - locals.var_q_dep0_dn0), (locals.var_q_depl_dn2 - locals.var_q_dep0_dn2), (locals.var_q_depl_dn6 - locals.var_q_dep0_dn6), (locals.var_q_depl_dn7 - locals.var_q_dep0_dn7), (locals.var_q_depl_dn10 - locals.var_q_dep0_dn10), (locals.var_q_depl_dn11 - locals.var_q_dep0_dn11), (locals.var_q_depl_dn12 - locals.var_q_dep0_dn12), (locals.var_q_depl_dn17 - locals.var_q_dep0_dn17),)
    } else {
        (locals.var_db, locals.var_db_dn0, locals.var_db_dn2, locals.var_db_dn6, locals.var_db_dn7, locals.var_db_dn10, locals.var_db_dn11, locals.var_db_dn12, locals.var_db_dn17,)
    }
};
        locals.var_db = assign16520_e24058;
        locals.var_db_dn0 = assign16520_e24058_d_n0;
        locals.var_db_dn2 = assign16520_e24058_d_n2;
        locals.var_db_dn6 = assign16520_e24058_d_n6;
        locals.var_db_dn7 = assign16520_e24058_d_n7;
        locals.var_db_dn10 = assign16520_e24058_d_n10;
        locals.var_db_dn11 = assign16520_e24058_d_n11;
        locals.var_db_dn12 = assign16520_e24058_d_n12;
        locals.var_db_dn17 = assign16520_e24058_d_n17;
        locals.var_db_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        let (assign16530_e24068, assign16530_e24068_d_n0, assign16530_e24068_d_n2, assign16530_e24068_d_n6, assign16530_e24068_d_n7, assign16530_e24068_d_n10, assign16530_e24068_d_n11, assign16530_e24068_d_n12, assign16530_e24068_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16530_e24065: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign16530_e24066: f64 = (-assign16530_e24065);
        (assign16530_e24066, (-(locals.var_q_nl_dn0 - locals.var_q_n0_dn0)), (-(locals.var_q_nl_dn2 - locals.var_q_n0_dn2)), (-(locals.var_q_nl_dn6 - locals.var_q_n0_dn6)), (-(locals.var_q_nl_dn7 - locals.var_q_n0_dn7)), (-(locals.var_q_nl_dn10 - locals.var_q_n0_dn10)), (-(locals.var_q_nl_dn11 - locals.var_q_n0_dn11)), (-(locals.var_q_nl_dn12 - locals.var_q_n0_dn12)), (-(locals.var_q_nl_dn17 - locals.var_q_n0_dn17)),)
    } else {
        (locals.var_di, locals.var_di_dn0, locals.var_di_dn2, locals.var_di_dn6, locals.var_di_dn7, locals.var_di_dn10, locals.var_di_dn11, locals.var_di_dn12, locals.var_di_dn17,)
    }
};
        locals.var_di = assign16530_e24068;
        locals.var_di_dn0 = assign16530_e24068_d_n0;
        locals.var_di_dn2 = assign16530_e24068_d_n2;
        locals.var_di_dn6 = assign16530_e24068_d_n6;
        locals.var_di_dn7 = assign16530_e24068_d_n7;
        locals.var_di_dn10 = assign16530_e24068_d_n10;
        locals.var_di_dn11 = assign16530_e24068_d_n11;
        locals.var_di_dn12 = assign16530_e24068_d_n12;
        locals.var_di_dn17 = assign16530_e24068_d_n17;
        locals.var_di_rv = 0.0;

        let (assign16540_e24077, assign16540_e24077_d_n0, assign16540_e24077_d_n2, assign16540_e24077_d_n6, assign16540_e24077_d_n7, assign16540_e24077_d_n10, assign16540_e24077_d_n11, assign16540_e24077_d_n12, assign16540_e24077_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16540_e24075: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        (assign16540_e24075, ((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)),)
    } else {
        (locals.var_c2, locals.var_c2_dn0, locals.var_c2_dn2, locals.var_c2_dn6, locals.var_c2_dn7, locals.var_c2_dn10, locals.var_c2_dn11, locals.var_c2_dn12, locals.var_c2_dn17,)
    }
};
        locals.var_c2 = assign16540_e24077;
        locals.var_c2_dn0 = assign16540_e24077_d_n0;
        locals.var_c2_dn2 = assign16540_e24077_d_n2;
        locals.var_c2_dn6 = assign16540_e24077_d_n6;
        locals.var_c2_dn7 = assign16540_e24077_d_n7;
        locals.var_c2_dn10 = assign16540_e24077_d_n10;
        locals.var_c2_dn11 = assign16540_e24077_d_n11;
        locals.var_c2_dn12 = assign16540_e24077_d_n12;
        locals.var_c2_dn17 = assign16540_e24077_d_n17;
        locals.var_c2_rv = 0.0;

        let assign16550_e24080: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard495 = assign16550_e24080;
        locals.var_guard495_rv = 0.0;

        let (assign16560_e24105, assign16560_e24105_d_n0, assign16560_e24105_d_n2, assign16560_e24105_d_n6, assign16560_e24105_d_n7, assign16560_e24105_d_n10, assign16560_e24105_d_n11, assign16560_e24105_d_n12, assign16560_e24105_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard495 != 0.0)) {
        let assign16560_e24089: f64 = (locals.var_ai * locals.var_beta);
        let assign16560_e24091: f64 = (assign16560_e24089 * locals.var_pds);
        let assign16560_e24093: f64 = (assign16560_e24091 - locals.var_di);
        let assign16560_e24096: f64 = (locals.var_db * locals.var_db);
        let assign16560_e24098: f64 = (assign16560_e24096 * locals.var_db);
        let assign16560_e24100: f64 = (assign16560_e24098 / locals.var_c2);
        let assign16560_e24102: f64 = (assign16560_e24100 / 6.0);
        let assign16560_e24103: f64 = (assign16560_e24093 - assign16560_e24102);
        (assign16560_e24103, (((((locals.var_ai_dn0 * locals.var_beta) * locals.var_pds) + (assign16560_e24089 * locals.var_pds_dn0)) - locals.var_di_dn0) - ((((((((locals.var_db_dn0 * locals.var_db) + (locals.var_db * locals.var_db_dn0)) * locals.var_db) + (assign16560_e24096 * locals.var_db_dn0)) * locals.var_c2) - (assign16560_e24098 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn2 * locals.var_beta) * locals.var_pds) + (assign16560_e24089 * locals.var_pds_dn2)) - locals.var_di_dn2) - ((((((((locals.var_db_dn2 * locals.var_db) + (locals.var_db * locals.var_db_dn2)) * locals.var_db) + (assign16560_e24096 * locals.var_db_dn2)) * locals.var_c2) - (assign16560_e24098 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn6 * locals.var_beta) * locals.var_pds) + (assign16560_e24089 * locals.var_pds_dn6)) - locals.var_di_dn6) - ((((((((locals.var_db_dn6 * locals.var_db) + (locals.var_db * locals.var_db_dn6)) * locals.var_db) + (assign16560_e24096 * locals.var_db_dn6)) * locals.var_c2) - (assign16560_e24098 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn7 * locals.var_beta) * locals.var_pds) + (assign16560_e24089 * locals.var_pds_dn7)) - locals.var_di_dn7) - ((((((((locals.var_db_dn7 * locals.var_db) + (locals.var_db * locals.var_db_dn7)) * locals.var_db) + (assign16560_e24096 * locals.var_db_dn7)) * locals.var_c2) - (assign16560_e24098 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((locals.var_ai_dn10 * locals.var_beta) + (locals.var_ai * locals.var_beta_dn10)) * locals.var_pds) + (assign16560_e24089 * locals.var_pds_dn10)) - locals.var_di_dn10) - ((((((((locals.var_db_dn10 * locals.var_db) + (locals.var_db * locals.var_db_dn10)) * locals.var_db) + (assign16560_e24096 * locals.var_db_dn10)) * locals.var_c2) - (assign16560_e24098 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn11 * locals.var_beta) * locals.var_pds) + (assign16560_e24089 * locals.var_pds_dn11)) - locals.var_di_dn11) - ((((((((locals.var_db_dn11 * locals.var_db) + (locals.var_db * locals.var_db_dn11)) * locals.var_db) + (assign16560_e24096 * locals.var_db_dn11)) * locals.var_c2) - (assign16560_e24098 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn12 * locals.var_beta) * locals.var_pds) + (assign16560_e24089 * locals.var_pds_dn12)) - locals.var_di_dn12) - ((((((((locals.var_db_dn12 * locals.var_db) + (locals.var_db * locals.var_db_dn12)) * locals.var_db) + (assign16560_e24096 * locals.var_db_dn12)) * locals.var_c2) - (assign16560_e24098 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((locals.var_ai_dn17 * locals.var_beta) * locals.var_pds) + (assign16560_e24089 * locals.var_pds_dn17)) - locals.var_di_dn17) - ((((((((locals.var_db_dn17 * locals.var_db) + (locals.var_db * locals.var_db_dn17)) * locals.var_db) + (assign16560_e24096 * locals.var_db_dn17)) * locals.var_c2) - (assign16560_e24098 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16560_e24105;
        locals.var_idd_dn0 = assign16560_e24105_d_n0;
        locals.var_idd_dn2 = assign16560_e24105_d_n2;
        locals.var_idd_dn6 = assign16560_e24105_d_n6;
        locals.var_idd_dn7 = assign16560_e24105_d_n7;
        locals.var_idd_dn10 = assign16560_e24105_d_n10;
        locals.var_idd_dn11 = assign16560_e24105_d_n11;
        locals.var_idd_dn12 = assign16560_e24105_d_n12;
        locals.var_idd_dn17 = assign16560_e24105_d_n17;
        locals.var_idd_rv = 0.0;

        let (assign16570_e24117, assign16570_e24117_d_n0, assign16570_e24117_d_n2, assign16570_e24117_d_n6, assign16570_e24117_d_n7, assign16570_e24117_d_n10, assign16570_e24117_d_n11, assign16570_e24117_d_n12, assign16570_e24117_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard495 == 0.0)) {
        let assign16570_e24115: f64 = (locals.var_pds * locals.var_fdd);
        (assign16570_e24115, ((locals.var_pds_dn0 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn0)), ((locals.var_pds_dn2 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn2)), ((locals.var_pds_dn6 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn6)), ((locals.var_pds_dn7 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn7)), ((locals.var_pds_dn10 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn10)), ((locals.var_pds_dn11 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn11)), ((locals.var_pds_dn12 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn12)), ((locals.var_pds_dn17 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn17)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16570_e24117;
        locals.var_idd_dn0 = assign16570_e24117_d_n0;
        locals.var_idd_dn2 = assign16570_e24117_d_n2;
        locals.var_idd_dn6 = assign16570_e24117_d_n6;
        locals.var_idd_dn7 = assign16570_e24117_d_n7;
        locals.var_idd_dn10 = assign16570_e24117_d_n10;
        locals.var_idd_dn11 = assign16570_e24117_d_n11;
        locals.var_idd_dn12 = assign16570_e24117_d_n12;
        locals.var_idd_dn17 = assign16570_e24117_d_n17;
        locals.var_idd_rv = 0.0;

        let assign16580_e24124: f64 = if ((locals.var_flg_info >= 1.0) && (locals.var_idd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard496 = assign16580_e24124;
        locals.var_guard496_rv = 0.0;

        let (assign16590_e24133, assign16590_e24133_d_n0, assign16590_e24133_d_n2, assign16590_e24133_d_n6, assign16590_e24133_d_n7, assign16590_e24133_d_n10, assign16590_e24133_d_n11, assign16590_e24133_d_n12, assign16590_e24133_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard496 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign16590_e24133;
        locals.var_idd_dn0 = assign16590_e24133_d_n0;
        locals.var_idd_dn2 = assign16590_e24133_d_n2;
        locals.var_idd_dn6 = assign16590_e24133_d_n6;
        locals.var_idd_dn7 = assign16590_e24133_d_n7;
        locals.var_idd_dn10 = assign16590_e24133_d_n10;
        locals.var_idd_dn11 = assign16590_e24133_d_n11;
        locals.var_idd_dn12 = assign16590_e24133_d_n12;
        locals.var_idd_dn17 = assign16590_e24133_d_n17;
        locals.var_idd_rv = 0.0;

        let assign16600_e24136: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard497 = assign16600_e24136;
        locals.var_guard497_rv = 0.0;

        let assign16610_e24138: f64 = (locals.var_pds).abs();
        let assign16610_e24140: f64 = if assign16610_e24138 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign16610_e24140;
        locals.var_guard498_rv = 0.0;

        let (assign16620_e24197, assign16620_e24197_d_n0, assign16620_e24197_d_n2, assign16620_e24197_d_n6, assign16620_e24197_d_n7, assign16620_e24197_d_n10, assign16620_e24197_d_n11, assign16620_e24197_d_n12, assign16620_e24197_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) {
        let assign16620_e24152: f64 = (locals.var_ai * locals.var_beta);
        let assign16620_e24154: f64 = (assign16620_e24152 * locals.var_pds);
        let assign16620_e24156: f64 = (assign16620_e24154 - locals.var_di);
        let assign16620_e24157: f64 = (locals.var_ab * assign16620_e24156);
        let assign16620_e24161: f64 = (2.0 * locals.var_ab);
        let assign16620_e24162: f64 = (locals.var_ai - assign16620_e24161);
        let assign16620_e24165: f64 = (locals.var_c_fox / locals.var_beta);
        let assign16620_e24169: f64 = (2.0 * locals.var_ab);
        let assign16620_e24171: f64 = (assign16620_e24169 * locals.var_ab);
        let assign16620_e24173: f64 = (assign16620_e24171 / locals.var_c2);
        let assign16620_e24174: f64 = (1.0 - assign16620_e24173);
        let assign16620_e24177: f64 = (locals.var_db * locals.var_db);
        let assign16620_e24179: f64 = (assign16620_e24177 / locals.var_c2);
        let assign16620_e24181: f64 = (assign16620_e24179 / 10.0);
        let assign16620_e24182: f64 = (assign16620_e24174 + assign16620_e24181);
        let assign16620_e24183: f64 = (assign16620_e24165 * assign16620_e24182);
        let assign16620_e24184: f64 = (assign16620_e24162 + assign16620_e24183);
        let assign16620_e24186: f64 = (assign16620_e24184 * locals.var_db);
        let assign16620_e24188: f64 = (assign16620_e24186 * locals.var_db);
        let assign16620_e24190: f64 = (assign16620_e24188 * locals.var_db);
        let assign16620_e24192: f64 = (assign16620_e24190 / locals.var_c2);
        let assign16620_e24194: f64 = (assign16620_e24192 / 6.0);
        let assign16620_e24195: f64 = (assign16620_e24157 + assign16620_e24194);
        (assign16620_e24195, (((locals.var_ab_dn0 * assign16620_e24156) + (locals.var_ab * ((((locals.var_ai_dn0 * locals.var_beta) * locals.var_pds) + (assign16620_e24152 * locals.var_pds_dn0)) - locals.var_di_dn0))) + ((((((((((((locals.var_ai_dn0 - (2.0 * locals.var_ab_dn0)) + (((locals.var_c_fox_dn0 / locals.var_beta) * assign16620_e24182) + (assign16620_e24165 * ((-((((((2.0 * locals.var_ab_dn0) * locals.var_ab) + (assign16620_e24169 * locals.var_ab_dn0)) * locals.var_c2) - (assign16620_e24171 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn0 * locals.var_db) + (locals.var_db * locals.var_db_dn0)) * locals.var_c2) - (assign16620_e24177 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16620_e24184 * locals.var_db_dn0)) * locals.var_db) + (assign16620_e24186 * locals.var_db_dn0)) * locals.var_db) + (assign16620_e24188 * locals.var_db_dn0)) * locals.var_c2) - (assign16620_e24190 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn2 * assign16620_e24156) + (locals.var_ab * ((((locals.var_ai_dn2 * locals.var_beta) * locals.var_pds) + (assign16620_e24152 * locals.var_pds_dn2)) - locals.var_di_dn2))) + ((((((((((((locals.var_ai_dn2 - (2.0 * locals.var_ab_dn2)) + (((locals.var_c_fox_dn2 / locals.var_beta) * assign16620_e24182) + (assign16620_e24165 * ((-((((((2.0 * locals.var_ab_dn2) * locals.var_ab) + (assign16620_e24169 * locals.var_ab_dn2)) * locals.var_c2) - (assign16620_e24171 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn2 * locals.var_db) + (locals.var_db * locals.var_db_dn2)) * locals.var_c2) - (assign16620_e24177 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16620_e24184 * locals.var_db_dn2)) * locals.var_db) + (assign16620_e24186 * locals.var_db_dn2)) * locals.var_db) + (assign16620_e24188 * locals.var_db_dn2)) * locals.var_c2) - (assign16620_e24190 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn6 * assign16620_e24156) + (locals.var_ab * ((((locals.var_ai_dn6 * locals.var_beta) * locals.var_pds) + (assign16620_e24152 * locals.var_pds_dn6)) - locals.var_di_dn6))) + ((((((((((((locals.var_ai_dn6 - (2.0 * locals.var_ab_dn6)) + (((locals.var_c_fox_dn6 / locals.var_beta) * assign16620_e24182) + (assign16620_e24165 * ((-((((((2.0 * locals.var_ab_dn6) * locals.var_ab) + (assign16620_e24169 * locals.var_ab_dn6)) * locals.var_c2) - (assign16620_e24171 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn6 * locals.var_db) + (locals.var_db * locals.var_db_dn6)) * locals.var_c2) - (assign16620_e24177 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16620_e24184 * locals.var_db_dn6)) * locals.var_db) + (assign16620_e24186 * locals.var_db_dn6)) * locals.var_db) + (assign16620_e24188 * locals.var_db_dn6)) * locals.var_c2) - (assign16620_e24190 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn7 * assign16620_e24156) + (locals.var_ab * ((((locals.var_ai_dn7 * locals.var_beta) * locals.var_pds) + (assign16620_e24152 * locals.var_pds_dn7)) - locals.var_di_dn7))) + ((((((((((((locals.var_ai_dn7 - (2.0 * locals.var_ab_dn7)) + (((locals.var_c_fox_dn7 / locals.var_beta) * assign16620_e24182) + (assign16620_e24165 * ((-((((((2.0 * locals.var_ab_dn7) * locals.var_ab) + (assign16620_e24169 * locals.var_ab_dn7)) * locals.var_c2) - (assign16620_e24171 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn7 * locals.var_db) + (locals.var_db * locals.var_db_dn7)) * locals.var_c2) - (assign16620_e24177 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16620_e24184 * locals.var_db_dn7)) * locals.var_db) + (assign16620_e24186 * locals.var_db_dn7)) * locals.var_db) + (assign16620_e24188 * locals.var_db_dn7)) * locals.var_c2) - (assign16620_e24190 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn10 * assign16620_e24156) + (locals.var_ab * (((((locals.var_ai_dn10 * locals.var_beta) + (locals.var_ai * locals.var_beta_dn10)) * locals.var_pds) + (assign16620_e24152 * locals.var_pds_dn10)) - locals.var_di_dn10))) + ((((((((((((locals.var_ai_dn10 - (2.0 * locals.var_ab_dn10)) + (((((locals.var_c_fox_dn10 * locals.var_beta) - (locals.var_c_fox * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) * assign16620_e24182) + (assign16620_e24165 * ((-((((((2.0 * locals.var_ab_dn10) * locals.var_ab) + (assign16620_e24169 * locals.var_ab_dn10)) * locals.var_c2) - (assign16620_e24171 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn10 * locals.var_db) + (locals.var_db * locals.var_db_dn10)) * locals.var_c2) - (assign16620_e24177 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16620_e24184 * locals.var_db_dn10)) * locals.var_db) + (assign16620_e24186 * locals.var_db_dn10)) * locals.var_db) + (assign16620_e24188 * locals.var_db_dn10)) * locals.var_c2) - (assign16620_e24190 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn11 * assign16620_e24156) + (locals.var_ab * ((((locals.var_ai_dn11 * locals.var_beta) * locals.var_pds) + (assign16620_e24152 * locals.var_pds_dn11)) - locals.var_di_dn11))) + ((((((((((((locals.var_ai_dn11 - (2.0 * locals.var_ab_dn11)) + (((locals.var_c_fox_dn11 / locals.var_beta) * assign16620_e24182) + (assign16620_e24165 * ((-((((((2.0 * locals.var_ab_dn11) * locals.var_ab) + (assign16620_e24169 * locals.var_ab_dn11)) * locals.var_c2) - (assign16620_e24171 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn11 * locals.var_db) + (locals.var_db * locals.var_db_dn11)) * locals.var_c2) - (assign16620_e24177 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16620_e24184 * locals.var_db_dn11)) * locals.var_db) + (assign16620_e24186 * locals.var_db_dn11)) * locals.var_db) + (assign16620_e24188 * locals.var_db_dn11)) * locals.var_c2) - (assign16620_e24190 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn12 * assign16620_e24156) + (locals.var_ab * ((((locals.var_ai_dn12 * locals.var_beta) * locals.var_pds) + (assign16620_e24152 * locals.var_pds_dn12)) - locals.var_di_dn12))) + ((((((((((((locals.var_ai_dn12 - (2.0 * locals.var_ab_dn12)) + (((locals.var_c_fox_dn12 / locals.var_beta) * assign16620_e24182) + (assign16620_e24165 * ((-((((((2.0 * locals.var_ab_dn12) * locals.var_ab) + (assign16620_e24169 * locals.var_ab_dn12)) * locals.var_c2) - (assign16620_e24171 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn12 * locals.var_db) + (locals.var_db * locals.var_db_dn12)) * locals.var_c2) - (assign16620_e24177 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16620_e24184 * locals.var_db_dn12)) * locals.var_db) + (assign16620_e24186 * locals.var_db_dn12)) * locals.var_db) + (assign16620_e24188 * locals.var_db_dn12)) * locals.var_c2) - (assign16620_e24190 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((locals.var_ab_dn17 * assign16620_e24156) + (locals.var_ab * ((((locals.var_ai_dn17 * locals.var_beta) * locals.var_pds) + (assign16620_e24152 * locals.var_pds_dn17)) - locals.var_di_dn17))) + ((((((((((((locals.var_ai_dn17 - (2.0 * locals.var_ab_dn17)) + (((locals.var_c_fox_dn17 / locals.var_beta) * assign16620_e24182) + (assign16620_e24165 * ((-((((((2.0 * locals.var_ab_dn17) * locals.var_ab) + (assign16620_e24169 * locals.var_ab_dn17)) * locals.var_c2) - (assign16620_e24171 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2))) + ((((((locals.var_db_dn17 * locals.var_db) + (locals.var_db * locals.var_db_dn17)) * locals.var_c2) - (assign16620_e24177 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 10.0))))) * locals.var_db) + (assign16620_e24184 * locals.var_db_dn17)) * locals.var_db) + (assign16620_e24186 * locals.var_db_dn17)) * locals.var_db) + (assign16620_e24188 * locals.var_db_dn17)) * locals.var_c2) - (assign16620_e24190 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16620_e24197;
        locals.var_qbu_dn0 = assign16620_e24197_d_n0;
        locals.var_qbu_dn2 = assign16620_e24197_d_n2;
        locals.var_qbu_dn6 = assign16620_e24197_d_n6;
        locals.var_qbu_dn7 = assign16620_e24197_d_n7;
        locals.var_qbu_dn10 = assign16620_e24197_d_n10;
        locals.var_qbu_dn11 = assign16620_e24197_d_n11;
        locals.var_qbu_dn12 = assign16620_e24197_d_n12;
        locals.var_qbu_dn17 = assign16620_e24197_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16630_e24210, assign16630_e24210_d_n0, assign16630_e24210_d_n2, assign16630_e24210_d_n6, assign16630_e24210_d_n7, assign16630_e24210_d_n10, assign16630_e24210_d_n11, assign16630_e24210_d_n12, assign16630_e24210_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 != 0.0)) {
        let assign16630_e24208: f64 = (locals.var_qbu / locals.var_idd);
        (assign16630_e24208, (((locals.var_qbu_dn0 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn0)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn2 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn2)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn6 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn6)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn7 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn7)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn10 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn10)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn11 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn11)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn12 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn12)) / (locals.var_idd * locals.var_idd)), (((locals.var_qbu_dn17 * locals.var_idd) - (locals.var_qbu * locals.var_idd_dn17)) / (locals.var_idd * locals.var_idd)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16630_e24210;
        locals.var_qbu_dn0 = assign16630_e24210_d_n0;
        locals.var_qbu_dn2 = assign16630_e24210_d_n2;
        locals.var_qbu_dn6 = assign16630_e24210_d_n6;
        locals.var_qbu_dn7 = assign16630_e24210_d_n7;
        locals.var_qbu_dn10 = assign16630_e24210_d_n10;
        locals.var_qbu_dn11 = assign16630_e24210_d_n11;
        locals.var_qbu_dn12 = assign16630_e24210_d_n12;
        locals.var_qbu_dn17 = assign16630_e24210_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16640_e24222, assign16640_e24222_d_n0, assign16640_e24222_d_n2, assign16640_e24222_d_n6, assign16640_e24222_d_n7, assign16640_e24222_d_n10, assign16640_e24222_d_n11, assign16640_e24222_d_n12, assign16640_e24222_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard497 != 0.0)) && (locals.var_guard498 == 0.0)) {
        (locals.var_ab, locals.var_ab_dn0, locals.var_ab_dn2, locals.var_ab_dn6, locals.var_ab_dn7, locals.var_ab_dn10, locals.var_ab_dn11, locals.var_ab_dn12, locals.var_ab_dn17,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16640_e24222;
        locals.var_qbu_dn0 = assign16640_e24222_d_n0;
        locals.var_qbu_dn2 = assign16640_e24222_d_n2;
        locals.var_qbu_dn6 = assign16640_e24222_d_n6;
        locals.var_qbu_dn7 = assign16640_e24222_d_n7;
        locals.var_qbu_dn10 = assign16640_e24222_d_n10;
        locals.var_qbu_dn11 = assign16640_e24222_d_n11;
        locals.var_qbu_dn12 = assign16640_e24222_d_n12;
        locals.var_qbu_dn17 = assign16640_e24222_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16650_e24236, assign16650_e24236_d_n0, assign16650_e24236_d_n2, assign16650_e24236_d_n6, assign16650_e24236_d_n7, assign16650_e24236_d_n10, assign16650_e24236_d_n11, assign16650_e24236_d_n12, assign16650_e24236_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard497 == 0.0)) {
        let assign16650_e24233: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign16650_e24234: f64 = (0.5 * assign16650_e24233);
        (assign16650_e24234, (0.5 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (0.5 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (0.5 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (0.5 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (0.5 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (0.5 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (0.5 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (0.5 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign16650_e24236;
        locals.var_qbu_dn0 = assign16650_e24236_d_n0;
        locals.var_qbu_dn2 = assign16650_e24236_d_n2;
        locals.var_qbu_dn6 = assign16650_e24236_d_n6;
        locals.var_qbu_dn7 = assign16650_e24236_d_n7;
        locals.var_qbu_dn10 = assign16650_e24236_d_n10;
        locals.var_qbu_dn11 = assign16650_e24236_d_n11;
        locals.var_qbu_dn12 = assign16650_e24236_d_n12;
        locals.var_qbu_dn17 = assign16650_e24236_d_n17;
        locals.var_qbu_rv = 0.0;

        let (assign16660_e24245, assign16660_e24245_d_n0, assign16660_e24245_d_n2, assign16660_e24245_d_n6, assign16660_e24245_d_n7, assign16660_e24245_d_n10, assign16660_e24245_d_n11, assign16660_e24245_d_n12, assign16660_e24245_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16660_e24243: f64 = (2.0 * locals.var_fac1);
        (assign16660_e24243, (2.0 * locals.var_fac1_dn0), (2.0 * locals.var_fac1_dn2), (2.0 * locals.var_fac1_dn6), (2.0 * locals.var_fac1_dn7), (2.0 * locals.var_fac1_dn10), (2.0 * locals.var_fac1_dn11), (2.0 * locals.var_fac1_dn12), (2.0 * locals.var_fac1_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16660_e24245;
        locals.var_t1_dn0 = assign16660_e24245_d_n0;
        locals.var_t1_dn2 = assign16660_e24245_d_n2;
        locals.var_t1_dn6 = assign16660_e24245_d_n6;
        locals.var_t1_dn7 = assign16660_e24245_d_n7;
        locals.var_t1_dn10 = assign16660_e24245_d_n10;
        locals.var_t1_dn11 = assign16660_e24245_d_n11;
        locals.var_t1_dn12 = assign16660_e24245_d_n12;
        locals.var_t1_dn17 = assign16660_e24245_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16670_e24256, assign16670_e24256_d_n0, assign16670_e24256_d_n2, assign16670_e24256_d_n6, assign16670_e24256_d_n7, assign16670_e24256_d_n10, assign16670_e24256_d_n11, assign16670_e24256_d_n12, assign16670_e24256_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16670_e24253: f64 = (locals.var_f10 - locals.var_xi0p12);
        let assign16670_e24254: f64 = (locals.var_t1 * assign16670_e24253);
        (assign16670_e24254, ((locals.var_t1_dn0 * assign16670_e24253) + (locals.var_t1 * (locals.var_f10_dn0 - locals.var_xi0p12_dn0))), ((locals.var_t1_dn2 * assign16670_e24253) + (locals.var_t1 * (locals.var_f10_dn2 - locals.var_xi0p12_dn2))), ((locals.var_t1_dn6 * assign16670_e24253) + (locals.var_t1 * (locals.var_f10_dn6 - locals.var_xi0p12_dn6))), ((locals.var_t1_dn7 * assign16670_e24253) + (locals.var_t1 * (locals.var_f10_dn7 - locals.var_xi0p12_dn7))), ((locals.var_t1_dn10 * assign16670_e24253) + (locals.var_t1 * (locals.var_f10_dn10 - locals.var_xi0p12_dn10))), ((locals.var_t1_dn11 * assign16670_e24253) + (locals.var_t1 * (locals.var_f10_dn11 - locals.var_xi0p12_dn11))), ((locals.var_t1_dn12 * assign16670_e24253) + (locals.var_t1 * (locals.var_f10_dn12 - locals.var_xi0p12_dn12))), ((locals.var_t1_dn17 * assign16670_e24253) + (locals.var_t1 * (locals.var_f10_dn17 - locals.var_xi0p12_dn17))),)
    } else {
        (locals.var_dtpds, locals.var_dtpds_dn0, locals.var_dtpds_dn2, locals.var_dtpds_dn6, locals.var_dtpds_dn7, locals.var_dtpds_dn10, locals.var_dtpds_dn11, locals.var_dtpds_dn12, locals.var_dtpds_dn17,)
    }
};
        locals.var_dtpds = assign16670_e24256;
        locals.var_dtpds_dn0 = assign16670_e24256_d_n0;
        locals.var_dtpds_dn2 = assign16670_e24256_d_n2;
        locals.var_dtpds_dn6 = assign16670_e24256_d_n6;
        locals.var_dtpds_dn7 = assign16670_e24256_d_n7;
        locals.var_dtpds_dn10 = assign16670_e24256_d_n10;
        locals.var_dtpds_dn11 = assign16670_e24256_d_n11;
        locals.var_dtpds_dn12 = assign16670_e24256_d_n12;
        locals.var_dtpds_dn17 = assign16670_e24256_d_n17;
        locals.var_dtpds_rv = 0.0;

        let (assign16680_e24265, assign16680_e24265_d_n0, assign16680_e24265_d_n2, assign16680_e24265_d_n6, assign16680_e24265_d_n7, assign16680_e24265_d_n10, assign16680_e24265_d_n11, assign16680_e24265_d_n12, assign16680_e24265_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16680_e24263: f64 = (locals.var_pds + locals.var_dtpds);
        (assign16680_e24263, (locals.var_pds_dn0 + locals.var_dtpds_dn0), (locals.var_pds_dn2 + locals.var_dtpds_dn2), (locals.var_pds_dn6 + locals.var_dtpds_dn6), (locals.var_pds_dn7 + locals.var_dtpds_dn7), (locals.var_pds_dn10 + locals.var_dtpds_dn10), (locals.var_pds_dn11 + locals.var_dtpds_dn11), (locals.var_pds_dn12 + locals.var_dtpds_dn12), (locals.var_pds_dn17 + locals.var_dtpds_dn17),)
    } else {
        (locals.var_achi, locals.var_achi_dn0, locals.var_achi_dn2, locals.var_achi_dn6, locals.var_achi_dn7, locals.var_achi_dn10, locals.var_achi_dn11, locals.var_achi_dn12, locals.var_achi_dn17,)
    }
};
        locals.var_achi = assign16680_e24265;
        locals.var_achi_dn0 = assign16680_e24265_d_n0;
        locals.var_achi_dn2 = assign16680_e24265_d_n2;
        locals.var_achi_dn6 = assign16680_e24265_d_n6;
        locals.var_achi_dn7 = assign16680_e24265_d_n7;
        locals.var_achi_dn10 = assign16680_e24265_d_n10;
        locals.var_achi_dn11 = assign16680_e24265_d_n11;
        locals.var_achi_dn12 = assign16680_e24265_d_n12;
        locals.var_achi_dn17 = assign16680_e24265_d_n17;
        locals.var_achi_rv = 0.0;

        let (assign16690_e24274, assign16690_e24274_d_n0, assign16690_e24274_d_n2, assign16690_e24274_d_n6, assign16690_e24274_d_n7, assign16690_e24274_d_n10, assign16690_e24274_d_n11, assign16690_e24274_d_n12, assign16690_e24274_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16690_e24272: f64 = (1.0 / locals.var_vgvt);
        (assign16690_e24272, (-(locals.var_vgvt_dn0 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn2 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn6 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn7 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn10 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn11 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn12 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn17 / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign16690_e24274;
        locals.var_t1_dn0 = assign16690_e24274_d_n0;
        locals.var_t1_dn2 = assign16690_e24274_d_n2;
        locals.var_t1_dn6 = assign16690_e24274_d_n6;
        locals.var_t1_dn7 = assign16690_e24274_d_n7;
        locals.var_t1_dn10 = assign16690_e24274_d_n10;
        locals.var_t1_dn11 = assign16690_e24274_d_n11;
        locals.var_t1_dn12 = assign16690_e24274_d_n12;
        locals.var_t1_dn17 = assign16690_e24274_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign16700_e24283, assign16700_e24283_d_n0, assign16700_e24283_d_n2, assign16700_e24283_d_n6, assign16700_e24283_d_n7, assign16700_e24283_d_n10, assign16700_e24283_d_n11, assign16700_e24283_d_n12, assign16700_e24283_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16700_e24281: f64 = (locals.var_achi * locals.var_t1);
        (assign16700_e24281, ((locals.var_achi_dn0 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn0)), ((locals.var_achi_dn2 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn2)), ((locals.var_achi_dn6 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn6)), ((locals.var_achi_dn7 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn7)), ((locals.var_achi_dn10 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn10)), ((locals.var_achi_dn11 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn11)), ((locals.var_achi_dn12 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn12)), ((locals.var_achi_dn17 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign16700_e24283;
        locals.var_t2_dn0 = assign16700_e24283_d_n0;
        locals.var_t2_dn2 = assign16700_e24283_d_n2;
        locals.var_t2_dn6 = assign16700_e24283_d_n6;
        locals.var_t2_dn7 = assign16700_e24283_d_n7;
        locals.var_t2_dn10 = assign16700_e24283_d_n10;
        locals.var_t2_dn11 = assign16700_e24283_d_n11;
        locals.var_t2_dn12 = assign16700_e24283_d_n12;
        locals.var_t2_dn17 = assign16700_e24283_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign16710_e24292, assign16710_e24292_d_n0, assign16710_e24292_d_n2, assign16710_e24292_d_n6, assign16710_e24292_d_n7, assign16710_e24292_d_n10, assign16710_e24292_d_n11, assign16710_e24292_d_n12, assign16710_e24292_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16710_e24290: f64 = (1.0 - locals.var_t2);
        (assign16710_e24290, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn12), (-locals.var_t2_dn17),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign16710_e24292;
        locals.var_t3_dn0 = assign16710_e24292_d_n0;
        locals.var_t3_dn2 = assign16710_e24292_d_n2;
        locals.var_t3_dn6 = assign16710_e24292_d_n6;
        locals.var_t3_dn7 = assign16710_e24292_d_n7;
        locals.var_t3_dn10 = assign16710_e24292_d_n10;
        locals.var_t3_dn11 = assign16710_e24292_d_n11;
        locals.var_t3_dn12 = assign16710_e24292_d_n12;
        locals.var_t3_dn17 = assign16710_e24292_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign16720_e24301, assign16720_e24301_d_n0, assign16720_e24301_d_n2, assign16720_e24301_d_n6, assign16720_e24301_d_n7, assign16720_e24301_d_n10, assign16720_e24301_d_n11, assign16720_e24301_d_n12, assign16720_e24301_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16720_e24299: f64 = (1.0 - locals.var_t3);
        (assign16720_e24299, (-locals.var_t3_dn0), (-locals.var_t3_dn2), (-locals.var_t3_dn6), (-locals.var_t3_dn7), (-locals.var_t3_dn10), (-locals.var_t3_dn11), (-locals.var_t3_dn12), (-locals.var_t3_dn17),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign16720_e24301;
        locals.var_tx_dn0 = assign16720_e24301_d_n0;
        locals.var_tx_dn2 = assign16720_e24301_d_n2;
        locals.var_tx_dn6 = assign16720_e24301_d_n6;
        locals.var_tx_dn7 = assign16720_e24301_d_n7;
        locals.var_tx_dn10 = assign16720_e24301_d_n10;
        locals.var_tx_dn11 = assign16720_e24301_d_n11;
        locals.var_tx_dn12 = assign16720_e24301_d_n12;
        locals.var_tx_dn17 = assign16720_e24301_d_n17;
        locals.var_tx_rv = 0.0;

        let (assign16730_e24310, assign16730_e24310_d_n0, assign16730_e24310_d_n2, assign16730_e24310_d_n6, assign16730_e24310_d_n7, assign16730_e24310_d_n10, assign16730_e24310_d_n11, assign16730_e24310_d_n12, assign16730_e24310_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16730_e24308: f64 = (locals.var_tx * locals.var_tx);
        (assign16730_e24308, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn12 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn12)), ((locals.var_tx_dn17 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign16730_e24310;
        locals.var_x2_dn0 = assign16730_e24310_d_n0;
        locals.var_x2_dn2 = assign16730_e24310_d_n2;
        locals.var_x2_dn6 = assign16730_e24310_d_n6;
        locals.var_x2_dn7 = assign16730_e24310_d_n7;
        locals.var_x2_dn10 = assign16730_e24310_d_n10;
        locals.var_x2_dn11 = assign16730_e24310_d_n11;
        locals.var_x2_dn12 = assign16730_e24310_d_n12;
        locals.var_x2_dn17 = assign16730_e24310_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign16740_e24319, assign16740_e24319_d_n0, assign16740_e24319_d_n2, assign16740_e24319_d_n6, assign16740_e24319_d_n7, assign16740_e24319_d_n10, assign16740_e24319_d_n11, assign16740_e24319_d_n12, assign16740_e24319_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16740_e24317: f64 = 1.0;
        (assign16740_e24317, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign16740_e24319;
        locals.var_xmax2_dn0 = assign16740_e24319_d_n0;
        locals.var_xmax2_dn2 = assign16740_e24319_d_n2;
        locals.var_xmax2_dn6 = assign16740_e24319_d_n6;
        locals.var_xmax2_dn7 = assign16740_e24319_d_n7;
        locals.var_xmax2_dn10 = assign16740_e24319_d_n10;
        locals.var_xmax2_dn11 = assign16740_e24319_d_n11;
        locals.var_xmax2_dn12 = assign16740_e24319_d_n12;
        locals.var_xmax2_dn17 = assign16740_e24319_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign16750_e24326, assign16750_e24326_d_n0, assign16750_e24326_d_n2, assign16750_e24326_d_n6, assign16750_e24326_d_n7, assign16750_e24326_d_n10, assign16750_e24326_d_n11, assign16750_e24326_d_n12, assign16750_e24326_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16750_e24326;
        locals.var_xp_dn0 = assign16750_e24326_d_n0;
        locals.var_xp_dn2 = assign16750_e24326_d_n2;
        locals.var_xp_dn6 = assign16750_e24326_d_n6;
        locals.var_xp_dn7 = assign16750_e24326_d_n7;
        locals.var_xp_dn10 = assign16750_e24326_d_n10;
        locals.var_xp_dn11 = assign16750_e24326_d_n11;
        locals.var_xp_dn12 = assign16750_e24326_d_n12;
        locals.var_xp_dn17 = assign16750_e24326_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16760_e24333, assign16760_e24333_d_n0, assign16760_e24333_d_n2, assign16760_e24333_d_n6, assign16760_e24333_d_n7, assign16760_e24333_d_n10, assign16760_e24333_d_n11, assign16760_e24333_d_n12, assign16760_e24333_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16760_e24333;
        locals.var_xmp_dn0 = assign16760_e24333_d_n0;
        locals.var_xmp_dn2 = assign16760_e24333_d_n2;
        locals.var_xmp_dn6 = assign16760_e24333_d_n6;
        locals.var_xmp_dn7 = assign16760_e24333_d_n7;
        locals.var_xmp_dn10 = assign16760_e24333_d_n10;
        locals.var_xmp_dn11 = assign16760_e24333_d_n11;
        locals.var_xmp_dn12 = assign16760_e24333_d_n12;
        locals.var_xmp_dn17 = assign16760_e24333_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16770_e24340,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign16770_e24340;
        locals.var_m0_rv = 0.0;

        let (assign16780_e24347,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16780_e24347;
        locals.var_mm_rv = 0.0;

        let (assign16790_e24354, assign16790_e24354_d_n0, assign16790_e24354_d_n2, assign16790_e24354_d_n6, assign16790_e24354_d_n7, assign16790_e24354_d_n10, assign16790_e24354_d_n11, assign16790_e24354_d_n12, assign16790_e24354_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign16790_e24354;
        locals.var_arg_dn0 = assign16790_e24354_d_n0;
        locals.var_arg_dn2 = assign16790_e24354_d_n2;
        locals.var_arg_dn6 = assign16790_e24354_d_n6;
        locals.var_arg_dn7 = assign16790_e24354_d_n7;
        locals.var_arg_dn10 = assign16790_e24354_d_n10;
        locals.var_arg_dn11 = assign16790_e24354_d_n11;
        locals.var_arg_dn12 = assign16790_e24354_d_n12;
        locals.var_arg_dn17 = assign16790_e24354_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign16800_e24361, assign16800_e24361_d_n0, assign16800_e24361_d_n2, assign16800_e24361_d_n6, assign16800_e24361_d_n7, assign16800_e24361_d_n10, assign16800_e24361_d_n11, assign16800_e24361_d_n12, assign16800_e24361_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16800_e24361;
        locals.var_dnm_dn0 = assign16800_e24361_d_n0;
        locals.var_dnm_dn2 = assign16800_e24361_d_n2;
        locals.var_dnm_dn6 = assign16800_e24361_d_n6;
        locals.var_dnm_dn7 = assign16800_e24361_d_n7;
        locals.var_dnm_dn10 = assign16800_e24361_d_n10;
        locals.var_dnm_dn11 = assign16800_e24361_d_n11;
        locals.var_dnm_dn12 = assign16800_e24361_d_n12;
        locals.var_dnm_dn17 = assign16800_e24361_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign16810_e24370, assign16810_e24370_d_n0, assign16810_e24370_d_n2, assign16810_e24370_d_n6, assign16810_e24370_d_n7, assign16810_e24370_d_n10, assign16810_e24370_d_n11, assign16810_e24370_d_n12, assign16810_e24370_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16810_e24368: f64 = (locals.var_xp * locals.var_x2);
        (assign16810_e24368, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16810_e24370;
        locals.var_xp_dn0 = assign16810_e24370_d_n0;
        locals.var_xp_dn2 = assign16810_e24370_d_n2;
        locals.var_xp_dn6 = assign16810_e24370_d_n6;
        locals.var_xp_dn7 = assign16810_e24370_d_n7;
        locals.var_xp_dn10 = assign16810_e24370_d_n10;
        locals.var_xp_dn11 = assign16810_e24370_d_n11;
        locals.var_xp_dn12 = assign16810_e24370_d_n12;
        locals.var_xp_dn17 = assign16810_e24370_d_n17;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        locals: &mut StampLocals,
    ) {
        let (assign16820_e24379, assign16820_e24379_d_n0, assign16820_e24379_d_n2, assign16820_e24379_d_n6, assign16820_e24379_d_n7, assign16820_e24379_d_n10, assign16820_e24379_d_n11, assign16820_e24379_d_n12, assign16820_e24379_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16820_e24377: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16820_e24377, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16820_e24379;
        locals.var_xmp_dn0 = assign16820_e24379_d_n0;
        locals.var_xmp_dn2 = assign16820_e24379_d_n2;
        locals.var_xmp_dn6 = assign16820_e24379_d_n6;
        locals.var_xmp_dn7 = assign16820_e24379_d_n7;
        locals.var_xmp_dn10 = assign16820_e24379_d_n10;
        locals.var_xmp_dn11 = assign16820_e24379_d_n11;
        locals.var_xmp_dn12 = assign16820_e24379_d_n12;
        locals.var_xmp_dn17 = assign16820_e24379_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16830_e24388, assign16830_e24388_d_n0, assign16830_e24388_d_n2, assign16830_e24388_d_n6, assign16830_e24388_d_n7, assign16830_e24388_d_n10, assign16830_e24388_d_n11, assign16830_e24388_d_n12, assign16830_e24388_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16830_e24386: f64 = (locals.var_xp * locals.var_x2);
        (assign16830_e24386, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16830_e24388;
        locals.var_xp_dn0 = assign16830_e24388_d_n0;
        locals.var_xp_dn2 = assign16830_e24388_d_n2;
        locals.var_xp_dn6 = assign16830_e24388_d_n6;
        locals.var_xp_dn7 = assign16830_e24388_d_n7;
        locals.var_xp_dn10 = assign16830_e24388_d_n10;
        locals.var_xp_dn11 = assign16830_e24388_d_n11;
        locals.var_xp_dn12 = assign16830_e24388_d_n12;
        locals.var_xp_dn17 = assign16830_e24388_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16840_e24397, assign16840_e24397_d_n0, assign16840_e24397_d_n2, assign16840_e24397_d_n6, assign16840_e24397_d_n7, assign16840_e24397_d_n10, assign16840_e24397_d_n11, assign16840_e24397_d_n12, assign16840_e24397_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16840_e24395: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16840_e24395, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16840_e24397;
        locals.var_xmp_dn0 = assign16840_e24397_d_n0;
        locals.var_xmp_dn2 = assign16840_e24397_d_n2;
        locals.var_xmp_dn6 = assign16840_e24397_d_n6;
        locals.var_xmp_dn7 = assign16840_e24397_d_n7;
        locals.var_xmp_dn10 = assign16840_e24397_d_n10;
        locals.var_xmp_dn11 = assign16840_e24397_d_n11;
        locals.var_xmp_dn12 = assign16840_e24397_d_n12;
        locals.var_xmp_dn17 = assign16840_e24397_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16850_e24406, assign16850_e24406_d_n0, assign16850_e24406_d_n2, assign16850_e24406_d_n6, assign16850_e24406_d_n7, assign16850_e24406_d_n10, assign16850_e24406_d_n11, assign16850_e24406_d_n12, assign16850_e24406_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16850_e24404: f64 = (locals.var_xp * locals.var_x2);
        (assign16850_e24404, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16850_e24406;
        locals.var_xp_dn0 = assign16850_e24406_d_n0;
        locals.var_xp_dn2 = assign16850_e24406_d_n2;
        locals.var_xp_dn6 = assign16850_e24406_d_n6;
        locals.var_xp_dn7 = assign16850_e24406_d_n7;
        locals.var_xp_dn10 = assign16850_e24406_d_n10;
        locals.var_xp_dn11 = assign16850_e24406_d_n11;
        locals.var_xp_dn12 = assign16850_e24406_d_n12;
        locals.var_xp_dn17 = assign16850_e24406_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16860_e24415, assign16860_e24415_d_n0, assign16860_e24415_d_n2, assign16860_e24415_d_n6, assign16860_e24415_d_n7, assign16860_e24415_d_n10, assign16860_e24415_d_n11, assign16860_e24415_d_n12, assign16860_e24415_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16860_e24413: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16860_e24413, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16860_e24415;
        locals.var_xmp_dn0 = assign16860_e24415_d_n0;
        locals.var_xmp_dn2 = assign16860_e24415_d_n2;
        locals.var_xmp_dn6 = assign16860_e24415_d_n6;
        locals.var_xmp_dn7 = assign16860_e24415_d_n7;
        locals.var_xmp_dn10 = assign16860_e24415_d_n10;
        locals.var_xmp_dn11 = assign16860_e24415_d_n11;
        locals.var_xmp_dn12 = assign16860_e24415_d_n12;
        locals.var_xmp_dn17 = assign16860_e24415_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16870_e24424, assign16870_e24424_d_n0, assign16870_e24424_d_n2, assign16870_e24424_d_n6, assign16870_e24424_d_n7, assign16870_e24424_d_n10, assign16870_e24424_d_n11, assign16870_e24424_d_n12, assign16870_e24424_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16870_e24422: f64 = (locals.var_xp * locals.var_x2);
        (assign16870_e24422, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign16870_e24424;
        locals.var_xp_dn0 = assign16870_e24424_d_n0;
        locals.var_xp_dn2 = assign16870_e24424_d_n2;
        locals.var_xp_dn6 = assign16870_e24424_d_n6;
        locals.var_xp_dn7 = assign16870_e24424_d_n7;
        locals.var_xp_dn10 = assign16870_e24424_d_n10;
        locals.var_xp_dn11 = assign16870_e24424_d_n11;
        locals.var_xp_dn12 = assign16870_e24424_d_n12;
        locals.var_xp_dn17 = assign16870_e24424_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign16880_e24433, assign16880_e24433_d_n0, assign16880_e24433_d_n2, assign16880_e24433_d_n6, assign16880_e24433_d_n7, assign16880_e24433_d_n10, assign16880_e24433_d_n11, assign16880_e24433_d_n12, assign16880_e24433_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16880_e24431: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign16880_e24431, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign16880_e24433;
        locals.var_xmp_dn0 = assign16880_e24433_d_n0;
        locals.var_xmp_dn2 = assign16880_e24433_d_n2;
        locals.var_xmp_dn6 = assign16880_e24433_d_n6;
        locals.var_xmp_dn7 = assign16880_e24433_d_n7;
        locals.var_xmp_dn10 = assign16880_e24433_d_n10;
        locals.var_xmp_dn11 = assign16880_e24433_d_n11;
        locals.var_xmp_dn12 = assign16880_e24433_d_n12;
        locals.var_xmp_dn17 = assign16880_e24433_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign16890_e24442, assign16890_e24442_d_n0, assign16890_e24442_d_n2, assign16890_e24442_d_n6, assign16890_e24442_d_n7, assign16890_e24442_d_n10, assign16890_e24442_d_n11, assign16890_e24442_d_n12, assign16890_e24442_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign16890_e24440: f64 = (locals.var_xp + locals.var_xmp);
        (assign16890_e24440, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign16890_e24442;
        locals.var_arg_dn0 = assign16890_e24442_d_n0;
        locals.var_arg_dn2 = assign16890_e24442_d_n2;
        locals.var_arg_dn6 = assign16890_e24442_d_n6;
        locals.var_arg_dn7 = assign16890_e24442_d_n7;
        locals.var_arg_dn10 = assign16890_e24442_d_n10;
        locals.var_arg_dn11 = assign16890_e24442_d_n11;
        locals.var_arg_dn12 = assign16890_e24442_d_n12;
        locals.var_arg_dn17 = assign16890_e24442_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign16900_e24449, assign16900_e24449_d_n0, assign16900_e24449_d_n2, assign16900_e24449_d_n6, assign16900_e24449_d_n7, assign16900_e24449_d_n10, assign16900_e24449_d_n11, assign16900_e24449_d_n12, assign16900_e24449_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign16900_e24449;
        locals.var_dnm_dn0 = assign16900_e24449_d_n0;
        locals.var_dnm_dn2 = assign16900_e24449_d_n2;
        locals.var_dnm_dn6 = assign16900_e24449_d_n6;
        locals.var_dnm_dn7 = assign16900_e24449_d_n7;
        locals.var_dnm_dn10 = assign16900_e24449_d_n10;
        locals.var_dnm_dn11 = assign16900_e24449_d_n11;
        locals.var_dnm_dn12 = assign16900_e24449_d_n12;
        locals.var_dnm_dn17 = assign16900_e24449_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign16910_e24464: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard499 = assign16910_e24464;
        locals.var_guard499_rv = 0.0;

        let assign16920_e24467: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign16920_e24467;
        locals.var_guard500_rv = 0.0;

        let (assign16930_e24478,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16930_e24478;
        locals.var_mm_rv = 0.0;

        let assign16940_e24481: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign16940_e24481;
        locals.var_guard501_rv = 0.0;

        let (assign16950_e24495,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 == 0.0)) && (locals.var_guard501 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16950_e24495;
        locals.var_mm_rv = 0.0;

        let assign16960_e24498: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard502 = assign16960_e24498;
        locals.var_guard502_rv = 0.0;

        let (assign16970_e24515,) = {
    if ((((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 == 0.0)) && (locals.var_guard501 == 0.0)) && (locals.var_guard502 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16970_e24515;
        locals.var_mm_rv = 0.0;

        let assign16980_e24518: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard503 = assign16980_e24518;
        locals.var_guard503_rv = 0.0;

        let (assign16990_e24538,) = {
    if (((((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_guard500 == 0.0)) && (locals.var_guard501 == 0.0)) && (locals.var_guard502 == 0.0)) && (locals.var_guard503 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign16990_e24538;
        locals.var_mm_rv = 0.0;

        let (assign17000_e24547,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign17000_e24547;
        locals.var_m0_rv = 0.0;

        let mut assign17010_loop_guard: usize = 0;
        while {
            let assign17010_cond_e24557: f64 = if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign17010_cond_e24557 != 0.0
        } {
            assign17010_loop_guard += 1;
            assert!(assign17010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign17010_body0_e24567, assign17010_body0_e24567_d_n0, assign17010_body0_e24567_d_n2, assign17010_body0_e24567_d_n6, assign17010_body0_e24567_d_n7, assign17010_body0_e24567_d_n10, assign17010_body0_e24567_d_n11, assign17010_body0_e24567_d_n12, assign17010_body0_e24567_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 != 0.0)) {
        let assign17010_body0_e24565: f64 = (locals.var_dnm).sqrt();
        (assign17010_body0_e24565, (locals.var_dnm_dn0 / (2.0 * assign17010_body0_e24565)), (locals.var_dnm_dn2 / (2.0 * assign17010_body0_e24565)), (locals.var_dnm_dn6 / (2.0 * assign17010_body0_e24565)), (locals.var_dnm_dn7 / (2.0 * assign17010_body0_e24565)), (locals.var_dnm_dn10 / (2.0 * assign17010_body0_e24565)), (locals.var_dnm_dn11 / (2.0 * assign17010_body0_e24565)), (locals.var_dnm_dn12 / (2.0 * assign17010_body0_e24565)), (locals.var_dnm_dn17 / (2.0 * assign17010_body0_e24565)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign17010_body0_e24567;
            locals.var_dnm_dn0 = assign17010_body0_e24567_d_n0;
            locals.var_dnm_dn2 = assign17010_body0_e24567_d_n2;
            locals.var_dnm_dn6 = assign17010_body0_e24567_d_n6;
            locals.var_dnm_dn7 = assign17010_body0_e24567_d_n7;
            locals.var_dnm_dn10 = assign17010_body0_e24567_d_n10;
            locals.var_dnm_dn11 = assign17010_body0_e24567_d_n11;
            locals.var_dnm_dn12 = assign17010_body0_e24567_d_n12;
            locals.var_dnm_dn17 = assign17010_body0_e24567_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign17010_body1_e24578,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 != 0.0)) {
        let assign17010_body1_e24576: f64 = (locals.var_m0 + 1.0);
        (assign17010_body1_e24576,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign17010_body1_e24578;
            locals.var_m0_rv = 0.0;
        }

        let (assign17020_e24594, assign17020_e24594_d_n0, assign17020_e24594_d_n2, assign17020_e24594_d_n6, assign17020_e24594_d_n7, assign17020_e24594_d_n10, assign17020_e24594_d_n11, assign17020_e24594_d_n12, assign17020_e24594_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard499 == 0.0)) {
        let assign17020_e24590: f64 = (2.0 * 4.0);
        let assign17020_e24591: f64 = (1.0 / assign17020_e24590);
        let assign17020_e24592: f64 = (locals.var_dnm).powf(assign17020_e24591);
        (assign17020_e24592, if 0.0 == 0.0 && ((assign17020_e24591) as f64).is_finite() && ((assign17020_e24591) as f64).fract() == 0.0 { if assign17020_e24591 == 0.0 { 0.0 } else { (assign17020_e24591 * ((locals.var_dnm).powf(assign17020_e24591 - 1.0) * locals.var_dnm_dn0)) } } else { (assign17020_e24592 * (assign17020_e24591 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17020_e24591) as f64).is_finite() && ((assign17020_e24591) as f64).fract() == 0.0 { if assign17020_e24591 == 0.0 { 0.0 } else { (assign17020_e24591 * ((locals.var_dnm).powf(assign17020_e24591 - 1.0) * locals.var_dnm_dn2)) } } else { (assign17020_e24592 * (assign17020_e24591 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17020_e24591) as f64).is_finite() && ((assign17020_e24591) as f64).fract() == 0.0 { if assign17020_e24591 == 0.0 { 0.0 } else { (assign17020_e24591 * ((locals.var_dnm).powf(assign17020_e24591 - 1.0) * locals.var_dnm_dn6)) } } else { (assign17020_e24592 * (assign17020_e24591 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17020_e24591) as f64).is_finite() && ((assign17020_e24591) as f64).fract() == 0.0 { if assign17020_e24591 == 0.0 { 0.0 } else { (assign17020_e24591 * ((locals.var_dnm).powf(assign17020_e24591 - 1.0) * locals.var_dnm_dn7)) } } else { (assign17020_e24592 * (assign17020_e24591 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17020_e24591) as f64).is_finite() && ((assign17020_e24591) as f64).fract() == 0.0 { if assign17020_e24591 == 0.0 { 0.0 } else { (assign17020_e24591 * ((locals.var_dnm).powf(assign17020_e24591 - 1.0) * locals.var_dnm_dn10)) } } else { (assign17020_e24592 * (assign17020_e24591 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17020_e24591) as f64).is_finite() && ((assign17020_e24591) as f64).fract() == 0.0 { if assign17020_e24591 == 0.0 { 0.0 } else { (assign17020_e24591 * ((locals.var_dnm).powf(assign17020_e24591 - 1.0) * locals.var_dnm_dn11)) } } else { (assign17020_e24592 * (assign17020_e24591 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17020_e24591) as f64).is_finite() && ((assign17020_e24591) as f64).fract() == 0.0 { if assign17020_e24591 == 0.0 { 0.0 } else { (assign17020_e24591 * ((locals.var_dnm).powf(assign17020_e24591 - 1.0) * locals.var_dnm_dn12)) } } else { (assign17020_e24592 * (assign17020_e24591 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign17020_e24591) as f64).is_finite() && ((assign17020_e24591) as f64).fract() == 0.0 { if assign17020_e24591 == 0.0 { 0.0 } else { (assign17020_e24591 * ((locals.var_dnm).powf(assign17020_e24591 - 1.0) * locals.var_dnm_dn17)) } } else { (assign17020_e24592 * (assign17020_e24591 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign17020_e24594;
        locals.var_dnm_dn0 = assign17020_e24594_d_n0;
        locals.var_dnm_dn2 = assign17020_e24594_d_n2;
        locals.var_dnm_dn6 = assign17020_e24594_d_n6;
        locals.var_dnm_dn7 = assign17020_e24594_d_n7;
        locals.var_dnm_dn10 = assign17020_e24594_d_n10;
        locals.var_dnm_dn11 = assign17020_e24594_d_n11;
        locals.var_dnm_dn12 = assign17020_e24594_d_n12;
        locals.var_dnm_dn17 = assign17020_e24594_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign17030_e24603, assign17030_e24603_d_n0, assign17030_e24603_d_n2, assign17030_e24603_d_n6, assign17030_e24603_d_n7, assign17030_e24603_d_n10, assign17030_e24603_d_n11, assign17030_e24603_d_n12, assign17030_e24603_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign17030_e24601: f64 = (1.0 / locals.var_dnm);
        (assign17030_e24601, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign17030_e24603;
        locals.var_dnm_dn0 = assign17030_e24603_d_n0;
        locals.var_dnm_dn2 = assign17030_e24603_d_n2;
        locals.var_dnm_dn6 = assign17030_e24603_d_n6;
        locals.var_dnm_dn7 = assign17030_e24603_d_n7;
        locals.var_dnm_dn10 = assign17030_e24603_d_n10;
        locals.var_dnm_dn11 = assign17030_e24603_d_n11;
        locals.var_dnm_dn12 = assign17030_e24603_d_n12;
        locals.var_dnm_dn17 = assign17030_e24603_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign17040_e24614, assign17040_e24614_d_n0, assign17040_e24614_d_n2, assign17040_e24614_d_n6, assign17040_e24614_d_n7, assign17040_e24614_d_n10, assign17040_e24614_d_n11, assign17040_e24614_d_n12, assign17040_e24614_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign17040_e24610: f64 = locals.var_tx;
        let assign17040_e24612: f64 = (assign17040_e24610 * locals.var_dnm);
        (assign17040_e24612, ((locals.var_tx_dn0 * locals.var_dnm) + (assign17040_e24610 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign17040_e24610 * locals.var_dnm_dn2)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign17040_e24610 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign17040_e24610 * locals.var_dnm_dn7)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign17040_e24610 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign17040_e24610 * locals.var_dnm_dn11)), ((locals.var_tx_dn12 * locals.var_dnm) + (assign17040_e24610 * locals.var_dnm_dn12)), ((locals.var_tx_dn17 * locals.var_dnm) + (assign17040_e24610 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign17040_e24614;
        locals.var_ty_dn0 = assign17040_e24614_d_n0;
        locals.var_ty_dn2 = assign17040_e24614_d_n2;
        locals.var_ty_dn6 = assign17040_e24614_d_n6;
        locals.var_ty_dn7 = assign17040_e24614_d_n7;
        locals.var_ty_dn10 = assign17040_e24614_d_n10;
        locals.var_ty_dn11 = assign17040_e24614_d_n11;
        locals.var_ty_dn12 = assign17040_e24614_d_n12;
        locals.var_ty_dn17 = assign17040_e24614_d_n17;
        locals.var_ty_rv = 0.0;

        let (assign17050_e24623, assign17050_e24623_d_n0, assign17050_e24623_d_n2, assign17050_e24623_d_n6, assign17050_e24623_d_n7, assign17050_e24623_d_n10, assign17050_e24623_d_n11, assign17050_e24623_d_n12, assign17050_e24623_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign17050_e24621: f64 = (1.0 - locals.var_ty);
        (assign17050_e24621, (-locals.var_ty_dn0), (-locals.var_ty_dn2), (-locals.var_ty_dn6), (-locals.var_ty_dn7), (-locals.var_ty_dn10), (-locals.var_ty_dn11), (-locals.var_ty_dn12), (-locals.var_ty_dn17),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign17050_e24623;
        locals.var_alpha_dn0 = assign17050_e24623_d_n0;
        locals.var_alpha_dn2 = assign17050_e24623_d_n2;
        locals.var_alpha_dn6 = assign17050_e24623_d_n6;
        locals.var_alpha_dn7 = assign17050_e24623_d_n7;
        locals.var_alpha_dn10 = assign17050_e24623_d_n10;
        locals.var_alpha_dn11 = assign17050_e24623_d_n11;
        locals.var_alpha_dn12 = assign17050_e24623_d_n12;
        locals.var_alpha_dn17 = assign17050_e24623_d_n17;
        locals.var_alpha_rv = 0.0;

        let (assign17060_e24636, assign17060_e24636_d_n0, assign17060_e24636_d_n2, assign17060_e24636_d_n6, assign17060_e24636_d_n7, assign17060_e24636_d_n10, assign17060_e24636_d_n11, assign17060_e24636_d_n12, assign17060_e24636_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign17060_e24632: f64 = (1.0 + locals.var_alpha);
        let assign17060_e24633: f64 = (locals.var_alpha * assign17060_e24632);
        let assign17060_e24634: f64 = (1.0 + assign17060_e24633);
        (assign17060_e24634, ((locals.var_alpha_dn0 * assign17060_e24632) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign17060_e24632) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign17060_e24632) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign17060_e24632) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign17060_e24632) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign17060_e24632) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign17060_e24632) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign17060_e24632) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign17060_e24636;
        locals.var_qinm_dn0 = assign17060_e24636_d_n0;
        locals.var_qinm_dn2 = assign17060_e24636_d_n2;
        locals.var_qinm_dn6 = assign17060_e24636_d_n6;
        locals.var_qinm_dn7 = assign17060_e24636_d_n7;
        locals.var_qinm_dn10 = assign17060_e24636_d_n10;
        locals.var_qinm_dn11 = assign17060_e24636_d_n11;
        locals.var_qinm_dn12 = assign17060_e24636_d_n12;
        locals.var_qinm_dn17 = assign17060_e24636_d_n17;
        locals.var_qinm_rv = 0.0;

        let (assign17070_e24656, assign17070_e24656_d_n0, assign17070_e24656_d_n2, assign17070_e24656_d_n6, assign17070_e24656_d_n7, assign17070_e24656_d_n10, assign17070_e24656_d_n11, assign17070_e24656_d_n12, assign17070_e24656_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign17070_e24643: f64 = (1.0 + locals.var_alpha);
        let assign17070_e24646: f64 = (10.0 * 2.220446049250313e-16);
        let (assign17070_e24654, assign17070_e24654_d_n0, assign17070_e24654_d_n2, assign17070_e24654_d_n6, assign17070_e24654_d_n7, assign17070_e24654_d_n10, assign17070_e24654_d_n11, assign17070_e24654_d_n12, assign17070_e24654_d_n17,) = {
            if (assign17070_e24643 >= assign17070_e24646) {
                let assign17070_e24650: f64 = (1.0 + locals.var_alpha);
                (assign17070_e24650, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign17070_e24653: f64 = (10.0 * 2.220446049250313e-16);
                (assign17070_e24653, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign17070_e24654, assign17070_e24654_d_n0, assign17070_e24654_d_n2, assign17070_e24654_d_n6, assign17070_e24654_d_n7, assign17070_e24654_d_n10, assign17070_e24654_d_n11, assign17070_e24654_d_n12, assign17070_e24654_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign17070_e24656;
        locals.var_qidn_dn0 = assign17070_e24656_d_n0;
        locals.var_qidn_dn2 = assign17070_e24656_d_n2;
        locals.var_qidn_dn6 = assign17070_e24656_d_n6;
        locals.var_qidn_dn7 = assign17070_e24656_d_n7;
        locals.var_qidn_dn10 = assign17070_e24656_d_n10;
        locals.var_qidn_dn11 = assign17070_e24656_d_n11;
        locals.var_qidn_dn12 = assign17070_e24656_d_n12;
        locals.var_qidn_dn17 = assign17070_e24656_d_n17;
        locals.var_qidn_rv = 0.0;

        let (assign17080_e24669, assign17080_e24669_d_n0, assign17080_e24669_d_n2, assign17080_e24669_d_n6, assign17080_e24669_d_n7, assign17080_e24669_d_n10, assign17080_e24669_d_n11, assign17080_e24669_d_n12, assign17080_e24669_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) {
        let assign17080_e24663: f64 = (0.6666666666666667 * locals.var_vgvt);
        let assign17080_e24665: f64 = (assign17080_e24663 * locals.var_qinm);
        let assign17080_e24667: f64 = (assign17080_e24665 / locals.var_qidn);
        (assign17080_e24667, ((((((0.6666666666666667 * locals.var_vgvt_dn0) * locals.var_qinm) + (assign17080_e24663 * locals.var_qinm_dn0)) * locals.var_qidn) - (assign17080_e24665 * locals.var_qidn_dn0)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn2) * locals.var_qinm) + (assign17080_e24663 * locals.var_qinm_dn2)) * locals.var_qidn) - (assign17080_e24665 * locals.var_qidn_dn2)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn6) * locals.var_qinm) + (assign17080_e24663 * locals.var_qinm_dn6)) * locals.var_qidn) - (assign17080_e24665 * locals.var_qidn_dn6)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn7) * locals.var_qinm) + (assign17080_e24663 * locals.var_qinm_dn7)) * locals.var_qidn) - (assign17080_e24665 * locals.var_qidn_dn7)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn10) * locals.var_qinm) + (assign17080_e24663 * locals.var_qinm_dn10)) * locals.var_qidn) - (assign17080_e24665 * locals.var_qidn_dn10)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn11) * locals.var_qinm) + (assign17080_e24663 * locals.var_qinm_dn11)) * locals.var_qidn) - (assign17080_e24665 * locals.var_qidn_dn11)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn12) * locals.var_qinm) + (assign17080_e24663 * locals.var_qinm_dn12)) * locals.var_qidn) - (assign17080_e24665 * locals.var_qidn_dn12)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn17) * locals.var_qinm) + (assign17080_e24663 * locals.var_qinm_dn17)) * locals.var_qidn) - (assign17080_e24665 * locals.var_qidn_dn17)) / (locals.var_qidn * locals.var_qidn)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign17080_e24669;
        locals.var_t1_dn0 = assign17080_e24669_d_n0;
        locals.var_t1_dn2 = assign17080_e24669_d_n2;
        locals.var_t1_dn6 = assign17080_e24669_d_n6;
        locals.var_t1_dn7 = assign17080_e24669_d_n7;
        locals.var_t1_dn10 = assign17080_e24669_d_n10;
        locals.var_t1_dn11 = assign17080_e24669_d_n11;
        locals.var_t1_dn12 = assign17080_e24669_d_n12;
        locals.var_t1_dn17 = assign17080_e24669_d_n17;
        locals.var_t1_rv = 0.0;

        let assign17090_e24672: f64 = if locals.var_flg_depmode <= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard504 = assign17090_e24672;
        locals.var_guard504_rv = 0.0;

        let assign17100_e24674: f64 = (locals.var_pds).abs();
        let assign17100_e24676: f64 = if assign17100_e24674 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard505 = assign17100_e24676;
        locals.var_guard505_rv = 0.0;

        let (assign17110_e24729, assign17110_e24729_d_n0, assign17110_e24729_d_n2, assign17110_e24729_d_n6, assign17110_e24729_d_n7, assign17110_e24729_d_n10, assign17110_e24729_d_n11, assign17110_e24729_d_n12, assign17110_e24729_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard504 != 0.0)) && (locals.var_guard505 != 0.0)) {
        let assign17110_e24687: f64 = (locals.var_ai * locals.var_ai);
        let assign17110_e24690: f64 = (locals.var_di * locals.var_di);
        let assign17110_e24692: f64 = (assign17110_e24690 / 12.0);
        let assign17110_e24693: f64 = (assign17110_e24687 + assign17110_e24692);
        let assign17110_e24695: f64 = (assign17110_e24693 * locals.var_beta);
        let assign17110_e24697: f64 = (assign17110_e24695 * locals.var_pds);
        let assign17110_e24700: f64 = (locals.var_ai * locals.var_di);
        let assign17110_e24701: f64 = (assign17110_e24697 - assign17110_e24700);
        let assign17110_e24704: f64 = (2.0 * locals.var_ai);
        let assign17110_e24707: f64 = (locals.var_c_fox / locals.var_beta);
        let assign17110_e24709: f64 = (assign17110_e24707 * locals.var_db);
        let assign17110_e24711: f64 = (assign17110_e24709 * locals.var_db);
        let assign17110_e24713: f64 = (assign17110_e24711 / locals.var_c2);
        let assign17110_e24715: f64 = (assign17110_e24713 / 5.0);
        let assign17110_e24716: f64 = (assign17110_e24704 + assign17110_e24715);
        let assign17110_e24718: f64 = (assign17110_e24716 * locals.var_db);
        let assign17110_e24720: f64 = (assign17110_e24718 * locals.var_db);
        let assign17110_e24722: f64 = (assign17110_e24720 * locals.var_db);
        let assign17110_e24724: f64 = (assign17110_e24722 / locals.var_c2);
        let assign17110_e24726: f64 = (assign17110_e24724 / 6.0);
        let assign17110_e24727: f64 = (assign17110_e24701 - assign17110_e24726);
        (assign17110_e24727, ((((((((locals.var_ai_dn0 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn0)) + (((locals.var_di_dn0 * locals.var_di) + (locals.var_di * locals.var_di_dn0)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17110_e24695 * locals.var_pds_dn0)) - ((locals.var_ai_dn0 * locals.var_di) + (locals.var_ai * locals.var_di_dn0))) - ((((((((((((2.0 * locals.var_ai_dn0) + (((((((((locals.var_c_fox_dn0 / locals.var_beta) * locals.var_db) + (assign17110_e24707 * locals.var_db_dn0)) * locals.var_db) + (assign17110_e24709 * locals.var_db_dn0)) * locals.var_c2) - (assign17110_e24711 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17110_e24716 * locals.var_db_dn0)) * locals.var_db) + (assign17110_e24718 * locals.var_db_dn0)) * locals.var_db) + (assign17110_e24720 * locals.var_db_dn0)) * locals.var_c2) - (assign17110_e24722 * locals.var_c2_dn0)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn2 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn2)) + (((locals.var_di_dn2 * locals.var_di) + (locals.var_di * locals.var_di_dn2)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17110_e24695 * locals.var_pds_dn2)) - ((locals.var_ai_dn2 * locals.var_di) + (locals.var_ai * locals.var_di_dn2))) - ((((((((((((2.0 * locals.var_ai_dn2) + (((((((((locals.var_c_fox_dn2 / locals.var_beta) * locals.var_db) + (assign17110_e24707 * locals.var_db_dn2)) * locals.var_db) + (assign17110_e24709 * locals.var_db_dn2)) * locals.var_c2) - (assign17110_e24711 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17110_e24716 * locals.var_db_dn2)) * locals.var_db) + (assign17110_e24718 * locals.var_db_dn2)) * locals.var_db) + (assign17110_e24720 * locals.var_db_dn2)) * locals.var_c2) - (assign17110_e24722 * locals.var_c2_dn2)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn6 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn6)) + (((locals.var_di_dn6 * locals.var_di) + (locals.var_di * locals.var_di_dn6)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17110_e24695 * locals.var_pds_dn6)) - ((locals.var_ai_dn6 * locals.var_di) + (locals.var_ai * locals.var_di_dn6))) - ((((((((((((2.0 * locals.var_ai_dn6) + (((((((((locals.var_c_fox_dn6 / locals.var_beta) * locals.var_db) + (assign17110_e24707 * locals.var_db_dn6)) * locals.var_db) + (assign17110_e24709 * locals.var_db_dn6)) * locals.var_c2) - (assign17110_e24711 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17110_e24716 * locals.var_db_dn6)) * locals.var_db) + (assign17110_e24718 * locals.var_db_dn6)) * locals.var_db) + (assign17110_e24720 * locals.var_db_dn6)) * locals.var_c2) - (assign17110_e24722 * locals.var_c2_dn6)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn7 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn7)) + (((locals.var_di_dn7 * locals.var_di) + (locals.var_di * locals.var_di_dn7)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17110_e24695 * locals.var_pds_dn7)) - ((locals.var_ai_dn7 * locals.var_di) + (locals.var_ai * locals.var_di_dn7))) - ((((((((((((2.0 * locals.var_ai_dn7) + (((((((((locals.var_c_fox_dn7 / locals.var_beta) * locals.var_db) + (assign17110_e24707 * locals.var_db_dn7)) * locals.var_db) + (assign17110_e24709 * locals.var_db_dn7)) * locals.var_c2) - (assign17110_e24711 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17110_e24716 * locals.var_db_dn7)) * locals.var_db) + (assign17110_e24718 * locals.var_db_dn7)) * locals.var_db) + (assign17110_e24720 * locals.var_db_dn7)) * locals.var_c2) - (assign17110_e24722 * locals.var_c2_dn7)) / (locals.var_c2 * locals.var_c2)) / 6.0)), (((((((((locals.var_ai_dn10 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn10)) + (((locals.var_di_dn10 * locals.var_di) + (locals.var_di * locals.var_di_dn10)) / 12.0)) * locals.var_beta) + (assign17110_e24693 * locals.var_beta_dn10)) * locals.var_pds) + (assign17110_e24695 * locals.var_pds_dn10)) - ((locals.var_ai_dn10 * locals.var_di) + (locals.var_ai * locals.var_di_dn10))) - ((((((((((((2.0 * locals.var_ai_dn10) + (((((((((((locals.var_c_fox_dn10 * locals.var_beta) - (locals.var_c_fox * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) * locals.var_db) + (assign17110_e24707 * locals.var_db_dn10)) * locals.var_db) + (assign17110_e24709 * locals.var_db_dn10)) * locals.var_c2) - (assign17110_e24711 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17110_e24716 * locals.var_db_dn10)) * locals.var_db) + (assign17110_e24718 * locals.var_db_dn10)) * locals.var_db) + (assign17110_e24720 * locals.var_db_dn10)) * locals.var_c2) - (assign17110_e24722 * locals.var_c2_dn10)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn11 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn11)) + (((locals.var_di_dn11 * locals.var_di) + (locals.var_di * locals.var_di_dn11)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17110_e24695 * locals.var_pds_dn11)) - ((locals.var_ai_dn11 * locals.var_di) + (locals.var_ai * locals.var_di_dn11))) - ((((((((((((2.0 * locals.var_ai_dn11) + (((((((((locals.var_c_fox_dn11 / locals.var_beta) * locals.var_db) + (assign17110_e24707 * locals.var_db_dn11)) * locals.var_db) + (assign17110_e24709 * locals.var_db_dn11)) * locals.var_c2) - (assign17110_e24711 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17110_e24716 * locals.var_db_dn11)) * locals.var_db) + (assign17110_e24718 * locals.var_db_dn11)) * locals.var_db) + (assign17110_e24720 * locals.var_db_dn11)) * locals.var_c2) - (assign17110_e24722 * locals.var_c2_dn11)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn12 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn12)) + (((locals.var_di_dn12 * locals.var_di) + (locals.var_di * locals.var_di_dn12)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17110_e24695 * locals.var_pds_dn12)) - ((locals.var_ai_dn12 * locals.var_di) + (locals.var_ai * locals.var_di_dn12))) - ((((((((((((2.0 * locals.var_ai_dn12) + (((((((((locals.var_c_fox_dn12 / locals.var_beta) * locals.var_db) + (assign17110_e24707 * locals.var_db_dn12)) * locals.var_db) + (assign17110_e24709 * locals.var_db_dn12)) * locals.var_c2) - (assign17110_e24711 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17110_e24716 * locals.var_db_dn12)) * locals.var_db) + (assign17110_e24718 * locals.var_db_dn12)) * locals.var_db) + (assign17110_e24720 * locals.var_db_dn12)) * locals.var_c2) - (assign17110_e24722 * locals.var_c2_dn12)) / (locals.var_c2 * locals.var_c2)) / 6.0)), ((((((((locals.var_ai_dn17 * locals.var_ai) + (locals.var_ai * locals.var_ai_dn17)) + (((locals.var_di_dn17 * locals.var_di) + (locals.var_di * locals.var_di_dn17)) / 12.0)) * locals.var_beta) * locals.var_pds) + (assign17110_e24695 * locals.var_pds_dn17)) - ((locals.var_ai_dn17 * locals.var_di) + (locals.var_ai * locals.var_di_dn17))) - ((((((((((((2.0 * locals.var_ai_dn17) + (((((((((locals.var_c_fox_dn17 / locals.var_beta) * locals.var_db) + (assign17110_e24707 * locals.var_db_dn17)) * locals.var_db) + (assign17110_e24709 * locals.var_db_dn17)) * locals.var_c2) - (assign17110_e24711 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 5.0)) * locals.var_db) + (assign17110_e24716 * locals.var_db_dn17)) * locals.var_db) + (assign17110_e24718 * locals.var_db_dn17)) * locals.var_db) + (assign17110_e24720 * locals.var_db_dn17)) * locals.var_c2) - (assign17110_e24722 * locals.var_c2_dn17)) / (locals.var_c2 * locals.var_c2)) / 6.0)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17110_e24729;
        locals.var_qiu_dn0 = assign17110_e24729_d_n0;
        locals.var_qiu_dn2 = assign17110_e24729_d_n2;
        locals.var_qiu_dn6 = assign17110_e24729_d_n6;
        locals.var_qiu_dn7 = assign17110_e24729_d_n7;
        locals.var_qiu_dn10 = assign17110_e24729_d_n10;
        locals.var_qiu_dn11 = assign17110_e24729_d_n11;
        locals.var_qiu_dn12 = assign17110_e24729_d_n12;
        locals.var_qiu_dn17 = assign17110_e24729_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign17120_e24742, assign17120_e24742_d_n0, assign17120_e24742_d_n2, assign17120_e24742_d_n6, assign17120_e24742_d_n7, assign17120_e24742_d_n10, assign17120_e24742_d_n11, assign17120_e24742_d_n12, assign17120_e24742_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard504 != 0.0)) && (locals.var_guard505 != 0.0)) {
        let assign17120_e24740: f64 = (locals.var_qiu / locals.var_idd);
        (assign17120_e24740, (((locals.var_qiu_dn0 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn0)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn2 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn2)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn6 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn6)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn7 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn7)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn10 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn10)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn11 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn11)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn12 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn12)) / (locals.var_idd * locals.var_idd)), (((locals.var_qiu_dn17 * locals.var_idd) - (locals.var_qiu * locals.var_idd_dn17)) / (locals.var_idd * locals.var_idd)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17120_e24742;
        locals.var_qiu_dn0 = assign17120_e24742_d_n0;
        locals.var_qiu_dn2 = assign17120_e24742_d_n2;
        locals.var_qiu_dn6 = assign17120_e24742_d_n6;
        locals.var_qiu_dn7 = assign17120_e24742_d_n7;
        locals.var_qiu_dn10 = assign17120_e24742_d_n10;
        locals.var_qiu_dn11 = assign17120_e24742_d_n11;
        locals.var_qiu_dn12 = assign17120_e24742_d_n12;
        locals.var_qiu_dn17 = assign17120_e24742_d_n17;
        locals.var_qiu_rv = 0.0;

        let (assign17130_e24754, assign17130_e24754_d_n0, assign17130_e24754_d_n2, assign17130_e24754_d_n6, assign17130_e24754_d_n7, assign17130_e24754_d_n10, assign17130_e24754_d_n11, assign17130_e24754_d_n12, assign17130_e24754_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard504 != 0.0)) && (locals.var_guard505 == 0.0)) {
        (locals.var_ai, locals.var_ai_dn0, locals.var_ai_dn2, locals.var_ai_dn6, locals.var_ai_dn7, locals.var_ai_dn10, locals.var_ai_dn11, locals.var_ai_dn12, locals.var_ai_dn17,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17130_e24754;
        locals.var_qiu_dn0 = assign17130_e24754_d_n0;
        locals.var_qiu_dn2 = assign17130_e24754_d_n2;
        locals.var_qiu_dn6 = assign17130_e24754_d_n6;
        locals.var_qiu_dn7 = assign17130_e24754_d_n7;
        locals.var_qiu_dn10 = assign17130_e24754_d_n10;
        locals.var_qiu_dn11 = assign17130_e24754_d_n11;
        locals.var_qiu_dn12 = assign17130_e24754_d_n12;
        locals.var_qiu_dn17 = assign17130_e24754_d_n17;
        locals.var_qiu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17140_e24769, assign17140_e24769_d_n0, assign17140_e24769_d_n2, assign17140_e24769_d_n6, assign17140_e24769_d_n7, assign17140_e24769_d_n10, assign17140_e24769_d_n11, assign17140_e24769_d_n12, assign17140_e24769_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard465 != 0.0)) && (locals.var_guard504 == 0.0)) {
        let assign17140_e24763: f64 = (-0.5);
        let assign17140_e24766: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign17140_e24767: f64 = (assign17140_e24763 * assign17140_e24766);
        (assign17140_e24767, (assign17140_e24763 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign17140_e24763 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign17140_e24763 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign17140_e24763 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign17140_e24763 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign17140_e24763 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign17140_e24763 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign17140_e24763 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign17140_e24769;
        locals.var_qiu_dn0 = assign17140_e24769_d_n0;
        locals.var_qiu_dn2 = assign17140_e24769_d_n2;
        locals.var_qiu_dn6 = assign17140_e24769_d_n6;
        locals.var_qiu_dn7 = assign17140_e24769_d_n7;
        locals.var_qiu_dn10 = assign17140_e24769_d_n10;
        locals.var_qiu_dn11 = assign17140_e24769_d_n11;
        locals.var_qiu_dn12 = assign17140_e24769_d_n12;
        locals.var_qiu_dn17 = assign17140_e24769_d_n17;
        locals.var_qiu_rv = 0.0;

        let assign17180_e24783: f64 = if locals.var_end_of_part_1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard509 = assign17180_e24783;
        locals.var_guard509_rv = 0.0;

        let (assign17190_e24789, assign17190_e24789_d_n0, assign17190_e24789_d_n2, assign17190_e24789_d_n6, assign17190_e24789_d_n7, assign17190_e24789_d_n10, assign17190_e24789_d_n11, assign17190_e24789_d_n12, assign17190_e24789_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17190_e24787: f64 = (0.5 + locals.var_alpha);
        (assign17190_e24787, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    } else {
        (locals.var_qdnm, locals.var_qdnm_dn0, locals.var_qdnm_dn2, locals.var_qdnm_dn6, locals.var_qdnm_dn7, locals.var_qdnm_dn10, locals.var_qdnm_dn11, locals.var_qdnm_dn12, locals.var_qdnm_dn17,)
    }
};
        locals.var_qdnm = assign17190_e24789;
        locals.var_qdnm_dn0 = assign17190_e24789_d_n0;
        locals.var_qdnm_dn2 = assign17190_e24789_d_n2;
        locals.var_qdnm_dn6 = assign17190_e24789_d_n6;
        locals.var_qdnm_dn7 = assign17190_e24789_d_n7;
        locals.var_qdnm_dn10 = assign17190_e24789_d_n10;
        locals.var_qdnm_dn11 = assign17190_e24789_d_n11;
        locals.var_qdnm_dn12 = assign17190_e24789_d_n12;
        locals.var_qdnm_dn17 = assign17190_e24789_d_n17;
        locals.var_qdnm_rv = 0.0;

        let (assign17200_e24795, assign17200_e24795_d_n0, assign17200_e24795_d_n2, assign17200_e24795_d_n6, assign17200_e24795_d_n7, assign17200_e24795_d_n10, assign17200_e24795_d_n11, assign17200_e24795_d_n12, assign17200_e24795_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17200_e24793: f64 = (locals.var_qidn * locals.var_qinm);
        (assign17200_e24793, ((locals.var_qidn_dn0 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn0)), ((locals.var_qidn_dn2 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn2)), ((locals.var_qidn_dn6 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn6)), ((locals.var_qidn_dn7 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn7)), ((locals.var_qidn_dn10 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn10)), ((locals.var_qidn_dn11 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn11)), ((locals.var_qidn_dn12 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn12)), ((locals.var_qidn_dn17 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn17)),)
    } else {
        (locals.var_qddn, locals.var_qddn_dn0, locals.var_qddn_dn2, locals.var_qddn_dn6, locals.var_qddn_dn7, locals.var_qddn_dn10, locals.var_qddn_dn11, locals.var_qddn_dn12, locals.var_qddn_dn17,)
    }
};
        locals.var_qddn = assign17200_e24795;
        locals.var_qddn_dn0 = assign17200_e24795_d_n0;
        locals.var_qddn_dn2 = assign17200_e24795_d_n2;
        locals.var_qddn_dn6 = assign17200_e24795_d_n6;
        locals.var_qddn_dn7 = assign17200_e24795_d_n7;
        locals.var_qddn_dn10 = assign17200_e24795_d_n10;
        locals.var_qddn_dn11 = assign17200_e24795_d_n11;
        locals.var_qddn_dn12 = assign17200_e24795_d_n12;
        locals.var_qddn_dn17 = assign17200_e24795_d_n17;
        locals.var_qddn_rv = 0.0;

        let (assign17210_e24803, assign17210_e24803_d_n0, assign17210_e24803_d_n2, assign17210_e24803_d_n6, assign17210_e24803_d_n7, assign17210_e24803_d_n10, assign17210_e24803_d_n11, assign17210_e24803_d_n12, assign17210_e24803_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17210_e24799: f64 = (0.4 * locals.var_qdnm);
        let assign17210_e24801: f64 = (assign17210_e24799 / locals.var_qddn);
        (assign17210_e24801, ((((0.4 * locals.var_qdnm_dn0) * locals.var_qddn) - (assign17210_e24799 * locals.var_qddn_dn0)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn2) * locals.var_qddn) - (assign17210_e24799 * locals.var_qddn_dn2)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn6) * locals.var_qddn) - (assign17210_e24799 * locals.var_qddn_dn6)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn7) * locals.var_qddn) - (assign17210_e24799 * locals.var_qddn_dn7)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn10) * locals.var_qddn) - (assign17210_e24799 * locals.var_qddn_dn10)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn11) * locals.var_qddn) - (assign17210_e24799 * locals.var_qddn_dn11)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn12) * locals.var_qddn) - (assign17210_e24799 * locals.var_qddn_dn12)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn17) * locals.var_qddn) - (assign17210_e24799 * locals.var_qddn_dn17)) / (locals.var_qddn * locals.var_qddn)),)
    } else {
        (locals.var_quot, locals.var_quot_dn0, locals.var_quot_dn2, locals.var_quot_dn6, locals.var_quot_dn7, locals.var_quot_dn10, locals.var_quot_dn11, locals.var_quot_dn12, locals.var_quot_dn17,)
    }
};
        locals.var_quot = assign17210_e24803;
        locals.var_quot_dn0 = assign17210_e24803_d_n0;
        locals.var_quot_dn2 = assign17210_e24803_d_n2;
        locals.var_quot_dn6 = assign17210_e24803_d_n6;
        locals.var_quot_dn7 = assign17210_e24803_d_n7;
        locals.var_quot_dn10 = assign17210_e24803_d_n10;
        locals.var_quot_dn11 = assign17210_e24803_d_n11;
        locals.var_quot_dn12 = assign17210_e24803_d_n12;
        locals.var_quot_dn17 = assign17210_e24803_d_n17;
        locals.var_quot_rv = 0.0;

        let (assign17220_e24809, assign17220_e24809_d_n0, assign17220_e24809_d_n2, assign17220_e24809_d_n6, assign17220_e24809_d_n7, assign17220_e24809_d_n10, assign17220_e24809_d_n11, assign17220_e24809_d_n12, assign17220_e24809_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17220_e24807: f64 = (0.6 - locals.var_quot);
        (assign17220_e24807, (-locals.var_quot_dn0), (-locals.var_quot_dn2), (-locals.var_quot_dn6), (-locals.var_quot_dn7), (-locals.var_quot_dn10), (-locals.var_quot_dn11), (-locals.var_quot_dn12), (-locals.var_quot_dn17),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17220_e24809;
        locals.var_qdrat_dn0 = assign17220_e24809_d_n0;
        locals.var_qdrat_dn2 = assign17220_e24809_d_n2;
        locals.var_qdrat_dn6 = assign17220_e24809_d_n6;
        locals.var_qdrat_dn7 = assign17220_e24809_d_n7;
        locals.var_qdrat_dn10 = assign17220_e24809_d_n10;
        locals.var_qdrat_dn11 = assign17220_e24809_d_n11;
        locals.var_qdrat_dn12 = assign17220_e24809_d_n12;
        locals.var_qdrat_dn17 = assign17220_e24809_d_n17;
        locals.var_qdrat_rv = 0.0;

        let assign17230_e24813: f64 = (0.5 + 1e-8);
        let assign17230_e24814: f64 = if locals.var_qdrat > assign17230_e24813 { 1.0 } else { 0.0 };
        locals.var_guard510 = assign17230_e24814;
        locals.var_guard510_rv = 0.0;

        let (assign17250_e24823, assign17250_e24823_d_n0, assign17250_e24823_d_n2, assign17250_e24823_d_n6, assign17250_e24823_d_n7, assign17250_e24823_d_n10, assign17250_e24823_d_n11, assign17250_e24823_d_n12, assign17250_e24823_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard510 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17250_e24823;
        locals.var_qdrat_dn0 = assign17250_e24823_d_n0;
        locals.var_qdrat_dn2 = assign17250_e24823_d_n2;
        locals.var_qdrat_dn6 = assign17250_e24823_d_n6;
        locals.var_qdrat_dn7 = assign17250_e24823_d_n7;
        locals.var_qdrat_dn10 = assign17250_e24823_d_n10;
        locals.var_qdrat_dn11 = assign17250_e24823_d_n11;
        locals.var_qdrat_dn12 = assign17250_e24823_d_n12;
        locals.var_qdrat_dn17 = assign17250_e24823_d_n17;
        locals.var_qdrat_rv = 0.0;

        let (assign17260_e24827, assign17260_e24827_d_n0, assign17260_e24827_d_n2, assign17260_e24827_d_n6, assign17260_e24827_d_n7, assign17260_e24827_d_n10, assign17260_e24827_d_n11, assign17260_e24827_d_n12, assign17260_e24827_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    } else {
        (locals.var_qdrat_noi, locals.var_qdrat_noi_dn0, locals.var_qdrat_noi_dn2, locals.var_qdrat_noi_dn6, locals.var_qdrat_noi_dn7, locals.var_qdrat_noi_dn10, locals.var_qdrat_noi_dn11, locals.var_qdrat_noi_dn12, locals.var_qdrat_noi_dn17,)
    }
};
        locals.var_qdrat_noi = assign17260_e24827;
        locals.var_qdrat_noi_dn0 = assign17260_e24827_d_n0;
        locals.var_qdrat_noi_dn2 = assign17260_e24827_d_n2;
        locals.var_qdrat_noi_dn6 = assign17260_e24827_d_n6;
        locals.var_qdrat_noi_dn7 = assign17260_e24827_d_n7;
        locals.var_qdrat_noi_dn10 = assign17260_e24827_d_n10;
        locals.var_qdrat_noi_dn11 = assign17260_e24827_d_n11;
        locals.var_qdrat_noi_dn12 = assign17260_e24827_d_n12;
        locals.var_qdrat_noi_dn17 = assign17260_e24827_d_n17;
        locals.var_qdrat_noi_rv = 0.0;

        let (assign17270_e24831, assign17270_e24831_d_n0, assign17270_e24831_d_n2, assign17270_e24831_d_n6, assign17270_e24831_d_n7, assign17270_e24831_d_n10, assign17270_e24831_d_n11, assign17270_e24831_d_n12, assign17270_e24831_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign17270_e24831;
        locals.var_qdrat_dn0 = assign17270_e24831_d_n0;
        locals.var_qdrat_dn2 = assign17270_e24831_d_n2;
        locals.var_qdrat_dn6 = assign17270_e24831_d_n6;
        locals.var_qdrat_dn7 = assign17270_e24831_d_n7;
        locals.var_qdrat_dn10 = assign17270_e24831_d_n10;
        locals.var_qdrat_dn11 = assign17270_e24831_d_n11;
        locals.var_qdrat_dn12 = assign17270_e24831_d_n12;
        locals.var_qdrat_dn17 = assign17270_e24831_d_n17;
        locals.var_qdrat_rv = 0.0;

        let assign17280_e24834: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard512 = assign17280_e24834;
        locals.var_guard512_rv = 0.0;

        let assign17290_e24838: f64 = (10.0 * 2.220446049250313e-16);
        let assign17290_e24843: f64 = (10.0 * 2.220446049250313e-16);
        let assign17290_e24845: f64 = if ((p.p190 < assign17290_e24838) && (p.p191 < assign17290_e24843)) { 1.0 } else { 0.0 };
        locals.var_guard528 = assign17290_e24845;
        locals.var_guard528_rv = 0.0;

        let (assign17300_e24853, assign17300_e24853_d_n0, assign17300_e24853_d_n2, assign17300_e24853_d_n6, assign17300_e24853_d_n7, assign17300_e24853_d_n10, assign17300_e24853_d_n11, assign17300_e24853_d_n12, assign17300_e24853_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17300_e24853;
        locals.var_lred_dn0 = assign17300_e24853_d_n0;
        locals.var_lred_dn2 = assign17300_e24853_d_n2;
        locals.var_lred_dn6 = assign17300_e24853_d_n6;
        locals.var_lred_dn7 = assign17300_e24853_d_n7;
        locals.var_lred_dn10 = assign17300_e24853_d_n10;
        locals.var_lred_dn11 = assign17300_e24853_d_n11;
        locals.var_lred_dn12 = assign17300_e24853_d_n12;
        locals.var_lred_dn17 = assign17300_e24853_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17310_e24861, assign17310_e24861_d_n0, assign17310_e24861_d_n2, assign17310_e24861_d_n6, assign17310_e24861_d_n7, assign17310_e24861_d_n10, assign17310_e24861_d_n11, assign17310_e24861_d_n12, assign17310_e24861_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17310_e24861;
        locals.var_psdl_dn0 = assign17310_e24861_d_n0;
        locals.var_psdl_dn2 = assign17310_e24861_d_n2;
        locals.var_psdl_dn6 = assign17310_e24861_d_n6;
        locals.var_psdl_dn7 = assign17310_e24861_d_n7;
        locals.var_psdl_dn10 = assign17310_e24861_d_n10;
        locals.var_psdl_dn11 = assign17310_e24861_d_n11;
        locals.var_psdl_dn12 = assign17310_e24861_d_n12;
        locals.var_psdl_dn17 = assign17310_e24861_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign17320_e24865: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17320_e24868: f64 = (10.0 * 2.220446049250313e-16);
        let assign17320_e24869: f64 = (assign17320_e24865 - assign17320_e24868);
        let assign17320_e24870: f64 = if locals.var_psdl > assign17320_e24869 { 1.0 } else { 0.0 };
        locals.var_guard529 = assign17320_e24870;
        locals.var_guard529_rv = 0.0;

        let (assign17330_e24886, assign17330_e24886_d_n0, assign17330_e24886_d_n2, assign17330_e24886_d_n6, assign17330_e24886_d_n7, assign17330_e24886_d_n10, assign17330_e24886_d_n11, assign17330_e24886_d_n12, assign17330_e24886_d_n17,) = {
    if ((((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 != 0.0)) {
        let assign17330_e24880: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17330_e24883: f64 = (10.0 * 2.220446049250313e-16);
        let assign17330_e24884: f64 = (assign17330_e24880 - assign17330_e24883);
        (assign17330_e24884, (locals.var_ps0_dn0 + locals.var_vdsz_dn0), (locals.var_ps0_dn2 + locals.var_vdsz_dn2), (locals.var_ps0_dn6 + locals.var_vdsz_dn6), (locals.var_ps0_dn7 + locals.var_vdsz_dn7), (locals.var_ps0_dn10 + locals.var_vdsz_dn10), (locals.var_ps0_dn11 + locals.var_vdsz_dn11), (locals.var_ps0_dn12 + locals.var_vdsz_dn12), (locals.var_ps0_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17330_e24886;
        locals.var_psdl_dn0 = assign17330_e24886_d_n0;
        locals.var_psdl_dn2 = assign17330_e24886_d_n2;
        locals.var_psdl_dn6 = assign17330_e24886_d_n6;
        locals.var_psdl_dn7 = assign17330_e24886_d_n7;
        locals.var_psdl_dn10 = assign17330_e24886_d_n10;
        locals.var_psdl_dn11 = assign17330_e24886_d_n11;
        locals.var_psdl_dn12 = assign17330_e24886_d_n12;
        locals.var_psdl_dn17 = assign17330_e24886_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign17340_e24900,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let (assign17340_e24898,) = {
            if (p.p43 == 1.0) {
                (p.p237,)
            } else {
                (locals.var_wdsoi_0,)
            }
        };
        (assign17340_e24898,)
    } else {
        (locals.var_wd,)
    }
};
        locals.var_wd = assign17340_e24900;
        locals.var_wd_rv = 0.0;

        let (assign17350_e24911,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17350_e24909: f64 = (1.0 / locals.var_wd);
        (assign17350_e24909,)
    } else {
        (locals.var_t0__blk513,)
    }
};
        locals.var_t0__blk513 = assign17350_e24911;
        locals.var_t0__blk513_rv = 0.0;

        let (assign17360_e24922, assign17360_e24922_d_n0, assign17360_e24922_d_n2, assign17360_e24922_d_n6, assign17360_e24922_d_n7, assign17360_e24922_d_n10, assign17360_e24922_d_n11, assign17360_e24922_d_n12, assign17360_e24922_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17360_e24920: f64 = (locals.var_qn0 * locals.var_t0__blk513);
        (assign17360_e24920, (locals.var_qn0_dn0 * locals.var_t0__blk513), (locals.var_qn0_dn2 * locals.var_t0__blk513), (locals.var_qn0_dn6 * locals.var_t0__blk513), (locals.var_qn0_dn7 * locals.var_t0__blk513), (locals.var_qn0_dn10 * locals.var_t0__blk513), (locals.var_qn0_dn11 * locals.var_t0__blk513), (locals.var_qn0_dn12 * locals.var_t0__blk513), (locals.var_qn0_dn17 * locals.var_t0__blk513),)
    } else {
        (locals.var_t1__blk514, locals.var_t1__blk514_dn0, locals.var_t1__blk514_dn2, locals.var_t1__blk514_dn6, locals.var_t1__blk514_dn7, locals.var_t1__blk514_dn10, locals.var_t1__blk514_dn11, locals.var_t1__blk514_dn12, locals.var_t1__blk514_dn17,)
    }
};
        locals.var_t1__blk514 = assign17360_e24922;
        locals.var_t1__blk514_dn0 = assign17360_e24922_d_n0;
        locals.var_t1__blk514_dn2 = assign17360_e24922_d_n2;
        locals.var_t1__blk514_dn6 = assign17360_e24922_d_n6;
        locals.var_t1__blk514_dn7 = assign17360_e24922_d_n7;
        locals.var_t1__blk514_dn10 = assign17360_e24922_d_n10;
        locals.var_t1__blk514_dn11 = assign17360_e24922_d_n11;
        locals.var_t1__blk514_dn12 = assign17360_e24922_d_n12;
        locals.var_t1__blk514_dn17 = assign17360_e24922_d_n17;
        locals.var_t1__blk514_rv = 0.0;

        let (assign17370_e24933, assign17370_e24933_d_n0, assign17370_e24933_d_n2, assign17370_e24933_d_n6, assign17370_e24933_d_n7, assign17370_e24933_d_n10, assign17370_e24933_d_n11, assign17370_e24933_d_n12, assign17370_e24933_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17370_e24931: f64 = (p.p191 * locals.var_t1__blk514);
        (assign17370_e24931, (p.p191 * locals.var_t1__blk514_dn0), (p.p191 * locals.var_t1__blk514_dn2), (p.p191 * locals.var_t1__blk514_dn6), (p.p191 * locals.var_t1__blk514_dn7), (p.p191 * locals.var_t1__blk514_dn10), (p.p191 * locals.var_t1__blk514_dn11), (p.p191 * locals.var_t1__blk514_dn12), (p.p191 * locals.var_t1__blk514_dn17),)
    } else {
        (locals.var_t2__blk515, locals.var_t2__blk515_dn0, locals.var_t2__blk515_dn2, locals.var_t2__blk515_dn6, locals.var_t2__blk515_dn7, locals.var_t2__blk515_dn10, locals.var_t2__blk515_dn11, locals.var_t2__blk515_dn12, locals.var_t2__blk515_dn17,)
    }
};
        locals.var_t2__blk515 = assign17370_e24933;
        locals.var_t2__blk515_dn0 = assign17370_e24933_d_n0;
        locals.var_t2__blk515_dn2 = assign17370_e24933_d_n2;
        locals.var_t2__blk515_dn6 = assign17370_e24933_d_n6;
        locals.var_t2__blk515_dn7 = assign17370_e24933_d_n7;
        locals.var_t2__blk515_dn10 = assign17370_e24933_d_n10;
        locals.var_t2__blk515_dn11 = assign17370_e24933_d_n11;
        locals.var_t2__blk515_dn12 = assign17370_e24933_d_n12;
        locals.var_t2__blk515_dn17 = assign17370_e24933_d_n17;
        locals.var_t2__blk515_rv = 0.0;

        let (assign17380_e24946, assign17380_e24946_d_n0, assign17380_e24946_d_n2, assign17380_e24946_d_n6, assign17380_e24946_d_n7, assign17380_e24946_d_n10, assign17380_e24946_d_n11, assign17380_e24946_d_n12, assign17380_e24946_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17380_e24942: f64 = (locals.var_uc_clm2 * locals.var_q_nsub);
        let assign17380_e24944: f64 = (assign17380_e24942 + locals.var_t2__blk515);
        (assign17380_e24944, (((locals.var_uc_clm2_dn0 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn0)) + locals.var_t2__blk515_dn0), (((locals.var_uc_clm2_dn2 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn2)) + locals.var_t2__blk515_dn2), (((locals.var_uc_clm2_dn6 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn6)) + locals.var_t2__blk515_dn6), (((locals.var_uc_clm2_dn7 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn7)) + locals.var_t2__blk515_dn7), (((locals.var_uc_clm2_dn10 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn10)) + locals.var_t2__blk515_dn10), (((locals.var_uc_clm2_dn11 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn11)) + locals.var_t2__blk515_dn11), (((locals.var_uc_clm2_dn12 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn12)) + locals.var_t2__blk515_dn12), (((locals.var_uc_clm2_dn17 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn17)) + locals.var_t2__blk515_dn17),)
    } else {
        (locals.var_t5__blk518, locals.var_t5__blk518_dn0, locals.var_t5__blk518_dn2, locals.var_t5__blk518_dn6, locals.var_t5__blk518_dn7, locals.var_t5__blk518_dn10, locals.var_t5__blk518_dn11, locals.var_t5__blk518_dn12, locals.var_t5__blk518_dn17,)
    }
};
        locals.var_t5__blk518 = assign17380_e24946;
        locals.var_t5__blk518_dn0 = assign17380_e24946_d_n0;
        locals.var_t5__blk518_dn2 = assign17380_e24946_d_n2;
        locals.var_t5__blk518_dn6 = assign17380_e24946_d_n6;
        locals.var_t5__blk518_dn7 = assign17380_e24946_d_n7;
        locals.var_t5__blk518_dn10 = assign17380_e24946_d_n10;
        locals.var_t5__blk518_dn11 = assign17380_e24946_d_n11;
        locals.var_t5__blk518_dn12 = assign17380_e24946_d_n12;
        locals.var_t5__blk518_dn17 = assign17380_e24946_d_n17;
        locals.var_t5__blk518_rv = 0.0;

        let (assign17390_e24957, assign17390_e24957_d_n0, assign17390_e24957_d_n2, assign17390_e24957_d_n6, assign17390_e24957_d_n7, assign17390_e24957_d_n10, assign17390_e24957_d_n11, assign17390_e24957_d_n12, assign17390_e24957_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17390_e24955: f64 = (1.0 / locals.var_t5__blk518);
        (assign17390_e24955, (-(locals.var_t5__blk518_dn0 / (locals.var_t5__blk518 * locals.var_t5__blk518))), (-(locals.var_t5__blk518_dn2 / (locals.var_t5__blk518 * locals.var_t5__blk518))), (-(locals.var_t5__blk518_dn6 / (locals.var_t5__blk518 * locals.var_t5__blk518))), (-(locals.var_t5__blk518_dn7 / (locals.var_t5__blk518 * locals.var_t5__blk518))), (-(locals.var_t5__blk518_dn10 / (locals.var_t5__blk518 * locals.var_t5__blk518))), (-(locals.var_t5__blk518_dn11 / (locals.var_t5__blk518 * locals.var_t5__blk518))), (-(locals.var_t5__blk518_dn12 / (locals.var_t5__blk518 * locals.var_t5__blk518))), (-(locals.var_t5__blk518_dn17 / (locals.var_t5__blk518 * locals.var_t5__blk518))),)
    } else {
        (locals.var_t1__blk514, locals.var_t1__blk514_dn0, locals.var_t1__blk514_dn2, locals.var_t1__blk514_dn6, locals.var_t1__blk514_dn7, locals.var_t1__blk514_dn10, locals.var_t1__blk514_dn11, locals.var_t1__blk514_dn12, locals.var_t1__blk514_dn17,)
    }
};
        locals.var_t1__blk514 = assign17390_e24957;
        locals.var_t1__blk514_dn0 = assign17390_e24957_d_n0;
        locals.var_t1__blk514_dn2 = assign17390_e24957_d_n2;
        locals.var_t1__blk514_dn6 = assign17390_e24957_d_n6;
        locals.var_t1__blk514_dn7 = assign17390_e24957_d_n7;
        locals.var_t1__blk514_dn10 = assign17390_e24957_d_n10;
        locals.var_t1__blk514_dn11 = assign17390_e24957_d_n11;
        locals.var_t1__blk514_dn12 = assign17390_e24957_d_n12;
        locals.var_t1__blk514_dn17 = assign17390_e24957_d_n17;
        locals.var_t1__blk514_rv = 0.0;

        let (assign17400_e24968, assign17400_e24968_d_n0, assign17400_e24968_d_n2, assign17400_e24968_d_n6, assign17400_e24968_d_n7, assign17400_e24968_d_n10, assign17400_e24968_d_n11, assign17400_e24968_d_n12, assign17400_e24968_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17400_e24966: f64 = (1.034943e-10 * locals.var_t1__blk514);
        (assign17400_e24966, (1.034943e-10 * locals.var_t1__blk514_dn0), (1.034943e-10 * locals.var_t1__blk514_dn2), (1.034943e-10 * locals.var_t1__blk514_dn6), (1.034943e-10 * locals.var_t1__blk514_dn7), (1.034943e-10 * locals.var_t1__blk514_dn10), (1.034943e-10 * locals.var_t1__blk514_dn11), (1.034943e-10 * locals.var_t1__blk514_dn12), (1.034943e-10 * locals.var_t1__blk514_dn17),)
    } else {
        (locals.var_t4__blk517, locals.var_t4__blk517_dn0, locals.var_t4__blk517_dn2, locals.var_t4__blk517_dn6, locals.var_t4__blk517_dn7, locals.var_t4__blk517_dn10, locals.var_t4__blk517_dn11, locals.var_t4__blk517_dn12, locals.var_t4__blk517_dn17,)
    }
};
        locals.var_t4__blk517 = assign17400_e24968;
        locals.var_t4__blk517_dn0 = assign17400_e24968_d_n0;
        locals.var_t4__blk517_dn2 = assign17400_e24968_d_n2;
        locals.var_t4__blk517_dn6 = assign17400_e24968_d_n6;
        locals.var_t4__blk517_dn7 = assign17400_e24968_d_n7;
        locals.var_t4__blk517_dn10 = assign17400_e24968_d_n10;
        locals.var_t4__blk517_dn11 = assign17400_e24968_d_n11;
        locals.var_t4__blk517_dn12 = assign17400_e24968_d_n12;
        locals.var_t4__blk517_dn17 = assign17400_e24968_d_n17;
        locals.var_t4__blk517_rv = 0.0;

        let (assign17410_e24979, assign17410_e24979_d_n0, assign17410_e24979_d_n2, assign17410_e24979_d_n6, assign17410_e24979_d_n7, assign17410_e24979_d_n10, assign17410_e24979_d_n11, assign17410_e24979_d_n12, assign17410_e24979_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17410_e24977: f64 = (1.0 - p.p189);
        (assign17410_e24977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk514, locals.var_t1__blk514_dn0, locals.var_t1__blk514_dn2, locals.var_t1__blk514_dn6, locals.var_t1__blk514_dn7, locals.var_t1__blk514_dn10, locals.var_t1__blk514_dn11, locals.var_t1__blk514_dn12, locals.var_t1__blk514_dn17,)
    }
};
        locals.var_t1__blk514 = assign17410_e24979;
        locals.var_t1__blk514_dn0 = assign17410_e24979_d_n0;
        locals.var_t1__blk514_dn2 = assign17410_e24979_d_n2;
        locals.var_t1__blk514_dn6 = assign17410_e24979_d_n6;
        locals.var_t1__blk514_dn7 = assign17410_e24979_d_n7;
        locals.var_t1__blk514_dn10 = assign17410_e24979_d_n10;
        locals.var_t1__blk514_dn11 = assign17410_e24979_d_n11;
        locals.var_t1__blk514_dn12 = assign17410_e24979_d_n12;
        locals.var_t1__blk514_dn17 = assign17410_e24979_d_n17;
        locals.var_t1__blk514_rv = 0.0;

        let (assign17420_e24996, assign17420_e24996_d_n0, assign17420_e24996_d_n2, assign17420_e24996_d_n6, assign17420_e24996_d_n7, assign17420_e24996_d_n10, assign17420_e24996_d_n11, assign17420_e24996_d_n12, assign17420_e24996_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17420_e24989: f64 = (locals.var_vds + locals.var_ps0);
        let assign17420_e24990: f64 = (p.p189 * assign17420_e24989);
        let assign17420_e24993: f64 = (locals.var_t1__blk514 * locals.var_psl);
        let assign17420_e24994: f64 = (assign17420_e24990 + assign17420_e24993);
        (assign17420_e24994, ((p.p189 * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + ((locals.var_t1__blk514_dn0 * locals.var_psl) + (locals.var_t1__blk514 * locals.var_psl_dn0))), ((p.p189 * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + ((locals.var_t1__blk514_dn2 * locals.var_psl) + (locals.var_t1__blk514 * locals.var_psl_dn2))), ((p.p189 * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + ((locals.var_t1__blk514_dn6 * locals.var_psl) + (locals.var_t1__blk514 * locals.var_psl_dn6))), ((p.p189 * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + ((locals.var_t1__blk514_dn7 * locals.var_psl) + (locals.var_t1__blk514 * locals.var_psl_dn7))), ((p.p189 * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + ((locals.var_t1__blk514_dn10 * locals.var_psl) + (locals.var_t1__blk514 * locals.var_psl_dn10))), ((p.p189 * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + ((locals.var_t1__blk514_dn11 * locals.var_psl) + (locals.var_t1__blk514 * locals.var_psl_dn11))), ((p.p189 * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + ((locals.var_t1__blk514_dn12 * locals.var_psl) + (locals.var_t1__blk514 * locals.var_psl_dn12))), ((p.p189 * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + ((locals.var_t1__blk514_dn17 * locals.var_psl) + (locals.var_t1__blk514 * locals.var_psl_dn17))),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17420_e24996;
        locals.var_psdl_dn0 = assign17420_e24996_d_n0;
        locals.var_psdl_dn2 = assign17420_e24996_d_n2;
        locals.var_psdl_dn6 = assign17420_e24996_d_n6;
        locals.var_psdl_dn7 = assign17420_e24996_d_n7;
        locals.var_psdl_dn10 = assign17420_e24996_d_n10;
        locals.var_psdl_dn11 = assign17420_e24996_d_n11;
        locals.var_psdl_dn12 = assign17420_e24996_d_n12;
        locals.var_psdl_dn17 = assign17420_e24996_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign17430_e25000: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17430_e25003: f64 = (10.0 * 2.220446049250313e-16);
        let assign17430_e25004: f64 = (assign17430_e25000 - assign17430_e25003);
        let assign17430_e25005: f64 = if locals.var_psdl > assign17430_e25004 { 1.0 } else { 0.0 };
        locals.var_guard530 = assign17430_e25005;
        locals.var_guard530_rv = 0.0;

        let (assign17440_e25022, assign17440_e25022_d_n0, assign17440_e25022_d_n2, assign17440_e25022_d_n6, assign17440_e25022_d_n7, assign17440_e25022_d_n10, assign17440_e25022_d_n11, assign17440_e25022_d_n12, assign17440_e25022_d_n17,) = {
    if ((((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) && (locals.var_guard530 != 0.0)) {
        let assign17440_e25016: f64 = (locals.var_ps0 + locals.var_vdsz);
        let assign17440_e25019: f64 = (10.0 * 2.220446049250313e-16);
        let assign17440_e25020: f64 = (assign17440_e25016 - assign17440_e25019);
        (assign17440_e25020, (locals.var_ps0_dn0 + locals.var_vdsz_dn0), (locals.var_ps0_dn2 + locals.var_vdsz_dn2), (locals.var_ps0_dn6 + locals.var_vdsz_dn6), (locals.var_ps0_dn7 + locals.var_vdsz_dn7), (locals.var_ps0_dn10 + locals.var_vdsz_dn10), (locals.var_ps0_dn11 + locals.var_vdsz_dn11), (locals.var_ps0_dn12 + locals.var_vdsz_dn12), (locals.var_ps0_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign17440_e25022;
        locals.var_psdl_dn0 = assign17440_e25022_d_n0;
        locals.var_psdl_dn2 = assign17440_e25022_d_n2;
        locals.var_psdl_dn6 = assign17440_e25022_d_n6;
        locals.var_psdl_dn7 = assign17440_e25022_d_n7;
        locals.var_psdl_dn10 = assign17440_e25022_d_n10;
        locals.var_psdl_dn11 = assign17440_e25022_d_n11;
        locals.var_psdl_dn12 = assign17440_e25022_d_n12;
        locals.var_psdl_dn17 = assign17440_e25022_d_n17;
        locals.var_psdl_rv = 0.0;

        let (assign17450_e25033, assign17450_e25033_d_n0, assign17450_e25033_d_n2, assign17450_e25033_d_n6, assign17450_e25033_d_n7, assign17450_e25033_d_n10, assign17450_e25033_d_n11, assign17450_e25033_d_n12, assign17450_e25033_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17450_e25031: f64 = (locals.var_psdl - locals.var_psl);
        (assign17450_e25031, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn11 - locals.var_psl_dn11), (locals.var_psdl_dn12 - locals.var_psl_dn12), (locals.var_psdl_dn17 - locals.var_psl_dn17),)
    } else {
        (locals.var_t6w__blk520, locals.var_t6w__blk520_dn0, locals.var_t6w__blk520_dn2, locals.var_t6w__blk520_dn6, locals.var_t6w__blk520_dn7, locals.var_t6w__blk520_dn10, locals.var_t6w__blk520_dn11, locals.var_t6w__blk520_dn12, locals.var_t6w__blk520_dn17,)
    }
};
        locals.var_t6w__blk520 = assign17450_e25033;
        locals.var_t6w__blk520_dn0 = assign17450_e25033_d_n0;
        locals.var_t6w__blk520_dn2 = assign17450_e25033_d_n2;
        locals.var_t6w__blk520_dn6 = assign17450_e25033_d_n6;
        locals.var_t6w__blk520_dn7 = assign17450_e25033_d_n7;
        locals.var_t6w__blk520_dn10 = assign17450_e25033_d_n10;
        locals.var_t6w__blk520_dn11 = assign17450_e25033_d_n11;
        locals.var_t6w__blk520_dn12 = assign17450_e25033_d_n12;
        locals.var_t6w__blk520_dn17 = assign17450_e25033_d_n17;
        locals.var_t6w__blk520_rv = 0.0;

        let (assign17460_e25051, assign17460_e25051_d_n0, assign17460_e25051_d_n2, assign17460_e25051_d_n6, assign17460_e25051_d_n7, assign17460_e25051_d_n10, assign17460_e25051_d_n11, assign17460_e25051_d_n12, assign17460_e25051_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17460_e25042: f64 = (locals.var_t6w__blk520 * locals.var_t6w__blk520);
        let assign17460_e25045: f64 = (4.0 * 0.001);
        let assign17460_e25047: f64 = (assign17460_e25045 * 0.001);
        let assign17460_e25048: f64 = (assign17460_e25042 + assign17460_e25047);
        let assign17460_e25049: f64 = (assign17460_e25048).sqrt();
        (assign17460_e25049, (((locals.var_t6w__blk520_dn0 * locals.var_t6w__blk520) + (locals.var_t6w__blk520 * locals.var_t6w__blk520_dn0)) / (2.0 * assign17460_e25049)), (((locals.var_t6w__blk520_dn2 * locals.var_t6w__blk520) + (locals.var_t6w__blk520 * locals.var_t6w__blk520_dn2)) / (2.0 * assign17460_e25049)), (((locals.var_t6w__blk520_dn6 * locals.var_t6w__blk520) + (locals.var_t6w__blk520 * locals.var_t6w__blk520_dn6)) / (2.0 * assign17460_e25049)), (((locals.var_t6w__blk520_dn7 * locals.var_t6w__blk520) + (locals.var_t6w__blk520 * locals.var_t6w__blk520_dn7)) / (2.0 * assign17460_e25049)), (((locals.var_t6w__blk520_dn10 * locals.var_t6w__blk520) + (locals.var_t6w__blk520 * locals.var_t6w__blk520_dn10)) / (2.0 * assign17460_e25049)), (((locals.var_t6w__blk520_dn11 * locals.var_t6w__blk520) + (locals.var_t6w__blk520 * locals.var_t6w__blk520_dn11)) / (2.0 * assign17460_e25049)), (((locals.var_t6w__blk520_dn12 * locals.var_t6w__blk520) + (locals.var_t6w__blk520 * locals.var_t6w__blk520_dn12)) / (2.0 * assign17460_e25049)), (((locals.var_t6w__blk520_dn17 * locals.var_t6w__blk520) + (locals.var_t6w__blk520 * locals.var_t6w__blk520_dn17)) / (2.0 * assign17460_e25049)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17460_e25051;
        locals.var_tmf1_dn0 = assign17460_e25051_d_n0;
        locals.var_tmf1_dn2 = assign17460_e25051_d_n2;
        locals.var_tmf1_dn6 = assign17460_e25051_d_n6;
        locals.var_tmf1_dn7 = assign17460_e25051_d_n7;
        locals.var_tmf1_dn10 = assign17460_e25051_d_n10;
        locals.var_tmf1_dn11 = assign17460_e25051_d_n11;
        locals.var_tmf1_dn12 = assign17460_e25051_d_n12;
        locals.var_tmf1_dn17 = assign17460_e25051_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign17470_e25068, assign17470_e25068_d_n0, assign17470_e25068_d_n2, assign17470_e25068_d_n6, assign17470_e25068_d_n7, assign17470_e25068_d_n10, assign17470_e25068_d_n11, assign17470_e25068_d_n12, assign17470_e25068_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17470_e25061: f64 = (locals.var_t6w__blk520 + locals.var_tmf1);
        let assign17470_e25062: f64 = (0.5 * assign17470_e25061);
        let assign17470_e25065: f64 = (1e-10 * 0.001);
        let assign17470_e25066: f64 = (assign17470_e25062 + assign17470_e25065);
        (assign17470_e25066, (0.5 * (locals.var_t6w__blk520_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t6w__blk520_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t6w__blk520_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t6w__blk520_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t6w__blk520_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t6w__blk520_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t6w__blk520_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t6w__blk520_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t6__blk519, locals.var_t6__blk519_dn0, locals.var_t6__blk519_dn2, locals.var_t6__blk519_dn6, locals.var_t6__blk519_dn7, locals.var_t6__blk519_dn10, locals.var_t6__blk519_dn11, locals.var_t6__blk519_dn12, locals.var_t6__blk519_dn17,)
    }
};
        locals.var_t6__blk519 = assign17470_e25068;
        locals.var_t6__blk519_dn0 = assign17470_e25068_d_n0;
        locals.var_t6__blk519_dn2 = assign17470_e25068_d_n2;
        locals.var_t6__blk519_dn6 = assign17470_e25068_d_n6;
        locals.var_t6__blk519_dn7 = assign17470_e25068_d_n7;
        locals.var_t6__blk519_dn10 = assign17470_e25068_d_n10;
        locals.var_t6__blk519_dn11 = assign17470_e25068_d_n11;
        locals.var_t6__blk519_dn12 = assign17470_e25068_d_n12;
        locals.var_t6__blk519_dn17 = assign17470_e25068_d_n17;
        locals.var_t6__blk519_rv = 0.0;

        let assign17480_e25071: f64 = if locals.var_t6__blk519 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign17480_e25071;
        locals.var_guard531_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17490_e25082, assign17490_e25082_d_n0, assign17490_e25082_d_n2, assign17490_e25082_d_n6, assign17490_e25082_d_n7, assign17490_e25082_d_n10, assign17490_e25082_d_n11, assign17490_e25082_d_n12, assign17490_e25082_d_n17,) = {
    if ((((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) && (locals.var_guard531 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk519, locals.var_t6__blk519_dn0, locals.var_t6__blk519_dn2, locals.var_t6__blk519_dn6, locals.var_t6__blk519_dn7, locals.var_t6__blk519_dn10, locals.var_t6__blk519_dn11, locals.var_t6__blk519_dn12, locals.var_t6__blk519_dn17,)
    }
};
        locals.var_t6__blk519 = assign17490_e25082;
        locals.var_t6__blk519_dn0 = assign17490_e25082_d_n0;
        locals.var_t6__blk519_dn2 = assign17490_e25082_d_n2;
        locals.var_t6__blk519_dn6 = assign17490_e25082_d_n6;
        locals.var_t6__blk519_dn7 = assign17490_e25082_d_n7;
        locals.var_t6__blk519_dn10 = assign17490_e25082_d_n10;
        locals.var_t6__blk519_dn11 = assign17490_e25082_d_n11;
        locals.var_t6__blk519_dn12 = assign17490_e25082_d_n12;
        locals.var_t6__blk519_dn17 = assign17490_e25082_d_n17;
        locals.var_t6__blk519_rv = 0.0;

        let (assign17500_e25093, assign17500_e25093_d_n0, assign17500_e25093_d_n2, assign17500_e25093_d_n6, assign17500_e25093_d_n7, assign17500_e25093_d_n10, assign17500_e25093_d_n11, assign17500_e25093_d_n12, assign17500_e25093_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17500_e25091: f64 = (locals.var_beta * locals.var_qn0);
        (assign17500_e25091, (locals.var_beta * locals.var_qn0_dn0), (locals.var_beta * locals.var_qn0_dn2), (locals.var_beta * locals.var_qn0_dn6), (locals.var_beta * locals.var_qn0_dn7), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), (locals.var_beta * locals.var_qn0_dn11), (locals.var_beta * locals.var_qn0_dn12), (locals.var_beta * locals.var_qn0_dn17),)
    } else {
        (locals.var_t3__blk516, locals.var_t3__blk516_dn0, locals.var_t3__blk516_dn2, locals.var_t3__blk516_dn6, locals.var_t3__blk516_dn7, locals.var_t3__blk516_dn10, locals.var_t3__blk516_dn11, locals.var_t3__blk516_dn12, locals.var_t3__blk516_dn17,)
    }
};
        locals.var_t3__blk516 = assign17500_e25093;
        locals.var_t3__blk516_dn0 = assign17500_e25093_d_n0;
        locals.var_t3__blk516_dn2 = assign17500_e25093_d_n2;
        locals.var_t3__blk516_dn6 = assign17500_e25093_d_n6;
        locals.var_t3__blk516_dn7 = assign17500_e25093_d_n7;
        locals.var_t3__blk516_dn10 = assign17500_e25093_d_n10;
        locals.var_t3__blk516_dn11 = assign17500_e25093_d_n11;
        locals.var_t3__blk516_dn12 = assign17500_e25093_d_n12;
        locals.var_t3__blk516_dn17 = assign17500_e25093_d_n17;
        locals.var_t3__blk516_rv = 0.0;

        let (assign17510_e25104, assign17510_e25104_d_n0, assign17510_e25104_d_n2, assign17510_e25104_d_n6, assign17510_e25104_d_n7, assign17510_e25104_d_n10, assign17510_e25104_d_n11, assign17510_e25104_d_n12, assign17510_e25104_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17510_e25102: f64 = (1.0 / locals.var_t3__blk516);
        (assign17510_e25102, (-(locals.var_t3__blk516_dn0 / (locals.var_t3__blk516 * locals.var_t3__blk516))), (-(locals.var_t3__blk516_dn2 / (locals.var_t3__blk516 * locals.var_t3__blk516))), (-(locals.var_t3__blk516_dn6 / (locals.var_t3__blk516 * locals.var_t3__blk516))), (-(locals.var_t3__blk516_dn7 / (locals.var_t3__blk516 * locals.var_t3__blk516))), (-(locals.var_t3__blk516_dn10 / (locals.var_t3__blk516 * locals.var_t3__blk516))), (-(locals.var_t3__blk516_dn11 / (locals.var_t3__blk516 * locals.var_t3__blk516))), (-(locals.var_t3__blk516_dn12 / (locals.var_t3__blk516 * locals.var_t3__blk516))), (-(locals.var_t3__blk516_dn17 / (locals.var_t3__blk516 * locals.var_t3__blk516))),)
    } else {
        (locals.var_t1__blk514, locals.var_t1__blk514_dn0, locals.var_t1__blk514_dn2, locals.var_t1__blk514_dn6, locals.var_t1__blk514_dn7, locals.var_t1__blk514_dn10, locals.var_t1__blk514_dn11, locals.var_t1__blk514_dn12, locals.var_t1__blk514_dn17,)
    }
};
        locals.var_t1__blk514 = assign17510_e25104;
        locals.var_t1__blk514_dn0 = assign17510_e25104_d_n0;
        locals.var_t1__blk514_dn2 = assign17510_e25104_d_n2;
        locals.var_t1__blk514_dn6 = assign17510_e25104_d_n6;
        locals.var_t1__blk514_dn7 = assign17510_e25104_d_n7;
        locals.var_t1__blk514_dn10 = assign17510_e25104_d_n10;
        locals.var_t1__blk514_dn11 = assign17510_e25104_d_n11;
        locals.var_t1__blk514_dn12 = assign17510_e25104_d_n12;
        locals.var_t1__blk514_dn17 = assign17510_e25104_d_n17;
        locals.var_t1__blk514_rv = 0.0;

        let (assign17520_e25115, assign17520_e25115_d_n0, assign17520_e25115_d_n2, assign17520_e25115_d_n6, assign17520_e25115_d_n7, assign17520_e25115_d_n10, assign17520_e25115_d_n11, assign17520_e25115_d_n12, assign17520_e25115_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17520_e25113: f64 = (locals.var_idd * locals.var_t1__blk514);
        (assign17520_e25113, ((locals.var_idd_dn0 * locals.var_t1__blk514) + (locals.var_idd * locals.var_t1__blk514_dn0)), ((locals.var_idd_dn2 * locals.var_t1__blk514) + (locals.var_idd * locals.var_t1__blk514_dn2)), ((locals.var_idd_dn6 * locals.var_t1__blk514) + (locals.var_idd * locals.var_t1__blk514_dn6)), ((locals.var_idd_dn7 * locals.var_t1__blk514) + (locals.var_idd * locals.var_t1__blk514_dn7)), ((locals.var_idd_dn10 * locals.var_t1__blk514) + (locals.var_idd * locals.var_t1__blk514_dn10)), ((locals.var_idd_dn11 * locals.var_t1__blk514) + (locals.var_idd * locals.var_t1__blk514_dn11)), ((locals.var_idd_dn12 * locals.var_t1__blk514) + (locals.var_idd * locals.var_t1__blk514_dn12)), ((locals.var_idd_dn17 * locals.var_t1__blk514) + (locals.var_idd * locals.var_t1__blk514_dn17)),)
    } else {
        (locals.var_t5__blk518, locals.var_t5__blk518_dn0, locals.var_t5__blk518_dn2, locals.var_t5__blk518_dn6, locals.var_t5__blk518_dn7, locals.var_t5__blk518_dn10, locals.var_t5__blk518_dn11, locals.var_t5__blk518_dn12, locals.var_t5__blk518_dn17,)
    }
};
        locals.var_t5__blk518 = assign17520_e25115;
        locals.var_t5__blk518_dn0 = assign17520_e25115_d_n0;
        locals.var_t5__blk518_dn2 = assign17520_e25115_d_n2;
        locals.var_t5__blk518_dn6 = assign17520_e25115_d_n6;
        locals.var_t5__blk518_dn7 = assign17520_e25115_d_n7;
        locals.var_t5__blk518_dn10 = assign17520_e25115_d_n10;
        locals.var_t5__blk518_dn11 = assign17520_e25115_d_n11;
        locals.var_t5__blk518_dn12 = assign17520_e25115_d_n12;
        locals.var_t5__blk518_dn17 = assign17520_e25115_d_n17;
        locals.var_t5__blk518_rv = 0.0;

        let assign17530_e25118: f64 = if locals.var_t5__blk518 < locals.var_beta_inv { 1.0 } else { 0.0 };
        locals.var_guard532 = assign17530_e25118;
        locals.var_guard532_rv = 0.0;

        let (assign17540_e25129, assign17540_e25129_d_n0, assign17540_e25129_d_n2, assign17540_e25129_d_n6, assign17540_e25129_d_n7, assign17540_e25129_d_n10, assign17540_e25129_d_n11, assign17540_e25129_d_n12, assign17540_e25129_d_n17,) = {
    if ((((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) && (locals.var_guard532 != 0.0)) {
        (locals.var_beta_inv, 0.0, 0.0, 0.0, 0.0, locals.var_beta_inv_dn10, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk518, locals.var_t5__blk518_dn0, locals.var_t5__blk518_dn2, locals.var_t5__blk518_dn6, locals.var_t5__blk518_dn7, locals.var_t5__blk518_dn10, locals.var_t5__blk518_dn11, locals.var_t5__blk518_dn12, locals.var_t5__blk518_dn17,)
    }
};
        locals.var_t5__blk518 = assign17540_e25129;
        locals.var_t5__blk518_dn0 = assign17540_e25129_d_n0;
        locals.var_t5__blk518_dn2 = assign17540_e25129_d_n2;
        locals.var_t5__blk518_dn6 = assign17540_e25129_d_n6;
        locals.var_t5__blk518_dn7 = assign17540_e25129_d_n7;
        locals.var_t5__blk518_dn10 = assign17540_e25129_d_n10;
        locals.var_t5__blk518_dn11 = assign17540_e25129_d_n11;
        locals.var_t5__blk518_dn12 = assign17540_e25129_d_n12;
        locals.var_t5__blk518_dn17 = assign17540_e25129_d_n17;
        locals.var_t5__blk518_rv = 0.0;

        let (assign17550_e25140, assign17550_e25140_d_n0, assign17550_e25140_d_n2, assign17550_e25140_d_n6, assign17550_e25140_d_n7, assign17550_e25140_d_n10, assign17550_e25140_d_n11, assign17550_e25140_d_n12, assign17550_e25140_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17550_e25138: f64 = (locals.var_q_nsub / 1.034943e-10);
        (assign17550_e25138, (locals.var_q_nsub_dn0 / 1.034943e-10), (locals.var_q_nsub_dn2 / 1.034943e-10), (locals.var_q_nsub_dn6 / 1.034943e-10), (locals.var_q_nsub_dn7 / 1.034943e-10), (locals.var_q_nsub_dn10 / 1.034943e-10), (locals.var_q_nsub_dn11 / 1.034943e-10), (locals.var_q_nsub_dn12 / 1.034943e-10), (locals.var_q_nsub_dn17 / 1.034943e-10),)
    } else {
        (locals.var_t10__blk524, locals.var_t10__blk524_dn0, locals.var_t10__blk524_dn2, locals.var_t10__blk524_dn6, locals.var_t10__blk524_dn7, locals.var_t10__blk524_dn10, locals.var_t10__blk524_dn11, locals.var_t10__blk524_dn12, locals.var_t10__blk524_dn17,)
    }
};
        locals.var_t10__blk524 = assign17550_e25140;
        locals.var_t10__blk524_dn0 = assign17550_e25140_d_n0;
        locals.var_t10__blk524_dn2 = assign17550_e25140_d_n2;
        locals.var_t10__blk524_dn6 = assign17550_e25140_d_n6;
        locals.var_t10__blk524_dn7 = assign17550_e25140_d_n7;
        locals.var_t10__blk524_dn10 = assign17550_e25140_d_n10;
        locals.var_t10__blk524_dn11 = assign17550_e25140_d_n11;
        locals.var_t10__blk524_dn12 = assign17550_e25140_d_n12;
        locals.var_t10__blk524_dn17 = assign17550_e25140_d_n17;
        locals.var_t10__blk524_rv = 0.0;

        let (assign17560_e25151, assign17560_e25151_d_n0, assign17560_e25151_d_n2, assign17560_e25151_d_n6, assign17560_e25151_d_n7, assign17560_e25151_d_n10, assign17560_e25151_d_n11, assign17560_e25151_d_n12, assign17560_e25151_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17560_e25149: f64 = (100000.0 * 10000.0);
        (assign17560_e25149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk514, locals.var_t1__blk514_dn0, locals.var_t1__blk514_dn2, locals.var_t1__blk514_dn6, locals.var_t1__blk514_dn7, locals.var_t1__blk514_dn10, locals.var_t1__blk514_dn11, locals.var_t1__blk514_dn12, locals.var_t1__blk514_dn17,)
    }
};
        locals.var_t1__blk514 = assign17560_e25151;
        locals.var_t1__blk514_dn0 = assign17560_e25151_d_n0;
        locals.var_t1__blk514_dn2 = assign17560_e25151_d_n2;
        locals.var_t1__blk514_dn6 = assign17560_e25151_d_n6;
        locals.var_t1__blk514_dn7 = assign17560_e25151_d_n7;
        locals.var_t1__blk514_dn10 = assign17560_e25151_d_n10;
        locals.var_t1__blk514_dn11 = assign17560_e25151_d_n11;
        locals.var_t1__blk514_dn12 = assign17560_e25151_d_n12;
        locals.var_t1__blk514_dn17 = assign17560_e25151_d_n17;
        locals.var_t1__blk514_rv = 0.0;

        let (assign17570_e25162, assign17570_e25162_d_n0, assign17570_e25162_d_n2, assign17570_e25162_d_n6, assign17570_e25162_d_n7, assign17570_e25162_d_n10, assign17570_e25162_d_n11, assign17570_e25162_d_n12, assign17570_e25162_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17570_e25160: f64 = (1.0 / locals.var_leff);
        (assign17570_e25160, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk515, locals.var_t2__blk515_dn0, locals.var_t2__blk515_dn2, locals.var_t2__blk515_dn6, locals.var_t2__blk515_dn7, locals.var_t2__blk515_dn10, locals.var_t2__blk515_dn11, locals.var_t2__blk515_dn12, locals.var_t2__blk515_dn17,)
    }
};
        locals.var_t2__blk515 = assign17570_e25162;
        locals.var_t2__blk515_dn0 = assign17570_e25162_d_n0;
        locals.var_t2__blk515_dn2 = assign17570_e25162_d_n2;
        locals.var_t2__blk515_dn6 = assign17570_e25162_d_n6;
        locals.var_t2__blk515_dn7 = assign17570_e25162_d_n7;
        locals.var_t2__blk515_dn10 = assign17570_e25162_d_n10;
        locals.var_t2__blk515_dn11 = assign17570_e25162_d_n11;
        locals.var_t2__blk515_dn12 = assign17570_e25162_d_n12;
        locals.var_t2__blk515_dn17 = assign17570_e25162_d_n17;
        locals.var_t2__blk515_rv = 0.0;

        let (assign17580_e25187, assign17580_e25187_d_n0, assign17580_e25187_d_n2, assign17580_e25187_d_n6, assign17580_e25187_d_n7, assign17580_e25187_d_n10, assign17580_e25187_d_n11, assign17580_e25187_d_n12, assign17580_e25187_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17580_e25171: f64 = (2.0 * locals.var_t5__blk518);
        let assign17580_e25174: f64 = (2.0 * locals.var_t10__blk524);
        let assign17580_e25176: f64 = (assign17580_e25174 * locals.var_t6__blk519);
        let assign17580_e25178: f64 = (assign17580_e25176 * locals.var_t4__blk517);
        let assign17580_e25179: f64 = (assign17580_e25171 + assign17580_e25178);
        let assign17580_e25182: f64 = (locals.var_t1__blk514 * locals.var_t4__blk517);
        let assign17580_e25183: f64 = (assign17580_e25179 + assign17580_e25182);
        let assign17580_e25185: f64 = (assign17580_e25183 * locals.var_t2__blk515);
        (assign17580_e25185, (((((2.0 * locals.var_t5__blk518_dn0) + (((((2.0 * locals.var_t10__blk524_dn0) * locals.var_t6__blk519) + (assign17580_e25174 * locals.var_t6__blk519_dn0)) * locals.var_t4__blk517) + (assign17580_e25176 * locals.var_t4__blk517_dn0))) + ((locals.var_t1__blk514_dn0 * locals.var_t4__blk517) + (locals.var_t1__blk514 * locals.var_t4__blk517_dn0))) * locals.var_t2__blk515) + (assign17580_e25183 * locals.var_t2__blk515_dn0)), (((((2.0 * locals.var_t5__blk518_dn2) + (((((2.0 * locals.var_t10__blk524_dn2) * locals.var_t6__blk519) + (assign17580_e25174 * locals.var_t6__blk519_dn2)) * locals.var_t4__blk517) + (assign17580_e25176 * locals.var_t4__blk517_dn2))) + ((locals.var_t1__blk514_dn2 * locals.var_t4__blk517) + (locals.var_t1__blk514 * locals.var_t4__blk517_dn2))) * locals.var_t2__blk515) + (assign17580_e25183 * locals.var_t2__blk515_dn2)), (((((2.0 * locals.var_t5__blk518_dn6) + (((((2.0 * locals.var_t10__blk524_dn6) * locals.var_t6__blk519) + (assign17580_e25174 * locals.var_t6__blk519_dn6)) * locals.var_t4__blk517) + (assign17580_e25176 * locals.var_t4__blk517_dn6))) + ((locals.var_t1__blk514_dn6 * locals.var_t4__blk517) + (locals.var_t1__blk514 * locals.var_t4__blk517_dn6))) * locals.var_t2__blk515) + (assign17580_e25183 * locals.var_t2__blk515_dn6)), (((((2.0 * locals.var_t5__blk518_dn7) + (((((2.0 * locals.var_t10__blk524_dn7) * locals.var_t6__blk519) + (assign17580_e25174 * locals.var_t6__blk519_dn7)) * locals.var_t4__blk517) + (assign17580_e25176 * locals.var_t4__blk517_dn7))) + ((locals.var_t1__blk514_dn7 * locals.var_t4__blk517) + (locals.var_t1__blk514 * locals.var_t4__blk517_dn7))) * locals.var_t2__blk515) + (assign17580_e25183 * locals.var_t2__blk515_dn7)), (((((2.0 * locals.var_t5__blk518_dn10) + (((((2.0 * locals.var_t10__blk524_dn10) * locals.var_t6__blk519) + (assign17580_e25174 * locals.var_t6__blk519_dn10)) * locals.var_t4__blk517) + (assign17580_e25176 * locals.var_t4__blk517_dn10))) + ((locals.var_t1__blk514_dn10 * locals.var_t4__blk517) + (locals.var_t1__blk514 * locals.var_t4__blk517_dn10))) * locals.var_t2__blk515) + (assign17580_e25183 * locals.var_t2__blk515_dn10)), (((((2.0 * locals.var_t5__blk518_dn11) + (((((2.0 * locals.var_t10__blk524_dn11) * locals.var_t6__blk519) + (assign17580_e25174 * locals.var_t6__blk519_dn11)) * locals.var_t4__blk517) + (assign17580_e25176 * locals.var_t4__blk517_dn11))) + ((locals.var_t1__blk514_dn11 * locals.var_t4__blk517) + (locals.var_t1__blk514 * locals.var_t4__blk517_dn11))) * locals.var_t2__blk515) + (assign17580_e25183 * locals.var_t2__blk515_dn11)), (((((2.0 * locals.var_t5__blk518_dn12) + (((((2.0 * locals.var_t10__blk524_dn12) * locals.var_t6__blk519) + (assign17580_e25174 * locals.var_t6__blk519_dn12)) * locals.var_t4__blk517) + (assign17580_e25176 * locals.var_t4__blk517_dn12))) + ((locals.var_t1__blk514_dn12 * locals.var_t4__blk517) + (locals.var_t1__blk514 * locals.var_t4__blk517_dn12))) * locals.var_t2__blk515) + (assign17580_e25183 * locals.var_t2__blk515_dn12)), (((((2.0 * locals.var_t5__blk518_dn17) + (((((2.0 * locals.var_t10__blk524_dn17) * locals.var_t6__blk519) + (assign17580_e25174 * locals.var_t6__blk519_dn17)) * locals.var_t4__blk517) + (assign17580_e25176 * locals.var_t4__blk517_dn17))) + ((locals.var_t1__blk514_dn17 * locals.var_t4__blk517) + (locals.var_t1__blk514 * locals.var_t4__blk517_dn17))) * locals.var_t2__blk515) + (assign17580_e25183 * locals.var_t2__blk515_dn17)),)
    } else {
        (locals.var_t11w, locals.var_t11w_dn0, locals.var_t11w_dn2, locals.var_t11w_dn6, locals.var_t11w_dn7, locals.var_t11w_dn10, locals.var_t11w_dn11, locals.var_t11w_dn12, locals.var_t11w_dn17,)
    }
};
        locals.var_t11w = assign17580_e25187;
        locals.var_t11w_dn0 = assign17580_e25187_d_n0;
        locals.var_t11w_dn2 = assign17580_e25187_d_n2;
        locals.var_t11w_dn6 = assign17580_e25187_d_n6;
        locals.var_t11w_dn7 = assign17580_e25187_d_n7;
        locals.var_t11w_dn10 = assign17580_e25187_d_n10;
        locals.var_t11w_dn11 = assign17580_e25187_d_n11;
        locals.var_t11w_dn12 = assign17580_e25187_d_n12;
        locals.var_t11w_dn17 = assign17580_e25187_d_n17;
        locals.var_t11w_rv = 0.0;

        let (assign17590_e25198, assign17590_e25198_d_n0, assign17590_e25198_d_n2, assign17590_e25198_d_n6, assign17590_e25198_d_n7, assign17590_e25198_d_n10, assign17590_e25198_d_n11, assign17590_e25198_d_n12, assign17590_e25198_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17590_e25196: f64 = (locals.var_t11w * locals.var_t4__blk517);
        (assign17590_e25196, ((locals.var_t11w_dn0 * locals.var_t4__blk517) + (locals.var_t11w * locals.var_t4__blk517_dn0)), ((locals.var_t11w_dn2 * locals.var_t4__blk517) + (locals.var_t11w * locals.var_t4__blk517_dn2)), ((locals.var_t11w_dn6 * locals.var_t4__blk517) + (locals.var_t11w * locals.var_t4__blk517_dn6)), ((locals.var_t11w_dn7 * locals.var_t4__blk517) + (locals.var_t11w * locals.var_t4__blk517_dn7)), ((locals.var_t11w_dn10 * locals.var_t4__blk517) + (locals.var_t11w * locals.var_t4__blk517_dn10)), ((locals.var_t11w_dn11 * locals.var_t4__blk517) + (locals.var_t11w * locals.var_t4__blk517_dn11)), ((locals.var_t11w_dn12 * locals.var_t4__blk517) + (locals.var_t11w * locals.var_t4__blk517_dn12)), ((locals.var_t11w_dn17 * locals.var_t4__blk517) + (locals.var_t11w * locals.var_t4__blk517_dn17)),)
    } else {
        (locals.var_t7__blk521, locals.var_t7__blk521_dn0, locals.var_t7__blk521_dn2, locals.var_t7__blk521_dn6, locals.var_t7__blk521_dn7, locals.var_t7__blk521_dn10, locals.var_t7__blk521_dn11, locals.var_t7__blk521_dn12, locals.var_t7__blk521_dn17,)
    }
};
        locals.var_t7__blk521 = assign17590_e25198;
        locals.var_t7__blk521_dn0 = assign17590_e25198_d_n0;
        locals.var_t7__blk521_dn2 = assign17590_e25198_d_n2;
        locals.var_t7__blk521_dn6 = assign17590_e25198_d_n6;
        locals.var_t7__blk521_dn7 = assign17590_e25198_d_n7;
        locals.var_t7__blk521_dn10 = assign17590_e25198_d_n10;
        locals.var_t7__blk521_dn11 = assign17590_e25198_d_n11;
        locals.var_t7__blk521_dn12 = assign17590_e25198_d_n12;
        locals.var_t7__blk521_dn17 = assign17590_e25198_d_n17;
        locals.var_t7__blk521_rv = 0.0;

        let (assign17600_e25215, assign17600_e25215_d_n0, assign17600_e25215_d_n2, assign17600_e25215_d_n6, assign17600_e25215_d_n7, assign17600_e25215_d_n10, assign17600_e25215_d_n11, assign17600_e25215_d_n12, assign17600_e25215_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17600_e25208: f64 = (2.0 * locals.var_t10__blk524);
        let assign17600_e25210: f64 = (assign17600_e25208 * locals.var_t6__blk519);
        let assign17600_e25212: f64 = (assign17600_e25210 + locals.var_t1__blk514);
        let assign17600_e25213: f64 = (4.0 * assign17600_e25212);
        (assign17600_e25213, (4.0 * ((((2.0 * locals.var_t10__blk524_dn0) * locals.var_t6__blk519) + (assign17600_e25208 * locals.var_t6__blk519_dn0)) + locals.var_t1__blk514_dn0)), (4.0 * ((((2.0 * locals.var_t10__blk524_dn2) * locals.var_t6__blk519) + (assign17600_e25208 * locals.var_t6__blk519_dn2)) + locals.var_t1__blk514_dn2)), (4.0 * ((((2.0 * locals.var_t10__blk524_dn6) * locals.var_t6__blk519) + (assign17600_e25208 * locals.var_t6__blk519_dn6)) + locals.var_t1__blk514_dn6)), (4.0 * ((((2.0 * locals.var_t10__blk524_dn7) * locals.var_t6__blk519) + (assign17600_e25208 * locals.var_t6__blk519_dn7)) + locals.var_t1__blk514_dn7)), (4.0 * ((((2.0 * locals.var_t10__blk524_dn10) * locals.var_t6__blk519) + (assign17600_e25208 * locals.var_t6__blk519_dn10)) + locals.var_t1__blk514_dn10)), (4.0 * ((((2.0 * locals.var_t10__blk524_dn11) * locals.var_t6__blk519) + (assign17600_e25208 * locals.var_t6__blk519_dn11)) + locals.var_t1__blk514_dn11)), (4.0 * ((((2.0 * locals.var_t10__blk524_dn12) * locals.var_t6__blk519) + (assign17600_e25208 * locals.var_t6__blk519_dn12)) + locals.var_t1__blk514_dn12)), (4.0 * ((((2.0 * locals.var_t10__blk524_dn17) * locals.var_t6__blk519) + (assign17600_e25208 * locals.var_t6__blk519_dn17)) + locals.var_t1__blk514_dn17)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12, locals.var_t11_dn17,)
    }
};
        locals.var_t11 = assign17600_e25215;
        locals.var_t11_dn0 = assign17600_e25215_d_n0;
        locals.var_t11_dn2 = assign17600_e25215_d_n2;
        locals.var_t11_dn6 = assign17600_e25215_d_n6;
        locals.var_t11_dn7 = assign17600_e25215_d_n7;
        locals.var_t11_dn10 = assign17600_e25215_d_n10;
        locals.var_t11_dn11 = assign17600_e25215_d_n11;
        locals.var_t11_dn12 = assign17600_e25215_d_n12;
        locals.var_t11_dn17 = assign17600_e25215_d_n17;
        locals.var_t11_rv = 0.0;

        let (assign17610_e25228, assign17610_e25228_d_n0, assign17610_e25228_d_n2, assign17610_e25228_d_n6, assign17610_e25228_d_n7, assign17610_e25228_d_n10, assign17610_e25228_d_n11, assign17610_e25228_d_n12, assign17610_e25228_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17610_e25224: f64 = (locals.var_t11 * locals.var_t4__blk517);
        let assign17610_e25226: f64 = (assign17610_e25224 * locals.var_t4__blk517);
        (assign17610_e25226, ((((locals.var_t11_dn0 * locals.var_t4__blk517) + (locals.var_t11 * locals.var_t4__blk517_dn0)) * locals.var_t4__blk517) + (assign17610_e25224 * locals.var_t4__blk517_dn0)), ((((locals.var_t11_dn2 * locals.var_t4__blk517) + (locals.var_t11 * locals.var_t4__blk517_dn2)) * locals.var_t4__blk517) + (assign17610_e25224 * locals.var_t4__blk517_dn2)), ((((locals.var_t11_dn6 * locals.var_t4__blk517) + (locals.var_t11 * locals.var_t4__blk517_dn6)) * locals.var_t4__blk517) + (assign17610_e25224 * locals.var_t4__blk517_dn6)), ((((locals.var_t11_dn7 * locals.var_t4__blk517) + (locals.var_t11 * locals.var_t4__blk517_dn7)) * locals.var_t4__blk517) + (assign17610_e25224 * locals.var_t4__blk517_dn7)), ((((locals.var_t11_dn10 * locals.var_t4__blk517) + (locals.var_t11 * locals.var_t4__blk517_dn10)) * locals.var_t4__blk517) + (assign17610_e25224 * locals.var_t4__blk517_dn10)), ((((locals.var_t11_dn11 * locals.var_t4__blk517) + (locals.var_t11 * locals.var_t4__blk517_dn11)) * locals.var_t4__blk517) + (assign17610_e25224 * locals.var_t4__blk517_dn11)), ((((locals.var_t11_dn12 * locals.var_t4__blk517) + (locals.var_t11 * locals.var_t4__blk517_dn12)) * locals.var_t4__blk517) + (assign17610_e25224 * locals.var_t4__blk517_dn12)), ((((locals.var_t11_dn17 * locals.var_t4__blk517) + (locals.var_t11 * locals.var_t4__blk517_dn17)) * locals.var_t4__blk517) + (assign17610_e25224 * locals.var_t4__blk517_dn17)),)
    } else {
        (locals.var_t8__blk522, locals.var_t8__blk522_dn0, locals.var_t8__blk522_dn2, locals.var_t8__blk522_dn6, locals.var_t8__blk522_dn7, locals.var_t8__blk522_dn10, locals.var_t8__blk522_dn11, locals.var_t8__blk522_dn12, locals.var_t8__blk522_dn17,)
    }
};
        locals.var_t8__blk522 = assign17610_e25228;
        locals.var_t8__blk522_dn0 = assign17610_e25228_d_n0;
        locals.var_t8__blk522_dn2 = assign17610_e25228_d_n2;
        locals.var_t8__blk522_dn6 = assign17610_e25228_d_n6;
        locals.var_t8__blk522_dn7 = assign17610_e25228_d_n7;
        locals.var_t8__blk522_dn10 = assign17610_e25228_d_n10;
        locals.var_t8__blk522_dn11 = assign17610_e25228_d_n11;
        locals.var_t8__blk522_dn12 = assign17610_e25228_d_n12;
        locals.var_t8__blk522_dn17 = assign17610_e25228_d_n17;
        locals.var_t8__blk522_rv = 0.0;

        let (assign17620_e25242, assign17620_e25242_d_n0, assign17620_e25242_d_n2, assign17620_e25242_d_n6, assign17620_e25242_d_n7, assign17620_e25242_d_n10, assign17620_e25242_d_n11, assign17620_e25242_d_n12, assign17620_e25242_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17620_e25237: f64 = (locals.var_t7__blk521 * locals.var_t7__blk521);
        let assign17620_e25239: f64 = (assign17620_e25237 + locals.var_t8__blk522);
        let assign17620_e25240: f64 = (assign17620_e25239).sqrt();
        (assign17620_e25240, ((((locals.var_t7__blk521_dn0 * locals.var_t7__blk521) + (locals.var_t7__blk521 * locals.var_t7__blk521_dn0)) + locals.var_t8__blk522_dn0) / (2.0 * assign17620_e25240)), ((((locals.var_t7__blk521_dn2 * locals.var_t7__blk521) + (locals.var_t7__blk521 * locals.var_t7__blk521_dn2)) + locals.var_t8__blk522_dn2) / (2.0 * assign17620_e25240)), ((((locals.var_t7__blk521_dn6 * locals.var_t7__blk521) + (locals.var_t7__blk521 * locals.var_t7__blk521_dn6)) + locals.var_t8__blk522_dn6) / (2.0 * assign17620_e25240)), ((((locals.var_t7__blk521_dn7 * locals.var_t7__blk521) + (locals.var_t7__blk521 * locals.var_t7__blk521_dn7)) + locals.var_t8__blk522_dn7) / (2.0 * assign17620_e25240)), ((((locals.var_t7__blk521_dn10 * locals.var_t7__blk521) + (locals.var_t7__blk521 * locals.var_t7__blk521_dn10)) + locals.var_t8__blk522_dn10) / (2.0 * assign17620_e25240)), ((((locals.var_t7__blk521_dn11 * locals.var_t7__blk521) + (locals.var_t7__blk521 * locals.var_t7__blk521_dn11)) + locals.var_t8__blk522_dn11) / (2.0 * assign17620_e25240)), ((((locals.var_t7__blk521_dn12 * locals.var_t7__blk521) + (locals.var_t7__blk521 * locals.var_t7__blk521_dn12)) + locals.var_t8__blk522_dn12) / (2.0 * assign17620_e25240)), ((((locals.var_t7__blk521_dn17 * locals.var_t7__blk521) + (locals.var_t7__blk521 * locals.var_t7__blk521_dn17)) + locals.var_t8__blk522_dn17) / (2.0 * assign17620_e25240)),)
    } else {
        (locals.var_t9__blk523, locals.var_t9__blk523_dn0, locals.var_t9__blk523_dn2, locals.var_t9__blk523_dn6, locals.var_t9__blk523_dn7, locals.var_t9__blk523_dn10, locals.var_t9__blk523_dn11, locals.var_t9__blk523_dn12, locals.var_t9__blk523_dn17,)
    }
};
        locals.var_t9__blk523 = assign17620_e25242;
        locals.var_t9__blk523_dn0 = assign17620_e25242_d_n0;
        locals.var_t9__blk523_dn2 = assign17620_e25242_d_n2;
        locals.var_t9__blk523_dn6 = assign17620_e25242_d_n6;
        locals.var_t9__blk523_dn7 = assign17620_e25242_d_n7;
        locals.var_t9__blk523_dn10 = assign17620_e25242_d_n10;
        locals.var_t9__blk523_dn11 = assign17620_e25242_d_n11;
        locals.var_t9__blk523_dn12 = assign17620_e25242_d_n12;
        locals.var_t9__blk523_dn17 = assign17620_e25242_d_n17;
        locals.var_t9__blk523_rv = 0.0;

        let (assign17630_e25258, assign17630_e25258_d_n0, assign17630_e25258_d_n2, assign17630_e25258_d_n6, assign17630_e25258_d_n7, assign17630_e25258_d_n10, assign17630_e25258_d_n11, assign17630_e25258_d_n12, assign17630_e25258_d_n17,) = {
    if (((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign17630_e25252: f64 = (-locals.var_t7__blk521);
        let assign17630_e25254: f64 = (assign17630_e25252 + locals.var_t9__blk523);
        let assign17630_e25255: f64 = (0.5 * assign17630_e25254);
        let assign17630_e25256: f64 = (locals.var_fmdvds * assign17630_e25255);
        (assign17630_e25256, ((locals.var_fmdvds_dn0 * assign17630_e25255) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk521_dn0) + locals.var_t9__blk523_dn0)))), ((locals.var_fmdvds_dn2 * assign17630_e25255) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk521_dn2) + locals.var_t9__blk523_dn2)))), ((locals.var_fmdvds_dn6 * assign17630_e25255) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk521_dn6) + locals.var_t9__blk523_dn6)))), ((locals.var_fmdvds_dn7 * assign17630_e25255) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk521_dn7) + locals.var_t9__blk523_dn7)))), ((locals.var_fmdvds_dn10 * assign17630_e25255) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk521_dn10) + locals.var_t9__blk523_dn10)))), ((locals.var_fmdvds_dn11 * assign17630_e25255) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk521_dn11) + locals.var_t9__blk523_dn11)))), ((locals.var_fmdvds_dn12 * assign17630_e25255) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk521_dn12) + locals.var_t9__blk523_dn12)))), ((locals.var_fmdvds_dn17 * assign17630_e25255) + (locals.var_fmdvds * (0.5 * ((-locals.var_t7__blk521_dn17) + locals.var_t9__blk523_dn17)))),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17630_e25258;
        locals.var_lred_dn0 = assign17630_e25258_d_n0;
        locals.var_lred_dn2 = assign17630_e25258_d_n2;
        locals.var_lred_dn6 = assign17630_e25258_d_n6;
        locals.var_lred_dn7 = assign17630_e25258_d_n7;
        locals.var_lred_dn10 = assign17630_e25258_d_n10;
        locals.var_lred_dn11 = assign17630_e25258_d_n11;
        locals.var_lred_dn12 = assign17630_e25258_d_n12;
        locals.var_lred_dn17 = assign17630_e25258_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17640_e25266, assign17640_e25266_d_n0, assign17640_e25266_d_n2, assign17640_e25266_d_n6, assign17640_e25266_d_n7, assign17640_e25266_d_n10, assign17640_e25266_d_n11, assign17640_e25266_d_n12, assign17640_e25266_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard512 != 0.0)) {
        let assign17640_e25264: f64 = (locals.var_lred * locals.var_clmmod);
        (assign17640_e25264, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn12 * locals.var_clmmod), (locals.var_lred_dn17 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12, locals.var_lred_dn17,)
    }
};
        locals.var_lred = assign17640_e25266;
        locals.var_lred_dn0 = assign17640_e25266_d_n0;
        locals.var_lred_dn2 = assign17640_e25266_d_n2;
        locals.var_lred_dn6 = assign17640_e25266_d_n6;
        locals.var_lred_dn7 = assign17640_e25266_d_n7;
        locals.var_lred_dn10 = assign17640_e25266_d_n10;
        locals.var_lred_dn11 = assign17640_e25266_d_n11;
        locals.var_lred_dn12 = assign17640_e25266_d_n12;
        locals.var_lred_dn17 = assign17640_e25266_d_n17;
        locals.var_lred_rv = 0.0;

        let (assign17650_e25272, assign17650_e25272_d_n0, assign17650_e25272_d_n2, assign17650_e25272_d_n6, assign17650_e25272_d_n7, assign17650_e25272_d_n10, assign17650_e25272_d_n11, assign17650_e25272_d_n12, assign17650_e25272_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17650_e25270: f64 = (locals.var_leff - locals.var_lred);
        (assign17650_e25270, (-locals.var_lred_dn0), (-locals.var_lred_dn2), (-locals.var_lred_dn6), (-locals.var_lred_dn7), (-locals.var_lred_dn10), (-locals.var_lred_dn11), (-locals.var_lred_dn12), (-locals.var_lred_dn17),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    }
};
        locals.var_lch = assign17650_e25272;
        locals.var_lch_dn0 = assign17650_e25272_d_n0;
        locals.var_lch_dn2 = assign17650_e25272_d_n2;
        locals.var_lch_dn6 = assign17650_e25272_d_n6;
        locals.var_lch_dn7 = assign17650_e25272_d_n7;
        locals.var_lch_dn10 = assign17650_e25272_d_n10;
        locals.var_lch_dn11 = assign17650_e25272_d_n11;
        locals.var_lch_dn12 = assign17650_e25272_d_n12;
        locals.var_lch_dn17 = assign17650_e25272_d_n17;
        locals.var_lch_rv = 0.0;

        let assign17670_e25281: f64 = if locals.var_lch < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard533 = assign17670_e25281;
        locals.var_guard533_rv = 0.0;

        let (assign17680_e25287, assign17680_e25287_d_n0, assign17680_e25287_d_n2, assign17680_e25287_d_n6, assign17680_e25287_d_n7, assign17680_e25287_d_n10, assign17680_e25287_d_n11, assign17680_e25287_d_n12, assign17680_e25287_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard533 != 0.0)) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12, locals.var_lch_dn17,)
    }
};
        locals.var_lch = assign17680_e25287;
        locals.var_lch_dn0 = assign17680_e25287_d_n0;
        locals.var_lch_dn2 = assign17680_e25287_d_n2;
        locals.var_lch_dn6 = assign17680_e25287_d_n6;
        locals.var_lch_dn7 = assign17680_e25287_d_n7;
        locals.var_lch_dn10 = assign17680_e25287_d_n10;
        locals.var_lch_dn11 = assign17680_e25287_d_n11;
        locals.var_lch_dn12 = assign17680_e25287_d_n12;
        locals.var_lch_dn17 = assign17680_e25287_d_n17;
        locals.var_lch_rv = 0.0;

        let (assign17690_e25294, assign17690_e25294_d_n0, assign17690_e25294_d_n2, assign17690_e25294_d_n6, assign17690_e25294_d_n7, assign17690_e25294_d_n10, assign17690_e25294_d_n11, assign17690_e25294_d_n12, assign17690_e25294_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17690_e25290: f64 = (-locals.var_weffcv_nf);
        let assign17690_e25292: f64 = (assign17690_e25290 * locals.var_leff_cv);
        (assign17690_e25292, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign17690_e25294;
        locals.var_t1_dn0 = assign17690_e25294_d_n0;
        locals.var_t1_dn2 = assign17690_e25294_d_n2;
        locals.var_t1_dn6 = assign17690_e25294_d_n6;
        locals.var_t1_dn7 = assign17690_e25294_d_n7;
        locals.var_t1_dn10 = assign17690_e25294_d_n10;
        locals.var_t1_dn11 = assign17690_e25294_d_n11;
        locals.var_t1_dn12 = assign17690_e25294_d_n12;
        locals.var_t1_dn17 = assign17690_e25294_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign17700_e25300, assign17700_e25300_d_n0, assign17700_e25300_d_n2, assign17700_e25300_d_n6, assign17700_e25300_d_n7, assign17700_e25300_d_n10, assign17700_e25300_d_n11, assign17700_e25300_d_n12, assign17700_e25300_d_n13, assign17700_e25300_d_n15, assign17700_e25300_d_n16, assign17700_e25300_d_n17, assign17700_e25300_d_n18,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17700_e25298: f64 = (locals.var_t1 * locals.var_qbu);
        (assign17700_e25298, ((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0)), ((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2)), ((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6)), ((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7)), ((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10)), ((locals.var_t1_dn11 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn11)), ((locals.var_t1_dn12 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t1_dn17 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign17700_e25300;
        locals.var_qb_dn0 = assign17700_e25300_d_n0;
        locals.var_qb_dn2 = assign17700_e25300_d_n2;
        locals.var_qb_dn6 = assign17700_e25300_d_n6;
        locals.var_qb_dn7 = assign17700_e25300_d_n7;
        locals.var_qb_dn10 = assign17700_e25300_d_n10;
        locals.var_qb_dn11 = assign17700_e25300_d_n11;
        locals.var_qb_dn12 = assign17700_e25300_d_n12;
        locals.var_qb_dn13 = assign17700_e25300_d_n13;
        locals.var_qb_dn15 = assign17700_e25300_d_n15;
        locals.var_qb_dn16 = assign17700_e25300_d_n16;
        locals.var_qb_dn17 = assign17700_e25300_d_n17;
        locals.var_qb_dn18 = assign17700_e25300_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign17710_e25306, assign17710_e25306_d_n0, assign17710_e25306_d_n2, assign17710_e25306_d_n6, assign17710_e25306_d_n7, assign17710_e25306_d_n10, assign17710_e25306_d_n11, assign17710_e25306_d_n12, assign17710_e25306_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17710_e25304: f64 = (locals.var_t1 * locals.var_qiu);
        (assign17710_e25304, ((locals.var_t1_dn0 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn0)), ((locals.var_t1_dn2 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn2)), ((locals.var_t1_dn6 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn6)), ((locals.var_t1_dn7 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn7)), ((locals.var_t1_dn10 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn10)), ((locals.var_t1_dn11 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn11)), ((locals.var_t1_dn12 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn12)), ((locals.var_t1_dn17 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn17)),)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign17710_e25306;
        locals.var_qi_dn0 = assign17710_e25306_d_n0;
        locals.var_qi_dn2 = assign17710_e25306_d_n2;
        locals.var_qi_dn6 = assign17710_e25306_d_n6;
        locals.var_qi_dn7 = assign17710_e25306_d_n7;
        locals.var_qi_dn10 = assign17710_e25306_d_n10;
        locals.var_qi_dn11 = assign17710_e25306_d_n11;
        locals.var_qi_dn12 = assign17710_e25306_d_n12;
        locals.var_qi_dn17 = assign17710_e25306_d_n17;
        locals.var_qi_rv = 0.0;

        let (assign17720_e25312, assign17720_e25312_d_n0, assign17720_e25312_d_n2, assign17720_e25312_d_n6, assign17720_e25312_d_n7, assign17720_e25312_d_n10, assign17720_e25312_d_n11, assign17720_e25312_d_n12, assign17720_e25312_d_n13, assign17720_e25312_d_n15, assign17720_e25312_d_n16, assign17720_e25312_d_n17, assign17720_e25312_d_n18,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17720_e25310: f64 = (locals.var_qi * locals.var_qdrat);
        (assign17720_e25310, ((locals.var_qi_dn0 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn0)), ((locals.var_qi_dn2 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn2)), ((locals.var_qi_dn6 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn6)), ((locals.var_qi_dn7 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn7)), ((locals.var_qi_dn10 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn10)), ((locals.var_qi_dn11 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn11)), ((locals.var_qi_dn12 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn12)), 0.0, 0.0, 0.0, ((locals.var_qi_dn17 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn17)), 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign17720_e25312;
        locals.var_qd_dn0 = assign17720_e25312_d_n0;
        locals.var_qd_dn2 = assign17720_e25312_d_n2;
        locals.var_qd_dn6 = assign17720_e25312_d_n6;
        locals.var_qd_dn7 = assign17720_e25312_d_n7;
        locals.var_qd_dn10 = assign17720_e25312_d_n10;
        locals.var_qd_dn11 = assign17720_e25312_d_n11;
        locals.var_qd_dn12 = assign17720_e25312_d_n12;
        locals.var_qd_dn13 = assign17720_e25312_d_n13;
        locals.var_qd_dn15 = assign17720_e25312_d_n15;
        locals.var_qd_dn16 = assign17720_e25312_d_n16;
        locals.var_qd_dn17 = assign17720_e25312_d_n17;
        locals.var_qd_dn18 = assign17720_e25312_d_n18;
        locals.var_qd_rv = 0.0;

        let assign17730_e25315: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard534 = assign17730_e25315;
        locals.var_guard534_rv = 0.0;

        let (assign17740_e25323, assign17740_e25323_d_n0, assign17740_e25323_d_n2, assign17740_e25323_d_n6, assign17740_e25323_d_n7, assign17740_e25323_d_n10, assign17740_e25323_d_n11, assign17740_e25323_d_n12, assign17740_e25323_d_n13, assign17740_e25323_d_n15, assign17740_e25323_d_n16, assign17740_e25323_d_n17, assign17740_e25323_d_n18,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard534 != 0.0)) {
        let assign17740_e25321: f64 = (locals.var_qb * 0.5);
        (assign17740_e25321, (locals.var_qb_dn0 * 0.5), (locals.var_qb_dn2 * 0.5), (locals.var_qb_dn6 * 0.5), (locals.var_qb_dn7 * 0.5), (locals.var_qb_dn10 * 0.5), (locals.var_qb_dn11 * 0.5), (locals.var_qb_dn12 * 0.5), (locals.var_qb_dn13 * 0.5), (locals.var_qb_dn15 * 0.5), (locals.var_qb_dn16 * 0.5), (locals.var_qb_dn17 * 0.5), (locals.var_qb_dn18 * 0.5),)
    } else {
        (locals.var_qd_fb, locals.var_qd_fb_dn0, locals.var_qd_fb_dn2, locals.var_qd_fb_dn6, locals.var_qd_fb_dn7, locals.var_qd_fb_dn10, locals.var_qd_fb_dn11, locals.var_qd_fb_dn12, locals.var_qd_fb_dn13, locals.var_qd_fb_dn15, locals.var_qd_fb_dn16, locals.var_qd_fb_dn17, locals.var_qd_fb_dn18,)
    }
};
        locals.var_qd_fb = assign17740_e25323;
        locals.var_qd_fb_dn0 = assign17740_e25323_d_n0;
        locals.var_qd_fb_dn2 = assign17740_e25323_d_n2;
        locals.var_qd_fb_dn6 = assign17740_e25323_d_n6;
        locals.var_qd_fb_dn7 = assign17740_e25323_d_n7;
        locals.var_qd_fb_dn10 = assign17740_e25323_d_n10;
        locals.var_qd_fb_dn11 = assign17740_e25323_d_n11;
        locals.var_qd_fb_dn12 = assign17740_e25323_d_n12;
        locals.var_qd_fb_dn13 = assign17740_e25323_d_n13;
        locals.var_qd_fb_dn15 = assign17740_e25323_d_n15;
        locals.var_qd_fb_dn16 = assign17740_e25323_d_n16;
        locals.var_qd_fb_dn17 = assign17740_e25323_d_n17;
        locals.var_qd_fb_dn18 = assign17740_e25323_d_n18;
        locals.var_qd_fb_rv = 0.0;

        let (assign17750_e25333, assign17750_e25333_d_n0, assign17750_e25333_d_n2, assign17750_e25333_d_n6, assign17750_e25333_d_n7, assign17750_e25333_d_n10, assign17750_e25333_d_n11, assign17750_e25333_d_n12, assign17750_e25333_d_n13, assign17750_e25333_d_n15, assign17750_e25333_d_n16, assign17750_e25333_d_n17, assign17750_e25333_d_n18,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard534 != 0.0)) {
        let assign17750_e25330: f64 = (1.0 - 0.5);
        let assign17750_e25331: f64 = (locals.var_qb * assign17750_e25330);
        (assign17750_e25331, (locals.var_qb_dn0 * assign17750_e25330), (locals.var_qb_dn2 * assign17750_e25330), (locals.var_qb_dn6 * assign17750_e25330), (locals.var_qb_dn7 * assign17750_e25330), (locals.var_qb_dn10 * assign17750_e25330), (locals.var_qb_dn11 * assign17750_e25330), (locals.var_qb_dn12 * assign17750_e25330), (locals.var_qb_dn13 * assign17750_e25330), (locals.var_qb_dn15 * assign17750_e25330), (locals.var_qb_dn16 * assign17750_e25330), (locals.var_qb_dn17 * assign17750_e25330), (locals.var_qb_dn18 * assign17750_e25330),)
    } else {
        (locals.var_qs_fb, locals.var_qs_fb_dn0, locals.var_qs_fb_dn2, locals.var_qs_fb_dn6, locals.var_qs_fb_dn7, locals.var_qs_fb_dn10, locals.var_qs_fb_dn11, locals.var_qs_fb_dn12, locals.var_qs_fb_dn13, locals.var_qs_fb_dn15, locals.var_qs_fb_dn16, locals.var_qs_fb_dn17, locals.var_qs_fb_dn18,)
    }
};
        locals.var_qs_fb = assign17750_e25333;
        locals.var_qs_fb_dn0 = assign17750_e25333_d_n0;
        locals.var_qs_fb_dn2 = assign17750_e25333_d_n2;
        locals.var_qs_fb_dn6 = assign17750_e25333_d_n6;
        locals.var_qs_fb_dn7 = assign17750_e25333_d_n7;
        locals.var_qs_fb_dn10 = assign17750_e25333_d_n10;
        locals.var_qs_fb_dn11 = assign17750_e25333_d_n11;
        locals.var_qs_fb_dn12 = assign17750_e25333_d_n12;
        locals.var_qs_fb_dn13 = assign17750_e25333_d_n13;
        locals.var_qs_fb_dn15 = assign17750_e25333_d_n15;
        locals.var_qs_fb_dn16 = assign17750_e25333_d_n16;
        locals.var_qs_fb_dn17 = assign17750_e25333_d_n17;
        locals.var_qs_fb_dn18 = assign17750_e25333_d_n18;
        locals.var_qs_fb_rv = 0.0;

        let (assign17760_e25347, assign17760_e25347_d_n0, assign17760_e25347_d_n2, assign17760_e25347_d_n6, assign17760_e25347_d_n7, assign17760_e25347_d_n10, assign17760_e25347_d_n11, assign17760_e25347_d_n12, assign17760_e25347_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard534 != 0.0)) {
        let assign17760_e25340: f64 = (locals.var_q_s0_bulk + locals.var_q_sl_bulk);
        let assign17760_e25341: f64 = (0.5 * assign17760_e25340);
        let assign17760_e25343: f64 = (assign17760_e25341 * locals.var_leff_cv);
        let assign17760_e25345: f64 = (assign17760_e25343 * locals.var_weffcv_nf);
        (assign17760_e25345, (((0.5 * (locals.var_q_s0_bulk_dn0 + locals.var_q_sl_bulk_dn0)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn2 + locals.var_q_sl_bulk_dn2)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn6 + locals.var_q_sl_bulk_dn6)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn7 + locals.var_q_sl_bulk_dn7)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn10 + locals.var_q_sl_bulk_dn10)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn11 + locals.var_q_sl_bulk_dn11)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn12 + locals.var_q_sl_bulk_dn12)) * locals.var_leff_cv) * locals.var_weffcv_nf), (((0.5 * (locals.var_q_s0_bulk_dn17 + locals.var_q_sl_bulk_dn17)) * locals.var_leff_cv) * locals.var_weffcv_nf),)
    } else {
        (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12, locals.var_qsub_dn17,)
    }
};
        locals.var_qsub = assign17760_e25347;
        locals.var_qsub_dn0 = assign17760_e25347_d_n0;
        locals.var_qsub_dn2 = assign17760_e25347_d_n2;
        locals.var_qsub_dn6 = assign17760_e25347_d_n6;
        locals.var_qsub_dn7 = assign17760_e25347_d_n7;
        locals.var_qsub_dn10 = assign17760_e25347_d_n10;
        locals.var_qsub_dn11 = assign17760_e25347_d_n11;
        locals.var_qsub_dn12 = assign17760_e25347_d_n12;
        locals.var_qsub_dn17 = assign17760_e25347_d_n17;
        locals.var_qsub_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17770_e25355, assign17770_e25355_d_n0, assign17770_e25355_d_n2, assign17770_e25355_d_n6, assign17770_e25355_d_n7, assign17770_e25355_d_n10, assign17770_e25355_d_n11, assign17770_e25355_d_n12, assign17770_e25355_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17770_e25351: f64 = (locals.var_vds - locals.var_pds);
        let assign17770_e25353: f64 = (assign17770_e25351 / 2.0);
        (assign17770_e25353, ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0), ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0), ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0), ((locals.var_vds_dn7 - locals.var_pds_dn7) / 2.0), ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0), ((locals.var_vds_dn11 - locals.var_pds_dn11) / 2.0), ((locals.var_vds_dn12 - locals.var_pds_dn12) / 2.0), ((locals.var_vds_dn17 - locals.var_pds_dn17) / 2.0),)
    } else {
        (locals.var_t1__blk535, locals.var_t1__blk535_dn0, locals.var_t1__blk535_dn2, locals.var_t1__blk535_dn6, locals.var_t1__blk535_dn7, locals.var_t1__blk535_dn10, locals.var_t1__blk535_dn11, locals.var_t1__blk535_dn12, locals.var_t1__blk535_dn17,)
    }
};
        locals.var_t1__blk535 = assign17770_e25355;
        locals.var_t1__blk535_dn0 = assign17770_e25355_d_n0;
        locals.var_t1__blk535_dn2 = assign17770_e25355_d_n2;
        locals.var_t1__blk535_dn6 = assign17770_e25355_d_n6;
        locals.var_t1__blk535_dn7 = assign17770_e25355_d_n7;
        locals.var_t1__blk535_dn10 = assign17770_e25355_d_n10;
        locals.var_t1__blk535_dn11 = assign17770_e25355_d_n11;
        locals.var_t1__blk535_dn12 = assign17770_e25355_d_n12;
        locals.var_t1__blk535_dn17 = assign17770_e25355_d_n17;
        locals.var_t1__blk535_rv = 0.0;

        let (assign17780_e25363, assign17780_e25363_d_n0, assign17780_e25363_d_n2, assign17780_e25363_d_n6, assign17780_e25363_d_n7, assign17780_e25363_d_n10, assign17780_e25363_d_n11, assign17780_e25363_d_n12, assign17780_e25363_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17780_e25359: f64 = (2.0 * locals.var_t1__blk535);
        let assign17780_e25361: f64 = (assign17780_e25359 / p.p227);
        (assign17780_e25361, ((2.0 * locals.var_t1__blk535_dn0) / p.p227), ((2.0 * locals.var_t1__blk535_dn2) / p.p227), ((2.0 * locals.var_t1__blk535_dn6) / p.p227), ((2.0 * locals.var_t1__blk535_dn7) / p.p227), ((2.0 * locals.var_t1__blk535_dn10) / p.p227), ((2.0 * locals.var_t1__blk535_dn11) / p.p227), ((2.0 * locals.var_t1__blk535_dn12) / p.p227), ((2.0 * locals.var_t1__blk535_dn17) / p.p227),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17780_e25363;
        locals.var_tmf1_dn0 = assign17780_e25363_d_n0;
        locals.var_tmf1_dn2 = assign17780_e25363_d_n2;
        locals.var_tmf1_dn6 = assign17780_e25363_d_n6;
        locals.var_tmf1_dn7 = assign17780_e25363_d_n7;
        locals.var_tmf1_dn10 = assign17780_e25363_d_n10;
        locals.var_tmf1_dn11 = assign17780_e25363_d_n11;
        locals.var_tmf1_dn12 = assign17780_e25363_d_n12;
        locals.var_tmf1_dn17 = assign17780_e25363_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign17790_e25403, assign17790_e25403_d_n0, assign17790_e25403_d_n2, assign17790_e25403_d_n6, assign17790_e25403_d_n7, assign17790_e25403_d_n10, assign17790_e25403_d_n11, assign17790_e25403_d_n12, assign17790_e25403_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17790_e25369: f64 = (1.0 / 2.0);
        let assign17790_e25373: f64 = (1.0 / 6.0);
        let assign17790_e25377: f64 = (1.0 / 24.0);
        let assign17790_e25381: f64 = (1.0 / 120.0);
        let assign17790_e25385: f64 = (1.0 / 720.0);
        let assign17790_e25389: f64 = (1.0 / 5040.0);
        let assign17790_e25390: f64 = (locals.var_tmf1 * assign17790_e25389);
        let assign17790_e25391: f64 = (assign17790_e25385 + assign17790_e25390);
        let assign17790_e25392: f64 = (locals.var_tmf1 * assign17790_e25391);
        let assign17790_e25393: f64 = (assign17790_e25381 + assign17790_e25392);
        let assign17790_e25394: f64 = (locals.var_tmf1 * assign17790_e25393);
        let assign17790_e25395: f64 = (assign17790_e25377 + assign17790_e25394);
        let assign17790_e25396: f64 = (locals.var_tmf1 * assign17790_e25395);
        let assign17790_e25397: f64 = (assign17790_e25373 + assign17790_e25396);
        let assign17790_e25398: f64 = (locals.var_tmf1 * assign17790_e25397);
        let assign17790_e25399: f64 = (assign17790_e25369 + assign17790_e25398);
        let assign17790_e25400: f64 = (locals.var_tmf1 * assign17790_e25399);
        let assign17790_e25401: f64 = (1.0 + assign17790_e25400);
        (assign17790_e25401, ((locals.var_tmf1_dn0 * assign17790_e25399) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17790_e25397) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17790_e25395) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17790_e25393) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign17790_e25391) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign17790_e25389))))))))))), ((locals.var_tmf1_dn2 * assign17790_e25399) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17790_e25397) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17790_e25395) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17790_e25393) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign17790_e25391) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign17790_e25389))))))))))), ((locals.var_tmf1_dn6 * assign17790_e25399) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17790_e25397) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17790_e25395) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17790_e25393) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign17790_e25391) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign17790_e25389))))))))))), ((locals.var_tmf1_dn7 * assign17790_e25399) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17790_e25397) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17790_e25395) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17790_e25393) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign17790_e25391) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign17790_e25389))))))))))), ((locals.var_tmf1_dn10 * assign17790_e25399) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17790_e25397) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17790_e25395) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17790_e25393) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign17790_e25391) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign17790_e25389))))))))))), ((locals.var_tmf1_dn11 * assign17790_e25399) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17790_e25397) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17790_e25395) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17790_e25393) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign17790_e25391) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign17790_e25389))))))))))), ((locals.var_tmf1_dn12 * assign17790_e25399) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17790_e25397) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17790_e25395) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17790_e25393) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign17790_e25391) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign17790_e25389))))))))))), ((locals.var_tmf1_dn17 * assign17790_e25399) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17790_e25397) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17790_e25395) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17790_e25393) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign17790_e25391) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign17790_e25389))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign17790_e25403;
        locals.var_tmf2_dn0 = assign17790_e25403_d_n0;
        locals.var_tmf2_dn2 = assign17790_e25403_d_n2;
        locals.var_tmf2_dn6 = assign17790_e25403_d_n6;
        locals.var_tmf2_dn7 = assign17790_e25403_d_n7;
        locals.var_tmf2_dn10 = assign17790_e25403_d_n10;
        locals.var_tmf2_dn11 = assign17790_e25403_d_n11;
        locals.var_tmf2_dn12 = assign17790_e25403_d_n12;
        locals.var_tmf2_dn17 = assign17790_e25403_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign17800_e25409, assign17800_e25409_d_n0, assign17800_e25409_d_n2, assign17800_e25409_d_n6, assign17800_e25409_d_n7, assign17800_e25409_d_n10, assign17800_e25409_d_n11, assign17800_e25409_d_n12, assign17800_e25409_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17800_e25407: f64 = (p.p227 / locals.var_tmf2);
        (assign17800_e25407, (-((p.p227 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p227 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn12, locals.var_pzadd_dn17,)
    }
};
        locals.var_pzadd = assign17800_e25409;
        locals.var_pzadd_dn0 = assign17800_e25409_d_n0;
        locals.var_pzadd_dn2 = assign17800_e25409_d_n2;
        locals.var_pzadd_dn6 = assign17800_e25409_d_n6;
        locals.var_pzadd_dn7 = assign17800_e25409_d_n7;
        locals.var_pzadd_dn10 = assign17800_e25409_d_n10;
        locals.var_pzadd_dn11 = assign17800_e25409_d_n11;
        locals.var_pzadd_dn12 = assign17800_e25409_d_n12;
        locals.var_pzadd_dn17 = assign17800_e25409_d_n17;
        locals.var_pzadd_rv = 0.0;

        let assign17810_e25413: f64 = (10.0 * 2.220446049250313e-16);
        let assign17810_e25414: f64 = if locals.var_pzadd < assign17810_e25413 { 1.0 } else { 0.0 };
        locals.var_guard536 = assign17810_e25414;
        locals.var_guard536_rv = 0.0;

        let (assign17820_e25422, assign17820_e25422_d_n0, assign17820_e25422_d_n2, assign17820_e25422_d_n6, assign17820_e25422_d_n7, assign17820_e25422_d_n10, assign17820_e25422_d_n11, assign17820_e25422_d_n12, assign17820_e25422_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard536 != 0.0)) {
        let assign17820_e25420: f64 = (10.0 * 2.220446049250313e-16);
        (assign17820_e25420, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn12, locals.var_pzadd_dn17,)
    }
};
        locals.var_pzadd = assign17820_e25422;
        locals.var_pzadd_dn0 = assign17820_e25422_d_n0;
        locals.var_pzadd_dn2 = assign17820_e25422_d_n2;
        locals.var_pzadd_dn6 = assign17820_e25422_d_n6;
        locals.var_pzadd_dn7 = assign17820_e25422_d_n7;
        locals.var_pzadd_dn10 = assign17820_e25422_d_n10;
        locals.var_pzadd_dn11 = assign17820_e25422_d_n11;
        locals.var_pzadd_dn12 = assign17820_e25422_d_n12;
        locals.var_pzadd_dn17 = assign17820_e25422_d_n17;
        locals.var_pzadd_rv = 0.0;

        let (assign17830_e25428, assign17830_e25428_d_n0, assign17830_e25428_d_n2, assign17830_e25428_d_n6, assign17830_e25428_d_n7, assign17830_e25428_d_n10, assign17830_e25428_d_n11, assign17830_e25428_d_n12, assign17830_e25428_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17830_e25426: f64 = (locals.var_ps0 + locals.var_pzadd);
        (assign17830_e25426, (locals.var_ps0_dn0 + locals.var_pzadd_dn0), (locals.var_ps0_dn2 + locals.var_pzadd_dn2), (locals.var_ps0_dn6 + locals.var_pzadd_dn6), (locals.var_ps0_dn7 + locals.var_pzadd_dn7), (locals.var_ps0_dn10 + locals.var_pzadd_dn10), (locals.var_ps0_dn11 + locals.var_pzadd_dn11), (locals.var_ps0_dn12 + locals.var_pzadd_dn12), (locals.var_ps0_dn17 + locals.var_pzadd_dn17),)
    } else {
        (locals.var_ps0z, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn10, locals.var_ps0z_dn11, locals.var_ps0z_dn12, locals.var_ps0z_dn17,)
    }
};
        locals.var_ps0z = assign17830_e25428;
        locals.var_ps0z_dn0 = assign17830_e25428_d_n0;
        locals.var_ps0z_dn2 = assign17830_e25428_d_n2;
        locals.var_ps0z_dn6 = assign17830_e25428_d_n6;
        locals.var_ps0z_dn7 = assign17830_e25428_d_n7;
        locals.var_ps0z_dn10 = assign17830_e25428_d_n10;
        locals.var_ps0z_dn11 = assign17830_e25428_d_n11;
        locals.var_ps0z_dn12 = assign17830_e25428_d_n12;
        locals.var_ps0z_dn17 = assign17830_e25428_d_n17;
        locals.var_ps0z_rv = 0.0;

        let (assign17840_e25434,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17840_e25432: f64 = (1.034943e-10 / 100.0);
        (assign17840_e25432,)
    } else {
        (locals.var_cgs_esi,)
    }
};
        locals.var_cgs_esi = assign17840_e25434;
        locals.var_cgs_esi_rv = 0.0;

        let (assign17850_e25440, assign17850_e25440_d_n0, assign17850_e25440_d_n2, assign17850_e25440_d_n6, assign17850_e25440_d_n7, assign17850_e25440_d_n10, assign17850_e25440_d_n11, assign17850_e25440_d_n12, assign17850_e25440_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17850_e25438: f64 = (locals.var_qbu / 10000.0);
        (assign17850_e25438, (locals.var_qbu_dn0 / 10000.0), (locals.var_qbu_dn2 / 10000.0), (locals.var_qbu_dn6 / 10000.0), (locals.var_qbu_dn7 / 10000.0), (locals.var_qbu_dn10 / 10000.0), (locals.var_qbu_dn11 / 10000.0), (locals.var_qbu_dn12 / 10000.0), (locals.var_qbu_dn17 / 10000.0),)
    } else {
        (locals.var_cgs_qbu, locals.var_cgs_qbu_dn0, locals.var_cgs_qbu_dn2, locals.var_cgs_qbu_dn6, locals.var_cgs_qbu_dn7, locals.var_cgs_qbu_dn10, locals.var_cgs_qbu_dn11, locals.var_cgs_qbu_dn12, locals.var_cgs_qbu_dn17,)
    }
};
        locals.var_cgs_qbu = assign17850_e25440;
        locals.var_cgs_qbu_dn0 = assign17850_e25440_d_n0;
        locals.var_cgs_qbu_dn2 = assign17850_e25440_d_n2;
        locals.var_cgs_qbu_dn6 = assign17850_e25440_d_n6;
        locals.var_cgs_qbu_dn7 = assign17850_e25440_d_n7;
        locals.var_cgs_qbu_dn10 = assign17850_e25440_d_n10;
        locals.var_cgs_qbu_dn11 = assign17850_e25440_d_n11;
        locals.var_cgs_qbu_dn12 = assign17850_e25440_d_n12;
        locals.var_cgs_qbu_dn17 = assign17850_e25440_d_n17;
        locals.var_cgs_qbu_rv = 0.0;

        let (assign17860_e25446, assign17860_e25446_d_n0, assign17860_e25446_d_n2, assign17860_e25446_d_n6, assign17860_e25446_d_n7, assign17860_e25446_d_n10, assign17860_e25446_d_n11, assign17860_e25446_d_n12, assign17860_e25446_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17860_e25444: f64 = (locals.var_qiu / 10000.0);
        (assign17860_e25444, (locals.var_qiu_dn0 / 10000.0), (locals.var_qiu_dn2 / 10000.0), (locals.var_qiu_dn6 / 10000.0), (locals.var_qiu_dn7 / 10000.0), (locals.var_qiu_dn10 / 10000.0), (locals.var_qiu_dn11 / 10000.0), (locals.var_qiu_dn12 / 10000.0), (locals.var_qiu_dn17 / 10000.0),)
    } else {
        (locals.var_cgs_qiu, locals.var_cgs_qiu_dn0, locals.var_cgs_qiu_dn2, locals.var_cgs_qiu_dn6, locals.var_cgs_qiu_dn7, locals.var_cgs_qiu_dn10, locals.var_cgs_qiu_dn11, locals.var_cgs_qiu_dn12, locals.var_cgs_qiu_dn17,)
    }
};
        locals.var_cgs_qiu = assign17860_e25446;
        locals.var_cgs_qiu_dn0 = assign17860_e25446_d_n0;
        locals.var_cgs_qiu_dn2 = assign17860_e25446_d_n2;
        locals.var_cgs_qiu_dn6 = assign17860_e25446_d_n6;
        locals.var_cgs_qiu_dn7 = assign17860_e25446_d_n7;
        locals.var_cgs_qiu_dn10 = assign17860_e25446_d_n10;
        locals.var_cgs_qiu_dn11 = assign17860_e25446_d_n11;
        locals.var_cgs_qiu_dn12 = assign17860_e25446_d_n12;
        locals.var_cgs_qiu_dn17 = assign17860_e25446_d_n17;
        locals.var_cgs_qiu_rv = 0.0;

        let (assign17870_e25452, assign17870_e25452_d_n0, assign17870_e25452_d_n2, assign17870_e25452_d_n6, assign17870_e25452_d_n7, assign17870_e25452_d_n10, assign17870_e25452_d_n11, assign17870_e25452_d_n12, assign17870_e25452_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17870_e25450: f64 = (p.p92 / locals.var_cgs_esi);
        (assign17870_e25450, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk537, locals.var_t1__blk537_dn0, locals.var_t1__blk537_dn2, locals.var_t1__blk537_dn6, locals.var_t1__blk537_dn7, locals.var_t1__blk537_dn10, locals.var_t1__blk537_dn11, locals.var_t1__blk537_dn12, locals.var_t1__blk537_dn17,)
    }
};
        locals.var_t1__blk537 = assign17870_e25452;
        locals.var_t1__blk537_dn0 = assign17870_e25452_d_n0;
        locals.var_t1__blk537_dn2 = assign17870_e25452_d_n2;
        locals.var_t1__blk537_dn6 = assign17870_e25452_d_n6;
        locals.var_t1__blk537_dn7 = assign17870_e25452_d_n7;
        locals.var_t1__blk537_dn10 = assign17870_e25452_d_n10;
        locals.var_t1__blk537_dn11 = assign17870_e25452_d_n11;
        locals.var_t1__blk537_dn12 = assign17870_e25452_d_n12;
        locals.var_t1__blk537_dn17 = assign17870_e25452_d_n17;
        locals.var_t1__blk537_rv = 0.0;

        let (assign17880_e25458,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17880_e25456: f64 = (p.p93 / locals.var_cgs_esi);
        (assign17880_e25456,)
    } else {
        (locals.var_t2__blk538,)
    }
};
        locals.var_t2__blk538 = assign17880_e25458;
        locals.var_t2__blk538_rv = 0.0;

        let (assign17890_e25462, assign17890_e25462_d_n0, assign17890_e25462_d_n2, assign17890_e25462_d_n6, assign17890_e25462_d_n7, assign17890_e25462_d_n10, assign17890_e25462_d_n11, assign17890_e25462_d_n12, assign17890_e25462_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        (p.p94, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk539, locals.var_t0__blk539_dn0, locals.var_t0__blk539_dn2, locals.var_t0__blk539_dn6, locals.var_t0__blk539_dn7, locals.var_t0__blk539_dn10, locals.var_t0__blk539_dn11, locals.var_t0__blk539_dn12, locals.var_t0__blk539_dn17,)
    }
};
        locals.var_t0__blk539 = assign17890_e25462;
        locals.var_t0__blk539_dn0 = assign17890_e25462_d_n0;
        locals.var_t0__blk539_dn2 = assign17890_e25462_d_n2;
        locals.var_t0__blk539_dn6 = assign17890_e25462_d_n6;
        locals.var_t0__blk539_dn7 = assign17890_e25462_d_n7;
        locals.var_t0__blk539_dn10 = assign17890_e25462_d_n10;
        locals.var_t0__blk539_dn11 = assign17890_e25462_d_n11;
        locals.var_t0__blk539_dn12 = assign17890_e25462_d_n12;
        locals.var_t0__blk539_dn17 = assign17890_e25462_d_n17;
        locals.var_t0__blk539_rv = 0.0;

        let (assign17900_e25472, assign17900_e25472_d_n0, assign17900_e25472_d_n2, assign17900_e25472_d_n6, assign17900_e25472_d_n7, assign17900_e25472_d_n10, assign17900_e25472_d_n11, assign17900_e25472_d_n12, assign17900_e25472_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17900_e25467: f64 = (locals.var_psl - locals.var_ps0);
        let assign17900_e25469: f64 = (assign17900_e25467 * locals.var_t0__blk539);
        let assign17900_e25470: f64 = (1.0 + assign17900_e25469);
        (assign17900_e25470, (((locals.var_psl_dn0 - locals.var_ps0_dn0) * locals.var_t0__blk539) + (assign17900_e25467 * locals.var_t0__blk539_dn0)), (((locals.var_psl_dn2 - locals.var_ps0_dn2) * locals.var_t0__blk539) + (assign17900_e25467 * locals.var_t0__blk539_dn2)), (((locals.var_psl_dn6 - locals.var_ps0_dn6) * locals.var_t0__blk539) + (assign17900_e25467 * locals.var_t0__blk539_dn6)), (((locals.var_psl_dn7 - locals.var_ps0_dn7) * locals.var_t0__blk539) + (assign17900_e25467 * locals.var_t0__blk539_dn7)), (((locals.var_psl_dn10 - locals.var_ps0_dn10) * locals.var_t0__blk539) + (assign17900_e25467 * locals.var_t0__blk539_dn10)), (((locals.var_psl_dn11 - locals.var_ps0_dn11) * locals.var_t0__blk539) + (assign17900_e25467 * locals.var_t0__blk539_dn11)), (((locals.var_psl_dn12 - locals.var_ps0_dn12) * locals.var_t0__blk539) + (assign17900_e25467 * locals.var_t0__blk539_dn12)), (((locals.var_psl_dn17 - locals.var_ps0_dn17) * locals.var_t0__blk539) + (assign17900_e25467 * locals.var_t0__blk539_dn17)),)
    } else {
        (locals.var_t4__blk540, locals.var_t4__blk540_dn0, locals.var_t4__blk540_dn2, locals.var_t4__blk540_dn6, locals.var_t4__blk540_dn7, locals.var_t4__blk540_dn10, locals.var_t4__blk540_dn11, locals.var_t4__blk540_dn12, locals.var_t4__blk540_dn17,)
    }
};
        locals.var_t4__blk540 = assign17900_e25472;
        locals.var_t4__blk540_dn0 = assign17900_e25472_d_n0;
        locals.var_t4__blk540_dn2 = assign17900_e25472_d_n2;
        locals.var_t4__blk540_dn6 = assign17900_e25472_d_n6;
        locals.var_t4__blk540_dn7 = assign17900_e25472_d_n7;
        locals.var_t4__blk540_dn10 = assign17900_e25472_d_n10;
        locals.var_t4__blk540_dn11 = assign17900_e25472_d_n11;
        locals.var_t4__blk540_dn12 = assign17900_e25472_d_n12;
        locals.var_t4__blk540_dn17 = assign17900_e25472_d_n17;
        locals.var_t4__blk540_rv = 0.0;

        let (assign17910_e25482, assign17910_e25482_d_n0, assign17910_e25482_d_n2, assign17910_e25482_d_n6, assign17910_e25482_d_n7, assign17910_e25482_d_n10, assign17910_e25482_d_n11, assign17910_e25482_d_n12, assign17910_e25482_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17910_e25476: f64 = (locals.var_t1__blk537 * locals.var_cgs_qbu);
        let assign17910_e25479: f64 = (locals.var_t2__blk538 * locals.var_cgs_qiu);
        let assign17910_e25480: f64 = (assign17910_e25476 + assign17910_e25479);
        (assign17910_e25480, (((locals.var_t1__blk537_dn0 * locals.var_cgs_qbu) + (locals.var_t1__blk537 * locals.var_cgs_qbu_dn0)) + (locals.var_t2__blk538 * locals.var_cgs_qiu_dn0)), (((locals.var_t1__blk537_dn2 * locals.var_cgs_qbu) + (locals.var_t1__blk537 * locals.var_cgs_qbu_dn2)) + (locals.var_t2__blk538 * locals.var_cgs_qiu_dn2)), (((locals.var_t1__blk537_dn6 * locals.var_cgs_qbu) + (locals.var_t1__blk537 * locals.var_cgs_qbu_dn6)) + (locals.var_t2__blk538 * locals.var_cgs_qiu_dn6)), (((locals.var_t1__blk537_dn7 * locals.var_cgs_qbu) + (locals.var_t1__blk537 * locals.var_cgs_qbu_dn7)) + (locals.var_t2__blk538 * locals.var_cgs_qiu_dn7)), (((locals.var_t1__blk537_dn10 * locals.var_cgs_qbu) + (locals.var_t1__blk537 * locals.var_cgs_qbu_dn10)) + (locals.var_t2__blk538 * locals.var_cgs_qiu_dn10)), (((locals.var_t1__blk537_dn11 * locals.var_cgs_qbu) + (locals.var_t1__blk537 * locals.var_cgs_qbu_dn11)) + (locals.var_t2__blk538 * locals.var_cgs_qiu_dn11)), (((locals.var_t1__blk537_dn12 * locals.var_cgs_qbu) + (locals.var_t1__blk537 * locals.var_cgs_qbu_dn12)) + (locals.var_t2__blk538 * locals.var_cgs_qiu_dn12)), (((locals.var_t1__blk537_dn17 * locals.var_cgs_qbu) + (locals.var_t1__blk537 * locals.var_cgs_qbu_dn17)) + (locals.var_t2__blk538 * locals.var_cgs_qiu_dn17)),)
    } else {
        (locals.var_t5__blk541, locals.var_t5__blk541_dn0, locals.var_t5__blk541_dn2, locals.var_t5__blk541_dn6, locals.var_t5__blk541_dn7, locals.var_t5__blk541_dn10, locals.var_t5__blk541_dn11, locals.var_t5__blk541_dn12, locals.var_t5__blk541_dn17,)
    }
};
        locals.var_t5__blk541 = assign17910_e25482;
        locals.var_t5__blk541_dn0 = assign17910_e25482_d_n0;
        locals.var_t5__blk541_dn2 = assign17910_e25482_d_n2;
        locals.var_t5__blk541_dn6 = assign17910_e25482_d_n6;
        locals.var_t5__blk541_dn7 = assign17910_e25482_d_n7;
        locals.var_t5__blk541_dn10 = assign17910_e25482_d_n10;
        locals.var_t5__blk541_dn11 = assign17910_e25482_d_n11;
        locals.var_t5__blk541_dn12 = assign17910_e25482_d_n12;
        locals.var_t5__blk541_dn17 = assign17910_e25482_d_n17;
        locals.var_t5__blk541_rv = 0.0;

        let (assign17920_e25488, assign17920_e25488_d_n0, assign17920_e25488_d_n2, assign17920_e25488_d_n6, assign17920_e25488_d_n7, assign17920_e25488_d_n10, assign17920_e25488_d_n11, assign17920_e25488_d_n12, assign17920_e25488_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17920_e25486: f64 = (locals.var_t5__blk541 / locals.var_t4__blk540);
        (assign17920_e25486, (((locals.var_t5__blk541_dn0 * locals.var_t4__blk540) - (locals.var_t5__blk541 * locals.var_t4__blk540_dn0)) / (locals.var_t4__blk540 * locals.var_t4__blk540)), (((locals.var_t5__blk541_dn2 * locals.var_t4__blk540) - (locals.var_t5__blk541 * locals.var_t4__blk540_dn2)) / (locals.var_t4__blk540 * locals.var_t4__blk540)), (((locals.var_t5__blk541_dn6 * locals.var_t4__blk540) - (locals.var_t5__blk541 * locals.var_t4__blk540_dn6)) / (locals.var_t4__blk540 * locals.var_t4__blk540)), (((locals.var_t5__blk541_dn7 * locals.var_t4__blk540) - (locals.var_t5__blk541 * locals.var_t4__blk540_dn7)) / (locals.var_t4__blk540 * locals.var_t4__blk540)), (((locals.var_t5__blk541_dn10 * locals.var_t4__blk540) - (locals.var_t5__blk541 * locals.var_t4__blk540_dn10)) / (locals.var_t4__blk540 * locals.var_t4__blk540)), (((locals.var_t5__blk541_dn11 * locals.var_t4__blk540) - (locals.var_t5__blk541 * locals.var_t4__blk540_dn11)) / (locals.var_t4__blk540 * locals.var_t4__blk540)), (((locals.var_t5__blk541_dn12 * locals.var_t4__blk540) - (locals.var_t5__blk541 * locals.var_t4__blk540_dn12)) / (locals.var_t4__blk540 * locals.var_t4__blk540)), (((locals.var_t5__blk541_dn17 * locals.var_t4__blk540) - (locals.var_t5__blk541 * locals.var_t4__blk540_dn17)) / (locals.var_t4__blk540 * locals.var_t4__blk540)),)
    } else {
        (locals.var_t3__blk542, locals.var_t3__blk542_dn0, locals.var_t3__blk542_dn2, locals.var_t3__blk542_dn6, locals.var_t3__blk542_dn7, locals.var_t3__blk542_dn10, locals.var_t3__blk542_dn11, locals.var_t3__blk542_dn12, locals.var_t3__blk542_dn17,)
    }
};
        locals.var_t3__blk542 = assign17920_e25488;
        locals.var_t3__blk542_dn0 = assign17920_e25488_d_n0;
        locals.var_t3__blk542_dn2 = assign17920_e25488_d_n2;
        locals.var_t3__blk542_dn6 = assign17920_e25488_d_n6;
        locals.var_t3__blk542_dn7 = assign17920_e25488_d_n7;
        locals.var_t3__blk542_dn10 = assign17920_e25488_d_n10;
        locals.var_t3__blk542_dn11 = assign17920_e25488_d_n11;
        locals.var_t3__blk542_dn12 = assign17920_e25488_d_n12;
        locals.var_t3__blk542_dn17 = assign17920_e25488_d_n17;
        locals.var_t3__blk542_rv = 0.0;

        let (assign17930_e25492, assign17930_e25492_d_n0, assign17930_e25492_d_n2, assign17930_e25492_d_n6, assign17930_e25492_d_n7, assign17930_e25492_d_n10, assign17930_e25492_d_n11, assign17930_e25492_d_n12, assign17930_e25492_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        (locals.var_t3__blk542, locals.var_t3__blk542_dn0, locals.var_t3__blk542_dn2, locals.var_t3__blk542_dn6, locals.var_t3__blk542_dn7, locals.var_t3__blk542_dn10, locals.var_t3__blk542_dn11, locals.var_t3__blk542_dn12, locals.var_t3__blk542_dn17,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn12, locals.var_eeff_dn17,)
    }
};
        locals.var_eeff = assign17930_e25492;
        locals.var_eeff_dn0 = assign17930_e25492_d_n0;
        locals.var_eeff_dn2 = assign17930_e25492_d_n2;
        locals.var_eeff_dn6 = assign17930_e25492_d_n6;
        locals.var_eeff_dn7 = assign17930_e25492_d_n7;
        locals.var_eeff_dn10 = assign17930_e25492_d_n10;
        locals.var_eeff_dn11 = assign17930_e25492_d_n11;
        locals.var_eeff_dn12 = assign17930_e25492_d_n12;
        locals.var_eeff_dn17 = assign17930_e25492_d_n17;
        locals.var_eeff_rv = 0.0;

        let (assign17940_e25505, assign17940_e25505_d_n0, assign17940_e25505_d_n2, assign17940_e25505_d_n6, assign17940_e25505_d_n7, assign17940_e25505_d_n10, assign17940_e25505_d_n11, assign17940_e25505_d_n12, assign17940_e25505_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17940_e25496: f64 = (locals.var_eeff * locals.var_eeff);
        let assign17940_e25499: f64 = (4.0 * 3000.0);
        let assign17940_e25501: f64 = (assign17940_e25499 * 3000.0);
        let assign17940_e25502: f64 = (assign17940_e25496 + assign17940_e25501);
        let assign17940_e25503: f64 = (assign17940_e25502).sqrt();
        (assign17940_e25503, (((locals.var_eeff_dn0 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn0)) / (2.0 * assign17940_e25503)), (((locals.var_eeff_dn2 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn2)) / (2.0 * assign17940_e25503)), (((locals.var_eeff_dn6 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn6)) / (2.0 * assign17940_e25503)), (((locals.var_eeff_dn7 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn7)) / (2.0 * assign17940_e25503)), (((locals.var_eeff_dn10 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn10)) / (2.0 * assign17940_e25503)), (((locals.var_eeff_dn11 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn11)) / (2.0 * assign17940_e25503)), (((locals.var_eeff_dn12 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn12)) / (2.0 * assign17940_e25503)), (((locals.var_eeff_dn17 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn17)) / (2.0 * assign17940_e25503)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign17940_e25505;
        locals.var_tmf1_dn0 = assign17940_e25505_d_n0;
        locals.var_tmf1_dn2 = assign17940_e25505_d_n2;
        locals.var_tmf1_dn6 = assign17940_e25505_d_n6;
        locals.var_tmf1_dn7 = assign17940_e25505_d_n7;
        locals.var_tmf1_dn10 = assign17940_e25505_d_n10;
        locals.var_tmf1_dn11 = assign17940_e25505_d_n11;
        locals.var_tmf1_dn12 = assign17940_e25505_d_n12;
        locals.var_tmf1_dn17 = assign17940_e25505_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign17950_e25517, assign17950_e25517_d_n0, assign17950_e25517_d_n2, assign17950_e25517_d_n6, assign17950_e25517_d_n7, assign17950_e25517_d_n10, assign17950_e25517_d_n11, assign17950_e25517_d_n12, assign17950_e25517_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17950_e25510: f64 = (locals.var_eeff + locals.var_tmf1);
        let assign17950_e25511: f64 = (0.5 * assign17950_e25510);
        let assign17950_e25514: f64 = (1e-10 * 3000.0);
        let assign17950_e25515: f64 = (assign17950_e25511 + assign17950_e25514);
        (assign17950_e25515, (0.5 * (locals.var_eeff_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_eeff_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_eeff_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_eeff_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_eeff_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_eeff_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_eeff_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_eeff_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t0__blk539, locals.var_t0__blk539_dn0, locals.var_t0__blk539_dn2, locals.var_t0__blk539_dn6, locals.var_t0__blk539_dn7, locals.var_t0__blk539_dn10, locals.var_t0__blk539_dn11, locals.var_t0__blk539_dn12, locals.var_t0__blk539_dn17,)
    }
};
        locals.var_t0__blk539 = assign17950_e25517;
        locals.var_t0__blk539_dn0 = assign17950_e25517_d_n0;
        locals.var_t0__blk539_dn2 = assign17950_e25517_d_n2;
        locals.var_t0__blk539_dn6 = assign17950_e25517_d_n6;
        locals.var_t0__blk539_dn7 = assign17950_e25517_d_n7;
        locals.var_t0__blk539_dn10 = assign17950_e25517_d_n10;
        locals.var_t0__blk539_dn11 = assign17950_e25517_d_n11;
        locals.var_t0__blk539_dn12 = assign17950_e25517_d_n12;
        locals.var_t0__blk539_dn17 = assign17950_e25517_d_n17;
        locals.var_t0__blk539_rv = 0.0;

        let assign17960_e25520: f64 = if locals.var_t0__blk539 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign17960_e25520;
        locals.var_guard549_rv = 0.0;

        let (assign17970_e25526, assign17970_e25526_d_n0, assign17970_e25526_d_n2, assign17970_e25526_d_n6, assign17970_e25526_d_n7, assign17970_e25526_d_n10, assign17970_e25526_d_n11, assign17970_e25526_d_n12, assign17970_e25526_d_n17,) = {
    if ((locals.var_guard509 != 0.0) && (locals.var_guard549 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk539, locals.var_t0__blk539_dn0, locals.var_t0__blk539_dn2, locals.var_t0__blk539_dn6, locals.var_t0__blk539_dn7, locals.var_t0__blk539_dn10, locals.var_t0__blk539_dn11, locals.var_t0__blk539_dn12, locals.var_t0__blk539_dn17,)
    }
};
        locals.var_t0__blk539 = assign17970_e25526;
        locals.var_t0__blk539_dn0 = assign17970_e25526_d_n0;
        locals.var_t0__blk539_dn2 = assign17970_e25526_d_n2;
        locals.var_t0__blk539_dn6 = assign17970_e25526_d_n6;
        locals.var_t0__blk539_dn7 = assign17970_e25526_d_n7;
        locals.var_t0__blk539_dn10 = assign17970_e25526_d_n10;
        locals.var_t0__blk539_dn11 = assign17970_e25526_d_n11;
        locals.var_t0__blk539_dn12 = assign17970_e25526_d_n12;
        locals.var_t0__blk539_dn17 = assign17970_e25526_d_n17;
        locals.var_t0__blk539_rv = 0.0;

        let (assign17980_e25534, assign17980_e25534_d_n0, assign17980_e25534_d_n2, assign17980_e25534_d_n6, assign17980_e25534_d_n7, assign17980_e25534_d_n10, assign17980_e25534_d_n11, assign17980_e25534_d_n12, assign17980_e25534_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17980_e25531: f64 = (p.p97 - 1.0);
        let assign17980_e25532: f64 = (locals.var_t0__blk539).powf(assign17980_e25531);
        (assign17980_e25532, if 0.0 == 0.0 && ((assign17980_e25531) as f64).is_finite() && ((assign17980_e25531) as f64).fract() == 0.0 { if assign17980_e25531 == 0.0 { 0.0 } else { (assign17980_e25531 * ((locals.var_t0__blk539).powf(assign17980_e25531 - 1.0) * locals.var_t0__blk539_dn0)) } } else { (assign17980_e25532 * (assign17980_e25531 * (locals.var_t0__blk539_dn0 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign17980_e25531) as f64).is_finite() && ((assign17980_e25531) as f64).fract() == 0.0 { if assign17980_e25531 == 0.0 { 0.0 } else { (assign17980_e25531 * ((locals.var_t0__blk539).powf(assign17980_e25531 - 1.0) * locals.var_t0__blk539_dn2)) } } else { (assign17980_e25532 * (assign17980_e25531 * (locals.var_t0__blk539_dn2 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign17980_e25531) as f64).is_finite() && ((assign17980_e25531) as f64).fract() == 0.0 { if assign17980_e25531 == 0.0 { 0.0 } else { (assign17980_e25531 * ((locals.var_t0__blk539).powf(assign17980_e25531 - 1.0) * locals.var_t0__blk539_dn6)) } } else { (assign17980_e25532 * (assign17980_e25531 * (locals.var_t0__blk539_dn6 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign17980_e25531) as f64).is_finite() && ((assign17980_e25531) as f64).fract() == 0.0 { if assign17980_e25531 == 0.0 { 0.0 } else { (assign17980_e25531 * ((locals.var_t0__blk539).powf(assign17980_e25531 - 1.0) * locals.var_t0__blk539_dn7)) } } else { (assign17980_e25532 * (assign17980_e25531 * (locals.var_t0__blk539_dn7 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign17980_e25531) as f64).is_finite() && ((assign17980_e25531) as f64).fract() == 0.0 { if assign17980_e25531 == 0.0 { 0.0 } else { (assign17980_e25531 * ((locals.var_t0__blk539).powf(assign17980_e25531 - 1.0) * locals.var_t0__blk539_dn10)) } } else { (assign17980_e25532 * (assign17980_e25531 * (locals.var_t0__blk539_dn10 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign17980_e25531) as f64).is_finite() && ((assign17980_e25531) as f64).fract() == 0.0 { if assign17980_e25531 == 0.0 { 0.0 } else { (assign17980_e25531 * ((locals.var_t0__blk539).powf(assign17980_e25531 - 1.0) * locals.var_t0__blk539_dn11)) } } else { (assign17980_e25532 * (assign17980_e25531 * (locals.var_t0__blk539_dn11 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign17980_e25531) as f64).is_finite() && ((assign17980_e25531) as f64).fract() == 0.0 { if assign17980_e25531 == 0.0 { 0.0 } else { (assign17980_e25531 * ((locals.var_t0__blk539).powf(assign17980_e25531 - 1.0) * locals.var_t0__blk539_dn12)) } } else { (assign17980_e25532 * (assign17980_e25531 * (locals.var_t0__blk539_dn12 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign17980_e25531) as f64).is_finite() && ((assign17980_e25531) as f64).fract() == 0.0 { if assign17980_e25531 == 0.0 { 0.0 } else { (assign17980_e25531 * ((locals.var_t0__blk539).powf(assign17980_e25531 - 1.0) * locals.var_t0__blk539_dn17)) } } else { (assign17980_e25532 * (assign17980_e25531 * (locals.var_t0__blk539_dn17 / locals.var_t0__blk539))) },)
    } else {
        (locals.var_t5__blk541, locals.var_t5__blk541_dn0, locals.var_t5__blk541_dn2, locals.var_t5__blk541_dn6, locals.var_t5__blk541_dn7, locals.var_t5__blk541_dn10, locals.var_t5__blk541_dn11, locals.var_t5__blk541_dn12, locals.var_t5__blk541_dn17,)
    }
};
        locals.var_t5__blk541 = assign17980_e25534;
        locals.var_t5__blk541_dn0 = assign17980_e25534_d_n0;
        locals.var_t5__blk541_dn2 = assign17980_e25534_d_n2;
        locals.var_t5__blk541_dn6 = assign17980_e25534_d_n6;
        locals.var_t5__blk541_dn7 = assign17980_e25534_d_n7;
        locals.var_t5__blk541_dn10 = assign17980_e25534_d_n10;
        locals.var_t5__blk541_dn11 = assign17980_e25534_d_n11;
        locals.var_t5__blk541_dn12 = assign17980_e25534_d_n12;
        locals.var_t5__blk541_dn17 = assign17980_e25534_d_n17;
        locals.var_t5__blk541_rv = 0.0;

        let (assign17990_e25540, assign17990_e25540_d_n0, assign17990_e25540_d_n2, assign17990_e25540_d_n6, assign17990_e25540_d_n7, assign17990_e25540_d_n10, assign17990_e25540_d_n11, assign17990_e25540_d_n12, assign17990_e25540_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign17990_e25538: f64 = (locals.var_t5__blk541 * locals.var_t0__blk539);
        (assign17990_e25538, ((locals.var_t5__blk541_dn0 * locals.var_t0__blk539) + (locals.var_t5__blk541 * locals.var_t0__blk539_dn0)), ((locals.var_t5__blk541_dn2 * locals.var_t0__blk539) + (locals.var_t5__blk541 * locals.var_t0__blk539_dn2)), ((locals.var_t5__blk541_dn6 * locals.var_t0__blk539) + (locals.var_t5__blk541 * locals.var_t0__blk539_dn6)), ((locals.var_t5__blk541_dn7 * locals.var_t0__blk539) + (locals.var_t5__blk541 * locals.var_t0__blk539_dn7)), ((locals.var_t5__blk541_dn10 * locals.var_t0__blk539) + (locals.var_t5__blk541 * locals.var_t0__blk539_dn10)), ((locals.var_t5__blk541_dn11 * locals.var_t0__blk539) + (locals.var_t5__blk541 * locals.var_t0__blk539_dn11)), ((locals.var_t5__blk541_dn12 * locals.var_t0__blk539) + (locals.var_t5__blk541 * locals.var_t0__blk539_dn12)), ((locals.var_t5__blk541_dn17 * locals.var_t0__blk539) + (locals.var_t5__blk541 * locals.var_t0__blk539_dn17)),)
    } else {
        (locals.var_t8__blk543, locals.var_t8__blk543_dn0, locals.var_t8__blk543_dn2, locals.var_t8__blk543_dn6, locals.var_t8__blk543_dn7, locals.var_t8__blk543_dn10, locals.var_t8__blk543_dn11, locals.var_t8__blk543_dn12, locals.var_t8__blk543_dn17,)
    }
};
        locals.var_t8__blk543 = assign17990_e25540;
        locals.var_t8__blk543_dn0 = assign17990_e25540_d_n0;
        locals.var_t8__blk543_dn2 = assign17990_e25540_d_n2;
        locals.var_t8__blk543_dn6 = assign17990_e25540_d_n6;
        locals.var_t8__blk543_dn7 = assign17990_e25540_d_n7;
        locals.var_t8__blk543_dn10 = assign17990_e25540_d_n10;
        locals.var_t8__blk543_dn11 = assign17990_e25540_d_n11;
        locals.var_t8__blk543_dn12 = assign17990_e25540_d_n12;
        locals.var_t8__blk543_dn17 = assign17990_e25540_d_n17;
        locals.var_t8__blk543_rv = 0.0;

        let (assign18000_e25548, assign18000_e25548_d_n0, assign18000_e25548_d_n2, assign18000_e25548_d_n6, assign18000_e25548_d_n7, assign18000_e25548_d_n10, assign18000_e25548_d_n11, assign18000_e25548_d_n12, assign18000_e25548_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18000_e25545: f64 = (locals.var_muesr - 1.0);
        let assign18000_e25546: f64 = (locals.var_t0__blk539).powf(assign18000_e25545);
        (assign18000_e25546, if 0.0 == 0.0 && ((assign18000_e25545) as f64).is_finite() && ((assign18000_e25545) as f64).fract() == 0.0 { if assign18000_e25545 == 0.0 { 0.0 } else { (assign18000_e25545 * ((locals.var_t0__blk539).powf(assign18000_e25545 - 1.0) * locals.var_t0__blk539_dn0)) } } else { (assign18000_e25546 * (assign18000_e25545 * (locals.var_t0__blk539_dn0 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign18000_e25545) as f64).is_finite() && ((assign18000_e25545) as f64).fract() == 0.0 { if assign18000_e25545 == 0.0 { 0.0 } else { (assign18000_e25545 * ((locals.var_t0__blk539).powf(assign18000_e25545 - 1.0) * locals.var_t0__blk539_dn2)) } } else { (assign18000_e25546 * (assign18000_e25545 * (locals.var_t0__blk539_dn2 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign18000_e25545) as f64).is_finite() && ((assign18000_e25545) as f64).fract() == 0.0 { if assign18000_e25545 == 0.0 { 0.0 } else { (assign18000_e25545 * ((locals.var_t0__blk539).powf(assign18000_e25545 - 1.0) * locals.var_t0__blk539_dn6)) } } else { (assign18000_e25546 * (assign18000_e25545 * (locals.var_t0__blk539_dn6 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign18000_e25545) as f64).is_finite() && ((assign18000_e25545) as f64).fract() == 0.0 { if assign18000_e25545 == 0.0 { 0.0 } else { (assign18000_e25545 * ((locals.var_t0__blk539).powf(assign18000_e25545 - 1.0) * locals.var_t0__blk539_dn7)) } } else { (assign18000_e25546 * (assign18000_e25545 * (locals.var_t0__blk539_dn7 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign18000_e25545) as f64).is_finite() && ((assign18000_e25545) as f64).fract() == 0.0 { if assign18000_e25545 == 0.0 { 0.0 } else { (assign18000_e25545 * ((locals.var_t0__blk539).powf(assign18000_e25545 - 1.0) * locals.var_t0__blk539_dn10)) } } else { (assign18000_e25546 * (assign18000_e25545 * (locals.var_t0__blk539_dn10 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign18000_e25545) as f64).is_finite() && ((assign18000_e25545) as f64).fract() == 0.0 { if assign18000_e25545 == 0.0 { 0.0 } else { (assign18000_e25545 * ((locals.var_t0__blk539).powf(assign18000_e25545 - 1.0) * locals.var_t0__blk539_dn11)) } } else { (assign18000_e25546 * (assign18000_e25545 * (locals.var_t0__blk539_dn11 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign18000_e25545) as f64).is_finite() && ((assign18000_e25545) as f64).fract() == 0.0 { if assign18000_e25545 == 0.0 { 0.0 } else { (assign18000_e25545 * ((locals.var_t0__blk539).powf(assign18000_e25545 - 1.0) * locals.var_t0__blk539_dn12)) } } else { (assign18000_e25546 * (assign18000_e25545 * (locals.var_t0__blk539_dn12 / locals.var_t0__blk539))) }, if 0.0 == 0.0 && ((assign18000_e25545) as f64).is_finite() && ((assign18000_e25545) as f64).fract() == 0.0 { if assign18000_e25545 == 0.0 { 0.0 } else { (assign18000_e25545 * ((locals.var_t0__blk539).powf(assign18000_e25545 - 1.0) * locals.var_t0__blk539_dn17)) } } else { (assign18000_e25546 * (assign18000_e25545 * (locals.var_t0__blk539_dn17 / locals.var_t0__blk539))) },)
    } else {
        (locals.var_t7__blk544, locals.var_t7__blk544_dn0, locals.var_t7__blk544_dn2, locals.var_t7__blk544_dn6, locals.var_t7__blk544_dn7, locals.var_t7__blk544_dn10, locals.var_t7__blk544_dn11, locals.var_t7__blk544_dn12, locals.var_t7__blk544_dn17,)
    }
};
        locals.var_t7__blk544 = assign18000_e25548;
        locals.var_t7__blk544_dn0 = assign18000_e25548_d_n0;
        locals.var_t7__blk544_dn2 = assign18000_e25548_d_n2;
        locals.var_t7__blk544_dn6 = assign18000_e25548_d_n6;
        locals.var_t7__blk544_dn7 = assign18000_e25548_d_n7;
        locals.var_t7__blk544_dn10 = assign18000_e25548_d_n10;
        locals.var_t7__blk544_dn11 = assign18000_e25548_d_n11;
        locals.var_t7__blk544_dn12 = assign18000_e25548_d_n12;
        locals.var_t7__blk544_dn17 = assign18000_e25548_d_n17;
        locals.var_t7__blk544_rv = 0.0;

        let (assign18010_e25554, assign18010_e25554_d_n0, assign18010_e25554_d_n2, assign18010_e25554_d_n6, assign18010_e25554_d_n7, assign18010_e25554_d_n10, assign18010_e25554_d_n11, assign18010_e25554_d_n12, assign18010_e25554_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18010_e25552: f64 = (locals.var_t7__blk544 * locals.var_t0__blk539);
        (assign18010_e25552, ((locals.var_t7__blk544_dn0 * locals.var_t0__blk539) + (locals.var_t7__blk544 * locals.var_t0__blk539_dn0)), ((locals.var_t7__blk544_dn2 * locals.var_t0__blk539) + (locals.var_t7__blk544 * locals.var_t0__blk539_dn2)), ((locals.var_t7__blk544_dn6 * locals.var_t0__blk539) + (locals.var_t7__blk544 * locals.var_t0__blk539_dn6)), ((locals.var_t7__blk544_dn7 * locals.var_t0__blk539) + (locals.var_t7__blk544 * locals.var_t0__blk539_dn7)), ((locals.var_t7__blk544_dn10 * locals.var_t0__blk539) + (locals.var_t7__blk544 * locals.var_t0__blk539_dn10)), ((locals.var_t7__blk544_dn11 * locals.var_t0__blk539) + (locals.var_t7__blk544 * locals.var_t0__blk539_dn11)), ((locals.var_t7__blk544_dn12 * locals.var_t0__blk539) + (locals.var_t7__blk544 * locals.var_t0__blk539_dn12)), ((locals.var_t7__blk544_dn17 * locals.var_t0__blk539) + (locals.var_t7__blk544 * locals.var_t0__blk539_dn17)),)
    } else {
        (locals.var_t6__blk545, locals.var_t6__blk545_dn0, locals.var_t6__blk545_dn2, locals.var_t6__blk545_dn6, locals.var_t6__blk545_dn7, locals.var_t6__blk545_dn10, locals.var_t6__blk545_dn11, locals.var_t6__blk545_dn12, locals.var_t6__blk545_dn17,)
    }
};
        locals.var_t6__blk545 = assign18010_e25554;
        locals.var_t6__blk545_dn0 = assign18010_e25554_d_n0;
        locals.var_t6__blk545_dn2 = assign18010_e25554_d_n2;
        locals.var_t6__blk545_dn6 = assign18010_e25554_d_n6;
        locals.var_t6__blk545_dn7 = assign18010_e25554_d_n7;
        locals.var_t6__blk545_dn10 = assign18010_e25554_d_n10;
        locals.var_t6__blk545_dn11 = assign18010_e25554_d_n11;
        locals.var_t6__blk545_dn12 = assign18010_e25554_d_n12;
        locals.var_t6__blk545_dn17 = assign18010_e25554_d_n17;
        locals.var_t6__blk545_rv = 0.0;

        let (assign18020_e25560, assign18020_e25560_d_n0, assign18020_e25560_d_n2, assign18020_e25560_d_n6, assign18020_e25560_d_n7, assign18020_e25560_d_n10, assign18020_e25560_d_n11, assign18020_e25560_d_n12, assign18020_e25560_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18020_e25558: f64 = (locals.var_cgs_qiu / 1.6021918e-19);
        (assign18020_e25558, (locals.var_cgs_qiu_dn0 / 1.6021918e-19), (locals.var_cgs_qiu_dn2 / 1.6021918e-19), (locals.var_cgs_qiu_dn6 / 1.6021918e-19), (locals.var_cgs_qiu_dn7 / 1.6021918e-19), (locals.var_cgs_qiu_dn10 / 1.6021918e-19), (locals.var_cgs_qiu_dn11 / 1.6021918e-19), (locals.var_cgs_qiu_dn12 / 1.6021918e-19), (locals.var_cgs_qiu_dn17 / 1.6021918e-19),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn12, locals.var_rns_dn17,)
    }
};
        locals.var_rns = assign18020_e25560;
        locals.var_rns_dn0 = assign18020_e25560_d_n0;
        locals.var_rns_dn2 = assign18020_e25560_d_n2;
        locals.var_rns_dn6 = assign18020_e25560_d_n6;
        locals.var_rns_dn7 = assign18020_e25560_d_n7;
        locals.var_rns_dn10 = assign18020_e25560_d_n10;
        locals.var_rns_dn11 = assign18020_e25560_d_n11;
        locals.var_rns_dn12 = assign18020_e25560_d_n12;
        locals.var_rns_dn17 = assign18020_e25560_d_n17;
        locals.var_rns_rv = 0.0;

        let (assign18030_e25580, assign18030_e25580_d_n0, assign18030_e25580_d_n2, assign18030_e25580_d_n6, assign18030_e25580_d_n7, assign18030_e25580_d_n10, assign18030_e25580_d_n11, assign18030_e25580_d_n12, assign18030_e25580_d_n17,) = {
    if (locals.var_guard509 != 0.0) {
        let assign18030_e25566: f64 = (p.p96 * locals.var_rns);
        let assign18030_e25568: f64 = (assign18030_e25566 / 100000000000.0);
        let assign18030_e25569: f64 = (p.p95 + assign18030_e25568);
        let assign18030_e25570: f64 = (1.0 / assign18030_e25569);
        let assign18030_e25573: f64 = (locals.var_cgs_mphn0 * locals.var_t8__blk543);
        let assign18030_e25574: f64 = (assign18030_e25570 + assign18030_e25573);
        let assign18030_e25577: f64 = (locals.var_t6__blk545 / p.p106);
        let assign18030_e25578: f64 = (assign18030_e25574 + assign18030_e25577);
        (assign18030_e25578, (((-(((p.p96 * locals.var_rns_dn0) / 100000000000.0) / (assign18030_e25569 * assign18030_e25569))) + (locals.var_cgs_mphn0 * locals.var_t8__blk543_dn0)) + (locals.var_t6__blk545_dn0 / p.p106)), (((-(((p.p96 * locals.var_rns_dn2) / 100000000000.0) / (assign18030_e25569 * assign18030_e25569))) + (locals.var_cgs_mphn0 * locals.var_t8__blk543_dn2)) + (locals.var_t6__blk545_dn2 / p.p106)), (((-(((p.p96 * locals.var_rns_dn6) / 100000000000.0) / (assign18030_e25569 * assign18030_e25569))) + (locals.var_cgs_mphn0 * locals.var_t8__blk543_dn6)) + (locals.var_t6__blk545_dn6 / p.p106)), (((-(((p.p96 * locals.var_rns_dn7) / 100000000000.0) / (assign18030_e25569 * assign18030_e25569))) + (locals.var_cgs_mphn0 * locals.var_t8__blk543_dn7)) + (locals.var_t6__blk545_dn7 / p.p106)), (((-(((p.p96 * locals.var_rns_dn10) / 100000000000.0) / (assign18030_e25569 * assign18030_e25569))) + ((locals.var_cgs_mphn0_dn10 * locals.var_t8__blk543) + (locals.var_cgs_mphn0 * locals.var_t8__blk543_dn10))) + (locals.var_t6__blk545_dn10 / p.p106)), (((-(((p.p96 * locals.var_rns_dn11) / 100000000000.0) / (assign18030_e25569 * assign18030_e25569))) + (locals.var_cgs_mphn0 * locals.var_t8__blk543_dn11)) + (locals.var_t6__blk545_dn11 / p.p106)), (((-(((p.p96 * locals.var_rns_dn12) / 100000000000.0) / (assign18030_e25569 * assign18030_e25569))) + (locals.var_cgs_mphn0 * locals.var_t8__blk543_dn12)) + (locals.var_t6__blk545_dn12 / p.p106)), (((-(((p.p96 * locals.var_rns_dn17) / 100000000000.0) / (assign18030_e25569 * assign18030_e25569))) + (locals.var_cgs_mphn0 * locals.var_t8__blk543_dn17)) + (locals.var_t6__blk545_dn17 / p.p106)),)
    } else {
        (locals.var_t1__blk537, locals.var_t1__blk537_dn0, locals.var_t1__blk537_dn2, locals.var_t1__blk537_dn6, locals.var_t1__blk537_dn7, locals.var_t1__blk537_dn10, locals.var_t1__blk537_dn11, locals.var_t1__blk537_dn12, locals.var_t1__blk537_dn17,)
    }
};
        locals.var_t1__blk537 = assign18030_e25580;
        locals.var_t1__blk537_dn0 = assign18030_e25580_d_n0;
        locals.var_t1__blk537_dn2 = assign18030_e25580_d_n2;
        locals.var_t1__blk537_dn6 = assign18030_e25580_d_n6;
        locals.var_t1__blk537_dn7 = assign18030_e25580_d_n7;
        locals.var_t1__blk537_dn10 = assign18030_e25580_d_n10;
        locals.var_t1__blk537_dn11 = assign18030_e25580_d_n11;
        locals.var_t1__blk537_dn12 = assign18030_e25580_d_n12;
        locals.var_t1__blk537_dn17 = assign18030_e25580_d_n17;
        locals.var_t1__blk537_rv = 0.0;

    }
}
