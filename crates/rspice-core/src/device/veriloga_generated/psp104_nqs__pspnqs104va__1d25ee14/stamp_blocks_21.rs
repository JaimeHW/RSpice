#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_168(
        locals: &mut StampLocals,
    ) {
        let assign81850_e122062: f64 = (locals.var_nqs_y0).abs();
        let assign81850_e122064: f64 = if assign81850_e122062 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign81850_e122064;
        locals.var_guard2230_rv = 0.0;

        let (assign81860_e122090, assign81860_e122090_d_n5, assign81860_e122090_d_n6, assign81860_e122090_d_n7, assign81860_e122090_d_n8, assign81860_e122090_d_n12, assign81860_e122090_d_n13, assign81860_e122090_d_n14, assign81860_e122090_d_n15, assign81860_e122090_d_n16, assign81860_e122090_d_n17, assign81860_e122090_d_n18, assign81860_e122090_d_n19, assign81860_e122090_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) && (locals.var_guard2230 != 0.0)) {
        let assign81860_e122088: f64 = (locals.var_nqs_y0).exp();
        (assign81860_e122088, (assign81860_e122088 * locals.var_nqs_y0_dn5), (assign81860_e122088 * locals.var_nqs_y0_dn6), (assign81860_e122088 * locals.var_nqs_y0_dn7), (assign81860_e122088 * locals.var_nqs_y0_dn8), (assign81860_e122088 * locals.var_nqs_y0_dn12), (assign81860_e122088 * locals.var_nqs_y0_dn13), (assign81860_e122088 * locals.var_nqs_y0_dn14), (assign81860_e122088 * locals.var_nqs_y0_dn15), (assign81860_e122088 * locals.var_nqs_y0_dn16), (assign81860_e122088 * locals.var_nqs_y0_dn17), (assign81860_e122088 * locals.var_nqs_y0_dn18), (assign81860_e122088 * locals.var_nqs_y0_dn19), (assign81860_e122088 * locals.var_nqs_y0_dn20),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign81860_e122090;
        locals.var_nqs_d0_dn5 = assign81860_e122090_d_n5;
        locals.var_nqs_d0_dn6 = assign81860_e122090_d_n6;
        locals.var_nqs_d0_dn7 = assign81860_e122090_d_n7;
        locals.var_nqs_d0_dn8 = assign81860_e122090_d_n8;
        locals.var_nqs_d0_dn12 = assign81860_e122090_d_n12;
        locals.var_nqs_d0_dn13 = assign81860_e122090_d_n13;
        locals.var_nqs_d0_dn14 = assign81860_e122090_d_n14;
        locals.var_nqs_d0_dn15 = assign81860_e122090_d_n15;
        locals.var_nqs_d0_dn16 = assign81860_e122090_d_n16;
        locals.var_nqs_d0_dn17 = assign81860_e122090_d_n17;
        locals.var_nqs_d0_dn18 = assign81860_e122090_d_n18;
        locals.var_nqs_d0_dn19 = assign81860_e122090_d_n19;
        locals.var_nqs_d0_dn20 = assign81860_e122090_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let assign81870_e122093: f64 = if locals.var_nqs_y0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign81870_e122093;
        locals.var_guard2231_rv = 0.0;

        let (assign81880_e122146, assign81880_e122146_d_n5, assign81880_e122146_d_n6, assign81880_e122146_d_n7, assign81880_e122146_d_n8, assign81880_e122146_d_n12, assign81880_e122146_d_n13, assign81880_e122146_d_n14, assign81880_e122146_d_n15, assign81880_e122146_d_n16, assign81880_e122146_d_n17, assign81880_e122146_d_n18, assign81880_e122146_d_n19, assign81880_e122146_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) && (locals.var_guard2230 == 0.0)) && (locals.var_guard2231 != 0.0)) {
        let assign81880_e122122: f64 = (-230.25850929940458);
        let assign81880_e122124: f64 = (assign81880_e122122 - locals.var_nqs_y0);
        let assign81880_e122128: f64 = (-230.25850929940458);
        let assign81880_e122130: f64 = (assign81880_e122128 - locals.var_nqs_y0);
        let assign81880_e122133: f64 = (-230.25850929940458);
        let assign81880_e122135: f64 = (assign81880_e122133 - locals.var_nqs_y0);
        let assign81880_e122137: f64 = (assign81880_e122135 * 0.3333333333333333);
        let assign81880_e122138: f64 = (1.0 + assign81880_e122137);
        let assign81880_e122139: f64 = (assign81880_e122130 * assign81880_e122138);
        let assign81880_e122140: f64 = (0.5 * assign81880_e122139);
        let assign81880_e122141: f64 = (1.0 + assign81880_e122140);
        let assign81880_e122142: f64 = (assign81880_e122124 * assign81880_e122141);
        let assign81880_e122143: f64 = (1.0 + assign81880_e122142);
        let assign81880_e122144: f64 = (1e-100 / assign81880_e122143);
        (assign81880_e122144, (-((1e-100 * (((-locals.var_nqs_y0_dn5) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn5) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn5) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn6) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn6) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn6) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn7) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn7) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn7) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn8) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn8) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn8) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn12) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn12) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn12) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn13) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn13) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn13) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn14) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn14) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn14) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn15) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn15) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn15) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn16) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn16) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn16) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn17) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn17) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn17) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn18) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn18) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn18) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn19) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn19) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn19) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))), (-((1e-100 * (((-locals.var_nqs_y0_dn20) * assign81880_e122141) + (assign81880_e122124 * (0.5 * (((-locals.var_nqs_y0_dn20) * assign81880_e122138) + (assign81880_e122130 * ((-locals.var_nqs_y0_dn20) * 0.3333333333333333))))))) / (assign81880_e122143 * assign81880_e122143))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign81880_e122146;
        locals.var_nqs_d0_dn5 = assign81880_e122146_d_n5;
        locals.var_nqs_d0_dn6 = assign81880_e122146_d_n6;
        locals.var_nqs_d0_dn7 = assign81880_e122146_d_n7;
        locals.var_nqs_d0_dn8 = assign81880_e122146_d_n8;
        locals.var_nqs_d0_dn12 = assign81880_e122146_d_n12;
        locals.var_nqs_d0_dn13 = assign81880_e122146_d_n13;
        locals.var_nqs_d0_dn14 = assign81880_e122146_d_n14;
        locals.var_nqs_d0_dn15 = assign81880_e122146_d_n15;
        locals.var_nqs_d0_dn16 = assign81880_e122146_d_n16;
        locals.var_nqs_d0_dn17 = assign81880_e122146_d_n17;
        locals.var_nqs_d0_dn18 = assign81880_e122146_d_n18;
        locals.var_nqs_d0_dn19 = assign81880_e122146_d_n19;
        locals.var_nqs_d0_dn20 = assign81880_e122146_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign81890_e122197, assign81890_e122197_d_n5, assign81890_e122197_d_n6, assign81890_e122197_d_n7, assign81890_e122197_d_n8, assign81890_e122197_d_n12, assign81890_e122197_d_n13, assign81890_e122197_d_n14, assign81890_e122197_d_n15, assign81890_e122197_d_n16, assign81890_e122197_d_n17, assign81890_e122197_d_n18, assign81890_e122197_d_n19, assign81890_e122197_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) && (locals.var_guard2230 == 0.0)) && (locals.var_guard2231 == 0.0)) {
        let assign81890_e122177: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign81890_e122182: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign81890_e122186: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign81890_e122188: f64 = (assign81890_e122186 * 0.3333333333333333);
        let assign81890_e122189: f64 = (1.0 + assign81890_e122188);
        let assign81890_e122190: f64 = (assign81890_e122182 * assign81890_e122189);
        let assign81890_e122191: f64 = (0.5 * assign81890_e122190);
        let assign81890_e122192: f64 = (1.0 + assign81890_e122191);
        let assign81890_e122193: f64 = (assign81890_e122177 * assign81890_e122192);
        let assign81890_e122194: f64 = (1.0 + assign81890_e122193);
        let assign81890_e122195: f64 = (1e100 * assign81890_e122194);
        (assign81890_e122195, (1e100 * ((locals.var_nqs_y0_dn5 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn5 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn6 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn6 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn7 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn7 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn8 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn8 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn12 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn12 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn12 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn13 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn13 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn13 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn14 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn14 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn14 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn15 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn15 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn15 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn16 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn16 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn16 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn17 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn17 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn17 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn18 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn18 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn18 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn19 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn19 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn19 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn20 * assign81890_e122192) + (assign81890_e122177 * (0.5 * ((locals.var_nqs_y0_dn20 * assign81890_e122189) + (assign81890_e122182 * (locals.var_nqs_y0_dn20 * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign81890_e122197;
        locals.var_nqs_d0_dn5 = assign81890_e122197_d_n5;
        locals.var_nqs_d0_dn6 = assign81890_e122197_d_n6;
        locals.var_nqs_d0_dn7 = assign81890_e122197_d_n7;
        locals.var_nqs_d0_dn8 = assign81890_e122197_d_n8;
        locals.var_nqs_d0_dn12 = assign81890_e122197_d_n12;
        locals.var_nqs_d0_dn13 = assign81890_e122197_d_n13;
        locals.var_nqs_d0_dn14 = assign81890_e122197_d_n14;
        locals.var_nqs_d0_dn15 = assign81890_e122197_d_n15;
        locals.var_nqs_d0_dn16 = assign81890_e122197_d_n16;
        locals.var_nqs_d0_dn17 = assign81890_e122197_d_n17;
        locals.var_nqs_d0_dn18 = assign81890_e122197_d_n18;
        locals.var_nqs_d0_dn19 = assign81890_e122197_d_n19;
        locals.var_nqs_d0_dn20 = assign81890_e122197_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign81900_e122226, assign81900_e122226_d_n5, assign81900_e122226_d_n6, assign81900_e122226_d_n7, assign81900_e122226_d_n8, assign81900_e122226_d_n12, assign81900_e122226_d_n13, assign81900_e122226_d_n14, assign81900_e122226_d_n15, assign81900_e122226_d_n16, assign81900_e122226_d_n17, assign81900_e122226_d_n18, assign81900_e122226_d_n19, assign81900_e122226_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81900_e122221: f64 = (locals.var_gp2 * locals.var_nqs_d0);
        let assign81900_e122223: f64 = (assign81900_e122221 * 0.5);
        let assign81900_e122224: f64 = (1.0 - assign81900_e122223);
        (assign81900_e122224, (-(((locals.var_gp2_dn5 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn5)) * 0.5)), (-(((locals.var_gp2_dn6 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn6)) * 0.5)), (-(((locals.var_gp2_dn7 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn7)) * 0.5)), (-(((locals.var_gp2_dn8 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn8)) * 0.5)), (-(((locals.var_gp2_dn12 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn12)) * 0.5)), (-(((locals.var_gp2_dn13 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn13)) * 0.5)), (-(((locals.var_gp2_dn14 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn14)) * 0.5)), (-(((locals.var_gp2_dn15 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn15)) * 0.5)), (-(((locals.var_gp2_dn16 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn16)) * 0.5)), (-(((locals.var_gp2_dn17 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn17)) * 0.5)), (-(((locals.var_gp2_dn18 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn18)) * 0.5)), (-(((locals.var_gp2_dn19 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn19)) * 0.5)), (-(((locals.var_gp2_dn20 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn20)) * 0.5)),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign81900_e122226;
        locals.var_nqs_xi_dn5 = assign81900_e122226_d_n5;
        locals.var_nqs_xi_dn6 = assign81900_e122226_d_n6;
        locals.var_nqs_xi_dn7 = assign81900_e122226_d_n7;
        locals.var_nqs_xi_dn8 = assign81900_e122226_d_n8;
        locals.var_nqs_xi_dn12 = assign81900_e122226_d_n12;
        locals.var_nqs_xi_dn13 = assign81900_e122226_d_n13;
        locals.var_nqs_xi_dn14 = assign81900_e122226_d_n14;
        locals.var_nqs_xi_dn15 = assign81900_e122226_d_n15;
        locals.var_nqs_xi_dn16 = assign81900_e122226_d_n16;
        locals.var_nqs_xi_dn17 = assign81900_e122226_d_n17;
        locals.var_nqs_xi_dn18 = assign81900_e122226_d_n18;
        locals.var_nqs_xi_dn19 = assign81900_e122226_d_n19;
        locals.var_nqs_xi_dn20 = assign81900_e122226_d_n20;
        locals.var_nqs_xi_rv = 0.0;

        let (assign81910_e122259, assign81910_e122259_d_n5, assign81910_e122259_d_n6, assign81910_e122259_d_n7, assign81910_e122259_d_n8, assign81910_e122259_d_n12, assign81910_e122259_d_n13, assign81910_e122259_d_n14, assign81910_e122259_d_n15, assign81910_e122259_d_n16, assign81910_e122259_d_n17, assign81910_e122259_d_n18, assign81910_e122259_d_n19, assign81910_e122259_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81910_e122250: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign81910_e122251: f64 = (2.0 * assign81910_e122250);
        let assign81910_e122255: f64 = (locals.var_nqs_d0 - 1.0);
        let assign81910_e122256: f64 = (locals.var_gp2 * assign81910_e122255);
        let assign81910_e122257: f64 = (assign81910_e122251 + assign81910_e122256);
        (assign81910_e122257, ((2.0 * (locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5)) + ((locals.var_gp2_dn5 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn5))), ((2.0 * (locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6)) + ((locals.var_gp2_dn6 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn6))), ((2.0 * (locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7)) + ((locals.var_gp2_dn7 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn7))), ((2.0 * (locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8)) + ((locals.var_gp2_dn8 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn8))), ((2.0 * (locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12)) + ((locals.var_gp2_dn12 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn12))), ((2.0 * (locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13)) + ((locals.var_gp2_dn13 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn13))), ((2.0 * (locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14)) + ((locals.var_gp2_dn14 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn14))), ((2.0 * (locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15)) + ((locals.var_gp2_dn15 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn15))), ((2.0 * (locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16)) + ((locals.var_gp2_dn16 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn16))), ((2.0 * (locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17)) + ((locals.var_gp2_dn17 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn17))), ((2.0 * (locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18)) + ((locals.var_gp2_dn18 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn18))), ((2.0 * (locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19)) + ((locals.var_gp2_dn19 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn19))), ((2.0 * (locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20)) + ((locals.var_gp2_dn20 * assign81910_e122255) + (locals.var_gp2 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign81910_e122259;
        locals.var_nqs_p_dn5 = assign81910_e122259_d_n5;
        locals.var_nqs_p_dn6 = assign81910_e122259_d_n6;
        locals.var_nqs_p_dn7 = assign81910_e122259_d_n7;
        locals.var_nqs_p_dn8 = assign81910_e122259_d_n8;
        locals.var_nqs_p_dn12 = assign81910_e122259_d_n12;
        locals.var_nqs_p_dn13 = assign81910_e122259_d_n13;
        locals.var_nqs_p_dn14 = assign81910_e122259_d_n14;
        locals.var_nqs_p_dn15 = assign81910_e122259_d_n15;
        locals.var_nqs_p_dn16 = assign81910_e122259_d_n16;
        locals.var_nqs_p_dn17 = assign81910_e122259_d_n17;
        locals.var_nqs_p_dn18 = assign81910_e122259_d_n18;
        locals.var_nqs_p_dn19 = assign81910_e122259_d_n19;
        locals.var_nqs_p_dn20 = assign81910_e122259_d_n20;
        locals.var_nqs_p_rv = 0.0;

        let (assign81920_e122296, assign81920_e122296_d_n5, assign81920_e122296_d_n6, assign81920_e122296_d_n7, assign81920_e122296_d_n8, assign81920_e122296_d_n12, assign81920_e122296_d_n13, assign81920_e122296_d_n14, assign81920_e122296_d_n15, assign81920_e122296_d_n16, assign81920_e122296_d_n17, assign81920_e122296_d_n18, assign81920_e122296_d_n19, assign81920_e122296_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81920_e122282: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign81920_e122285: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign81920_e122286: f64 = (assign81920_e122282 * assign81920_e122285);
        let assign81920_e122290: f64 = (locals.var_nqs_y0 + 1.0);
        let assign81920_e122292: f64 = (assign81920_e122290 - locals.var_nqs_d0);
        let assign81920_e122293: f64 = (locals.var_gp2 * assign81920_e122292);
        let assign81920_e122294: f64 = (assign81920_e122286 + assign81920_e122293);
        (assign81920_e122294, ((((locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5))) + ((locals.var_gp2_dn5 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn5 - locals.var_nqs_d0_dn5)))), ((((locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6))) + ((locals.var_gp2_dn6 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn6 - locals.var_nqs_d0_dn6)))), ((((locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7))) + ((locals.var_gp2_dn7 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn7 - locals.var_nqs_d0_dn7)))), ((((locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8))) + ((locals.var_gp2_dn8 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn8 - locals.var_nqs_d0_dn8)))), ((((locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12))) + ((locals.var_gp2_dn12 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn12 - locals.var_nqs_d0_dn12)))), ((((locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13))) + ((locals.var_gp2_dn13 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn13 - locals.var_nqs_d0_dn13)))), ((((locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14))) + ((locals.var_gp2_dn14 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn14 - locals.var_nqs_d0_dn14)))), ((((locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15))) + ((locals.var_gp2_dn15 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn15 - locals.var_nqs_d0_dn15)))), ((((locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16))) + ((locals.var_gp2_dn16 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn16 - locals.var_nqs_d0_dn16)))), ((((locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17))) + ((locals.var_gp2_dn17 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn17 - locals.var_nqs_d0_dn17)))), ((((locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18))) + ((locals.var_gp2_dn18 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn18 - locals.var_nqs_d0_dn18)))), ((((locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19))) + ((locals.var_gp2_dn19 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn19 - locals.var_nqs_d0_dn19)))), ((((locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20) * assign81920_e122285) + (assign81920_e122282 * (locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20))) + ((locals.var_gp2_dn20 * assign81920_e122292) + (locals.var_gp2 * (locals.var_nqs_y0_dn20 - locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign81920_e122296;
        locals.var_nqs_q_dn5 = assign81920_e122296_d_n5;
        locals.var_nqs_q_dn6 = assign81920_e122296_d_n6;
        locals.var_nqs_q_dn7 = assign81920_e122296_d_n7;
        locals.var_nqs_q_dn8 = assign81920_e122296_d_n8;
        locals.var_nqs_q_dn12 = assign81920_e122296_d_n12;
        locals.var_nqs_q_dn13 = assign81920_e122296_d_n13;
        locals.var_nqs_q_dn14 = assign81920_e122296_d_n14;
        locals.var_nqs_q_dn15 = assign81920_e122296_d_n15;
        locals.var_nqs_q_dn16 = assign81920_e122296_d_n16;
        locals.var_nqs_q_dn17 = assign81920_e122296_d_n17;
        locals.var_nqs_q_dn18 = assign81920_e122296_d_n18;
        locals.var_nqs_q_dn19 = assign81920_e122296_d_n19;
        locals.var_nqs_q_dn20 = assign81920_e122296_d_n20;
        locals.var_nqs_q_rv = 0.0;

        let (assign81930_e122327, assign81930_e122327_d_n5, assign81930_e122327_d_n6, assign81930_e122327_d_n7, assign81930_e122327_d_n8, assign81930_e122327_d_n12, assign81930_e122327_d_n13, assign81930_e122327_d_n14, assign81930_e122327_d_n15, assign81930_e122327_d_n16, assign81930_e122327_d_n17, assign81930_e122327_d_n18, assign81930_e122327_d_n19, assign81930_e122327_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81930_e122319: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign81930_e122322: f64 = (4.0 * locals.var_nqs_xi);
        let assign81930_e122324: f64 = (assign81930_e122322 * locals.var_nqs_q);
        let assign81930_e122325: f64 = (assign81930_e122319 - assign81930_e122324);
        (assign81930_e122325, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign81930_e122322 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign81930_e122327;
        locals.var_nqs_temp_dn5 = assign81930_e122327_d_n5;
        locals.var_nqs_temp_dn6 = assign81930_e122327_d_n6;
        locals.var_nqs_temp_dn7 = assign81930_e122327_d_n7;
        locals.var_nqs_temp_dn8 = assign81930_e122327_d_n8;
        locals.var_nqs_temp_dn12 = assign81930_e122327_d_n12;
        locals.var_nqs_temp_dn13 = assign81930_e122327_d_n13;
        locals.var_nqs_temp_dn14 = assign81930_e122327_d_n14;
        locals.var_nqs_temp_dn15 = assign81930_e122327_d_n15;
        locals.var_nqs_temp_dn16 = assign81930_e122327_d_n16;
        locals.var_nqs_temp_dn17 = assign81930_e122327_d_n17;
        locals.var_nqs_temp_dn18 = assign81930_e122327_d_n18;
        locals.var_nqs_temp_dn19 = assign81930_e122327_d_n19;
        locals.var_nqs_temp_dn20 = assign81930_e122327_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign81940_e122357, assign81940_e122357_d_n5, assign81940_e122357_d_n6, assign81940_e122357_d_n7, assign81940_e122357_d_n8, assign81940_e122357_d_n12, assign81940_e122357_d_n13, assign81940_e122357_d_n14, assign81940_e122357_d_n15, assign81940_e122357_d_n16, assign81940_e122357_d_n17, assign81940_e122357_d_n18, assign81940_e122357_d_n19, assign81940_e122357_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81940_e122350: f64 = (2.0 * locals.var_nqs_q);
        let assign81940_e122353: f64 = (locals.var_nqs_temp).sqrt();
        let assign81940_e122354: f64 = (locals.var_nqs_p + assign81940_e122353);
        let assign81940_e122355: f64 = (assign81940_e122350 / assign81940_e122354);
        (assign81940_e122355, ((((2.0 * locals.var_nqs_q_dn5) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn6) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn7) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn8) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn12) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn13) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn14) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn15) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn16) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn17) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn18) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn19) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)), ((((2.0 * locals.var_nqs_q_dn20) * assign81940_e122354) - (assign81940_e122350 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign81940_e122353))))) / (assign81940_e122354 * assign81940_e122354)),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign81940_e122357;
        locals.var_nqs_w_dn5 = assign81940_e122357_d_n5;
        locals.var_nqs_w_dn6 = assign81940_e122357_d_n6;
        locals.var_nqs_w_dn7 = assign81940_e122357_d_n7;
        locals.var_nqs_w_dn8 = assign81940_e122357_d_n8;
        locals.var_nqs_w_dn12 = assign81940_e122357_d_n12;
        locals.var_nqs_w_dn13 = assign81940_e122357_d_n13;
        locals.var_nqs_w_dn14 = assign81940_e122357_d_n14;
        locals.var_nqs_w_dn15 = assign81940_e122357_d_n15;
        locals.var_nqs_w_dn16 = assign81940_e122357_d_n16;
        locals.var_nqs_w_dn17 = assign81940_e122357_d_n17;
        locals.var_nqs_w_dn18 = assign81940_e122357_d_n18;
        locals.var_nqs_w_dn19 = assign81940_e122357_d_n19;
        locals.var_nqs_w_dn20 = assign81940_e122357_d_n20;
        locals.var_nqs_w_rv = 0.0;

        let (assign81950_e122383, assign81950_e122383_d_n5, assign81950_e122383_d_n6, assign81950_e122383_d_n7, assign81950_e122383_d_n8, assign81950_e122383_d_n12, assign81950_e122383_d_n13, assign81950_e122383_d_n14, assign81950_e122383_d_n15, assign81950_e122383_d_n16, assign81950_e122383_d_n17, assign81950_e122383_d_n18, assign81950_e122383_d_n19, assign81950_e122383_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 != 0.0)) {
        let assign81950_e122380: f64 = (locals.var_nqs_y0 + locals.var_nqs_w);
        let assign81950_e122381: f64 = (-assign81950_e122380);
        (assign81950_e122381, (-(locals.var_nqs_y0_dn5 + locals.var_nqs_w_dn5)), (-(locals.var_nqs_y0_dn6 + locals.var_nqs_w_dn6)), (-(locals.var_nqs_y0_dn7 + locals.var_nqs_w_dn7)), (-(locals.var_nqs_y0_dn8 + locals.var_nqs_w_dn8)), (-(locals.var_nqs_y0_dn12 + locals.var_nqs_w_dn12)), (-(locals.var_nqs_y0_dn13 + locals.var_nqs_w_dn13)), (-(locals.var_nqs_y0_dn14 + locals.var_nqs_w_dn14)), (-(locals.var_nqs_y0_dn15 + locals.var_nqs_w_dn15)), (-(locals.var_nqs_y0_dn16 + locals.var_nqs_w_dn16)), (-(locals.var_nqs_y0_dn17 + locals.var_nqs_w_dn17)), (-(locals.var_nqs_y0_dn18 + locals.var_nqs_w_dn18)), (-(locals.var_nqs_y0_dn19 + locals.var_nqs_w_dn19)), (-(locals.var_nqs_y0_dn20 + locals.var_nqs_w_dn20)),)
    } else {
        (locals.var_temp8, locals.var_temp8_dn5, locals.var_temp8_dn6, locals.var_temp8_dn7, locals.var_temp8_dn8, locals.var_temp8_dn12, locals.var_temp8_dn13, locals.var_temp8_dn14, locals.var_temp8_dn15, locals.var_temp8_dn16, locals.var_temp8_dn17, locals.var_temp8_dn18, locals.var_temp8_dn19, locals.var_temp8_dn20,)
    }
};
        locals.var_temp8 = assign81950_e122383;
        locals.var_temp8_dn5 = assign81950_e122383_d_n5;
        locals.var_temp8_dn6 = assign81950_e122383_d_n6;
        locals.var_temp8_dn7 = assign81950_e122383_d_n7;
        locals.var_temp8_dn8 = assign81950_e122383_d_n8;
        locals.var_temp8_dn12 = assign81950_e122383_d_n12;
        locals.var_temp8_dn13 = assign81950_e122383_d_n13;
        locals.var_temp8_dn14 = assign81950_e122383_d_n14;
        locals.var_temp8_dn15 = assign81950_e122383_d_n15;
        locals.var_temp8_dn16 = assign81950_e122383_d_n16;
        locals.var_temp8_dn17 = assign81950_e122383_d_n17;
        locals.var_temp8_dn18 = assign81950_e122383_d_n18;
        locals.var_temp8_dn19 = assign81950_e122383_d_n19;
        locals.var_temp8_dn20 = assign81950_e122383_d_n20;
        locals.var_temp8_rv = 0.0;

        let (assign81960_e122413, assign81960_e122413_d_n5, assign81960_e122413_d_n6, assign81960_e122413_d_n7, assign81960_e122413_d_n8, assign81960_e122413_d_n12, assign81960_e122413_d_n13, assign81960_e122413_d_n14, assign81960_e122413_d_n15, assign81960_e122413_d_n16, assign81960_e122413_d_n17, assign81960_e122413_d_n18, assign81960_e122413_d_n19, assign81960_e122413_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign81960_e122409: f64 = (0.732464877560822 * locals.var_gp);
        let assign81960_e122410: f64 = (1.25 + assign81960_e122409);
        let assign81960_e122411: f64 = (1.0 / assign81960_e122410);
        (assign81960_e122411, (-((0.732464877560822 * locals.var_gp_dn5) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn6) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn7) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn8) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn12) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn13) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn14) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn15) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn16) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn17) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn18) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn19) / (assign81960_e122410 * assign81960_e122410))), (-((0.732464877560822 * locals.var_gp_dn20) / (assign81960_e122410 * assign81960_e122410))),)
    } else {
        (locals.var_nqs_xg1, locals.var_nqs_xg1_dn5, locals.var_nqs_xg1_dn6, locals.var_nqs_xg1_dn7, locals.var_nqs_xg1_dn8, locals.var_nqs_xg1_dn12, locals.var_nqs_xg1_dn13, locals.var_nqs_xg1_dn14, locals.var_nqs_xg1_dn15, locals.var_nqs_xg1_dn16, locals.var_nqs_xg1_dn17, locals.var_nqs_xg1_dn18, locals.var_nqs_xg1_dn19, locals.var_nqs_xg1_dn20,)
    }
};
        locals.var_nqs_xg1 = assign81960_e122413;
        locals.var_nqs_xg1_dn5 = assign81960_e122413_d_n5;
        locals.var_nqs_xg1_dn6 = assign81960_e122413_d_n6;
        locals.var_nqs_xg1_dn7 = assign81960_e122413_d_n7;
        locals.var_nqs_xg1_dn8 = assign81960_e122413_d_n8;
        locals.var_nqs_xg1_dn12 = assign81960_e122413_d_n12;
        locals.var_nqs_xg1_dn13 = assign81960_e122413_d_n13;
        locals.var_nqs_xg1_dn14 = assign81960_e122413_d_n14;
        locals.var_nqs_xg1_dn15 = assign81960_e122413_d_n15;
        locals.var_nqs_xg1_dn16 = assign81960_e122413_d_n16;
        locals.var_nqs_xg1_dn17 = assign81960_e122413_d_n17;
        locals.var_nqs_xg1_dn18 = assign81960_e122413_d_n18;
        locals.var_nqs_xg1_dn19 = assign81960_e122413_d_n19;
        locals.var_nqs_xg1_dn20 = assign81960_e122413_d_n20;
        locals.var_nqs_xg1_rv = 0.0;

        let (assign81970_e122445, assign81970_e122445_d_n5, assign81970_e122445_d_n6, assign81970_e122445_d_n7, assign81970_e122445_d_n8, assign81970_e122445_d_n12, assign81970_e122445_d_n13, assign81970_e122445_d_n14, assign81970_e122445_d_n15, assign81970_e122445_d_n16, assign81970_e122445_d_n17, assign81970_e122445_d_n18, assign81970_e122445_d_n19, assign81970_e122445_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign81970_e122437: f64 = (1.25 * locals.var_a_factrp);
        let assign81970_e122439: f64 = (assign81970_e122437 * locals.var_nqs_xg1);
        let assign81970_e122441: f64 = (assign81970_e122439 - 1.0);
        let assign81970_e122443: f64 = (assign81970_e122441 * locals.var_nqs_xg1);
        (assign81970_e122443, (((((1.25 * locals.var_a_factrp_dn5) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn5)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn5)), (((((1.25 * locals.var_a_factrp_dn6) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn6)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn6)), (((((1.25 * locals.var_a_factrp_dn7) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn7)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn7)), (((((1.25 * locals.var_a_factrp_dn8) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn8)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn8)), (((((1.25 * locals.var_a_factrp_dn12) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn12)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn12)), (((((1.25 * locals.var_a_factrp_dn13) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn13)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn13)), (((((1.25 * locals.var_a_factrp_dn14) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn14)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn14)), (((((1.25 * locals.var_a_factrp_dn15) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn15)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn15)), (((((1.25 * locals.var_a_factrp_dn16) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn16)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn16)), (((((1.25 * locals.var_a_factrp_dn17) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn17)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn17)), (((((1.25 * locals.var_a_factrp_dn18) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn18)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn18)), (((((1.25 * locals.var_a_factrp_dn19) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn19)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn19)), (((((1.25 * locals.var_a_factrp_dn20) * locals.var_nqs_xg1) + (assign81970_e122437 * locals.var_nqs_xg1_dn20)) * locals.var_nqs_xg1) + (assign81970_e122441 * locals.var_nqs_xg1_dn20)),)
    } else {
        (locals.var_nqs_a_fac, locals.var_nqs_a_fac_dn5, locals.var_nqs_a_fac_dn6, locals.var_nqs_a_fac_dn7, locals.var_nqs_a_fac_dn8, locals.var_nqs_a_fac_dn12, locals.var_nqs_a_fac_dn13, locals.var_nqs_a_fac_dn14, locals.var_nqs_a_fac_dn15, locals.var_nqs_a_fac_dn16, locals.var_nqs_a_fac_dn17, locals.var_nqs_a_fac_dn18, locals.var_nqs_a_fac_dn19, locals.var_nqs_a_fac_dn20,)
    }
};
        locals.var_nqs_a_fac = assign81970_e122445;
        locals.var_nqs_a_fac_dn5 = assign81970_e122445_d_n5;
        locals.var_nqs_a_fac_dn6 = assign81970_e122445_d_n6;
        locals.var_nqs_a_fac_dn7 = assign81970_e122445_d_n7;
        locals.var_nqs_a_fac_dn8 = assign81970_e122445_d_n8;
        locals.var_nqs_a_fac_dn12 = assign81970_e122445_d_n12;
        locals.var_nqs_a_fac_dn13 = assign81970_e122445_d_n13;
        locals.var_nqs_a_fac_dn14 = assign81970_e122445_d_n14;
        locals.var_nqs_a_fac_dn15 = assign81970_e122445_d_n15;
        locals.var_nqs_a_fac_dn16 = assign81970_e122445_d_n16;
        locals.var_nqs_a_fac_dn17 = assign81970_e122445_d_n17;
        locals.var_nqs_a_fac_dn18 = assign81970_e122445_d_n18;
        locals.var_nqs_a_fac_dn19 = assign81970_e122445_d_n19;
        locals.var_nqs_a_fac_dn20 = assign81970_e122445_d_n20;
        locals.var_nqs_a_fac_rv = 0.0;

        let (assign81980_e122477, assign81980_e122477_d_n5, assign81980_e122477_d_n6, assign81980_e122477_d_n7, assign81980_e122477_d_n8, assign81980_e122477_d_n12, assign81980_e122477_d_n13, assign81980_e122477_d_n14, assign81980_e122477_d_n15, assign81980_e122477_d_n16, assign81980_e122477_d_n17, assign81980_e122477_d_n18, assign81980_e122477_d_n19, assign81980_e122477_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign81980_e122469: f64 = (locals.var_temp__blk1038 / locals.var_a_factrp);
        let assign81980_e122473: f64 = (locals.var_nqs_a_fac * locals.var_temp__blk1038);
        let assign81980_e122474: f64 = (1.0 + assign81980_e122473);
        let assign81980_e122475: f64 = (assign81980_e122469 * assign81980_e122474);
        (assign81980_e122475, (((((locals.var_temp__blk1038_dn5 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn5 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn5)))), (((((locals.var_temp__blk1038_dn6 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn6 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn6)))), (((((locals.var_temp__blk1038_dn7 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn7 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn7)))), (((((locals.var_temp__blk1038_dn8 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn8 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn8)))), (((((locals.var_temp__blk1038_dn12 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn12 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn12)))), (((((locals.var_temp__blk1038_dn13 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn13 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn13)))), (((((locals.var_temp__blk1038_dn14 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn14 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn14)))), (((((locals.var_temp__blk1038_dn15 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn15 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn15)))), (((((locals.var_temp__blk1038_dn16 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn16 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn16)))), (((((locals.var_temp__blk1038_dn17 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn17 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn17)))), (((((locals.var_temp__blk1038_dn18 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn18 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn18)))), (((((locals.var_temp__blk1038_dn19 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn19 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn19)))), (((((locals.var_temp__blk1038_dn20 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign81980_e122474) + (assign81980_e122469 * ((locals.var_nqs_a_fac_dn20 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn20)))),)
    } else {
        (locals.var_nqs_xbar, locals.var_nqs_xbar_dn5, locals.var_nqs_xbar_dn6, locals.var_nqs_xbar_dn7, locals.var_nqs_xbar_dn8, locals.var_nqs_xbar_dn12, locals.var_nqs_xbar_dn13, locals.var_nqs_xbar_dn14, locals.var_nqs_xbar_dn15, locals.var_nqs_xbar_dn16, locals.var_nqs_xbar_dn17, locals.var_nqs_xbar_dn18, locals.var_nqs_xbar_dn19, locals.var_nqs_xbar_dn20,)
    }
};
        locals.var_nqs_xbar = assign81980_e122477;
        locals.var_nqs_xbar_dn5 = assign81980_e122477_d_n5;
        locals.var_nqs_xbar_dn6 = assign81980_e122477_d_n6;
        locals.var_nqs_xbar_dn7 = assign81980_e122477_d_n7;
        locals.var_nqs_xbar_dn8 = assign81980_e122477_d_n8;
        locals.var_nqs_xbar_dn12 = assign81980_e122477_d_n12;
        locals.var_nqs_xbar_dn13 = assign81980_e122477_d_n13;
        locals.var_nqs_xbar_dn14 = assign81980_e122477_d_n14;
        locals.var_nqs_xbar_dn15 = assign81980_e122477_d_n15;
        locals.var_nqs_xbar_dn16 = assign81980_e122477_d_n16;
        locals.var_nqs_xbar_dn17 = assign81980_e122477_d_n17;
        locals.var_nqs_xbar_dn18 = assign81980_e122477_d_n18;
        locals.var_nqs_xbar_dn19 = assign81980_e122477_d_n19;
        locals.var_nqs_xbar_dn20 = assign81980_e122477_d_n20;
        locals.var_nqs_xbar_rv = 0.0;

        let assign81990_e122479: f64 = (-locals.var_nqs_xbar);
        let assign81990_e122480: f64 = (assign81990_e122479).abs();
        let assign81990_e122482: f64 = if assign81990_e122480 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign81990_e122482;
        locals.var_guard2232_rv = 0.0;

        let (assign82000_e122510, assign82000_e122510_d_n5, assign82000_e122510_d_n6, assign82000_e122510_d_n7, assign82000_e122510_d_n8, assign82000_e122510_d_n12, assign82000_e122510_d_n13, assign82000_e122510_d_n14, assign82000_e122510_d_n15, assign82000_e122510_d_n16, assign82000_e122510_d_n17, assign82000_e122510_d_n18, assign82000_e122510_d_n19, assign82000_e122510_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2232 != 0.0)) {
        let assign82000_e122507: f64 = (-locals.var_nqs_xbar);
        let assign82000_e122508: f64 = (assign82000_e122507).exp();
        (assign82000_e122508, (assign82000_e122508 * (-locals.var_nqs_xbar_dn5)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn6)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn7)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn8)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn12)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn13)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn14)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn15)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn16)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn17)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn18)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn19)), (assign82000_e122508 * (-locals.var_nqs_xbar_dn20)),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82000_e122510;
        locals.var_nqs_temp_dn5 = assign82000_e122510_d_n5;
        locals.var_nqs_temp_dn6 = assign82000_e122510_d_n6;
        locals.var_nqs_temp_dn7 = assign82000_e122510_d_n7;
        locals.var_nqs_temp_dn8 = assign82000_e122510_d_n8;
        locals.var_nqs_temp_dn12 = assign82000_e122510_d_n12;
        locals.var_nqs_temp_dn13 = assign82000_e122510_d_n13;
        locals.var_nqs_temp_dn14 = assign82000_e122510_d_n14;
        locals.var_nqs_temp_dn15 = assign82000_e122510_d_n15;
        locals.var_nqs_temp_dn16 = assign82000_e122510_d_n16;
        locals.var_nqs_temp_dn17 = assign82000_e122510_d_n17;
        locals.var_nqs_temp_dn18 = assign82000_e122510_d_n18;
        locals.var_nqs_temp_dn19 = assign82000_e122510_d_n19;
        locals.var_nqs_temp_dn20 = assign82000_e122510_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let assign82010_e122512: f64 = (-locals.var_nqs_xbar);
        let assign82010_e122514: f64 = if assign82010_e122512 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign82010_e122514;
        locals.var_guard2233_rv = 0.0;

        let (assign82020_e122571, assign82020_e122571_d_n5, assign82020_e122571_d_n6, assign82020_e122571_d_n7, assign82020_e122571_d_n8, assign82020_e122571_d_n12, assign82020_e122571_d_n13, assign82020_e122571_d_n14, assign82020_e122571_d_n15, assign82020_e122571_d_n16, assign82020_e122571_d_n17, assign82020_e122571_d_n18, assign82020_e122571_d_n19, assign82020_e122571_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2232 == 0.0)) && (locals.var_guard2233 != 0.0)) {
        let assign82020_e122544: f64 = (-230.25850929940458);
        let assign82020_e122546: f64 = (-locals.var_nqs_xbar);
        let assign82020_e122547: f64 = (assign82020_e122544 - assign82020_e122546);
        let assign82020_e122551: f64 = (-230.25850929940458);
        let assign82020_e122553: f64 = (-locals.var_nqs_xbar);
        let assign82020_e122554: f64 = (assign82020_e122551 - assign82020_e122553);
        let assign82020_e122557: f64 = (-230.25850929940458);
        let assign82020_e122559: f64 = (-locals.var_nqs_xbar);
        let assign82020_e122560: f64 = (assign82020_e122557 - assign82020_e122559);
        let assign82020_e122562: f64 = (assign82020_e122560 * 0.3333333333333333);
        let assign82020_e122563: f64 = (1.0 + assign82020_e122562);
        let assign82020_e122564: f64 = (assign82020_e122554 * assign82020_e122563);
        let assign82020_e122565: f64 = (0.5 * assign82020_e122564);
        let assign82020_e122566: f64 = (1.0 + assign82020_e122565);
        let assign82020_e122567: f64 = (assign82020_e122547 * assign82020_e122566);
        let assign82020_e122568: f64 = (1.0 + assign82020_e122567);
        let assign82020_e122569: f64 = (1e-100 / assign82020_e122568);
        (assign82020_e122569, (-((1e-100 * (((-(-locals.var_nqs_xbar_dn5)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn5)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn5)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn6)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn6)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn6)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn7)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn7)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn7)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn8)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn8)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn8)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn12)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn12)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn12)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn13)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn13)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn13)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn14)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn14)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn14)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn15)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn15)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn15)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn16)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn16)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn16)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn17)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn17)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn17)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn18)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn18)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn18)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn19)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn19)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn19)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn20)) * assign82020_e122566) + (assign82020_e122547 * (0.5 * (((-(-locals.var_nqs_xbar_dn20)) * assign82020_e122563) + (assign82020_e122554 * ((-(-locals.var_nqs_xbar_dn20)) * 0.3333333333333333))))))) / (assign82020_e122568 * assign82020_e122568))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82020_e122571;
        locals.var_nqs_temp_dn5 = assign82020_e122571_d_n5;
        locals.var_nqs_temp_dn6 = assign82020_e122571_d_n6;
        locals.var_nqs_temp_dn7 = assign82020_e122571_d_n7;
        locals.var_nqs_temp_dn8 = assign82020_e122571_d_n8;
        locals.var_nqs_temp_dn12 = assign82020_e122571_d_n12;
        locals.var_nqs_temp_dn13 = assign82020_e122571_d_n13;
        locals.var_nqs_temp_dn14 = assign82020_e122571_d_n14;
        locals.var_nqs_temp_dn15 = assign82020_e122571_d_n15;
        locals.var_nqs_temp_dn16 = assign82020_e122571_d_n16;
        locals.var_nqs_temp_dn17 = assign82020_e122571_d_n17;
        locals.var_nqs_temp_dn18 = assign82020_e122571_d_n18;
        locals.var_nqs_temp_dn19 = assign82020_e122571_d_n19;
        locals.var_nqs_temp_dn20 = assign82020_e122571_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82030_e122626, assign82030_e122626_d_n5, assign82030_e122626_d_n6, assign82030_e122626_d_n7, assign82030_e122626_d_n8, assign82030_e122626_d_n12, assign82030_e122626_d_n13, assign82030_e122626_d_n14, assign82030_e122626_d_n15, assign82030_e122626_d_n16, assign82030_e122626_d_n17, assign82030_e122626_d_n18, assign82030_e122626_d_n19, assign82030_e122626_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2232 == 0.0)) && (locals.var_guard2233 == 0.0)) {
        let assign82030_e122602: f64 = (-locals.var_nqs_xbar);
        let assign82030_e122604: f64 = (assign82030_e122602 - 230.25850929940458);
        let assign82030_e122608: f64 = (-locals.var_nqs_xbar);
        let assign82030_e122610: f64 = (assign82030_e122608 - 230.25850929940458);
        let assign82030_e122613: f64 = (-locals.var_nqs_xbar);
        let assign82030_e122615: f64 = (assign82030_e122613 - 230.25850929940458);
        let assign82030_e122617: f64 = (assign82030_e122615 * 0.3333333333333333);
        let assign82030_e122618: f64 = (1.0 + assign82030_e122617);
        let assign82030_e122619: f64 = (assign82030_e122610 * assign82030_e122618);
        let assign82030_e122620: f64 = (0.5 * assign82030_e122619);
        let assign82030_e122621: f64 = (1.0 + assign82030_e122620);
        let assign82030_e122622: f64 = (assign82030_e122604 * assign82030_e122621);
        let assign82030_e122623: f64 = (1.0 + assign82030_e122622);
        let assign82030_e122624: f64 = (1e100 * assign82030_e122623);
        (assign82030_e122624, (1e100 * (((-locals.var_nqs_xbar_dn5) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn5) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn6) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn6) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn7) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn7) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn8) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn8) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn12) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn12) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn13) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn13) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn14) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn14) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn15) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn15) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn16) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn16) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn17) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn17) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn18) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn18) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn19) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn19) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn20) * assign82030_e122621) + (assign82030_e122604 * (0.5 * (((-locals.var_nqs_xbar_dn20) * assign82030_e122618) + (assign82030_e122610 * ((-locals.var_nqs_xbar_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82030_e122626;
        locals.var_nqs_temp_dn5 = assign82030_e122626_d_n5;
        locals.var_nqs_temp_dn6 = assign82030_e122626_d_n6;
        locals.var_nqs_temp_dn7 = assign82030_e122626_d_n7;
        locals.var_nqs_temp_dn8 = assign82030_e122626_d_n8;
        locals.var_nqs_temp_dn12 = assign82030_e122626_d_n12;
        locals.var_nqs_temp_dn13 = assign82030_e122626_d_n13;
        locals.var_nqs_temp_dn14 = assign82030_e122626_d_n14;
        locals.var_nqs_temp_dn15 = assign82030_e122626_d_n15;
        locals.var_nqs_temp_dn16 = assign82030_e122626_d_n16;
        locals.var_nqs_temp_dn17 = assign82030_e122626_d_n17;
        locals.var_nqs_temp_dn18 = assign82030_e122626_d_n18;
        locals.var_nqs_temp_dn19 = assign82030_e122626_d_n19;
        locals.var_nqs_temp_dn20 = assign82030_e122626_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82040_e122652, assign82040_e122652_d_n5, assign82040_e122652_d_n6, assign82040_e122652_d_n7, assign82040_e122652_d_n8, assign82040_e122652_d_n12, assign82040_e122652_d_n13, assign82040_e122652_d_n14, assign82040_e122652_d_n15, assign82040_e122652_d_n16, assign82040_e122652_d_n17, assign82040_e122652_d_n18, assign82040_e122652_d_n19, assign82040_e122652_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82040_e122650: f64 = (1.0 - locals.var_nqs_temp);
        (assign82040_e122650, (-locals.var_nqs_temp_dn5), (-locals.var_nqs_temp_dn6), (-locals.var_nqs_temp_dn7), (-locals.var_nqs_temp_dn8), (-locals.var_nqs_temp_dn12), (-locals.var_nqs_temp_dn13), (-locals.var_nqs_temp_dn14), (-locals.var_nqs_temp_dn15), (-locals.var_nqs_temp_dn16), (-locals.var_nqs_temp_dn17), (-locals.var_nqs_temp_dn18), (-locals.var_nqs_temp_dn19), (-locals.var_nqs_temp_dn20),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign82040_e122652;
        locals.var_nqs_w_dn5 = assign82040_e122652_d_n5;
        locals.var_nqs_w_dn6 = assign82040_e122652_d_n6;
        locals.var_nqs_w_dn7 = assign82040_e122652_d_n7;
        locals.var_nqs_w_dn8 = assign82040_e122652_d_n8;
        locals.var_nqs_w_dn12 = assign82040_e122652_d_n12;
        locals.var_nqs_w_dn13 = assign82040_e122652_d_n13;
        locals.var_nqs_w_dn14 = assign82040_e122652_d_n14;
        locals.var_nqs_w_dn15 = assign82040_e122652_d_n15;
        locals.var_nqs_w_dn16 = assign82040_e122652_d_n16;
        locals.var_nqs_w_dn17 = assign82040_e122652_d_n17;
        locals.var_nqs_w_dn18 = assign82040_e122652_d_n18;
        locals.var_nqs_w_dn19 = assign82040_e122652_d_n19;
        locals.var_nqs_w_dn20 = assign82040_e122652_d_n20;
        locals.var_nqs_w_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_169(
        locals: &mut StampLocals,
    ) {
        let (assign82050_e122691, assign82050_e122691_d_n5, assign82050_e122691_d_n6, assign82050_e122691_d_n7, assign82050_e122691_d_n8, assign82050_e122691_d_n12, assign82050_e122691_d_n13, assign82050_e122691_d_n14, assign82050_e122691_d_n15, assign82050_e122691_d_n16, assign82050_e122691_d_n17, assign82050_e122691_d_n18, assign82050_e122691_d_n19, assign82050_e122691_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82050_e122677: f64 = (locals.var_gp2 * 0.5);
        let assign82050_e122678: f64 = (locals.var_temp__blk1038 + assign82050_e122677);
        let assign82050_e122683: f64 = (locals.var_gp2 * 0.25);
        let assign82050_e122684: f64 = (locals.var_temp__blk1038 + assign82050_e122683);
        let assign82050_e122686: f64 = (assign82050_e122684 - locals.var_nqs_w);
        let assign82050_e122687: f64 = (assign82050_e122686).sqrt();
        let assign82050_e122688: f64 = (locals.var_gp * assign82050_e122687);
        let assign82050_e122689: f64 = (assign82050_e122678 - assign82050_e122688);
        (assign82050_e122689, ((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.5)) - ((locals.var_gp_dn5 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.25)) - locals.var_nqs_w_dn5) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.5)) - ((locals.var_gp_dn6 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.25)) - locals.var_nqs_w_dn6) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.5)) - ((locals.var_gp_dn7 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.25)) - locals.var_nqs_w_dn7) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.5)) - ((locals.var_gp_dn8 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.25)) - locals.var_nqs_w_dn8) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.5)) - ((locals.var_gp_dn12 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.25)) - locals.var_nqs_w_dn12) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.5)) - ((locals.var_gp_dn13 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.25)) - locals.var_nqs_w_dn13) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.5)) - ((locals.var_gp_dn14 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.25)) - locals.var_nqs_w_dn14) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.5)) - ((locals.var_gp_dn15 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.25)) - locals.var_nqs_w_dn15) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.5)) - ((locals.var_gp_dn16 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.25)) - locals.var_nqs_w_dn16) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.5)) - ((locals.var_gp_dn17 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.25)) - locals.var_nqs_w_dn17) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.5)) - ((locals.var_gp_dn18 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.25)) - locals.var_nqs_w_dn18) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.5)) - ((locals.var_gp_dn19 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.25)) - locals.var_nqs_w_dn19) / (2.0 * assign82050_e122687))))), ((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.5)) - ((locals.var_gp_dn20 * assign82050_e122687) + (locals.var_gp * (((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.25)) - locals.var_nqs_w_dn20) / (2.0 * assign82050_e122687))))),)
    } else {
        (locals.var_nqs_x0, locals.var_nqs_x0_dn5, locals.var_nqs_x0_dn6, locals.var_nqs_x0_dn7, locals.var_nqs_x0_dn8, locals.var_nqs_x0_dn12, locals.var_nqs_x0_dn13, locals.var_nqs_x0_dn14, locals.var_nqs_x0_dn15, locals.var_nqs_x0_dn16, locals.var_nqs_x0_dn17, locals.var_nqs_x0_dn18, locals.var_nqs_x0_dn19, locals.var_nqs_x0_dn20,)
    }
};
        locals.var_nqs_x0 = assign82050_e122691;
        locals.var_nqs_x0_dn5 = assign82050_e122691_d_n5;
        locals.var_nqs_x0_dn6 = assign82050_e122691_d_n6;
        locals.var_nqs_x0_dn7 = assign82050_e122691_d_n7;
        locals.var_nqs_x0_dn8 = assign82050_e122691_d_n8;
        locals.var_nqs_x0_dn12 = assign82050_e122691_d_n12;
        locals.var_nqs_x0_dn13 = assign82050_e122691_d_n13;
        locals.var_nqs_x0_dn14 = assign82050_e122691_d_n14;
        locals.var_nqs_x0_dn15 = assign82050_e122691_d_n15;
        locals.var_nqs_x0_dn16 = assign82050_e122691_d_n16;
        locals.var_nqs_x0_dn17 = assign82050_e122691_d_n17;
        locals.var_nqs_x0_dn18 = assign82050_e122691_d_n18;
        locals.var_nqs_x0_dn19 = assign82050_e122691_d_n19;
        locals.var_nqs_x0_dn20 = assign82050_e122691_d_n20;
        locals.var_nqs_x0_rv = 0.0;

        let assign82060_e122693: f64 = (-locals.var_nqs_x0);
        let assign82060_e122694: f64 = (assign82060_e122693).abs();
        let assign82060_e122696: f64 = if assign82060_e122694 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign82060_e122696;
        locals.var_guard2234_rv = 0.0;

        let (assign82070_e122724, assign82070_e122724_d_n5, assign82070_e122724_d_n6, assign82070_e122724_d_n7, assign82070_e122724_d_n8, assign82070_e122724_d_n12, assign82070_e122724_d_n13, assign82070_e122724_d_n14, assign82070_e122724_d_n15, assign82070_e122724_d_n16, assign82070_e122724_d_n17, assign82070_e122724_d_n18, assign82070_e122724_d_n19, assign82070_e122724_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2234 != 0.0)) {
        let assign82070_e122721: f64 = (-locals.var_nqs_x0);
        let assign82070_e122722: f64 = (assign82070_e122721).exp();
        (assign82070_e122722, (assign82070_e122722 * (-locals.var_nqs_x0_dn5)), (assign82070_e122722 * (-locals.var_nqs_x0_dn6)), (assign82070_e122722 * (-locals.var_nqs_x0_dn7)), (assign82070_e122722 * (-locals.var_nqs_x0_dn8)), (assign82070_e122722 * (-locals.var_nqs_x0_dn12)), (assign82070_e122722 * (-locals.var_nqs_x0_dn13)), (assign82070_e122722 * (-locals.var_nqs_x0_dn14)), (assign82070_e122722 * (-locals.var_nqs_x0_dn15)), (assign82070_e122722 * (-locals.var_nqs_x0_dn16)), (assign82070_e122722 * (-locals.var_nqs_x0_dn17)), (assign82070_e122722 * (-locals.var_nqs_x0_dn18)), (assign82070_e122722 * (-locals.var_nqs_x0_dn19)), (assign82070_e122722 * (-locals.var_nqs_x0_dn20)),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82070_e122724;
        locals.var_nqs_d0_dn5 = assign82070_e122724_d_n5;
        locals.var_nqs_d0_dn6 = assign82070_e122724_d_n6;
        locals.var_nqs_d0_dn7 = assign82070_e122724_d_n7;
        locals.var_nqs_d0_dn8 = assign82070_e122724_d_n8;
        locals.var_nqs_d0_dn12 = assign82070_e122724_d_n12;
        locals.var_nqs_d0_dn13 = assign82070_e122724_d_n13;
        locals.var_nqs_d0_dn14 = assign82070_e122724_d_n14;
        locals.var_nqs_d0_dn15 = assign82070_e122724_d_n15;
        locals.var_nqs_d0_dn16 = assign82070_e122724_d_n16;
        locals.var_nqs_d0_dn17 = assign82070_e122724_d_n17;
        locals.var_nqs_d0_dn18 = assign82070_e122724_d_n18;
        locals.var_nqs_d0_dn19 = assign82070_e122724_d_n19;
        locals.var_nqs_d0_dn20 = assign82070_e122724_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let assign82080_e122726: f64 = (-locals.var_nqs_x0);
        let assign82080_e122728: f64 = if assign82080_e122726 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign82080_e122728;
        locals.var_guard2235_rv = 0.0;

        let (assign82090_e122785, assign82090_e122785_d_n5, assign82090_e122785_d_n6, assign82090_e122785_d_n7, assign82090_e122785_d_n8, assign82090_e122785_d_n12, assign82090_e122785_d_n13, assign82090_e122785_d_n14, assign82090_e122785_d_n15, assign82090_e122785_d_n16, assign82090_e122785_d_n17, assign82090_e122785_d_n18, assign82090_e122785_d_n19, assign82090_e122785_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2234 == 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign82090_e122758: f64 = (-230.25850929940458);
        let assign82090_e122760: f64 = (-locals.var_nqs_x0);
        let assign82090_e122761: f64 = (assign82090_e122758 - assign82090_e122760);
        let assign82090_e122765: f64 = (-230.25850929940458);
        let assign82090_e122767: f64 = (-locals.var_nqs_x0);
        let assign82090_e122768: f64 = (assign82090_e122765 - assign82090_e122767);
        let assign82090_e122771: f64 = (-230.25850929940458);
        let assign82090_e122773: f64 = (-locals.var_nqs_x0);
        let assign82090_e122774: f64 = (assign82090_e122771 - assign82090_e122773);
        let assign82090_e122776: f64 = (assign82090_e122774 * 0.3333333333333333);
        let assign82090_e122777: f64 = (1.0 + assign82090_e122776);
        let assign82090_e122778: f64 = (assign82090_e122768 * assign82090_e122777);
        let assign82090_e122779: f64 = (0.5 * assign82090_e122778);
        let assign82090_e122780: f64 = (1.0 + assign82090_e122779);
        let assign82090_e122781: f64 = (assign82090_e122761 * assign82090_e122780);
        let assign82090_e122782: f64 = (1.0 + assign82090_e122781);
        let assign82090_e122783: f64 = (1e-100 / assign82090_e122782);
        (assign82090_e122783, (-((1e-100 * (((-(-locals.var_nqs_x0_dn5)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn5)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn5)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn6)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn6)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn6)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn7)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn7)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn7)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn8)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn8)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn8)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn12)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn12)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn12)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn13)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn13)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn13)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn14)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn14)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn14)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn15)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn15)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn15)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn16)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn16)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn16)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn17)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn17)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn17)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn18)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn18)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn18)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn19)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn19)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn19)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn20)) * assign82090_e122780) + (assign82090_e122761 * (0.5 * (((-(-locals.var_nqs_x0_dn20)) * assign82090_e122777) + (assign82090_e122768 * ((-(-locals.var_nqs_x0_dn20)) * 0.3333333333333333))))))) / (assign82090_e122782 * assign82090_e122782))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82090_e122785;
        locals.var_nqs_d0_dn5 = assign82090_e122785_d_n5;
        locals.var_nqs_d0_dn6 = assign82090_e122785_d_n6;
        locals.var_nqs_d0_dn7 = assign82090_e122785_d_n7;
        locals.var_nqs_d0_dn8 = assign82090_e122785_d_n8;
        locals.var_nqs_d0_dn12 = assign82090_e122785_d_n12;
        locals.var_nqs_d0_dn13 = assign82090_e122785_d_n13;
        locals.var_nqs_d0_dn14 = assign82090_e122785_d_n14;
        locals.var_nqs_d0_dn15 = assign82090_e122785_d_n15;
        locals.var_nqs_d0_dn16 = assign82090_e122785_d_n16;
        locals.var_nqs_d0_dn17 = assign82090_e122785_d_n17;
        locals.var_nqs_d0_dn18 = assign82090_e122785_d_n18;
        locals.var_nqs_d0_dn19 = assign82090_e122785_d_n19;
        locals.var_nqs_d0_dn20 = assign82090_e122785_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign82100_e122840, assign82100_e122840_d_n5, assign82100_e122840_d_n6, assign82100_e122840_d_n7, assign82100_e122840_d_n8, assign82100_e122840_d_n12, assign82100_e122840_d_n13, assign82100_e122840_d_n14, assign82100_e122840_d_n15, assign82100_e122840_d_n16, assign82100_e122840_d_n17, assign82100_e122840_d_n18, assign82100_e122840_d_n19, assign82100_e122840_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) && (locals.var_guard2234 == 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign82100_e122816: f64 = (-locals.var_nqs_x0);
        let assign82100_e122818: f64 = (assign82100_e122816 - 230.25850929940458);
        let assign82100_e122822: f64 = (-locals.var_nqs_x0);
        let assign82100_e122824: f64 = (assign82100_e122822 - 230.25850929940458);
        let assign82100_e122827: f64 = (-locals.var_nqs_x0);
        let assign82100_e122829: f64 = (assign82100_e122827 - 230.25850929940458);
        let assign82100_e122831: f64 = (assign82100_e122829 * 0.3333333333333333);
        let assign82100_e122832: f64 = (1.0 + assign82100_e122831);
        let assign82100_e122833: f64 = (assign82100_e122824 * assign82100_e122832);
        let assign82100_e122834: f64 = (0.5 * assign82100_e122833);
        let assign82100_e122835: f64 = (1.0 + assign82100_e122834);
        let assign82100_e122836: f64 = (assign82100_e122818 * assign82100_e122835);
        let assign82100_e122837: f64 = (1.0 + assign82100_e122836);
        let assign82100_e122838: f64 = (1e100 * assign82100_e122837);
        (assign82100_e122838, (1e100 * (((-locals.var_nqs_x0_dn5) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn5) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn6) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn6) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn7) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn7) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn8) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn8) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn12) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn12) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn13) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn13) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn14) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn14) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn15) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn15) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn16) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn16) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn17) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn17) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn18) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn18) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn19) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn19) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn20) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-locals.var_nqs_x0_dn20) * assign82100_e122832) + (assign82100_e122824 * ((-locals.var_nqs_x0_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82100_e122840;
        locals.var_nqs_d0_dn5 = assign82100_e122840_d_n5;
        locals.var_nqs_d0_dn6 = assign82100_e122840_d_n6;
        locals.var_nqs_d0_dn7 = assign82100_e122840_d_n7;
        locals.var_nqs_d0_dn8 = assign82100_e122840_d_n8;
        locals.var_nqs_d0_dn12 = assign82100_e122840_d_n12;
        locals.var_nqs_d0_dn13 = assign82100_e122840_d_n13;
        locals.var_nqs_d0_dn14 = assign82100_e122840_d_n14;
        locals.var_nqs_d0_dn15 = assign82100_e122840_d_n15;
        locals.var_nqs_d0_dn16 = assign82100_e122840_d_n16;
        locals.var_nqs_d0_dn17 = assign82100_e122840_d_n17;
        locals.var_nqs_d0_dn18 = assign82100_e122840_d_n18;
        locals.var_nqs_d0_dn19 = assign82100_e122840_d_n19;
        locals.var_nqs_d0_dn20 = assign82100_e122840_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign82110_e122870, assign82110_e122870_d_n5, assign82110_e122870_d_n6, assign82110_e122870_d_n7, assign82110_e122870_d_n8, assign82110_e122870_d_n12, assign82110_e122870_d_n13, assign82110_e122870_d_n14, assign82110_e122870_d_n15, assign82110_e122870_d_n16, assign82110_e122870_d_n17, assign82110_e122870_d_n18, assign82110_e122870_d_n19, assign82110_e122870_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82110_e122865: f64 = (locals.var_gp2 * 0.5);
        let assign82110_e122867: f64 = (assign82110_e122865 * locals.var_nqs_d0);
        let assign82110_e122868: f64 = (1.0 - assign82110_e122867);
        (assign82110_e122868, (-(((locals.var_gp2_dn5 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn5))), (-(((locals.var_gp2_dn6 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn6))), (-(((locals.var_gp2_dn7 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn7))), (-(((locals.var_gp2_dn8 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn8))), (-(((locals.var_gp2_dn12 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn12))), (-(((locals.var_gp2_dn13 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn13))), (-(((locals.var_gp2_dn14 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn14))), (-(((locals.var_gp2_dn15 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn15))), (-(((locals.var_gp2_dn16 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn16))), (-(((locals.var_gp2_dn17 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn17))), (-(((locals.var_gp2_dn18 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn18))), (-(((locals.var_gp2_dn19 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn19))), (-(((locals.var_gp2_dn20 * 0.5) * locals.var_nqs_d0) + (assign82110_e122865 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign82110_e122870;
        locals.var_nqs_xi_dn5 = assign82110_e122870_d_n5;
        locals.var_nqs_xi_dn6 = assign82110_e122870_d_n6;
        locals.var_nqs_xi_dn7 = assign82110_e122870_d_n7;
        locals.var_nqs_xi_dn8 = assign82110_e122870_d_n8;
        locals.var_nqs_xi_dn12 = assign82110_e122870_d_n12;
        locals.var_nqs_xi_dn13 = assign82110_e122870_d_n13;
        locals.var_nqs_xi_dn14 = assign82110_e122870_d_n14;
        locals.var_nqs_xi_dn15 = assign82110_e122870_d_n15;
        locals.var_nqs_xi_dn16 = assign82110_e122870_d_n16;
        locals.var_nqs_xi_dn17 = assign82110_e122870_d_n17;
        locals.var_nqs_xi_dn18 = assign82110_e122870_d_n18;
        locals.var_nqs_xi_dn19 = assign82110_e122870_d_n19;
        locals.var_nqs_xi_dn20 = assign82110_e122870_d_n20;
        locals.var_nqs_xi_rv = 0.0;

        let (assign82120_e122904, assign82120_e122904_d_n5, assign82120_e122904_d_n6, assign82120_e122904_d_n7, assign82120_e122904_d_n8, assign82120_e122904_d_n12, assign82120_e122904_d_n13, assign82120_e122904_d_n14, assign82120_e122904_d_n15, assign82120_e122904_d_n16, assign82120_e122904_d_n17, assign82120_e122904_d_n18, assign82120_e122904_d_n19, assign82120_e122904_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82120_e122895: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82120_e122896: f64 = (2.0 * assign82120_e122895);
        let assign82120_e122900: f64 = (1.0 - locals.var_nqs_d0);
        let assign82120_e122901: f64 = (locals.var_gp2 * assign82120_e122900);
        let assign82120_e122902: f64 = (assign82120_e122896 + assign82120_e122901);
        (assign82120_e122902, ((2.0 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5)) + ((locals.var_gp2_dn5 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn5)))), ((2.0 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6)) + ((locals.var_gp2_dn6 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn6)))), ((2.0 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7)) + ((locals.var_gp2_dn7 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn7)))), ((2.0 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8)) + ((locals.var_gp2_dn8 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn8)))), ((2.0 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12)) + ((locals.var_gp2_dn12 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn12)))), ((2.0 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13)) + ((locals.var_gp2_dn13 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn13)))), ((2.0 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14)) + ((locals.var_gp2_dn14 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn14)))), ((2.0 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15)) + ((locals.var_gp2_dn15 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn15)))), ((2.0 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16)) + ((locals.var_gp2_dn16 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn16)))), ((2.0 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17)) + ((locals.var_gp2_dn17 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn17)))), ((2.0 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18)) + ((locals.var_gp2_dn18 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn18)))), ((2.0 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19)) + ((locals.var_gp2_dn19 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn19)))), ((2.0 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20)) + ((locals.var_gp2_dn20 * assign82120_e122900) + (locals.var_gp2 * (-locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign82120_e122904;
        locals.var_nqs_p_dn5 = assign82120_e122904_d_n5;
        locals.var_nqs_p_dn6 = assign82120_e122904_d_n6;
        locals.var_nqs_p_dn7 = assign82120_e122904_d_n7;
        locals.var_nqs_p_dn8 = assign82120_e122904_d_n8;
        locals.var_nqs_p_dn12 = assign82120_e122904_d_n12;
        locals.var_nqs_p_dn13 = assign82120_e122904_d_n13;
        locals.var_nqs_p_dn14 = assign82120_e122904_d_n14;
        locals.var_nqs_p_dn15 = assign82120_e122904_d_n15;
        locals.var_nqs_p_dn16 = assign82120_e122904_d_n16;
        locals.var_nqs_p_dn17 = assign82120_e122904_d_n17;
        locals.var_nqs_p_dn18 = assign82120_e122904_d_n18;
        locals.var_nqs_p_dn19 = assign82120_e122904_d_n19;
        locals.var_nqs_p_dn20 = assign82120_e122904_d_n20;
        locals.var_nqs_p_rv = 0.0;

        let (assign82130_e122942, assign82130_e122942_d_n5, assign82130_e122942_d_n6, assign82130_e122942_d_n7, assign82130_e122942_d_n8, assign82130_e122942_d_n12, assign82130_e122942_d_n13, assign82130_e122942_d_n14, assign82130_e122942_d_n15, assign82130_e122942_d_n16, assign82130_e122942_d_n17, assign82130_e122942_d_n18, assign82130_e122942_d_n19, assign82130_e122942_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82130_e122928: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82130_e122931: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82130_e122932: f64 = (assign82130_e122928 * assign82130_e122931);
        let assign82130_e122936: f64 = (locals.var_nqs_x0 - 1.0);
        let assign82130_e122938: f64 = (assign82130_e122936 + locals.var_nqs_d0);
        let assign82130_e122939: f64 = (locals.var_gp2 * assign82130_e122938);
        let assign82130_e122940: f64 = (assign82130_e122932 - assign82130_e122939);
        (assign82130_e122940, ((((locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5))) - ((locals.var_gp2_dn5 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn5 + locals.var_nqs_d0_dn5)))), ((((locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6))) - ((locals.var_gp2_dn6 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn6 + locals.var_nqs_d0_dn6)))), ((((locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7))) - ((locals.var_gp2_dn7 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn7 + locals.var_nqs_d0_dn7)))), ((((locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8))) - ((locals.var_gp2_dn8 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn8 + locals.var_nqs_d0_dn8)))), ((((locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12))) - ((locals.var_gp2_dn12 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn12 + locals.var_nqs_d0_dn12)))), ((((locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13))) - ((locals.var_gp2_dn13 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn13 + locals.var_nqs_d0_dn13)))), ((((locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14))) - ((locals.var_gp2_dn14 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn14 + locals.var_nqs_d0_dn14)))), ((((locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15))) - ((locals.var_gp2_dn15 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn15 + locals.var_nqs_d0_dn15)))), ((((locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16))) - ((locals.var_gp2_dn16 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn16 + locals.var_nqs_d0_dn16)))), ((((locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17))) - ((locals.var_gp2_dn17 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn17 + locals.var_nqs_d0_dn17)))), ((((locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18))) - ((locals.var_gp2_dn18 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn18 + locals.var_nqs_d0_dn18)))), ((((locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19))) - ((locals.var_gp2_dn19 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn19 + locals.var_nqs_d0_dn19)))), ((((locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20) * assign82130_e122931) + (assign82130_e122928 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20))) - ((locals.var_gp2_dn20 * assign82130_e122938) + (locals.var_gp2 * (locals.var_nqs_x0_dn20 + locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign82130_e122942;
        locals.var_nqs_q_dn5 = assign82130_e122942_d_n5;
        locals.var_nqs_q_dn6 = assign82130_e122942_d_n6;
        locals.var_nqs_q_dn7 = assign82130_e122942_d_n7;
        locals.var_nqs_q_dn8 = assign82130_e122942_d_n8;
        locals.var_nqs_q_dn12 = assign82130_e122942_d_n12;
        locals.var_nqs_q_dn13 = assign82130_e122942_d_n13;
        locals.var_nqs_q_dn14 = assign82130_e122942_d_n14;
        locals.var_nqs_q_dn15 = assign82130_e122942_d_n15;
        locals.var_nqs_q_dn16 = assign82130_e122942_d_n16;
        locals.var_nqs_q_dn17 = assign82130_e122942_d_n17;
        locals.var_nqs_q_dn18 = assign82130_e122942_d_n18;
        locals.var_nqs_q_dn19 = assign82130_e122942_d_n19;
        locals.var_nqs_q_dn20 = assign82130_e122942_d_n20;
        locals.var_nqs_q_rv = 0.0;

        let (assign82140_e122974, assign82140_e122974_d_n5, assign82140_e122974_d_n6, assign82140_e122974_d_n7, assign82140_e122974_d_n8, assign82140_e122974_d_n12, assign82140_e122974_d_n13, assign82140_e122974_d_n14, assign82140_e122974_d_n15, assign82140_e122974_d_n16, assign82140_e122974_d_n17, assign82140_e122974_d_n18, assign82140_e122974_d_n19, assign82140_e122974_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82140_e122966: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign82140_e122969: f64 = (4.0 * locals.var_nqs_xi);
        let assign82140_e122971: f64 = (assign82140_e122969 * locals.var_nqs_q);
        let assign82140_e122972: f64 = (assign82140_e122966 - assign82140_e122971);
        (assign82140_e122972, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign82140_e122969 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82140_e122974;
        locals.var_nqs_temp_dn5 = assign82140_e122974_d_n5;
        locals.var_nqs_temp_dn6 = assign82140_e122974_d_n6;
        locals.var_nqs_temp_dn7 = assign82140_e122974_d_n7;
        locals.var_nqs_temp_dn8 = assign82140_e122974_d_n8;
        locals.var_nqs_temp_dn12 = assign82140_e122974_d_n12;
        locals.var_nqs_temp_dn13 = assign82140_e122974_d_n13;
        locals.var_nqs_temp_dn14 = assign82140_e122974_d_n14;
        locals.var_nqs_temp_dn15 = assign82140_e122974_d_n15;
        locals.var_nqs_temp_dn16 = assign82140_e122974_d_n16;
        locals.var_nqs_temp_dn17 = assign82140_e122974_d_n17;
        locals.var_nqs_temp_dn18 = assign82140_e122974_d_n18;
        locals.var_nqs_temp_dn19 = assign82140_e122974_d_n19;
        locals.var_nqs_temp_dn20 = assign82140_e122974_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82150_e123005, assign82150_e123005_d_n5, assign82150_e123005_d_n6, assign82150_e123005_d_n7, assign82150_e123005_d_n8, assign82150_e123005_d_n12, assign82150_e123005_d_n13, assign82150_e123005_d_n14, assign82150_e123005_d_n15, assign82150_e123005_d_n16, assign82150_e123005_d_n17, assign82150_e123005_d_n18, assign82150_e123005_d_n19, assign82150_e123005_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82150_e122998: f64 = (2.0 * locals.var_nqs_q);
        let assign82150_e123001: f64 = (locals.var_nqs_temp).sqrt();
        let assign82150_e123002: f64 = (locals.var_nqs_p + assign82150_e123001);
        let assign82150_e123003: f64 = (assign82150_e122998 / assign82150_e123002);
        (assign82150_e123003, ((((2.0 * locals.var_nqs_q_dn5) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn6) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn7) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn8) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn12) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn13) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn14) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn15) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn16) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn17) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn18) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn19) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * locals.var_nqs_q_dn20) * assign82150_e123002) - (assign82150_e122998 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)),)
    } else {
        (locals.var_nqs_u, locals.var_nqs_u_dn5, locals.var_nqs_u_dn6, locals.var_nqs_u_dn7, locals.var_nqs_u_dn8, locals.var_nqs_u_dn12, locals.var_nqs_u_dn13, locals.var_nqs_u_dn14, locals.var_nqs_u_dn15, locals.var_nqs_u_dn16, locals.var_nqs_u_dn17, locals.var_nqs_u_dn18, locals.var_nqs_u_dn19, locals.var_nqs_u_dn20,)
    }
};
        locals.var_nqs_u = assign82150_e123005;
        locals.var_nqs_u_dn5 = assign82150_e123005_d_n5;
        locals.var_nqs_u_dn6 = assign82150_e123005_d_n6;
        locals.var_nqs_u_dn7 = assign82150_e123005_d_n7;
        locals.var_nqs_u_dn8 = assign82150_e123005_d_n8;
        locals.var_nqs_u_dn12 = assign82150_e123005_d_n12;
        locals.var_nqs_u_dn13 = assign82150_e123005_d_n13;
        locals.var_nqs_u_dn14 = assign82150_e123005_d_n14;
        locals.var_nqs_u_dn15 = assign82150_e123005_d_n15;
        locals.var_nqs_u_dn16 = assign82150_e123005_d_n16;
        locals.var_nqs_u_dn17 = assign82150_e123005_d_n17;
        locals.var_nqs_u_dn18 = assign82150_e123005_d_n18;
        locals.var_nqs_u_dn19 = assign82150_e123005_d_n19;
        locals.var_nqs_u_dn20 = assign82150_e123005_d_n20;
        locals.var_nqs_u_rv = 0.0;

        let (assign82160_e123031, assign82160_e123031_d_n5, assign82160_e123031_d_n6, assign82160_e123031_d_n7, assign82160_e123031_d_n8, assign82160_e123031_d_n12, assign82160_e123031_d_n13, assign82160_e123031_d_n14, assign82160_e123031_d_n15, assign82160_e123031_d_n16, assign82160_e123031_d_n17, assign82160_e123031_d_n18, assign82160_e123031_d_n19, assign82160_e123031_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2228 == 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign82160_e123029: f64 = (locals.var_nqs_x0 + locals.var_nqs_u);
        (assign82160_e123029, (locals.var_nqs_x0_dn5 + locals.var_nqs_u_dn5), (locals.var_nqs_x0_dn6 + locals.var_nqs_u_dn6), (locals.var_nqs_x0_dn7 + locals.var_nqs_u_dn7), (locals.var_nqs_x0_dn8 + locals.var_nqs_u_dn8), (locals.var_nqs_x0_dn12 + locals.var_nqs_u_dn12), (locals.var_nqs_x0_dn13 + locals.var_nqs_u_dn13), (locals.var_nqs_x0_dn14 + locals.var_nqs_u_dn14), (locals.var_nqs_x0_dn15 + locals.var_nqs_u_dn15), (locals.var_nqs_x0_dn16 + locals.var_nqs_u_dn16), (locals.var_nqs_x0_dn17 + locals.var_nqs_u_dn17), (locals.var_nqs_x0_dn18 + locals.var_nqs_u_dn18), (locals.var_nqs_x0_dn19 + locals.var_nqs_u_dn19), (locals.var_nqs_x0_dn20 + locals.var_nqs_u_dn20),)
    } else {
        (locals.var_temp8, locals.var_temp8_dn5, locals.var_temp8_dn6, locals.var_temp8_dn7, locals.var_temp8_dn8, locals.var_temp8_dn12, locals.var_temp8_dn13, locals.var_temp8_dn14, locals.var_temp8_dn15, locals.var_temp8_dn16, locals.var_temp8_dn17, locals.var_temp8_dn18, locals.var_temp8_dn19, locals.var_temp8_dn20,)
    }
};
        locals.var_temp8 = assign82160_e123031;
        locals.var_temp8_dn5 = assign82160_e123031_d_n5;
        locals.var_temp8_dn6 = assign82160_e123031_d_n6;
        locals.var_temp8_dn7 = assign82160_e123031_d_n7;
        locals.var_temp8_dn8 = assign82160_e123031_d_n8;
        locals.var_temp8_dn12 = assign82160_e123031_d_n12;
        locals.var_temp8_dn13 = assign82160_e123031_d_n13;
        locals.var_temp8_dn14 = assign82160_e123031_d_n14;
        locals.var_temp8_dn15 = assign82160_e123031_d_n15;
        locals.var_temp8_dn16 = assign82160_e123031_d_n16;
        locals.var_temp8_dn17 = assign82160_e123031_d_n17;
        locals.var_temp8_dn18 = assign82160_e123031_d_n18;
        locals.var_temp8_dn19 = assign82160_e123031_d_n19;
        locals.var_temp8_dn20 = assign82160_e123031_d_n20;
        locals.var_temp8_rv = 0.0;

        let (assign82170_e123053, assign82170_e123053_d_n5, assign82170_e123053_d_n6, assign82170_e123053_d_n7, assign82170_e123053_d_n8, assign82170_e123053_d_n12, assign82170_e123053_d_n13, assign82170_e123053_d_n14, assign82170_e123053_d_n15, assign82170_e123053_d_n16, assign82170_e123053_d_n17, assign82170_e123053_d_n18, assign82170_e123053_d_n19, assign82170_e123053_d_n20,) = {
    if ((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) {
        let assign82170_e123049: f64 = (locals.var_qp9 / locals.var_pd);
        let assign82170_e123051: f64 = (assign82170_e123049 + locals.var_xg_ac);
        (assign82170_e123051, ((-((locals.var_qp9 * locals.var_pd_dn5) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn5), ((-((locals.var_qp9 * locals.var_pd_dn6) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn6), ((-((locals.var_qp9 * locals.var_pd_dn7) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn7), ((-((locals.var_qp9 * locals.var_pd_dn8) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn8), ((-((locals.var_qp9 * locals.var_pd_dn12) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn12), ((-((locals.var_qp9 * locals.var_pd_dn13) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn13), ((-((locals.var_qp9 * locals.var_pd_dn14) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn14), ((-((locals.var_qp9 * locals.var_pd_dn15) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn15), ((-((locals.var_qp9 * locals.var_pd_dn16) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn16), ((-((locals.var_qp9 * locals.var_pd_dn17) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn17), ((-((locals.var_qp9 * locals.var_pd_dn18) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn18), ((-((locals.var_qp9 * locals.var_pd_dn19) / (locals.var_pd * locals.var_pd))) + locals.var_xg_ac_dn19), ((((locals.var_qp9_dn20 * locals.var_pd) - (locals.var_qp9 * locals.var_pd_dn20)) / (locals.var_pd * locals.var_pd)) + locals.var_xg_ac_dn20),)
    } else {
        (locals.var_temp__blk1038, locals.var_temp__blk1038_dn5, locals.var_temp__blk1038_dn6, locals.var_temp__blk1038_dn7, locals.var_temp__blk1038_dn8, locals.var_temp__blk1038_dn12, locals.var_temp__blk1038_dn13, locals.var_temp__blk1038_dn14, locals.var_temp__blk1038_dn15, locals.var_temp__blk1038_dn16, locals.var_temp__blk1038_dn17, locals.var_temp__blk1038_dn18, locals.var_temp__blk1038_dn19, locals.var_temp__blk1038_dn20,)
    }
};
        locals.var_temp__blk1038 = assign82170_e123053;
        locals.var_temp__blk1038_dn5 = assign82170_e123053_d_n5;
        locals.var_temp__blk1038_dn6 = assign82170_e123053_d_n6;
        locals.var_temp__blk1038_dn7 = assign82170_e123053_d_n7;
        locals.var_temp__blk1038_dn8 = assign82170_e123053_d_n8;
        locals.var_temp__blk1038_dn12 = assign82170_e123053_d_n12;
        locals.var_temp__blk1038_dn13 = assign82170_e123053_d_n13;
        locals.var_temp__blk1038_dn14 = assign82170_e123053_d_n14;
        locals.var_temp__blk1038_dn15 = assign82170_e123053_d_n15;
        locals.var_temp__blk1038_dn16 = assign82170_e123053_d_n16;
        locals.var_temp__blk1038_dn17 = assign82170_e123053_d_n17;
        locals.var_temp__blk1038_dn18 = assign82170_e123053_d_n18;
        locals.var_temp__blk1038_dn19 = assign82170_e123053_d_n19;
        locals.var_temp__blk1038_dn20 = assign82170_e123053_d_n20;
        locals.var_temp__blk1038_rv = 0.0;

        let assign82180_e123055: f64 = (locals.var_temp__blk1038).abs();
        let assign82180_e123057: f64 = if assign82180_e123055 <= locals.var_marginp { 1.0 } else { 0.0 };
        locals.var_guard2236 = assign82180_e123057;
        locals.var_guard2236_rv = 0.0;

        let (assign82190_e123079, assign82190_e123079_d_n5, assign82190_e123079_d_n6, assign82190_e123079_d_n7, assign82190_e123079_d_n8, assign82190_e123079_d_n12, assign82190_e123079_d_n13, assign82190_e123079_d_n14, assign82190_e123079_d_n15, assign82190_e123079_d_n16, assign82190_e123079_d_n17, assign82190_e123079_d_n18, assign82190_e123079_d_n19, assign82190_e123079_d_n20,) = {
    if (((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 != 0.0)) {
        let assign82190_e123077: f64 = (locals.var_temp__blk1038 / locals.var_a_factrp);
        (assign82190_e123077, (((locals.var_temp__blk1038_dn5 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn6 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn7 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn8 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn12 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn13 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn14 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn15 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn16 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn17 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn18 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn19 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)), (((locals.var_temp__blk1038_dn20 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)),)
    } else {
        (locals.var_temp9, locals.var_temp9_dn5, locals.var_temp9_dn6, locals.var_temp9_dn7, locals.var_temp9_dn8, locals.var_temp9_dn12, locals.var_temp9_dn13, locals.var_temp9_dn14, locals.var_temp9_dn15, locals.var_temp9_dn16, locals.var_temp9_dn17, locals.var_temp9_dn18, locals.var_temp9_dn19, locals.var_temp9_dn20,)
    }
};
        locals.var_temp9 = assign82190_e123079;
        locals.var_temp9_dn5 = assign82190_e123079_d_n5;
        locals.var_temp9_dn6 = assign82190_e123079_d_n6;
        locals.var_temp9_dn7 = assign82190_e123079_d_n7;
        locals.var_temp9_dn8 = assign82190_e123079_d_n8;
        locals.var_temp9_dn12 = assign82190_e123079_d_n12;
        locals.var_temp9_dn13 = assign82190_e123079_d_n13;
        locals.var_temp9_dn14 = assign82190_e123079_d_n14;
        locals.var_temp9_dn15 = assign82190_e123079_d_n15;
        locals.var_temp9_dn16 = assign82190_e123079_d_n16;
        locals.var_temp9_dn17 = assign82190_e123079_d_n17;
        locals.var_temp9_dn18 = assign82190_e123079_d_n18;
        locals.var_temp9_dn19 = assign82190_e123079_d_n19;
        locals.var_temp9_dn20 = assign82190_e123079_d_n20;
        locals.var_temp9_rv = 0.0;

        let assign82200_e123082: f64 = (-locals.var_marginp);
        let assign82200_e123083: f64 = if locals.var_temp__blk1038 < assign82200_e123082 { 1.0 } else { 0.0 };
        locals.var_guard2237 = assign82200_e123083;
        locals.var_guard2237_rv = 0.0;

        let (assign82210_e123107, assign82210_e123107_d_n5, assign82210_e123107_d_n6, assign82210_e123107_d_n7, assign82210_e123107_d_n8, assign82210_e123107_d_n12, assign82210_e123107_d_n13, assign82210_e123107_d_n14, assign82210_e123107_d_n15, assign82210_e123107_d_n16, assign82210_e123107_d_n17, assign82210_e123107_d_n18, assign82210_e123107_d_n19, assign82210_e123107_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82210_e123105: f64 = (-locals.var_temp__blk1038);
        (assign82210_e123105, (-locals.var_temp__blk1038_dn5), (-locals.var_temp__blk1038_dn6), (-locals.var_temp__blk1038_dn7), (-locals.var_temp__blk1038_dn8), (-locals.var_temp__blk1038_dn12), (-locals.var_temp__blk1038_dn13), (-locals.var_temp__blk1038_dn14), (-locals.var_temp__blk1038_dn15), (-locals.var_temp__blk1038_dn16), (-locals.var_temp__blk1038_dn17), (-locals.var_temp__blk1038_dn18), (-locals.var_temp__blk1038_dn19), (-locals.var_temp__blk1038_dn20),)
    } else {
        (locals.var_nqs_yg, locals.var_nqs_yg_dn5, locals.var_nqs_yg_dn6, locals.var_nqs_yg_dn7, locals.var_nqs_yg_dn8, locals.var_nqs_yg_dn12, locals.var_nqs_yg_dn13, locals.var_nqs_yg_dn14, locals.var_nqs_yg_dn15, locals.var_nqs_yg_dn16, locals.var_nqs_yg_dn17, locals.var_nqs_yg_dn18, locals.var_nqs_yg_dn19, locals.var_nqs_yg_dn20,)
    }
};
        locals.var_nqs_yg = assign82210_e123107;
        locals.var_nqs_yg_dn5 = assign82210_e123107_d_n5;
        locals.var_nqs_yg_dn6 = assign82210_e123107_d_n6;
        locals.var_nqs_yg_dn7 = assign82210_e123107_d_n7;
        locals.var_nqs_yg_dn8 = assign82210_e123107_d_n8;
        locals.var_nqs_yg_dn12 = assign82210_e123107_d_n12;
        locals.var_nqs_yg_dn13 = assign82210_e123107_d_n13;
        locals.var_nqs_yg_dn14 = assign82210_e123107_d_n14;
        locals.var_nqs_yg_dn15 = assign82210_e123107_d_n15;
        locals.var_nqs_yg_dn16 = assign82210_e123107_d_n16;
        locals.var_nqs_yg_dn17 = assign82210_e123107_d_n17;
        locals.var_nqs_yg_dn18 = assign82210_e123107_d_n18;
        locals.var_nqs_yg_dn19 = assign82210_e123107_d_n19;
        locals.var_nqs_yg_dn20 = assign82210_e123107_d_n20;
        locals.var_nqs_yg_rv = 0.0;

        let (assign82220_e123134, assign82220_e123134_d_n5, assign82220_e123134_d_n6, assign82220_e123134_d_n7, assign82220_e123134_d_n8, assign82220_e123134_d_n12, assign82220_e123134_d_n13, assign82220_e123134_d_n14, assign82220_e123134_d_n15, assign82220_e123134_d_n16, assign82220_e123134_d_n17, assign82220_e123134_d_n18, assign82220_e123134_d_n19, assign82220_e123134_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82220_e123130: f64 = (1.25 * locals.var_nqs_yg);
        let assign82220_e123132: f64 = (assign82220_e123130 / locals.var_a_factrp);
        (assign82220_e123132, ((((1.25 * locals.var_nqs_yg_dn5) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn6) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn7) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn8) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn12) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn13) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn14) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn15) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn16) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn17) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn18) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn19) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)), ((((1.25 * locals.var_nqs_yg_dn20) * locals.var_a_factrp) - (assign82220_e123130 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)),)
    } else {
        (locals.var_nqs_z, locals.var_nqs_z_dn5, locals.var_nqs_z_dn6, locals.var_nqs_z_dn7, locals.var_nqs_z_dn8, locals.var_nqs_z_dn12, locals.var_nqs_z_dn13, locals.var_nqs_z_dn14, locals.var_nqs_z_dn15, locals.var_nqs_z_dn16, locals.var_nqs_z_dn17, locals.var_nqs_z_dn18, locals.var_nqs_z_dn19, locals.var_nqs_z_dn20,)
    }
};
        locals.var_nqs_z = assign82220_e123134;
        locals.var_nqs_z_dn5 = assign82220_e123134_d_n5;
        locals.var_nqs_z_dn6 = assign82220_e123134_d_n6;
        locals.var_nqs_z_dn7 = assign82220_e123134_d_n7;
        locals.var_nqs_z_dn8 = assign82220_e123134_d_n8;
        locals.var_nqs_z_dn12 = assign82220_e123134_d_n12;
        locals.var_nqs_z_dn13 = assign82220_e123134_d_n13;
        locals.var_nqs_z_dn14 = assign82220_e123134_d_n14;
        locals.var_nqs_z_dn15 = assign82220_e123134_d_n15;
        locals.var_nqs_z_dn16 = assign82220_e123134_d_n16;
        locals.var_nqs_z_dn17 = assign82220_e123134_d_n17;
        locals.var_nqs_z_dn18 = assign82220_e123134_d_n18;
        locals.var_nqs_z_dn19 = assign82220_e123134_d_n19;
        locals.var_nqs_z_dn20 = assign82220_e123134_d_n20;
        locals.var_nqs_z_rv = 0.0;

        let (assign82230_e123172, assign82230_e123172_d_n5, assign82230_e123172_d_n6, assign82230_e123172_d_n7, assign82230_e123172_d_n8, assign82230_e123172_d_n12, assign82230_e123172_d_n13, assign82230_e123172_d_n14, assign82230_e123172_d_n15, assign82230_e123172_d_n16, assign82230_e123172_d_n17, assign82230_e123172_d_n18, assign82230_e123172_d_n19, assign82230_e123172_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82230_e123157: f64 = (locals.var_nqs_z + 10.0);
        let assign82230_e123160: f64 = (locals.var_nqs_z - 6.0);
        let assign82230_e123163: f64 = (locals.var_nqs_z - 6.0);
        let assign82230_e123164: f64 = (assign82230_e123160 * assign82230_e123163);
        let assign82230_e123166: f64 = (assign82230_e123164 + 64.0);
        let assign82230_e123167: f64 = (assign82230_e123166).sqrt();
        let assign82230_e123168: f64 = (assign82230_e123157 - assign82230_e123167);
        let assign82230_e123170: f64 = (assign82230_e123168 * 0.5);
        (assign82230_e123170, ((locals.var_nqs_z_dn5 - (((locals.var_nqs_z_dn5 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn5)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn6 - (((locals.var_nqs_z_dn6 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn6)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn7 - (((locals.var_nqs_z_dn7 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn7)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn8 - (((locals.var_nqs_z_dn8 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn8)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn12 - (((locals.var_nqs_z_dn12 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn12)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn13 - (((locals.var_nqs_z_dn13 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn13)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn14 - (((locals.var_nqs_z_dn14 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn14)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn15 - (((locals.var_nqs_z_dn15 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn15)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn16 - (((locals.var_nqs_z_dn16 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn16)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn17 - (((locals.var_nqs_z_dn17 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn17)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn18 - (((locals.var_nqs_z_dn18 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn18)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn19 - (((locals.var_nqs_z_dn19 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn19)) / (2.0 * assign82230_e123167))) * 0.5), ((locals.var_nqs_z_dn20 - (((locals.var_nqs_z_dn20 * assign82230_e123163) + (assign82230_e123160 * locals.var_nqs_z_dn20)) / (2.0 * assign82230_e123167))) * 0.5),)
    } else {
        (locals.var_nqs_eta, locals.var_nqs_eta_dn5, locals.var_nqs_eta_dn6, locals.var_nqs_eta_dn7, locals.var_nqs_eta_dn8, locals.var_nqs_eta_dn12, locals.var_nqs_eta_dn13, locals.var_nqs_eta_dn14, locals.var_nqs_eta_dn15, locals.var_nqs_eta_dn16, locals.var_nqs_eta_dn17, locals.var_nqs_eta_dn18, locals.var_nqs_eta_dn19, locals.var_nqs_eta_dn20,)
    }
};
        locals.var_nqs_eta = assign82230_e123172;
        locals.var_nqs_eta_dn5 = assign82230_e123172_d_n5;
        locals.var_nqs_eta_dn6 = assign82230_e123172_d_n6;
        locals.var_nqs_eta_dn7 = assign82230_e123172_d_n7;
        locals.var_nqs_eta_dn8 = assign82230_e123172_d_n8;
        locals.var_nqs_eta_dn12 = assign82230_e123172_d_n12;
        locals.var_nqs_eta_dn13 = assign82230_e123172_d_n13;
        locals.var_nqs_eta_dn14 = assign82230_e123172_d_n14;
        locals.var_nqs_eta_dn15 = assign82230_e123172_d_n15;
        locals.var_nqs_eta_dn16 = assign82230_e123172_d_n16;
        locals.var_nqs_eta_dn17 = assign82230_e123172_d_n17;
        locals.var_nqs_eta_dn18 = assign82230_e123172_d_n18;
        locals.var_nqs_eta_dn19 = assign82230_e123172_d_n19;
        locals.var_nqs_eta_dn20 = assign82230_e123172_d_n20;
        locals.var_nqs_eta_rv = 0.0;

        let (assign82240_e123207, assign82240_e123207_d_n5, assign82240_e123207_d_n6, assign82240_e123207_d_n7, assign82240_e123207_d_n8, assign82240_e123207_d_n12, assign82240_e123207_d_n13, assign82240_e123207_d_n14, assign82240_e123207_d_n15, assign82240_e123207_d_n16, assign82240_e123207_d_n17, assign82240_e123207_d_n18, assign82240_e123207_d_n19, assign82240_e123207_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82240_e123195: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign82240_e123198: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign82240_e123199: f64 = (assign82240_e123195 * assign82240_e123198);
        let assign82240_e123203: f64 = (locals.var_nqs_eta + 1.0);
        let assign82240_e123204: f64 = (locals.var_gp2 * assign82240_e123203);
        let assign82240_e123205: f64 = (assign82240_e123199 + assign82240_e123204);
        (assign82240_e123205, ((((locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5))) + ((locals.var_gp2_dn5 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn5))), ((((locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6))) + ((locals.var_gp2_dn6 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn6))), ((((locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7))) + ((locals.var_gp2_dn7 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn7))), ((((locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8))) + ((locals.var_gp2_dn8 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn8))), ((((locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12))) + ((locals.var_gp2_dn12 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn12))), ((((locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13))) + ((locals.var_gp2_dn13 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn13))), ((((locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14))) + ((locals.var_gp2_dn14 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn14))), ((((locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15))) + ((locals.var_gp2_dn15 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn15))), ((((locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16))) + ((locals.var_gp2_dn16 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn16))), ((((locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17))) + ((locals.var_gp2_dn17 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn17))), ((((locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18))) + ((locals.var_gp2_dn18 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn18))), ((((locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19))) + ((locals.var_gp2_dn19 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn19))), ((((locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20) * assign82240_e123198) + (assign82240_e123195 * (locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20))) + ((locals.var_gp2_dn20 * assign82240_e123203) + (locals.var_gp2 * locals.var_nqs_eta_dn20))),)
    } else {
        (locals.var_nqs_a, locals.var_nqs_a_dn5, locals.var_nqs_a_dn6, locals.var_nqs_a_dn7, locals.var_nqs_a_dn8, locals.var_nqs_a_dn12, locals.var_nqs_a_dn13, locals.var_nqs_a_dn14, locals.var_nqs_a_dn15, locals.var_nqs_a_dn16, locals.var_nqs_a_dn17, locals.var_nqs_a_dn18, locals.var_nqs_a_dn19, locals.var_nqs_a_dn20,)
    }
};
        locals.var_nqs_a = assign82240_e123207;
        locals.var_nqs_a_dn5 = assign82240_e123207_d_n5;
        locals.var_nqs_a_dn6 = assign82240_e123207_d_n6;
        locals.var_nqs_a_dn7 = assign82240_e123207_d_n7;
        locals.var_nqs_a_dn8 = assign82240_e123207_d_n8;
        locals.var_nqs_a_dn12 = assign82240_e123207_d_n12;
        locals.var_nqs_a_dn13 = assign82240_e123207_d_n13;
        locals.var_nqs_a_dn14 = assign82240_e123207_d_n14;
        locals.var_nqs_a_dn15 = assign82240_e123207_d_n15;
        locals.var_nqs_a_dn16 = assign82240_e123207_d_n16;
        locals.var_nqs_a_dn17 = assign82240_e123207_d_n17;
        locals.var_nqs_a_dn18 = assign82240_e123207_d_n18;
        locals.var_nqs_a_dn19 = assign82240_e123207_d_n19;
        locals.var_nqs_a_dn20 = assign82240_e123207_d_n20;
        locals.var_nqs_a_rv = 0.0;

        let (assign82250_e123236, assign82250_e123236_d_n5, assign82250_e123236_d_n6, assign82250_e123236_d_n7, assign82250_e123236_d_n8, assign82250_e123236_d_n12, assign82250_e123236_d_n13, assign82250_e123236_d_n14, assign82250_e123236_d_n15, assign82250_e123236_d_n16, assign82250_e123236_d_n17, assign82250_e123236_d_n18, assign82250_e123236_d_n19, assign82250_e123236_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82250_e123231: f64 = (locals.var_nqs_yg - locals.var_nqs_eta);
        let assign82250_e123232: f64 = (2.0 * assign82250_e123231);
        let assign82250_e123234: f64 = (assign82250_e123232 - locals.var_gp2);
        (assign82250_e123234, ((2.0 * (locals.var_nqs_yg_dn5 - locals.var_nqs_eta_dn5)) - locals.var_gp2_dn5), ((2.0 * (locals.var_nqs_yg_dn6 - locals.var_nqs_eta_dn6)) - locals.var_gp2_dn6), ((2.0 * (locals.var_nqs_yg_dn7 - locals.var_nqs_eta_dn7)) - locals.var_gp2_dn7), ((2.0 * (locals.var_nqs_yg_dn8 - locals.var_nqs_eta_dn8)) - locals.var_gp2_dn8), ((2.0 * (locals.var_nqs_yg_dn12 - locals.var_nqs_eta_dn12)) - locals.var_gp2_dn12), ((2.0 * (locals.var_nqs_yg_dn13 - locals.var_nqs_eta_dn13)) - locals.var_gp2_dn13), ((2.0 * (locals.var_nqs_yg_dn14 - locals.var_nqs_eta_dn14)) - locals.var_gp2_dn14), ((2.0 * (locals.var_nqs_yg_dn15 - locals.var_nqs_eta_dn15)) - locals.var_gp2_dn15), ((2.0 * (locals.var_nqs_yg_dn16 - locals.var_nqs_eta_dn16)) - locals.var_gp2_dn16), ((2.0 * (locals.var_nqs_yg_dn17 - locals.var_nqs_eta_dn17)) - locals.var_gp2_dn17), ((2.0 * (locals.var_nqs_yg_dn18 - locals.var_nqs_eta_dn18)) - locals.var_gp2_dn18), ((2.0 * (locals.var_nqs_yg_dn19 - locals.var_nqs_eta_dn19)) - locals.var_gp2_dn19), ((2.0 * (locals.var_nqs_yg_dn20 - locals.var_nqs_eta_dn20)) - locals.var_gp2_dn20),)
    } else {
        (locals.var_nqs_c, locals.var_nqs_c_dn5, locals.var_nqs_c_dn6, locals.var_nqs_c_dn7, locals.var_nqs_c_dn8, locals.var_nqs_c_dn12, locals.var_nqs_c_dn13, locals.var_nqs_c_dn14, locals.var_nqs_c_dn15, locals.var_nqs_c_dn16, locals.var_nqs_c_dn17, locals.var_nqs_c_dn18, locals.var_nqs_c_dn19, locals.var_nqs_c_dn20,)
    }
};
        locals.var_nqs_c = assign82250_e123236;
        locals.var_nqs_c_dn5 = assign82250_e123236_d_n5;
        locals.var_nqs_c_dn6 = assign82250_e123236_d_n6;
        locals.var_nqs_c_dn7 = assign82250_e123236_d_n7;
        locals.var_nqs_c_dn8 = assign82250_e123236_d_n8;
        locals.var_nqs_c_dn12 = assign82250_e123236_d_n12;
        locals.var_nqs_c_dn13 = assign82250_e123236_d_n13;
        locals.var_nqs_c_dn14 = assign82250_e123236_d_n14;
        locals.var_nqs_c_dn15 = assign82250_e123236_d_n15;
        locals.var_nqs_c_dn16 = assign82250_e123236_d_n16;
        locals.var_nqs_c_dn17 = assign82250_e123236_d_n17;
        locals.var_nqs_c_dn18 = assign82250_e123236_d_n18;
        locals.var_nqs_c_dn19 = assign82250_e123236_d_n19;
        locals.var_nqs_c_dn20 = assign82250_e123236_d_n20;
        locals.var_nqs_c_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_170(
        locals: &mut StampLocals,
    ) {
        let (assign82260_e123264, assign82260_e123264_d_n5, assign82260_e123264_d_n6, assign82260_e123264_d_n7, assign82260_e123264_d_n8, assign82260_e123264_d_n12, assign82260_e123264_d_n13, assign82260_e123264_d_n14, assign82260_e123264_d_n15, assign82260_e123264_d_n16, assign82260_e123264_d_n17, assign82260_e123264_d_n18, assign82260_e123264_d_n19, assign82260_e123264_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82260_e123259: f64 = (locals.var_nqs_a / locals.var_gp2);
        let assign82260_e123260: f64 = (assign82260_e123259).ln();
        let assign82260_e123262: f64 = (assign82260_e123260 - locals.var_nqs_eta);
        (assign82260_e123262, (((((locals.var_nqs_a_dn5 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn5)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn5), (((((locals.var_nqs_a_dn6 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn6)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn6), (((((locals.var_nqs_a_dn7 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn7)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn7), (((((locals.var_nqs_a_dn8 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn8)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn8), (((((locals.var_nqs_a_dn12 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn12)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn12), (((((locals.var_nqs_a_dn13 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn13)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn13), (((((locals.var_nqs_a_dn14 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn14)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn14), (((((locals.var_nqs_a_dn15 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn15)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn15), (((((locals.var_nqs_a_dn16 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn16)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn16), (((((locals.var_nqs_a_dn17 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn17)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn17), (((((locals.var_nqs_a_dn18 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn18)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn18), (((((locals.var_nqs_a_dn19 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn19)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn19), (((((locals.var_nqs_a_dn20 * locals.var_gp2) - (locals.var_nqs_a * locals.var_gp2_dn20)) / (locals.var_gp2 * locals.var_gp2)) / assign82260_e123259) - locals.var_nqs_eta_dn20),)
    } else {
        (locals.var_nqs_tau, locals.var_nqs_tau_dn5, locals.var_nqs_tau_dn6, locals.var_nqs_tau_dn7, locals.var_nqs_tau_dn8, locals.var_nqs_tau_dn12, locals.var_nqs_tau_dn13, locals.var_nqs_tau_dn14, locals.var_nqs_tau_dn15, locals.var_nqs_tau_dn16, locals.var_nqs_tau_dn17, locals.var_nqs_tau_dn18, locals.var_nqs_tau_dn19, locals.var_nqs_tau_dn20,)
    }
};
        locals.var_nqs_tau = assign82260_e123264;
        locals.var_nqs_tau_dn5 = assign82260_e123264_d_n5;
        locals.var_nqs_tau_dn6 = assign82260_e123264_d_n6;
        locals.var_nqs_tau_dn7 = assign82260_e123264_d_n7;
        locals.var_nqs_tau_dn8 = assign82260_e123264_d_n8;
        locals.var_nqs_tau_dn12 = assign82260_e123264_d_n12;
        locals.var_nqs_tau_dn13 = assign82260_e123264_d_n13;
        locals.var_nqs_tau_dn14 = assign82260_e123264_d_n14;
        locals.var_nqs_tau_dn15 = assign82260_e123264_d_n15;
        locals.var_nqs_tau_dn16 = assign82260_e123264_d_n16;
        locals.var_nqs_tau_dn17 = assign82260_e123264_d_n17;
        locals.var_nqs_tau_dn18 = assign82260_e123264_d_n18;
        locals.var_nqs_tau_dn19 = assign82260_e123264_d_n19;
        locals.var_nqs_tau_dn20 = assign82260_e123264_d_n20;
        locals.var_nqs_tau_rv = 0.0;

        let (assign82270_e123289, assign82270_e123289_d_n5, assign82270_e123289_d_n6, assign82270_e123289_d_n7, assign82270_e123289_d_n8, assign82270_e123289_d_n12, assign82270_e123289_d_n13, assign82270_e123289_d_n14, assign82270_e123289_d_n15, assign82270_e123289_d_n16, assign82270_e123289_d_n17, assign82270_e123289_d_n18, assign82270_e123289_d_n19, assign82270_e123289_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82270_e123287: f64 = (locals.var_nqs_a + locals.var_nqs_c);
        (assign82270_e123287, (locals.var_nqs_a_dn5 + locals.var_nqs_c_dn5), (locals.var_nqs_a_dn6 + locals.var_nqs_c_dn6), (locals.var_nqs_a_dn7 + locals.var_nqs_c_dn7), (locals.var_nqs_a_dn8 + locals.var_nqs_c_dn8), (locals.var_nqs_a_dn12 + locals.var_nqs_c_dn12), (locals.var_nqs_a_dn13 + locals.var_nqs_c_dn13), (locals.var_nqs_a_dn14 + locals.var_nqs_c_dn14), (locals.var_nqs_a_dn15 + locals.var_nqs_c_dn15), (locals.var_nqs_a_dn16 + locals.var_nqs_c_dn16), (locals.var_nqs_a_dn17 + locals.var_nqs_c_dn17), (locals.var_nqs_a_dn18 + locals.var_nqs_c_dn18), (locals.var_nqs_a_dn19 + locals.var_nqs_c_dn19), (locals.var_nqs_a_dn20 + locals.var_nqs_c_dn20),)
    } else {
        (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn12, locals.var_nu_dn13, locals.var_nu_dn14, locals.var_nu_dn15, locals.var_nu_dn16, locals.var_nu_dn17, locals.var_nu_dn18, locals.var_nu_dn19, locals.var_nu_dn20,)
    }
};
        locals.var_nu = assign82270_e123289;
        locals.var_nu_dn5 = assign82270_e123289_d_n5;
        locals.var_nu_dn6 = assign82270_e123289_d_n6;
        locals.var_nu_dn7 = assign82270_e123289_d_n7;
        locals.var_nu_dn8 = assign82270_e123289_d_n8;
        locals.var_nu_dn12 = assign82270_e123289_d_n12;
        locals.var_nu_dn13 = assign82270_e123289_d_n13;
        locals.var_nu_dn14 = assign82270_e123289_d_n14;
        locals.var_nu_dn15 = assign82270_e123289_d_n15;
        locals.var_nu_dn16 = assign82270_e123289_d_n16;
        locals.var_nu_dn17 = assign82270_e123289_d_n17;
        locals.var_nu_dn18 = assign82270_e123289_d_n18;
        locals.var_nu_dn19 = assign82270_e123289_d_n19;
        locals.var_nu_dn20 = assign82270_e123289_d_n20;
        locals.var_nu_rv = 0.0;

        let (assign82280_e123324, assign82280_e123324_d_n5, assign82280_e123324_d_n6, assign82280_e123324_d_n7, assign82280_e123324_d_n8, assign82280_e123324_d_n12, assign82280_e123324_d_n13, assign82280_e123324_d_n14, assign82280_e123324_d_n15, assign82280_e123324_d_n16, assign82280_e123324_d_n17, assign82280_e123324_d_n18, assign82280_e123324_d_n19, assign82280_e123324_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82280_e123312: f64 = (locals.var_nu * locals.var_nu);
        let assign82280_e123317: f64 = (locals.var_nqs_c * locals.var_nqs_c);
        let assign82280_e123318: f64 = (0.5 * assign82280_e123317);
        let assign82280_e123320: f64 = (assign82280_e123318 - locals.var_nqs_a);
        let assign82280_e123321: f64 = (locals.var_nqs_tau * assign82280_e123320);
        let assign82280_e123322: f64 = (assign82280_e123312 + assign82280_e123321);
        (assign82280_e123322, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_nqs_tau_dn5 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn5 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn5))) - locals.var_nqs_a_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_nqs_tau_dn6 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn6 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn6))) - locals.var_nqs_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_nqs_tau_dn7 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn7 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn7))) - locals.var_nqs_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_nqs_tau_dn8 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn8 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn8))) - locals.var_nqs_a_dn8)))), (((locals.var_nu_dn12 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn12)) + ((locals.var_nqs_tau_dn12 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn12 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn12))) - locals.var_nqs_a_dn12)))), (((locals.var_nu_dn13 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn13)) + ((locals.var_nqs_tau_dn13 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn13 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn13))) - locals.var_nqs_a_dn13)))), (((locals.var_nu_dn14 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn14)) + ((locals.var_nqs_tau_dn14 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn14 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn14))) - locals.var_nqs_a_dn14)))), (((locals.var_nu_dn15 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn15)) + ((locals.var_nqs_tau_dn15 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn15 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn15))) - locals.var_nqs_a_dn15)))), (((locals.var_nu_dn16 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn16)) + ((locals.var_nqs_tau_dn16 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn16 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn16))) - locals.var_nqs_a_dn16)))), (((locals.var_nu_dn17 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn17)) + ((locals.var_nqs_tau_dn17 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn17 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn17))) - locals.var_nqs_a_dn17)))), (((locals.var_nu_dn18 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn18)) + ((locals.var_nqs_tau_dn18 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn18 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn18))) - locals.var_nqs_a_dn18)))), (((locals.var_nu_dn19 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn19)) + ((locals.var_nqs_tau_dn19 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn19 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn19))) - locals.var_nqs_a_dn19)))), (((locals.var_nu_dn20 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn20)) + ((locals.var_nqs_tau_dn20 * assign82280_e123320) + (locals.var_nqs_tau * ((0.5 * ((locals.var_nqs_c_dn20 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn20))) - locals.var_nqs_a_dn20)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn12, locals.var_mutau_dn13, locals.var_mutau_dn14, locals.var_mutau_dn15, locals.var_mutau_dn16, locals.var_mutau_dn17, locals.var_mutau_dn18, locals.var_mutau_dn19, locals.var_mutau_dn20,)
    }
};
        locals.var_mutau = assign82280_e123324;
        locals.var_mutau_dn5 = assign82280_e123324_d_n5;
        locals.var_mutau_dn6 = assign82280_e123324_d_n6;
        locals.var_mutau_dn7 = assign82280_e123324_d_n7;
        locals.var_mutau_dn8 = assign82280_e123324_d_n8;
        locals.var_mutau_dn12 = assign82280_e123324_d_n12;
        locals.var_mutau_dn13 = assign82280_e123324_d_n13;
        locals.var_mutau_dn14 = assign82280_e123324_d_n14;
        locals.var_mutau_dn15 = assign82280_e123324_d_n15;
        locals.var_mutau_dn16 = assign82280_e123324_d_n16;
        locals.var_mutau_dn17 = assign82280_e123324_d_n17;
        locals.var_mutau_dn18 = assign82280_e123324_d_n18;
        locals.var_mutau_dn19 = assign82280_e123324_d_n19;
        locals.var_mutau_dn20 = assign82280_e123324_d_n20;
        locals.var_mutau_rv = 0.0;

        let (assign82290_e123373, assign82290_e123373_d_n5, assign82290_e123373_d_n6, assign82290_e123373_d_n7, assign82290_e123373_d_n8, assign82290_e123373_d_n12, assign82290_e123373_d_n13, assign82290_e123373_d_n14, assign82290_e123373_d_n15, assign82290_e123373_d_n16, assign82290_e123373_d_n17, assign82290_e123373_d_n18, assign82290_e123373_d_n19, assign82290_e123373_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82290_e123348: f64 = (locals.var_nqs_a * locals.var_nu);
        let assign82290_e123350: f64 = (assign82290_e123348 * locals.var_nqs_tau);
        let assign82290_e123354: f64 = (locals.var_nu / locals.var_mutau);
        let assign82290_e123356: f64 = (assign82290_e123354 * locals.var_nqs_tau);
        let assign82290_e123358: f64 = (assign82290_e123356 * locals.var_nqs_tau);
        let assign82290_e123360: f64 = (assign82290_e123358 * locals.var_nqs_c);
        let assign82290_e123363: f64 = (locals.var_nqs_c * locals.var_nqs_c);
        let assign82290_e123365: f64 = (assign82290_e123363 * 0.3333333333333333);
        let assign82290_e123367: f64 = (assign82290_e123365 - locals.var_nqs_a);
        let assign82290_e123368: f64 = (assign82290_e123360 * assign82290_e123367);
        let assign82290_e123369: f64 = (locals.var_mutau + assign82290_e123368);
        let assign82290_e123370: f64 = (assign82290_e123350 / assign82290_e123369);
        let assign82290_e123371: f64 = (locals.var_nqs_eta + assign82290_e123370);
        (assign82290_e123371, (locals.var_nqs_eta_dn5 + (((((((locals.var_nqs_a_dn5 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn5)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn5)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn5)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn5)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn5)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn5 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn5)) * 0.3333333333333333) - locals.var_nqs_a_dn5)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn6 + (((((((locals.var_nqs_a_dn6 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn6)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn6)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn6)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn6)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn6)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn6 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn6)) * 0.3333333333333333) - locals.var_nqs_a_dn6)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn7 + (((((((locals.var_nqs_a_dn7 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn7)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn7)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn7)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn7)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn7)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn7 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn7)) * 0.3333333333333333) - locals.var_nqs_a_dn7)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn8 + (((((((locals.var_nqs_a_dn8 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn8)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn8)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn8)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn8)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn8)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn8 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn8)) * 0.3333333333333333) - locals.var_nqs_a_dn8)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn12 + (((((((locals.var_nqs_a_dn12 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn12)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn12)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn12 + (((((((((((locals.var_nu_dn12 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn12)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn12)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn12)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn12)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn12 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn12)) * 0.3333333333333333) - locals.var_nqs_a_dn12)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn13 + (((((((locals.var_nqs_a_dn13 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn13)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn13)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn13 + (((((((((((locals.var_nu_dn13 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn13)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn13)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn13)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn13)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn13 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn13)) * 0.3333333333333333) - locals.var_nqs_a_dn13)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn14 + (((((((locals.var_nqs_a_dn14 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn14)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn14)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn14 + (((((((((((locals.var_nu_dn14 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn14)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn14)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn14)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn14)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn14 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn14)) * 0.3333333333333333) - locals.var_nqs_a_dn14)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn15 + (((((((locals.var_nqs_a_dn15 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn15)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn15)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn15 + (((((((((((locals.var_nu_dn15 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn15)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn15)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn15)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn15)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn15 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn15)) * 0.3333333333333333) - locals.var_nqs_a_dn15)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn16 + (((((((locals.var_nqs_a_dn16 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn16)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn16)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn16 + (((((((((((locals.var_nu_dn16 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn16)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn16)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn16)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn16)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn16 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn16)) * 0.3333333333333333) - locals.var_nqs_a_dn16)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn17 + (((((((locals.var_nqs_a_dn17 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn17)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn17)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn17 + (((((((((((locals.var_nu_dn17 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn17)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn17)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn17)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn17)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn17 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn17)) * 0.3333333333333333) - locals.var_nqs_a_dn17)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn18 + (((((((locals.var_nqs_a_dn18 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn18)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn18)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn18 + (((((((((((locals.var_nu_dn18 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn18)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn18)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn18)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn18)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn18 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn18)) * 0.3333333333333333) - locals.var_nqs_a_dn18)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn19 + (((((((locals.var_nqs_a_dn19 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn19)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn19)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn19 + (((((((((((locals.var_nu_dn19 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn19)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn19)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn19)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn19)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn19 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn19)) * 0.3333333333333333) - locals.var_nqs_a_dn19)))))) / (assign82290_e123369 * assign82290_e123369))), (locals.var_nqs_eta_dn20 + (((((((locals.var_nqs_a_dn20 * locals.var_nu) + (locals.var_nqs_a * locals.var_nu_dn20)) * locals.var_nqs_tau) + (assign82290_e123348 * locals.var_nqs_tau_dn20)) * assign82290_e123369) - (assign82290_e123350 * (locals.var_mutau_dn20 + (((((((((((locals.var_nu_dn20 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn20)) / (locals.var_mutau * locals.var_mutau)) * locals.var_nqs_tau) + (assign82290_e123354 * locals.var_nqs_tau_dn20)) * locals.var_nqs_tau) + (assign82290_e123356 * locals.var_nqs_tau_dn20)) * locals.var_nqs_c) + (assign82290_e123358 * locals.var_nqs_c_dn20)) * assign82290_e123367) + (assign82290_e123360 * ((((locals.var_nqs_c_dn20 * locals.var_nqs_c) + (locals.var_nqs_c * locals.var_nqs_c_dn20)) * 0.3333333333333333) - locals.var_nqs_a_dn20)))))) / (assign82290_e123369 * assign82290_e123369))),)
    } else {
        (locals.var_nqs_y0, locals.var_nqs_y0_dn5, locals.var_nqs_y0_dn6, locals.var_nqs_y0_dn7, locals.var_nqs_y0_dn8, locals.var_nqs_y0_dn12, locals.var_nqs_y0_dn13, locals.var_nqs_y0_dn14, locals.var_nqs_y0_dn15, locals.var_nqs_y0_dn16, locals.var_nqs_y0_dn17, locals.var_nqs_y0_dn18, locals.var_nqs_y0_dn19, locals.var_nqs_y0_dn20,)
    }
};
        locals.var_nqs_y0 = assign82290_e123373;
        locals.var_nqs_y0_dn5 = assign82290_e123373_d_n5;
        locals.var_nqs_y0_dn6 = assign82290_e123373_d_n6;
        locals.var_nqs_y0_dn7 = assign82290_e123373_d_n7;
        locals.var_nqs_y0_dn8 = assign82290_e123373_d_n8;
        locals.var_nqs_y0_dn12 = assign82290_e123373_d_n12;
        locals.var_nqs_y0_dn13 = assign82290_e123373_d_n13;
        locals.var_nqs_y0_dn14 = assign82290_e123373_d_n14;
        locals.var_nqs_y0_dn15 = assign82290_e123373_d_n15;
        locals.var_nqs_y0_dn16 = assign82290_e123373_d_n16;
        locals.var_nqs_y0_dn17 = assign82290_e123373_d_n17;
        locals.var_nqs_y0_dn18 = assign82290_e123373_d_n18;
        locals.var_nqs_y0_dn19 = assign82290_e123373_d_n19;
        locals.var_nqs_y0_dn20 = assign82290_e123373_d_n20;
        locals.var_nqs_y0_rv = 0.0;

        let assign82300_e123375: f64 = (locals.var_nqs_y0).abs();
        let assign82300_e123377: f64 = if assign82300_e123375 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2238 = assign82300_e123377;
        locals.var_guard2238_rv = 0.0;

        let (assign82310_e123403, assign82310_e123403_d_n5, assign82310_e123403_d_n6, assign82310_e123403_d_n7, assign82310_e123403_d_n8, assign82310_e123403_d_n12, assign82310_e123403_d_n13, assign82310_e123403_d_n14, assign82310_e123403_d_n15, assign82310_e123403_d_n16, assign82310_e123403_d_n17, assign82310_e123403_d_n18, assign82310_e123403_d_n19, assign82310_e123403_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign82310_e123401: f64 = (locals.var_nqs_y0).exp();
        (assign82310_e123401, (assign82310_e123401 * locals.var_nqs_y0_dn5), (assign82310_e123401 * locals.var_nqs_y0_dn6), (assign82310_e123401 * locals.var_nqs_y0_dn7), (assign82310_e123401 * locals.var_nqs_y0_dn8), (assign82310_e123401 * locals.var_nqs_y0_dn12), (assign82310_e123401 * locals.var_nqs_y0_dn13), (assign82310_e123401 * locals.var_nqs_y0_dn14), (assign82310_e123401 * locals.var_nqs_y0_dn15), (assign82310_e123401 * locals.var_nqs_y0_dn16), (assign82310_e123401 * locals.var_nqs_y0_dn17), (assign82310_e123401 * locals.var_nqs_y0_dn18), (assign82310_e123401 * locals.var_nqs_y0_dn19), (assign82310_e123401 * locals.var_nqs_y0_dn20),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82310_e123403;
        locals.var_nqs_d0_dn5 = assign82310_e123403_d_n5;
        locals.var_nqs_d0_dn6 = assign82310_e123403_d_n6;
        locals.var_nqs_d0_dn7 = assign82310_e123403_d_n7;
        locals.var_nqs_d0_dn8 = assign82310_e123403_d_n8;
        locals.var_nqs_d0_dn12 = assign82310_e123403_d_n12;
        locals.var_nqs_d0_dn13 = assign82310_e123403_d_n13;
        locals.var_nqs_d0_dn14 = assign82310_e123403_d_n14;
        locals.var_nqs_d0_dn15 = assign82310_e123403_d_n15;
        locals.var_nqs_d0_dn16 = assign82310_e123403_d_n16;
        locals.var_nqs_d0_dn17 = assign82310_e123403_d_n17;
        locals.var_nqs_d0_dn18 = assign82310_e123403_d_n18;
        locals.var_nqs_d0_dn19 = assign82310_e123403_d_n19;
        locals.var_nqs_d0_dn20 = assign82310_e123403_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let assign82320_e123406: f64 = if locals.var_nqs_y0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2239 = assign82320_e123406;
        locals.var_guard2239_rv = 0.0;

        let (assign82330_e123459, assign82330_e123459_d_n5, assign82330_e123459_d_n6, assign82330_e123459_d_n7, assign82330_e123459_d_n8, assign82330_e123459_d_n12, assign82330_e123459_d_n13, assign82330_e123459_d_n14, assign82330_e123459_d_n15, assign82330_e123459_d_n16, assign82330_e123459_d_n17, assign82330_e123459_d_n18, assign82330_e123459_d_n19, assign82330_e123459_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) && (locals.var_guard2239 != 0.0)) {
        let assign82330_e123435: f64 = (-230.25850929940458);
        let assign82330_e123437: f64 = (assign82330_e123435 - locals.var_nqs_y0);
        let assign82330_e123441: f64 = (-230.25850929940458);
        let assign82330_e123443: f64 = (assign82330_e123441 - locals.var_nqs_y0);
        let assign82330_e123446: f64 = (-230.25850929940458);
        let assign82330_e123448: f64 = (assign82330_e123446 - locals.var_nqs_y0);
        let assign82330_e123450: f64 = (assign82330_e123448 * 0.3333333333333333);
        let assign82330_e123451: f64 = (1.0 + assign82330_e123450);
        let assign82330_e123452: f64 = (assign82330_e123443 * assign82330_e123451);
        let assign82330_e123453: f64 = (0.5 * assign82330_e123452);
        let assign82330_e123454: f64 = (1.0 + assign82330_e123453);
        let assign82330_e123455: f64 = (assign82330_e123437 * assign82330_e123454);
        let assign82330_e123456: f64 = (1.0 + assign82330_e123455);
        let assign82330_e123457: f64 = (1e-100 / assign82330_e123456);
        (assign82330_e123457, (-((1e-100 * (((-locals.var_nqs_y0_dn5) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn5) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn5) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn6) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn6) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn6) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn7) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn7) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn7) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn8) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn8) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn8) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn12) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn12) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn12) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn13) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn13) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn13) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn14) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn14) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn14) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn15) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn15) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn15) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn16) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn16) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn16) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn17) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn17) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn17) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn18) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn18) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn18) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn19) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn19) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn19) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-locals.var_nqs_y0_dn20) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-locals.var_nqs_y0_dn20) * assign82330_e123451) + (assign82330_e123443 * ((-locals.var_nqs_y0_dn20) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82330_e123459;
        locals.var_nqs_d0_dn5 = assign82330_e123459_d_n5;
        locals.var_nqs_d0_dn6 = assign82330_e123459_d_n6;
        locals.var_nqs_d0_dn7 = assign82330_e123459_d_n7;
        locals.var_nqs_d0_dn8 = assign82330_e123459_d_n8;
        locals.var_nqs_d0_dn12 = assign82330_e123459_d_n12;
        locals.var_nqs_d0_dn13 = assign82330_e123459_d_n13;
        locals.var_nqs_d0_dn14 = assign82330_e123459_d_n14;
        locals.var_nqs_d0_dn15 = assign82330_e123459_d_n15;
        locals.var_nqs_d0_dn16 = assign82330_e123459_d_n16;
        locals.var_nqs_d0_dn17 = assign82330_e123459_d_n17;
        locals.var_nqs_d0_dn18 = assign82330_e123459_d_n18;
        locals.var_nqs_d0_dn19 = assign82330_e123459_d_n19;
        locals.var_nqs_d0_dn20 = assign82330_e123459_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign82340_e123510, assign82340_e123510_d_n5, assign82340_e123510_d_n6, assign82340_e123510_d_n7, assign82340_e123510_d_n8, assign82340_e123510_d_n12, assign82340_e123510_d_n13, assign82340_e123510_d_n14, assign82340_e123510_d_n15, assign82340_e123510_d_n16, assign82340_e123510_d_n17, assign82340_e123510_d_n18, assign82340_e123510_d_n19, assign82340_e123510_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) && (locals.var_guard2239 == 0.0)) {
        let assign82340_e123490: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign82340_e123495: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign82340_e123499: f64 = (locals.var_nqs_y0 - 230.25850929940458);
        let assign82340_e123501: f64 = (assign82340_e123499 * 0.3333333333333333);
        let assign82340_e123502: f64 = (1.0 + assign82340_e123501);
        let assign82340_e123503: f64 = (assign82340_e123495 * assign82340_e123502);
        let assign82340_e123504: f64 = (0.5 * assign82340_e123503);
        let assign82340_e123505: f64 = (1.0 + assign82340_e123504);
        let assign82340_e123506: f64 = (assign82340_e123490 * assign82340_e123505);
        let assign82340_e123507: f64 = (1.0 + assign82340_e123506);
        let assign82340_e123508: f64 = (1e100 * assign82340_e123507);
        (assign82340_e123508, (1e100 * ((locals.var_nqs_y0_dn5 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn5 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn6 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn6 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn7 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn7 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn8 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn8 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn12 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn12 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn12 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn13 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn13 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn13 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn14 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn14 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn14 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn15 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn15 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn15 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn16 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn16 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn16 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn17 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn17 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn17 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn18 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn18 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn18 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn19 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn19 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn19 * 0.3333333333333333))))))), (1e100 * ((locals.var_nqs_y0_dn20 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((locals.var_nqs_y0_dn20 * assign82340_e123502) + (assign82340_e123495 * (locals.var_nqs_y0_dn20 * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82340_e123510;
        locals.var_nqs_d0_dn5 = assign82340_e123510_d_n5;
        locals.var_nqs_d0_dn6 = assign82340_e123510_d_n6;
        locals.var_nqs_d0_dn7 = assign82340_e123510_d_n7;
        locals.var_nqs_d0_dn8 = assign82340_e123510_d_n8;
        locals.var_nqs_d0_dn12 = assign82340_e123510_d_n12;
        locals.var_nqs_d0_dn13 = assign82340_e123510_d_n13;
        locals.var_nqs_d0_dn14 = assign82340_e123510_d_n14;
        locals.var_nqs_d0_dn15 = assign82340_e123510_d_n15;
        locals.var_nqs_d0_dn16 = assign82340_e123510_d_n16;
        locals.var_nqs_d0_dn17 = assign82340_e123510_d_n17;
        locals.var_nqs_d0_dn18 = assign82340_e123510_d_n18;
        locals.var_nqs_d0_dn19 = assign82340_e123510_d_n19;
        locals.var_nqs_d0_dn20 = assign82340_e123510_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign82350_e123539, assign82350_e123539_d_n5, assign82350_e123539_d_n6, assign82350_e123539_d_n7, assign82350_e123539_d_n8, assign82350_e123539_d_n12, assign82350_e123539_d_n13, assign82350_e123539_d_n14, assign82350_e123539_d_n15, assign82350_e123539_d_n16, assign82350_e123539_d_n17, assign82350_e123539_d_n18, assign82350_e123539_d_n19, assign82350_e123539_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82350_e123534: f64 = (locals.var_gp2 * locals.var_nqs_d0);
        let assign82350_e123536: f64 = (assign82350_e123534 * 0.5);
        let assign82350_e123537: f64 = (1.0 - assign82350_e123536);
        (assign82350_e123537, (-(((locals.var_gp2_dn5 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn5)) * 0.5)), (-(((locals.var_gp2_dn6 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn6)) * 0.5)), (-(((locals.var_gp2_dn7 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn7)) * 0.5)), (-(((locals.var_gp2_dn8 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn8)) * 0.5)), (-(((locals.var_gp2_dn12 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn12)) * 0.5)), (-(((locals.var_gp2_dn13 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn13)) * 0.5)), (-(((locals.var_gp2_dn14 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn14)) * 0.5)), (-(((locals.var_gp2_dn15 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn15)) * 0.5)), (-(((locals.var_gp2_dn16 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn16)) * 0.5)), (-(((locals.var_gp2_dn17 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn17)) * 0.5)), (-(((locals.var_gp2_dn18 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn18)) * 0.5)), (-(((locals.var_gp2_dn19 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn19)) * 0.5)), (-(((locals.var_gp2_dn20 * locals.var_nqs_d0) + (locals.var_gp2 * locals.var_nqs_d0_dn20)) * 0.5)),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign82350_e123539;
        locals.var_nqs_xi_dn5 = assign82350_e123539_d_n5;
        locals.var_nqs_xi_dn6 = assign82350_e123539_d_n6;
        locals.var_nqs_xi_dn7 = assign82350_e123539_d_n7;
        locals.var_nqs_xi_dn8 = assign82350_e123539_d_n8;
        locals.var_nqs_xi_dn12 = assign82350_e123539_d_n12;
        locals.var_nqs_xi_dn13 = assign82350_e123539_d_n13;
        locals.var_nqs_xi_dn14 = assign82350_e123539_d_n14;
        locals.var_nqs_xi_dn15 = assign82350_e123539_d_n15;
        locals.var_nqs_xi_dn16 = assign82350_e123539_d_n16;
        locals.var_nqs_xi_dn17 = assign82350_e123539_d_n17;
        locals.var_nqs_xi_dn18 = assign82350_e123539_d_n18;
        locals.var_nqs_xi_dn19 = assign82350_e123539_d_n19;
        locals.var_nqs_xi_dn20 = assign82350_e123539_d_n20;
        locals.var_nqs_xi_rv = 0.0;

        let (assign82360_e123572, assign82360_e123572_d_n5, assign82360_e123572_d_n6, assign82360_e123572_d_n7, assign82360_e123572_d_n8, assign82360_e123572_d_n12, assign82360_e123572_d_n13, assign82360_e123572_d_n14, assign82360_e123572_d_n15, assign82360_e123572_d_n16, assign82360_e123572_d_n17, assign82360_e123572_d_n18, assign82360_e123572_d_n19, assign82360_e123572_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82360_e123563: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign82360_e123564: f64 = (2.0 * assign82360_e123563);
        let assign82360_e123568: f64 = (locals.var_nqs_d0 - 1.0);
        let assign82360_e123569: f64 = (locals.var_gp2 * assign82360_e123568);
        let assign82360_e123570: f64 = (assign82360_e123564 + assign82360_e123569);
        (assign82360_e123570, ((2.0 * (locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5)) + ((locals.var_gp2_dn5 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn5))), ((2.0 * (locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6)) + ((locals.var_gp2_dn6 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn6))), ((2.0 * (locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7)) + ((locals.var_gp2_dn7 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn7))), ((2.0 * (locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8)) + ((locals.var_gp2_dn8 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn8))), ((2.0 * (locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12)) + ((locals.var_gp2_dn12 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn12))), ((2.0 * (locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13)) + ((locals.var_gp2_dn13 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn13))), ((2.0 * (locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14)) + ((locals.var_gp2_dn14 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn14))), ((2.0 * (locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15)) + ((locals.var_gp2_dn15 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn15))), ((2.0 * (locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16)) + ((locals.var_gp2_dn16 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn16))), ((2.0 * (locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17)) + ((locals.var_gp2_dn17 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn17))), ((2.0 * (locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18)) + ((locals.var_gp2_dn18 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn18))), ((2.0 * (locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19)) + ((locals.var_gp2_dn19 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn19))), ((2.0 * (locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20)) + ((locals.var_gp2_dn20 * assign82360_e123568) + (locals.var_gp2 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign82360_e123572;
        locals.var_nqs_p_dn5 = assign82360_e123572_d_n5;
        locals.var_nqs_p_dn6 = assign82360_e123572_d_n6;
        locals.var_nqs_p_dn7 = assign82360_e123572_d_n7;
        locals.var_nqs_p_dn8 = assign82360_e123572_d_n8;
        locals.var_nqs_p_dn12 = assign82360_e123572_d_n12;
        locals.var_nqs_p_dn13 = assign82360_e123572_d_n13;
        locals.var_nqs_p_dn14 = assign82360_e123572_d_n14;
        locals.var_nqs_p_dn15 = assign82360_e123572_d_n15;
        locals.var_nqs_p_dn16 = assign82360_e123572_d_n16;
        locals.var_nqs_p_dn17 = assign82360_e123572_d_n17;
        locals.var_nqs_p_dn18 = assign82360_e123572_d_n18;
        locals.var_nqs_p_dn19 = assign82360_e123572_d_n19;
        locals.var_nqs_p_dn20 = assign82360_e123572_d_n20;
        locals.var_nqs_p_rv = 0.0;

        let (assign82370_e123609, assign82370_e123609_d_n5, assign82370_e123609_d_n6, assign82370_e123609_d_n7, assign82370_e123609_d_n8, assign82370_e123609_d_n12, assign82370_e123609_d_n13, assign82370_e123609_d_n14, assign82370_e123609_d_n15, assign82370_e123609_d_n16, assign82370_e123609_d_n17, assign82370_e123609_d_n18, assign82370_e123609_d_n19, assign82370_e123609_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82370_e123595: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign82370_e123598: f64 = (locals.var_nqs_yg - locals.var_nqs_y0);
        let assign82370_e123599: f64 = (assign82370_e123595 * assign82370_e123598);
        let assign82370_e123603: f64 = (locals.var_nqs_y0 + 1.0);
        let assign82370_e123605: f64 = (assign82370_e123603 - locals.var_nqs_d0);
        let assign82370_e123606: f64 = (locals.var_gp2 * assign82370_e123605);
        let assign82370_e123607: f64 = (assign82370_e123599 + assign82370_e123606);
        (assign82370_e123607, ((((locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn5 - locals.var_nqs_y0_dn5))) + ((locals.var_gp2_dn5 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn5 - locals.var_nqs_d0_dn5)))), ((((locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn6 - locals.var_nqs_y0_dn6))) + ((locals.var_gp2_dn6 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn6 - locals.var_nqs_d0_dn6)))), ((((locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn7 - locals.var_nqs_y0_dn7))) + ((locals.var_gp2_dn7 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn7 - locals.var_nqs_d0_dn7)))), ((((locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn8 - locals.var_nqs_y0_dn8))) + ((locals.var_gp2_dn8 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn8 - locals.var_nqs_d0_dn8)))), ((((locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn12 - locals.var_nqs_y0_dn12))) + ((locals.var_gp2_dn12 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn12 - locals.var_nqs_d0_dn12)))), ((((locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn13 - locals.var_nqs_y0_dn13))) + ((locals.var_gp2_dn13 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn13 - locals.var_nqs_d0_dn13)))), ((((locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn14 - locals.var_nqs_y0_dn14))) + ((locals.var_gp2_dn14 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn14 - locals.var_nqs_d0_dn14)))), ((((locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn15 - locals.var_nqs_y0_dn15))) + ((locals.var_gp2_dn15 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn15 - locals.var_nqs_d0_dn15)))), ((((locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn16 - locals.var_nqs_y0_dn16))) + ((locals.var_gp2_dn16 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn16 - locals.var_nqs_d0_dn16)))), ((((locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn17 - locals.var_nqs_y0_dn17))) + ((locals.var_gp2_dn17 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn17 - locals.var_nqs_d0_dn17)))), ((((locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn18 - locals.var_nqs_y0_dn18))) + ((locals.var_gp2_dn18 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn18 - locals.var_nqs_d0_dn18)))), ((((locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn19 - locals.var_nqs_y0_dn19))) + ((locals.var_gp2_dn19 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn19 - locals.var_nqs_d0_dn19)))), ((((locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20) * assign82370_e123598) + (assign82370_e123595 * (locals.var_nqs_yg_dn20 - locals.var_nqs_y0_dn20))) + ((locals.var_gp2_dn20 * assign82370_e123605) + (locals.var_gp2 * (locals.var_nqs_y0_dn20 - locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign82370_e123609;
        locals.var_nqs_q_dn5 = assign82370_e123609_d_n5;
        locals.var_nqs_q_dn6 = assign82370_e123609_d_n6;
        locals.var_nqs_q_dn7 = assign82370_e123609_d_n7;
        locals.var_nqs_q_dn8 = assign82370_e123609_d_n8;
        locals.var_nqs_q_dn12 = assign82370_e123609_d_n12;
        locals.var_nqs_q_dn13 = assign82370_e123609_d_n13;
        locals.var_nqs_q_dn14 = assign82370_e123609_d_n14;
        locals.var_nqs_q_dn15 = assign82370_e123609_d_n15;
        locals.var_nqs_q_dn16 = assign82370_e123609_d_n16;
        locals.var_nqs_q_dn17 = assign82370_e123609_d_n17;
        locals.var_nqs_q_dn18 = assign82370_e123609_d_n18;
        locals.var_nqs_q_dn19 = assign82370_e123609_d_n19;
        locals.var_nqs_q_dn20 = assign82370_e123609_d_n20;
        locals.var_nqs_q_rv = 0.0;

        let (assign82380_e123640, assign82380_e123640_d_n5, assign82380_e123640_d_n6, assign82380_e123640_d_n7, assign82380_e123640_d_n8, assign82380_e123640_d_n12, assign82380_e123640_d_n13, assign82380_e123640_d_n14, assign82380_e123640_d_n15, assign82380_e123640_d_n16, assign82380_e123640_d_n17, assign82380_e123640_d_n18, assign82380_e123640_d_n19, assign82380_e123640_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82380_e123632: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign82380_e123635: f64 = (4.0 * locals.var_nqs_xi);
        let assign82380_e123637: f64 = (assign82380_e123635 * locals.var_nqs_q);
        let assign82380_e123638: f64 = (assign82380_e123632 - assign82380_e123637);
        (assign82380_e123638, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign82380_e123635 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82380_e123640;
        locals.var_nqs_temp_dn5 = assign82380_e123640_d_n5;
        locals.var_nqs_temp_dn6 = assign82380_e123640_d_n6;
        locals.var_nqs_temp_dn7 = assign82380_e123640_d_n7;
        locals.var_nqs_temp_dn8 = assign82380_e123640_d_n8;
        locals.var_nqs_temp_dn12 = assign82380_e123640_d_n12;
        locals.var_nqs_temp_dn13 = assign82380_e123640_d_n13;
        locals.var_nqs_temp_dn14 = assign82380_e123640_d_n14;
        locals.var_nqs_temp_dn15 = assign82380_e123640_d_n15;
        locals.var_nqs_temp_dn16 = assign82380_e123640_d_n16;
        locals.var_nqs_temp_dn17 = assign82380_e123640_d_n17;
        locals.var_nqs_temp_dn18 = assign82380_e123640_d_n18;
        locals.var_nqs_temp_dn19 = assign82380_e123640_d_n19;
        locals.var_nqs_temp_dn20 = assign82380_e123640_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82390_e123670, assign82390_e123670_d_n5, assign82390_e123670_d_n6, assign82390_e123670_d_n7, assign82390_e123670_d_n8, assign82390_e123670_d_n12, assign82390_e123670_d_n13, assign82390_e123670_d_n14, assign82390_e123670_d_n15, assign82390_e123670_d_n16, assign82390_e123670_d_n17, assign82390_e123670_d_n18, assign82390_e123670_d_n19, assign82390_e123670_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82390_e123663: f64 = (2.0 * locals.var_nqs_q);
        let assign82390_e123666: f64 = (locals.var_nqs_temp).sqrt();
        let assign82390_e123667: f64 = (locals.var_nqs_p + assign82390_e123666);
        let assign82390_e123668: f64 = (assign82390_e123663 / assign82390_e123667);
        (assign82390_e123668, ((((2.0 * locals.var_nqs_q_dn5) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn6) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn7) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn8) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn12) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn13) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn14) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn15) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn16) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn17) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn18) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn19) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * locals.var_nqs_q_dn20) * assign82390_e123667) - (assign82390_e123663 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign82390_e123670;
        locals.var_nqs_w_dn5 = assign82390_e123670_d_n5;
        locals.var_nqs_w_dn6 = assign82390_e123670_d_n6;
        locals.var_nqs_w_dn7 = assign82390_e123670_d_n7;
        locals.var_nqs_w_dn8 = assign82390_e123670_d_n8;
        locals.var_nqs_w_dn12 = assign82390_e123670_d_n12;
        locals.var_nqs_w_dn13 = assign82390_e123670_d_n13;
        locals.var_nqs_w_dn14 = assign82390_e123670_d_n14;
        locals.var_nqs_w_dn15 = assign82390_e123670_d_n15;
        locals.var_nqs_w_dn16 = assign82390_e123670_d_n16;
        locals.var_nqs_w_dn17 = assign82390_e123670_d_n17;
        locals.var_nqs_w_dn18 = assign82390_e123670_d_n18;
        locals.var_nqs_w_dn19 = assign82390_e123670_d_n19;
        locals.var_nqs_w_dn20 = assign82390_e123670_d_n20;
        locals.var_nqs_w_rv = 0.0;

        let (assign82400_e123696, assign82400_e123696_d_n5, assign82400_e123696_d_n6, assign82400_e123696_d_n7, assign82400_e123696_d_n8, assign82400_e123696_d_n12, assign82400_e123696_d_n13, assign82400_e123696_d_n14, assign82400_e123696_d_n15, assign82400_e123696_d_n16, assign82400_e123696_d_n17, assign82400_e123696_d_n18, assign82400_e123696_d_n19, assign82400_e123696_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 != 0.0)) {
        let assign82400_e123693: f64 = (locals.var_nqs_y0 + locals.var_nqs_w);
        let assign82400_e123694: f64 = (-assign82400_e123693);
        (assign82400_e123694, (-(locals.var_nqs_y0_dn5 + locals.var_nqs_w_dn5)), (-(locals.var_nqs_y0_dn6 + locals.var_nqs_w_dn6)), (-(locals.var_nqs_y0_dn7 + locals.var_nqs_w_dn7)), (-(locals.var_nqs_y0_dn8 + locals.var_nqs_w_dn8)), (-(locals.var_nqs_y0_dn12 + locals.var_nqs_w_dn12)), (-(locals.var_nqs_y0_dn13 + locals.var_nqs_w_dn13)), (-(locals.var_nqs_y0_dn14 + locals.var_nqs_w_dn14)), (-(locals.var_nqs_y0_dn15 + locals.var_nqs_w_dn15)), (-(locals.var_nqs_y0_dn16 + locals.var_nqs_w_dn16)), (-(locals.var_nqs_y0_dn17 + locals.var_nqs_w_dn17)), (-(locals.var_nqs_y0_dn18 + locals.var_nqs_w_dn18)), (-(locals.var_nqs_y0_dn19 + locals.var_nqs_w_dn19)), (-(locals.var_nqs_y0_dn20 + locals.var_nqs_w_dn20)),)
    } else {
        (locals.var_temp9, locals.var_temp9_dn5, locals.var_temp9_dn6, locals.var_temp9_dn7, locals.var_temp9_dn8, locals.var_temp9_dn12, locals.var_temp9_dn13, locals.var_temp9_dn14, locals.var_temp9_dn15, locals.var_temp9_dn16, locals.var_temp9_dn17, locals.var_temp9_dn18, locals.var_temp9_dn19, locals.var_temp9_dn20,)
    }
};
        locals.var_temp9 = assign82400_e123696;
        locals.var_temp9_dn5 = assign82400_e123696_d_n5;
        locals.var_temp9_dn6 = assign82400_e123696_d_n6;
        locals.var_temp9_dn7 = assign82400_e123696_d_n7;
        locals.var_temp9_dn8 = assign82400_e123696_d_n8;
        locals.var_temp9_dn12 = assign82400_e123696_d_n12;
        locals.var_temp9_dn13 = assign82400_e123696_d_n13;
        locals.var_temp9_dn14 = assign82400_e123696_d_n14;
        locals.var_temp9_dn15 = assign82400_e123696_d_n15;
        locals.var_temp9_dn16 = assign82400_e123696_d_n16;
        locals.var_temp9_dn17 = assign82400_e123696_d_n17;
        locals.var_temp9_dn18 = assign82400_e123696_d_n18;
        locals.var_temp9_dn19 = assign82400_e123696_d_n19;
        locals.var_temp9_dn20 = assign82400_e123696_d_n20;
        locals.var_temp9_rv = 0.0;

        let (assign82410_e123726, assign82410_e123726_d_n5, assign82410_e123726_d_n6, assign82410_e123726_d_n7, assign82410_e123726_d_n8, assign82410_e123726_d_n12, assign82410_e123726_d_n13, assign82410_e123726_d_n14, assign82410_e123726_d_n15, assign82410_e123726_d_n16, assign82410_e123726_d_n17, assign82410_e123726_d_n18, assign82410_e123726_d_n19, assign82410_e123726_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82410_e123722: f64 = (0.732464877560822 * locals.var_gp);
        let assign82410_e123723: f64 = (1.25 + assign82410_e123722);
        let assign82410_e123724: f64 = (1.0 / assign82410_e123723);
        (assign82410_e123724, (-((0.732464877560822 * locals.var_gp_dn5) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn6) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn7) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn8) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn12) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn13) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn14) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn15) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn16) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn17) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn18) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn19) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * locals.var_gp_dn20) / (assign82410_e123723 * assign82410_e123723))),)
    } else {
        (locals.var_nqs_xg1, locals.var_nqs_xg1_dn5, locals.var_nqs_xg1_dn6, locals.var_nqs_xg1_dn7, locals.var_nqs_xg1_dn8, locals.var_nqs_xg1_dn12, locals.var_nqs_xg1_dn13, locals.var_nqs_xg1_dn14, locals.var_nqs_xg1_dn15, locals.var_nqs_xg1_dn16, locals.var_nqs_xg1_dn17, locals.var_nqs_xg1_dn18, locals.var_nqs_xg1_dn19, locals.var_nqs_xg1_dn20,)
    }
};
        locals.var_nqs_xg1 = assign82410_e123726;
        locals.var_nqs_xg1_dn5 = assign82410_e123726_d_n5;
        locals.var_nqs_xg1_dn6 = assign82410_e123726_d_n6;
        locals.var_nqs_xg1_dn7 = assign82410_e123726_d_n7;
        locals.var_nqs_xg1_dn8 = assign82410_e123726_d_n8;
        locals.var_nqs_xg1_dn12 = assign82410_e123726_d_n12;
        locals.var_nqs_xg1_dn13 = assign82410_e123726_d_n13;
        locals.var_nqs_xg1_dn14 = assign82410_e123726_d_n14;
        locals.var_nqs_xg1_dn15 = assign82410_e123726_d_n15;
        locals.var_nqs_xg1_dn16 = assign82410_e123726_d_n16;
        locals.var_nqs_xg1_dn17 = assign82410_e123726_d_n17;
        locals.var_nqs_xg1_dn18 = assign82410_e123726_d_n18;
        locals.var_nqs_xg1_dn19 = assign82410_e123726_d_n19;
        locals.var_nqs_xg1_dn20 = assign82410_e123726_d_n20;
        locals.var_nqs_xg1_rv = 0.0;

        let (assign82420_e123758, assign82420_e123758_d_n5, assign82420_e123758_d_n6, assign82420_e123758_d_n7, assign82420_e123758_d_n8, assign82420_e123758_d_n12, assign82420_e123758_d_n13, assign82420_e123758_d_n14, assign82420_e123758_d_n15, assign82420_e123758_d_n16, assign82420_e123758_d_n17, assign82420_e123758_d_n18, assign82420_e123758_d_n19, assign82420_e123758_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82420_e123750: f64 = (1.25 * locals.var_a_factrp);
        let assign82420_e123752: f64 = (assign82420_e123750 * locals.var_nqs_xg1);
        let assign82420_e123754: f64 = (assign82420_e123752 - 1.0);
        let assign82420_e123756: f64 = (assign82420_e123754 * locals.var_nqs_xg1);
        (assign82420_e123756, (((((1.25 * locals.var_a_factrp_dn5) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn5)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn5)), (((((1.25 * locals.var_a_factrp_dn6) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn6)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn6)), (((((1.25 * locals.var_a_factrp_dn7) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn7)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn7)), (((((1.25 * locals.var_a_factrp_dn8) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn8)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn8)), (((((1.25 * locals.var_a_factrp_dn12) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn12)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn12)), (((((1.25 * locals.var_a_factrp_dn13) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn13)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn13)), (((((1.25 * locals.var_a_factrp_dn14) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn14)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn14)), (((((1.25 * locals.var_a_factrp_dn15) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn15)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn15)), (((((1.25 * locals.var_a_factrp_dn16) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn16)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn16)), (((((1.25 * locals.var_a_factrp_dn17) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn17)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn17)), (((((1.25 * locals.var_a_factrp_dn18) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn18)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn18)), (((((1.25 * locals.var_a_factrp_dn19) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn19)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn19)), (((((1.25 * locals.var_a_factrp_dn20) * locals.var_nqs_xg1) + (assign82420_e123750 * locals.var_nqs_xg1_dn20)) * locals.var_nqs_xg1) + (assign82420_e123754 * locals.var_nqs_xg1_dn20)),)
    } else {
        (locals.var_nqs_a_fac, locals.var_nqs_a_fac_dn5, locals.var_nqs_a_fac_dn6, locals.var_nqs_a_fac_dn7, locals.var_nqs_a_fac_dn8, locals.var_nqs_a_fac_dn12, locals.var_nqs_a_fac_dn13, locals.var_nqs_a_fac_dn14, locals.var_nqs_a_fac_dn15, locals.var_nqs_a_fac_dn16, locals.var_nqs_a_fac_dn17, locals.var_nqs_a_fac_dn18, locals.var_nqs_a_fac_dn19, locals.var_nqs_a_fac_dn20,)
    }
};
        locals.var_nqs_a_fac = assign82420_e123758;
        locals.var_nqs_a_fac_dn5 = assign82420_e123758_d_n5;
        locals.var_nqs_a_fac_dn6 = assign82420_e123758_d_n6;
        locals.var_nqs_a_fac_dn7 = assign82420_e123758_d_n7;
        locals.var_nqs_a_fac_dn8 = assign82420_e123758_d_n8;
        locals.var_nqs_a_fac_dn12 = assign82420_e123758_d_n12;
        locals.var_nqs_a_fac_dn13 = assign82420_e123758_d_n13;
        locals.var_nqs_a_fac_dn14 = assign82420_e123758_d_n14;
        locals.var_nqs_a_fac_dn15 = assign82420_e123758_d_n15;
        locals.var_nqs_a_fac_dn16 = assign82420_e123758_d_n16;
        locals.var_nqs_a_fac_dn17 = assign82420_e123758_d_n17;
        locals.var_nqs_a_fac_dn18 = assign82420_e123758_d_n18;
        locals.var_nqs_a_fac_dn19 = assign82420_e123758_d_n19;
        locals.var_nqs_a_fac_dn20 = assign82420_e123758_d_n20;
        locals.var_nqs_a_fac_rv = 0.0;

        let (assign82430_e123790, assign82430_e123790_d_n5, assign82430_e123790_d_n6, assign82430_e123790_d_n7, assign82430_e123790_d_n8, assign82430_e123790_d_n12, assign82430_e123790_d_n13, assign82430_e123790_d_n14, assign82430_e123790_d_n15, assign82430_e123790_d_n16, assign82430_e123790_d_n17, assign82430_e123790_d_n18, assign82430_e123790_d_n19, assign82430_e123790_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82430_e123782: f64 = (locals.var_temp__blk1038 / locals.var_a_factrp);
        let assign82430_e123786: f64 = (locals.var_nqs_a_fac * locals.var_temp__blk1038);
        let assign82430_e123787: f64 = (1.0 + assign82430_e123786);
        let assign82430_e123788: f64 = (assign82430_e123782 * assign82430_e123787);
        (assign82430_e123788, (((((locals.var_temp__blk1038_dn5 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn5)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn5 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn5)))), (((((locals.var_temp__blk1038_dn6 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn6)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn6 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn6)))), (((((locals.var_temp__blk1038_dn7 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn7)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn7 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn7)))), (((((locals.var_temp__blk1038_dn8 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn8)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn8 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn8)))), (((((locals.var_temp__blk1038_dn12 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn12)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn12 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn12)))), (((((locals.var_temp__blk1038_dn13 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn13)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn13 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn13)))), (((((locals.var_temp__blk1038_dn14 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn14)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn14 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn14)))), (((((locals.var_temp__blk1038_dn15 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn15)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn15 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn15)))), (((((locals.var_temp__blk1038_dn16 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn16)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn16 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn16)))), (((((locals.var_temp__blk1038_dn17 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn17)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn17 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn17)))), (((((locals.var_temp__blk1038_dn18 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn18)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn18 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn18)))), (((((locals.var_temp__blk1038_dn19 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn19)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn19 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn19)))), (((((locals.var_temp__blk1038_dn20 * locals.var_a_factrp) - (locals.var_temp__blk1038 * locals.var_a_factrp_dn20)) / (locals.var_a_factrp * locals.var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((locals.var_nqs_a_fac_dn20 * locals.var_temp__blk1038) + (locals.var_nqs_a_fac * locals.var_temp__blk1038_dn20)))),)
    } else {
        (locals.var_nqs_xbar, locals.var_nqs_xbar_dn5, locals.var_nqs_xbar_dn6, locals.var_nqs_xbar_dn7, locals.var_nqs_xbar_dn8, locals.var_nqs_xbar_dn12, locals.var_nqs_xbar_dn13, locals.var_nqs_xbar_dn14, locals.var_nqs_xbar_dn15, locals.var_nqs_xbar_dn16, locals.var_nqs_xbar_dn17, locals.var_nqs_xbar_dn18, locals.var_nqs_xbar_dn19, locals.var_nqs_xbar_dn20,)
    }
};
        locals.var_nqs_xbar = assign82430_e123790;
        locals.var_nqs_xbar_dn5 = assign82430_e123790_d_n5;
        locals.var_nqs_xbar_dn6 = assign82430_e123790_d_n6;
        locals.var_nqs_xbar_dn7 = assign82430_e123790_d_n7;
        locals.var_nqs_xbar_dn8 = assign82430_e123790_d_n8;
        locals.var_nqs_xbar_dn12 = assign82430_e123790_d_n12;
        locals.var_nqs_xbar_dn13 = assign82430_e123790_d_n13;
        locals.var_nqs_xbar_dn14 = assign82430_e123790_d_n14;
        locals.var_nqs_xbar_dn15 = assign82430_e123790_d_n15;
        locals.var_nqs_xbar_dn16 = assign82430_e123790_d_n16;
        locals.var_nqs_xbar_dn17 = assign82430_e123790_d_n17;
        locals.var_nqs_xbar_dn18 = assign82430_e123790_d_n18;
        locals.var_nqs_xbar_dn19 = assign82430_e123790_d_n19;
        locals.var_nqs_xbar_dn20 = assign82430_e123790_d_n20;
        locals.var_nqs_xbar_rv = 0.0;

        let assign82440_e123792: f64 = (-locals.var_nqs_xbar);
        let assign82440_e123793: f64 = (assign82440_e123792).abs();
        let assign82440_e123795: f64 = if assign82440_e123793 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2240 = assign82440_e123795;
        locals.var_guard2240_rv = 0.0;

        let (assign82450_e123823, assign82450_e123823_d_n5, assign82450_e123823_d_n6, assign82450_e123823_d_n7, assign82450_e123823_d_n8, assign82450_e123823_d_n12, assign82450_e123823_d_n13, assign82450_e123823_d_n14, assign82450_e123823_d_n15, assign82450_e123823_d_n16, assign82450_e123823_d_n17, assign82450_e123823_d_n18, assign82450_e123823_d_n19, assign82450_e123823_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2240 != 0.0)) {
        let assign82450_e123820: f64 = (-locals.var_nqs_xbar);
        let assign82450_e123821: f64 = (assign82450_e123820).exp();
        (assign82450_e123821, (assign82450_e123821 * (-locals.var_nqs_xbar_dn5)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn6)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn7)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn8)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn12)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn13)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn14)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn15)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn16)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn17)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn18)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn19)), (assign82450_e123821 * (-locals.var_nqs_xbar_dn20)),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82450_e123823;
        locals.var_nqs_temp_dn5 = assign82450_e123823_d_n5;
        locals.var_nqs_temp_dn6 = assign82450_e123823_d_n6;
        locals.var_nqs_temp_dn7 = assign82450_e123823_d_n7;
        locals.var_nqs_temp_dn8 = assign82450_e123823_d_n8;
        locals.var_nqs_temp_dn12 = assign82450_e123823_d_n12;
        locals.var_nqs_temp_dn13 = assign82450_e123823_d_n13;
        locals.var_nqs_temp_dn14 = assign82450_e123823_d_n14;
        locals.var_nqs_temp_dn15 = assign82450_e123823_d_n15;
        locals.var_nqs_temp_dn16 = assign82450_e123823_d_n16;
        locals.var_nqs_temp_dn17 = assign82450_e123823_d_n17;
        locals.var_nqs_temp_dn18 = assign82450_e123823_d_n18;
        locals.var_nqs_temp_dn19 = assign82450_e123823_d_n19;
        locals.var_nqs_temp_dn20 = assign82450_e123823_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let assign82460_e123825: f64 = (-locals.var_nqs_xbar);
        let assign82460_e123827: f64 = if assign82460_e123825 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2241 = assign82460_e123827;
        locals.var_guard2241_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_171(
        locals: &mut StampLocals,
    ) {
        let (assign82470_e123884, assign82470_e123884_d_n5, assign82470_e123884_d_n6, assign82470_e123884_d_n7, assign82470_e123884_d_n8, assign82470_e123884_d_n12, assign82470_e123884_d_n13, assign82470_e123884_d_n14, assign82470_e123884_d_n15, assign82470_e123884_d_n16, assign82470_e123884_d_n17, assign82470_e123884_d_n18, assign82470_e123884_d_n19, assign82470_e123884_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2240 == 0.0)) && (locals.var_guard2241 != 0.0)) {
        let assign82470_e123857: f64 = (-230.25850929940458);
        let assign82470_e123859: f64 = (-locals.var_nqs_xbar);
        let assign82470_e123860: f64 = (assign82470_e123857 - assign82470_e123859);
        let assign82470_e123864: f64 = (-230.25850929940458);
        let assign82470_e123866: f64 = (-locals.var_nqs_xbar);
        let assign82470_e123867: f64 = (assign82470_e123864 - assign82470_e123866);
        let assign82470_e123870: f64 = (-230.25850929940458);
        let assign82470_e123872: f64 = (-locals.var_nqs_xbar);
        let assign82470_e123873: f64 = (assign82470_e123870 - assign82470_e123872);
        let assign82470_e123875: f64 = (assign82470_e123873 * 0.3333333333333333);
        let assign82470_e123876: f64 = (1.0 + assign82470_e123875);
        let assign82470_e123877: f64 = (assign82470_e123867 * assign82470_e123876);
        let assign82470_e123878: f64 = (0.5 * assign82470_e123877);
        let assign82470_e123879: f64 = (1.0 + assign82470_e123878);
        let assign82470_e123880: f64 = (assign82470_e123860 * assign82470_e123879);
        let assign82470_e123881: f64 = (1.0 + assign82470_e123880);
        let assign82470_e123882: f64 = (1e-100 / assign82470_e123881);
        (assign82470_e123882, (-((1e-100 * (((-(-locals.var_nqs_xbar_dn5)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn5)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn5)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn6)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn6)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn6)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn7)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn7)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn7)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn8)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn8)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn8)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn12)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn12)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn12)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn13)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn13)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn13)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn14)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn14)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn14)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn15)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn15)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn15)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn16)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn16)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn16)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn17)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn17)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn17)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn18)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn18)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn18)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn19)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn19)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn19)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-locals.var_nqs_xbar_dn20)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-locals.var_nqs_xbar_dn20)) * assign82470_e123876) + (assign82470_e123867 * ((-(-locals.var_nqs_xbar_dn20)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82470_e123884;
        locals.var_nqs_temp_dn5 = assign82470_e123884_d_n5;
        locals.var_nqs_temp_dn6 = assign82470_e123884_d_n6;
        locals.var_nqs_temp_dn7 = assign82470_e123884_d_n7;
        locals.var_nqs_temp_dn8 = assign82470_e123884_d_n8;
        locals.var_nqs_temp_dn12 = assign82470_e123884_d_n12;
        locals.var_nqs_temp_dn13 = assign82470_e123884_d_n13;
        locals.var_nqs_temp_dn14 = assign82470_e123884_d_n14;
        locals.var_nqs_temp_dn15 = assign82470_e123884_d_n15;
        locals.var_nqs_temp_dn16 = assign82470_e123884_d_n16;
        locals.var_nqs_temp_dn17 = assign82470_e123884_d_n17;
        locals.var_nqs_temp_dn18 = assign82470_e123884_d_n18;
        locals.var_nqs_temp_dn19 = assign82470_e123884_d_n19;
        locals.var_nqs_temp_dn20 = assign82470_e123884_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82480_e123939, assign82480_e123939_d_n5, assign82480_e123939_d_n6, assign82480_e123939_d_n7, assign82480_e123939_d_n8, assign82480_e123939_d_n12, assign82480_e123939_d_n13, assign82480_e123939_d_n14, assign82480_e123939_d_n15, assign82480_e123939_d_n16, assign82480_e123939_d_n17, assign82480_e123939_d_n18, assign82480_e123939_d_n19, assign82480_e123939_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2240 == 0.0)) && (locals.var_guard2241 == 0.0)) {
        let assign82480_e123915: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123917: f64 = (assign82480_e123915 - 230.25850929940458);
        let assign82480_e123921: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123923: f64 = (assign82480_e123921 - 230.25850929940458);
        let assign82480_e123926: f64 = (-locals.var_nqs_xbar);
        let assign82480_e123928: f64 = (assign82480_e123926 - 230.25850929940458);
        let assign82480_e123930: f64 = (assign82480_e123928 * 0.3333333333333333);
        let assign82480_e123931: f64 = (1.0 + assign82480_e123930);
        let assign82480_e123932: f64 = (assign82480_e123923 * assign82480_e123931);
        let assign82480_e123933: f64 = (0.5 * assign82480_e123932);
        let assign82480_e123934: f64 = (1.0 + assign82480_e123933);
        let assign82480_e123935: f64 = (assign82480_e123917 * assign82480_e123934);
        let assign82480_e123936: f64 = (1.0 + assign82480_e123935);
        let assign82480_e123937: f64 = (1e100 * assign82480_e123936);
        (assign82480_e123937, (1e100 * (((-locals.var_nqs_xbar_dn5) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn5) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn6) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn6) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn7) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn7) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn8) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn8) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn12) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn12) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn13) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn13) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn14) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn14) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn15) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn15) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn16) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn16) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn17) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn17) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn18) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn18) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn19) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn19) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_xbar_dn20) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-locals.var_nqs_xbar_dn20) * assign82480_e123931) + (assign82480_e123923 * ((-locals.var_nqs_xbar_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82480_e123939;
        locals.var_nqs_temp_dn5 = assign82480_e123939_d_n5;
        locals.var_nqs_temp_dn6 = assign82480_e123939_d_n6;
        locals.var_nqs_temp_dn7 = assign82480_e123939_d_n7;
        locals.var_nqs_temp_dn8 = assign82480_e123939_d_n8;
        locals.var_nqs_temp_dn12 = assign82480_e123939_d_n12;
        locals.var_nqs_temp_dn13 = assign82480_e123939_d_n13;
        locals.var_nqs_temp_dn14 = assign82480_e123939_d_n14;
        locals.var_nqs_temp_dn15 = assign82480_e123939_d_n15;
        locals.var_nqs_temp_dn16 = assign82480_e123939_d_n16;
        locals.var_nqs_temp_dn17 = assign82480_e123939_d_n17;
        locals.var_nqs_temp_dn18 = assign82480_e123939_d_n18;
        locals.var_nqs_temp_dn19 = assign82480_e123939_d_n19;
        locals.var_nqs_temp_dn20 = assign82480_e123939_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82490_e123965, assign82490_e123965_d_n5, assign82490_e123965_d_n6, assign82490_e123965_d_n7, assign82490_e123965_d_n8, assign82490_e123965_d_n12, assign82490_e123965_d_n13, assign82490_e123965_d_n14, assign82490_e123965_d_n15, assign82490_e123965_d_n16, assign82490_e123965_d_n17, assign82490_e123965_d_n18, assign82490_e123965_d_n19, assign82490_e123965_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82490_e123963: f64 = (1.0 - locals.var_nqs_temp);
        (assign82490_e123963, (-locals.var_nqs_temp_dn5), (-locals.var_nqs_temp_dn6), (-locals.var_nqs_temp_dn7), (-locals.var_nqs_temp_dn8), (-locals.var_nqs_temp_dn12), (-locals.var_nqs_temp_dn13), (-locals.var_nqs_temp_dn14), (-locals.var_nqs_temp_dn15), (-locals.var_nqs_temp_dn16), (-locals.var_nqs_temp_dn17), (-locals.var_nqs_temp_dn18), (-locals.var_nqs_temp_dn19), (-locals.var_nqs_temp_dn20),)
    } else {
        (locals.var_nqs_w, locals.var_nqs_w_dn5, locals.var_nqs_w_dn6, locals.var_nqs_w_dn7, locals.var_nqs_w_dn8, locals.var_nqs_w_dn12, locals.var_nqs_w_dn13, locals.var_nqs_w_dn14, locals.var_nqs_w_dn15, locals.var_nqs_w_dn16, locals.var_nqs_w_dn17, locals.var_nqs_w_dn18, locals.var_nqs_w_dn19, locals.var_nqs_w_dn20,)
    }
};
        locals.var_nqs_w = assign82490_e123965;
        locals.var_nqs_w_dn5 = assign82490_e123965_d_n5;
        locals.var_nqs_w_dn6 = assign82490_e123965_d_n6;
        locals.var_nqs_w_dn7 = assign82490_e123965_d_n7;
        locals.var_nqs_w_dn8 = assign82490_e123965_d_n8;
        locals.var_nqs_w_dn12 = assign82490_e123965_d_n12;
        locals.var_nqs_w_dn13 = assign82490_e123965_d_n13;
        locals.var_nqs_w_dn14 = assign82490_e123965_d_n14;
        locals.var_nqs_w_dn15 = assign82490_e123965_d_n15;
        locals.var_nqs_w_dn16 = assign82490_e123965_d_n16;
        locals.var_nqs_w_dn17 = assign82490_e123965_d_n17;
        locals.var_nqs_w_dn18 = assign82490_e123965_d_n18;
        locals.var_nqs_w_dn19 = assign82490_e123965_d_n19;
        locals.var_nqs_w_dn20 = assign82490_e123965_d_n20;
        locals.var_nqs_w_rv = 0.0;

        let (assign82500_e124004, assign82500_e124004_d_n5, assign82500_e124004_d_n6, assign82500_e124004_d_n7, assign82500_e124004_d_n8, assign82500_e124004_d_n12, assign82500_e124004_d_n13, assign82500_e124004_d_n14, assign82500_e124004_d_n15, assign82500_e124004_d_n16, assign82500_e124004_d_n17, assign82500_e124004_d_n18, assign82500_e124004_d_n19, assign82500_e124004_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82500_e123990: f64 = (locals.var_gp2 * 0.5);
        let assign82500_e123991: f64 = (locals.var_temp__blk1038 + assign82500_e123990);
        let assign82500_e123996: f64 = (locals.var_gp2 * 0.25);
        let assign82500_e123997: f64 = (locals.var_temp__blk1038 + assign82500_e123996);
        let assign82500_e123999: f64 = (assign82500_e123997 - locals.var_nqs_w);
        let assign82500_e124000: f64 = (assign82500_e123999).sqrt();
        let assign82500_e124001: f64 = (locals.var_gp * assign82500_e124000);
        let assign82500_e124002: f64 = (assign82500_e123991 - assign82500_e124001);
        (assign82500_e124002, ((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.5)) - ((locals.var_gp_dn5 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn5 + (locals.var_gp2_dn5 * 0.25)) - locals.var_nqs_w_dn5) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.5)) - ((locals.var_gp_dn6 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn6 + (locals.var_gp2_dn6 * 0.25)) - locals.var_nqs_w_dn6) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.5)) - ((locals.var_gp_dn7 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn7 + (locals.var_gp2_dn7 * 0.25)) - locals.var_nqs_w_dn7) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.5)) - ((locals.var_gp_dn8 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn8 + (locals.var_gp2_dn8 * 0.25)) - locals.var_nqs_w_dn8) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.5)) - ((locals.var_gp_dn12 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn12 + (locals.var_gp2_dn12 * 0.25)) - locals.var_nqs_w_dn12) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.5)) - ((locals.var_gp_dn13 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn13 + (locals.var_gp2_dn13 * 0.25)) - locals.var_nqs_w_dn13) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.5)) - ((locals.var_gp_dn14 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn14 + (locals.var_gp2_dn14 * 0.25)) - locals.var_nqs_w_dn14) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.5)) - ((locals.var_gp_dn15 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn15 + (locals.var_gp2_dn15 * 0.25)) - locals.var_nqs_w_dn15) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.5)) - ((locals.var_gp_dn16 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn16 + (locals.var_gp2_dn16 * 0.25)) - locals.var_nqs_w_dn16) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.5)) - ((locals.var_gp_dn17 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn17 + (locals.var_gp2_dn17 * 0.25)) - locals.var_nqs_w_dn17) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.5)) - ((locals.var_gp_dn18 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn18 + (locals.var_gp2_dn18 * 0.25)) - locals.var_nqs_w_dn18) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.5)) - ((locals.var_gp_dn19 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn19 + (locals.var_gp2_dn19 * 0.25)) - locals.var_nqs_w_dn19) / (2.0 * assign82500_e124000))))), ((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.5)) - ((locals.var_gp_dn20 * assign82500_e124000) + (locals.var_gp * (((locals.var_temp__blk1038_dn20 + (locals.var_gp2_dn20 * 0.25)) - locals.var_nqs_w_dn20) / (2.0 * assign82500_e124000))))),)
    } else {
        (locals.var_nqs_x0, locals.var_nqs_x0_dn5, locals.var_nqs_x0_dn6, locals.var_nqs_x0_dn7, locals.var_nqs_x0_dn8, locals.var_nqs_x0_dn12, locals.var_nqs_x0_dn13, locals.var_nqs_x0_dn14, locals.var_nqs_x0_dn15, locals.var_nqs_x0_dn16, locals.var_nqs_x0_dn17, locals.var_nqs_x0_dn18, locals.var_nqs_x0_dn19, locals.var_nqs_x0_dn20,)
    }
};
        locals.var_nqs_x0 = assign82500_e124004;
        locals.var_nqs_x0_dn5 = assign82500_e124004_d_n5;
        locals.var_nqs_x0_dn6 = assign82500_e124004_d_n6;
        locals.var_nqs_x0_dn7 = assign82500_e124004_d_n7;
        locals.var_nqs_x0_dn8 = assign82500_e124004_d_n8;
        locals.var_nqs_x0_dn12 = assign82500_e124004_d_n12;
        locals.var_nqs_x0_dn13 = assign82500_e124004_d_n13;
        locals.var_nqs_x0_dn14 = assign82500_e124004_d_n14;
        locals.var_nqs_x0_dn15 = assign82500_e124004_d_n15;
        locals.var_nqs_x0_dn16 = assign82500_e124004_d_n16;
        locals.var_nqs_x0_dn17 = assign82500_e124004_d_n17;
        locals.var_nqs_x0_dn18 = assign82500_e124004_d_n18;
        locals.var_nqs_x0_dn19 = assign82500_e124004_d_n19;
        locals.var_nqs_x0_dn20 = assign82500_e124004_d_n20;
        locals.var_nqs_x0_rv = 0.0;

        let assign82510_e124006: f64 = (-locals.var_nqs_x0);
        let assign82510_e124007: f64 = (assign82510_e124006).abs();
        let assign82510_e124009: f64 = if assign82510_e124007 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign82510_e124009;
        locals.var_guard2242_rv = 0.0;

        let (assign82520_e124037, assign82520_e124037_d_n5, assign82520_e124037_d_n6, assign82520_e124037_d_n7, assign82520_e124037_d_n8, assign82520_e124037_d_n12, assign82520_e124037_d_n13, assign82520_e124037_d_n14, assign82520_e124037_d_n15, assign82520_e124037_d_n16, assign82520_e124037_d_n17, assign82520_e124037_d_n18, assign82520_e124037_d_n19, assign82520_e124037_d_n20,) = {
    if (((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 != 0.0)) {
        let assign82520_e124034: f64 = (-locals.var_nqs_x0);
        let assign82520_e124035: f64 = (assign82520_e124034).exp();
        (assign82520_e124035, (assign82520_e124035 * (-locals.var_nqs_x0_dn5)), (assign82520_e124035 * (-locals.var_nqs_x0_dn6)), (assign82520_e124035 * (-locals.var_nqs_x0_dn7)), (assign82520_e124035 * (-locals.var_nqs_x0_dn8)), (assign82520_e124035 * (-locals.var_nqs_x0_dn12)), (assign82520_e124035 * (-locals.var_nqs_x0_dn13)), (assign82520_e124035 * (-locals.var_nqs_x0_dn14)), (assign82520_e124035 * (-locals.var_nqs_x0_dn15)), (assign82520_e124035 * (-locals.var_nqs_x0_dn16)), (assign82520_e124035 * (-locals.var_nqs_x0_dn17)), (assign82520_e124035 * (-locals.var_nqs_x0_dn18)), (assign82520_e124035 * (-locals.var_nqs_x0_dn19)), (assign82520_e124035 * (-locals.var_nqs_x0_dn20)),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82520_e124037;
        locals.var_nqs_d0_dn5 = assign82520_e124037_d_n5;
        locals.var_nqs_d0_dn6 = assign82520_e124037_d_n6;
        locals.var_nqs_d0_dn7 = assign82520_e124037_d_n7;
        locals.var_nqs_d0_dn8 = assign82520_e124037_d_n8;
        locals.var_nqs_d0_dn12 = assign82520_e124037_d_n12;
        locals.var_nqs_d0_dn13 = assign82520_e124037_d_n13;
        locals.var_nqs_d0_dn14 = assign82520_e124037_d_n14;
        locals.var_nqs_d0_dn15 = assign82520_e124037_d_n15;
        locals.var_nqs_d0_dn16 = assign82520_e124037_d_n16;
        locals.var_nqs_d0_dn17 = assign82520_e124037_d_n17;
        locals.var_nqs_d0_dn18 = assign82520_e124037_d_n18;
        locals.var_nqs_d0_dn19 = assign82520_e124037_d_n19;
        locals.var_nqs_d0_dn20 = assign82520_e124037_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let assign82530_e124039: f64 = (-locals.var_nqs_x0);
        let assign82530_e124041: f64 = if assign82530_e124039 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign82530_e124041;
        locals.var_guard2243_rv = 0.0;

        let (assign82540_e124098, assign82540_e124098_d_n5, assign82540_e124098_d_n6, assign82540_e124098_d_n7, assign82540_e124098_d_n8, assign82540_e124098_d_n12, assign82540_e124098_d_n13, assign82540_e124098_d_n14, assign82540_e124098_d_n15, assign82540_e124098_d_n16, assign82540_e124098_d_n17, assign82540_e124098_d_n18, assign82540_e124098_d_n19, assign82540_e124098_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 == 0.0)) && (locals.var_guard2243 != 0.0)) {
        let assign82540_e124071: f64 = (-230.25850929940458);
        let assign82540_e124073: f64 = (-locals.var_nqs_x0);
        let assign82540_e124074: f64 = (assign82540_e124071 - assign82540_e124073);
        let assign82540_e124078: f64 = (-230.25850929940458);
        let assign82540_e124080: f64 = (-locals.var_nqs_x0);
        let assign82540_e124081: f64 = (assign82540_e124078 - assign82540_e124080);
        let assign82540_e124084: f64 = (-230.25850929940458);
        let assign82540_e124086: f64 = (-locals.var_nqs_x0);
        let assign82540_e124087: f64 = (assign82540_e124084 - assign82540_e124086);
        let assign82540_e124089: f64 = (assign82540_e124087 * 0.3333333333333333);
        let assign82540_e124090: f64 = (1.0 + assign82540_e124089);
        let assign82540_e124091: f64 = (assign82540_e124081 * assign82540_e124090);
        let assign82540_e124092: f64 = (0.5 * assign82540_e124091);
        let assign82540_e124093: f64 = (1.0 + assign82540_e124092);
        let assign82540_e124094: f64 = (assign82540_e124074 * assign82540_e124093);
        let assign82540_e124095: f64 = (1.0 + assign82540_e124094);
        let assign82540_e124096: f64 = (1e-100 / assign82540_e124095);
        (assign82540_e124096, (-((1e-100 * (((-(-locals.var_nqs_x0_dn5)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn5)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn5)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn6)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn6)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn6)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn7)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn7)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn7)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn8)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn8)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn8)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn12)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn12)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn12)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn13)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn13)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn13)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn14)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn14)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn14)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn15)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn15)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn15)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn16)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn16)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn16)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn17)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn17)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn17)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn18)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn18)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn18)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn19)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn19)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn19)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-locals.var_nqs_x0_dn20)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-locals.var_nqs_x0_dn20)) * assign82540_e124090) + (assign82540_e124081 * ((-(-locals.var_nqs_x0_dn20)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82540_e124098;
        locals.var_nqs_d0_dn5 = assign82540_e124098_d_n5;
        locals.var_nqs_d0_dn6 = assign82540_e124098_d_n6;
        locals.var_nqs_d0_dn7 = assign82540_e124098_d_n7;
        locals.var_nqs_d0_dn8 = assign82540_e124098_d_n8;
        locals.var_nqs_d0_dn12 = assign82540_e124098_d_n12;
        locals.var_nqs_d0_dn13 = assign82540_e124098_d_n13;
        locals.var_nqs_d0_dn14 = assign82540_e124098_d_n14;
        locals.var_nqs_d0_dn15 = assign82540_e124098_d_n15;
        locals.var_nqs_d0_dn16 = assign82540_e124098_d_n16;
        locals.var_nqs_d0_dn17 = assign82540_e124098_d_n17;
        locals.var_nqs_d0_dn18 = assign82540_e124098_d_n18;
        locals.var_nqs_d0_dn19 = assign82540_e124098_d_n19;
        locals.var_nqs_d0_dn20 = assign82540_e124098_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign82550_e124153, assign82550_e124153_d_n5, assign82550_e124153_d_n6, assign82550_e124153_d_n7, assign82550_e124153_d_n8, assign82550_e124153_d_n12, assign82550_e124153_d_n13, assign82550_e124153_d_n14, assign82550_e124153_d_n15, assign82550_e124153_d_n16, assign82550_e124153_d_n17, assign82550_e124153_d_n18, assign82550_e124153_d_n19, assign82550_e124153_d_n20,) = {
    if ((((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) && (locals.var_guard2242 == 0.0)) && (locals.var_guard2243 == 0.0)) {
        let assign82550_e124129: f64 = (-locals.var_nqs_x0);
        let assign82550_e124131: f64 = (assign82550_e124129 - 230.25850929940458);
        let assign82550_e124135: f64 = (-locals.var_nqs_x0);
        let assign82550_e124137: f64 = (assign82550_e124135 - 230.25850929940458);
        let assign82550_e124140: f64 = (-locals.var_nqs_x0);
        let assign82550_e124142: f64 = (assign82550_e124140 - 230.25850929940458);
        let assign82550_e124144: f64 = (assign82550_e124142 * 0.3333333333333333);
        let assign82550_e124145: f64 = (1.0 + assign82550_e124144);
        let assign82550_e124146: f64 = (assign82550_e124137 * assign82550_e124145);
        let assign82550_e124147: f64 = (0.5 * assign82550_e124146);
        let assign82550_e124148: f64 = (1.0 + assign82550_e124147);
        let assign82550_e124149: f64 = (assign82550_e124131 * assign82550_e124148);
        let assign82550_e124150: f64 = (1.0 + assign82550_e124149);
        let assign82550_e124151: f64 = (1e100 * assign82550_e124150);
        (assign82550_e124151, (1e100 * (((-locals.var_nqs_x0_dn5) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn5) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn6) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn6) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn7) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn7) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn8) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn8) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn12) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn12) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn12) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn13) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn13) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn13) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn14) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn14) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn14) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn15) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn15) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn15) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn16) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn16) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn16) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn17) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn17) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn17) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn18) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn18) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn18) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn19) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn19) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn19) * 0.3333333333333333))))))), (1e100 * (((-locals.var_nqs_x0_dn20) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-locals.var_nqs_x0_dn20) * assign82550_e124145) + (assign82550_e124137 * ((-locals.var_nqs_x0_dn20) * 0.3333333333333333))))))),)
    } else {
        (locals.var_nqs_d0, locals.var_nqs_d0_dn5, locals.var_nqs_d0_dn6, locals.var_nqs_d0_dn7, locals.var_nqs_d0_dn8, locals.var_nqs_d0_dn12, locals.var_nqs_d0_dn13, locals.var_nqs_d0_dn14, locals.var_nqs_d0_dn15, locals.var_nqs_d0_dn16, locals.var_nqs_d0_dn17, locals.var_nqs_d0_dn18, locals.var_nqs_d0_dn19, locals.var_nqs_d0_dn20,)
    }
};
        locals.var_nqs_d0 = assign82550_e124153;
        locals.var_nqs_d0_dn5 = assign82550_e124153_d_n5;
        locals.var_nqs_d0_dn6 = assign82550_e124153_d_n6;
        locals.var_nqs_d0_dn7 = assign82550_e124153_d_n7;
        locals.var_nqs_d0_dn8 = assign82550_e124153_d_n8;
        locals.var_nqs_d0_dn12 = assign82550_e124153_d_n12;
        locals.var_nqs_d0_dn13 = assign82550_e124153_d_n13;
        locals.var_nqs_d0_dn14 = assign82550_e124153_d_n14;
        locals.var_nqs_d0_dn15 = assign82550_e124153_d_n15;
        locals.var_nqs_d0_dn16 = assign82550_e124153_d_n16;
        locals.var_nqs_d0_dn17 = assign82550_e124153_d_n17;
        locals.var_nqs_d0_dn18 = assign82550_e124153_d_n18;
        locals.var_nqs_d0_dn19 = assign82550_e124153_d_n19;
        locals.var_nqs_d0_dn20 = assign82550_e124153_d_n20;
        locals.var_nqs_d0_rv = 0.0;

        let (assign82560_e124183, assign82560_e124183_d_n5, assign82560_e124183_d_n6, assign82560_e124183_d_n7, assign82560_e124183_d_n8, assign82560_e124183_d_n12, assign82560_e124183_d_n13, assign82560_e124183_d_n14, assign82560_e124183_d_n15, assign82560_e124183_d_n16, assign82560_e124183_d_n17, assign82560_e124183_d_n18, assign82560_e124183_d_n19, assign82560_e124183_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82560_e124178: f64 = (locals.var_gp2 * 0.5);
        let assign82560_e124180: f64 = (assign82560_e124178 * locals.var_nqs_d0);
        let assign82560_e124181: f64 = (1.0 - assign82560_e124180);
        (assign82560_e124181, (-(((locals.var_gp2_dn5 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn5))), (-(((locals.var_gp2_dn6 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn6))), (-(((locals.var_gp2_dn7 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn7))), (-(((locals.var_gp2_dn8 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn8))), (-(((locals.var_gp2_dn12 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn12))), (-(((locals.var_gp2_dn13 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn13))), (-(((locals.var_gp2_dn14 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn14))), (-(((locals.var_gp2_dn15 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn15))), (-(((locals.var_gp2_dn16 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn16))), (-(((locals.var_gp2_dn17 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn17))), (-(((locals.var_gp2_dn18 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn18))), (-(((locals.var_gp2_dn19 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn19))), (-(((locals.var_gp2_dn20 * 0.5) * locals.var_nqs_d0) + (assign82560_e124178 * locals.var_nqs_d0_dn20))),)
    } else {
        (locals.var_nqs_xi, locals.var_nqs_xi_dn5, locals.var_nqs_xi_dn6, locals.var_nqs_xi_dn7, locals.var_nqs_xi_dn8, locals.var_nqs_xi_dn12, locals.var_nqs_xi_dn13, locals.var_nqs_xi_dn14, locals.var_nqs_xi_dn15, locals.var_nqs_xi_dn16, locals.var_nqs_xi_dn17, locals.var_nqs_xi_dn18, locals.var_nqs_xi_dn19, locals.var_nqs_xi_dn20,)
    }
};
        locals.var_nqs_xi = assign82560_e124183;
        locals.var_nqs_xi_dn5 = assign82560_e124183_d_n5;
        locals.var_nqs_xi_dn6 = assign82560_e124183_d_n6;
        locals.var_nqs_xi_dn7 = assign82560_e124183_d_n7;
        locals.var_nqs_xi_dn8 = assign82560_e124183_d_n8;
        locals.var_nqs_xi_dn12 = assign82560_e124183_d_n12;
        locals.var_nqs_xi_dn13 = assign82560_e124183_d_n13;
        locals.var_nqs_xi_dn14 = assign82560_e124183_d_n14;
        locals.var_nqs_xi_dn15 = assign82560_e124183_d_n15;
        locals.var_nqs_xi_dn16 = assign82560_e124183_d_n16;
        locals.var_nqs_xi_dn17 = assign82560_e124183_d_n17;
        locals.var_nqs_xi_dn18 = assign82560_e124183_d_n18;
        locals.var_nqs_xi_dn19 = assign82560_e124183_d_n19;
        locals.var_nqs_xi_dn20 = assign82560_e124183_d_n20;
        locals.var_nqs_xi_rv = 0.0;

        let (assign82570_e124217, assign82570_e124217_d_n5, assign82570_e124217_d_n6, assign82570_e124217_d_n7, assign82570_e124217_d_n8, assign82570_e124217_d_n12, assign82570_e124217_d_n13, assign82570_e124217_d_n14, assign82570_e124217_d_n15, assign82570_e124217_d_n16, assign82570_e124217_d_n17, assign82570_e124217_d_n18, assign82570_e124217_d_n19, assign82570_e124217_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82570_e124208: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82570_e124209: f64 = (2.0 * assign82570_e124208);
        let assign82570_e124213: f64 = (1.0 - locals.var_nqs_d0);
        let assign82570_e124214: f64 = (locals.var_gp2 * assign82570_e124213);
        let assign82570_e124215: f64 = (assign82570_e124209 + assign82570_e124214);
        (assign82570_e124215, ((2.0 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5)) + ((locals.var_gp2_dn5 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn5)))), ((2.0 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6)) + ((locals.var_gp2_dn6 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn6)))), ((2.0 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7)) + ((locals.var_gp2_dn7 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn7)))), ((2.0 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8)) + ((locals.var_gp2_dn8 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn8)))), ((2.0 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12)) + ((locals.var_gp2_dn12 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn12)))), ((2.0 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13)) + ((locals.var_gp2_dn13 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn13)))), ((2.0 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14)) + ((locals.var_gp2_dn14 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn14)))), ((2.0 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15)) + ((locals.var_gp2_dn15 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn15)))), ((2.0 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16)) + ((locals.var_gp2_dn16 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn16)))), ((2.0 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17)) + ((locals.var_gp2_dn17 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn17)))), ((2.0 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18)) + ((locals.var_gp2_dn18 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn18)))), ((2.0 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19)) + ((locals.var_gp2_dn19 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn19)))), ((2.0 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20)) + ((locals.var_gp2_dn20 * assign82570_e124213) + (locals.var_gp2 * (-locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_p, locals.var_nqs_p_dn5, locals.var_nqs_p_dn6, locals.var_nqs_p_dn7, locals.var_nqs_p_dn8, locals.var_nqs_p_dn12, locals.var_nqs_p_dn13, locals.var_nqs_p_dn14, locals.var_nqs_p_dn15, locals.var_nqs_p_dn16, locals.var_nqs_p_dn17, locals.var_nqs_p_dn18, locals.var_nqs_p_dn19, locals.var_nqs_p_dn20,)
    }
};
        locals.var_nqs_p = assign82570_e124217;
        locals.var_nqs_p_dn5 = assign82570_e124217_d_n5;
        locals.var_nqs_p_dn6 = assign82570_e124217_d_n6;
        locals.var_nqs_p_dn7 = assign82570_e124217_d_n7;
        locals.var_nqs_p_dn8 = assign82570_e124217_d_n8;
        locals.var_nqs_p_dn12 = assign82570_e124217_d_n12;
        locals.var_nqs_p_dn13 = assign82570_e124217_d_n13;
        locals.var_nqs_p_dn14 = assign82570_e124217_d_n14;
        locals.var_nqs_p_dn15 = assign82570_e124217_d_n15;
        locals.var_nqs_p_dn16 = assign82570_e124217_d_n16;
        locals.var_nqs_p_dn17 = assign82570_e124217_d_n17;
        locals.var_nqs_p_dn18 = assign82570_e124217_d_n18;
        locals.var_nqs_p_dn19 = assign82570_e124217_d_n19;
        locals.var_nqs_p_dn20 = assign82570_e124217_d_n20;
        locals.var_nqs_p_rv = 0.0;

        let (assign82580_e124255, assign82580_e124255_d_n5, assign82580_e124255_d_n6, assign82580_e124255_d_n7, assign82580_e124255_d_n8, assign82580_e124255_d_n12, assign82580_e124255_d_n13, assign82580_e124255_d_n14, assign82580_e124255_d_n15, assign82580_e124255_d_n16, assign82580_e124255_d_n17, assign82580_e124255_d_n18, assign82580_e124255_d_n19, assign82580_e124255_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82580_e124241: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82580_e124244: f64 = (locals.var_temp__blk1038 - locals.var_nqs_x0);
        let assign82580_e124245: f64 = (assign82580_e124241 * assign82580_e124244);
        let assign82580_e124249: f64 = (locals.var_nqs_x0 - 1.0);
        let assign82580_e124251: f64 = (assign82580_e124249 + locals.var_nqs_d0);
        let assign82580_e124252: f64 = (locals.var_gp2 * assign82580_e124251);
        let assign82580_e124253: f64 = (assign82580_e124245 - assign82580_e124252);
        (assign82580_e124253, ((((locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn5 - locals.var_nqs_x0_dn5))) - ((locals.var_gp2_dn5 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn5 + locals.var_nqs_d0_dn5)))), ((((locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn6 - locals.var_nqs_x0_dn6))) - ((locals.var_gp2_dn6 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn6 + locals.var_nqs_d0_dn6)))), ((((locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn7 - locals.var_nqs_x0_dn7))) - ((locals.var_gp2_dn7 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn7 + locals.var_nqs_d0_dn7)))), ((((locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn8 - locals.var_nqs_x0_dn8))) - ((locals.var_gp2_dn8 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn8 + locals.var_nqs_d0_dn8)))), ((((locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn12 - locals.var_nqs_x0_dn12))) - ((locals.var_gp2_dn12 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn12 + locals.var_nqs_d0_dn12)))), ((((locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn13 - locals.var_nqs_x0_dn13))) - ((locals.var_gp2_dn13 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn13 + locals.var_nqs_d0_dn13)))), ((((locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn14 - locals.var_nqs_x0_dn14))) - ((locals.var_gp2_dn14 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn14 + locals.var_nqs_d0_dn14)))), ((((locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn15 - locals.var_nqs_x0_dn15))) - ((locals.var_gp2_dn15 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn15 + locals.var_nqs_d0_dn15)))), ((((locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn16 - locals.var_nqs_x0_dn16))) - ((locals.var_gp2_dn16 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn16 + locals.var_nqs_d0_dn16)))), ((((locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn17 - locals.var_nqs_x0_dn17))) - ((locals.var_gp2_dn17 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn17 + locals.var_nqs_d0_dn17)))), ((((locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn18 - locals.var_nqs_x0_dn18))) - ((locals.var_gp2_dn18 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn18 + locals.var_nqs_d0_dn18)))), ((((locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn19 - locals.var_nqs_x0_dn19))) - ((locals.var_gp2_dn19 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn19 + locals.var_nqs_d0_dn19)))), ((((locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20) * assign82580_e124244) + (assign82580_e124241 * (locals.var_temp__blk1038_dn20 - locals.var_nqs_x0_dn20))) - ((locals.var_gp2_dn20 * assign82580_e124251) + (locals.var_gp2 * (locals.var_nqs_x0_dn20 + locals.var_nqs_d0_dn20)))),)
    } else {
        (locals.var_nqs_q, locals.var_nqs_q_dn5, locals.var_nqs_q_dn6, locals.var_nqs_q_dn7, locals.var_nqs_q_dn8, locals.var_nqs_q_dn12, locals.var_nqs_q_dn13, locals.var_nqs_q_dn14, locals.var_nqs_q_dn15, locals.var_nqs_q_dn16, locals.var_nqs_q_dn17, locals.var_nqs_q_dn18, locals.var_nqs_q_dn19, locals.var_nqs_q_dn20,)
    }
};
        locals.var_nqs_q = assign82580_e124255;
        locals.var_nqs_q_dn5 = assign82580_e124255_d_n5;
        locals.var_nqs_q_dn6 = assign82580_e124255_d_n6;
        locals.var_nqs_q_dn7 = assign82580_e124255_d_n7;
        locals.var_nqs_q_dn8 = assign82580_e124255_d_n8;
        locals.var_nqs_q_dn12 = assign82580_e124255_d_n12;
        locals.var_nqs_q_dn13 = assign82580_e124255_d_n13;
        locals.var_nqs_q_dn14 = assign82580_e124255_d_n14;
        locals.var_nqs_q_dn15 = assign82580_e124255_d_n15;
        locals.var_nqs_q_dn16 = assign82580_e124255_d_n16;
        locals.var_nqs_q_dn17 = assign82580_e124255_d_n17;
        locals.var_nqs_q_dn18 = assign82580_e124255_d_n18;
        locals.var_nqs_q_dn19 = assign82580_e124255_d_n19;
        locals.var_nqs_q_dn20 = assign82580_e124255_d_n20;
        locals.var_nqs_q_rv = 0.0;

        let (assign82590_e124287, assign82590_e124287_d_n5, assign82590_e124287_d_n6, assign82590_e124287_d_n7, assign82590_e124287_d_n8, assign82590_e124287_d_n12, assign82590_e124287_d_n13, assign82590_e124287_d_n14, assign82590_e124287_d_n15, assign82590_e124287_d_n16, assign82590_e124287_d_n17, assign82590_e124287_d_n18, assign82590_e124287_d_n19, assign82590_e124287_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82590_e124279: f64 = (locals.var_nqs_p * locals.var_nqs_p);
        let assign82590_e124282: f64 = (4.0 * locals.var_nqs_xi);
        let assign82590_e124284: f64 = (assign82590_e124282 * locals.var_nqs_q);
        let assign82590_e124285: f64 = (assign82590_e124279 - assign82590_e124284);
        (assign82590_e124285, (((locals.var_nqs_p_dn5 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn5)) - (((4.0 * locals.var_nqs_xi_dn5) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn5))), (((locals.var_nqs_p_dn6 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn6)) - (((4.0 * locals.var_nqs_xi_dn6) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn6))), (((locals.var_nqs_p_dn7 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn7)) - (((4.0 * locals.var_nqs_xi_dn7) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn7))), (((locals.var_nqs_p_dn8 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn8)) - (((4.0 * locals.var_nqs_xi_dn8) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn8))), (((locals.var_nqs_p_dn12 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn12)) - (((4.0 * locals.var_nqs_xi_dn12) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn12))), (((locals.var_nqs_p_dn13 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn13)) - (((4.0 * locals.var_nqs_xi_dn13) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn13))), (((locals.var_nqs_p_dn14 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn14)) - (((4.0 * locals.var_nqs_xi_dn14) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn14))), (((locals.var_nqs_p_dn15 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn15)) - (((4.0 * locals.var_nqs_xi_dn15) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn15))), (((locals.var_nqs_p_dn16 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn16)) - (((4.0 * locals.var_nqs_xi_dn16) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn16))), (((locals.var_nqs_p_dn17 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn17)) - (((4.0 * locals.var_nqs_xi_dn17) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn17))), (((locals.var_nqs_p_dn18 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn18)) - (((4.0 * locals.var_nqs_xi_dn18) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn18))), (((locals.var_nqs_p_dn19 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn19)) - (((4.0 * locals.var_nqs_xi_dn19) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn19))), (((locals.var_nqs_p_dn20 * locals.var_nqs_p) + (locals.var_nqs_p * locals.var_nqs_p_dn20)) - (((4.0 * locals.var_nqs_xi_dn20) * locals.var_nqs_q) + (assign82590_e124282 * locals.var_nqs_q_dn20))),)
    } else {
        (locals.var_nqs_temp, locals.var_nqs_temp_dn5, locals.var_nqs_temp_dn6, locals.var_nqs_temp_dn7, locals.var_nqs_temp_dn8, locals.var_nqs_temp_dn12, locals.var_nqs_temp_dn13, locals.var_nqs_temp_dn14, locals.var_nqs_temp_dn15, locals.var_nqs_temp_dn16, locals.var_nqs_temp_dn17, locals.var_nqs_temp_dn18, locals.var_nqs_temp_dn19, locals.var_nqs_temp_dn20,)
    }
};
        locals.var_nqs_temp = assign82590_e124287;
        locals.var_nqs_temp_dn5 = assign82590_e124287_d_n5;
        locals.var_nqs_temp_dn6 = assign82590_e124287_d_n6;
        locals.var_nqs_temp_dn7 = assign82590_e124287_d_n7;
        locals.var_nqs_temp_dn8 = assign82590_e124287_d_n8;
        locals.var_nqs_temp_dn12 = assign82590_e124287_d_n12;
        locals.var_nqs_temp_dn13 = assign82590_e124287_d_n13;
        locals.var_nqs_temp_dn14 = assign82590_e124287_d_n14;
        locals.var_nqs_temp_dn15 = assign82590_e124287_d_n15;
        locals.var_nqs_temp_dn16 = assign82590_e124287_d_n16;
        locals.var_nqs_temp_dn17 = assign82590_e124287_d_n17;
        locals.var_nqs_temp_dn18 = assign82590_e124287_d_n18;
        locals.var_nqs_temp_dn19 = assign82590_e124287_d_n19;
        locals.var_nqs_temp_dn20 = assign82590_e124287_d_n20;
        locals.var_nqs_temp_rv = 0.0;

        let (assign82600_e124318, assign82600_e124318_d_n5, assign82600_e124318_d_n6, assign82600_e124318_d_n7, assign82600_e124318_d_n8, assign82600_e124318_d_n12, assign82600_e124318_d_n13, assign82600_e124318_d_n14, assign82600_e124318_d_n15, assign82600_e124318_d_n16, assign82600_e124318_d_n17, assign82600_e124318_d_n18, assign82600_e124318_d_n19, assign82600_e124318_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82600_e124311: f64 = (2.0 * locals.var_nqs_q);
        let assign82600_e124314: f64 = (locals.var_nqs_temp).sqrt();
        let assign82600_e124315: f64 = (locals.var_nqs_p + assign82600_e124314);
        let assign82600_e124316: f64 = (assign82600_e124311 / assign82600_e124315);
        (assign82600_e124316, ((((2.0 * locals.var_nqs_q_dn5) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn5 + (locals.var_nqs_temp_dn5 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn6) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn6 + (locals.var_nqs_temp_dn6 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn7) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn7 + (locals.var_nqs_temp_dn7 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn8) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn8 + (locals.var_nqs_temp_dn8 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn12) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn12 + (locals.var_nqs_temp_dn12 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn13) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn13 + (locals.var_nqs_temp_dn13 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn14) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn14 + (locals.var_nqs_temp_dn14 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn15) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn15 + (locals.var_nqs_temp_dn15 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn16) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn16 + (locals.var_nqs_temp_dn16 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn17) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn17 + (locals.var_nqs_temp_dn17 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn18) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn18 + (locals.var_nqs_temp_dn18 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn19) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn19 + (locals.var_nqs_temp_dn19 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * locals.var_nqs_q_dn20) * assign82600_e124315) - (assign82600_e124311 * (locals.var_nqs_p_dn20 + (locals.var_nqs_temp_dn20 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)),)
    } else {
        (locals.var_nqs_u, locals.var_nqs_u_dn5, locals.var_nqs_u_dn6, locals.var_nqs_u_dn7, locals.var_nqs_u_dn8, locals.var_nqs_u_dn12, locals.var_nqs_u_dn13, locals.var_nqs_u_dn14, locals.var_nqs_u_dn15, locals.var_nqs_u_dn16, locals.var_nqs_u_dn17, locals.var_nqs_u_dn18, locals.var_nqs_u_dn19, locals.var_nqs_u_dn20,)
    }
};
        locals.var_nqs_u = assign82600_e124318;
        locals.var_nqs_u_dn5 = assign82600_e124318_d_n5;
        locals.var_nqs_u_dn6 = assign82600_e124318_d_n6;
        locals.var_nqs_u_dn7 = assign82600_e124318_d_n7;
        locals.var_nqs_u_dn8 = assign82600_e124318_d_n8;
        locals.var_nqs_u_dn12 = assign82600_e124318_d_n12;
        locals.var_nqs_u_dn13 = assign82600_e124318_d_n13;
        locals.var_nqs_u_dn14 = assign82600_e124318_d_n14;
        locals.var_nqs_u_dn15 = assign82600_e124318_d_n15;
        locals.var_nqs_u_dn16 = assign82600_e124318_d_n16;
        locals.var_nqs_u_dn17 = assign82600_e124318_d_n17;
        locals.var_nqs_u_dn18 = assign82600_e124318_d_n18;
        locals.var_nqs_u_dn19 = assign82600_e124318_d_n19;
        locals.var_nqs_u_dn20 = assign82600_e124318_d_n20;
        locals.var_nqs_u_rv = 0.0;

        let (assign82610_e124344, assign82610_e124344_d_n5, assign82610_e124344_d_n6, assign82610_e124344_d_n7, assign82610_e124344_d_n8, assign82610_e124344_d_n12, assign82610_e124344_d_n13, assign82610_e124344_d_n14, assign82610_e124344_d_n15, assign82610_e124344_d_n16, assign82610_e124344_d_n17, assign82610_e124344_d_n18, assign82610_e124344_d_n19, assign82610_e124344_d_n20,) = {
    if ((((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) && (locals.var_guard2236 == 0.0)) && (locals.var_guard2237 == 0.0)) {
        let assign82610_e124342: f64 = (locals.var_nqs_x0 + locals.var_nqs_u);
        (assign82610_e124342, (locals.var_nqs_x0_dn5 + locals.var_nqs_u_dn5), (locals.var_nqs_x0_dn6 + locals.var_nqs_u_dn6), (locals.var_nqs_x0_dn7 + locals.var_nqs_u_dn7), (locals.var_nqs_x0_dn8 + locals.var_nqs_u_dn8), (locals.var_nqs_x0_dn12 + locals.var_nqs_u_dn12), (locals.var_nqs_x0_dn13 + locals.var_nqs_u_dn13), (locals.var_nqs_x0_dn14 + locals.var_nqs_u_dn14), (locals.var_nqs_x0_dn15 + locals.var_nqs_u_dn15), (locals.var_nqs_x0_dn16 + locals.var_nqs_u_dn16), (locals.var_nqs_x0_dn17 + locals.var_nqs_u_dn17), (locals.var_nqs_x0_dn18 + locals.var_nqs_u_dn18), (locals.var_nqs_x0_dn19 + locals.var_nqs_u_dn19), (locals.var_nqs_x0_dn20 + locals.var_nqs_u_dn20),)
    } else {
        (locals.var_temp9, locals.var_temp9_dn5, locals.var_temp9_dn6, locals.var_temp9_dn7, locals.var_temp9_dn8, locals.var_temp9_dn12, locals.var_temp9_dn13, locals.var_temp9_dn14, locals.var_temp9_dn15, locals.var_temp9_dn16, locals.var_temp9_dn17, locals.var_temp9_dn18, locals.var_temp9_dn19, locals.var_temp9_dn20,)
    }
};
        locals.var_temp9 = assign82610_e124344;
        locals.var_temp9_dn5 = assign82610_e124344_d_n5;
        locals.var_temp9_dn6 = assign82610_e124344_d_n6;
        locals.var_temp9_dn7 = assign82610_e124344_d_n7;
        locals.var_temp9_dn8 = assign82610_e124344_d_n8;
        locals.var_temp9_dn12 = assign82610_e124344_d_n12;
        locals.var_temp9_dn13 = assign82610_e124344_d_n13;
        locals.var_temp9_dn14 = assign82610_e124344_d_n14;
        locals.var_temp9_dn15 = assign82610_e124344_d_n15;
        locals.var_temp9_dn16 = assign82610_e124344_d_n16;
        locals.var_temp9_dn17 = assign82610_e124344_d_n17;
        locals.var_temp9_dn18 = assign82610_e124344_d_n18;
        locals.var_temp9_dn19 = assign82610_e124344_d_n19;
        locals.var_temp9_dn20 = assign82610_e124344_d_n20;
        locals.var_temp9_rv = 0.0;

        let (assign82620_e124390, assign82620_e124390_d_n5, assign82620_e124390_d_n6, assign82620_e124390_d_n7, assign82620_e124390_d_n8, assign82620_e124390_d_n12, assign82620_e124390_d_n13, assign82620_e124390_d_n14, assign82620_e124390_d_n15, assign82620_e124390_d_n16, assign82620_e124390_d_n17, assign82620_e124390_d_n18, assign82620_e124390_d_n19, assign82620_e124390_d_n20,) = {
    if ((((((locals.var_guard2078 != 0.0) && (locals.var_guard2079 == 0.0)) && (locals.var_guard2088 == 0.0)) && (locals.var_guard2105 == 0.0)) && (locals.var_guard2130 == 0.0)) && (locals.var_guard2171 != 0.0)) {
        let assign82620_e124365: f64 = (locals.var_temp1 + locals.var_temp3);
        let assign82620_e124367: f64 = (assign82620_e124365 + locals.var_temp5);
        let assign82620_e124369: f64 = (assign82620_e124367 + locals.var_temp7);
        let assign82620_e124371: f64 = (assign82620_e124369 + locals.var_temp9);
        let assign82620_e124372: f64 = (4.0 * assign82620_e124371);
        let assign82620_e124373: f64 = (locals.var_x_sp + assign82620_e124372);
        let assign82620_e124377: f64 = (locals.var_temp2 + locals.var_temp4);
        let assign82620_e124379: f64 = (assign82620_e124377 + locals.var_temp6);
        let assign82620_e124381: f64 = (assign82620_e124379 + locals.var_temp8);
        let assign82620_e124382: f64 = (2.0 * assign82620_e124381);
        let assign82620_e124383: f64 = (assign82620_e124373 + assign82620_e124382);
        let assign82620_e124385: f64 = (assign82620_e124383 + locals.var_x_dp);
        let assign82620_e124387: f64 = (assign82620_e124385 / 30.0);
        let assign82620_e124388: f64 = (locals.var_xg_ac - assign82620_e124387);
        (assign82620_e124388, (locals.var_xg_ac_dn5 - ((((locals.var_x_sp_dn5 + (4.0 * ((((locals.var_temp1_dn5 + locals.var_temp3_dn5) + locals.var_temp5_dn5) + locals.var_temp7_dn5) + locals.var_temp9_dn5))) + (2.0 * (((locals.var_temp2_dn5 + locals.var_temp4_dn5) + locals.var_temp6_dn5) + locals.var_temp8_dn5))) + locals.var_x_dp_dn5) / 30.0)), (locals.var_xg_ac_dn6 - ((((locals.var_x_sp_dn6 + (4.0 * ((((locals.var_temp1_dn6 + locals.var_temp3_dn6) + locals.var_temp5_dn6) + locals.var_temp7_dn6) + locals.var_temp9_dn6))) + (2.0 * (((locals.var_temp2_dn6 + locals.var_temp4_dn6) + locals.var_temp6_dn6) + locals.var_temp8_dn6))) + locals.var_x_dp_dn6) / 30.0)), (locals.var_xg_ac_dn7 - ((((locals.var_x_sp_dn7 + (4.0 * ((((locals.var_temp1_dn7 + locals.var_temp3_dn7) + locals.var_temp5_dn7) + locals.var_temp7_dn7) + locals.var_temp9_dn7))) + (2.0 * (((locals.var_temp2_dn7 + locals.var_temp4_dn7) + locals.var_temp6_dn7) + locals.var_temp8_dn7))) + locals.var_x_dp_dn7) / 30.0)), (locals.var_xg_ac_dn8 - ((((locals.var_x_sp_dn8 + (4.0 * ((((locals.var_temp1_dn8 + locals.var_temp3_dn8) + locals.var_temp5_dn8) + locals.var_temp7_dn8) + locals.var_temp9_dn8))) + (2.0 * (((locals.var_temp2_dn8 + locals.var_temp4_dn8) + locals.var_temp6_dn8) + locals.var_temp8_dn8))) + locals.var_x_dp_dn8) / 30.0)), (locals.var_xg_ac_dn12 - ((((locals.var_x_sp_dn12 + (4.0 * ((((locals.var_temp1_dn12 + locals.var_temp3_dn12) + locals.var_temp5_dn12) + locals.var_temp7_dn12) + locals.var_temp9_dn12))) + (2.0 * (((locals.var_temp2_dn12 + locals.var_temp4_dn12) + locals.var_temp6_dn12) + locals.var_temp8_dn12))) + locals.var_x_dp_dn12) / 30.0)), (locals.var_xg_ac_dn13 - ((((locals.var_x_sp_dn13 + (4.0 * ((((locals.var_temp1_dn13 + locals.var_temp3_dn13) + locals.var_temp5_dn13) + locals.var_temp7_dn13) + locals.var_temp9_dn13))) + (2.0 * (((locals.var_temp2_dn13 + locals.var_temp4_dn13) + locals.var_temp6_dn13) + locals.var_temp8_dn13))) + locals.var_x_dp_dn13) / 30.0)), (locals.var_xg_ac_dn14 - ((((locals.var_x_sp_dn14 + (4.0 * ((((locals.var_temp1_dn14 + locals.var_temp3_dn14) + locals.var_temp5_dn14) + locals.var_temp7_dn14) + locals.var_temp9_dn14))) + (2.0 * (((locals.var_temp2_dn14 + locals.var_temp4_dn14) + locals.var_temp6_dn14) + locals.var_temp8_dn14))) + locals.var_x_dp_dn14) / 30.0)), (locals.var_xg_ac_dn15 - ((((locals.var_x_sp_dn15 + (4.0 * ((((locals.var_temp1_dn15 + locals.var_temp3_dn15) + locals.var_temp5_dn15) + locals.var_temp7_dn15) + locals.var_temp9_dn15))) + (2.0 * (((locals.var_temp2_dn15 + locals.var_temp4_dn15) + locals.var_temp6_dn15) + locals.var_temp8_dn15))) + locals.var_x_dp_dn15) / 30.0)), (locals.var_xg_ac_dn16 - ((((locals.var_x_sp_dn16 + (4.0 * ((((locals.var_temp1_dn16 + locals.var_temp3_dn16) + locals.var_temp5_dn16) + locals.var_temp7_dn16) + locals.var_temp9_dn16))) + (2.0 * (((locals.var_temp2_dn16 + locals.var_temp4_dn16) + locals.var_temp6_dn16) + locals.var_temp8_dn16))) + locals.var_x_dp_dn16) / 30.0)), (locals.var_xg_ac_dn17 - ((((locals.var_x_sp_dn17 + (4.0 * ((((locals.var_temp1_dn17 + locals.var_temp3_dn17) + locals.var_temp5_dn17) + locals.var_temp7_dn17) + locals.var_temp9_dn17))) + (2.0 * (((locals.var_temp2_dn17 + locals.var_temp4_dn17) + locals.var_temp6_dn17) + locals.var_temp8_dn17))) + locals.var_x_dp_dn17) / 30.0)), (locals.var_xg_ac_dn18 - ((((locals.var_x_sp_dn18 + (4.0 * ((((locals.var_temp1_dn18 + locals.var_temp3_dn18) + locals.var_temp5_dn18) + locals.var_temp7_dn18) + locals.var_temp9_dn18))) + (2.0 * (((locals.var_temp2_dn18 + locals.var_temp4_dn18) + locals.var_temp6_dn18) + locals.var_temp8_dn18))) + locals.var_x_dp_dn18) / 30.0)), (locals.var_xg_ac_dn19 - ((((locals.var_x_sp_dn19 + (4.0 * ((((locals.var_temp1_dn19 + locals.var_temp3_dn19) + locals.var_temp5_dn19) + locals.var_temp7_dn19) + locals.var_temp9_dn19))) + (2.0 * (((locals.var_temp2_dn19 + locals.var_temp4_dn19) + locals.var_temp6_dn19) + locals.var_temp8_dn19))) + locals.var_x_dp_dn19) / 30.0)), (locals.var_xg_ac_dn20 - ((((locals.var_x_sp_dn20 + (4.0 * ((((locals.var_temp1_dn20 + locals.var_temp3_dn20) + locals.var_temp5_dn20) + locals.var_temp7_dn20) + locals.var_temp9_dn20))) + (2.0 * (((locals.var_temp2_dn20 + locals.var_temp4_dn20) + locals.var_temp6_dn20) + locals.var_temp8_dn20))) + locals.var_x_dp_dn20) / 30.0)),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn5, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn8, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn14, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, locals.var_qg_nqs_dn19, locals.var_qg_nqs_dn20,)
    }
};
        locals.var_qg_nqs = assign82620_e124390;
        locals.var_qg_nqs_dn5 = assign82620_e124390_d_n5;
        locals.var_qg_nqs_dn6 = assign82620_e124390_d_n6;
        locals.var_qg_nqs_dn7 = assign82620_e124390_d_n7;
        locals.var_qg_nqs_dn8 = assign82620_e124390_d_n8;
        locals.var_qg_nqs_dn12 = assign82620_e124390_d_n12;
        locals.var_qg_nqs_dn13 = assign82620_e124390_d_n13;
        locals.var_qg_nqs_dn14 = assign82620_e124390_d_n14;
        locals.var_qg_nqs_dn15 = assign82620_e124390_d_n15;
        locals.var_qg_nqs_dn16 = assign82620_e124390_d_n16;
        locals.var_qg_nqs_dn17 = assign82620_e124390_d_n17;
        locals.var_qg_nqs_dn18 = assign82620_e124390_d_n18;
        locals.var_qg_nqs_dn19 = assign82620_e124390_d_n19;
        locals.var_qg_nqs_dn20 = assign82620_e124390_d_n20;
        locals.var_qg_nqs_rv = 0.0;

        let (assign82630_e124396, assign82630_e124396_d_n5, assign82630_e124396_d_n6, assign82630_e124396_d_n7, assign82630_e124396_d_n8, assign82630_e124396_d_n12, assign82630_e124396_d_n13, assign82630_e124396_d_n14, assign82630_e124396_d_n15, assign82630_e124396_d_n16, assign82630_e124396_d_n17, assign82630_e124396_d_n18, assign82630_e124396_d_n19, assign82630_e124396_d_n20,) = {
    if (locals.var_guard2078 != 0.0) {
        let assign82630_e124394: f64 = (locals.var_pd * locals.var_qg_nqs);
        (assign82630_e124394, ((locals.var_pd_dn5 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn5)), ((locals.var_pd_dn6 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn6)), ((locals.var_pd_dn7 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn7)), ((locals.var_pd_dn8 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn8)), ((locals.var_pd_dn12 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn12)), ((locals.var_pd_dn13 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn13)), ((locals.var_pd_dn14 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn14)), ((locals.var_pd_dn15 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn15)), ((locals.var_pd_dn16 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn16)), ((locals.var_pd_dn17 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn17)), ((locals.var_pd_dn18 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn18)), ((locals.var_pd_dn19 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn19)), ((locals.var_pd_dn20 * locals.var_qg_nqs) + (locals.var_pd * locals.var_qg_nqs_dn20)),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn5, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn8, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn14, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, locals.var_qg_nqs_dn19, locals.var_qg_nqs_dn20,)
    }
};
        locals.var_qg_nqs = assign82630_e124396;
        locals.var_qg_nqs_dn5 = assign82630_e124396_d_n5;
        locals.var_qg_nqs_dn6 = assign82630_e124396_d_n6;
        locals.var_qg_nqs_dn7 = assign82630_e124396_d_n7;
        locals.var_qg_nqs_dn8 = assign82630_e124396_d_n8;
        locals.var_qg_nqs_dn12 = assign82630_e124396_d_n12;
        locals.var_qg_nqs_dn13 = assign82630_e124396_d_n13;
        locals.var_qg_nqs_dn14 = assign82630_e124396_d_n14;
        locals.var_qg_nqs_dn15 = assign82630_e124396_d_n15;
        locals.var_qg_nqs_dn16 = assign82630_e124396_d_n16;
        locals.var_qg_nqs_dn17 = assign82630_e124396_d_n17;
        locals.var_qg_nqs_dn18 = assign82630_e124396_d_n18;
        locals.var_qg_nqs_dn19 = assign82630_e124396_d_n19;
        locals.var_qg_nqs_dn20 = assign82630_e124396_d_n20;
        locals.var_qg_nqs_rv = 0.0;

        let assign82640_e124399: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2244 = assign82640_e124399;
        locals.var_guard2244_rv = 0.0;

        let (assign82650_e124409, assign82650_e124409_d_n5, assign82650_e124409_d_n6, assign82650_e124409_d_n7, assign82650_e124409_d_n8, assign82650_e124409_d_n12, assign82650_e124409_d_n13, assign82650_e124409_d_n14, assign82650_e124409_d_n15, assign82650_e124409_d_n16, assign82650_e124409_d_n17, assign82650_e124409_d_n18, assign82650_e124409_d_n19, assign82650_e124409_d_n20,) = {
    if ((locals.var_guard2078 != 0.0) && (locals.var_guard2244 != 0.0)) {
        let assign82650_e124405: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82650_e124407: f64 = (assign82650_e124405 * locals.var_qs_nqs);
        (assign82650_e124407, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qs_nqs) + (assign82650_e124405 * locals.var_qs_nqs_dn20)),)
    } else {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn12, locals.var_qs_dn13, locals.var_qs_dn14, locals.var_qs_dn15, locals.var_qs_dn16, locals.var_qs_dn17, locals.var_qs_dn18, locals.var_qs_dn19, locals.var_qs_dn20,)
    }
};
        locals.var_qs = assign82650_e124409;
        locals.var_qs_dn5 = assign82650_e124409_d_n5;
        locals.var_qs_dn6 = assign82650_e124409_d_n6;
        locals.var_qs_dn7 = assign82650_e124409_d_n7;
        locals.var_qs_dn8 = assign82650_e124409_d_n8;
        locals.var_qs_dn12 = assign82650_e124409_d_n12;
        locals.var_qs_dn13 = assign82650_e124409_d_n13;
        locals.var_qs_dn14 = assign82650_e124409_d_n14;
        locals.var_qs_dn15 = assign82650_e124409_d_n15;
        locals.var_qs_dn16 = assign82650_e124409_d_n16;
        locals.var_qs_dn17 = assign82650_e124409_d_n17;
        locals.var_qs_dn18 = assign82650_e124409_d_n18;
        locals.var_qs_dn19 = assign82650_e124409_d_n19;
        locals.var_qs_dn20 = assign82650_e124409_d_n20;
        locals.var_qs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_172(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82660_e124419, assign82660_e124419_d_n5, assign82660_e124419_d_n6, assign82660_e124419_d_n7, assign82660_e124419_d_n8, assign82660_e124419_d_n12, assign82660_e124419_d_n13, assign82660_e124419_d_n14, assign82660_e124419_d_n15, assign82660_e124419_d_n16, assign82660_e124419_d_n17, assign82660_e124419_d_n18, assign82660_e124419_d_n19, assign82660_e124419_d_n20,) = {
    if ((locals.var_guard2078 != 0.0) && (locals.var_guard2244 != 0.0)) {
        let assign82660_e124415: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82660_e124417: f64 = (assign82660_e124415 * locals.var_qd_nqs);
        (assign82660_e124417, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qd_nqs) + (assign82660_e124415 * locals.var_qd_nqs_dn20)),)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn14, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, locals.var_qd_dn19, locals.var_qd_dn20,)
    }
};
        locals.var_qd = assign82660_e124419;
        locals.var_qd_dn5 = assign82660_e124419_d_n5;
        locals.var_qd_dn6 = assign82660_e124419_d_n6;
        locals.var_qd_dn7 = assign82660_e124419_d_n7;
        locals.var_qd_dn8 = assign82660_e124419_d_n8;
        locals.var_qd_dn12 = assign82660_e124419_d_n12;
        locals.var_qd_dn13 = assign82660_e124419_d_n13;
        locals.var_qd_dn14 = assign82660_e124419_d_n14;
        locals.var_qd_dn15 = assign82660_e124419_d_n15;
        locals.var_qd_dn16 = assign82660_e124419_d_n16;
        locals.var_qd_dn17 = assign82660_e124419_d_n17;
        locals.var_qd_dn18 = assign82660_e124419_d_n18;
        locals.var_qd_dn19 = assign82660_e124419_d_n19;
        locals.var_qd_dn20 = assign82660_e124419_d_n20;
        locals.var_qd_rv = 0.0;

        let (assign82670_e124430, assign82670_e124430_d_n5, assign82670_e124430_d_n6, assign82670_e124430_d_n7, assign82670_e124430_d_n8, assign82670_e124430_d_n12, assign82670_e124430_d_n13, assign82670_e124430_d_n14, assign82670_e124430_d_n15, assign82670_e124430_d_n16, assign82670_e124430_d_n17, assign82670_e124430_d_n18, assign82670_e124430_d_n19, assign82670_e124430_d_n20,) = {
    if ((locals.var_guard2078 != 0.0) && (locals.var_guard2244 == 0.0)) {
        let assign82670_e124426: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82670_e124428: f64 = (assign82670_e124426 * locals.var_qd_nqs);
        (assign82670_e124428, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qd_nqs) + (assign82670_e124426 * locals.var_qd_nqs_dn20)),)
    } else {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn12, locals.var_qs_dn13, locals.var_qs_dn14, locals.var_qs_dn15, locals.var_qs_dn16, locals.var_qs_dn17, locals.var_qs_dn18, locals.var_qs_dn19, locals.var_qs_dn20,)
    }
};
        locals.var_qs = assign82670_e124430;
        locals.var_qs_dn5 = assign82670_e124430_d_n5;
        locals.var_qs_dn6 = assign82670_e124430_d_n6;
        locals.var_qs_dn7 = assign82670_e124430_d_n7;
        locals.var_qs_dn8 = assign82670_e124430_d_n8;
        locals.var_qs_dn12 = assign82670_e124430_d_n12;
        locals.var_qs_dn13 = assign82670_e124430_d_n13;
        locals.var_qs_dn14 = assign82670_e124430_d_n14;
        locals.var_qs_dn15 = assign82670_e124430_d_n15;
        locals.var_qs_dn16 = assign82670_e124430_d_n16;
        locals.var_qs_dn17 = assign82670_e124430_d_n17;
        locals.var_qs_dn18 = assign82670_e124430_d_n18;
        locals.var_qs_dn19 = assign82670_e124430_d_n19;
        locals.var_qs_dn20 = assign82670_e124430_d_n20;
        locals.var_qs_rv = 0.0;

        let (assign82680_e124441, assign82680_e124441_d_n5, assign82680_e124441_d_n6, assign82680_e124441_d_n7, assign82680_e124441_d_n8, assign82680_e124441_d_n12, assign82680_e124441_d_n13, assign82680_e124441_d_n14, assign82680_e124441_d_n15, assign82680_e124441_d_n16, assign82680_e124441_d_n17, assign82680_e124441_d_n18, assign82680_e124441_d_n19, assign82680_e124441_d_n20,) = {
    if ((locals.var_guard2078 != 0.0) && (locals.var_guard2244 == 0.0)) {
        let assign82680_e124437: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82680_e124439: f64 = (assign82680_e124437 * locals.var_qs_nqs);
        (assign82680_e124439, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qs_nqs) + (assign82680_e124437 * locals.var_qs_nqs_dn20)),)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn14, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, locals.var_qd_dn19, locals.var_qd_dn20,)
    }
};
        locals.var_qd = assign82680_e124441;
        locals.var_qd_dn5 = assign82680_e124441_d_n5;
        locals.var_qd_dn6 = assign82680_e124441_d_n6;
        locals.var_qd_dn7 = assign82680_e124441_d_n7;
        locals.var_qd_dn8 = assign82680_e124441_d_n8;
        locals.var_qd_dn12 = assign82680_e124441_d_n12;
        locals.var_qd_dn13 = assign82680_e124441_d_n13;
        locals.var_qd_dn14 = assign82680_e124441_d_n14;
        locals.var_qd_dn15 = assign82680_e124441_d_n15;
        locals.var_qd_dn16 = assign82680_e124441_d_n16;
        locals.var_qd_dn17 = assign82680_e124441_d_n17;
        locals.var_qd_dn18 = assign82680_e124441_d_n18;
        locals.var_qd_dn19 = assign82680_e124441_d_n19;
        locals.var_qd_dn20 = assign82680_e124441_d_n20;
        locals.var_qd_rv = 0.0;

        let (assign82690_e124449, assign82690_e124449_d_n5, assign82690_e124449_d_n6, assign82690_e124449_d_n7, assign82690_e124449_d_n8, assign82690_e124449_d_n12, assign82690_e124449_d_n13, assign82690_e124449_d_n14, assign82690_e124449_d_n15, assign82690_e124449_d_n16, assign82690_e124449_d_n17, assign82690_e124449_d_n18, assign82690_e124449_d_n19, assign82690_e124449_d_n20,) = {
    if (locals.var_guard2078 != 0.0) {
        let assign82690_e124445: f64 = (locals.var_cox_qm * locals.var_phit1_ac);
        let assign82690_e124447: f64 = (assign82690_e124445 * locals.var_qg_nqs);
        (assign82690_e124447, ((((locals.var_cox_qm_dn5 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn5)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn5)), ((((locals.var_cox_qm_dn6 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn6)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn6)), ((((locals.var_cox_qm_dn7 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn7)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn7)), ((((locals.var_cox_qm_dn8 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn8)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn8)), ((((locals.var_cox_qm_dn12 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn12)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn12)), ((((locals.var_cox_qm_dn13 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn13)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn13)), ((((locals.var_cox_qm_dn14 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn14)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn14)), ((((locals.var_cox_qm_dn15 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn15)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn15)), ((((locals.var_cox_qm_dn16 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn16)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn16)), ((((locals.var_cox_qm_dn17 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn17)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn17)), ((((locals.var_cox_qm_dn18 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn18)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn18)), ((((locals.var_cox_qm_dn19 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn19)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn19)), ((((locals.var_cox_qm_dn20 * locals.var_phit1_ac) + (locals.var_cox_qm * locals.var_phit1_ac_dn20)) * locals.var_qg_nqs) + (assign82690_e124445 * locals.var_qg_nqs_dn20)),)
    } else {
        (locals.var_qg, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn14, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, locals.var_qg_dn19, locals.var_qg_dn20,)
    }
};
        locals.var_qg = assign82690_e124449;
        locals.var_qg_dn5 = assign82690_e124449_d_n5;
        locals.var_qg_dn6 = assign82690_e124449_d_n6;
        locals.var_qg_dn7 = assign82690_e124449_d_n7;
        locals.var_qg_dn8 = assign82690_e124449_d_n8;
        locals.var_qg_dn12 = assign82690_e124449_d_n12;
        locals.var_qg_dn13 = assign82690_e124449_d_n13;
        locals.var_qg_dn14 = assign82690_e124449_d_n14;
        locals.var_qg_dn15 = assign82690_e124449_d_n15;
        locals.var_qg_dn16 = assign82690_e124449_d_n16;
        locals.var_qg_dn17 = assign82690_e124449_d_n17;
        locals.var_qg_dn18 = assign82690_e124449_d_n18;
        locals.var_qg_dn19 = assign82690_e124449_d_n19;
        locals.var_qg_dn20 = assign82690_e124449_d_n20;
        locals.var_qg_rv = 0.0;

        let (assign82700_e124458, assign82700_e124458_d_n5, assign82700_e124458_d_n6, assign82700_e124458_d_n7, assign82700_e124458_d_n8, assign82700_e124458_d_n12, assign82700_e124458_d_n13, assign82700_e124458_d_n14, assign82700_e124458_d_n15, assign82700_e124458_d_n16, assign82700_e124458_d_n17, assign82700_e124458_d_n18, assign82700_e124458_d_n19, assign82700_e124458_d_n20,) = {
    if (locals.var_guard2078 != 0.0) {
        let assign82700_e124452: f64 = (-locals.var_qg);
        let assign82700_e124454: f64 = (assign82700_e124452 - locals.var_qs);
        let assign82700_e124456: f64 = (assign82700_e124454 - locals.var_qd);
        (assign82700_e124456, (((-locals.var_qg_dn5) - locals.var_qs_dn5) - locals.var_qd_dn5), (((-locals.var_qg_dn6) - locals.var_qs_dn6) - locals.var_qd_dn6), (((-locals.var_qg_dn7) - locals.var_qs_dn7) - locals.var_qd_dn7), (((-locals.var_qg_dn8) - locals.var_qs_dn8) - locals.var_qd_dn8), (((-locals.var_qg_dn12) - locals.var_qs_dn12) - locals.var_qd_dn12), (((-locals.var_qg_dn13) - locals.var_qs_dn13) - locals.var_qd_dn13), (((-locals.var_qg_dn14) - locals.var_qs_dn14) - locals.var_qd_dn14), (((-locals.var_qg_dn15) - locals.var_qs_dn15) - locals.var_qd_dn15), (((-locals.var_qg_dn16) - locals.var_qs_dn16) - locals.var_qd_dn16), (((-locals.var_qg_dn17) - locals.var_qs_dn17) - locals.var_qd_dn17), (((-locals.var_qg_dn18) - locals.var_qs_dn18) - locals.var_qd_dn18), (((-locals.var_qg_dn19) - locals.var_qs_dn19) - locals.var_qd_dn19), (((-locals.var_qg_dn20) - locals.var_qs_dn20) - locals.var_qd_dn20),)
    } else {
        (locals.var_qb, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn14, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, locals.var_qb_dn19, locals.var_qb_dn20,)
    }
};
        locals.var_qb = assign82700_e124458;
        locals.var_qb_dn5 = assign82700_e124458_d_n5;
        locals.var_qb_dn6 = assign82700_e124458_d_n6;
        locals.var_qb_dn7 = assign82700_e124458_d_n7;
        locals.var_qb_dn8 = assign82700_e124458_d_n8;
        locals.var_qb_dn12 = assign82700_e124458_d_n12;
        locals.var_qb_dn13 = assign82700_e124458_d_n13;
        locals.var_qb_dn14 = assign82700_e124458_d_n14;
        locals.var_qb_dn15 = assign82700_e124458_d_n15;
        locals.var_qb_dn16 = assign82700_e124458_d_n16;
        locals.var_qb_dn17 = assign82700_e124458_d_n17;
        locals.var_qb_dn18 = assign82700_e124458_d_n18;
        locals.var_qb_dn19 = assign82700_e124458_d_n19;
        locals.var_qb_dn20 = assign82700_e124458_d_n20;
        locals.var_qb_rv = 0.0;

        let assign82710_e124461: f64 = (locals.var_qg + locals.var_qb);
        let assign82710_e124463: f64 = (assign82710_e124461 + locals.var_qd);
        let assign82710_e124464: f64 = (-assign82710_e124463);
        locals.var_qs = assign82710_e124464;
        locals.var_qs_dn5 = (-((locals.var_qg_dn5 + locals.var_qb_dn5) + locals.var_qd_dn5));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));
        locals.var_qs_dn12 = (-((locals.var_qg_dn12 + locals.var_qb_dn12) + locals.var_qd_dn12));
        locals.var_qs_dn13 = (-((locals.var_qg_dn13 + locals.var_qb_dn13) + locals.var_qd_dn13));
        locals.var_qs_dn14 = (-((locals.var_qg_dn14 + locals.var_qb_dn14) + locals.var_qd_dn14));
        locals.var_qs_dn15 = (-((locals.var_qg_dn15 + locals.var_qb_dn15) + locals.var_qd_dn15));
        locals.var_qs_dn16 = (-((locals.var_qg_dn16 + locals.var_qb_dn16) + locals.var_qd_dn16));
        locals.var_qs_dn17 = (-((locals.var_qg_dn17 + locals.var_qb_dn17) + locals.var_qd_dn17));
        locals.var_qs_dn18 = (-((locals.var_qg_dn18 + locals.var_qb_dn18) + locals.var_qd_dn18));
        locals.var_qs_dn19 = (-((locals.var_qg_dn19 + locals.var_qb_dn19) + locals.var_qd_dn19));
        locals.var_qs_dn20 = (-((locals.var_qg_dn20 + locals.var_qb_dn20) + locals.var_qd_dn20));
        locals.var_qs_rv = 0.0;

        let assign82760_e124495: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2246 = assign82760_e124495;
        locals.var_guard2246_rv = 0.0;

        let (assign82770_e124499, assign82770_e124499_d_n5, assign82770_e124499_d_n6, assign82770_e124499_d_n7, assign82770_e124499_d_n8, assign82770_e124499_d_n12, assign82770_e124499_d_n13, assign82770_e124499_d_n14, assign82770_e124499_d_n15, assign82770_e124499_d_n16, assign82770_e124499_d_n17, assign82770_e124499_d_n18, assign82770_e124499_d_n19, assign82770_e124499_d_n20,) = {
    if (locals.var_guard2246 != 0.0) {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn14, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, locals.var_qd_dn19, locals.var_qd_dn20,)
    } else {
        (locals.var_temp__blk2245, locals.var_temp__blk2245_dn5, locals.var_temp__blk2245_dn6, locals.var_temp__blk2245_dn7, locals.var_temp__blk2245_dn8, locals.var_temp__blk2245_dn12, locals.var_temp__blk2245_dn13, locals.var_temp__blk2245_dn14, locals.var_temp__blk2245_dn15, locals.var_temp__blk2245_dn16, locals.var_temp__blk2245_dn17, locals.var_temp__blk2245_dn18, locals.var_temp__blk2245_dn19, locals.var_temp__blk2245_dn20,)
    }
};
        locals.var_temp__blk2245 = assign82770_e124499;
        locals.var_temp__blk2245_dn5 = assign82770_e124499_d_n5;
        locals.var_temp__blk2245_dn6 = assign82770_e124499_d_n6;
        locals.var_temp__blk2245_dn7 = assign82770_e124499_d_n7;
        locals.var_temp__blk2245_dn8 = assign82770_e124499_d_n8;
        locals.var_temp__blk2245_dn12 = assign82770_e124499_d_n12;
        locals.var_temp__blk2245_dn13 = assign82770_e124499_d_n13;
        locals.var_temp__blk2245_dn14 = assign82770_e124499_d_n14;
        locals.var_temp__blk2245_dn15 = assign82770_e124499_d_n15;
        locals.var_temp__blk2245_dn16 = assign82770_e124499_d_n16;
        locals.var_temp__blk2245_dn17 = assign82770_e124499_d_n17;
        locals.var_temp__blk2245_dn18 = assign82770_e124499_d_n18;
        locals.var_temp__blk2245_dn19 = assign82770_e124499_d_n19;
        locals.var_temp__blk2245_dn20 = assign82770_e124499_d_n20;
        locals.var_temp__blk2245_rv = 0.0;

        let (assign82780_e124503, assign82780_e124503_d_n5, assign82780_e124503_d_n6, assign82780_e124503_d_n7, assign82780_e124503_d_n8, assign82780_e124503_d_n12, assign82780_e124503_d_n13, assign82780_e124503_d_n14, assign82780_e124503_d_n15, assign82780_e124503_d_n16, assign82780_e124503_d_n17, assign82780_e124503_d_n18, assign82780_e124503_d_n19, assign82780_e124503_d_n20,) = {
    if (locals.var_guard2246 != 0.0) {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn12, locals.var_qs_dn13, locals.var_qs_dn14, locals.var_qs_dn15, locals.var_qs_dn16, locals.var_qs_dn17, locals.var_qs_dn18, locals.var_qs_dn19, locals.var_qs_dn20,)
    } else {
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn14, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, locals.var_qd_dn19, locals.var_qd_dn20,)
    }
};
        locals.var_qd = assign82780_e124503;
        locals.var_qd_dn5 = assign82780_e124503_d_n5;
        locals.var_qd_dn6 = assign82780_e124503_d_n6;
        locals.var_qd_dn7 = assign82780_e124503_d_n7;
        locals.var_qd_dn8 = assign82780_e124503_d_n8;
        locals.var_qd_dn12 = assign82780_e124503_d_n12;
        locals.var_qd_dn13 = assign82780_e124503_d_n13;
        locals.var_qd_dn14 = assign82780_e124503_d_n14;
        locals.var_qd_dn15 = assign82780_e124503_d_n15;
        locals.var_qd_dn16 = assign82780_e124503_d_n16;
        locals.var_qd_dn17 = assign82780_e124503_d_n17;
        locals.var_qd_dn18 = assign82780_e124503_d_n18;
        locals.var_qd_dn19 = assign82780_e124503_d_n19;
        locals.var_qd_dn20 = assign82780_e124503_d_n20;
        locals.var_qd_rv = 0.0;

        let (assign82790_e124507, assign82790_e124507_d_n5, assign82790_e124507_d_n6, assign82790_e124507_d_n7, assign82790_e124507_d_n8, assign82790_e124507_d_n12, assign82790_e124507_d_n13, assign82790_e124507_d_n14, assign82790_e124507_d_n15, assign82790_e124507_d_n16, assign82790_e124507_d_n17, assign82790_e124507_d_n18, assign82790_e124507_d_n19, assign82790_e124507_d_n20,) = {
    if (locals.var_guard2246 != 0.0) {
        (locals.var_temp__blk2245, locals.var_temp__blk2245_dn5, locals.var_temp__blk2245_dn6, locals.var_temp__blk2245_dn7, locals.var_temp__blk2245_dn8, locals.var_temp__blk2245_dn12, locals.var_temp__blk2245_dn13, locals.var_temp__blk2245_dn14, locals.var_temp__blk2245_dn15, locals.var_temp__blk2245_dn16, locals.var_temp__blk2245_dn17, locals.var_temp__blk2245_dn18, locals.var_temp__blk2245_dn19, locals.var_temp__blk2245_dn20,)
    } else {
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn12, locals.var_qs_dn13, locals.var_qs_dn14, locals.var_qs_dn15, locals.var_qs_dn16, locals.var_qs_dn17, locals.var_qs_dn18, locals.var_qs_dn19, locals.var_qs_dn20,)
    }
};
        locals.var_qs = assign82790_e124507;
        locals.var_qs_dn5 = assign82790_e124507_d_n5;
        locals.var_qs_dn6 = assign82790_e124507_d_n6;
        locals.var_qs_dn7 = assign82790_e124507_d_n7;
        locals.var_qs_dn8 = assign82790_e124507_d_n8;
        locals.var_qs_dn12 = assign82790_e124507_d_n12;
        locals.var_qs_dn13 = assign82790_e124507_d_n13;
        locals.var_qs_dn14 = assign82790_e124507_d_n14;
        locals.var_qs_dn15 = assign82790_e124507_d_n15;
        locals.var_qs_dn16 = assign82790_e124507_d_n16;
        locals.var_qs_dn17 = assign82790_e124507_d_n17;
        locals.var_qs_dn18 = assign82790_e124507_d_n18;
        locals.var_qs_dn19 = assign82790_e124507_d_n19;
        locals.var_qs_dn20 = assign82790_e124507_d_n20;
        locals.var_qs_rv = 0.0;

        let assign82860_e124516: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        locals.var_cgeff = assign82860_e124516;
        locals.var_cgeff_dn5 = ((locals.var_cox_qm_dn5 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn5));
        locals.var_cgeff_dn6 = ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6));
        locals.var_cgeff_dn7 = ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7));
        locals.var_cgeff_dn8 = ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8));
        locals.var_cgeff_dn12 = ((locals.var_cox_qm_dn12 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn12));
        locals.var_cgeff_dn13 = ((locals.var_cox_qm_dn13 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn13));
        locals.var_cgeff_dn14 = ((locals.var_cox_qm_dn14 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn14));
        locals.var_cgeff_dn15 = ((locals.var_cox_qm_dn15 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn15));
        locals.var_cgeff_dn16 = ((locals.var_cox_qm_dn16 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn16));
        locals.var_cgeff_dn17 = ((locals.var_cox_qm_dn17 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn17));
        locals.var_cgeff_dn18 = ((locals.var_cox_qm_dn18 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn18));
        locals.var_cgeff_dn19 = ((locals.var_cox_qm_dn19 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn19));
        locals.var_cgeff_dn20 = ((locals.var_cox_qm_dn20 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn20));
        locals.var_cgeff_rv = 0.0;

        let assign82920_e124528: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign82920_e124528;
        locals.var_guard2279_rv = 0.0;

        let assign83230_e124894: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign83230_e124894;
        locals.var_guard2284_rv = 0.0;

        let (assign83280_e124991, assign83280_e124991_d_n5, assign83280_e124991_d_n6, assign83280_e124991_d_n7, assign83280_e124991_d_n8, assign83280_e124991_d_n12, assign83280_e124991_d_n13, assign83280_e124991_d_n14, assign83280_e124991_d_n15, assign83280_e124991_d_n16, assign83280_e124991_d_n17, assign83280_e124991_d_n18, assign83280_e124991_d_n19, assign83280_e124991_d_n20,) = {
    if ((locals.var_guard2279 != 0.0) && (locals.var_guard2284 != 0.0)) {
        let assign83280_e124981: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
        let assign83280_e124983: f64 = (assign83280_e124981 * locals.var_cox_qm);
        let assign83280_e124985: f64 = (assign83280_e124983 * locals.var_eta_p_ac);
        let assign83280_e124988: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
        let assign83280_e124989: f64 = (assign83280_e124985 / assign83280_e124988);
        (assign83280_e124989, (((((((((locals.var_gvsat_ac_dn5 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn5)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn5)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn5)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn5 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn5)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn6)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn7)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn8)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn12 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn12)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn12)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn12)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn12 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn12)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn13 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn13)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn13)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn13)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn13 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn13)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn14 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn14)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn14)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn14)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn14 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn14)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn15 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn15)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn15)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn15)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn15 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn15)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn16 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn16)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn16)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn16)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn16 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn16)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn17 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn17)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn17)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn17)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn17 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn17)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn18 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn18)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn18)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn18)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn18 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn18)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn19 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn19)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn19)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn19)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn19 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn19)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((locals.var_gvsat_ac_dn20 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn20)) * locals.var_cox_qm) + (assign83280_e124981 * locals.var_cox_qm_dn20)) * locals.var_eta_p_ac) + (assign83280_e124983 * locals.var_eta_p_ac_dn20)) * assign83280_e124988) - (assign83280_e124985 * ((locals.var_gmob_dl_ac_dn20 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn20)))) / (assign83280_e124988 * assign83280_e124988)),)
    } else {
        (locals.var_cgeff, locals.var_cgeff_dn5, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn12, locals.var_cgeff_dn13, locals.var_cgeff_dn14, locals.var_cgeff_dn15, locals.var_cgeff_dn16, locals.var_cgeff_dn17, locals.var_cgeff_dn18, locals.var_cgeff_dn19, locals.var_cgeff_dn20,)
    }
};
        locals.var_cgeff = assign83280_e124991;
        locals.var_cgeff_dn5 = assign83280_e124991_d_n5;
        locals.var_cgeff_dn6 = assign83280_e124991_d_n6;
        locals.var_cgeff_dn7 = assign83280_e124991_d_n7;
        locals.var_cgeff_dn8 = assign83280_e124991_d_n8;
        locals.var_cgeff_dn12 = assign83280_e124991_d_n12;
        locals.var_cgeff_dn13 = assign83280_e124991_d_n13;
        locals.var_cgeff_dn14 = assign83280_e124991_d_n14;
        locals.var_cgeff_dn15 = assign83280_e124991_d_n15;
        locals.var_cgeff_dn16 = assign83280_e124991_d_n16;
        locals.var_cgeff_dn17 = assign83280_e124991_d_n17;
        locals.var_cgeff_dn18 = assign83280_e124991_d_n18;
        locals.var_cgeff_dn19 = assign83280_e124991_d_n19;
        locals.var_cgeff_dn20 = assign83280_e124991_d_n20;
        locals.var_cgeff_rv = 0.0;

        let assign83540_e125205: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign83540_e125205;
        locals.var_guard2288_rv = 0.0;

        let (assign83550_e125213, assign83550_e125213_d_n5, assign83550_e125213_d_n6, assign83550_e125213_d_n7, assign83550_e125213_d_n8, assign83550_e125213_d_n12, assign83550_e125213_d_n13, assign83550_e125213_d_n14, assign83550_e125213_d_n15, assign83550_e125213_d_n16, assign83550_e125213_d_n17, assign83550_e125213_d_n18, assign83550_e125213_d_n19, assign83550_e125213_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83550_e125209: f64 = (4.0 * locals.var_dsqredge);
        let assign83550_e125211: f64 = (assign83550_e125209 / locals.var_gfedge2);
        (assign83550_e125211, ((4.0 * locals.var_dsqredge_dn5) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn12) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn13) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn14) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn15) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn16) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn17) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn18) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn19) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn20) / locals.var_gfedge2),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83550_e125213;
        locals.var_temp1_dn5 = assign83550_e125213_d_n5;
        locals.var_temp1_dn6 = assign83550_e125213_d_n6;
        locals.var_temp1_dn7 = assign83550_e125213_d_n7;
        locals.var_temp1_dn8 = assign83550_e125213_d_n8;
        locals.var_temp1_dn12 = assign83550_e125213_d_n12;
        locals.var_temp1_dn13 = assign83550_e125213_d_n13;
        locals.var_temp1_dn14 = assign83550_e125213_d_n14;
        locals.var_temp1_dn15 = assign83550_e125213_d_n15;
        locals.var_temp1_dn16 = assign83550_e125213_d_n16;
        locals.var_temp1_dn17 = assign83550_e125213_d_n17;
        locals.var_temp1_dn18 = assign83550_e125213_d_n18;
        locals.var_temp1_dn19 = assign83550_e125213_d_n19;
        locals.var_temp1_dn20 = assign83550_e125213_d_n20;
        locals.var_temp1_rv = 0.0;

        let (assign83570_e125233, assign83570_e125233_d_n5, assign83570_e125233_d_n6, assign83570_e125233_d_n7, assign83570_e125233_d_n8, assign83570_e125233_d_n12, assign83570_e125233_d_n13, assign83570_e125233_d_n14, assign83570_e125233_d_n15, assign83570_e125233_d_n16, assign83570_e125233_d_n17, assign83570_e125233_d_n18, assign83570_e125233_d_n19, assign83570_e125233_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83570_e125231: f64 = (locals.var_cox_over_q * locals.var_phit);
        (assign83570_e125231, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83570_e125233;
        locals.var_temp1_dn5 = assign83570_e125233_d_n5;
        locals.var_temp1_dn6 = assign83570_e125233_d_n6;
        locals.var_temp1_dn7 = assign83570_e125233_d_n7;
        locals.var_temp1_dn8 = assign83570_e125233_d_n8;
        locals.var_temp1_dn12 = assign83570_e125233_d_n12;
        locals.var_temp1_dn13 = assign83570_e125233_d_n13;
        locals.var_temp1_dn14 = assign83570_e125233_d_n14;
        locals.var_temp1_dn15 = assign83570_e125233_d_n15;
        locals.var_temp1_dn16 = assign83570_e125233_d_n16;
        locals.var_temp1_dn17 = assign83570_e125233_d_n17;
        locals.var_temp1_dn18 = assign83570_e125233_d_n18;
        locals.var_temp1_dn19 = assign83570_e125233_d_n19;
        locals.var_temp1_dn20 = assign83570_e125233_d_n20;
        locals.var_temp1_rv = 0.0;

        let (assign83700_e125373, assign83700_e125373_d_n5, assign83700_e125373_d_n6, assign83700_e125373_d_n7, assign83700_e125373_d_n8, assign83700_e125373_d_n12, assign83700_e125373_d_n13, assign83700_e125373_d_n14, assign83700_e125373_d_n15, assign83700_e125373_d_n16, assign83700_e125373_d_n17, assign83700_e125373_d_n18, assign83700_e125373_d_n19, assign83700_e125373_d_n20,) = {
    if (locals.var_guard2288 != 0.0) {
        let assign83700_e125371: f64 = (locals.var_alpha_dc * locals.var_h_dc);
        (assign83700_e125371, ((locals.var_alpha_dc_dn5 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn5)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)), ((locals.var_alpha_dc_dn12 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn12)), ((locals.var_alpha_dc_dn13 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn13)), ((locals.var_alpha_dc_dn14 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn14)), ((locals.var_alpha_dc_dn15 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn15)), ((locals.var_alpha_dc_dn16 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn16)), ((locals.var_alpha_dc_dn17 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn17)), ((locals.var_alpha_dc_dn18 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn18)), ((locals.var_alpha_dc_dn19 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn19)), ((locals.var_alpha_dc_dn20 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn20)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn12, locals.var_temp1_dn13, locals.var_temp1_dn14, locals.var_temp1_dn15, locals.var_temp1_dn16, locals.var_temp1_dn17, locals.var_temp1_dn18, locals.var_temp1_dn19, locals.var_temp1_dn20,)
    }
};
        locals.var_temp1 = assign83700_e125373;
        locals.var_temp1_dn5 = assign83700_e125373_d_n5;
        locals.var_temp1_dn6 = assign83700_e125373_d_n6;
        locals.var_temp1_dn7 = assign83700_e125373_d_n7;
        locals.var_temp1_dn8 = assign83700_e125373_d_n8;
        locals.var_temp1_dn12 = assign83700_e125373_d_n12;
        locals.var_temp1_dn13 = assign83700_e125373_d_n13;
        locals.var_temp1_dn14 = assign83700_e125373_d_n14;
        locals.var_temp1_dn15 = assign83700_e125373_d_n15;
        locals.var_temp1_dn16 = assign83700_e125373_d_n16;
        locals.var_temp1_dn17 = assign83700_e125373_d_n17;
        locals.var_temp1_dn18 = assign83700_e125373_d_n18;
        locals.var_temp1_dn19 = assign83700_e125373_d_n19;
        locals.var_temp1_dn20 = assign83700_e125373_d_n20;
        locals.var_temp1_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let (eq0_e955, eq0_e955_d_n5, eq0_e955_d_n6, eq0_e955_d_n7, eq0_e955_d_n8, eq0_e955_d_n12, eq0_e955_d_n13, eq0_e955_d_n14, eq0_e955_d_n15, eq0_e955_d_n16, eq0_e955_d_n17, eq0_e955_d_n18, eq0_e955_d_n19, eq0_e955_d_n20,) = {
    if (locals.var_guard1924 != 0.0) {
        let eq0_e949: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq0_e951: f64 = (eq0_e949 * p.p32);
        let eq0_e953: f64 = (eq0_e951 * locals.var_iimpact);
        let eq0_e953_d_n5: f64 = (eq0_e951 * locals.var_iimpact_dn5);
        let eq0_e953_d_n6: f64 = (eq0_e951 * locals.var_iimpact_dn6);
        let eq0_e953_d_n7: f64 = (eq0_e951 * locals.var_iimpact_dn7);
        let eq0_e953_d_n8: f64 = (eq0_e951 * locals.var_iimpact_dn8);
        let eq0_e953_d_n12: f64 = (eq0_e951 * locals.var_iimpact_dn12);
        let eq0_e953_d_n13: f64 = (eq0_e951 * locals.var_iimpact_dn13);
        let eq0_e953_d_n14: f64 = (eq0_e951 * locals.var_iimpact_dn14);
        let eq0_e953_d_n15: f64 = (eq0_e951 * locals.var_iimpact_dn15);
        let eq0_e953_d_n16: f64 = (eq0_e951 * locals.var_iimpact_dn16);
        let eq0_e953_d_n17: f64 = (eq0_e951 * locals.var_iimpact_dn17);
        let eq0_e953_d_n18: f64 = (eq0_e951 * locals.var_iimpact_dn18);
        let eq0_e953_d_n19: f64 = (eq0_e951 * locals.var_iimpact_dn19);
        let eq0_e953_d_n20: f64 = (eq0_e951 * locals.var_iimpact_dn20);
        (eq0_e953, eq0_e953_d_n5, eq0_e953_d_n6, eq0_e953_d_n7, eq0_e953_d_n8, eq0_e953_d_n12, eq0_e953_d_n13, eq0_e953_d_n14, eq0_e953_d_n15, eq0_e953_d_n16, eq0_e953_d_n17, eq0_e953_d_n18, eq0_e953_d_n19, eq0_e953_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e955;
        let eq0_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq0_node_derivatives: [f64; 13] = [eq0_e955_d_n5, eq0_e955_d_n6, eq0_e955_d_n7, eq0_e955_d_n8, eq0_e955_d_n12, eq0_e955_d_n13, eq0_e955_d_n14, eq0_e955_d_n15, eq0_e955_d_n16, eq0_e955_d_n17, eq0_e955_d_n18, eq0_e955_d_n19, eq0_e955_d_n20];
        let eq0_branch_derivative_indices: [usize; 0] = [];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            &eq0_node_derivative_indices,
            &eq0_node_derivatives,
            &eq0_branch_derivative_indices,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e967, eq1_e967_d_n5, eq1_e967_d_n6, eq1_e967_d_n7, eq1_e967_d_n8, eq1_e967_d_n12, eq1_e967_d_n13, eq1_e967_d_n14, eq1_e967_d_n15, eq1_e967_d_n16, eq1_e967_d_n17, eq1_e967_d_n18, eq1_e967_d_n19, eq1_e967_d_n20,) = {
    if (locals.var_guard1924 != 0.0) {
        let eq1_e959: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq1_e961: f64 = (eq1_e959 * p.p32);
        let eq1_e964: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let eq1_e964_d_n5: f64 = (locals.var_i_ds_dn5 + locals.var_i_dsedge_dn5);
        let eq1_e964_d_n6: f64 = (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6);
        let eq1_e964_d_n7: f64 = (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7);
        let eq1_e964_d_n8: f64 = (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8);
        let eq1_e964_d_n12: f64 = (locals.var_i_ds_dn12 + locals.var_i_dsedge_dn12);
        let eq1_e964_d_n13: f64 = (locals.var_i_ds_dn13 + locals.var_i_dsedge_dn13);
        let eq1_e964_d_n14: f64 = (locals.var_i_ds_dn14 + locals.var_i_dsedge_dn14);
        let eq1_e964_d_n15: f64 = (locals.var_i_ds_dn15 + locals.var_i_dsedge_dn15);
        let eq1_e964_d_n16: f64 = (locals.var_i_ds_dn16 + locals.var_i_dsedge_dn16);
        let eq1_e964_d_n17: f64 = (locals.var_i_ds_dn17 + locals.var_i_dsedge_dn17);
        let eq1_e964_d_n18: f64 = (locals.var_i_ds_dn18 + locals.var_i_dsedge_dn18);
        let eq1_e964_d_n19: f64 = (locals.var_i_ds_dn19 + locals.var_i_dsedge_dn19);
        let eq1_e964_d_n20: f64 = (locals.var_i_ds_dn20 + locals.var_i_dsedge_dn20);
        let eq1_e965: f64 = (eq1_e961 * eq1_e964);
        let eq1_e965_d_n5: f64 = (eq1_e961 * eq1_e964_d_n5);
        let eq1_e965_d_n6: f64 = (eq1_e961 * eq1_e964_d_n6);
        let eq1_e965_d_n7: f64 = (eq1_e961 * eq1_e964_d_n7);
        let eq1_e965_d_n8: f64 = (eq1_e961 * eq1_e964_d_n8);
        let eq1_e965_d_n12: f64 = (eq1_e961 * eq1_e964_d_n12);
        let eq1_e965_d_n13: f64 = (eq1_e961 * eq1_e964_d_n13);
        let eq1_e965_d_n14: f64 = (eq1_e961 * eq1_e964_d_n14);
        let eq1_e965_d_n15: f64 = (eq1_e961 * eq1_e964_d_n15);
        let eq1_e965_d_n16: f64 = (eq1_e961 * eq1_e964_d_n16);
        let eq1_e965_d_n17: f64 = (eq1_e961 * eq1_e964_d_n17);
        let eq1_e965_d_n18: f64 = (eq1_e961 * eq1_e964_d_n18);
        let eq1_e965_d_n19: f64 = (eq1_e961 * eq1_e964_d_n19);
        let eq1_e965_d_n20: f64 = (eq1_e961 * eq1_e964_d_n20);
        (eq1_e965, eq1_e965_d_n5, eq1_e965_d_n6, eq1_e965_d_n7, eq1_e965_d_n8, eq1_e965_d_n12, eq1_e965_d_n13, eq1_e965_d_n14, eq1_e965_d_n15, eq1_e965_d_n16, eq1_e965_d_n17, eq1_e965_d_n18, eq1_e965_d_n19, eq1_e965_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e967;
        let eq1_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq1_node_derivatives: [f64; 13] = [eq1_e967_d_n5, eq1_e967_d_n6, eq1_e967_d_n7, eq1_e967_d_n8, eq1_e967_d_n12, eq1_e967_d_n13, eq1_e967_d_n14, eq1_e967_d_n15, eq1_e967_d_n16, eq1_e967_d_n17, eq1_e967_d_n18, eq1_e967_d_n19, eq1_e967_d_n20];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e977, eq2_e977_d_n5, eq2_e977_d_n6, eq2_e977_d_n7, eq2_e977_d_n8, eq2_e977_d_n12, eq2_e977_d_n13, eq2_e977_d_n14, eq2_e977_d_n15, eq2_e977_d_n16, eq2_e977_d_n17, eq2_e977_d_n18, eq2_e977_d_n19, eq2_e977_d_n20,) = {
    if (locals.var_guard1924 != 0.0) {
        let eq2_e971: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq2_e973: f64 = (eq2_e971 * p.p32);
        let eq2_e975: f64 = (eq2_e973 * locals.var_i_gcs);
        let eq2_e975_d_n5: f64 = (eq2_e973 * locals.var_i_gcs_dn5);
        let eq2_e975_d_n6: f64 = (eq2_e973 * locals.var_i_gcs_dn6);
        let eq2_e975_d_n7: f64 = (eq2_e973 * locals.var_i_gcs_dn7);
        let eq2_e975_d_n8: f64 = (eq2_e973 * locals.var_i_gcs_dn8);
        let eq2_e975_d_n12: f64 = (eq2_e973 * locals.var_i_gcs_dn12);
        let eq2_e975_d_n13: f64 = (eq2_e973 * locals.var_i_gcs_dn13);
        let eq2_e975_d_n14: f64 = (eq2_e973 * locals.var_i_gcs_dn14);
        let eq2_e975_d_n15: f64 = (eq2_e973 * locals.var_i_gcs_dn15);
        let eq2_e975_d_n16: f64 = (eq2_e973 * locals.var_i_gcs_dn16);
        let eq2_e975_d_n17: f64 = (eq2_e973 * locals.var_i_gcs_dn17);
        let eq2_e975_d_n18: f64 = (eq2_e973 * locals.var_i_gcs_dn18);
        let eq2_e975_d_n19: f64 = (eq2_e973 * locals.var_i_gcs_dn19);
        let eq2_e975_d_n20: f64 = (eq2_e973 * locals.var_i_gcs_dn20);
        (eq2_e975, eq2_e975_d_n5, eq2_e975_d_n6, eq2_e975_d_n7, eq2_e975_d_n8, eq2_e975_d_n12, eq2_e975_d_n13, eq2_e975_d_n14, eq2_e975_d_n15, eq2_e975_d_n16, eq2_e975_d_n17, eq2_e975_d_n18, eq2_e975_d_n19, eq2_e975_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e977;
        let eq2_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq2_node_derivatives: [f64; 13] = [eq2_e977_d_n5, eq2_e977_d_n6, eq2_e977_d_n7, eq2_e977_d_n8, eq2_e977_d_n12, eq2_e977_d_n13, eq2_e977_d_n14, eq2_e977_d_n15, eq2_e977_d_n16, eq2_e977_d_n17, eq2_e977_d_n18, eq2_e977_d_n19, eq2_e977_d_n20];
        let eq2_branch_derivative_indices: [usize; 0] = [];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e987, eq3_e987_d_n5, eq3_e987_d_n6, eq3_e987_d_n7, eq3_e987_d_n8, eq3_e987_d_n12, eq3_e987_d_n13, eq3_e987_d_n14, eq3_e987_d_n15, eq3_e987_d_n16, eq3_e987_d_n17, eq3_e987_d_n18, eq3_e987_d_n19, eq3_e987_d_n20,) = {
    if (locals.var_guard1924 != 0.0) {
        let eq3_e981: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq3_e983: f64 = (eq3_e981 * p.p32);
        let eq3_e985: f64 = (eq3_e983 * locals.var_i_gcd);
        let eq3_e985_d_n5: f64 = (eq3_e983 * locals.var_i_gcd_dn5);
        let eq3_e985_d_n6: f64 = (eq3_e983 * locals.var_i_gcd_dn6);
        let eq3_e985_d_n7: f64 = (eq3_e983 * locals.var_i_gcd_dn7);
        let eq3_e985_d_n8: f64 = (eq3_e983 * locals.var_i_gcd_dn8);
        let eq3_e985_d_n12: f64 = (eq3_e983 * locals.var_i_gcd_dn12);
        let eq3_e985_d_n13: f64 = (eq3_e983 * locals.var_i_gcd_dn13);
        let eq3_e985_d_n14: f64 = (eq3_e983 * locals.var_i_gcd_dn14);
        let eq3_e985_d_n15: f64 = (eq3_e983 * locals.var_i_gcd_dn15);
        let eq3_e985_d_n16: f64 = (eq3_e983 * locals.var_i_gcd_dn16);
        let eq3_e985_d_n17: f64 = (eq3_e983 * locals.var_i_gcd_dn17);
        let eq3_e985_d_n18: f64 = (eq3_e983 * locals.var_i_gcd_dn18);
        let eq3_e985_d_n19: f64 = (eq3_e983 * locals.var_i_gcd_dn19);
        let eq3_e985_d_n20: f64 = (eq3_e983 * locals.var_i_gcd_dn20);
        (eq3_e985, eq3_e985_d_n5, eq3_e985_d_n6, eq3_e985_d_n7, eq3_e985_d_n8, eq3_e985_d_n12, eq3_e985_d_n13, eq3_e985_d_n14, eq3_e985_d_n15, eq3_e985_d_n16, eq3_e985_d_n17, eq3_e985_d_n18, eq3_e985_d_n19, eq3_e985_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e987;
        let eq3_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq3_node_derivatives: [f64; 13] = [eq3_e987_d_n5, eq3_e987_d_n6, eq3_e987_d_n7, eq3_e987_d_n8, eq3_e987_d_n12, eq3_e987_d_n13, eq3_e987_d_n14, eq3_e987_d_n15, eq3_e987_d_n16, eq3_e987_d_n17, eq3_e987_d_n18, eq3_e987_d_n19, eq3_e987_d_n20];
        let eq3_branch_derivative_indices: [usize; 0] = [];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivative_indices,
            &eq3_node_derivatives,
            &eq3_branch_derivative_indices,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e998, eq4_e998_d_n5, eq4_e998_d_n6, eq4_e998_d_n7, eq4_e998_d_n8, eq4_e998_d_n12, eq4_e998_d_n13, eq4_e998_d_n14, eq4_e998_d_n15, eq4_e998_d_n16, eq4_e998_d_n17, eq4_e998_d_n18, eq4_e998_d_n19, eq4_e998_d_n20,) = {
    if (locals.var_guard1924 == 0.0) {
        let eq4_e992: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq4_e994: f64 = (eq4_e992 * p.p32);
        let eq4_e996: f64 = (eq4_e994 * locals.var_iimpact);
        let eq4_e996_d_n5: f64 = (eq4_e994 * locals.var_iimpact_dn5);
        let eq4_e996_d_n6: f64 = (eq4_e994 * locals.var_iimpact_dn6);
        let eq4_e996_d_n7: f64 = (eq4_e994 * locals.var_iimpact_dn7);
        let eq4_e996_d_n8: f64 = (eq4_e994 * locals.var_iimpact_dn8);
        let eq4_e996_d_n12: f64 = (eq4_e994 * locals.var_iimpact_dn12);
        let eq4_e996_d_n13: f64 = (eq4_e994 * locals.var_iimpact_dn13);
        let eq4_e996_d_n14: f64 = (eq4_e994 * locals.var_iimpact_dn14);
        let eq4_e996_d_n15: f64 = (eq4_e994 * locals.var_iimpact_dn15);
        let eq4_e996_d_n16: f64 = (eq4_e994 * locals.var_iimpact_dn16);
        let eq4_e996_d_n17: f64 = (eq4_e994 * locals.var_iimpact_dn17);
        let eq4_e996_d_n18: f64 = (eq4_e994 * locals.var_iimpact_dn18);
        let eq4_e996_d_n19: f64 = (eq4_e994 * locals.var_iimpact_dn19);
        let eq4_e996_d_n20: f64 = (eq4_e994 * locals.var_iimpact_dn20);
        (eq4_e996, eq4_e996_d_n5, eq4_e996_d_n6, eq4_e996_d_n7, eq4_e996_d_n8, eq4_e996_d_n12, eq4_e996_d_n13, eq4_e996_d_n14, eq4_e996_d_n15, eq4_e996_d_n16, eq4_e996_d_n17, eq4_e996_d_n18, eq4_e996_d_n19, eq4_e996_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e998;
        let eq4_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq4_node_derivatives: [f64; 13] = [eq4_e998_d_n5, eq4_e998_d_n6, eq4_e998_d_n7, eq4_e998_d_n8, eq4_e998_d_n12, eq4_e998_d_n13, eq4_e998_d_n14, eq4_e998_d_n15, eq4_e998_d_n16, eq4_e998_d_n17, eq4_e998_d_n18, eq4_e998_d_n19, eq4_e998_d_n20];
        let eq4_branch_derivative_indices: [usize; 0] = [];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1011, eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20,) = {
    if (locals.var_guard1924 == 0.0) {
        let eq5_e1003: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq5_e1005: f64 = (eq5_e1003 * p.p32);
        let eq5_e1008: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let eq5_e1008_d_n5: f64 = (locals.var_i_ds_dn5 + locals.var_i_dsedge_dn5);
        let eq5_e1008_d_n6: f64 = (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6);
        let eq5_e1008_d_n7: f64 = (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7);
        let eq5_e1008_d_n8: f64 = (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8);
        let eq5_e1008_d_n12: f64 = (locals.var_i_ds_dn12 + locals.var_i_dsedge_dn12);
        let eq5_e1008_d_n13: f64 = (locals.var_i_ds_dn13 + locals.var_i_dsedge_dn13);
        let eq5_e1008_d_n14: f64 = (locals.var_i_ds_dn14 + locals.var_i_dsedge_dn14);
        let eq5_e1008_d_n15: f64 = (locals.var_i_ds_dn15 + locals.var_i_dsedge_dn15);
        let eq5_e1008_d_n16: f64 = (locals.var_i_ds_dn16 + locals.var_i_dsedge_dn16);
        let eq5_e1008_d_n17: f64 = (locals.var_i_ds_dn17 + locals.var_i_dsedge_dn17);
        let eq5_e1008_d_n18: f64 = (locals.var_i_ds_dn18 + locals.var_i_dsedge_dn18);
        let eq5_e1008_d_n19: f64 = (locals.var_i_ds_dn19 + locals.var_i_dsedge_dn19);
        let eq5_e1008_d_n20: f64 = (locals.var_i_ds_dn20 + locals.var_i_dsedge_dn20);
        let eq5_e1009: f64 = (eq5_e1005 * eq5_e1008);
        let eq5_e1009_d_n5: f64 = (eq5_e1005 * eq5_e1008_d_n5);
        let eq5_e1009_d_n6: f64 = (eq5_e1005 * eq5_e1008_d_n6);
        let eq5_e1009_d_n7: f64 = (eq5_e1005 * eq5_e1008_d_n7);
        let eq5_e1009_d_n8: f64 = (eq5_e1005 * eq5_e1008_d_n8);
        let eq5_e1009_d_n12: f64 = (eq5_e1005 * eq5_e1008_d_n12);
        let eq5_e1009_d_n13: f64 = (eq5_e1005 * eq5_e1008_d_n13);
        let eq5_e1009_d_n14: f64 = (eq5_e1005 * eq5_e1008_d_n14);
        let eq5_e1009_d_n15: f64 = (eq5_e1005 * eq5_e1008_d_n15);
        let eq5_e1009_d_n16: f64 = (eq5_e1005 * eq5_e1008_d_n16);
        let eq5_e1009_d_n17: f64 = (eq5_e1005 * eq5_e1008_d_n17);
        let eq5_e1009_d_n18: f64 = (eq5_e1005 * eq5_e1008_d_n18);
        let eq5_e1009_d_n19: f64 = (eq5_e1005 * eq5_e1008_d_n19);
        let eq5_e1009_d_n20: f64 = (eq5_e1005 * eq5_e1008_d_n20);
        (eq5_e1009, eq5_e1009_d_n5, eq5_e1009_d_n6, eq5_e1009_d_n7, eq5_e1009_d_n8, eq5_e1009_d_n12, eq5_e1009_d_n13, eq5_e1009_d_n14, eq5_e1009_d_n15, eq5_e1009_d_n16, eq5_e1009_d_n17, eq5_e1009_d_n18, eq5_e1009_d_n19, eq5_e1009_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1011;
        let eq5_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq5_node_derivatives: [f64; 13] = [eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1022, eq6_e1022_d_n5, eq6_e1022_d_n6, eq6_e1022_d_n7, eq6_e1022_d_n8, eq6_e1022_d_n12, eq6_e1022_d_n13, eq6_e1022_d_n14, eq6_e1022_d_n15, eq6_e1022_d_n16, eq6_e1022_d_n17, eq6_e1022_d_n18, eq6_e1022_d_n19, eq6_e1022_d_n20,) = {
    if (locals.var_guard1924 == 0.0) {
        let eq6_e1016: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq6_e1018: f64 = (eq6_e1016 * p.p32);
        let eq6_e1020: f64 = (eq6_e1018 * locals.var_i_gcs);
        let eq6_e1020_d_n5: f64 = (eq6_e1018 * locals.var_i_gcs_dn5);
        let eq6_e1020_d_n6: f64 = (eq6_e1018 * locals.var_i_gcs_dn6);
        let eq6_e1020_d_n7: f64 = (eq6_e1018 * locals.var_i_gcs_dn7);
        let eq6_e1020_d_n8: f64 = (eq6_e1018 * locals.var_i_gcs_dn8);
        let eq6_e1020_d_n12: f64 = (eq6_e1018 * locals.var_i_gcs_dn12);
        let eq6_e1020_d_n13: f64 = (eq6_e1018 * locals.var_i_gcs_dn13);
        let eq6_e1020_d_n14: f64 = (eq6_e1018 * locals.var_i_gcs_dn14);
        let eq6_e1020_d_n15: f64 = (eq6_e1018 * locals.var_i_gcs_dn15);
        let eq6_e1020_d_n16: f64 = (eq6_e1018 * locals.var_i_gcs_dn16);
        let eq6_e1020_d_n17: f64 = (eq6_e1018 * locals.var_i_gcs_dn17);
        let eq6_e1020_d_n18: f64 = (eq6_e1018 * locals.var_i_gcs_dn18);
        let eq6_e1020_d_n19: f64 = (eq6_e1018 * locals.var_i_gcs_dn19);
        let eq6_e1020_d_n20: f64 = (eq6_e1018 * locals.var_i_gcs_dn20);
        (eq6_e1020, eq6_e1020_d_n5, eq6_e1020_d_n6, eq6_e1020_d_n7, eq6_e1020_d_n8, eq6_e1020_d_n12, eq6_e1020_d_n13, eq6_e1020_d_n14, eq6_e1020_d_n15, eq6_e1020_d_n16, eq6_e1020_d_n17, eq6_e1020_d_n18, eq6_e1020_d_n19, eq6_e1020_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1022;
        let eq6_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq6_node_derivatives: [f64; 13] = [eq6_e1022_d_n5, eq6_e1022_d_n6, eq6_e1022_d_n7, eq6_e1022_d_n8, eq6_e1022_d_n12, eq6_e1022_d_n13, eq6_e1022_d_n14, eq6_e1022_d_n15, eq6_e1022_d_n16, eq6_e1022_d_n17, eq6_e1022_d_n18, eq6_e1022_d_n19, eq6_e1022_d_n20];
        let eq6_branch_derivative_indices: [usize; 0] = [];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivative_indices,
            &eq6_node_derivatives,
            &eq6_branch_derivative_indices,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1033, eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20,) = {
    if (locals.var_guard1924 == 0.0) {
        let eq7_e1027: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq7_e1029: f64 = (eq7_e1027 * p.p32);
        let eq7_e1031: f64 = (eq7_e1029 * locals.var_i_gcd);
        let eq7_e1031_d_n5: f64 = (eq7_e1029 * locals.var_i_gcd_dn5);
        let eq7_e1031_d_n6: f64 = (eq7_e1029 * locals.var_i_gcd_dn6);
        let eq7_e1031_d_n7: f64 = (eq7_e1029 * locals.var_i_gcd_dn7);
        let eq7_e1031_d_n8: f64 = (eq7_e1029 * locals.var_i_gcd_dn8);
        let eq7_e1031_d_n12: f64 = (eq7_e1029 * locals.var_i_gcd_dn12);
        let eq7_e1031_d_n13: f64 = (eq7_e1029 * locals.var_i_gcd_dn13);
        let eq7_e1031_d_n14: f64 = (eq7_e1029 * locals.var_i_gcd_dn14);
        let eq7_e1031_d_n15: f64 = (eq7_e1029 * locals.var_i_gcd_dn15);
        let eq7_e1031_d_n16: f64 = (eq7_e1029 * locals.var_i_gcd_dn16);
        let eq7_e1031_d_n17: f64 = (eq7_e1029 * locals.var_i_gcd_dn17);
        let eq7_e1031_d_n18: f64 = (eq7_e1029 * locals.var_i_gcd_dn18);
        let eq7_e1031_d_n19: f64 = (eq7_e1029 * locals.var_i_gcd_dn19);
        let eq7_e1031_d_n20: f64 = (eq7_e1029 * locals.var_i_gcd_dn20);
        (eq7_e1031, eq7_e1031_d_n5, eq7_e1031_d_n6, eq7_e1031_d_n7, eq7_e1031_d_n8, eq7_e1031_d_n12, eq7_e1031_d_n13, eq7_e1031_d_n14, eq7_e1031_d_n15, eq7_e1031_d_n16, eq7_e1031_d_n17, eq7_e1031_d_n18, eq7_e1031_d_n19, eq7_e1031_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1033;
        let eq7_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq7_node_derivatives: [f64; 13] = [eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20];
        let eq7_branch_derivative_indices: [usize; 0] = [];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            &eq7_node_derivative_indices,
            &eq7_node_derivatives,
            &eq7_branch_derivative_indices,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1036: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq8_e1038: f64 = (eq8_e1036 * p.p32);
        let eq8_e1040: f64 = (eq8_e1038 * locals.var_i_gb);
        let eq8_e1040_d_n5: f64 = (eq8_e1038 * locals.var_i_gb_dn5);
        let eq8_e1040_d_n6: f64 = (eq8_e1038 * locals.var_i_gb_dn6);
        let eq8_e1040_d_n7: f64 = (eq8_e1038 * locals.var_i_gb_dn7);
        let eq8_e1040_d_n8: f64 = (eq8_e1038 * locals.var_i_gb_dn8);
        let eq8_e1040_d_n12: f64 = (eq8_e1038 * locals.var_i_gb_dn12);
        let eq8_e1040_d_n13: f64 = (eq8_e1038 * locals.var_i_gb_dn13);
        let eq8_e1040_d_n14: f64 = (eq8_e1038 * locals.var_i_gb_dn14);
        let eq8_e1040_d_n15: f64 = (eq8_e1038 * locals.var_i_gb_dn15);
        let eq8_e1040_d_n16: f64 = (eq8_e1038 * locals.var_i_gb_dn16);
        let eq8_e1040_d_n17: f64 = (eq8_e1038 * locals.var_i_gb_dn17);
        let eq8_e1040_d_n18: f64 = (eq8_e1038 * locals.var_i_gb_dn18);
        let eq8_e1040_d_n19: f64 = (eq8_e1038 * locals.var_i_gb_dn19);
        let eq8_e1040_d_n20: f64 = (eq8_e1038 * locals.var_i_gb_dn20);
        let eq8_value: f64 = eq8_e1040;
        let eq8_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq8_node_derivatives: [f64; 13] = [eq8_e1040_d_n5, eq8_e1040_d_n6, eq8_e1040_d_n7, eq8_e1040_d_n8, eq8_e1040_d_n12, eq8_e1040_d_n13, eq8_e1040_d_n14, eq8_e1040_d_n15, eq8_e1040_d_n16, eq8_e1040_d_n17, eq8_e1040_d_n18, eq8_e1040_d_n19, eq8_e1040_d_n20];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1043: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq9_e1045: f64 = (eq9_e1043 * p.p32);
        let eq9_e1047: f64 = (eq9_e1045 * locals.var_igsov);
        let eq9_e1047_d_n5: f64 = (eq9_e1045 * locals.var_igsov_dn5);
        let eq9_e1047_d_n6: f64 = (eq9_e1045 * locals.var_igsov_dn6);
        let eq9_e1047_d_n7: f64 = (eq9_e1045 * locals.var_igsov_dn7);
        let eq9_e1047_d_n8: f64 = (eq9_e1045 * locals.var_igsov_dn8);
        let eq9_e1047_d_n12: f64 = (eq9_e1045 * locals.var_igsov_dn12);
        let eq9_e1047_d_n13: f64 = (eq9_e1045 * locals.var_igsov_dn13);
        let eq9_e1047_d_n14: f64 = (eq9_e1045 * locals.var_igsov_dn14);
        let eq9_e1047_d_n15: f64 = (eq9_e1045 * locals.var_igsov_dn15);
        let eq9_e1047_d_n16: f64 = (eq9_e1045 * locals.var_igsov_dn16);
        let eq9_e1047_d_n17: f64 = (eq9_e1045 * locals.var_igsov_dn17);
        let eq9_e1047_d_n18: f64 = (eq9_e1045 * locals.var_igsov_dn18);
        let eq9_e1047_d_n19: f64 = (eq9_e1045 * locals.var_igsov_dn19);
        let eq9_e1047_d_n20: f64 = (eq9_e1045 * locals.var_igsov_dn20);
        let eq9_value: f64 = eq9_e1047;
        let eq9_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq9_node_derivatives: [f64; 13] = [eq9_e1047_d_n5, eq9_e1047_d_n6, eq9_e1047_d_n7, eq9_e1047_d_n8, eq9_e1047_d_n12, eq9_e1047_d_n13, eq9_e1047_d_n14, eq9_e1047_d_n15, eq9_e1047_d_n16, eq9_e1047_d_n17, eq9_e1047_d_n18, eq9_e1047_d_n19, eq9_e1047_d_n20];
        let eq9_branch_derivative_indices: [usize; 0] = [];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivative_indices,
            &eq9_node_derivatives,
            &eq9_branch_derivative_indices,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1050: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq10_e1052: f64 = (eq10_e1050 * p.p32);
        let eq10_e1054: f64 = (eq10_e1052 * locals.var_igdov);
        let eq10_e1054_d_n5: f64 = (eq10_e1052 * locals.var_igdov_dn5);
        let eq10_e1054_d_n6: f64 = (eq10_e1052 * locals.var_igdov_dn6);
        let eq10_e1054_d_n7: f64 = (eq10_e1052 * locals.var_igdov_dn7);
        let eq10_e1054_d_n8: f64 = (eq10_e1052 * locals.var_igdov_dn8);
        let eq10_e1054_d_n12: f64 = (eq10_e1052 * locals.var_igdov_dn12);
        let eq10_e1054_d_n13: f64 = (eq10_e1052 * locals.var_igdov_dn13);
        let eq10_e1054_d_n14: f64 = (eq10_e1052 * locals.var_igdov_dn14);
        let eq10_e1054_d_n15: f64 = (eq10_e1052 * locals.var_igdov_dn15);
        let eq10_e1054_d_n16: f64 = (eq10_e1052 * locals.var_igdov_dn16);
        let eq10_e1054_d_n17: f64 = (eq10_e1052 * locals.var_igdov_dn17);
        let eq10_e1054_d_n18: f64 = (eq10_e1052 * locals.var_igdov_dn18);
        let eq10_e1054_d_n19: f64 = (eq10_e1052 * locals.var_igdov_dn19);
        let eq10_e1054_d_n20: f64 = (eq10_e1052 * locals.var_igdov_dn20);
        let eq10_value: f64 = eq10_e1054;
        let eq10_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq10_node_derivatives: [f64; 13] = [eq10_e1054_d_n5, eq10_e1054_d_n6, eq10_e1054_d_n7, eq10_e1054_d_n8, eq10_e1054_d_n12, eq10_e1054_d_n13, eq10_e1054_d_n14, eq10_e1054_d_n15, eq10_e1054_d_n16, eq10_e1054_d_n17, eq10_e1054_d_n18, eq10_e1054_d_n19, eq10_e1054_d_n20];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1057: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq11_e1059: f64 = (eq11_e1057 * p.p32);
        let eq11_e1061: f64 = (eq11_e1059 * locals.var_i_gisl);
        let eq11_e1061_d_n5: f64 = (eq11_e1059 * locals.var_i_gisl_dn5);
        let eq11_e1061_d_n6: f64 = (eq11_e1059 * locals.var_i_gisl_dn6);
        let eq11_e1061_d_n7: f64 = (eq11_e1059 * locals.var_i_gisl_dn7);
        let eq11_e1061_d_n8: f64 = (eq11_e1059 * locals.var_i_gisl_dn8);
        let eq11_e1061_d_n12: f64 = (eq11_e1059 * locals.var_i_gisl_dn12);
        let eq11_e1061_d_n13: f64 = (eq11_e1059 * locals.var_i_gisl_dn13);
        let eq11_e1061_d_n14: f64 = (eq11_e1059 * locals.var_i_gisl_dn14);
        let eq11_e1061_d_n15: f64 = (eq11_e1059 * locals.var_i_gisl_dn15);
        let eq11_e1061_d_n16: f64 = (eq11_e1059 * locals.var_i_gisl_dn16);
        let eq11_e1061_d_n17: f64 = (eq11_e1059 * locals.var_i_gisl_dn17);
        let eq11_e1061_d_n18: f64 = (eq11_e1059 * locals.var_i_gisl_dn18);
        let eq11_e1061_d_n19: f64 = (eq11_e1059 * locals.var_i_gisl_dn19);
        let eq11_e1061_d_n20: f64 = (eq11_e1059 * locals.var_i_gisl_dn20);
        let eq11_value: f64 = eq11_e1061;
        let eq11_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq11_node_derivatives: [f64; 13] = [eq11_e1061_d_n5, eq11_e1061_d_n6, eq11_e1061_d_n7, eq11_e1061_d_n8, eq11_e1061_d_n12, eq11_e1061_d_n13, eq11_e1061_d_n14, eq11_e1061_d_n15, eq11_e1061_d_n16, eq11_e1061_d_n17, eq11_e1061_d_n18, eq11_e1061_d_n19, eq11_e1061_d_n20];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1064: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq12_e1066: f64 = (eq12_e1064 * p.p32);
        let eq12_e1068: f64 = (eq12_e1066 * locals.var_i_gidl);
        let eq12_e1068_d_n5: f64 = (eq12_e1066 * locals.var_i_gidl_dn5);
        let eq12_e1068_d_n6: f64 = (eq12_e1066 * locals.var_i_gidl_dn6);
        let eq12_e1068_d_n7: f64 = (eq12_e1066 * locals.var_i_gidl_dn7);
        let eq12_e1068_d_n8: f64 = (eq12_e1066 * locals.var_i_gidl_dn8);
        let eq12_e1068_d_n12: f64 = (eq12_e1066 * locals.var_i_gidl_dn12);
        let eq12_e1068_d_n13: f64 = (eq12_e1066 * locals.var_i_gidl_dn13);
        let eq12_e1068_d_n14: f64 = (eq12_e1066 * locals.var_i_gidl_dn14);
        let eq12_e1068_d_n15: f64 = (eq12_e1066 * locals.var_i_gidl_dn15);
        let eq12_e1068_d_n16: f64 = (eq12_e1066 * locals.var_i_gidl_dn16);
        let eq12_e1068_d_n17: f64 = (eq12_e1066 * locals.var_i_gidl_dn17);
        let eq12_e1068_d_n18: f64 = (eq12_e1066 * locals.var_i_gidl_dn18);
        let eq12_e1068_d_n19: f64 = (eq12_e1066 * locals.var_i_gidl_dn19);
        let eq12_e1068_d_n20: f64 = (eq12_e1066 * locals.var_i_gidl_dn20);
        let eq12_value: f64 = eq12_e1068;
        let eq12_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq12_node_derivatives: [f64; 13] = [eq12_e1068_d_n5, eq12_e1068_d_n6, eq12_e1068_d_n7, eq12_e1068_d_n8, eq12_e1068_d_n12, eq12_e1068_d_n13, eq12_e1068_d_n14, eq12_e1068_d_n15, eq12_e1068_d_n16, eq12_e1068_d_n17, eq12_e1068_d_n18, eq12_e1068_d_n19, eq12_e1068_d_n20];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        idt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let eq39_e1275: f64 = (-locals.var_tnorm);
        let eq39_e1277: f64 = (eq39_e1275 * locals.var_fk1);
        let eq39_e1277_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn5));
        let eq39_e1277_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn6));
        let eq39_e1277_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn7));
        let eq39_e1277_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn8));
        let eq39_e1277_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn12));
        let eq39_e1277_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn13));
        let eq39_e1277_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn14));
        let eq39_e1277_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn15));
        let eq39_e1277_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn16));
        let eq39_e1277_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn17));
        let eq39_e1277_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn18));
        let eq39_e1277_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn19));
        let eq39_e1277_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk1) + (eq39_e1275 * locals.var_fk1_dn20));
        let eq39_e1279: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 0, eq39_e1277, locals.var_qp1_0);
        let eq39_e1280: f64 = (locals.var_vnorm_inv * eq39_e1279);
        let eq39_e1280_d_n5: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n5 * idt_scale));
        let eq39_e1280_d_n6: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n6 * idt_scale));
        let eq39_e1280_d_n7: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n7 * idt_scale));
        let eq39_e1280_d_n8: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n8 * idt_scale));
        let eq39_e1280_d_n12: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n12 * idt_scale));
        let eq39_e1280_d_n13: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n13 * idt_scale));
        let eq39_e1280_d_n14: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n14 * idt_scale));
        let eq39_e1280_d_n15: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n15 * idt_scale));
        let eq39_e1280_d_n16: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n16 * idt_scale));
        let eq39_e1280_d_n17: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n17 * idt_scale));
        let eq39_e1280_d_n18: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n18 * idt_scale));
        let eq39_e1280_d_n19: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n19 * idt_scale));
        let eq39_e1280_d_n20: f64 = (locals.var_vnorm_inv * (eq39_e1277_d_n20 * idt_scale));
        let eq39_value: f64 = eq39_e1280;
        let eq39_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq39_node_derivatives: [f64; 13] = [eq39_e1280_d_n5, eq39_e1280_d_n6, eq39_e1280_d_n7, eq39_e1280_d_n8, eq39_e1280_d_n12, eq39_e1280_d_n13, eq39_e1280_d_n14, eq39_e1280_d_n15, eq39_e1280_d_n16, eq39_e1280_d_n17, eq39_e1280_d_n18, eq39_e1280_d_n19, eq39_e1280_d_n20];
        let eq39_branch_derivative_indices: [usize; 0] = [];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            8,
            eq39_value,
            &eq39_node_derivative_indices,
            &eq39_node_derivatives,
            &eq39_branch_derivative_indices,
            &eq39_branch_derivatives,
        );
        let eq41_e1288: f64 = (-locals.var_tnorm);
        let eq41_e1290: f64 = (eq41_e1288 * locals.var_fk2);
        let eq41_e1290_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn5));
        let eq41_e1290_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn6));
        let eq41_e1290_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn7));
        let eq41_e1290_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn8));
        let eq41_e1290_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn12));
        let eq41_e1290_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn13));
        let eq41_e1290_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn14));
        let eq41_e1290_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn15));
        let eq41_e1290_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn16));
        let eq41_e1290_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn17));
        let eq41_e1290_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn18));
        let eq41_e1290_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn19));
        let eq41_e1290_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk2) + (eq41_e1288 * locals.var_fk2_dn20));
        let eq41_e1292: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 1, eq41_e1290, locals.var_qp2_0);
        let eq41_e1293: f64 = (locals.var_vnorm_inv * eq41_e1292);
        let eq41_e1293_d_n5: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n5 * idt_scale));
        let eq41_e1293_d_n6: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n6 * idt_scale));
        let eq41_e1293_d_n7: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n7 * idt_scale));
        let eq41_e1293_d_n8: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n8 * idt_scale));
        let eq41_e1293_d_n12: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n12 * idt_scale));
        let eq41_e1293_d_n13: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n13 * idt_scale));
        let eq41_e1293_d_n14: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n14 * idt_scale));
        let eq41_e1293_d_n15: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n15 * idt_scale));
        let eq41_e1293_d_n16: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n16 * idt_scale));
        let eq41_e1293_d_n17: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n17 * idt_scale));
        let eq41_e1293_d_n18: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n18 * idt_scale));
        let eq41_e1293_d_n19: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n19 * idt_scale));
        let eq41_e1293_d_n20: f64 = (locals.var_vnorm_inv * (eq41_e1290_d_n20 * idt_scale));
        let eq41_value: f64 = eq41_e1293;
        let eq41_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq41_node_derivatives: [f64; 13] = [eq41_e1293_d_n5, eq41_e1293_d_n6, eq41_e1293_d_n7, eq41_e1293_d_n8, eq41_e1293_d_n12, eq41_e1293_d_n13, eq41_e1293_d_n14, eq41_e1293_d_n15, eq41_e1293_d_n16, eq41_e1293_d_n17, eq41_e1293_d_n18, eq41_e1293_d_n19, eq41_e1293_d_n20];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            10,
            eq41_value,
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
        );
        let eq43_e1301: f64 = (-locals.var_tnorm);
        let eq43_e1303: f64 = (eq43_e1301 * locals.var_fk3);
        let eq43_e1303_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn5));
        let eq43_e1303_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn6));
        let eq43_e1303_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn7));
        let eq43_e1303_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn8));
        let eq43_e1303_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn12));
        let eq43_e1303_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn13));
        let eq43_e1303_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn14));
        let eq43_e1303_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn15));
        let eq43_e1303_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn16));
        let eq43_e1303_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn17));
        let eq43_e1303_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn18));
        let eq43_e1303_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn19));
        let eq43_e1303_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk3) + (eq43_e1301 * locals.var_fk3_dn20));
        let eq43_e1305: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 2, eq43_e1303, locals.var_qp3_0);
        let eq43_e1306: f64 = (locals.var_vnorm_inv * eq43_e1305);
        let eq43_e1306_d_n5: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n5 * idt_scale));
        let eq43_e1306_d_n6: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n6 * idt_scale));
        let eq43_e1306_d_n7: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n7 * idt_scale));
        let eq43_e1306_d_n8: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n8 * idt_scale));
        let eq43_e1306_d_n12: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n12 * idt_scale));
        let eq43_e1306_d_n13: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n13 * idt_scale));
        let eq43_e1306_d_n14: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n14 * idt_scale));
        let eq43_e1306_d_n15: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n15 * idt_scale));
        let eq43_e1306_d_n16: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n16 * idt_scale));
        let eq43_e1306_d_n17: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n17 * idt_scale));
        let eq43_e1306_d_n18: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n18 * idt_scale));
        let eq43_e1306_d_n19: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n19 * idt_scale));
        let eq43_e1306_d_n20: f64 = (locals.var_vnorm_inv * (eq43_e1303_d_n20 * idt_scale));
        let eq43_value: f64 = eq43_e1306;
        let eq43_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq43_node_derivatives: [f64; 13] = [eq43_e1306_d_n5, eq43_e1306_d_n6, eq43_e1306_d_n7, eq43_e1306_d_n8, eq43_e1306_d_n12, eq43_e1306_d_n13, eq43_e1306_d_n14, eq43_e1306_d_n15, eq43_e1306_d_n16, eq43_e1306_d_n17, eq43_e1306_d_n18, eq43_e1306_d_n19, eq43_e1306_d_n20];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            12,
            eq43_value,
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
        );
        let eq45_e1314: f64 = (-locals.var_tnorm);
        let eq45_e1316: f64 = (eq45_e1314 * locals.var_fk4);
        let eq45_e1316_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn5));
        let eq45_e1316_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn6));
        let eq45_e1316_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn7));
        let eq45_e1316_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn8));
        let eq45_e1316_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn12));
        let eq45_e1316_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn13));
        let eq45_e1316_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn14));
        let eq45_e1316_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn15));
        let eq45_e1316_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn16));
        let eq45_e1316_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn17));
        let eq45_e1316_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn18));
        let eq45_e1316_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn19));
        let eq45_e1316_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk4) + (eq45_e1314 * locals.var_fk4_dn20));
        let eq45_e1318: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 3, eq45_e1316, locals.var_qp4_0);
        let eq45_e1319: f64 = (locals.var_vnorm_inv * eq45_e1318);
        let eq45_e1319_d_n5: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n5 * idt_scale));
        let eq45_e1319_d_n6: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n6 * idt_scale));
        let eq45_e1319_d_n7: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n7 * idt_scale));
        let eq45_e1319_d_n8: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n8 * idt_scale));
        let eq45_e1319_d_n12: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n12 * idt_scale));
        let eq45_e1319_d_n13: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n13 * idt_scale));
        let eq45_e1319_d_n14: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n14 * idt_scale));
        let eq45_e1319_d_n15: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n15 * idt_scale));
        let eq45_e1319_d_n16: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n16 * idt_scale));
        let eq45_e1319_d_n17: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n17 * idt_scale));
        let eq45_e1319_d_n18: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n18 * idt_scale));
        let eq45_e1319_d_n19: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n19 * idt_scale));
        let eq45_e1319_d_n20: f64 = (locals.var_vnorm_inv * (eq45_e1316_d_n20 * idt_scale));
        let eq45_value: f64 = eq45_e1319;
        let eq45_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq45_node_derivatives: [f64; 13] = [eq45_e1319_d_n5, eq45_e1319_d_n6, eq45_e1319_d_n7, eq45_e1319_d_n8, eq45_e1319_d_n12, eq45_e1319_d_n13, eq45_e1319_d_n14, eq45_e1319_d_n15, eq45_e1319_d_n16, eq45_e1319_d_n17, eq45_e1319_d_n18, eq45_e1319_d_n19, eq45_e1319_d_n20];
        let eq45_branch_derivative_indices: [usize; 0] = [];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            14,
            eq45_value,
            &eq45_node_derivative_indices,
            &eq45_node_derivatives,
            &eq45_branch_derivative_indices,
            &eq45_branch_derivatives,
        );
        let eq47_e1327: f64 = (-locals.var_tnorm);
        let eq47_e1329: f64 = (eq47_e1327 * locals.var_fk5);
        let eq47_e1329_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn5));
        let eq47_e1329_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn6));
        let eq47_e1329_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn7));
        let eq47_e1329_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn8));
        let eq47_e1329_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn12));
        let eq47_e1329_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn13));
        let eq47_e1329_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn14));
        let eq47_e1329_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn15));
        let eq47_e1329_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn16));
        let eq47_e1329_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn17));
        let eq47_e1329_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn18));
        let eq47_e1329_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn19));
        let eq47_e1329_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk5) + (eq47_e1327 * locals.var_fk5_dn20));
        let eq47_e1331: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 4, eq47_e1329, locals.var_qp5_0);
        let eq47_e1332: f64 = (locals.var_vnorm_inv * eq47_e1331);
        let eq47_e1332_d_n5: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n5 * idt_scale));
        let eq47_e1332_d_n6: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n6 * idt_scale));
        let eq47_e1332_d_n7: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n7 * idt_scale));
        let eq47_e1332_d_n8: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n8 * idt_scale));
        let eq47_e1332_d_n12: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n12 * idt_scale));
        let eq47_e1332_d_n13: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n13 * idt_scale));
        let eq47_e1332_d_n14: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n14 * idt_scale));
        let eq47_e1332_d_n15: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n15 * idt_scale));
        let eq47_e1332_d_n16: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n16 * idt_scale));
        let eq47_e1332_d_n17: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n17 * idt_scale));
        let eq47_e1332_d_n18: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n18 * idt_scale));
        let eq47_e1332_d_n19: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n19 * idt_scale));
        let eq47_e1332_d_n20: f64 = (locals.var_vnorm_inv * (eq47_e1329_d_n20 * idt_scale));
        let eq47_value: f64 = eq47_e1332;
        let eq47_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq47_node_derivatives: [f64; 13] = [eq47_e1332_d_n5, eq47_e1332_d_n6, eq47_e1332_d_n7, eq47_e1332_d_n8, eq47_e1332_d_n12, eq47_e1332_d_n13, eq47_e1332_d_n14, eq47_e1332_d_n15, eq47_e1332_d_n16, eq47_e1332_d_n17, eq47_e1332_d_n18, eq47_e1332_d_n19, eq47_e1332_d_n20];
        let eq47_branch_derivative_indices: [usize; 0] = [];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            16,
            eq47_value,
            &eq47_node_derivative_indices,
            &eq47_node_derivatives,
            &eq47_branch_derivative_indices,
            &eq47_branch_derivatives,
        );
        let eq49_e1340: f64 = (-locals.var_tnorm);
        let eq49_e1342: f64 = (eq49_e1340 * locals.var_fk6);
        let eq49_e1342_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn5));
        let eq49_e1342_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn6));
        let eq49_e1342_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn7));
        let eq49_e1342_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn8));
        let eq49_e1342_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn12));
        let eq49_e1342_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn13));
        let eq49_e1342_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn14));
        let eq49_e1342_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn15));
        let eq49_e1342_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn16));
        let eq49_e1342_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn17));
        let eq49_e1342_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn18));
        let eq49_e1342_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn19));
        let eq49_e1342_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk6) + (eq49_e1340 * locals.var_fk6_dn20));
        let eq49_e1344: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 5, eq49_e1342, locals.var_qp6_0);
        let eq49_e1345: f64 = (locals.var_vnorm_inv * eq49_e1344);
        let eq49_e1345_d_n5: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n5 * idt_scale));
        let eq49_e1345_d_n6: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n6 * idt_scale));
        let eq49_e1345_d_n7: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n7 * idt_scale));
        let eq49_e1345_d_n8: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n8 * idt_scale));
        let eq49_e1345_d_n12: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n12 * idt_scale));
        let eq49_e1345_d_n13: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n13 * idt_scale));
        let eq49_e1345_d_n14: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n14 * idt_scale));
        let eq49_e1345_d_n15: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n15 * idt_scale));
        let eq49_e1345_d_n16: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n16 * idt_scale));
        let eq49_e1345_d_n17: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n17 * idt_scale));
        let eq49_e1345_d_n18: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n18 * idt_scale));
        let eq49_e1345_d_n19: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n19 * idt_scale));
        let eq49_e1345_d_n20: f64 = (locals.var_vnorm_inv * (eq49_e1342_d_n20 * idt_scale));
        let eq49_value: f64 = eq49_e1345;
        let eq49_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq49_node_derivatives: [f64; 13] = [eq49_e1345_d_n5, eq49_e1345_d_n6, eq49_e1345_d_n7, eq49_e1345_d_n8, eq49_e1345_d_n12, eq49_e1345_d_n13, eq49_e1345_d_n14, eq49_e1345_d_n15, eq49_e1345_d_n16, eq49_e1345_d_n17, eq49_e1345_d_n18, eq49_e1345_d_n19, eq49_e1345_d_n20];
        let eq49_branch_derivative_indices: [usize; 0] = [];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            18,
            eq49_value,
            &eq49_node_derivative_indices,
            &eq49_node_derivatives,
            &eq49_branch_derivative_indices,
            &eq49_branch_derivatives,
        );
        let eq51_e1353: f64 = (-locals.var_tnorm);
        let eq51_e1355: f64 = (eq51_e1353 * locals.var_fk7);
        let eq51_e1355_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn5));
        let eq51_e1355_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn6));
        let eq51_e1355_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn7));
        let eq51_e1355_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn8));
        let eq51_e1355_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn12));
        let eq51_e1355_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn13));
        let eq51_e1355_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn14));
        let eq51_e1355_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn15));
        let eq51_e1355_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn16));
        let eq51_e1355_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn17));
        let eq51_e1355_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn18));
        let eq51_e1355_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn19));
        let eq51_e1355_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk7) + (eq51_e1353 * locals.var_fk7_dn20));
        let eq51_e1357: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 6, eq51_e1355, locals.var_qp7_0);
        let eq51_e1358: f64 = (locals.var_vnorm_inv * eq51_e1357);
        let eq51_e1358_d_n5: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n5 * idt_scale));
        let eq51_e1358_d_n6: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n6 * idt_scale));
        let eq51_e1358_d_n7: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n7 * idt_scale));
        let eq51_e1358_d_n8: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n8 * idt_scale));
        let eq51_e1358_d_n12: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n12 * idt_scale));
        let eq51_e1358_d_n13: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n13 * idt_scale));
        let eq51_e1358_d_n14: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n14 * idt_scale));
        let eq51_e1358_d_n15: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n15 * idt_scale));
        let eq51_e1358_d_n16: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n16 * idt_scale));
        let eq51_e1358_d_n17: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n17 * idt_scale));
        let eq51_e1358_d_n18: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n18 * idt_scale));
        let eq51_e1358_d_n19: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n19 * idt_scale));
        let eq51_e1358_d_n20: f64 = (locals.var_vnorm_inv * (eq51_e1355_d_n20 * idt_scale));
        let eq51_value: f64 = eq51_e1358;
        let eq51_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq51_node_derivatives: [f64; 13] = [eq51_e1358_d_n5, eq51_e1358_d_n6, eq51_e1358_d_n7, eq51_e1358_d_n8, eq51_e1358_d_n12, eq51_e1358_d_n13, eq51_e1358_d_n14, eq51_e1358_d_n15, eq51_e1358_d_n16, eq51_e1358_d_n17, eq51_e1358_d_n18, eq51_e1358_d_n19, eq51_e1358_d_n20];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            20,
            eq51_value,
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
        );
        let eq53_e1366: f64 = (-locals.var_tnorm);
        let eq53_e1368: f64 = (eq53_e1366 * locals.var_fk8);
        let eq53_e1368_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn5));
        let eq53_e1368_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn6));
        let eq53_e1368_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn7));
        let eq53_e1368_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn8));
        let eq53_e1368_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn12));
        let eq53_e1368_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn13));
        let eq53_e1368_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn14));
        let eq53_e1368_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn15));
        let eq53_e1368_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn16));
        let eq53_e1368_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn17));
        let eq53_e1368_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn18));
        let eq53_e1368_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn19));
        let eq53_e1368_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk8) + (eq53_e1366 * locals.var_fk8_dn20));
        let eq53_e1370: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 7, eq53_e1368, locals.var_qp8_0);
        let eq53_e1371: f64 = (locals.var_vnorm_inv * eq53_e1370);
        let eq53_e1371_d_n5: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n5 * idt_scale));
        let eq53_e1371_d_n6: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n6 * idt_scale));
        let eq53_e1371_d_n7: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n7 * idt_scale));
        let eq53_e1371_d_n8: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n8 * idt_scale));
        let eq53_e1371_d_n12: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n12 * idt_scale));
        let eq53_e1371_d_n13: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n13 * idt_scale));
        let eq53_e1371_d_n14: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n14 * idt_scale));
        let eq53_e1371_d_n15: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n15 * idt_scale));
        let eq53_e1371_d_n16: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n16 * idt_scale));
        let eq53_e1371_d_n17: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n17 * idt_scale));
        let eq53_e1371_d_n18: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n18 * idt_scale));
        let eq53_e1371_d_n19: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n19 * idt_scale));
        let eq53_e1371_d_n20: f64 = (locals.var_vnorm_inv * (eq53_e1368_d_n20 * idt_scale));
        let eq53_value: f64 = eq53_e1371;
        let eq53_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq53_node_derivatives: [f64; 13] = [eq53_e1371_d_n5, eq53_e1371_d_n6, eq53_e1371_d_n7, eq53_e1371_d_n8, eq53_e1371_d_n12, eq53_e1371_d_n13, eq53_e1371_d_n14, eq53_e1371_d_n15, eq53_e1371_d_n16, eq53_e1371_d_n17, eq53_e1371_d_n18, eq53_e1371_d_n19, eq53_e1371_d_n20];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            22,
            eq53_value,
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
        );
        let eq55_e1379: f64 = (-locals.var_tnorm);
        let eq55_e1381: f64 = (eq55_e1379 * locals.var_fk9);
        let eq55_e1381_d_n5: f64 = (((-locals.var_tnorm_dn5) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn5));
        let eq55_e1381_d_n6: f64 = (((-locals.var_tnorm_dn6) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn6));
        let eq55_e1381_d_n7: f64 = (((-locals.var_tnorm_dn7) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn7));
        let eq55_e1381_d_n8: f64 = (((-locals.var_tnorm_dn8) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn8));
        let eq55_e1381_d_n12: f64 = (((-locals.var_tnorm_dn12) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn12));
        let eq55_e1381_d_n13: f64 = (((-locals.var_tnorm_dn13) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn13));
        let eq55_e1381_d_n14: f64 = (((-locals.var_tnorm_dn14) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn14));
        let eq55_e1381_d_n15: f64 = (((-locals.var_tnorm_dn15) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn15));
        let eq55_e1381_d_n16: f64 = (((-locals.var_tnorm_dn16) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn16));
        let eq55_e1381_d_n17: f64 = (((-locals.var_tnorm_dn17) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn17));
        let eq55_e1381_d_n18: f64 = (((-locals.var_tnorm_dn18) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn18));
        let eq55_e1381_d_n19: f64 = (((-locals.var_tnorm_dn19) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn19));
        let eq55_e1381_d_n20: f64 = (((-locals.var_tnorm_dn20) * locals.var_fk9) + (eq55_e1379 * locals.var_fk9_dn20));
        let eq55_e1383: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 8, eq55_e1381, locals.var_qp9_0);
        let eq55_e1384: f64 = (locals.var_vnorm_inv * eq55_e1383);
        let eq55_e1384_d_n5: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n5 * idt_scale));
        let eq55_e1384_d_n6: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n6 * idt_scale));
        let eq55_e1384_d_n7: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n7 * idt_scale));
        let eq55_e1384_d_n8: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n8 * idt_scale));
        let eq55_e1384_d_n12: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n12 * idt_scale));
        let eq55_e1384_d_n13: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n13 * idt_scale));
        let eq55_e1384_d_n14: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n14 * idt_scale));
        let eq55_e1384_d_n15: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n15 * idt_scale));
        let eq55_e1384_d_n16: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n16 * idt_scale));
        let eq55_e1384_d_n17: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n17 * idt_scale));
        let eq55_e1384_d_n18: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n18 * idt_scale));
        let eq55_e1384_d_n19: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n19 * idt_scale));
        let eq55_e1384_d_n20: f64 = (locals.var_vnorm_inv * (eq55_e1381_d_n20 * idt_scale));
        let eq55_value: f64 = eq55_e1384;
        let eq55_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq55_node_derivatives: [f64; 13] = [eq55_e1384_d_n5, eq55_e1384_d_n6, eq55_e1384_d_n7, eq55_e1384_d_n8, eq55_e1384_d_n12, eq55_e1384_d_n13, eq55_e1384_d_n14, eq55_e1384_d_n15, eq55_e1384_d_n16, eq55_e1384_d_n17, eq55_e1384_d_n18, eq55_e1384_d_n19, eq55_e1384_d_n20];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            24,
            eq55_value,
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
        );
        let eq56_e1387: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq56_e1389: f64 = (eq56_e1387 * p.p33);
        let eq56_e1391: f64 = (eq56_e1389 * locals.var_qg);
        let eq56_e1391_d_n5: f64 = (eq56_e1389 * locals.var_qg_dn5);
        let eq56_e1391_d_n6: f64 = (eq56_e1389 * locals.var_qg_dn6);
        let eq56_e1391_d_n7: f64 = (eq56_e1389 * locals.var_qg_dn7);
        let eq56_e1391_d_n8: f64 = (eq56_e1389 * locals.var_qg_dn8);
        let eq56_e1391_d_n12: f64 = (eq56_e1389 * locals.var_qg_dn12);
        let eq56_e1391_d_n13: f64 = (eq56_e1389 * locals.var_qg_dn13);
        let eq56_e1391_d_n14: f64 = (eq56_e1389 * locals.var_qg_dn14);
        let eq56_e1391_d_n15: f64 = (eq56_e1389 * locals.var_qg_dn15);
        let eq56_e1391_d_n16: f64 = (eq56_e1389 * locals.var_qg_dn16);
        let eq56_e1391_d_n17: f64 = (eq56_e1389 * locals.var_qg_dn17);
        let eq56_e1391_d_n18: f64 = (eq56_e1389 * locals.var_qg_dn18);
        let eq56_e1391_d_n19: f64 = (eq56_e1389 * locals.var_qg_dn19);
        let eq56_e1391_d_n20: f64 = (eq56_e1389 * locals.var_qg_dn20);
        let eq56_e1392: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq56_e1391);
        let eq56_value: f64 = eq56_e1392;
        let eq56_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq56_node_derivatives: [f64; 13] = [(eq56_e1391_d_n5 * ddt_scale), (eq56_e1391_d_n6 * ddt_scale), (eq56_e1391_d_n7 * ddt_scale), (eq56_e1391_d_n8 * ddt_scale), (eq56_e1391_d_n12 * ddt_scale), (eq56_e1391_d_n13 * ddt_scale), (eq56_e1391_d_n14 * ddt_scale), (eq56_e1391_d_n15 * ddt_scale), (eq56_e1391_d_n16 * ddt_scale), (eq56_e1391_d_n17 * ddt_scale), (eq56_e1391_d_n18 * ddt_scale), (eq56_e1391_d_n19 * ddt_scale), (eq56_e1391_d_n20 * ddt_scale)];
        let eq56_branch_derivative_indices: [usize; 0] = [];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivative_indices,
            &eq56_node_derivatives,
            &eq56_branch_derivative_indices,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let eq57_e1395: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq57_e1397: f64 = (eq57_e1395 * p.p33);
        let eq57_e1399: f64 = (eq57_e1397 * locals.var_qb);
        let eq57_e1399_d_n5: f64 = (eq57_e1397 * locals.var_qb_dn5);
        let eq57_e1399_d_n6: f64 = (eq57_e1397 * locals.var_qb_dn6);
        let eq57_e1399_d_n7: f64 = (eq57_e1397 * locals.var_qb_dn7);
        let eq57_e1399_d_n8: f64 = (eq57_e1397 * locals.var_qb_dn8);
        let eq57_e1399_d_n12: f64 = (eq57_e1397 * locals.var_qb_dn12);
        let eq57_e1399_d_n13: f64 = (eq57_e1397 * locals.var_qb_dn13);
        let eq57_e1399_d_n14: f64 = (eq57_e1397 * locals.var_qb_dn14);
        let eq57_e1399_d_n15: f64 = (eq57_e1397 * locals.var_qb_dn15);
        let eq57_e1399_d_n16: f64 = (eq57_e1397 * locals.var_qb_dn16);
        let eq57_e1399_d_n17: f64 = (eq57_e1397 * locals.var_qb_dn17);
        let eq57_e1399_d_n18: f64 = (eq57_e1397 * locals.var_qb_dn18);
        let eq57_e1399_d_n19: f64 = (eq57_e1397 * locals.var_qb_dn19);
        let eq57_e1399_d_n20: f64 = (eq57_e1397 * locals.var_qb_dn20);
        let eq57_e1400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq57_e1399);
        let eq57_value: f64 = eq57_e1400;
        let eq57_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq57_node_derivatives: [f64; 13] = [(eq57_e1399_d_n5 * ddt_scale), (eq57_e1399_d_n6 * ddt_scale), (eq57_e1399_d_n7 * ddt_scale), (eq57_e1399_d_n8 * ddt_scale), (eq57_e1399_d_n12 * ddt_scale), (eq57_e1399_d_n13 * ddt_scale), (eq57_e1399_d_n14 * ddt_scale), (eq57_e1399_d_n15 * ddt_scale), (eq57_e1399_d_n16 * ddt_scale), (eq57_e1399_d_n17 * ddt_scale), (eq57_e1399_d_n18 * ddt_scale), (eq57_e1399_d_n19 * ddt_scale), (eq57_e1399_d_n20 * ddt_scale)];
        let eq57_branch_derivative_indices: [usize; 0] = [];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq57_value),
            &eq57_node_derivative_indices,
            &eq57_node_derivatives,
            &eq57_branch_derivative_indices,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let eq58_e1403: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq58_e1405: f64 = (eq58_e1403 * p.p33);
        let eq58_e1407: f64 = (eq58_e1405 * locals.var_qd);
        let eq58_e1407_d_n5: f64 = (eq58_e1405 * locals.var_qd_dn5);
        let eq58_e1407_d_n6: f64 = (eq58_e1405 * locals.var_qd_dn6);
        let eq58_e1407_d_n7: f64 = (eq58_e1405 * locals.var_qd_dn7);
        let eq58_e1407_d_n8: f64 = (eq58_e1405 * locals.var_qd_dn8);
        let eq58_e1407_d_n12: f64 = (eq58_e1405 * locals.var_qd_dn12);
        let eq58_e1407_d_n13: f64 = (eq58_e1405 * locals.var_qd_dn13);
        let eq58_e1407_d_n14: f64 = (eq58_e1405 * locals.var_qd_dn14);
        let eq58_e1407_d_n15: f64 = (eq58_e1405 * locals.var_qd_dn15);
        let eq58_e1407_d_n16: f64 = (eq58_e1405 * locals.var_qd_dn16);
        let eq58_e1407_d_n17: f64 = (eq58_e1405 * locals.var_qd_dn17);
        let eq58_e1407_d_n18: f64 = (eq58_e1405 * locals.var_qd_dn18);
        let eq58_e1407_d_n19: f64 = (eq58_e1405 * locals.var_qd_dn19);
        let eq58_e1407_d_n20: f64 = (eq58_e1405 * locals.var_qd_dn20);
        let eq58_e1408: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq58_e1407);
        let eq58_value: f64 = eq58_e1408;
        let eq58_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq58_node_derivatives: [f64; 13] = [(eq58_e1407_d_n5 * ddt_scale), (eq58_e1407_d_n6 * ddt_scale), (eq58_e1407_d_n7 * ddt_scale), (eq58_e1407_d_n8 * ddt_scale), (eq58_e1407_d_n12 * ddt_scale), (eq58_e1407_d_n13 * ddt_scale), (eq58_e1407_d_n14 * ddt_scale), (eq58_e1407_d_n15 * ddt_scale), (eq58_e1407_d_n16 * ddt_scale), (eq58_e1407_d_n17 * ddt_scale), (eq58_e1407_d_n18 * ddt_scale), (eq58_e1407_d_n19 * ddt_scale), (eq58_e1407_d_n20 * ddt_scale)];
        let eq58_branch_derivative_indices: [usize; 0] = [];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq58_value),
            &eq58_node_derivative_indices,
            &eq58_node_derivatives,
            &eq58_branch_derivative_indices,
            &eq58_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq61_e1427: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq61_e1429: f64 = (eq61_e1427 * p.p33);
        let eq61_e1431: f64 = (eq61_e1429 * locals.var_qgb_ov);
        let eq61_e1431_d_n5: f64 = (eq61_e1429 * locals.var_qgb_ov_dn5);
        let eq61_e1431_d_n6: f64 = (eq61_e1429 * locals.var_qgb_ov_dn6);
        let eq61_e1431_d_n7: f64 = (eq61_e1429 * locals.var_qgb_ov_dn7);
        let eq61_e1431_d_n8: f64 = (eq61_e1429 * locals.var_qgb_ov_dn8);
        let eq61_e1431_d_n12: f64 = (eq61_e1429 * locals.var_qgb_ov_dn12);
        let eq61_e1431_d_n13: f64 = (eq61_e1429 * locals.var_qgb_ov_dn13);
        let eq61_e1431_d_n14: f64 = (eq61_e1429 * locals.var_qgb_ov_dn14);
        let eq61_e1431_d_n15: f64 = (eq61_e1429 * locals.var_qgb_ov_dn15);
        let eq61_e1431_d_n16: f64 = (eq61_e1429 * locals.var_qgb_ov_dn16);
        let eq61_e1431_d_n17: f64 = (eq61_e1429 * locals.var_qgb_ov_dn17);
        let eq61_e1431_d_n18: f64 = (eq61_e1429 * locals.var_qgb_ov_dn18);
        let eq61_e1431_d_n19: f64 = (eq61_e1429 * locals.var_qgb_ov_dn19);
        let eq61_e1431_d_n20: f64 = (eq61_e1429 * locals.var_qgb_ov_dn20);
        let eq61_e1432: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq61_e1431);
        let eq61_value: f64 = eq61_e1432;
        let eq61_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq61_node_derivatives: [f64; 13] = [(eq61_e1431_d_n5 * ddt_scale), (eq61_e1431_d_n6 * ddt_scale), (eq61_e1431_d_n7 * ddt_scale), (eq61_e1431_d_n8 * ddt_scale), (eq61_e1431_d_n12 * ddt_scale), (eq61_e1431_d_n13 * ddt_scale), (eq61_e1431_d_n14 * ddt_scale), (eq61_e1431_d_n15 * ddt_scale), (eq61_e1431_d_n16 * ddt_scale), (eq61_e1431_d_n17 * ddt_scale), (eq61_e1431_d_n18 * ddt_scale), (eq61_e1431_d_n19 * ddt_scale), (eq61_e1431_d_n20 * ddt_scale)];
        let eq61_branch_derivative_indices: [usize; 0] = [];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq61_value),
            &eq61_node_derivative_indices,
            &eq61_node_derivatives,
            &eq61_branch_derivative_indices,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_mig;
        let eq65_e1456: f64 = ((nv4 - 0.0) * __rspice_inv_cse_0);
        let eq65_e1456_d_n4: f64 = (1.0 * __rspice_inv_cse_0);
        let eq65_e1456_d_n5: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn5) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n6: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n7: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n8: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n12: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn12) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n13: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn13) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n14: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn14) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n15: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn15) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n16: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn16) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n17: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn17) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n18: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn18) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n19: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn19) / (locals.var_mig * locals.var_mig)));
        let eq65_e1456_d_n20: f64 = (-(((nv4 - 0.0) * locals.var_mig_dn20) / (locals.var_mig * locals.var_mig)));
        let eq65_value: f64 = eq65_e1456;
        let eq65_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq65_node_derivatives: [f64; 14] = [eq65_e1456_d_n4, eq65_e1456_d_n5, eq65_e1456_d_n6, eq65_e1456_d_n7, eq65_e1456_d_n8, eq65_e1456_d_n12, eq65_e1456_d_n13, eq65_e1456_d_n14, eq65_e1456_d_n15, eq65_e1456_d_n16, eq65_e1456_d_n17, eq65_e1456_d_n18, eq65_e1456_d_n19, eq65_e1456_d_n20];
        let eq65_branch_derivative_indices: [usize; 0] = [];
        let eq65_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq65_value),
            &eq65_node_derivative_indices,
            &eq65_node_derivatives,
            &eq65_branch_derivative_indices,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let eq66_e1459: f64 = (locals.var_cgeff * (nv4 - 0.0));
        let eq66_e1459_d_n5: f64 = (locals.var_cgeff_dn5 * (nv4 - 0.0));
        let eq66_e1459_d_n6: f64 = (locals.var_cgeff_dn6 * (nv4 - 0.0));
        let eq66_e1459_d_n7: f64 = (locals.var_cgeff_dn7 * (nv4 - 0.0));
        let eq66_e1459_d_n8: f64 = (locals.var_cgeff_dn8 * (nv4 - 0.0));
        let eq66_e1459_d_n12: f64 = (locals.var_cgeff_dn12 * (nv4 - 0.0));
        let eq66_e1459_d_n13: f64 = (locals.var_cgeff_dn13 * (nv4 - 0.0));
        let eq66_e1459_d_n14: f64 = (locals.var_cgeff_dn14 * (nv4 - 0.0));
        let eq66_e1459_d_n15: f64 = (locals.var_cgeff_dn15 * (nv4 - 0.0));
        let eq66_e1459_d_n16: f64 = (locals.var_cgeff_dn16 * (nv4 - 0.0));
        let eq66_e1459_d_n17: f64 = (locals.var_cgeff_dn17 * (nv4 - 0.0));
        let eq66_e1459_d_n18: f64 = (locals.var_cgeff_dn18 * (nv4 - 0.0));
        let eq66_e1459_d_n19: f64 = (locals.var_cgeff_dn19 * (nv4 - 0.0));
        let eq66_e1459_d_n20: f64 = (locals.var_cgeff_dn20 * (nv4 - 0.0));
        let eq66_e1460: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq66_e1459);
        let eq66_value: f64 = eq66_e1460;
        let eq66_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq66_node_derivatives: [f64; 14] = [(locals.var_cgeff * ddt_scale), (eq66_e1459_d_n5 * ddt_scale), (eq66_e1459_d_n6 * ddt_scale), (eq66_e1459_d_n7 * ddt_scale), (eq66_e1459_d_n8 * ddt_scale), (eq66_e1459_d_n12 * ddt_scale), (eq66_e1459_d_n13 * ddt_scale), (eq66_e1459_d_n14 * ddt_scale), (eq66_e1459_d_n15 * ddt_scale), (eq66_e1459_d_n16 * ddt_scale), (eq66_e1459_d_n17 * ddt_scale), (eq66_e1459_d_n18 * ddt_scale), (eq66_e1459_d_n19 * ddt_scale), (eq66_e1459_d_n20 * ddt_scale)];
        let eq66_branch_derivative_indices: [usize; 0] = [];
        let eq66_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq66_value),
            &eq66_node_derivative_indices,
            &eq66_node_derivatives,
            &eq66_branch_derivative_indices,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let eq67_e1463: f64 = (locals.var_mult_inst * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * locals.var_cgeff);
        let eq67_e1468_d_n5: f64 = (eq67_e1466 * locals.var_cgeff_dn5);
        let eq67_e1468_d_n6: f64 = (eq67_e1466 * locals.var_cgeff_dn6);
        let eq67_e1468_d_n7: f64 = (eq67_e1466 * locals.var_cgeff_dn7);
        let eq67_e1468_d_n8: f64 = (eq67_e1466 * locals.var_cgeff_dn8);
        let eq67_e1468_d_n12: f64 = (eq67_e1466 * locals.var_cgeff_dn12);
        let eq67_e1468_d_n13: f64 = (eq67_e1466 * locals.var_cgeff_dn13);
        let eq67_e1468_d_n14: f64 = (eq67_e1466 * locals.var_cgeff_dn14);
        let eq67_e1468_d_n15: f64 = (eq67_e1466 * locals.var_cgeff_dn15);
        let eq67_e1468_d_n16: f64 = (eq67_e1466 * locals.var_cgeff_dn16);
        let eq67_e1468_d_n17: f64 = (eq67_e1466 * locals.var_cgeff_dn17);
        let eq67_e1468_d_n18: f64 = (eq67_e1466 * locals.var_cgeff_dn18);
        let eq67_e1468_d_n19: f64 = (eq67_e1466 * locals.var_cgeff_dn19);
        let eq67_e1468_d_n20: f64 = (eq67_e1466 * locals.var_cgeff_dn20);
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n5: f64 = (eq67_e1468_d_n5 * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = (eq67_e1468_d_n6 * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = (eq67_e1468_d_n7 * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = (eq67_e1468_d_n8 * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = (eq67_e1468_d_n12 * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = (eq67_e1468_d_n13 * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = (eq67_e1468_d_n14 * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = (eq67_e1468_d_n15 * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = (eq67_e1468_d_n16 * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = (eq67_e1468_d_n17 * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = (eq67_e1468_d_n18 * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = (eq67_e1468_d_n19 * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = (eq67_e1468_d_n20 * (nv4 - 0.0));
        let eq67_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq67_e1470);
        let eq67_e1472: f64 = (-eq67_e1471);
        let eq67_e1472_d_n4: f64 = (-(eq67_e1468 * ddt_scale));
        let eq67_e1472_d_n5: f64 = (-(eq67_e1470_d_n5 * ddt_scale));
        let eq67_e1472_d_n6: f64 = (-(eq67_e1470_d_n6 * ddt_scale));
        let eq67_e1472_d_n7: f64 = (-(eq67_e1470_d_n7 * ddt_scale));
        let eq67_e1472_d_n8: f64 = (-(eq67_e1470_d_n8 * ddt_scale));
        let eq67_e1472_d_n12: f64 = (-(eq67_e1470_d_n12 * ddt_scale));
        let eq67_e1472_d_n13: f64 = (-(eq67_e1470_d_n13 * ddt_scale));
        let eq67_e1472_d_n14: f64 = (-(eq67_e1470_d_n14 * ddt_scale));
        let eq67_e1472_d_n15: f64 = (-(eq67_e1470_d_n15 * ddt_scale));
        let eq67_e1472_d_n16: f64 = (-(eq67_e1470_d_n16 * ddt_scale));
        let eq67_e1472_d_n17: f64 = (-(eq67_e1470_d_n17 * ddt_scale));
        let eq67_e1472_d_n18: f64 = (-(eq67_e1470_d_n18 * ddt_scale));
        let eq67_e1472_d_n19: f64 = (-(eq67_e1470_d_n19 * ddt_scale));
        let eq67_e1472_d_n20: f64 = (-(eq67_e1470_d_n20 * ddt_scale));
        let eq67_value: f64 = eq67_e1472;
        let eq67_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq67_node_derivatives: [f64; 14] = [eq67_e1472_d_n4, eq67_e1472_d_n5, eq67_e1472_d_n6, eq67_e1472_d_n7, eq67_e1472_d_n8, eq67_e1472_d_n12, eq67_e1472_d_n13, eq67_e1472_d_n14, eq67_e1472_d_n15, eq67_e1472_d_n16, eq67_e1472_d_n17, eq67_e1472_d_n18, eq67_e1472_d_n19, eq67_e1472_d_n20];
        let eq67_branch_derivative_indices: [usize; 0] = [];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq67_value),
            &eq67_node_derivative_indices,
            &eq67_node_derivatives,
            &eq67_branch_derivative_indices,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let eq68_e1475: f64 = (locals.var_mult_inst * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * locals.var_cgeff);
        let eq68_e1480_d_n5: f64 = (eq68_e1478 * locals.var_cgeff_dn5);
        let eq68_e1480_d_n6: f64 = (eq68_e1478 * locals.var_cgeff_dn6);
        let eq68_e1480_d_n7: f64 = (eq68_e1478 * locals.var_cgeff_dn7);
        let eq68_e1480_d_n8: f64 = (eq68_e1478 * locals.var_cgeff_dn8);
        let eq68_e1480_d_n12: f64 = (eq68_e1478 * locals.var_cgeff_dn12);
        let eq68_e1480_d_n13: f64 = (eq68_e1478 * locals.var_cgeff_dn13);
        let eq68_e1480_d_n14: f64 = (eq68_e1478 * locals.var_cgeff_dn14);
        let eq68_e1480_d_n15: f64 = (eq68_e1478 * locals.var_cgeff_dn15);
        let eq68_e1480_d_n16: f64 = (eq68_e1478 * locals.var_cgeff_dn16);
        let eq68_e1480_d_n17: f64 = (eq68_e1478 * locals.var_cgeff_dn17);
        let eq68_e1480_d_n18: f64 = (eq68_e1478 * locals.var_cgeff_dn18);
        let eq68_e1480_d_n19: f64 = (eq68_e1478 * locals.var_cgeff_dn19);
        let eq68_e1480_d_n20: f64 = (eq68_e1478 * locals.var_cgeff_dn20);
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq68_e1482);
        let eq68_e1484: f64 = (-eq68_e1483);
        let eq68_e1484_d_n4: f64 = (-(eq68_e1480 * ddt_scale));
        let eq68_e1484_d_n5: f64 = (-(eq68_e1482_d_n5 * ddt_scale));
        let eq68_e1484_d_n6: f64 = (-(eq68_e1482_d_n6 * ddt_scale));
        let eq68_e1484_d_n7: f64 = (-(eq68_e1482_d_n7 * ddt_scale));
        let eq68_e1484_d_n8: f64 = (-(eq68_e1482_d_n8 * ddt_scale));
        let eq68_e1484_d_n12: f64 = (-(eq68_e1482_d_n12 * ddt_scale));
        let eq68_e1484_d_n13: f64 = (-(eq68_e1482_d_n13 * ddt_scale));
        let eq68_e1484_d_n14: f64 = (-(eq68_e1482_d_n14 * ddt_scale));
        let eq68_e1484_d_n15: f64 = (-(eq68_e1482_d_n15 * ddt_scale));
        let eq68_e1484_d_n16: f64 = (-(eq68_e1482_d_n16 * ddt_scale));
        let eq68_e1484_d_n17: f64 = (-(eq68_e1482_d_n17 * ddt_scale));
        let eq68_e1484_d_n18: f64 = (-(eq68_e1482_d_n18 * ddt_scale));
        let eq68_e1484_d_n19: f64 = (-(eq68_e1482_d_n19 * ddt_scale));
        let eq68_e1484_d_n20: f64 = (-(eq68_e1482_d_n20 * ddt_scale));
        let eq68_value: f64 = eq68_e1484;
        let eq68_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq68_node_derivatives: [f64; 14] = [eq68_e1484_d_n4, eq68_e1484_d_n5, eq68_e1484_d_n6, eq68_e1484_d_n7, eq68_e1484_d_n8, eq68_e1484_d_n12, eq68_e1484_d_n13, eq68_e1484_d_n14, eq68_e1484_d_n15, eq68_e1484_d_n16, eq68_e1484_d_n17, eq68_e1484_d_n18, eq68_e1484_d_n19, eq68_e1484_d_n20];
        let eq68_branch_derivative_indices: [usize; 0] = [];
        let eq68_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq68_value),
            &eq68_node_derivative_indices,
            &eq68_node_derivatives,
            &eq68_branch_derivative_indices,
            &eq68_branch_derivatives,
            multiplicity,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq56_e1387: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq56_e1389: f64 = (eq56_e1387 * p.p33);
        let eq56_e1391: f64 = (eq56_e1389 * locals.var_qg);
        let eq56_e1391_d_n5: f64 = (eq56_e1389 * locals.var_qg_dn5);
        let eq56_e1391_d_n6: f64 = (eq56_e1389 * locals.var_qg_dn6);
        let eq56_e1391_d_n7: f64 = (eq56_e1389 * locals.var_qg_dn7);
        let eq56_e1391_d_n8: f64 = (eq56_e1389 * locals.var_qg_dn8);
        let eq56_e1391_d_n12: f64 = (eq56_e1389 * locals.var_qg_dn12);
        let eq56_e1391_d_n13: f64 = (eq56_e1389 * locals.var_qg_dn13);
        let eq56_e1391_d_n14: f64 = (eq56_e1389 * locals.var_qg_dn14);
        let eq56_e1391_d_n15: f64 = (eq56_e1389 * locals.var_qg_dn15);
        let eq56_e1391_d_n16: f64 = (eq56_e1389 * locals.var_qg_dn16);
        let eq56_e1391_d_n17: f64 = (eq56_e1389 * locals.var_qg_dn17);
        let eq56_e1391_d_n18: f64 = (eq56_e1389 * locals.var_qg_dn18);
        let eq56_e1391_d_n19: f64 = (eq56_e1389 * locals.var_qg_dn19);
        let eq56_e1391_d_n20: f64 = (eq56_e1389 * locals.var_qg_dn20);
        let eq56_e1392_q: f64 = eq56_e1391;
        let eq56_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, 0.0, eq56_e1391_d_n5, eq56_e1391_d_n6, eq56_e1391_d_n7, eq56_e1391_d_n8, 0.0, 0.0, 0.0, eq56_e1391_d_n12, eq56_e1391_d_n13, eq56_e1391_d_n14, eq56_e1391_d_n15, eq56_e1391_d_n16, eq56_e1391_d_n17, eq56_e1391_d_n18, eq56_e1391_d_n19, eq56_e1391_d_n20];
        let eq56_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let eq57_e1395: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq57_e1397: f64 = (eq57_e1395 * p.p33);
        let eq57_e1399: f64 = (eq57_e1397 * locals.var_qb);
        let eq57_e1399_d_n5: f64 = (eq57_e1397 * locals.var_qb_dn5);
        let eq57_e1399_d_n6: f64 = (eq57_e1397 * locals.var_qb_dn6);
        let eq57_e1399_d_n7: f64 = (eq57_e1397 * locals.var_qb_dn7);
        let eq57_e1399_d_n8: f64 = (eq57_e1397 * locals.var_qb_dn8);
        let eq57_e1399_d_n12: f64 = (eq57_e1397 * locals.var_qb_dn12);
        let eq57_e1399_d_n13: f64 = (eq57_e1397 * locals.var_qb_dn13);
        let eq57_e1399_d_n14: f64 = (eq57_e1397 * locals.var_qb_dn14);
        let eq57_e1399_d_n15: f64 = (eq57_e1397 * locals.var_qb_dn15);
        let eq57_e1399_d_n16: f64 = (eq57_e1397 * locals.var_qb_dn16);
        let eq57_e1399_d_n17: f64 = (eq57_e1397 * locals.var_qb_dn17);
        let eq57_e1399_d_n18: f64 = (eq57_e1397 * locals.var_qb_dn18);
        let eq57_e1399_d_n19: f64 = (eq57_e1397 * locals.var_qb_dn19);
        let eq57_e1399_d_n20: f64 = (eq57_e1397 * locals.var_qb_dn20);
        let eq57_e1400_q: f64 = eq57_e1399;
        let eq57_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, 0.0, eq57_e1399_d_n5, eq57_e1399_d_n6, eq57_e1399_d_n7, eq57_e1399_d_n8, 0.0, 0.0, 0.0, eq57_e1399_d_n12, eq57_e1399_d_n13, eq57_e1399_d_n14, eq57_e1399_d_n15, eq57_e1399_d_n16, eq57_e1399_d_n17, eq57_e1399_d_n18, eq57_e1399_d_n19, eq57_e1399_d_n20];
        let eq57_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq57_reactive_node_derivatives,
            branches,
            &eq57_reactive_branch_derivatives,
            multiplicity,
        );
        let eq58_e1403: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq58_e1405: f64 = (eq58_e1403 * p.p33);
        let eq58_e1407: f64 = (eq58_e1405 * locals.var_qd);
        let eq58_e1407_d_n5: f64 = (eq58_e1405 * locals.var_qd_dn5);
        let eq58_e1407_d_n6: f64 = (eq58_e1405 * locals.var_qd_dn6);
        let eq58_e1407_d_n7: f64 = (eq58_e1405 * locals.var_qd_dn7);
        let eq58_e1407_d_n8: f64 = (eq58_e1405 * locals.var_qd_dn8);
        let eq58_e1407_d_n12: f64 = (eq58_e1405 * locals.var_qd_dn12);
        let eq58_e1407_d_n13: f64 = (eq58_e1405 * locals.var_qd_dn13);
        let eq58_e1407_d_n14: f64 = (eq58_e1405 * locals.var_qd_dn14);
        let eq58_e1407_d_n15: f64 = (eq58_e1405 * locals.var_qd_dn15);
        let eq58_e1407_d_n16: f64 = (eq58_e1405 * locals.var_qd_dn16);
        let eq58_e1407_d_n17: f64 = (eq58_e1405 * locals.var_qd_dn17);
        let eq58_e1407_d_n18: f64 = (eq58_e1405 * locals.var_qd_dn18);
        let eq58_e1407_d_n19: f64 = (eq58_e1405 * locals.var_qd_dn19);
        let eq58_e1407_d_n20: f64 = (eq58_e1405 * locals.var_qd_dn20);
        let eq58_e1408_q: f64 = eq58_e1407;
        let eq58_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, 0.0, eq58_e1407_d_n5, eq58_e1407_d_n6, eq58_e1407_d_n7, eq58_e1407_d_n8, 0.0, 0.0, 0.0, eq58_e1407_d_n12, eq58_e1407_d_n13, eq58_e1407_d_n14, eq58_e1407_d_n15, eq58_e1407_d_n16, eq58_e1407_d_n17, eq58_e1407_d_n18, eq58_e1407_d_n19, eq58_e1407_d_n20];
        let eq58_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq58_reactive_node_derivatives,
            branches,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
        let eq61_e1427: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq61_e1429: f64 = (eq61_e1427 * p.p33);
        let eq61_e1431: f64 = (eq61_e1429 * locals.var_qgb_ov);
        let eq61_e1431_d_n5: f64 = (eq61_e1429 * locals.var_qgb_ov_dn5);
        let eq61_e1431_d_n6: f64 = (eq61_e1429 * locals.var_qgb_ov_dn6);
        let eq61_e1431_d_n7: f64 = (eq61_e1429 * locals.var_qgb_ov_dn7);
        let eq61_e1431_d_n8: f64 = (eq61_e1429 * locals.var_qgb_ov_dn8);
        let eq61_e1431_d_n12: f64 = (eq61_e1429 * locals.var_qgb_ov_dn12);
        let eq61_e1431_d_n13: f64 = (eq61_e1429 * locals.var_qgb_ov_dn13);
        let eq61_e1431_d_n14: f64 = (eq61_e1429 * locals.var_qgb_ov_dn14);
        let eq61_e1431_d_n15: f64 = (eq61_e1429 * locals.var_qgb_ov_dn15);
        let eq61_e1431_d_n16: f64 = (eq61_e1429 * locals.var_qgb_ov_dn16);
        let eq61_e1431_d_n17: f64 = (eq61_e1429 * locals.var_qgb_ov_dn17);
        let eq61_e1431_d_n18: f64 = (eq61_e1429 * locals.var_qgb_ov_dn18);
        let eq61_e1431_d_n19: f64 = (eq61_e1429 * locals.var_qgb_ov_dn19);
        let eq61_e1431_d_n20: f64 = (eq61_e1429 * locals.var_qgb_ov_dn20);
        let eq61_e1432_q: f64 = eq61_e1431;
        let eq61_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, 0.0, eq61_e1431_d_n5, eq61_e1431_d_n6, eq61_e1431_d_n7, eq61_e1431_d_n8, 0.0, 0.0, 0.0, eq61_e1431_d_n12, eq61_e1431_d_n13, eq61_e1431_d_n14, eq61_e1431_d_n15, eq61_e1431_d_n16, eq61_e1431_d_n17, eq61_e1431_d_n18, eq61_e1431_d_n19, eq61_e1431_d_n20];
        let eq61_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
        let eq66_e1459: f64 = (locals.var_cgeff * (nv4 - 0.0));
        let eq66_e1459_d_n5: f64 = (locals.var_cgeff_dn5 * (nv4 - 0.0));
        let eq66_e1459_d_n6: f64 = (locals.var_cgeff_dn6 * (nv4 - 0.0));
        let eq66_e1459_d_n7: f64 = (locals.var_cgeff_dn7 * (nv4 - 0.0));
        let eq66_e1459_d_n8: f64 = (locals.var_cgeff_dn8 * (nv4 - 0.0));
        let eq66_e1459_d_n12: f64 = (locals.var_cgeff_dn12 * (nv4 - 0.0));
        let eq66_e1459_d_n13: f64 = (locals.var_cgeff_dn13 * (nv4 - 0.0));
        let eq66_e1459_d_n14: f64 = (locals.var_cgeff_dn14 * (nv4 - 0.0));
        let eq66_e1459_d_n15: f64 = (locals.var_cgeff_dn15 * (nv4 - 0.0));
        let eq66_e1459_d_n16: f64 = (locals.var_cgeff_dn16 * (nv4 - 0.0));
        let eq66_e1459_d_n17: f64 = (locals.var_cgeff_dn17 * (nv4 - 0.0));
        let eq66_e1459_d_n18: f64 = (locals.var_cgeff_dn18 * (nv4 - 0.0));
        let eq66_e1459_d_n19: f64 = (locals.var_cgeff_dn19 * (nv4 - 0.0));
        let eq66_e1459_d_n20: f64 = (locals.var_cgeff_dn20 * (nv4 - 0.0));
        let eq66_e1460_q: f64 = eq66_e1459;
        let eq66_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, locals.var_cgeff, eq66_e1459_d_n5, eq66_e1459_d_n6, eq66_e1459_d_n7, eq66_e1459_d_n8, 0.0, 0.0, 0.0, eq66_e1459_d_n12, eq66_e1459_d_n13, eq66_e1459_d_n14, eq66_e1459_d_n15, eq66_e1459_d_n16, eq66_e1459_d_n17, eq66_e1459_d_n18, eq66_e1459_d_n19, eq66_e1459_d_n20];
        let eq66_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq66_reactive_node_derivatives,
            branches,
            &eq66_reactive_branch_derivatives,
            multiplicity,
        );
        let eq67_e1463: f64 = (locals.var_mult_inst * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * locals.var_cgeff);
        let eq67_e1468_d_n5: f64 = (eq67_e1466 * locals.var_cgeff_dn5);
        let eq67_e1468_d_n6: f64 = (eq67_e1466 * locals.var_cgeff_dn6);
        let eq67_e1468_d_n7: f64 = (eq67_e1466 * locals.var_cgeff_dn7);
        let eq67_e1468_d_n8: f64 = (eq67_e1466 * locals.var_cgeff_dn8);
        let eq67_e1468_d_n12: f64 = (eq67_e1466 * locals.var_cgeff_dn12);
        let eq67_e1468_d_n13: f64 = (eq67_e1466 * locals.var_cgeff_dn13);
        let eq67_e1468_d_n14: f64 = (eq67_e1466 * locals.var_cgeff_dn14);
        let eq67_e1468_d_n15: f64 = (eq67_e1466 * locals.var_cgeff_dn15);
        let eq67_e1468_d_n16: f64 = (eq67_e1466 * locals.var_cgeff_dn16);
        let eq67_e1468_d_n17: f64 = (eq67_e1466 * locals.var_cgeff_dn17);
        let eq67_e1468_d_n18: f64 = (eq67_e1466 * locals.var_cgeff_dn18);
        let eq67_e1468_d_n19: f64 = (eq67_e1466 * locals.var_cgeff_dn19);
        let eq67_e1468_d_n20: f64 = (eq67_e1466 * locals.var_cgeff_dn20);
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n5: f64 = (eq67_e1468_d_n5 * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = (eq67_e1468_d_n6 * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = (eq67_e1468_d_n7 * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = (eq67_e1468_d_n8 * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = (eq67_e1468_d_n12 * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = (eq67_e1468_d_n13 * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = (eq67_e1468_d_n14 * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = (eq67_e1468_d_n15 * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = (eq67_e1468_d_n16 * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = (eq67_e1468_d_n17 * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = (eq67_e1468_d_n18 * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = (eq67_e1468_d_n19 * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = (eq67_e1468_d_n20 * (nv4 - 0.0));
        let eq67_e1471_q: f64 = eq67_e1470;
        let eq67_e1472: f64 = (-eq67_e1470);
        let eq67_e1472_q: f64 = (-eq67_e1471_q);
        let eq67_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, (-eq67_e1468), (-eq67_e1470_d_n5), (-eq67_e1470_d_n6), (-eq67_e1470_d_n7), (-eq67_e1470_d_n8), 0.0, 0.0, 0.0, (-eq67_e1470_d_n12), (-eq67_e1470_d_n13), (-eq67_e1470_d_n14), (-eq67_e1470_d_n15), (-eq67_e1470_d_n16), (-eq67_e1470_d_n17), (-eq67_e1470_d_n18), (-eq67_e1470_d_n19), (-eq67_e1470_d_n20)];
        let eq67_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq67_reactive_node_derivatives,
            branches,
            &eq67_reactive_branch_derivatives,
            multiplicity,
        );
        let eq68_e1475: f64 = (locals.var_mult_inst * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * locals.var_cgeff);
        let eq68_e1480_d_n5: f64 = (eq68_e1478 * locals.var_cgeff_dn5);
        let eq68_e1480_d_n6: f64 = (eq68_e1478 * locals.var_cgeff_dn6);
        let eq68_e1480_d_n7: f64 = (eq68_e1478 * locals.var_cgeff_dn7);
        let eq68_e1480_d_n8: f64 = (eq68_e1478 * locals.var_cgeff_dn8);
        let eq68_e1480_d_n12: f64 = (eq68_e1478 * locals.var_cgeff_dn12);
        let eq68_e1480_d_n13: f64 = (eq68_e1478 * locals.var_cgeff_dn13);
        let eq68_e1480_d_n14: f64 = (eq68_e1478 * locals.var_cgeff_dn14);
        let eq68_e1480_d_n15: f64 = (eq68_e1478 * locals.var_cgeff_dn15);
        let eq68_e1480_d_n16: f64 = (eq68_e1478 * locals.var_cgeff_dn16);
        let eq68_e1480_d_n17: f64 = (eq68_e1478 * locals.var_cgeff_dn17);
        let eq68_e1480_d_n18: f64 = (eq68_e1478 * locals.var_cgeff_dn18);
        let eq68_e1480_d_n19: f64 = (eq68_e1478 * locals.var_cgeff_dn19);
        let eq68_e1480_d_n20: f64 = (eq68_e1478 * locals.var_cgeff_dn20);
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1483_q: f64 = eq68_e1482;
        let eq68_e1484: f64 = (-eq68_e1482);
        let eq68_e1484_q: f64 = (-eq68_e1483_q);
        let eq68_reactive_node_derivatives: [f64; 21] = [0.0, 0.0, 0.0, 0.0, (-eq68_e1480), (-eq68_e1482_d_n5), (-eq68_e1482_d_n6), (-eq68_e1482_d_n7), (-eq68_e1482_d_n8), 0.0, 0.0, 0.0, (-eq68_e1482_d_n12), (-eq68_e1482_d_n13), (-eq68_e1482_d_n14), (-eq68_e1482_d_n15), (-eq68_e1482_d_n16), (-eq68_e1482_d_n17), (-eq68_e1482_d_n18), (-eq68_e1482_d_n19), (-eq68_e1482_d_n20)];
        let eq68_reactive_branch_derivatives: [f64; 25] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq68_reactive_node_derivatives,
            branches,
            &eq68_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
