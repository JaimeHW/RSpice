#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_128(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36030_e50781, assign36030_e50781_d_n0, assign36030_e50781_d_n2, assign36030_e50781_d_n6, assign36030_e50781_d_n7, assign36030_e50781_d_n10, assign36030_e50781_d_n11, assign36030_e50781_d_n12, assign36030_e50781_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36030_e50779: f64 = (locals.var_mu0__blk1191 * locals.var_edri__blk1193);
        (assign36030_e50779, ((locals.var_mu0__blk1191_dn0 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn0)), ((locals.var_mu0__blk1191_dn2 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn2)), ((locals.var_mu0__blk1191_dn6 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn6)), ((locals.var_mu0__blk1191_dn7 * locals.var_edri__blk1193) + (locals.var_mu0__blk1191 * locals.var_edri__blk1193_dn7)), (locals.var_mu0__blk1191_dn10 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn11 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn12 * locals.var_edri__blk1193), (locals.var_mu0__blk1191_dn17 * locals.var_edri__blk1193),)
    } else {
        (locals.var_vdri__blk1194, locals.var_vdri__blk1194_dn0, locals.var_vdri__blk1194_dn2, locals.var_vdri__blk1194_dn6, locals.var_vdri__blk1194_dn7, locals.var_vdri__blk1194_dn10, locals.var_vdri__blk1194_dn11, locals.var_vdri__blk1194_dn12, locals.var_vdri__blk1194_dn17,)
    }
};
        locals.var_vdri__blk1194 = assign36030_e50781;
        locals.var_vdri__blk1194_dn0 = assign36030_e50781_d_n0;
        locals.var_vdri__blk1194_dn2 = assign36030_e50781_d_n2;
        locals.var_vdri__blk1194_dn6 = assign36030_e50781_d_n6;
        locals.var_vdri__blk1194_dn7 = assign36030_e50781_d_n7;
        locals.var_vdri__blk1194_dn10 = assign36030_e50781_d_n10;
        locals.var_vdri__blk1194_dn11 = assign36030_e50781_d_n11;
        locals.var_vdri__blk1194_dn12 = assign36030_e50781_d_n12;
        locals.var_vdri__blk1194_dn17 = assign36030_e50781_d_n17;
        locals.var_vdri__blk1194_rv = 0.0;

        let assign36040_e50784: f64 = if locals.var_vrdr__blk1187 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign36040_e50784;
        locals.var_guard1200_rv = 0.0;

        let (assign36050_e50792, assign36050_e50792_d_n0, assign36050_e50792_d_n2, assign36050_e50792_d_n6, assign36050_e50792_d_n7, assign36050_e50792_d_n10, assign36050_e50792_d_n11, assign36050_e50792_d_n12, assign36050_e50792_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign36050_e50790: f64 = (locals.var_vdri__blk1194 / locals.var_vmaxe__blk1192);
        (assign36050_e50790, (((locals.var_vdri__blk1194_dn0 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn0)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn2 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn2)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn6 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn6)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn7 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn7)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn10 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn10)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn11 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn11)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn12 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn12)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), (((locals.var_vdri__blk1194_dn17 * locals.var_vmaxe__blk1192) - (locals.var_vdri__blk1194 * locals.var_vmaxe__blk1192_dn17)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36050_e50792;
        locals.var_t1_dn0 = assign36050_e50792_d_n0;
        locals.var_t1_dn2 = assign36050_e50792_d_n2;
        locals.var_t1_dn6 = assign36050_e50792_d_n6;
        locals.var_t1_dn7 = assign36050_e50792_d_n7;
        locals.var_t1_dn10 = assign36050_e50792_d_n10;
        locals.var_t1_dn11 = assign36050_e50792_d_n11;
        locals.var_t1_dn12 = assign36050_e50792_d_n12;
        locals.var_t1_dn17 = assign36050_e50792_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign36060_e50802, assign36060_e50802_d_n0, assign36060_e50802_d_n2, assign36060_e50802_d_n6, assign36060_e50802_d_n7, assign36060_e50802_d_n10, assign36060_e50802_d_n11, assign36060_e50802_d_n12, assign36060_e50802_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign36060_e50798: f64 = (-locals.var_vdri__blk1194);
        let assign36060_e50800: f64 = (assign36060_e50798 / locals.var_vmaxe__blk1192);
        (assign36060_e50800, ((((-locals.var_vdri__blk1194_dn0) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn0)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn2) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn2)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn6) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn6)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn7) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn7)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn10) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn10)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn11) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn11)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn12) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn12)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)), ((((-locals.var_vdri__blk1194_dn17) * locals.var_vmaxe__blk1192) - (assign36060_e50798 * locals.var_vmaxe__blk1192_dn17)) / (locals.var_vmaxe__blk1192 * locals.var_vmaxe__blk1192)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36060_e50802;
        locals.var_t1_dn0 = assign36060_e50802_d_n0;
        locals.var_t1_dn2 = assign36060_e50802_d_n2;
        locals.var_t1_dn6 = assign36060_e50802_d_n6;
        locals.var_t1_dn7 = assign36060_e50802_d_n7;
        locals.var_t1_dn10 = assign36060_e50802_d_n10;
        locals.var_t1_dn11 = assign36060_e50802_d_n11;
        locals.var_t1_dn12 = assign36060_e50802_d_n12;
        locals.var_t1_dn17 = assign36060_e50802_d_n17;
        locals.var_t1_rv = 0.0;

        let assign36070_e50806: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50807: f64 = (1.0 - assign36070_e50806);
        let assign36070_e50814: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50815: f64 = (1.0 + assign36070_e50814);
        let assign36070_e50817: f64 = if ((assign36070_e50807 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36070_e50815)) { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign36070_e50817;
        locals.var_guard1201_rv = 0.0;

        let (assign36080_e50823, assign36080_e50823_d_n0, assign36080_e50823_d_n2, assign36080_e50823_d_n6, assign36080_e50823_d_n7, assign36080_e50823_d_n10, assign36080_e50823_d_n11, assign36080_e50823_d_n12, assign36080_e50823_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1201 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36080_e50823;
        locals.var_t3_dn0 = assign36080_e50823_d_n0;
        locals.var_t3_dn2 = assign36080_e50823_d_n2;
        locals.var_t3_dn6 = assign36080_e50823_d_n6;
        locals.var_t3_dn7 = assign36080_e50823_d_n7;
        locals.var_t3_dn10 = assign36080_e50823_d_n10;
        locals.var_t3_dn11 = assign36080_e50823_d_n11;
        locals.var_t3_dn12 = assign36080_e50823_d_n12;
        locals.var_t3_dn17 = assign36080_e50823_d_n17;
        locals.var_t3_rv = 0.0;

        let assign36090_e50827: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50828: f64 = (2.0 - assign36090_e50827);
        let assign36090_e50835: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50836: f64 = (2.0 + assign36090_e50835);
        let assign36090_e50838: f64 = if ((assign36090_e50828 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36090_e50836)) { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign36090_e50838;
        locals.var_guard1202_rv = 0.0;

        let (assign36100_e50847, assign36100_e50847_d_n0, assign36100_e50847_d_n2, assign36100_e50847_d_n6, assign36100_e50847_d_n7, assign36100_e50847_d_n10, assign36100_e50847_d_n11, assign36100_e50847_d_n12, assign36100_e50847_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36100_e50847;
        locals.var_t3_dn0 = assign36100_e50847_d_n0;
        locals.var_t3_dn2 = assign36100_e50847_d_n2;
        locals.var_t3_dn6 = assign36100_e50847_d_n6;
        locals.var_t3_dn7 = assign36100_e50847_d_n7;
        locals.var_t3_dn10 = assign36100_e50847_d_n10;
        locals.var_t3_dn11 = assign36100_e50847_d_n11;
        locals.var_t3_dn12 = assign36100_e50847_d_n12;
        locals.var_t3_dn17 = assign36100_e50847_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign36110_e50861, assign36110_e50861_d_n0, assign36110_e50861_d_n2, assign36110_e50861_d_n6, assign36110_e50861_d_n7, assign36110_e50861_d_n10, assign36110_e50861_d_n11, assign36110_e50861_d_n12, assign36110_e50861_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign36110_e50858: f64 = (locals.var_rrdrbb__blk1185 - 1.0);
        let assign36110_e50859: f64 = (locals.var_t1).powf(assign36110_e50858);
        (assign36110_e50859, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn0)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn2)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn6)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn7)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb__blk1185_dn10 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn10)) } } else { (assign36110_e50859 * ((locals.var_rrdrbb__blk1185_dn10 * (locals.var_t1).ln()) + (assign36110_e50858 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn11)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn12)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((locals.var_t1).powf(assign36110_e50858 - 1.0) * locals.var_t1_dn17)) } } else { (assign36110_e50859 * (assign36110_e50858 * (locals.var_t1_dn17 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign36110_e50861;
        locals.var_t3_dn0 = assign36110_e50861_d_n0;
        locals.var_t3_dn2 = assign36110_e50861_d_n2;
        locals.var_t3_dn6 = assign36110_e50861_d_n6;
        locals.var_t3_dn7 = assign36110_e50861_d_n7;
        locals.var_t3_dn10 = assign36110_e50861_d_n10;
        locals.var_t3_dn11 = assign36110_e50861_d_n11;
        locals.var_t3_dn12 = assign36110_e50861_d_n12;
        locals.var_t3_dn17 = assign36110_e50861_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign36120_e50867, assign36120_e50867_d_n0, assign36120_e50867_d_n2, assign36120_e50867_d_n6, assign36120_e50867_d_n7, assign36120_e50867_d_n10, assign36120_e50867_d_n11, assign36120_e50867_d_n12, assign36120_e50867_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36120_e50865: f64 = (locals.var_t1 * locals.var_t3);
        (assign36120_e50865, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign36120_e50867;
        locals.var_t2_dn0 = assign36120_e50867_d_n0;
        locals.var_t2_dn2 = assign36120_e50867_d_n2;
        locals.var_t2_dn6 = assign36120_e50867_d_n6;
        locals.var_t2_dn7 = assign36120_e50867_d_n7;
        locals.var_t2_dn10 = assign36120_e50867_d_n10;
        locals.var_t2_dn11 = assign36120_e50867_d_n11;
        locals.var_t2_dn12 = assign36120_e50867_d_n12;
        locals.var_t2_dn17 = assign36120_e50867_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign36130_e50873, assign36130_e50873_d_n0, assign36130_e50873_d_n2, assign36130_e50873_d_n6, assign36130_e50873_d_n7, assign36130_e50873_d_n10, assign36130_e50873_d_n11, assign36130_e50873_d_n12, assign36130_e50873_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36130_e50871: f64 = (1.0 + locals.var_t2);
        (assign36130_e50871, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign36130_e50873;
        locals.var_t4_dn0 = assign36130_e50873_d_n0;
        locals.var_t4_dn2 = assign36130_e50873_d_n2;
        locals.var_t4_dn6 = assign36130_e50873_d_n6;
        locals.var_t4_dn7 = assign36130_e50873_d_n7;
        locals.var_t4_dn10 = assign36130_e50873_d_n10;
        locals.var_t4_dn11 = assign36130_e50873_d_n11;
        locals.var_t4_dn12 = assign36130_e50873_d_n12;
        locals.var_t4_dn17 = assign36130_e50873_d_n17;
        locals.var_t4_rv = 0.0;

        let assign36140_e50877: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50878: f64 = (1.0 - assign36140_e50877);
        let assign36140_e50885: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50886: f64 = (1.0 + assign36140_e50885);
        let assign36140_e50888: f64 = if ((assign36140_e50878 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36140_e50886)) { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign36140_e50888;
        locals.var_guard1203_rv = 0.0;

        let (assign36150_e50896, assign36150_e50896_d_n0, assign36150_e50896_d_n2, assign36150_e50896_d_n6, assign36150_e50896_d_n7, assign36150_e50896_d_n10, assign36150_e50896_d_n11, assign36150_e50896_d_n12, assign36150_e50896_d_n17,) = {
    if ((locals.var_guard1179 != 0.0) && (locals.var_guard1203 != 0.0)) {
        let assign36150_e50894: f64 = (1.0 / locals.var_t4);
        (assign36150_e50894, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36150_e50896;
        locals.var_t5_dn0 = assign36150_e50896_d_n0;
        locals.var_t5_dn2 = assign36150_e50896_d_n2;
        locals.var_t5_dn6 = assign36150_e50896_d_n6;
        locals.var_t5_dn7 = assign36150_e50896_d_n7;
        locals.var_t5_dn10 = assign36150_e50896_d_n10;
        locals.var_t5_dn11 = assign36150_e50896_d_n11;
        locals.var_t5_dn12 = assign36150_e50896_d_n12;
        locals.var_t5_dn17 = assign36150_e50896_d_n17;
        locals.var_t5_rv = 0.0;

        let assign36160_e50900: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50901: f64 = (2.0 - assign36160_e50900);
        let assign36160_e50908: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50909: f64 = (2.0 + assign36160_e50908);
        let assign36160_e50911: f64 = if ((assign36160_e50901 <= locals.var_rrdrbb__blk1185) && (locals.var_rrdrbb__blk1185 <= assign36160_e50909)) { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign36160_e50911;
        locals.var_guard1204_rv = 0.0;

        let (assign36170_e50923, assign36170_e50923_d_n0, assign36170_e50923_d_n2, assign36170_e50923_d_n6, assign36170_e50923_d_n7, assign36170_e50923_d_n10, assign36170_e50923_d_n11, assign36170_e50923_d_n12, assign36170_e50923_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign36170_e50920: f64 = (locals.var_t4).sqrt();
        let assign36170_e50921: f64 = (1.0 / assign36170_e50920);
        (assign36170_e50921, (-((locals.var_t4_dn0 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn2 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn6 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn7 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn10 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn11 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn12 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((locals.var_t4_dn17 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36170_e50923;
        locals.var_t5_dn0 = assign36170_e50923_d_n0;
        locals.var_t5_dn2 = assign36170_e50923_d_n2;
        locals.var_t5_dn6 = assign36170_e50923_d_n6;
        locals.var_t5_dn7 = assign36170_e50923_d_n7;
        locals.var_t5_dn10 = assign36170_e50923_d_n10;
        locals.var_t5_dn11 = assign36170_e50923_d_n11;
        locals.var_t5_dn12 = assign36170_e50923_d_n12;
        locals.var_t5_dn17 = assign36170_e50923_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign36180_e50940, assign36180_e50940_d_n0, assign36180_e50940_d_n2, assign36180_e50940_d_n6, assign36180_e50940_d_n7, assign36180_e50940_d_n10, assign36180_e50940_d_n11, assign36180_e50940_d_n12, assign36180_e50940_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign36180_e50933: f64 = (-1.0);
        let assign36180_e50935: f64 = (assign36180_e50933 / locals.var_rrdrbb__blk1185);
        let assign36180_e50937: f64 = (assign36180_e50935 - 1.0);
        let assign36180_e50938: f64 = (locals.var_t4).powf(assign36180_e50937);
        (assign36180_e50938, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn0)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn2)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn6)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn7)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign36180_e50933 * locals.var_rrdrbb__blk1185_dn10) / (locals.var_rrdrbb__blk1185 * locals.var_rrdrbb__blk1185))) == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn10)) } } else { (assign36180_e50938 * (((-((assign36180_e50933 * locals.var_rrdrbb__blk1185_dn10) / (locals.var_rrdrbb__blk1185 * locals.var_rrdrbb__blk1185))) * (locals.var_t4).ln()) + (assign36180_e50937 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn11)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn12)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((locals.var_t4).powf(assign36180_e50937 - 1.0) * locals.var_t4_dn17)) } } else { (assign36180_e50938 * (assign36180_e50937 * (locals.var_t4_dn17 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
        locals.var_t6 = assign36180_e50940;
        locals.var_t6_dn0 = assign36180_e50940_d_n0;
        locals.var_t6_dn2 = assign36180_e50940_d_n2;
        locals.var_t6_dn6 = assign36180_e50940_d_n6;
        locals.var_t6_dn7 = assign36180_e50940_d_n7;
        locals.var_t6_dn10 = assign36180_e50940_d_n10;
        locals.var_t6_dn11 = assign36180_e50940_d_n11;
        locals.var_t6_dn12 = assign36180_e50940_d_n12;
        locals.var_t6_dn17 = assign36180_e50940_d_n17;
        locals.var_t6_rv = 0.0;

        let (assign36190_e50952, assign36190_e50952_d_n0, assign36190_e50952_d_n2, assign36190_e50952_d_n6, assign36190_e50952_d_n7, assign36190_e50952_d_n10, assign36190_e50952_d_n11, assign36190_e50952_d_n12, assign36190_e50952_d_n17,) = {
    if (((locals.var_guard1179 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign36190_e50950: f64 = (locals.var_t4 * locals.var_t6);
        (assign36190_e50950, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign36190_e50952;
        locals.var_t5_dn0 = assign36190_e50952_d_n0;
        locals.var_t5_dn2 = assign36190_e50952_d_n2;
        locals.var_t5_dn6 = assign36190_e50952_d_n6;
        locals.var_t5_dn7 = assign36190_e50952_d_n7;
        locals.var_t5_dn10 = assign36190_e50952_d_n10;
        locals.var_t5_dn11 = assign36190_e50952_d_n11;
        locals.var_t5_dn12 = assign36190_e50952_d_n12;
        locals.var_t5_dn17 = assign36190_e50952_d_n17;
        locals.var_t5_rv = 0.0;

        let (assign36210_e50964, assign36210_e50964_d_n0, assign36210_e50964_d_n2, assign36210_e50964_d_n6, assign36210_e50964_d_n7, assign36210_e50964_d_n10, assign36210_e50964_d_n11, assign36210_e50964_d_n12, assign36210_e50964_d_n17,) = {
    if (locals.var_guard1179 != 0.0) {
        let assign36210_e50962: f64 = (1.6021918e-19 / locals.var_ldrifte__blk1189);
        (assign36210_e50962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign36210_e50964;
        locals.var_t1_dn0 = assign36210_e50964_d_n0;
        locals.var_t1_dn2 = assign36210_e50964_d_n2;
        locals.var_t1_dn6 = assign36210_e50964_d_n6;
        locals.var_t1_dn7 = assign36210_e50964_d_n7;
        locals.var_t1_dn10 = assign36210_e50964_d_n10;
        locals.var_t1_dn11 = assign36210_e50964_d_n11;
        locals.var_t1_dn12 = assign36210_e50964_d_n12;
        locals.var_t1_dn17 = assign36210_e50964_d_n17;
        locals.var_t1_rv = 0.0;

        let assign36330_e51038: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign36330_e51038;
        locals.var_guard1207_rv = 0.0;

        let (assign36380_e51081, assign36380_e51081_d_n0, assign36380_e51081_d_n2, assign36380_e51081_d_n6, assign36380_e51081_d_n7, assign36380_e51081_d_n10, assign36380_e51081_d_n11, assign36380_e51081_d_n12, assign36380_e51081_d_n17,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,) = {
            if (locals.var_mode == 1.0) {
                (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
            } else {
                let assign36380_e51078: f64 = (1.0 - locals.var_xd);
                (assign36380_e51078, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn12), (-locals.var_xd_dn17),)
            }
        };
        (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17,)
    }
};
        locals.var_qdrat = assign36380_e51081;
        locals.var_qdrat_dn0 = assign36380_e51081_d_n0;
        locals.var_qdrat_dn2 = assign36380_e51081_d_n2;
        locals.var_qdrat_dn6 = assign36380_e51081_d_n6;
        locals.var_qdrat_dn7 = assign36380_e51081_d_n7;
        locals.var_qdrat_dn10 = assign36380_e51081_d_n10;
        locals.var_qdrat_dn11 = assign36380_e51081_d_n11;
        locals.var_qdrat_dn12 = assign36380_e51081_d_n12;
        locals.var_qdrat_dn17 = assign36380_e51081_d_n17;
        locals.var_qdrat_rv = 0.0;

        let (assign36410_e51111, assign36410_e51111_d_n0, assign36410_e51111_d_n2, assign36410_e51111_d_n6, assign36410_e51111_d_n7, assign36410_e51111_d_n10, assign36410_e51111_d_n11, assign36410_e51111_d_n12, assign36410_e51111_d_n15, assign36410_e51111_d_n17, assign36410_e51111_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36410_e51107: f64 = (locals.var_qi_nqs * locals.var_qdrat);
        let assign36410_e51109: f64 = (assign36410_e51107 + locals.var_q_bt_se);
        (assign36410_e51109, ((locals.var_qi_nqs * locals.var_qdrat_dn0) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * locals.var_qdrat_dn2) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * locals.var_qdrat_dn6) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * locals.var_qdrat_dn7) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * locals.var_qdrat_dn10) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * locals.var_qdrat_dn11) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * locals.var_qdrat_dn12) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * locals.var_qdrat_dn17) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * locals.var_qdrat),)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36410_e51111;
        locals.var_qd_nqs_dn0 = assign36410_e51111_d_n0;
        locals.var_qd_nqs_dn2 = assign36410_e51111_d_n2;
        locals.var_qd_nqs_dn6 = assign36410_e51111_d_n6;
        locals.var_qd_nqs_dn7 = assign36410_e51111_d_n7;
        locals.var_qd_nqs_dn10 = assign36410_e51111_d_n10;
        locals.var_qd_nqs_dn11 = assign36410_e51111_d_n11;
        locals.var_qd_nqs_dn12 = assign36410_e51111_d_n12;
        locals.var_qd_nqs_dn15 = assign36410_e51111_d_n15;
        locals.var_qd_nqs_dn17 = assign36410_e51111_d_n17;
        locals.var_qd_nqs_dn18 = assign36410_e51111_d_n18;
        locals.var_qd_nqs_rv = 0.0;

        let (assign36420_e51123, assign36420_e51123_d_n0, assign36420_e51123_d_n2, assign36420_e51123_d_n6, assign36420_e51123_d_n7, assign36420_e51123_d_n10, assign36420_e51123_d_n11, assign36420_e51123_d_n12, assign36420_e51123_d_n16, assign36420_e51123_d_n17, assign36420_e51123_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36420_e51118: f64 = (1.0 - locals.var_qdrat);
        let assign36420_e51119: f64 = (locals.var_qi_nqs * assign36420_e51118);
        let assign36420_e51121: f64 = (assign36420_e51119 + locals.var_q_bt_se);
        (assign36420_e51121, ((locals.var_qi_nqs * (-locals.var_qdrat_dn0)) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * (-locals.var_qdrat_dn2)) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * (-locals.var_qdrat_dn6)) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * (-locals.var_qdrat_dn7)) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * (-locals.var_qdrat_dn10)) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * (-locals.var_qdrat_dn11)) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * (-locals.var_qdrat_dn12)) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * (-locals.var_qdrat_dn17)) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * assign36420_e51118),)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36420_e51123;
        locals.var_qs_nqs_dn0 = assign36420_e51123_d_n0;
        locals.var_qs_nqs_dn2 = assign36420_e51123_d_n2;
        locals.var_qs_nqs_dn6 = assign36420_e51123_d_n6;
        locals.var_qs_nqs_dn7 = assign36420_e51123_d_n7;
        locals.var_qs_nqs_dn10 = assign36420_e51123_d_n10;
        locals.var_qs_nqs_dn11 = assign36420_e51123_d_n11;
        locals.var_qs_nqs_dn12 = assign36420_e51123_d_n12;
        locals.var_qs_nqs_dn16 = assign36420_e51123_d_n16;
        locals.var_qs_nqs_dn17 = assign36420_e51123_d_n17;
        locals.var_qs_nqs_dn18 = assign36420_e51123_d_n18;
        locals.var_qs_nqs_rv = 0.0;

        let (assign36430_e51134, assign36430_e51134_d_n0, assign36430_e51134_d_n2, assign36430_e51134_d_n6, assign36430_e51134_d_n7, assign36430_e51134_d_n10, assign36430_e51134_d_n11, assign36430_e51134_d_n12, assign36430_e51134_d_n13, assign36430_e51134_d_n15, assign36430_e51134_d_n16, assign36430_e51134_d_n17, assign36430_e51134_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36430_e51128: f64 = (-locals.var_qi_nqs);
        let assign36430_e51130: f64 = (assign36430_e51128 - locals.var_qb_nqs);
        let assign36430_e51132: f64 = (assign36430_e51130 + locals.var_q_bt_ge);
        (assign36430_e51132, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, (-locals.var_qb_nqs_dn13), 0.0, 0.0, locals.var_q_bt_ge_dn17, (-locals.var_qi_nqs_dn18),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36430_e51134;
        locals.var_qg_nqs_dn0 = assign36430_e51134_d_n0;
        locals.var_qg_nqs_dn2 = assign36430_e51134_d_n2;
        locals.var_qg_nqs_dn6 = assign36430_e51134_d_n6;
        locals.var_qg_nqs_dn7 = assign36430_e51134_d_n7;
        locals.var_qg_nqs_dn10 = assign36430_e51134_d_n10;
        locals.var_qg_nqs_dn11 = assign36430_e51134_d_n11;
        locals.var_qg_nqs_dn12 = assign36430_e51134_d_n12;
        locals.var_qg_nqs_dn13 = assign36430_e51134_d_n13;
        locals.var_qg_nqs_dn15 = assign36430_e51134_d_n15;
        locals.var_qg_nqs_dn16 = assign36430_e51134_d_n16;
        locals.var_qg_nqs_dn17 = assign36430_e51134_d_n17;
        locals.var_qg_nqs_dn18 = assign36430_e51134_d_n18;
        locals.var_qg_nqs_rv = 0.0;

        let (assign36460_e51155, assign36460_e51155_d_n0, assign36460_e51155_d_n2, assign36460_e51155_d_n6, assign36460_e51155_d_n7, assign36460_e51155_d_n10, assign36460_e51155_d_n11, assign36460_e51155_d_n12, assign36460_e51155_d_n15, assign36460_e51155_d_n17, assign36460_e51155_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36460_e51155;
        locals.var_qd_nqs_dn0 = assign36460_e51155_d_n0;
        locals.var_qd_nqs_dn2 = assign36460_e51155_d_n2;
        locals.var_qd_nqs_dn6 = assign36460_e51155_d_n6;
        locals.var_qd_nqs_dn7 = assign36460_e51155_d_n7;
        locals.var_qd_nqs_dn10 = assign36460_e51155_d_n10;
        locals.var_qd_nqs_dn11 = assign36460_e51155_d_n11;
        locals.var_qd_nqs_dn12 = assign36460_e51155_d_n12;
        locals.var_qd_nqs_dn15 = assign36460_e51155_d_n15;
        locals.var_qd_nqs_dn17 = assign36460_e51155_d_n17;
        locals.var_qd_nqs_dn18 = assign36460_e51155_d_n18;
        locals.var_qd_nqs_rv = 0.0;

        let (assign36470_e51162, assign36470_e51162_d_n0, assign36470_e51162_d_n2, assign36470_e51162_d_n6, assign36470_e51162_d_n7, assign36470_e51162_d_n10, assign36470_e51162_d_n11, assign36470_e51162_d_n12, assign36470_e51162_d_n16, assign36470_e51162_d_n17, assign36470_e51162_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36470_e51162;
        locals.var_qs_nqs_dn0 = assign36470_e51162_d_n0;
        locals.var_qs_nqs_dn2 = assign36470_e51162_d_n2;
        locals.var_qs_nqs_dn6 = assign36470_e51162_d_n6;
        locals.var_qs_nqs_dn7 = assign36470_e51162_d_n7;
        locals.var_qs_nqs_dn10 = assign36470_e51162_d_n10;
        locals.var_qs_nqs_dn11 = assign36470_e51162_d_n11;
        locals.var_qs_nqs_dn12 = assign36470_e51162_d_n12;
        locals.var_qs_nqs_dn16 = assign36470_e51162_d_n16;
        locals.var_qs_nqs_dn17 = assign36470_e51162_d_n17;
        locals.var_qs_nqs_dn18 = assign36470_e51162_d_n18;
        locals.var_qs_nqs_rv = 0.0;

        let (assign36480_e51169, assign36480_e51169_d_n0, assign36480_e51169_d_n2, assign36480_e51169_d_n6, assign36480_e51169_d_n7, assign36480_e51169_d_n10, assign36480_e51169_d_n11, assign36480_e51169_d_n12, assign36480_e51169_d_n13, assign36480_e51169_d_n15, assign36480_e51169_d_n16, assign36480_e51169_d_n17, assign36480_e51169_d_n18,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36480_e51169;
        locals.var_qg_nqs_dn0 = assign36480_e51169_d_n0;
        locals.var_qg_nqs_dn2 = assign36480_e51169_d_n2;
        locals.var_qg_nqs_dn6 = assign36480_e51169_d_n6;
        locals.var_qg_nqs_dn7 = assign36480_e51169_d_n7;
        locals.var_qg_nqs_dn10 = assign36480_e51169_d_n10;
        locals.var_qg_nqs_dn11 = assign36480_e51169_d_n11;
        locals.var_qg_nqs_dn12 = assign36480_e51169_d_n12;
        locals.var_qg_nqs_dn13 = assign36480_e51169_d_n13;
        locals.var_qg_nqs_dn15 = assign36480_e51169_d_n15;
        locals.var_qg_nqs_dn16 = assign36480_e51169_d_n16;
        locals.var_qg_nqs_dn17 = assign36480_e51169_d_n17;
        locals.var_qg_nqs_dn18 = assign36480_e51169_d_n18;
        locals.var_qg_nqs_rv = 0.0;

        let (assign36490_e51176, assign36490_e51176_d_n13,) = {
    if ((locals.var_guard1207 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign36490_e51176;
        locals.var_qb_nqs_dn13 = assign36490_e51176_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let (assign36580_e51260, assign36580_e51260_d_n0, assign36580_e51260_d_n2, assign36580_e51260_d_n6, assign36580_e51260_d_n7, assign36580_e51260_d_n10, assign36580_e51260_d_n11, assign36580_e51260_d_n12, assign36580_e51260_d_n13, assign36580_e51260_d_n15, assign36580_e51260_d_n16, assign36580_e51260_d_n17, assign36580_e51260_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign36580_e51254: f64 = (-locals.var_qd_nqs);
        let assign36580_e51256: f64 = (assign36580_e51254 - locals.var_qs_nqs);
        let assign36580_e51258: f64 = (assign36580_e51256 - locals.var_qb_nqs);
        (assign36580_e51258, ((-locals.var_qd_nqs_dn0) - locals.var_qs_nqs_dn0), ((-locals.var_qd_nqs_dn2) - locals.var_qs_nqs_dn2), ((-locals.var_qd_nqs_dn6) - locals.var_qs_nqs_dn6), ((-locals.var_qd_nqs_dn7) - locals.var_qs_nqs_dn7), ((-locals.var_qd_nqs_dn10) - locals.var_qs_nqs_dn10), ((-locals.var_qd_nqs_dn11) - locals.var_qs_nqs_dn11), ((-locals.var_qd_nqs_dn12) - locals.var_qs_nqs_dn12), (-locals.var_qb_nqs_dn13), (-locals.var_qd_nqs_dn15), (-locals.var_qs_nqs_dn16), ((-locals.var_qd_nqs_dn17) - locals.var_qs_nqs_dn17), ((-locals.var_qd_nqs_dn18) - locals.var_qs_nqs_dn18),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36580_e51260;
        locals.var_qg_nqs_dn0 = assign36580_e51260_d_n0;
        locals.var_qg_nqs_dn2 = assign36580_e51260_d_n2;
        locals.var_qg_nqs_dn6 = assign36580_e51260_d_n6;
        locals.var_qg_nqs_dn7 = assign36580_e51260_d_n7;
        locals.var_qg_nqs_dn10 = assign36580_e51260_d_n10;
        locals.var_qg_nqs_dn11 = assign36580_e51260_d_n11;
        locals.var_qg_nqs_dn12 = assign36580_e51260_d_n12;
        locals.var_qg_nqs_dn13 = assign36580_e51260_d_n13;
        locals.var_qg_nqs_dn15 = assign36580_e51260_d_n15;
        locals.var_qg_nqs_dn16 = assign36580_e51260_d_n16;
        locals.var_qg_nqs_dn17 = assign36580_e51260_d_n17;
        locals.var_qg_nqs_dn18 = assign36580_e51260_d_n18;
        locals.var_qg_nqs_rv = 0.0;

        let (assign36620_e51292, assign36620_e51292_d_n0, assign36620_e51292_d_n2, assign36620_e51292_d_n6, assign36620_e51292_d_n7, assign36620_e51292_d_n10, assign36620_e51292_d_n11, assign36620_e51292_d_n12, assign36620_e51292_d_n15, assign36620_e51292_d_n17, assign36620_e51292_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign36620_e51292;
        locals.var_qd_nqs_dn0 = assign36620_e51292_d_n0;
        locals.var_qd_nqs_dn2 = assign36620_e51292_d_n2;
        locals.var_qd_nqs_dn6 = assign36620_e51292_d_n6;
        locals.var_qd_nqs_dn7 = assign36620_e51292_d_n7;
        locals.var_qd_nqs_dn10 = assign36620_e51292_d_n10;
        locals.var_qd_nqs_dn11 = assign36620_e51292_d_n11;
        locals.var_qd_nqs_dn12 = assign36620_e51292_d_n12;
        locals.var_qd_nqs_dn15 = assign36620_e51292_d_n15;
        locals.var_qd_nqs_dn17 = assign36620_e51292_d_n17;
        locals.var_qd_nqs_dn18 = assign36620_e51292_d_n18;
        locals.var_qd_nqs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_129(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36630_e51300, assign36630_e51300_d_n0, assign36630_e51300_d_n2, assign36630_e51300_d_n6, assign36630_e51300_d_n7, assign36630_e51300_d_n10, assign36630_e51300_d_n11, assign36630_e51300_d_n12, assign36630_e51300_d_n16, assign36630_e51300_d_n17, assign36630_e51300_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36630_e51300;
        locals.var_qs_nqs_dn0 = assign36630_e51300_d_n0;
        locals.var_qs_nqs_dn2 = assign36630_e51300_d_n2;
        locals.var_qs_nqs_dn6 = assign36630_e51300_d_n6;
        locals.var_qs_nqs_dn7 = assign36630_e51300_d_n7;
        locals.var_qs_nqs_dn10 = assign36630_e51300_d_n10;
        locals.var_qs_nqs_dn11 = assign36630_e51300_d_n11;
        locals.var_qs_nqs_dn12 = assign36630_e51300_d_n12;
        locals.var_qs_nqs_dn16 = assign36630_e51300_d_n16;
        locals.var_qs_nqs_dn17 = assign36630_e51300_d_n17;
        locals.var_qs_nqs_dn18 = assign36630_e51300_d_n18;
        locals.var_qs_nqs_rv = 0.0;

        let (assign36640_e51308, assign36640_e51308_d_n0, assign36640_e51308_d_n2, assign36640_e51308_d_n6, assign36640_e51308_d_n7, assign36640_e51308_d_n10, assign36640_e51308_d_n11, assign36640_e51308_d_n12, assign36640_e51308_d_n13, assign36640_e51308_d_n15, assign36640_e51308_d_n16, assign36640_e51308_d_n17, assign36640_e51308_d_n18,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36640_e51308;
        locals.var_qg_nqs_dn0 = assign36640_e51308_d_n0;
        locals.var_qg_nqs_dn2 = assign36640_e51308_d_n2;
        locals.var_qg_nqs_dn6 = assign36640_e51308_d_n6;
        locals.var_qg_nqs_dn7 = assign36640_e51308_d_n7;
        locals.var_qg_nqs_dn10 = assign36640_e51308_d_n10;
        locals.var_qg_nqs_dn11 = assign36640_e51308_d_n11;
        locals.var_qg_nqs_dn12 = assign36640_e51308_d_n12;
        locals.var_qg_nqs_dn13 = assign36640_e51308_d_n13;
        locals.var_qg_nqs_dn15 = assign36640_e51308_d_n15;
        locals.var_qg_nqs_dn16 = assign36640_e51308_d_n16;
        locals.var_qg_nqs_dn17 = assign36640_e51308_d_n17;
        locals.var_qg_nqs_dn18 = assign36640_e51308_d_n18;
        locals.var_qg_nqs_rv = 0.0;

        let (assign36650_e51316, assign36650_e51316_d_n13,) = {
    if ((locals.var_guard1207 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign36650_e51316;
        locals.var_qb_nqs_dn13 = assign36650_e51316_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let assign36680_e51321: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign36680_e51321;
        locals.var_guard1212_rv = 0.0;

        let (assign36690_e51325, assign36690_e51325_d_n0, assign36690_e51325_d_n2, assign36690_e51325_d_n6, assign36690_e51325_d_n7, assign36690_e51325_d_n10, assign36690_e51325_d_n11, assign36690_e51325_d_n12, assign36690_e51325_d_n17,) = {
    if (locals.var_guard1212 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36690_e51325;
        locals.var_ids_dn0 = assign36690_e51325_d_n0;
        locals.var_ids_dn2 = assign36690_e51325_d_n2;
        locals.var_ids_dn6 = assign36690_e51325_d_n6;
        locals.var_ids_dn7 = assign36690_e51325_d_n7;
        locals.var_ids_dn10 = assign36690_e51325_d_n10;
        locals.var_ids_dn11 = assign36690_e51325_d_n11;
        locals.var_ids_dn12 = assign36690_e51325_d_n12;
        locals.var_ids_dn17 = assign36690_e51325_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign36700_e51329, assign36700_e51329_d_n0, assign36700_e51329_d_n2, assign36700_e51329_d_n6, assign36700_e51329_d_n7, assign36700_e51329_d_n10, assign36700_e51329_d_n11, assign36700_e51329_d_n12, assign36700_e51329_d_n17,) = {
    if (locals.var_guard1212 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36700_e51329;
        locals.var_isub_dn0 = assign36700_e51329_d_n0;
        locals.var_isub_dn2 = assign36700_e51329_d_n2;
        locals.var_isub_dn6 = assign36700_e51329_d_n6;
        locals.var_isub_dn7 = assign36700_e51329_d_n7;
        locals.var_isub_dn10 = assign36700_e51329_d_n10;
        locals.var_isub_dn11 = assign36700_e51329_d_n11;
        locals.var_isub_dn12 = assign36700_e51329_d_n12;
        locals.var_isub_dn17 = assign36700_e51329_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign36720_e51339, assign36720_e51339_d_n0, assign36720_e51339_d_n2, assign36720_e51339_d_n6, assign36720_e51339_d_n7, assign36720_e51339_d_n10, assign36720_e51339_d_n11, assign36720_e51339_d_n12, assign36720_e51339_d_n13, assign36720_e51339_d_n15, assign36720_e51339_d_n16, assign36720_e51339_d_n17, assign36720_e51339_d_n18,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36720_e51337: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36720_e51337, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36720_e51339;
        locals.var_qg_dn0 = assign36720_e51339_d_n0;
        locals.var_qg_dn2 = assign36720_e51339_d_n2;
        locals.var_qg_dn6 = assign36720_e51339_d_n6;
        locals.var_qg_dn7 = assign36720_e51339_d_n7;
        locals.var_qg_dn10 = assign36720_e51339_d_n10;
        locals.var_qg_dn11 = assign36720_e51339_d_n11;
        locals.var_qg_dn12 = assign36720_e51339_d_n12;
        locals.var_qg_dn13 = assign36720_e51339_d_n13;
        locals.var_qg_dn15 = assign36720_e51339_d_n15;
        locals.var_qg_dn16 = assign36720_e51339_d_n16;
        locals.var_qg_dn17 = assign36720_e51339_d_n17;
        locals.var_qg_dn18 = assign36720_e51339_d_n18;
        locals.var_qg_rv = 0.0;

        let (assign36730_e51345, assign36730_e51345_d_n0, assign36730_e51345_d_n2, assign36730_e51345_d_n6, assign36730_e51345_d_n7, assign36730_e51345_d_n10, assign36730_e51345_d_n11, assign36730_e51345_d_n12, assign36730_e51345_d_n13, assign36730_e51345_d_n15, assign36730_e51345_d_n16, assign36730_e51345_d_n17, assign36730_e51345_d_n18,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36730_e51343: f64 = (locals.var_qde + locals.var_qd_nqs);
        (assign36730_e51343, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36730_e51345;
        locals.var_qd_dn0 = assign36730_e51345_d_n0;
        locals.var_qd_dn2 = assign36730_e51345_d_n2;
        locals.var_qd_dn6 = assign36730_e51345_d_n6;
        locals.var_qd_dn7 = assign36730_e51345_d_n7;
        locals.var_qd_dn10 = assign36730_e51345_d_n10;
        locals.var_qd_dn11 = assign36730_e51345_d_n11;
        locals.var_qd_dn12 = assign36730_e51345_d_n12;
        locals.var_qd_dn13 = assign36730_e51345_d_n13;
        locals.var_qd_dn15 = assign36730_e51345_d_n15;
        locals.var_qd_dn16 = assign36730_e51345_d_n16;
        locals.var_qd_dn17 = assign36730_e51345_d_n17;
        locals.var_qd_dn18 = assign36730_e51345_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign36750_e51360, assign36750_e51360_d_n0, assign36750_e51360_d_n2, assign36750_e51360_d_n6, assign36750_e51360_d_n7, assign36750_e51360_d_n10, assign36750_e51360_d_n11, assign36750_e51360_d_n12, assign36750_e51360_d_n13, assign36750_e51360_d_n15, assign36750_e51360_d_n16, assign36750_e51360_d_n17, assign36750_e51360_d_n18,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36750_e51355: f64 = (locals.var_qge + locals.var_qde);
        let assign36750_e51357: f64 = (assign36750_e51355 + locals.var_qse);
        let assign36750_e51358: f64 = (-assign36750_e51357);
        (assign36750_e51358, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36750_e51360;
        locals.var_qbe_dn0 = assign36750_e51360_d_n0;
        locals.var_qbe_dn2 = assign36750_e51360_d_n2;
        locals.var_qbe_dn6 = assign36750_e51360_d_n6;
        locals.var_qbe_dn7 = assign36750_e51360_d_n7;
        locals.var_qbe_dn10 = assign36750_e51360_d_n10;
        locals.var_qbe_dn11 = assign36750_e51360_d_n11;
        locals.var_qbe_dn12 = assign36750_e51360_d_n12;
        locals.var_qbe_dn13 = assign36750_e51360_d_n13;
        locals.var_qbe_dn15 = assign36750_e51360_d_n15;
        locals.var_qbe_dn16 = assign36750_e51360_d_n16;
        locals.var_qbe_dn17 = assign36750_e51360_d_n17;
        locals.var_qbe_dn18 = assign36750_e51360_d_n18;
        locals.var_qbe_rv = 0.0;

        let (assign36760_e51366, assign36760_e51366_d_n0, assign36760_e51366_d_n2, assign36760_e51366_d_n6, assign36760_e51366_d_n7, assign36760_e51366_d_n10, assign36760_e51366_d_n11, assign36760_e51366_d_n12, assign36760_e51366_d_n13, assign36760_e51366_d_n15, assign36760_e51366_d_n16, assign36760_e51366_d_n17, assign36760_e51366_d_n18,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign36760_e51364: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36760_e51364, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36760_e51366;
        locals.var_qb_dn0 = assign36760_e51366_d_n0;
        locals.var_qb_dn2 = assign36760_e51366_d_n2;
        locals.var_qb_dn6 = assign36760_e51366_d_n6;
        locals.var_qb_dn7 = assign36760_e51366_d_n7;
        locals.var_qb_dn10 = assign36760_e51366_d_n10;
        locals.var_qb_dn11 = assign36760_e51366_d_n11;
        locals.var_qb_dn12 = assign36760_e51366_d_n12;
        locals.var_qb_dn13 = assign36760_e51366_d_n13;
        locals.var_qb_dn15 = assign36760_e51366_d_n15;
        locals.var_qb_dn16 = assign36760_e51366_d_n16;
        locals.var_qb_dn17 = assign36760_e51366_d_n17;
        locals.var_qb_dn18 = assign36760_e51366_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign36770_e51372, assign36770_e51372_d_n0, assign36770_e51372_d_n2, assign36770_e51372_d_n6, assign36770_e51372_d_n7, assign36770_e51372_d_n10, assign36770_e51372_d_n11, assign36770_e51372_d_n12, assign36770_e51372_d_n17,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36770_e51370: f64 = (-locals.var_idse);
        (assign36770_e51370, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36770_e51372;
        locals.var_ids_dn0 = assign36770_e51372_d_n0;
        locals.var_ids_dn2 = assign36770_e51372_d_n2;
        locals.var_ids_dn6 = assign36770_e51372_d_n6;
        locals.var_ids_dn7 = assign36770_e51372_d_n7;
        locals.var_ids_dn10 = assign36770_e51372_d_n10;
        locals.var_ids_dn11 = assign36770_e51372_d_n11;
        locals.var_ids_dn12 = assign36770_e51372_d_n12;
        locals.var_ids_dn17 = assign36770_e51372_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign36790_e51382, assign36790_e51382_d_n0, assign36790_e51382_d_n2, assign36790_e51382_d_n6, assign36790_e51382_d_n7, assign36790_e51382_d_n10, assign36790_e51382_d_n11, assign36790_e51382_d_n12, assign36790_e51382_d_n17,) = {
    if (locals.var_guard1212 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36790_e51382;
        locals.var_isub_dn0 = assign36790_e51382_d_n0;
        locals.var_isub_dn2 = assign36790_e51382_d_n2;
        locals.var_isub_dn6 = assign36790_e51382_d_n6;
        locals.var_isub_dn7 = assign36790_e51382_d_n7;
        locals.var_isub_dn10 = assign36790_e51382_d_n10;
        locals.var_isub_dn11 = assign36790_e51382_d_n11;
        locals.var_isub_dn12 = assign36790_e51382_d_n12;
        locals.var_isub_dn17 = assign36790_e51382_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign36800_e51389, assign36800_e51389_d_n0, assign36800_e51389_d_n2, assign36800_e51389_d_n6, assign36800_e51389_d_n7, assign36800_e51389_d_n10, assign36800_e51389_d_n11, assign36800_e51389_d_n12, assign36800_e51389_d_n13, assign36800_e51389_d_n15, assign36800_e51389_d_n16, assign36800_e51389_d_n17, assign36800_e51389_d_n18,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36800_e51387: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36800_e51387, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36800_e51389;
        locals.var_qg_dn0 = assign36800_e51389_d_n0;
        locals.var_qg_dn2 = assign36800_e51389_d_n2;
        locals.var_qg_dn6 = assign36800_e51389_d_n6;
        locals.var_qg_dn7 = assign36800_e51389_d_n7;
        locals.var_qg_dn10 = assign36800_e51389_d_n10;
        locals.var_qg_dn11 = assign36800_e51389_d_n11;
        locals.var_qg_dn12 = assign36800_e51389_d_n12;
        locals.var_qg_dn13 = assign36800_e51389_d_n13;
        locals.var_qg_dn15 = assign36800_e51389_d_n15;
        locals.var_qg_dn16 = assign36800_e51389_d_n16;
        locals.var_qg_dn17 = assign36800_e51389_d_n17;
        locals.var_qg_dn18 = assign36800_e51389_d_n18;
        locals.var_qg_rv = 0.0;

        let (assign36810_e51396, assign36810_e51396_d_n0, assign36810_e51396_d_n2, assign36810_e51396_d_n6, assign36810_e51396_d_n7, assign36810_e51396_d_n10, assign36810_e51396_d_n11, assign36810_e51396_d_n12, assign36810_e51396_d_n13, assign36810_e51396_d_n15, assign36810_e51396_d_n16, assign36810_e51396_d_n17, assign36810_e51396_d_n18,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36810_e51394: f64 = (locals.var_qse + locals.var_qs_nqs);
        (assign36810_e51394, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36810_e51396;
        locals.var_qd_dn0 = assign36810_e51396_d_n0;
        locals.var_qd_dn2 = assign36810_e51396_d_n2;
        locals.var_qd_dn6 = assign36810_e51396_d_n6;
        locals.var_qd_dn7 = assign36810_e51396_d_n7;
        locals.var_qd_dn10 = assign36810_e51396_d_n10;
        locals.var_qd_dn11 = assign36810_e51396_d_n11;
        locals.var_qd_dn12 = assign36810_e51396_d_n12;
        locals.var_qd_dn13 = assign36810_e51396_d_n13;
        locals.var_qd_dn15 = assign36810_e51396_d_n15;
        locals.var_qd_dn16 = assign36810_e51396_d_n16;
        locals.var_qd_dn17 = assign36810_e51396_d_n17;
        locals.var_qd_dn18 = assign36810_e51396_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign36830_e51413, assign36830_e51413_d_n0, assign36830_e51413_d_n2, assign36830_e51413_d_n6, assign36830_e51413_d_n7, assign36830_e51413_d_n10, assign36830_e51413_d_n11, assign36830_e51413_d_n12, assign36830_e51413_d_n13, assign36830_e51413_d_n15, assign36830_e51413_d_n16, assign36830_e51413_d_n17, assign36830_e51413_d_n18,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36830_e51408: f64 = (locals.var_qge + locals.var_qde);
        let assign36830_e51410: f64 = (assign36830_e51408 + locals.var_qse);
        let assign36830_e51411: f64 = (-assign36830_e51410);
        (assign36830_e51411, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36830_e51413;
        locals.var_qbe_dn0 = assign36830_e51413_d_n0;
        locals.var_qbe_dn2 = assign36830_e51413_d_n2;
        locals.var_qbe_dn6 = assign36830_e51413_d_n6;
        locals.var_qbe_dn7 = assign36830_e51413_d_n7;
        locals.var_qbe_dn10 = assign36830_e51413_d_n10;
        locals.var_qbe_dn11 = assign36830_e51413_d_n11;
        locals.var_qbe_dn12 = assign36830_e51413_d_n12;
        locals.var_qbe_dn13 = assign36830_e51413_d_n13;
        locals.var_qbe_dn15 = assign36830_e51413_d_n15;
        locals.var_qbe_dn16 = assign36830_e51413_d_n16;
        locals.var_qbe_dn17 = assign36830_e51413_d_n17;
        locals.var_qbe_dn18 = assign36830_e51413_d_n18;
        locals.var_qbe_rv = 0.0;

        let (assign36840_e51420, assign36840_e51420_d_n0, assign36840_e51420_d_n2, assign36840_e51420_d_n6, assign36840_e51420_d_n7, assign36840_e51420_d_n10, assign36840_e51420_d_n11, assign36840_e51420_d_n12, assign36840_e51420_d_n13, assign36840_e51420_d_n15, assign36840_e51420_d_n16, assign36840_e51420_d_n17, assign36840_e51420_d_n18,) = {
    if (locals.var_guard1212 == 0.0) {
        let assign36840_e51418: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36840_e51418, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36840_e51420;
        locals.var_qb_dn0 = assign36840_e51420_d_n0;
        locals.var_qb_dn2 = assign36840_e51420_d_n2;
        locals.var_qb_dn6 = assign36840_e51420_d_n6;
        locals.var_qb_dn7 = assign36840_e51420_d_n7;
        locals.var_qb_dn10 = assign36840_e51420_d_n10;
        locals.var_qb_dn11 = assign36840_e51420_d_n11;
        locals.var_qb_dn12 = assign36840_e51420_d_n12;
        locals.var_qb_dn13 = assign36840_e51420_d_n13;
        locals.var_qb_dn15 = assign36840_e51420_d_n15;
        locals.var_qb_dn16 = assign36840_e51420_d_n16;
        locals.var_qb_dn17 = assign36840_e51420_d_n17;
        locals.var_qb_dn18 = assign36840_e51420_d_n18;
        locals.var_qb_rv = 0.0;

        let assign36900_e51428: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign36900_e51428;
        locals.var_guard1213_rv = 0.0;

        let (assign36910_e51432, assign36910_e51432_d_n0, assign36910_e51432_d_n2, assign36910_e51432_d_n6, assign36910_e51432_d_n7, assign36910_e51432_d_n10, assign36910_e51432_d_n11, assign36910_e51432_d_n12, assign36910_e51432_d_n17,) = {
    if (locals.var_guard1213 != 0.0) {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign36910_e51432;
        locals.var_ibd_dn0 = assign36910_e51432_d_n0;
        locals.var_ibd_dn2 = assign36910_e51432_d_n2;
        locals.var_ibd_dn6 = assign36910_e51432_d_n6;
        locals.var_ibd_dn7 = assign36910_e51432_d_n7;
        locals.var_ibd_dn10 = assign36910_e51432_d_n10;
        locals.var_ibd_dn11 = assign36910_e51432_d_n11;
        locals.var_ibd_dn12 = assign36910_e51432_d_n12;
        locals.var_ibd_dn17 = assign36910_e51432_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign36920_e51436, assign36920_e51436_d_n0, assign36920_e51436_d_n2, assign36920_e51436_d_n6, assign36920_e51436_d_n7, assign36920_e51436_d_n10, assign36920_e51436_d_n11, assign36920_e51436_d_n12, assign36920_e51436_d_n17,) = {
    if (locals.var_guard1213 != 0.0) {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign36920_e51436;
        locals.var_qbd_dn0 = assign36920_e51436_d_n0;
        locals.var_qbd_dn2 = assign36920_e51436_d_n2;
        locals.var_qbd_dn6 = assign36920_e51436_d_n6;
        locals.var_qbd_dn7 = assign36920_e51436_d_n7;
        locals.var_qbd_dn10 = assign36920_e51436_d_n10;
        locals.var_qbd_dn11 = assign36920_e51436_d_n11;
        locals.var_qbd_dn12 = assign36920_e51436_d_n12;
        locals.var_qbd_dn17 = assign36920_e51436_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign36930_e51440, assign36930_e51440_d_n0, assign36930_e51440_d_n2, assign36930_e51440_d_n6, assign36930_e51440_d_n7, assign36930_e51440_d_n10, assign36930_e51440_d_n11, assign36930_e51440_d_n12, assign36930_e51440_d_n17,) = {
    if (locals.var_guard1213 != 0.0) {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign36930_e51440;
        locals.var_ibs_dn0 = assign36930_e51440_d_n0;
        locals.var_ibs_dn2 = assign36930_e51440_d_n2;
        locals.var_ibs_dn6 = assign36930_e51440_d_n6;
        locals.var_ibs_dn7 = assign36930_e51440_d_n7;
        locals.var_ibs_dn10 = assign36930_e51440_d_n10;
        locals.var_ibs_dn11 = assign36930_e51440_d_n11;
        locals.var_ibs_dn12 = assign36930_e51440_d_n12;
        locals.var_ibs_dn17 = assign36930_e51440_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign36940_e51444, assign36940_e51444_d_n0, assign36940_e51444_d_n2, assign36940_e51444_d_n6, assign36940_e51444_d_n7, assign36940_e51444_d_n10, assign36940_e51444_d_n11, assign36940_e51444_d_n12, assign36940_e51444_d_n17,) = {
    if (locals.var_guard1213 != 0.0) {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign36940_e51444;
        locals.var_qbs_dn0 = assign36940_e51444_d_n0;
        locals.var_qbs_dn2 = assign36940_e51444_d_n2;
        locals.var_qbs_dn6 = assign36940_e51444_d_n6;
        locals.var_qbs_dn7 = assign36940_e51444_d_n7;
        locals.var_qbs_dn10 = assign36940_e51444_d_n10;
        locals.var_qbs_dn11 = assign36940_e51444_d_n11;
        locals.var_qbs_dn12 = assign36940_e51444_d_n12;
        locals.var_qbs_dn17 = assign36940_e51444_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign36950_e51451: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign36950_e51451;
        locals.var_guard1214_rv = 0.0;

        let (assign36970_e51461,) = {
    if (locals.var_guard1214 != 0.0) {
        (locals.var_cth,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign36970_e51461;
        locals.var_cthe_rv = 0.0;

        let (assign37000_e51477,) = {
    if (locals.var_guard1214 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign37000_e51477;
        locals.var_cthe_rv = 0.0;

        locals.var_idse = locals.var_ids;
        locals.var_idse_dn0 = locals.var_ids_dn0;
        locals.var_idse_dn2 = locals.var_ids_dn2;
        locals.var_idse_dn6 = locals.var_ids_dn6;
        locals.var_idse_dn7 = locals.var_ids_dn7;
        locals.var_idse_dn10 = locals.var_ids_dn10;
        locals.var_idse_dn11 = locals.var_ids_dn11;
        locals.var_idse_dn12 = locals.var_ids_dn12;
        locals.var_idse_dn17 = locals.var_ids_dn17;
        locals.var_idse_rv = 0.0;

        let assign37170_e51531: f64 = locals.var_qg_dn6;
        locals.var_cgdbd = assign37170_e51531;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn12 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;
        locals.var_cgdbd_dn15 = 0.0;
        locals.var_cgdbd_dn16 = 0.0;
        locals.var_cgdbd_dn17 = 0.0;
        locals.var_cgdbd_dn18 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign37180_e51534: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign37180_e51534;
        locals.var_cgdbd_dn0 = (p.p50 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p50 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn6 = (p.p50 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p50 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn10 = (p.p50 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p50 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn12 = (p.p50 * locals.var_cgdbd_dn12);
        locals.var_cgdbd_dn13 = (p.p50 * locals.var_cgdbd_dn13);
        locals.var_cgdbd_dn15 = (p.p50 * locals.var_cgdbd_dn15);
        locals.var_cgdbd_dn16 = (p.p50 * locals.var_cgdbd_dn16);
        locals.var_cgdbd_dn17 = (p.p50 * locals.var_cgdbd_dn17);
        locals.var_cgdbd_dn18 = (p.p50 * locals.var_cgdbd_dn18);
        locals.var_cgdbd_rv = 0.0;

        let assign37190_e51537: f64 = locals.var_qg_dn7;
        locals.var_cgsbd = assign37190_e51537;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn12 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;
        locals.var_cgsbd_dn15 = 0.0;
        locals.var_cgsbd_dn16 = 0.0;
        locals.var_cgsbd_dn17 = 0.0;
        locals.var_cgsbd_dn18 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign37200_e51540: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign37200_e51540;
        locals.var_cgsbd_dn0 = (p.p50 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p50 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn6 = (p.p50 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p50 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn10 = (p.p50 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p50 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn12 = (p.p50 * locals.var_cgsbd_dn12);
        locals.var_cgsbd_dn13 = (p.p50 * locals.var_cgsbd_dn13);
        locals.var_cgsbd_dn15 = (p.p50 * locals.var_cgsbd_dn15);
        locals.var_cgsbd_dn16 = (p.p50 * locals.var_cgsbd_dn16);
        locals.var_cgsbd_dn17 = (p.p50 * locals.var_cgsbd_dn17);
        locals.var_cgsbd_dn18 = (p.p50 * locals.var_cgsbd_dn18);
        locals.var_cgsbd_rv = 0.0;

        let assign37470_e51621: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign37470_e51621;
        locals.var_guard1216_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_130(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign37480_e51627, assign37480_e51627_d_n0, assign37480_e51627_d_n2, assign37480_e51627_d_n6, assign37480_e51627_d_n7, assign37480_e51627_d_n10, assign37480_e51627_d_n11, assign37480_e51627_d_n12, assign37480_e51627_d_n17,) = {
    if (locals.var_guard1216 != 0.0) {
        let assign37480_e51625: f64 = (p.p50 * locals.var_ibd);
        (assign37480_e51625, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign37480_e51627;
        locals.var_ibdb_dn0 = assign37480_e51627_d_n0;
        locals.var_ibdb_dn2 = assign37480_e51627_d_n2;
        locals.var_ibdb_dn6 = assign37480_e51627_d_n6;
        locals.var_ibdb_dn7 = assign37480_e51627_d_n7;
        locals.var_ibdb_dn10 = assign37480_e51627_d_n10;
        locals.var_ibdb_dn11 = assign37480_e51627_d_n11;
        locals.var_ibdb_dn12 = assign37480_e51627_d_n12;
        locals.var_ibdb_dn17 = assign37480_e51627_d_n17;
        locals.var_ibdb_rv = 0.0;

        let (assign37490_e51633, assign37490_e51633_d_n0, assign37490_e51633_d_n2, assign37490_e51633_d_n6, assign37490_e51633_d_n7, assign37490_e51633_d_n10, assign37490_e51633_d_n11, assign37490_e51633_d_n12, assign37490_e51633_d_n17,) = {
    if (locals.var_guard1216 != 0.0) {
        let assign37490_e51631: f64 = (p.p50 * locals.var_ibs);
        (assign37490_e51631, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign37490_e51633;
        locals.var_ibsb_dn0 = assign37490_e51633_d_n0;
        locals.var_ibsb_dn2 = assign37490_e51633_d_n2;
        locals.var_ibsb_dn6 = assign37490_e51633_d_n6;
        locals.var_ibsb_dn7 = assign37490_e51633_d_n7;
        locals.var_ibsb_dn10 = assign37490_e51633_d_n10;
        locals.var_ibsb_dn11 = assign37490_e51633_d_n11;
        locals.var_ibsb_dn12 = assign37490_e51633_d_n12;
        locals.var_ibsb_dn17 = assign37490_e51633_d_n17;
        locals.var_ibsb_rv = 0.0;

        let assign37610_e51685: f64 = (4.0 * 1.3806226e-23);
        let assign37610_e51687: f64 = (assign37610_e51685 * locals.var_ttemp);
        let assign37610_e51689: f64 = assign37610_e51687;
        locals.var_whi_noise = assign37610_e51689;
        locals.var_whi_noise_dn10 = (assign37610_e51685 * locals.var_ttemp_dn10);
        locals.var_whi_noise_rv = 0.0;

        locals.var_qdrat = locals.var_qdrat_noi;
        locals.var_qdrat_dn0 = locals.var_qdrat_noi_dn0;
        locals.var_qdrat_dn2 = locals.var_qdrat_noi_dn2;
        locals.var_qdrat_dn6 = locals.var_qdrat_noi_dn6;
        locals.var_qdrat_dn7 = locals.var_qdrat_noi_dn7;
        locals.var_qdrat_dn10 = locals.var_qdrat_noi_dn10;
        locals.var_qdrat_dn11 = locals.var_qdrat_noi_dn11;
        locals.var_qdrat_dn12 = locals.var_qdrat_noi_dn12;
        locals.var_qdrat_dn17 = locals.var_qdrat_noi_dn17;
        locals.var_qdrat_rv = 0.0;

        let assign37640_e51696: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign37640_e51696;
        locals.var_sid_dn0 = (locals.var_whi_noise * locals.var_noithrml_dn0);
        locals.var_sid_dn2 = (locals.var_whi_noise * locals.var_noithrml_dn2);
        locals.var_sid_dn6 = (locals.var_whi_noise * locals.var_noithrml_dn6);
        locals.var_sid_dn7 = (locals.var_whi_noise * locals.var_noithrml_dn7);
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn11 = (locals.var_whi_noise * locals.var_noithrml_dn11);
        locals.var_sid_dn12 = (locals.var_whi_noise * locals.var_noithrml_dn12);
        locals.var_sid_dn17 = (locals.var_whi_noise * locals.var_noithrml_dn17);
        locals.var_sid_rv = 0.0;

        let (assign37660_e51710, assign37660_e51710_d_n0, assign37660_e51710_d_n2, assign37660_e51710_d_n6, assign37660_e51710_d_n7, assign37660_e51710_d_n10, assign37660_e51710_d_n11, assign37660_e51710_d_n12, assign37660_e51710_d_n13, assign37660_e51710_d_n15, assign37660_e51710_d_n16, assign37660_e51710_d_n17, assign37660_e51710_d_n18,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign37660_e51707: f64 = (locals.var_noiigate / locals.var_sid);
        let assign37660_e51708: f64 = (assign37660_e51707).sqrt();
        (assign37660_e51708, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn13 / locals.var_sid) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn15 / locals.var_sid) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn16 / locals.var_sid) / (2.0 * assign37660_e51708)), ((((locals.var_noiigate_dn17 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn17)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37660_e51708)), ((locals.var_noiigate_dn18 / locals.var_sid) / (2.0 * assign37660_e51708)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign37660_e51710;
        locals.var_sigrat_dn0 = assign37660_e51710_d_n0;
        locals.var_sigrat_dn2 = assign37660_e51710_d_n2;
        locals.var_sigrat_dn6 = assign37660_e51710_d_n6;
        locals.var_sigrat_dn7 = assign37660_e51710_d_n7;
        locals.var_sigrat_dn10 = assign37660_e51710_d_n10;
        locals.var_sigrat_dn11 = assign37660_e51710_d_n11;
        locals.var_sigrat_dn12 = assign37660_e51710_d_n12;
        locals.var_sigrat_dn13 = assign37660_e51710_d_n13;
        locals.var_sigrat_dn15 = assign37660_e51710_d_n15;
        locals.var_sigrat_dn16 = assign37660_e51710_d_n16;
        locals.var_sigrat_dn17 = assign37660_e51710_d_n17;
        locals.var_sigrat_dn18 = assign37660_e51710_d_n18;
        locals.var_sigrat_rv = 0.0;

        let (assign37670_e51722, assign37670_e51722_d_n0, assign37670_e51722_d_n2, assign37670_e51722_d_n6, assign37670_e51722_d_n7, assign37670_e51722_d_n10, assign37670_e51722_d_n11, assign37670_e51722_d_n12, assign37670_e51722_d_n13, assign37670_e51722_d_n15, assign37670_e51722_d_n16, assign37670_e51722_d_n17, assign37670_e51722_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37670_e51717: f64 = (1.0 - locals.var_qdrat);
        let assign37670_e51718: f64 = (locals.var_sigrat * assign37670_e51717);
        (assign37670_e51718, ((locals.var_sigrat_dn0 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37670_e51717), (locals.var_sigrat_dn15 * assign37670_e51717), (locals.var_sigrat_dn16 * assign37670_e51717), ((locals.var_sigrat_dn17 * assign37670_e51717) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37670_e51717),)
    } else {
        let assign37670_e51721: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37670_e51721, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    }
};
        locals.var_sigrat_s = assign37670_e51722;
        locals.var_sigrat_s_dn0 = assign37670_e51722_d_n0;
        locals.var_sigrat_s_dn2 = assign37670_e51722_d_n2;
        locals.var_sigrat_s_dn6 = assign37670_e51722_d_n6;
        locals.var_sigrat_s_dn7 = assign37670_e51722_d_n7;
        locals.var_sigrat_s_dn10 = assign37670_e51722_d_n10;
        locals.var_sigrat_s_dn11 = assign37670_e51722_d_n11;
        locals.var_sigrat_s_dn12 = assign37670_e51722_d_n12;
        locals.var_sigrat_s_dn13 = assign37670_e51722_d_n13;
        locals.var_sigrat_s_dn15 = assign37670_e51722_d_n15;
        locals.var_sigrat_s_dn16 = assign37670_e51722_d_n16;
        locals.var_sigrat_s_dn17 = assign37670_e51722_d_n17;
        locals.var_sigrat_s_dn18 = assign37670_e51722_d_n18;
        locals.var_sigrat_s_rv = 0.0;

        let (assign37680_e51734, assign37680_e51734_d_n0, assign37680_e51734_d_n2, assign37680_e51734_d_n6, assign37680_e51734_d_n7, assign37680_e51734_d_n10, assign37680_e51734_d_n11, assign37680_e51734_d_n12, assign37680_e51734_d_n13, assign37680_e51734_d_n15, assign37680_e51734_d_n16, assign37680_e51734_d_n17, assign37680_e51734_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37680_e51728: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37680_e51728, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    } else {
        let assign37680_e51732: f64 = (1.0 - locals.var_qdrat);
        let assign37680_e51733: f64 = (locals.var_sigrat * assign37680_e51732);
        (assign37680_e51733, ((locals.var_sigrat_dn0 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37680_e51732), (locals.var_sigrat_dn15 * assign37680_e51732), (locals.var_sigrat_dn16 * assign37680_e51732), ((locals.var_sigrat_dn17 * assign37680_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37680_e51732),)
    }
};
        locals.var_sigrat_d = assign37680_e51734;
        locals.var_sigrat_d_dn0 = assign37680_e51734_d_n0;
        locals.var_sigrat_d_dn2 = assign37680_e51734_d_n2;
        locals.var_sigrat_d_dn6 = assign37680_e51734_d_n6;
        locals.var_sigrat_d_dn7 = assign37680_e51734_d_n7;
        locals.var_sigrat_d_dn10 = assign37680_e51734_d_n10;
        locals.var_sigrat_d_dn11 = assign37680_e51734_d_n11;
        locals.var_sigrat_d_dn12 = assign37680_e51734_d_n12;
        locals.var_sigrat_d_dn13 = assign37680_e51734_d_n13;
        locals.var_sigrat_d_dn15 = assign37680_e51734_d_n15;
        locals.var_sigrat_d_dn16 = assign37680_e51734_d_n16;
        locals.var_sigrat_d_dn17 = assign37680_e51734_d_n17;
        locals.var_sigrat_d_dn18 = assign37680_e51734_d_n18;
        locals.var_sigrat_d_rv = 0.0;

        let assign37700_e51744: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign37700_e51744;
        locals.var_guard1224_rv = 0.0;

        let assign37720_e51751: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign37720_e51751;
        locals.var_guard1225_rv = 0.0;

        let assign37730_e51760: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign37730_e51760;
        locals.var_guard1226_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq2_e316: f64 = (p.p50 * locals.var_ids);
        let eq2_e316_d_n0: f64 = (p.p50 * locals.var_ids_dn0);
        let eq2_e316_d_n2: f64 = (p.p50 * locals.var_ids_dn2);
        let eq2_e316_d_n6: f64 = (p.p50 * locals.var_ids_dn6);
        let eq2_e316_d_n7: f64 = (p.p50 * locals.var_ids_dn7);
        let eq2_e316_d_n10: f64 = (p.p50 * locals.var_ids_dn10);
        let eq2_e316_d_n11: f64 = (p.p50 * locals.var_ids_dn11);
        let eq2_e316_d_n12: f64 = (p.p50 * locals.var_ids_dn12);
        let eq2_e316_d_n17: f64 = (p.p50 * locals.var_ids_dn17);
        let eq2_value: f64 = eq2_e316;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq2_e316_d_n0), multiplicity * (eq2_e316_d_n2), multiplicity * (eq2_e316_d_n6), multiplicity * (eq2_e316_d_n7), multiplicity * (eq2_e316_d_n10), multiplicity * (eq2_e316_d_n11), multiplicity * (eq2_e316_d_n12), multiplicity * (eq2_e316_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq3_e322, eq3_e322_d_n0, eq3_e322_d_n2, eq3_e322_d_n6, eq3_e322_d_n7, eq3_e322_d_n10, eq3_e322_d_n11, eq3_e322_d_n12, eq3_e322_d_n17,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq3_e320: f64 = (p.p50 * locals.var_igs);
        let eq3_e320_d_n0: f64 = (p.p50 * locals.var_igs_dn0);
        let eq3_e320_d_n2: f64 = (p.p50 * locals.var_igs_dn2);
        let eq3_e320_d_n6: f64 = (p.p50 * locals.var_igs_dn6);
        let eq3_e320_d_n7: f64 = (p.p50 * locals.var_igs_dn7);
        let eq3_e320_d_n10: f64 = (p.p50 * locals.var_igs_dn10);
        let eq3_e320_d_n11: f64 = (p.p50 * locals.var_igs_dn11);
        let eq3_e320_d_n12: f64 = (p.p50 * locals.var_igs_dn12);
        let eq3_e320_d_n17: f64 = (p.p50 * locals.var_igs_dn17);
        (eq3_e320, eq3_e320_d_n0, eq3_e320_d_n2, eq3_e320_d_n6, eq3_e320_d_n7, eq3_e320_d_n10, eq3_e320_d_n11, eq3_e320_d_n12, eq3_e320_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e322;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq3_e322_d_n0), multiplicity * (eq3_e322_d_n2), multiplicity * (eq3_e322_d_n6), multiplicity * (eq3_e322_d_n7), multiplicity * (eq3_e322_d_n10), multiplicity * (eq3_e322_d_n11), multiplicity * (eq3_e322_d_n12), multiplicity * (eq3_e322_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n2, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n17,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq4_e326: f64 = (p.p50 * locals.var_igd);
        let eq4_e326_d_n0: f64 = (p.p50 * locals.var_igd_dn0);
        let eq4_e326_d_n2: f64 = (p.p50 * locals.var_igd_dn2);
        let eq4_e326_d_n6: f64 = (p.p50 * locals.var_igd_dn6);
        let eq4_e326_d_n7: f64 = (p.p50 * locals.var_igd_dn7);
        let eq4_e326_d_n10: f64 = (p.p50 * locals.var_igd_dn10);
        let eq4_e326_d_n11: f64 = (p.p50 * locals.var_igd_dn11);
        let eq4_e326_d_n12: f64 = (p.p50 * locals.var_igd_dn12);
        let eq4_e326_d_n17: f64 = (p.p50 * locals.var_igd_dn17);
        (eq4_e326, eq4_e326_d_n0, eq4_e326_d_n2, eq4_e326_d_n6, eq4_e326_d_n7, eq4_e326_d_n10, eq4_e326_d_n11, eq4_e326_d_n12, eq4_e326_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e328;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq4_e328_d_n0), multiplicity * (eq4_e328_d_n2), multiplicity * (eq4_e328_d_n6), multiplicity * (eq4_e328_d_n7), multiplicity * (eq4_e328_d_n10), multiplicity * (eq4_e328_d_n11), multiplicity * (eq4_e328_d_n12), multiplicity * (eq4_e328_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n2, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n17,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq5_e332: f64 = (p.p50 * locals.var_igb);
        let eq5_e332_d_n0: f64 = (p.p50 * locals.var_igb_dn0);
        let eq5_e332_d_n2: f64 = (p.p50 * locals.var_igb_dn2);
        let eq5_e332_d_n6: f64 = (p.p50 * locals.var_igb_dn6);
        let eq5_e332_d_n7: f64 = (p.p50 * locals.var_igb_dn7);
        let eq5_e332_d_n10: f64 = (p.p50 * locals.var_igb_dn10);
        let eq5_e332_d_n11: f64 = (p.p50 * locals.var_igb_dn11);
        let eq5_e332_d_n12: f64 = (p.p50 * locals.var_igb_dn12);
        let eq5_e332_d_n17: f64 = (p.p50 * locals.var_igb_dn17);
        (eq5_e332, eq5_e332_d_n0, eq5_e332_d_n2, eq5_e332_d_n6, eq5_e332_d_n7, eq5_e332_d_n10, eq5_e332_d_n11, eq5_e332_d_n12, eq5_e332_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e334;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq5_e334_d_n0), multiplicity * (eq5_e334_d_n2), multiplicity * (eq5_e334_d_n6), multiplicity * (eq5_e334_d_n7), multiplicity * (eq5_e334_d_n10), multiplicity * (eq5_e334_d_n11), multiplicity * (eq5_e334_d_n12), multiplicity * (eq5_e334_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n2, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n17,) = {
    if (p.p259 != 0.0) {
        let eq6_e338: f64 = ((nv7 - nv2) / locals.var_rsd);
        let eq6_e338_d_n0: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn0) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n2: f64 = (((-locals.var_rsd) - ((nv7 - nv2) * locals.var_rsd_dn2)) / (locals.var_rsd * locals.var_rsd));
        let eq6_e338_d_n6: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn6) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n7: f64 = ((locals.var_rsd - ((nv7 - nv2) * locals.var_rsd_dn7)) / (locals.var_rsd * locals.var_rsd));
        let eq6_e338_d_n10: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn10) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n11: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn11) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n12: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn12) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e338_d_n17: f64 = (-(((nv7 - nv2) * locals.var_rsd_dn17) / (locals.var_rsd * locals.var_rsd)));
        (eq6_e338, eq6_e338_d_n0, eq6_e338_d_n2, eq6_e338_d_n6, eq6_e338_d_n7, eq6_e338_d_n10, eq6_e338_d_n11, eq6_e338_d_n12, eq6_e338_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e340;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(2),
            multiplicity * (eq6_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq6_e340_d_n0), multiplicity * (eq6_e340_d_n2), multiplicity * (eq6_e340_d_n6), multiplicity * (eq6_e340_d_n7), multiplicity * (eq6_e340_d_n10), multiplicity * (eq6_e340_d_n11), multiplicity * (eq6_e340_d_n12), multiplicity * (eq6_e340_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq8_e351, eq8_e351_d_n0, eq8_e351_d_n2, eq8_e351_d_n6, eq8_e351_d_n7, eq8_e351_d_n10, eq8_e351_d_n11, eq8_e351_d_n12, eq8_e351_d_n17,) = {
    if (p.p260 != 0.0) {
        let eq8_e349: f64 = ((nv0 - nv6) / locals.var_rdd);
        let eq8_e349_d_n0: f64 = ((locals.var_rdd - ((nv0 - nv6) * locals.var_rdd_dn0)) / (locals.var_rdd * locals.var_rdd));
        let eq8_e349_d_n2: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn2) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n6: f64 = (((-locals.var_rdd) - ((nv0 - nv6) * locals.var_rdd_dn6)) / (locals.var_rdd * locals.var_rdd));
        let eq8_e349_d_n7: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn7) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n10: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn10) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n11: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn11) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n12: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn12) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e349_d_n17: f64 = (-(((nv0 - nv6) * locals.var_rdd_dn17) / (locals.var_rdd * locals.var_rdd)));
        (eq8_e349, eq8_e349_d_n0, eq8_e349_d_n2, eq8_e349_d_n6, eq8_e349_d_n7, eq8_e349_d_n10, eq8_e349_d_n11, eq8_e349_d_n12, eq8_e349_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e351;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq8_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq8_e351_d_n0), multiplicity * (eq8_e351_d_n2), multiplicity * (eq8_e351_d_n6), multiplicity * (eq8_e351_d_n7), multiplicity * (eq8_e351_d_n10), multiplicity * (eq8_e351_d_n11), multiplicity * (eq8_e351_d_n12), multiplicity * (eq8_e351_d_n17)],
            [],
            [],
            1.0,
        );
        let eq10_e359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qg);
        let eq10_e360: f64 = (p.p50 * eq10_e359);
        let eq10_e360_d_n0: f64 = (p.p50 * (locals.var_qg_dn0 * ddt_scale));
        let eq10_e360_d_n2: f64 = (p.p50 * (locals.var_qg_dn2 * ddt_scale));
        let eq10_e360_d_n6: f64 = (p.p50 * (locals.var_qg_dn6 * ddt_scale));
        let eq10_e360_d_n7: f64 = (p.p50 * (locals.var_qg_dn7 * ddt_scale));
        let eq10_e360_d_n10: f64 = (p.p50 * (locals.var_qg_dn10 * ddt_scale));
        let eq10_e360_d_n11: f64 = (p.p50 * (locals.var_qg_dn11 * ddt_scale));
        let eq10_e360_d_n12: f64 = (p.p50 * (locals.var_qg_dn12 * ddt_scale));
        let eq10_e360_d_n13: f64 = (p.p50 * (locals.var_qg_dn13 * ddt_scale));
        let eq10_e360_d_n15: f64 = (p.p50 * (locals.var_qg_dn15 * ddt_scale));
        let eq10_e360_d_n16: f64 = (p.p50 * (locals.var_qg_dn16 * ddt_scale));
        let eq10_e360_d_n17: f64 = (p.p50 * (locals.var_qg_dn17 * ddt_scale));
        let eq10_e360_d_n18: f64 = (p.p50 * (locals.var_qg_dn18 * ddt_scale));
        let eq10_value: f64 = eq10_e360;
        let eq10_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq10_node_derivatives: [f64; 12] = [eq10_e360_d_n0, eq10_e360_d_n2, eq10_e360_d_n6, eq10_e360_d_n7, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qd);
        let eq11_e364: f64 = (p.p50 * eq11_e363);
        let eq11_e364_d_n0: f64 = (p.p50 * (locals.var_qd_dn0 * ddt_scale));
        let eq11_e364_d_n2: f64 = (p.p50 * (locals.var_qd_dn2 * ddt_scale));
        let eq11_e364_d_n6: f64 = (p.p50 * (locals.var_qd_dn6 * ddt_scale));
        let eq11_e364_d_n7: f64 = (p.p50 * (locals.var_qd_dn7 * ddt_scale));
        let eq11_e364_d_n10: f64 = (p.p50 * (locals.var_qd_dn10 * ddt_scale));
        let eq11_e364_d_n11: f64 = (p.p50 * (locals.var_qd_dn11 * ddt_scale));
        let eq11_e364_d_n12: f64 = (p.p50 * (locals.var_qd_dn12 * ddt_scale));
        let eq11_e364_d_n13: f64 = (p.p50 * (locals.var_qd_dn13 * ddt_scale));
        let eq11_e364_d_n15: f64 = (p.p50 * (locals.var_qd_dn15 * ddt_scale));
        let eq11_e364_d_n16: f64 = (p.p50 * (locals.var_qd_dn16 * ddt_scale));
        let eq11_e364_d_n17: f64 = (p.p50 * (locals.var_qd_dn17 * ddt_scale));
        let eq11_e364_d_n18: f64 = (p.p50 * (locals.var_qd_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e364;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e364_d_n0, eq11_e364_d_n2, eq11_e364_d_n6, eq11_e364_d_n7, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qb);
        let eq12_e368: f64 = (p.p50 * eq12_e367);
        let eq12_e368_d_n0: f64 = (p.p50 * (locals.var_qb_dn0 * ddt_scale));
        let eq12_e368_d_n2: f64 = (p.p50 * (locals.var_qb_dn2 * ddt_scale));
        let eq12_e368_d_n6: f64 = (p.p50 * (locals.var_qb_dn6 * ddt_scale));
        let eq12_e368_d_n7: f64 = (p.p50 * (locals.var_qb_dn7 * ddt_scale));
        let eq12_e368_d_n10: f64 = (p.p50 * (locals.var_qb_dn10 * ddt_scale));
        let eq12_e368_d_n11: f64 = (p.p50 * (locals.var_qb_dn11 * ddt_scale));
        let eq12_e368_d_n12: f64 = (p.p50 * (locals.var_qb_dn12 * ddt_scale));
        let eq12_e368_d_n13: f64 = (p.p50 * (locals.var_qb_dn13 * ddt_scale));
        let eq12_e368_d_n15: f64 = (p.p50 * (locals.var_qb_dn15 * ddt_scale));
        let eq12_e368_d_n16: f64 = (p.p50 * (locals.var_qb_dn16 * ddt_scale));
        let eq12_e368_d_n17: f64 = (p.p50 * (locals.var_qb_dn17 * ddt_scale));
        let eq12_e368_d_n18: f64 = (p.p50 * (locals.var_qb_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e368;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e368_d_n0, eq12_e368_d_n2, eq12_e368_d_n6, eq12_e368_d_n7, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq17_e394: f64 = (locals.var_ci * (nv14 - 0.0));
        let eq17_e394_d_n0: f64 = (locals.var_ci_dn0 * (nv14 - 0.0));
        let eq17_e394_d_n2: f64 = (locals.var_ci_dn2 * (nv14 - 0.0));
        let eq17_e394_d_n6: f64 = (locals.var_ci_dn6 * (nv14 - 0.0));
        let eq17_e394_d_n7: f64 = (locals.var_ci_dn7 * (nv14 - 0.0));
        let eq17_e394_d_n10: f64 = (locals.var_ci_dn10 * (nv14 - 0.0));
        let eq17_e394_d_n11: f64 = (locals.var_ci_dn11 * (nv14 - 0.0));
        let eq17_e394_d_n12: f64 = (locals.var_ci_dn12 * (nv14 - 0.0));
        let eq17_e394_d_n17: f64 = (locals.var_ci_dn17 * (nv14 - 0.0));
        let eq17_value: f64 = eq17_e394;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            [0, 2, 6, 7, 10, 11, 12, 14, 17],
            [multiplicity * (eq17_e394_d_n0), multiplicity * (eq17_e394_d_n2), multiplicity * (eq17_e394_d_n6), multiplicity * (eq17_e394_d_n7), multiplicity * (eq17_e394_d_n10), multiplicity * (eq17_e394_d_n11), multiplicity * (eq17_e394_d_n12), multiplicity * (locals.var_ci), multiplicity * (eq17_e394_d_n17)],
            [],
            [],
            1.0,
        );
        let eq18_e397: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq18_e398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e397);
        let eq18_value: f64 = eq18_e398;
        let eq18_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq18_node_derivatives: [f64; 13] = [(eq18_e397_d_n0 * ddt_scale), (eq18_e397_d_n2 * ddt_scale), (eq18_e397_d_n6 * ddt_scale), (eq18_e397_d_n7 * ddt_scale), (eq18_e397_d_n10 * ddt_scale), (eq18_e397_d_n11 * ddt_scale), (eq18_e397_d_n12 * ddt_scale), (eq18_e397_d_n13 * ddt_scale), (locals.var_sigrat_s * ddt_scale), (eq18_e397_d_n15 * ddt_scale), (eq18_e397_d_n16 * ddt_scale), (eq18_e397_d_n17 * ddt_scale), (eq18_e397_d_n18 * ddt_scale)];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e401: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq19_e402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e401);
        let eq19_value: f64 = eq19_e402;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq19_node_derivatives: [f64; 13] = [(eq19_e401_d_n0 * ddt_scale), (eq19_e401_d_n2 * ddt_scale), (eq19_e401_d_n6 * ddt_scale), (eq19_e401_d_n7 * ddt_scale), (eq19_e401_d_n10 * ddt_scale), (eq19_e401_d_n11 * ddt_scale), (eq19_e401_d_n12 * ddt_scale), (eq19_e401_d_n13 * ddt_scale), (locals.var_sigrat_d * ddt_scale), (eq19_e401_d_n15 * ddt_scale), (eq19_e401_d_n16 * ddt_scale), (eq19_e401_d_n17 * ddt_scale), (eq19_e401_d_n18 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq25_e454, eq25_e454_d_n1, eq25_e454_d_n11,) = {
    if (p.p35 != 0.0) {
        let eq25_e452: f64 = (locals.var_grg * (nv1 - nv11));
        (eq25_e452, locals.var_grg, (-locals.var_grg),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e454;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(11),
            multiplicity * (eq25_value),
            1,
            multiplicity * (eq25_e454_d_n1),
            11,
            multiplicity * (eq25_e454_d_n11),
        );
        let (eq27_e465, eq27_e465_d_n10,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq27_e463: f64 = ((nv10 - 0.0) * locals.var_gth);
        (eq27_e463, locals.var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e465;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e465_d_n10),
        );
        let (eq28_e470, eq28_e470_d_n0, eq28_e470_d_n2, eq28_e470_d_n6, eq28_e470_d_n7, eq28_e470_d_n10, eq28_e470_d_n11, eq28_e470_d_n12, eq28_e470_d_n17,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq28_e468: f64 = (-locals.var_itemp);
        (eq28_e468, (-locals.var_itemp_dn0), (-locals.var_itemp_dn2), (-locals.var_itemp_dn6), (-locals.var_itemp_dn7), (-locals.var_itemp_dn10), (-locals.var_itemp_dn11), (-locals.var_itemp_dn12), (-locals.var_itemp_dn17),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e470;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(10),
            None,
            multiplicity * (eq28_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq28_e470_d_n0), multiplicity * (eq28_e470_d_n2), multiplicity * (eq28_e470_d_n6), multiplicity * (eq28_e470_d_n7), multiplicity * (eq28_e470_d_n10), multiplicity * (eq28_e470_d_n11), multiplicity * (eq28_e470_d_n12), multiplicity * (eq28_e470_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq30_e483, eq30_e483_d_n10,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq30_e480: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e481: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq30_e480);
        (eq30_e481, (locals.var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e483;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e483_d_n10),
        );
        let (eq32_e498, eq32_e498_d_n0, eq32_e498_d_n2, eq32_e498_d_n6, eq32_e498_d_n7, eq32_e498_d_n10, eq32_e498_d_n11, eq32_e498_d_n12, eq32_e498_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq32_e495: f64 = (locals.var_igidl + locals.var_isub);
        let eq32_e495_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq32_e495_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq32_e495_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq32_e495_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq32_e495_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq32_e495_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq32_e495_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq32_e495_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq32_e496: f64 = (p.p50 * eq32_e495);
        let eq32_e496_d_n0: f64 = (p.p50 * eq32_e495_d_n0);
        let eq32_e496_d_n2: f64 = (p.p50 * eq32_e495_d_n2);
        let eq32_e496_d_n6: f64 = (p.p50 * eq32_e495_d_n6);
        let eq32_e496_d_n7: f64 = (p.p50 * eq32_e495_d_n7);
        let eq32_e496_d_n10: f64 = (p.p50 * eq32_e495_d_n10);
        let eq32_e496_d_n11: f64 = (p.p50 * eq32_e495_d_n11);
        let eq32_e496_d_n12: f64 = (p.p50 * eq32_e495_d_n12);
        let eq32_e496_d_n17: f64 = (p.p50 * eq32_e495_d_n17);
        (eq32_e496, eq32_e496_d_n0, eq32_e496_d_n2, eq32_e496_d_n6, eq32_e496_d_n7, eq32_e496_d_n10, eq32_e496_d_n11, eq32_e496_d_n12, eq32_e496_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e498;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq32_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq32_e498_d_n0), multiplicity * (eq32_e498_d_n2), multiplicity * (eq32_e498_d_n6), multiplicity * (eq32_e498_d_n7), multiplicity * (eq32_e498_d_n10), multiplicity * (eq32_e498_d_n11), multiplicity * (eq32_e498_d_n12), multiplicity * (eq32_e498_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n2, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq33_e503: f64 = (locals.var_igisl + locals.var_isubs);
        let eq33_e503_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq33_e503_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq33_e503_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq33_e503_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq33_e503_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq33_e503_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq33_e503_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq33_e503_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq33_e504: f64 = (p.p50 * eq33_e503);
        let eq33_e504_d_n0: f64 = (p.p50 * eq33_e503_d_n0);
        let eq33_e504_d_n2: f64 = (p.p50 * eq33_e503_d_n2);
        let eq33_e504_d_n6: f64 = (p.p50 * eq33_e503_d_n6);
        let eq33_e504_d_n7: f64 = (p.p50 * eq33_e503_d_n7);
        let eq33_e504_d_n10: f64 = (p.p50 * eq33_e503_d_n10);
        let eq33_e504_d_n11: f64 = (p.p50 * eq33_e503_d_n11);
        let eq33_e504_d_n12: f64 = (p.p50 * eq33_e503_d_n12);
        let eq33_e504_d_n17: f64 = (p.p50 * eq33_e503_d_n17);
        (eq33_e504, eq33_e504_d_n0, eq33_e504_d_n2, eq33_e504_d_n6, eq33_e504_d_n7, eq33_e504_d_n10, eq33_e504_d_n11, eq33_e504_d_n12, eq33_e504_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e506;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq33_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq33_e506_d_n0), multiplicity * (eq33_e506_d_n2), multiplicity * (eq33_e506_d_n6), multiplicity * (eq33_e506_d_n7), multiplicity * (eq33_e506_d_n10), multiplicity * (eq33_e506_d_n11), multiplicity * (eq33_e506_d_n12), multiplicity * (eq33_e506_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n2, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq34_e511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbs);
        let eq34_e512: f64 = (locals.var_ibs + eq34_e511);
        let eq34_e512_d_n0: f64 = (locals.var_ibs_dn0 + (locals.var_qbs_dn0 * ddt_scale));
        let eq34_e512_d_n2: f64 = (locals.var_ibs_dn2 + (locals.var_qbs_dn2 * ddt_scale));
        let eq34_e512_d_n6: f64 = (locals.var_ibs_dn6 + (locals.var_qbs_dn6 * ddt_scale));
        let eq34_e512_d_n7: f64 = (locals.var_ibs_dn7 + (locals.var_qbs_dn7 * ddt_scale));
        let eq34_e512_d_n10: f64 = (locals.var_ibs_dn10 + (locals.var_qbs_dn10 * ddt_scale));
        let eq34_e512_d_n11: f64 = (locals.var_ibs_dn11 + (locals.var_qbs_dn11 * ddt_scale));
        let eq34_e512_d_n12: f64 = (locals.var_ibs_dn12 + (locals.var_qbs_dn12 * ddt_scale));
        let eq34_e512_d_n17: f64 = (locals.var_ibs_dn17 + (locals.var_qbs_dn17 * ddt_scale));
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n2, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e515;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e515_d_n0), multiplicity * (eq34_e515_d_n2), multiplicity * (eq34_e515_d_n6), multiplicity * (eq34_e515_d_n7), multiplicity * (eq34_e515_d_n10), multiplicity * (eq34_e515_d_n11), multiplicity * (eq34_e515_d_n12), multiplicity * (eq34_e515_d_n17)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        locals: &mut StampLocals,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n2, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq35_e520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbd);
        let eq35_e521: f64 = (locals.var_ibd + eq35_e520);
        let eq35_e521_d_n0: f64 = (locals.var_ibd_dn0 + (locals.var_qbd_dn0 * ddt_scale));
        let eq35_e521_d_n2: f64 = (locals.var_ibd_dn2 + (locals.var_qbd_dn2 * ddt_scale));
        let eq35_e521_d_n6: f64 = (locals.var_ibd_dn6 + (locals.var_qbd_dn6 * ddt_scale));
        let eq35_e521_d_n7: f64 = (locals.var_ibd_dn7 + (locals.var_qbd_dn7 * ddt_scale));
        let eq35_e521_d_n10: f64 = (locals.var_ibd_dn10 + (locals.var_qbd_dn10 * ddt_scale));
        let eq35_e521_d_n11: f64 = (locals.var_ibd_dn11 + (locals.var_qbd_dn11 * ddt_scale));
        let eq35_e521_d_n12: f64 = (locals.var_ibd_dn12 + (locals.var_qbd_dn12 * ddt_scale));
        let eq35_e521_d_n17: f64 = (locals.var_ibd_dn17 + (locals.var_qbd_dn17 * ddt_scale));
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n2, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e524;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e524_d_n0), multiplicity * (eq35_e524_d_n2), multiplicity * (eq35_e524_d_n6), multiplicity * (eq35_e524_d_n7), multiplicity * (eq35_e524_d_n10), multiplicity * (eq35_e524_d_n11), multiplicity * (eq35_e524_d_n12), multiplicity * (eq35_e524_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq43_e581, eq43_e581_d_n0, eq43_e581_d_n2, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn7, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, locals.var_iqb_nqs_dn13, locals.var_iqb_nqs_dn15, locals.var_iqb_nqs_dn16, locals.var_iqb_nqs_dn17, locals.var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e581;
        let eq43_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq43_node_derivatives: [f64; 12] = [eq43_e581_d_n0, eq43_e581_d_n2, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq46_e608, eq46_e608_d_n18,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq46_e605);
        (eq46_e606, (eq46_e603 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e608;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq46_value),
            18,
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq47_e616);
        (eq47_e617, (eq47_e614 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e619;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq47_value),
            13,
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq52_e658, eq52_e658_d_n17,) = {
    if ((locals.var_guard1225 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e655);
        (eq52_e656, (eq52_e653 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e658;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq52_value),
            17,
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq54_e674, eq54_e674_d_n0, eq54_e674_d_n2, eq54_e674_d_n6, eq54_e674_d_n7, eq54_e674_d_n10, eq54_e674_d_n11, eq54_e674_d_n12, eq54_e674_d_n17,) = {
    if (locals.var_guard1225 == 0.0) {
        let eq54_e671: f64 = (locals.var_igidl + locals.var_isub);
        let eq54_e671_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq54_e671_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq54_e671_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq54_e671_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq54_e671_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq54_e671_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq54_e671_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq54_e671_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq54_e672: f64 = (p.p50 * eq54_e671);
        let eq54_e672_d_n0: f64 = (p.p50 * eq54_e671_d_n0);
        let eq54_e672_d_n2: f64 = (p.p50 * eq54_e671_d_n2);
        let eq54_e672_d_n6: f64 = (p.p50 * eq54_e671_d_n6);
        let eq54_e672_d_n7: f64 = (p.p50 * eq54_e671_d_n7);
        let eq54_e672_d_n10: f64 = (p.p50 * eq54_e671_d_n10);
        let eq54_e672_d_n11: f64 = (p.p50 * eq54_e671_d_n11);
        let eq54_e672_d_n12: f64 = (p.p50 * eq54_e671_d_n12);
        let eq54_e672_d_n17: f64 = (p.p50 * eq54_e671_d_n17);
        (eq54_e672, eq54_e672_d_n0, eq54_e672_d_n2, eq54_e672_d_n6, eq54_e672_d_n7, eq54_e672_d_n10, eq54_e672_d_n11, eq54_e672_d_n12, eq54_e672_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e674;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq54_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq54_e674_d_n0), multiplicity * (eq54_e674_d_n2), multiplicity * (eq54_e674_d_n6), multiplicity * (eq54_e674_d_n7), multiplicity * (eq54_e674_d_n10), multiplicity * (eq54_e674_d_n11), multiplicity * (eq54_e674_d_n12), multiplicity * (eq54_e674_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq55_e683, eq55_e683_d_n0, eq55_e683_d_n2, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n17,) = {
    if (locals.var_guard1225 == 0.0) {
        let eq55_e680: f64 = (locals.var_igisl + locals.var_isubs);
        let eq55_e680_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq55_e680_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq55_e680_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq55_e680_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq55_e680_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq55_e680_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq55_e680_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq55_e680_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq55_e681: f64 = (p.p50 * eq55_e680);
        let eq55_e681_d_n0: f64 = (p.p50 * eq55_e680_d_n0);
        let eq55_e681_d_n2: f64 = (p.p50 * eq55_e680_d_n2);
        let eq55_e681_d_n6: f64 = (p.p50 * eq55_e680_d_n6);
        let eq55_e681_d_n7: f64 = (p.p50 * eq55_e680_d_n7);
        let eq55_e681_d_n10: f64 = (p.p50 * eq55_e680_d_n10);
        let eq55_e681_d_n11: f64 = (p.p50 * eq55_e680_d_n11);
        let eq55_e681_d_n12: f64 = (p.p50 * eq55_e680_d_n12);
        let eq55_e681_d_n17: f64 = (p.p50 * eq55_e680_d_n17);
        (eq55_e681, eq55_e681_d_n0, eq55_e681_d_n2, eq55_e681_d_n6, eq55_e681_d_n7, eq55_e681_d_n10, eq55_e681_d_n11, eq55_e681_d_n12, eq55_e681_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e683;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq55_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq55_e683_d_n0), multiplicity * (eq55_e683_d_n2), multiplicity * (eq55_e683_d_n6), multiplicity * (eq55_e683_d_n7), multiplicity * (eq55_e683_d_n10), multiplicity * (eq55_e683_d_n11), multiplicity * (eq55_e683_d_n12), multiplicity * (eq55_e683_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq59_e716, eq59_e716_d_n17,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq59_e713);
        (eq59_e714, (eq59_e711 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e716;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq67_e784, eq67_e784_d_n15,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq67_e781);
        (eq67_e782, (eq67_e779 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e784;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq67_value),
            15,
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq68_e793);
        (eq68_e794, (eq68_e791 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e796;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq68_value),
            16,
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq69_e805);
        (eq69_e806, (eq69_e803 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e808;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq69_value),
            13,
            multiplicity * (eq69_e808_d_n13),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let eq10_e359_q: f64 = locals.var_qg;
        let eq10_e360: f64 = (p.p50 * locals.var_qg);
        let eq10_e360_d_n0: f64 = (p.p50 * locals.var_qg_dn0);
        let eq10_e360_d_n2: f64 = (p.p50 * locals.var_qg_dn2);
        let eq10_e360_d_n6: f64 = (p.p50 * locals.var_qg_dn6);
        let eq10_e360_d_n7: f64 = (p.p50 * locals.var_qg_dn7);
        let eq10_e360_d_n10: f64 = (p.p50 * locals.var_qg_dn10);
        let eq10_e360_d_n11: f64 = (p.p50 * locals.var_qg_dn11);
        let eq10_e360_d_n12: f64 = (p.p50 * locals.var_qg_dn12);
        let eq10_e360_d_n13: f64 = (p.p50 * locals.var_qg_dn13);
        let eq10_e360_d_n15: f64 = (p.p50 * locals.var_qg_dn15);
        let eq10_e360_d_n16: f64 = (p.p50 * locals.var_qg_dn16);
        let eq10_e360_d_n17: f64 = (p.p50 * locals.var_qg_dn17);
        let eq10_e360_d_n18: f64 = (p.p50 * locals.var_qg_dn18);
        let eq10_e360_q: f64 = (p.p50 * eq10_e359_q);
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e360_d_n0, 0.0, eq10_e360_d_n2, 0.0, 0.0, 0.0, eq10_e360_d_n6, eq10_e360_d_n7, 0.0, 0.0, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, 0.0, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e363_q: f64 = locals.var_qd;
        let eq11_e364: f64 = (p.p50 * locals.var_qd);
        let eq11_e364_d_n0: f64 = (p.p50 * locals.var_qd_dn0);
        let eq11_e364_d_n2: f64 = (p.p50 * locals.var_qd_dn2);
        let eq11_e364_d_n6: f64 = (p.p50 * locals.var_qd_dn6);
        let eq11_e364_d_n7: f64 = (p.p50 * locals.var_qd_dn7);
        let eq11_e364_d_n10: f64 = (p.p50 * locals.var_qd_dn10);
        let eq11_e364_d_n11: f64 = (p.p50 * locals.var_qd_dn11);
        let eq11_e364_d_n12: f64 = (p.p50 * locals.var_qd_dn12);
        let eq11_e364_d_n13: f64 = (p.p50 * locals.var_qd_dn13);
        let eq11_e364_d_n15: f64 = (p.p50 * locals.var_qd_dn15);
        let eq11_e364_d_n16: f64 = (p.p50 * locals.var_qd_dn16);
        let eq11_e364_d_n17: f64 = (p.p50 * locals.var_qd_dn17);
        let eq11_e364_d_n18: f64 = (p.p50 * locals.var_qd_dn18);
        let eq11_e364_q: f64 = (p.p50 * eq11_e363_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e364_d_n0, 0.0, eq11_e364_d_n2, 0.0, 0.0, 0.0, eq11_e364_d_n6, eq11_e364_d_n7, 0.0, 0.0, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, 0.0, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e367_q: f64 = locals.var_qb;
        let eq12_e368: f64 = (p.p50 * locals.var_qb);
        let eq12_e368_d_n0: f64 = (p.p50 * locals.var_qb_dn0);
        let eq12_e368_d_n2: f64 = (p.p50 * locals.var_qb_dn2);
        let eq12_e368_d_n6: f64 = (p.p50 * locals.var_qb_dn6);
        let eq12_e368_d_n7: f64 = (p.p50 * locals.var_qb_dn7);
        let eq12_e368_d_n10: f64 = (p.p50 * locals.var_qb_dn10);
        let eq12_e368_d_n11: f64 = (p.p50 * locals.var_qb_dn11);
        let eq12_e368_d_n12: f64 = (p.p50 * locals.var_qb_dn12);
        let eq12_e368_d_n13: f64 = (p.p50 * locals.var_qb_dn13);
        let eq12_e368_d_n15: f64 = (p.p50 * locals.var_qb_dn15);
        let eq12_e368_d_n16: f64 = (p.p50 * locals.var_qb_dn16);
        let eq12_e368_d_n17: f64 = (p.p50 * locals.var_qb_dn17);
        let eq12_e368_d_n18: f64 = (p.p50 * locals.var_qb_dn18);
        let eq12_e368_q: f64 = (p.p50 * eq12_e367_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e368_d_n0, 0.0, eq12_e368_d_n2, 0.0, 0.0, 0.0, eq12_e368_d_n6, eq12_e368_d_n7, 0.0, 0.0, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, 0.0, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e397: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq18_e398_q: f64 = eq18_e397;
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e397_d_n0, 0.0, eq18_e397_d_n2, 0.0, 0.0, 0.0, eq18_e397_d_n6, eq18_e397_d_n7, 0.0, 0.0, eq18_e397_d_n10, eq18_e397_d_n11, eq18_e397_d_n12, eq18_e397_d_n13, locals.var_sigrat_s, eq18_e397_d_n15, eq18_e397_d_n16, eq18_e397_d_n17, eq18_e397_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e401: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq19_e402_q: f64 = eq19_e401;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e401_d_n0, 0.0, eq19_e401_d_n2, 0.0, 0.0, 0.0, eq19_e401_d_n6, eq19_e401_d_n7, 0.0, 0.0, eq19_e401_d_n10, eq19_e401_d_n11, eq19_e401_d_n12, eq19_e401_d_n13, locals.var_sigrat_d, eq19_e401_d_n15, eq19_e401_d_n16, eq19_e401_d_n17, eq19_e401_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e483, eq30_e483_d_n10, eq30_e483_q,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq30_e480: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e481_q: f64 = eq30_e480;
        (eq30_e480, locals.var_cthe, eq30_e481_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e483_d_n10),
        );
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n2, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n17, eq34_e515_q, eq34_e515_q_d_n0, eq34_e515_q_d_n2, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq34_e511_q: f64 = locals.var_qbs;
        let eq34_e512: f64 = (locals.var_ibs + locals.var_qbs);
        let eq34_e512_d_n0: f64 = (locals.var_ibs_dn0 + locals.var_qbs_dn0);
        let eq34_e512_d_n2: f64 = (locals.var_ibs_dn2 + locals.var_qbs_dn2);
        let eq34_e512_d_n6: f64 = (locals.var_ibs_dn6 + locals.var_qbs_dn6);
        let eq34_e512_d_n7: f64 = (locals.var_ibs_dn7 + locals.var_qbs_dn7);
        let eq34_e512_d_n10: f64 = (locals.var_ibs_dn10 + locals.var_qbs_dn10);
        let eq34_e512_d_n11: f64 = (locals.var_ibs_dn11 + locals.var_qbs_dn11);
        let eq34_e512_d_n12: f64 = (locals.var_ibs_dn12 + locals.var_qbs_dn12);
        let eq34_e512_d_n17: f64 = (locals.var_ibs_dn17 + locals.var_qbs_dn17);
        let eq34_e512_q: f64 = eq34_e511_q;
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        let eq34_e513_q: f64 = (p.p50 * eq34_e512_q);
        let eq34_e513_q_d_n0: f64 = (p.p50 * locals.var_qbs_dn0);
        let eq34_e513_q_d_n2: f64 = (p.p50 * locals.var_qbs_dn2);
        let eq34_e513_q_d_n6: f64 = (p.p50 * locals.var_qbs_dn6);
        let eq34_e513_q_d_n7: f64 = (p.p50 * locals.var_qbs_dn7);
        let eq34_e513_q_d_n10: f64 = (p.p50 * locals.var_qbs_dn10);
        let eq34_e513_q_d_n11: f64 = (p.p50 * locals.var_qbs_dn11);
        let eq34_e513_q_d_n12: f64 = (p.p50 * locals.var_qbs_dn12);
        let eq34_e513_q_d_n17: f64 = (p.p50 * locals.var_qbs_dn17);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n2, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n17, eq34_e513_q, eq34_e513_q_d_n0, eq34_e513_q_d_n2, eq34_e513_q_d_n6, eq34_e513_q_d_n7, eq34_e513_q_d_n10, eq34_e513_q_d_n11, eq34_e513_q_d_n12, eq34_e513_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e515_q_d_n0, 0.0, eq34_e515_q_d_n2, 0.0, 0.0, 0.0, eq34_e515_q_d_n6, eq34_e515_q_d_n7, 0.0, 0.0, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq34_e515_q_d_n17, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n2, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n17, eq35_e524_q, eq35_e524_q_d_n0, eq35_e524_q_d_n2, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n17,) = {
    if (locals.var_guard1225 != 0.0) {
        let eq35_e520_q: f64 = locals.var_qbd;
        let eq35_e521: f64 = (locals.var_ibd + locals.var_qbd);
        let eq35_e521_d_n0: f64 = (locals.var_ibd_dn0 + locals.var_qbd_dn0);
        let eq35_e521_d_n2: f64 = (locals.var_ibd_dn2 + locals.var_qbd_dn2);
        let eq35_e521_d_n6: f64 = (locals.var_ibd_dn6 + locals.var_qbd_dn6);
        let eq35_e521_d_n7: f64 = (locals.var_ibd_dn7 + locals.var_qbd_dn7);
        let eq35_e521_d_n10: f64 = (locals.var_ibd_dn10 + locals.var_qbd_dn10);
        let eq35_e521_d_n11: f64 = (locals.var_ibd_dn11 + locals.var_qbd_dn11);
        let eq35_e521_d_n12: f64 = (locals.var_ibd_dn12 + locals.var_qbd_dn12);
        let eq35_e521_d_n17: f64 = (locals.var_ibd_dn17 + locals.var_qbd_dn17);
        let eq35_e521_q: f64 = eq35_e520_q;
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        let eq35_e522_q: f64 = (p.p50 * eq35_e521_q);
        let eq35_e522_q_d_n0: f64 = (p.p50 * locals.var_qbd_dn0);
        let eq35_e522_q_d_n2: f64 = (p.p50 * locals.var_qbd_dn2);
        let eq35_e522_q_d_n6: f64 = (p.p50 * locals.var_qbd_dn6);
        let eq35_e522_q_d_n7: f64 = (p.p50 * locals.var_qbd_dn7);
        let eq35_e522_q_d_n10: f64 = (p.p50 * locals.var_qbd_dn10);
        let eq35_e522_q_d_n11: f64 = (p.p50 * locals.var_qbd_dn11);
        let eq35_e522_q_d_n12: f64 = (p.p50 * locals.var_qbd_dn12);
        let eq35_e522_q_d_n17: f64 = (p.p50 * locals.var_qbd_dn17);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n2, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n17, eq35_e522_q, eq35_e522_q_d_n0, eq35_e522_q_d_n2, eq35_e522_q_d_n6, eq35_e522_q_d_n7, eq35_e522_q_d_n10, eq35_e522_q_d_n11, eq35_e522_q_d_n12, eq35_e522_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e524_q_d_n0, 0.0, eq35_e524_q_d_n2, 0.0, 0.0, 0.0, eq35_e524_q_d_n6, eq35_e524_q_d_n7, 0.0, 0.0, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e524_q_d_n17, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e608, eq46_e608_d_n18, eq46_e608_q,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606_q: f64 = eq46_e605;
        (eq46_e605, eq46_e603, eq46_e606_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13, eq47_e619_q,) = {
    if ((locals.var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq52_e658, eq52_e658_d_n17, eq52_e658_q,) = {
    if ((locals.var_guard1225 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656_q: f64 = eq52_e655;
        (eq52_e655, eq52_e653, eq52_e656_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq59_e716, eq59_e716_d_n17, eq59_e716_q,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714_q: f64 = eq59_e713;
        (eq59_e713, eq59_e711, eq59_e714_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq67_e784, eq67_e784_d_n15, eq67_e784_q,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782_q: f64 = eq67_e781;
        (eq67_e781, eq67_e779, eq67_e782_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16, eq68_e796_q,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794_q: f64 = eq68_e793;
        (eq68_e793, eq68_e791, eq68_e794_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13, eq69_e808_q,) = {
    if ((locals.var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806_q: f64 = eq69_e805;
        (eq69_e805, eq69_e803, eq69_e806_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq69_e808_d_n13),
        );
    }
}
