#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_224(
        locals: &mut StampLocals,
    ) {
        let (assign65540_e102262, assign65540_e102262_d_n0, assign65540_e102262_d_n2, assign65540_e102262_d_n4, assign65540_e102262_d_n5, assign65540_e102262_d_n6, assign65540_e102262_d_n7, assign65540_e102262_d_n8, assign65540_e102262_d_n9, assign65540_e102262_d_n10, assign65540_e102262_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65540_e102256: f64 = (0.2 * locals.var_beta);
        let assign65540_e102259: f64 = (0.2 * locals.var_beta);
        let assign65540_e102260: f64 = (assign65540_e102256 * assign65540_e102259);
        (assign65540_e102260, (((0.2 * locals.var_beta_dn0) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn0))), (((0.2 * locals.var_beta_dn2) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn2))), (((0.2 * locals.var_beta_dn4) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn4))), (((0.2 * locals.var_beta_dn5) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn5))), (((0.2 * locals.var_beta_dn6) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn6))), (((0.2 * locals.var_beta_dn7) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn7))), (((0.2 * locals.var_beta_dn8) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn8))), (((0.2 * locals.var_beta_dn9) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn9))), (((0.2 * locals.var_beta_dn10) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn10))), (((0.2 * locals.var_beta_dn13) * assign65540_e102259) + (assign65540_e102256 * (0.2 * locals.var_beta_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign65540_e102262;
        locals.var_xmax2_dn0 = assign65540_e102262_d_n0;
        locals.var_xmax2_dn2 = assign65540_e102262_d_n2;
        locals.var_xmax2_dn4 = assign65540_e102262_d_n4;
        locals.var_xmax2_dn5 = assign65540_e102262_d_n5;
        locals.var_xmax2_dn6 = assign65540_e102262_d_n6;
        locals.var_xmax2_dn7 = assign65540_e102262_d_n7;
        locals.var_xmax2_dn8 = assign65540_e102262_d_n8;
        locals.var_xmax2_dn9 = assign65540_e102262_d_n9;
        locals.var_xmax2_dn10 = assign65540_e102262_d_n10;
        locals.var_xmax2_dn13 = assign65540_e102262_d_n13;

        let (assign65550_e102271, assign65550_e102271_d_n0, assign65550_e102271_d_n2, assign65550_e102271_d_n4, assign65550_e102271_d_n5, assign65550_e102271_d_n6, assign65550_e102271_d_n7, assign65550_e102271_d_n8, assign65550_e102271_d_n9, assign65550_e102271_d_n10, assign65550_e102271_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign65550_e102271;
        locals.var_xp_dn0 = assign65550_e102271_d_n0;
        locals.var_xp_dn2 = assign65550_e102271_d_n2;
        locals.var_xp_dn4 = assign65550_e102271_d_n4;
        locals.var_xp_dn5 = assign65550_e102271_d_n5;
        locals.var_xp_dn6 = assign65550_e102271_d_n6;
        locals.var_xp_dn7 = assign65550_e102271_d_n7;
        locals.var_xp_dn8 = assign65550_e102271_d_n8;
        locals.var_xp_dn9 = assign65550_e102271_d_n9;
        locals.var_xp_dn10 = assign65550_e102271_d_n10;
        locals.var_xp_dn13 = assign65550_e102271_d_n13;

        let (assign65560_e102280, assign65560_e102280_d_n0, assign65560_e102280_d_n2, assign65560_e102280_d_n4, assign65560_e102280_d_n5, assign65560_e102280_d_n6, assign65560_e102280_d_n7, assign65560_e102280_d_n8, assign65560_e102280_d_n9, assign65560_e102280_d_n10, assign65560_e102280_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign65560_e102280;
        locals.var_xmp_dn0 = assign65560_e102280_d_n0;
        locals.var_xmp_dn2 = assign65560_e102280_d_n2;
        locals.var_xmp_dn4 = assign65560_e102280_d_n4;
        locals.var_xmp_dn5 = assign65560_e102280_d_n5;
        locals.var_xmp_dn6 = assign65560_e102280_d_n6;
        locals.var_xmp_dn7 = assign65560_e102280_d_n7;
        locals.var_xmp_dn8 = assign65560_e102280_d_n8;
        locals.var_xmp_dn9 = assign65560_e102280_d_n9;
        locals.var_xmp_dn10 = assign65560_e102280_d_n10;
        locals.var_xmp_dn13 = assign65560_e102280_d_n13;

        let (assign65570_e102289,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign65570_e102289;

        let (assign65580_e102298,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65580_e102298;

        let (assign65590_e102307, assign65590_e102307_d_n0, assign65590_e102307_d_n2, assign65590_e102307_d_n4, assign65590_e102307_d_n5, assign65590_e102307_d_n6, assign65590_e102307_d_n7, assign65590_e102307_d_n8, assign65590_e102307_d_n9, assign65590_e102307_d_n10, assign65590_e102307_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign65590_e102307;
        locals.var_arg_dn0 = assign65590_e102307_d_n0;
        locals.var_arg_dn2 = assign65590_e102307_d_n2;
        locals.var_arg_dn4 = assign65590_e102307_d_n4;
        locals.var_arg_dn5 = assign65590_e102307_d_n5;
        locals.var_arg_dn6 = assign65590_e102307_d_n6;
        locals.var_arg_dn7 = assign65590_e102307_d_n7;
        locals.var_arg_dn8 = assign65590_e102307_d_n8;
        locals.var_arg_dn9 = assign65590_e102307_d_n9;
        locals.var_arg_dn10 = assign65590_e102307_d_n10;
        locals.var_arg_dn13 = assign65590_e102307_d_n13;

        let (assign65600_e102316, assign65600_e102316_d_n0, assign65600_e102316_d_n2, assign65600_e102316_d_n4, assign65600_e102316_d_n5, assign65600_e102316_d_n6, assign65600_e102316_d_n7, assign65600_e102316_d_n8, assign65600_e102316_d_n9, assign65600_e102316_d_n10, assign65600_e102316_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign65600_e102316;
        locals.var_dnm_dn0 = assign65600_e102316_d_n0;
        locals.var_dnm_dn2 = assign65600_e102316_d_n2;
        locals.var_dnm_dn4 = assign65600_e102316_d_n4;
        locals.var_dnm_dn5 = assign65600_e102316_d_n5;
        locals.var_dnm_dn6 = assign65600_e102316_d_n6;
        locals.var_dnm_dn7 = assign65600_e102316_d_n7;
        locals.var_dnm_dn8 = assign65600_e102316_d_n8;
        locals.var_dnm_dn9 = assign65600_e102316_d_n9;
        locals.var_dnm_dn10 = assign65600_e102316_d_n10;
        locals.var_dnm_dn13 = assign65600_e102316_d_n13;

        let (assign65610_e102327, assign65610_e102327_d_n0, assign65610_e102327_d_n2, assign65610_e102327_d_n4, assign65610_e102327_d_n5, assign65610_e102327_d_n6, assign65610_e102327_d_n7, assign65610_e102327_d_n8, assign65610_e102327_d_n9, assign65610_e102327_d_n10, assign65610_e102327_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65610_e102325: f64 = (locals.var_xp * locals.var_x2);
        (assign65610_e102325, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign65610_e102327;
        locals.var_xp_dn0 = assign65610_e102327_d_n0;
        locals.var_xp_dn2 = assign65610_e102327_d_n2;
        locals.var_xp_dn4 = assign65610_e102327_d_n4;
        locals.var_xp_dn5 = assign65610_e102327_d_n5;
        locals.var_xp_dn6 = assign65610_e102327_d_n6;
        locals.var_xp_dn7 = assign65610_e102327_d_n7;
        locals.var_xp_dn8 = assign65610_e102327_d_n8;
        locals.var_xp_dn9 = assign65610_e102327_d_n9;
        locals.var_xp_dn10 = assign65610_e102327_d_n10;
        locals.var_xp_dn13 = assign65610_e102327_d_n13;

        let (assign65620_e102338, assign65620_e102338_d_n0, assign65620_e102338_d_n2, assign65620_e102338_d_n4, assign65620_e102338_d_n5, assign65620_e102338_d_n6, assign65620_e102338_d_n7, assign65620_e102338_d_n8, assign65620_e102338_d_n9, assign65620_e102338_d_n10, assign65620_e102338_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65620_e102336: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign65620_e102336, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign65620_e102338;
        locals.var_xmp_dn0 = assign65620_e102338_d_n0;
        locals.var_xmp_dn2 = assign65620_e102338_d_n2;
        locals.var_xmp_dn4 = assign65620_e102338_d_n4;
        locals.var_xmp_dn5 = assign65620_e102338_d_n5;
        locals.var_xmp_dn6 = assign65620_e102338_d_n6;
        locals.var_xmp_dn7 = assign65620_e102338_d_n7;
        locals.var_xmp_dn8 = assign65620_e102338_d_n8;
        locals.var_xmp_dn9 = assign65620_e102338_d_n9;
        locals.var_xmp_dn10 = assign65620_e102338_d_n10;
        locals.var_xmp_dn13 = assign65620_e102338_d_n13;

        let (assign65630_e102349, assign65630_e102349_d_n0, assign65630_e102349_d_n2, assign65630_e102349_d_n4, assign65630_e102349_d_n5, assign65630_e102349_d_n6, assign65630_e102349_d_n7, assign65630_e102349_d_n8, assign65630_e102349_d_n9, assign65630_e102349_d_n10, assign65630_e102349_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65630_e102347: f64 = (locals.var_xp + locals.var_xmp);
        (assign65630_e102347, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign65630_e102349;
        locals.var_arg_dn0 = assign65630_e102349_d_n0;
        locals.var_arg_dn2 = assign65630_e102349_d_n2;
        locals.var_arg_dn4 = assign65630_e102349_d_n4;
        locals.var_arg_dn5 = assign65630_e102349_d_n5;
        locals.var_arg_dn6 = assign65630_e102349_d_n6;
        locals.var_arg_dn7 = assign65630_e102349_d_n7;
        locals.var_arg_dn8 = assign65630_e102349_d_n8;
        locals.var_arg_dn9 = assign65630_e102349_d_n9;
        locals.var_arg_dn10 = assign65630_e102349_d_n10;
        locals.var_arg_dn13 = assign65630_e102349_d_n13;

        let (assign65640_e102358, assign65640_e102358_d_n0, assign65640_e102358_d_n2, assign65640_e102358_d_n4, assign65640_e102358_d_n5, assign65640_e102358_d_n6, assign65640_e102358_d_n7, assign65640_e102358_d_n8, assign65640_e102358_d_n9, assign65640_e102358_d_n10, assign65640_e102358_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign65640_e102358;
        locals.var_dnm_dn0 = assign65640_e102358_d_n0;
        locals.var_dnm_dn2 = assign65640_e102358_d_n2;
        locals.var_dnm_dn4 = assign65640_e102358_d_n4;
        locals.var_dnm_dn5 = assign65640_e102358_d_n5;
        locals.var_dnm_dn6 = assign65640_e102358_d_n6;
        locals.var_dnm_dn7 = assign65640_e102358_d_n7;
        locals.var_dnm_dn8 = assign65640_e102358_d_n8;
        locals.var_dnm_dn9 = assign65640_e102358_d_n9;
        locals.var_dnm_dn10 = assign65640_e102358_d_n10;
        locals.var_dnm_dn13 = assign65640_e102358_d_n13;

        let assign65650_e102373: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1569 = assign65650_e102373;

        let assign65660_e102376: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1570 = assign65660_e102376;

        let (assign65670_e102389,) = {
    if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65670_e102389;

        let assign65680_e102392: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1571 = assign65680_e102392;

        let (assign65690_e102408,) = {
    if ((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_guard1570 == 0.0)) && (locals.var_guard1571 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65690_e102408;

        let assign65700_e102411: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1572 = assign65700_e102411;

        let (assign65710_e102430,) = {
    if (((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_guard1570 == 0.0)) && (locals.var_guard1571 == 0.0)) && (locals.var_guard1572 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65710_e102430;

        let assign65720_e102433: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1573 = assign65720_e102433;

        let (assign65730_e102455,) = {
    if ((((((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_guard1570 == 0.0)) && (locals.var_guard1571 == 0.0)) && (locals.var_guard1572 == 0.0)) && (locals.var_guard1573 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65730_e102455;

        let (assign65740_e102466,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign65740_e102466;

        let mut assign65750_loop_guard: usize = 0;
        while {
            let assign65750_cond_e102478: f64 = if (((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign65750_cond_e102478 != 0.0
        } {
            assign65750_loop_guard += 1;
            assert!(assign65750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign65750_body0_e102490, assign65750_body0_e102490_d_n0, assign65750_body0_e102490_d_n2, assign65750_body0_e102490_d_n4, assign65750_body0_e102490_d_n5, assign65750_body0_e102490_d_n6, assign65750_body0_e102490_d_n7, assign65750_body0_e102490_d_n8, assign65750_body0_e102490_d_n9, assign65750_body0_e102490_d_n10, assign65750_body0_e102490_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) {
        let assign65750_body0_e102488: f64 = (locals.var_dnm).sqrt();
        (assign65750_body0_e102488, (locals.var_dnm_dn0 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn2 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn4 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn5 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn6 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn7 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn8 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn9 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn10 / (2.0 * assign65750_body0_e102488)), (locals.var_dnm_dn13 / (2.0 * assign65750_body0_e102488)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign65750_body0_e102490;
            locals.var_dnm_dn0 = assign65750_body0_e102490_d_n0;
            locals.var_dnm_dn2 = assign65750_body0_e102490_d_n2;
            locals.var_dnm_dn4 = assign65750_body0_e102490_d_n4;
            locals.var_dnm_dn5 = assign65750_body0_e102490_d_n5;
            locals.var_dnm_dn6 = assign65750_body0_e102490_d_n6;
            locals.var_dnm_dn7 = assign65750_body0_e102490_d_n7;
            locals.var_dnm_dn8 = assign65750_body0_e102490_d_n8;
            locals.var_dnm_dn9 = assign65750_body0_e102490_d_n9;
            locals.var_dnm_dn10 = assign65750_body0_e102490_d_n10;
            locals.var_dnm_dn13 = assign65750_body0_e102490_d_n13;
            let (assign65750_body1_e102503,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 != 0.0)) {
        let assign65750_body1_e102501: f64 = (locals.var_m0 + 1.0);
        (assign65750_body1_e102501,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign65750_body1_e102503;
        }

        let (assign65760_e102526, assign65760_e102526_d_n0, assign65760_e102526_d_n2, assign65760_e102526_d_n4, assign65760_e102526_d_n5, assign65760_e102526_d_n6, assign65760_e102526_d_n7, assign65760_e102526_d_n8, assign65760_e102526_d_n9, assign65760_e102526_d_n10, assign65760_e102526_d_n13,) = {
    if ((((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) && (locals.var_guard1569 == 0.0)) {
        let (assign65760_e102524, assign65760_e102524_d_n0, assign65760_e102524_d_n2, assign65760_e102524_d_n4, assign65760_e102524_d_n5, assign65760_e102524_d_n6, assign65760_e102524_d_n7, assign65760_e102524_d_n8, assign65760_e102524_d_n9, assign65760_e102524_d_n10, assign65760_e102524_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65760_e102521: f64 = 2.0;
                let assign65760_e102522: f64 = (1.0 / assign65760_e102521);
                let assign65760_e102523: f64 = (locals.var_dnm).powf(assign65760_e102522);
                (assign65760_e102523, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn0)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn2)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn4)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn5)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn6)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn7)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn8)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn9)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn10)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65760_e102522) as f64).is_finite() && ((assign65760_e102522) as f64).fract() == 0.0 { if assign65760_e102522 == 0.0 { 0.0 } else { (assign65760_e102522 * ((locals.var_dnm).powf(assign65760_e102522 - 1.0) * locals.var_dnm_dn13)) } } else { (assign65760_e102523 * (assign65760_e102522 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign65760_e102524, assign65760_e102524_d_n0, assign65760_e102524_d_n2, assign65760_e102524_d_n4, assign65760_e102524_d_n5, assign65760_e102524_d_n6, assign65760_e102524_d_n7, assign65760_e102524_d_n8, assign65760_e102524_d_n9, assign65760_e102524_d_n10, assign65760_e102524_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign65760_e102526;
        locals.var_dnm_dn0 = assign65760_e102526_d_n0;
        locals.var_dnm_dn2 = assign65760_e102526_d_n2;
        locals.var_dnm_dn4 = assign65760_e102526_d_n4;
        locals.var_dnm_dn5 = assign65760_e102526_d_n5;
        locals.var_dnm_dn6 = assign65760_e102526_d_n6;
        locals.var_dnm_dn7 = assign65760_e102526_d_n7;
        locals.var_dnm_dn8 = assign65760_e102526_d_n8;
        locals.var_dnm_dn9 = assign65760_e102526_d_n9;
        locals.var_dnm_dn10 = assign65760_e102526_d_n10;
        locals.var_dnm_dn13 = assign65760_e102526_d_n13;

        let (assign65770_e102537, assign65770_e102537_d_n0, assign65770_e102537_d_n2, assign65770_e102537_d_n4, assign65770_e102537_d_n5, assign65770_e102537_d_n6, assign65770_e102537_d_n7, assign65770_e102537_d_n8, assign65770_e102537_d_n9, assign65770_e102537_d_n10, assign65770_e102537_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65770_e102535: f64 = (1.0 / locals.var_dnm);
        (assign65770_e102535, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign65770_e102537;
        locals.var_dnm_dn0 = assign65770_e102537_d_n0;
        locals.var_dnm_dn2 = assign65770_e102537_d_n2;
        locals.var_dnm_dn4 = assign65770_e102537_d_n4;
        locals.var_dnm_dn5 = assign65770_e102537_d_n5;
        locals.var_dnm_dn6 = assign65770_e102537_d_n6;
        locals.var_dnm_dn7 = assign65770_e102537_d_n7;
        locals.var_dnm_dn8 = assign65770_e102537_d_n8;
        locals.var_dnm_dn9 = assign65770_e102537_d_n9;
        locals.var_dnm_dn10 = assign65770_e102537_d_n10;
        locals.var_dnm_dn13 = assign65770_e102537_d_n13;

        let (assign65780_e102552, assign65780_e102552_d_n0, assign65780_e102552_d_n2, assign65780_e102552_d_n4, assign65780_e102552_d_n5, assign65780_e102552_d_n6, assign65780_e102552_d_n7, assign65780_e102552_d_n8, assign65780_e102552_d_n9, assign65780_e102552_d_n10, assign65780_e102552_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65780_e102547: f64 = (0.2 * locals.var_beta);
        let assign65780_e102548: f64 = (locals.var_tmf1 * assign65780_e102547);
        let assign65780_e102550: f64 = (assign65780_e102548 * locals.var_dnm);
        (assign65780_e102550, ((((locals.var_tmf1_dn0 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn0))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn2))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn4))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn5))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn6))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn7))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn8))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn9))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn10))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign65780_e102547) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn13))) * locals.var_dnm) + (assign65780_e102548 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign65780_e102552;
        locals.var_tmf0_dn0 = assign65780_e102552_d_n0;
        locals.var_tmf0_dn2 = assign65780_e102552_d_n2;
        locals.var_tmf0_dn4 = assign65780_e102552_d_n4;
        locals.var_tmf0_dn5 = assign65780_e102552_d_n5;
        locals.var_tmf0_dn6 = assign65780_e102552_d_n6;
        locals.var_tmf0_dn7 = assign65780_e102552_d_n7;
        locals.var_tmf0_dn8 = assign65780_e102552_d_n8;
        locals.var_tmf0_dn9 = assign65780_e102552_d_n9;
        locals.var_tmf0_dn10 = assign65780_e102552_d_n10;
        locals.var_tmf0_dn13 = assign65780_e102552_d_n13;

        let (assign65790_e102569, assign65790_e102569_d_n0, assign65790_e102569_d_n2, assign65790_e102569_d_n4, assign65790_e102569_d_n5, assign65790_e102569_d_n6, assign65790_e102569_d_n7, assign65790_e102569_d_n8, assign65790_e102569_d_n9, assign65790_e102569_d_n10, assign65790_e102569_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65790_e102561: f64 = (0.2 * locals.var_beta);
        let assign65790_e102563: f64 = (assign65790_e102561 * locals.var_xmp);
        let assign65790_e102565: f64 = (assign65790_e102563 * locals.var_dnm);
        let assign65790_e102567: f64 = (assign65790_e102565 / locals.var_arg);
        (assign65790_e102567, ((((((((0.2 * locals.var_beta_dn0) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn0)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn2) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn2)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn4) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn4)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn5) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn5)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn6) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn6)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn7) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn7)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn8) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn8)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn9) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn9)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn10) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn10)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn13) * locals.var_xmp) + (assign65790_e102561 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign65790_e102563 * locals.var_dnm_dn13)) * locals.var_arg) - (assign65790_e102565 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign65790_e102569;
        locals.var_t0_dn0 = assign65790_e102569_d_n0;
        locals.var_t0_dn2 = assign65790_e102569_d_n2;
        locals.var_t0_dn4 = assign65790_e102569_d_n4;
        locals.var_t0_dn5 = assign65790_e102569_d_n5;
        locals.var_t0_dn6 = assign65790_e102569_d_n6;
        locals.var_t0_dn7 = assign65790_e102569_d_n7;
        locals.var_t0_dn8 = assign65790_e102569_d_n8;
        locals.var_t0_dn9 = assign65790_e102569_d_n9;
        locals.var_t0_dn10 = assign65790_e102569_d_n10;
        locals.var_t0_dn13 = assign65790_e102569_d_n13;

        let (assign65800_e102584, assign65800_e102584_d_n0, assign65800_e102584_d_n2, assign65800_e102584_d_n4, assign65800_e102584_d_n5, assign65800_e102584_d_n6, assign65800_e102584_d_n7, assign65800_e102584_d_n8, assign65800_e102584_d_n9, assign65800_e102584_d_n10, assign65800_e102584_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65800_e102579: f64 = (0.2 * locals.var_beta);
        let assign65800_e102580: f64 = assign65800_e102579;
        let assign65800_e102582: f64 = (assign65800_e102580 - locals.var_tmf0);
        (assign65800_e102582, ((0.2 * locals.var_beta_dn0) - locals.var_tmf0_dn0), ((0.2 * locals.var_beta_dn2) - locals.var_tmf0_dn2), ((0.2 * locals.var_beta_dn4) - locals.var_tmf0_dn4), ((0.2 * locals.var_beta_dn5) - locals.var_tmf0_dn5), ((0.2 * locals.var_beta_dn6) - locals.var_tmf0_dn6), ((0.2 * locals.var_beta_dn7) - locals.var_tmf0_dn7), ((0.2 * locals.var_beta_dn8) - locals.var_tmf0_dn8), ((0.2 * locals.var_beta_dn9) - locals.var_tmf0_dn9), ((0.2 * locals.var_beta_dn10) - locals.var_tmf0_dn10), ((0.2 * locals.var_beta_dn13) - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65800_e102584;
        locals.var_t1_dn0 = assign65800_e102584_d_n0;
        locals.var_t1_dn2 = assign65800_e102584_d_n2;
        locals.var_t1_dn4 = assign65800_e102584_d_n4;
        locals.var_t1_dn5 = assign65800_e102584_d_n5;
        locals.var_t1_dn6 = assign65800_e102584_d_n6;
        locals.var_t1_dn7 = assign65800_e102584_d_n7;
        locals.var_t1_dn8 = assign65800_e102584_d_n8;
        locals.var_t1_dn9 = assign65800_e102584_d_n9;
        locals.var_t1_dn10 = assign65800_e102584_d_n10;
        locals.var_t1_dn13 = assign65800_e102584_d_n13;

        let (assign65810_e102593, assign65810_e102593_d_n0, assign65810_e102593_d_n2, assign65810_e102593_d_n4, assign65810_e102593_d_n5, assign65810_e102593_d_n6, assign65810_e102593_d_n7, assign65810_e102593_d_n8, assign65810_e102593_d_n9, assign65810_e102593_d_n10, assign65810_e102593_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign65810_e102593;
        locals.var_t0_dn0 = assign65810_e102593_d_n0;
        locals.var_t0_dn2 = assign65810_e102593_d_n2;
        locals.var_t0_dn4 = assign65810_e102593_d_n4;
        locals.var_t0_dn5 = assign65810_e102593_d_n5;
        locals.var_t0_dn6 = assign65810_e102593_d_n6;
        locals.var_t0_dn7 = assign65810_e102593_d_n7;
        locals.var_t0_dn8 = assign65810_e102593_d_n8;
        locals.var_t0_dn9 = assign65810_e102593_d_n9;
        locals.var_t0_dn10 = assign65810_e102593_d_n10;
        locals.var_t0_dn13 = assign65810_e102593_d_n13;

        let (assign65820_e102603, assign65820_e102603_d_n0, assign65820_e102603_d_n2, assign65820_e102603_d_n4, assign65820_e102603_d_n5, assign65820_e102603_d_n6, assign65820_e102603_d_n7, assign65820_e102603_d_n8, assign65820_e102603_d_n9, assign65820_e102603_d_n10, assign65820_e102603_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 == 0.0)) {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn4, locals.var_t1w_dn5, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn8, locals.var_t1w_dn9, locals.var_t1w_dn10, locals.var_t1w_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign65820_e102603;
        locals.var_t1_dn0 = assign65820_e102603_d_n0;
        locals.var_t1_dn2 = assign65820_e102603_d_n2;
        locals.var_t1_dn4 = assign65820_e102603_d_n4;
        locals.var_t1_dn5 = assign65820_e102603_d_n5;
        locals.var_t1_dn6 = assign65820_e102603_d_n6;
        locals.var_t1_dn7 = assign65820_e102603_d_n7;
        locals.var_t1_dn8 = assign65820_e102603_d_n8;
        locals.var_t1_dn9 = assign65820_e102603_d_n9;
        locals.var_t1_dn10 = assign65820_e102603_d_n10;
        locals.var_t1_dn13 = assign65820_e102603_d_n13;

        let (assign65830_e102613, assign65830_e102613_d_n0, assign65830_e102613_d_n2, assign65830_e102613_d_n4, assign65830_e102613_d_n5, assign65830_e102613_d_n6, assign65830_e102613_d_n7, assign65830_e102613_d_n8, assign65830_e102613_d_n9, assign65830_e102613_d_n10, assign65830_e102613_d_n13,) = {
    if (((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) && (locals.var_guard1568 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign65830_e102613;
        locals.var_t0_dn0 = assign65830_e102613_d_n0;
        locals.var_t0_dn2 = assign65830_e102613_d_n2;
        locals.var_t0_dn4 = assign65830_e102613_d_n4;
        locals.var_t0_dn5 = assign65830_e102613_d_n5;
        locals.var_t0_dn6 = assign65830_e102613_d_n6;
        locals.var_t0_dn7 = assign65830_e102613_d_n7;
        locals.var_t0_dn8 = assign65830_e102613_d_n8;
        locals.var_t0_dn9 = assign65830_e102613_d_n9;
        locals.var_t0_dn10 = assign65830_e102613_d_n10;
        locals.var_t0_dn13 = assign65830_e102613_d_n13;

        let (assign65840_e102625, assign65840_e102625_d_n0, assign65840_e102625_d_n2, assign65840_e102625_d_n4, assign65840_e102625_d_n5, assign65840_e102625_d_n6, assign65840_e102625_d_n7, assign65840_e102625_d_n8, assign65840_e102625_d_n9, assign65840_e102625_d_n10, assign65840_e102625_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65840_e102621: f64 = (10.0 * 2.220446049250313e-16);
        let assign65840_e102622: f64 = (locals.var_t1 + assign65840_e102621);
        let assign65840_e102623: f64 = (assign65840_e102622).sqrt();
        (assign65840_e102623, (locals.var_t1_dn0 / (2.0 * assign65840_e102623)), (locals.var_t1_dn2 / (2.0 * assign65840_e102623)), (locals.var_t1_dn4 / (2.0 * assign65840_e102623)), (locals.var_t1_dn5 / (2.0 * assign65840_e102623)), (locals.var_t1_dn6 / (2.0 * assign65840_e102623)), (locals.var_t1_dn7 / (2.0 * assign65840_e102623)), (locals.var_t1_dn8 / (2.0 * assign65840_e102623)), (locals.var_t1_dn9 / (2.0 * assign65840_e102623)), (locals.var_t1_dn10 / (2.0 * assign65840_e102623)), (locals.var_t1_dn13 / (2.0 * assign65840_e102623)),)
    } else {
        (locals.var_sq1npt, locals.var_sq1npt_dn0, locals.var_sq1npt_dn2, locals.var_sq1npt_dn4, locals.var_sq1npt_dn5, locals.var_sq1npt_dn6, locals.var_sq1npt_dn7, locals.var_sq1npt_dn8, locals.var_sq1npt_dn9, locals.var_sq1npt_dn10, locals.var_sq1npt_dn13,)
    }
};
        locals.var_sq1npt = assign65840_e102625;
        locals.var_sq1npt_dn0 = assign65840_e102625_d_n0;
        locals.var_sq1npt_dn2 = assign65840_e102625_d_n2;
        locals.var_sq1npt_dn4 = assign65840_e102625_d_n4;
        locals.var_sq1npt_dn5 = assign65840_e102625_d_n5;
        locals.var_sq1npt_dn6 = assign65840_e102625_d_n6;
        locals.var_sq1npt_dn7 = assign65840_e102625_d_n7;
        locals.var_sq1npt_dn8 = assign65840_e102625_d_n8;
        locals.var_sq1npt_dn9 = assign65840_e102625_d_n9;
        locals.var_sq1npt_dn10 = assign65840_e102625_d_n10;
        locals.var_sq1npt_dn13 = assign65840_e102625_d_n13;

        let (assign65850_e102634, assign65850_e102634_d_n0, assign65850_e102634_d_n2, assign65850_e102634_d_n4, assign65850_e102634_d_n5, assign65850_e102634_d_n6, assign65850_e102634_d_n7, assign65850_e102634_d_n8, assign65850_e102634_d_n9, assign65850_e102634_d_n10, assign65850_e102634_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65850_e102632: f64 = (locals.var_conpt0 * locals.var_sq1npt);
        (assign65850_e102632, ((locals.var_conpt0_dn0 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn0)), ((locals.var_conpt0_dn2 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn2)), ((locals.var_conpt0_dn4 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn4)), ((locals.var_conpt0_dn5 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn5)), ((locals.var_conpt0_dn6 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn6)), ((locals.var_conpt0_dn7 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn7)), ((locals.var_conpt0_dn8 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn8)), ((locals.var_conpt0_dn9 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn9)), ((locals.var_conpt0_dn10 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn10)), ((locals.var_conpt0_dn13 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn13)),)
    } else {
        (locals.var_qn0npt, locals.var_qn0npt_dn0, locals.var_qn0npt_dn2, locals.var_qn0npt_dn4, locals.var_qn0npt_dn5, locals.var_qn0npt_dn6, locals.var_qn0npt_dn7, locals.var_qn0npt_dn8, locals.var_qn0npt_dn9, locals.var_qn0npt_dn10, locals.var_qn0npt_dn13,)
    }
};
        locals.var_qn0npt = assign65850_e102634;
        locals.var_qn0npt_dn0 = assign65850_e102634_d_n0;
        locals.var_qn0npt_dn2 = assign65850_e102634_d_n2;
        locals.var_qn0npt_dn4 = assign65850_e102634_d_n4;
        locals.var_qn0npt_dn5 = assign65850_e102634_d_n5;
        locals.var_qn0npt_dn6 = assign65850_e102634_d_n6;
        locals.var_qn0npt_dn7 = assign65850_e102634_d_n7;
        locals.var_qn0npt_dn8 = assign65850_e102634_d_n8;
        locals.var_qn0npt_dn9 = assign65850_e102634_d_n9;
        locals.var_qn0npt_dn10 = assign65850_e102634_d_n10;
        locals.var_qn0npt_dn13 = assign65850_e102634_d_n13;

    }

    pub(super) fn stamp_transient_block_225(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65860_e102649, assign65860_e102649_d_n0, assign65860_e102649_d_n2, assign65860_e102649_d_n4, assign65860_e102649_d_n5, assign65860_e102649_d_n6, assign65860_e102649_d_n7, assign65860_e102649_d_n8, assign65860_e102649_d_n9, assign65860_e102649_d_n10, assign65860_e102649_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65860_e102641: f64 = (2.0 * locals.var_beta_inv);
        let assign65860_e102643: f64 = (assign65860_e102641 / locals.var_leff);
        let assign65860_e102645: f64 = (assign65860_e102643 * locals.var_qn0npt);
        let assign65860_e102647: f64 = (assign65860_e102645 * p.p454);
        (assign65860_e102647, (((((2.0 * locals.var_beta_inv_dn0) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn0)) * p.p454), (((((2.0 * locals.var_beta_inv_dn2) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn2)) * p.p454), (((((2.0 * locals.var_beta_inv_dn4) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn4)) * p.p454), (((((2.0 * locals.var_beta_inv_dn5) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn5)) * p.p454), (((((2.0 * locals.var_beta_inv_dn6) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn6)) * p.p454), (((((2.0 * locals.var_beta_inv_dn7) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn7)) * p.p454), (((((2.0 * locals.var_beta_inv_dn8) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn8)) * p.p454), (((((2.0 * locals.var_beta_inv_dn9) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn9)) * p.p454), (((((2.0 * locals.var_beta_inv_dn10) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn10)) * p.p454), (((((2.0 * locals.var_beta_inv_dn13) / locals.var_leff) * locals.var_qn0npt) + (assign65860_e102643 * locals.var_qn0npt_dn13)) * p.p454),)
    } else {
        (locals.var_wk_jnpt_a, locals.var_wk_jnpt_a_dn0, locals.var_wk_jnpt_a_dn2, locals.var_wk_jnpt_a_dn4, locals.var_wk_jnpt_a_dn5, locals.var_wk_jnpt_a_dn6, locals.var_wk_jnpt_a_dn7, locals.var_wk_jnpt_a_dn8, locals.var_wk_jnpt_a_dn9, locals.var_wk_jnpt_a_dn10, locals.var_wk_jnpt_a_dn13,)
    }
};
        locals.var_wk_jnpt_a = assign65860_e102649;
        locals.var_wk_jnpt_a_dn0 = assign65860_e102649_d_n0;
        locals.var_wk_jnpt_a_dn2 = assign65860_e102649_d_n2;
        locals.var_wk_jnpt_a_dn4 = assign65860_e102649_d_n4;
        locals.var_wk_jnpt_a_dn5 = assign65860_e102649_d_n5;
        locals.var_wk_jnpt_a_dn6 = assign65860_e102649_d_n6;
        locals.var_wk_jnpt_a_dn7 = assign65860_e102649_d_n7;
        locals.var_wk_jnpt_a_dn8 = assign65860_e102649_d_n8;
        locals.var_wk_jnpt_a_dn9 = assign65860_e102649_d_n9;
        locals.var_wk_jnpt_a_dn10 = assign65860_e102649_d_n10;
        locals.var_wk_jnpt_a_dn13 = assign65860_e102649_d_n13;

        let (assign65870_e102660, assign65870_e102660_d_n0, assign65870_e102660_d_n2, assign65870_e102660_d_n4, assign65870_e102660_d_n5, assign65870_e102660_d_n6, assign65870_e102660_d_n7, assign65870_e102660_d_n8, assign65870_e102660_d_n9, assign65870_e102660_d_n10, assign65870_e102660_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65870_e102656: f64 = (locals.var_wk_jnpt_a * locals.var_weff_nf);
        let assign65870_e102658: f64 = (assign65870_e102656 * locals.var_ty);
        (assign65870_e102658, (((locals.var_wk_jnpt_a_dn0 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn0)), (((locals.var_wk_jnpt_a_dn2 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn2)), (((locals.var_wk_jnpt_a_dn4 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn4)), (((locals.var_wk_jnpt_a_dn5 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn5)), (((locals.var_wk_jnpt_a_dn6 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn6)), (((locals.var_wk_jnpt_a_dn7 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn7)), (((locals.var_wk_jnpt_a_dn8 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn8)), (((locals.var_wk_jnpt_a_dn9 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn9)), (((locals.var_wk_jnpt_a_dn10 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn10)), (((locals.var_wk_jnpt_a_dn13 * locals.var_weff_nf) * locals.var_ty) + (assign65870_e102656 * locals.var_ty_dn13)),)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn4, locals.var_idspt1_dn5, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn8, locals.var_idspt1_dn9, locals.var_idspt1_dn10, locals.var_idspt1_dn13,)
    }
};
        locals.var_idspt1 = assign65870_e102660;
        locals.var_idspt1_dn0 = assign65870_e102660_d_n0;
        locals.var_idspt1_dn2 = assign65870_e102660_d_n2;
        locals.var_idspt1_dn4 = assign65870_e102660_d_n4;
        locals.var_idspt1_dn5 = assign65870_e102660_d_n5;
        locals.var_idspt1_dn6 = assign65870_e102660_d_n6;
        locals.var_idspt1_dn7 = assign65870_e102660_d_n7;
        locals.var_idspt1_dn8 = assign65870_e102660_d_n8;
        locals.var_idspt1_dn9 = assign65870_e102660_d_n9;
        locals.var_idspt1_dn10 = assign65870_e102660_d_n10;
        locals.var_idspt1_dn13 = assign65870_e102660_d_n13;

        let (assign65880_e102669, assign65880_e102669_d_n0, assign65880_e102669_d_n2, assign65880_e102669_d_n4, assign65880_e102669_d_n5, assign65880_e102669_d_n6, assign65880_e102669_d_n7, assign65880_e102669_d_n8, assign65880_e102669_d_n9, assign65880_e102669_d_n10, assign65880_e102669_d_n13,) = {
    if ((locals.var_guard443 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign65880_e102667: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign65880_e102667, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn4 + locals.var_idspt1_dn4), (locals.var_idsorg_dn5 + locals.var_idspt1_dn5), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn8 + locals.var_idspt1_dn8), (locals.var_idsorg_dn9 + locals.var_idspt1_dn9), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn13 + locals.var_idspt1_dn13),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign65880_e102669;
        locals.var_ids_dn0 = assign65880_e102669_d_n0;
        locals.var_ids_dn2 = assign65880_e102669_d_n2;
        locals.var_ids_dn4 = assign65880_e102669_d_n4;
        locals.var_ids_dn5 = assign65880_e102669_d_n5;
        locals.var_ids_dn6 = assign65880_e102669_d_n6;
        locals.var_ids_dn7 = assign65880_e102669_d_n7;
        locals.var_ids_dn8 = assign65880_e102669_d_n8;
        locals.var_ids_dn9 = assign65880_e102669_d_n9;
        locals.var_ids_dn10 = assign65880_e102669_d_n10;
        locals.var_ids_dn13 = assign65880_e102669_d_n13;

        let (assign65890_e102676, assign65890_e102676_d_n0, assign65890_e102676_d_n2, assign65890_e102676_d_n4, assign65890_e102676_d_n5, assign65890_e102676_d_n6, assign65890_e102676_d_n7, assign65890_e102676_d_n8, assign65890_e102676_d_n9, assign65890_e102676_d_n10, assign65890_e102676_d_n13,) = {
    if (locals.var_guard443 == 0.0) {
        let assign65890_e102674: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign65890_e102674, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn4 + locals.var_idspt1_dn4), (locals.var_idsorg_dn5 + locals.var_idspt1_dn5), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn8 + locals.var_idspt1_dn8), (locals.var_idsorg_dn9 + locals.var_idspt1_dn9), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn13 + locals.var_idspt1_dn13),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign65890_e102676;
        locals.var_ids_dn0 = assign65890_e102676_d_n0;
        locals.var_ids_dn2 = assign65890_e102676_d_n2;
        locals.var_ids_dn4 = assign65890_e102676_d_n4;
        locals.var_ids_dn5 = assign65890_e102676_d_n5;
        locals.var_ids_dn6 = assign65890_e102676_d_n6;
        locals.var_ids_dn7 = assign65890_e102676_d_n7;
        locals.var_ids_dn8 = assign65890_e102676_d_n8;
        locals.var_ids_dn9 = assign65890_e102676_d_n9;
        locals.var_ids_dn10 = assign65890_e102676_d_n10;
        locals.var_ids_dn13 = assign65890_e102676_d_n13;

        let (assign65910_e102688, assign65910_e102688_d_n0, assign65910_e102688_d_n2, assign65910_e102688_d_n4, assign65910_e102688_d_n5, assign65910_e102688_d_n6, assign65910_e102688_d_n7, assign65910_e102688_d_n8, assign65910_e102688_d_n9, assign65910_e102688_d_n10, assign65910_e102688_d_n13,) = {
    if (locals.var_guard443 == 0.0) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    } else {
        (locals.var_qiu_noi, locals.var_qiu_noi_dn0, locals.var_qiu_noi_dn2, locals.var_qiu_noi_dn4, locals.var_qiu_noi_dn5, locals.var_qiu_noi_dn6, locals.var_qiu_noi_dn7, locals.var_qiu_noi_dn8, locals.var_qiu_noi_dn9, locals.var_qiu_noi_dn10, locals.var_qiu_noi_dn13,)
    }
};
        locals.var_qiu_noi = assign65910_e102688;
        locals.var_qiu_noi_dn0 = assign65910_e102688_d_n0;
        locals.var_qiu_noi_dn2 = assign65910_e102688_d_n2;
        locals.var_qiu_noi_dn4 = assign65910_e102688_d_n4;
        locals.var_qiu_noi_dn5 = assign65910_e102688_d_n5;
        locals.var_qiu_noi_dn6 = assign65910_e102688_d_n6;
        locals.var_qiu_noi_dn7 = assign65910_e102688_d_n7;
        locals.var_qiu_noi_dn8 = assign65910_e102688_d_n8;
        locals.var_qiu_noi_dn9 = assign65910_e102688_d_n9;
        locals.var_qiu_noi_dn10 = assign65910_e102688_d_n10;
        locals.var_qiu_noi_dn13 = assign65910_e102688_d_n13;

        let assign65920_e102690: f64 = (-locals.var_weffcv_nf);
        let assign65920_e102692: f64 = (assign65920_e102690 * locals.var_leff);
        locals.var_t1 = assign65920_e102692;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;

        let assign65930_e102695: f64 = (locals.var_t1 * locals.var_qbu);
        locals.var_qb = assign65930_e102695;
        locals.var_qb_dn0 = ((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0));
        locals.var_qb_dn2 = ((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2));
        locals.var_qb_dn4 = ((locals.var_t1_dn4 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn4));
        locals.var_qb_dn5 = ((locals.var_t1_dn5 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn5));
        locals.var_qb_dn6 = ((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6));
        locals.var_qb_dn7 = ((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7));
        locals.var_qb_dn8 = ((locals.var_t1_dn8 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn8));
        locals.var_qb_dn9 = ((locals.var_t1_dn9 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn9));
        locals.var_qb_dn10 = ((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10));
        locals.var_qb_dn13 = ((locals.var_t1_dn13 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn13));

        let assign65940_e102698: f64 = (locals.var_t1 * locals.var_qiu);
        locals.var_qi = assign65940_e102698;
        locals.var_qi_dn0 = ((locals.var_t1_dn0 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn0));
        locals.var_qi_dn2 = ((locals.var_t1_dn2 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn2));
        locals.var_qi_dn4 = ((locals.var_t1_dn4 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn4));
        locals.var_qi_dn5 = ((locals.var_t1_dn5 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn5));
        locals.var_qi_dn6 = ((locals.var_t1_dn6 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn6));
        locals.var_qi_dn7 = ((locals.var_t1_dn7 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn7));
        locals.var_qi_dn8 = ((locals.var_t1_dn8 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn8));
        locals.var_qi_dn9 = ((locals.var_t1_dn9 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn9));
        locals.var_qi_dn10 = ((locals.var_t1_dn10 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn10));
        locals.var_qi_dn13 = ((locals.var_t1_dn13 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn13));

        let assign65950_e102701: f64 = (locals.var_qi * locals.var_qdrat);
        locals.var_qd = assign65950_e102701;
        locals.var_qd_dn0 = ((locals.var_qi_dn0 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn0));
        locals.var_qd_dn2 = ((locals.var_qi_dn2 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn2));
        locals.var_qd_dn4 = ((locals.var_qi_dn4 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn4));
        locals.var_qd_dn5 = ((locals.var_qi_dn5 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn5));
        locals.var_qd_dn6 = ((locals.var_qi_dn6 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn6));
        locals.var_qd_dn7 = ((locals.var_qi_dn7 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn7));
        locals.var_qd_dn8 = ((locals.var_qi_dn8 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn8));
        locals.var_qd_dn9 = ((locals.var_qi_dn9 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn9));
        locals.var_qd_dn10 = ((locals.var_qi_dn10 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn10));
        locals.var_qd_dn13 = ((locals.var_qi_dn13 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn13));

        let assign65960_e102704: f64 = (locals.var_t1 * locals.var_qiu_noi);
        locals.var_qi_noi = assign65960_e102704;
        locals.var_qi_noi_dn0 = ((locals.var_t1_dn0 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn0));
        locals.var_qi_noi_dn2 = ((locals.var_t1_dn2 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn2));
        locals.var_qi_noi_dn4 = ((locals.var_t1_dn4 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn4));
        locals.var_qi_noi_dn5 = ((locals.var_t1_dn5 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn5));
        locals.var_qi_noi_dn6 = ((locals.var_t1_dn6 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn6));
        locals.var_qi_noi_dn7 = ((locals.var_t1_dn7 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn7));
        locals.var_qi_noi_dn8 = ((locals.var_t1_dn8 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn8));
        locals.var_qi_noi_dn9 = ((locals.var_t1_dn9 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn9));
        locals.var_qi_noi_dn10 = ((locals.var_t1_dn10 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn10));
        locals.var_qi_noi_dn13 = ((locals.var_t1_dn13 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn13));

        let assign65970_e102707: f64 = (locals.var_vds - locals.var_pds);
        let assign65970_e102709: f64 = (assign65970_e102707 / 2.0);
        locals.var_t1 = assign65970_e102709;
        locals.var_t1_dn0 = ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0);
        locals.var_t1_dn2 = ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0);
        locals.var_t1_dn4 = ((locals.var_vds_dn4 - locals.var_pds_dn4) / 2.0);
        locals.var_t1_dn5 = ((locals.var_vds_dn5 - locals.var_pds_dn5) / 2.0);
        locals.var_t1_dn6 = ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0);
        locals.var_t1_dn7 = ((locals.var_vds_dn7 - locals.var_pds_dn7) / 2.0);
        locals.var_t1_dn8 = ((locals.var_vds_dn8 - locals.var_pds_dn8) / 2.0);
        locals.var_t1_dn9 = ((locals.var_vds_dn9 - locals.var_pds_dn9) / 2.0);
        locals.var_t1_dn10 = ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0);
        locals.var_t1_dn13 = ((locals.var_vds_dn13 - locals.var_pds_dn13) / 2.0);

        let assign65980_e102712: f64 = (2.0 * locals.var_t1);
        let assign65980_e102714: f64 = (assign65980_e102712 / p.p263);
        locals.var_tmf1 = assign65980_e102714;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p263);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p263);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p263);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p263);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p263);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p263);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p263);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p263);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p263);
        locals.var_tmf1_dn13 = ((2.0 * locals.var_t1_dn13) / p.p263);

        let assign65990_e102719: f64 = (1.0 / 2.0);
        let assign65990_e102723: f64 = (1.0 / 6.0);
        let assign65990_e102727: f64 = (1.0 / 24.0);
        let assign65990_e102731: f64 = (1.0 / 120.0);
        let assign65990_e102735: f64 = (1.0 / 720.0);
        let assign65990_e102739: f64 = (1.0 / 5040.0);
        let assign65990_e102740: f64 = (locals.var_tmf1 * assign65990_e102739);
        let assign65990_e102741: f64 = (assign65990_e102735 + assign65990_e102740);
        let assign65990_e102742: f64 = (locals.var_tmf1 * assign65990_e102741);
        let assign65990_e102743: f64 = (assign65990_e102731 + assign65990_e102742);
        let assign65990_e102744: f64 = (locals.var_tmf1 * assign65990_e102743);
        let assign65990_e102745: f64 = (assign65990_e102727 + assign65990_e102744);
        let assign65990_e102746: f64 = (locals.var_tmf1 * assign65990_e102745);
        let assign65990_e102747: f64 = (assign65990_e102723 + assign65990_e102746);
        let assign65990_e102748: f64 = (locals.var_tmf1 * assign65990_e102747);
        let assign65990_e102749: f64 = (assign65990_e102719 + assign65990_e102748);
        let assign65990_e102750: f64 = (locals.var_tmf1 * assign65990_e102749);
        let assign65990_e102751: f64 = (1.0 + assign65990_e102750);
        locals.var_tmf2 = assign65990_e102751;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign65990_e102739)))))))))));
        locals.var_tmf2_dn13 = ((locals.var_tmf1_dn13 * assign65990_e102749) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign65990_e102747) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign65990_e102745) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign65990_e102743) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign65990_e102741) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign65990_e102739)))))))))));

        let assign66000_e102754: f64 = (1.0 / 2.0);
        let assign66000_e102758: f64 = (1.0 / 3.0);
        let assign66000_e102762: f64 = (1.0 / 8.0);
        let assign66000_e102766: f64 = (1.0 / 30.0);
        let assign66000_e102770: f64 = (1.0 / 144.0);
        let assign66000_e102774: f64 = (1.0 / 840.0);
        let assign66000_e102775: f64 = (locals.var_tmf1 * assign66000_e102774);
        let assign66000_e102776: f64 = (assign66000_e102770 + assign66000_e102775);
        let assign66000_e102777: f64 = (locals.var_tmf1 * assign66000_e102776);
        let assign66000_e102778: f64 = (assign66000_e102766 + assign66000_e102777);
        let assign66000_e102779: f64 = (locals.var_tmf1 * assign66000_e102778);
        let assign66000_e102780: f64 = (assign66000_e102762 + assign66000_e102779);
        let assign66000_e102781: f64 = (locals.var_tmf1 * assign66000_e102780);
        let assign66000_e102782: f64 = (assign66000_e102758 + assign66000_e102781);
        let assign66000_e102783: f64 = (locals.var_tmf1 * assign66000_e102782);
        let assign66000_e102784: f64 = (assign66000_e102754 + assign66000_e102783);
        locals.var_tmf3 = assign66000_e102784;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign66000_e102774)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign66000_e102774)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign66000_e102774)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign66000_e102774)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign66000_e102774)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign66000_e102774)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign66000_e102774)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign66000_e102774)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign66000_e102774)))))))));
        locals.var_tmf3_dn13 = ((locals.var_tmf1_dn13 * assign66000_e102782) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign66000_e102780) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign66000_e102778) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign66000_e102776) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign66000_e102774)))))))));

        let assign66010_e102787: f64 = (p.p263 / locals.var_tmf2);
        locals.var_pzadd = assign66010_e102787;
        locals.var_pzadd_dn0 = (-((p.p263 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn2 = (-((p.p263 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn4 = (-((p.p263 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn5 = (-((p.p263 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn6 = (-((p.p263 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn7 = (-((p.p263 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn8 = (-((p.p263 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn9 = (-((p.p263 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn10 = (-((p.p263 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn13 = (-((p.p263 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)));

        let assign66020_e102789: f64 = (-2.0);
        let assign66020_e102791: f64 = (assign66020_e102789 * locals.var_tmf3);
        let assign66020_e102794: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign66020_e102795: f64 = (assign66020_e102791 / assign66020_e102794);
        locals.var_t2 = assign66020_e102795;
        locals.var_t2_dn0 = ((((assign66020_e102789 * locals.var_tmf3_dn0) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn2 = ((((assign66020_e102789 * locals.var_tmf3_dn2) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn4 = ((((assign66020_e102789 * locals.var_tmf3_dn4) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn5 = ((((assign66020_e102789 * locals.var_tmf3_dn5) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn6 = ((((assign66020_e102789 * locals.var_tmf3_dn6) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn7 = ((((assign66020_e102789 * locals.var_tmf3_dn7) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn8 = ((((assign66020_e102789 * locals.var_tmf3_dn8) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn9 = ((((assign66020_e102789 * locals.var_tmf3_dn9) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn10 = ((((assign66020_e102789 * locals.var_tmf3_dn10) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign66020_e102794 * assign66020_e102794));
        locals.var_t2_dn13 = ((((assign66020_e102789 * locals.var_tmf3_dn13) * assign66020_e102794) - (assign66020_e102791 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign66020_e102794 * assign66020_e102794));

        let assign66030_e102799: f64 = (10.0 * 2.220446049250313e-16);
        let assign66030_e102802: f64 = (10.0 * 2.220446049250313e-16);
        let assign66030_e102803: f64 = (assign66030_e102799 + assign66030_e102802);
        let assign66030_e102807: f64 = (10.0 * 2.220446049250313e-16);
        let assign66030_e102810: f64 = if ((locals.var_pzadd < assign66030_e102803) && (assign66030_e102807 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1574 = assign66030_e102810;

        let (assign66040_e102822, assign66040_e102822_d_n0, assign66040_e102822_d_n2, assign66040_e102822_d_n4, assign66040_e102822_d_n5, assign66040_e102822_d_n6, assign66040_e102822_d_n7, assign66040_e102822_d_n8, assign66040_e102822_d_n9, assign66040_e102822_d_n10, assign66040_e102822_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66040_e102814: f64 = (10.0 * 2.220446049250313e-16);
        let assign66040_e102817: f64 = (10.0 * 2.220446049250313e-16);
        let assign66040_e102818: f64 = (assign66040_e102814 + assign66040_e102817);
        let assign66040_e102820: f64 = (assign66040_e102818 - locals.var_pzadd);
        (assign66040_e102820, (-locals.var_pzadd_dn0), (-locals.var_pzadd_dn2), (-locals.var_pzadd_dn4), (-locals.var_pzadd_dn5), (-locals.var_pzadd_dn6), (-locals.var_pzadd_dn7), (-locals.var_pzadd_dn8), (-locals.var_pzadd_dn9), (-locals.var_pzadd_dn10), (-locals.var_pzadd_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign66040_e102822;
        locals.var_tmf1_dn0 = assign66040_e102822_d_n0;
        locals.var_tmf1_dn2 = assign66040_e102822_d_n2;
        locals.var_tmf1_dn4 = assign66040_e102822_d_n4;
        locals.var_tmf1_dn5 = assign66040_e102822_d_n5;
        locals.var_tmf1_dn6 = assign66040_e102822_d_n6;
        locals.var_tmf1_dn7 = assign66040_e102822_d_n7;
        locals.var_tmf1_dn8 = assign66040_e102822_d_n8;
        locals.var_tmf1_dn9 = assign66040_e102822_d_n9;
        locals.var_tmf1_dn10 = assign66040_e102822_d_n10;
        locals.var_tmf1_dn13 = assign66040_e102822_d_n13;

        let (assign66050_e102828, assign66050_e102828_d_n0, assign66050_e102828_d_n2, assign66050_e102828_d_n4, assign66050_e102828_d_n5, assign66050_e102828_d_n6, assign66050_e102828_d_n7, assign66050_e102828_d_n8, assign66050_e102828_d_n9, assign66050_e102828_d_n10, assign66050_e102828_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66050_e102826: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign66050_e102826, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign66050_e102828;
        locals.var_x2_dn0 = assign66050_e102828_d_n0;
        locals.var_x2_dn2 = assign66050_e102828_d_n2;
        locals.var_x2_dn4 = assign66050_e102828_d_n4;
        locals.var_x2_dn5 = assign66050_e102828_d_n5;
        locals.var_x2_dn6 = assign66050_e102828_d_n6;
        locals.var_x2_dn7 = assign66050_e102828_d_n7;
        locals.var_x2_dn8 = assign66050_e102828_d_n8;
        locals.var_x2_dn9 = assign66050_e102828_d_n9;
        locals.var_x2_dn10 = assign66050_e102828_d_n10;
        locals.var_x2_dn13 = assign66050_e102828_d_n13;

        let (assign66060_e102838, assign66060_e102838_d_n0, assign66060_e102838_d_n2, assign66060_e102838_d_n4, assign66060_e102838_d_n5, assign66060_e102838_d_n6, assign66060_e102838_d_n7, assign66060_e102838_d_n8, assign66060_e102838_d_n9, assign66060_e102838_d_n10, assign66060_e102838_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66060_e102832: f64 = (10.0 * 2.220446049250313e-16);
        let assign66060_e102835: f64 = (10.0 * 2.220446049250313e-16);
        let assign66060_e102836: f64 = (assign66060_e102832 * assign66060_e102835);
        (assign66060_e102836, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign66060_e102838;
        locals.var_xmax2_dn0 = assign66060_e102838_d_n0;
        locals.var_xmax2_dn2 = assign66060_e102838_d_n2;
        locals.var_xmax2_dn4 = assign66060_e102838_d_n4;
        locals.var_xmax2_dn5 = assign66060_e102838_d_n5;
        locals.var_xmax2_dn6 = assign66060_e102838_d_n6;
        locals.var_xmax2_dn7 = assign66060_e102838_d_n7;
        locals.var_xmax2_dn8 = assign66060_e102838_d_n8;
        locals.var_xmax2_dn9 = assign66060_e102838_d_n9;
        locals.var_xmax2_dn10 = assign66060_e102838_d_n10;
        locals.var_xmax2_dn13 = assign66060_e102838_d_n13;

        let (assign66070_e102842, assign66070_e102842_d_n0, assign66070_e102842_d_n2, assign66070_e102842_d_n4, assign66070_e102842_d_n5, assign66070_e102842_d_n6, assign66070_e102842_d_n7, assign66070_e102842_d_n8, assign66070_e102842_d_n9, assign66070_e102842_d_n10, assign66070_e102842_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66070_e102842;
        locals.var_xp_dn0 = assign66070_e102842_d_n0;
        locals.var_xp_dn2 = assign66070_e102842_d_n2;
        locals.var_xp_dn4 = assign66070_e102842_d_n4;
        locals.var_xp_dn5 = assign66070_e102842_d_n5;
        locals.var_xp_dn6 = assign66070_e102842_d_n6;
        locals.var_xp_dn7 = assign66070_e102842_d_n7;
        locals.var_xp_dn8 = assign66070_e102842_d_n8;
        locals.var_xp_dn9 = assign66070_e102842_d_n9;
        locals.var_xp_dn10 = assign66070_e102842_d_n10;
        locals.var_xp_dn13 = assign66070_e102842_d_n13;

        let (assign66080_e102846, assign66080_e102846_d_n0, assign66080_e102846_d_n2, assign66080_e102846_d_n4, assign66080_e102846_d_n5, assign66080_e102846_d_n6, assign66080_e102846_d_n7, assign66080_e102846_d_n8, assign66080_e102846_d_n9, assign66080_e102846_d_n10, assign66080_e102846_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66080_e102846;
        locals.var_xmp_dn0 = assign66080_e102846_d_n0;
        locals.var_xmp_dn2 = assign66080_e102846_d_n2;
        locals.var_xmp_dn4 = assign66080_e102846_d_n4;
        locals.var_xmp_dn5 = assign66080_e102846_d_n5;
        locals.var_xmp_dn6 = assign66080_e102846_d_n6;
        locals.var_xmp_dn7 = assign66080_e102846_d_n7;
        locals.var_xmp_dn8 = assign66080_e102846_d_n8;
        locals.var_xmp_dn9 = assign66080_e102846_d_n9;
        locals.var_xmp_dn10 = assign66080_e102846_d_n10;
        locals.var_xmp_dn13 = assign66080_e102846_d_n13;

        let (assign66090_e102850,) = {
    if (locals.var_guard1574 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66090_e102850;

        let (assign66100_e102854,) = {
    if (locals.var_guard1574 != 0.0) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66100_e102854;

        let (assign66110_e102858, assign66110_e102858_d_n0, assign66110_e102858_d_n2, assign66110_e102858_d_n4, assign66110_e102858_d_n5, assign66110_e102858_d_n6, assign66110_e102858_d_n7, assign66110_e102858_d_n8, assign66110_e102858_d_n9, assign66110_e102858_d_n10, assign66110_e102858_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign66110_e102858;
        locals.var_arg_dn0 = assign66110_e102858_d_n0;
        locals.var_arg_dn2 = assign66110_e102858_d_n2;
        locals.var_arg_dn4 = assign66110_e102858_d_n4;
        locals.var_arg_dn5 = assign66110_e102858_d_n5;
        locals.var_arg_dn6 = assign66110_e102858_d_n6;
        locals.var_arg_dn7 = assign66110_e102858_d_n7;
        locals.var_arg_dn8 = assign66110_e102858_d_n8;
        locals.var_arg_dn9 = assign66110_e102858_d_n9;
        locals.var_arg_dn10 = assign66110_e102858_d_n10;
        locals.var_arg_dn13 = assign66110_e102858_d_n13;

        let (assign66120_e102862, assign66120_e102862_d_n0, assign66120_e102862_d_n2, assign66120_e102862_d_n4, assign66120_e102862_d_n5, assign66120_e102862_d_n6, assign66120_e102862_d_n7, assign66120_e102862_d_n8, assign66120_e102862_d_n9, assign66120_e102862_d_n10, assign66120_e102862_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66120_e102862;
        locals.var_dnm_dn0 = assign66120_e102862_d_n0;
        locals.var_dnm_dn2 = assign66120_e102862_d_n2;
        locals.var_dnm_dn4 = assign66120_e102862_d_n4;
        locals.var_dnm_dn5 = assign66120_e102862_d_n5;
        locals.var_dnm_dn6 = assign66120_e102862_d_n6;
        locals.var_dnm_dn7 = assign66120_e102862_d_n7;
        locals.var_dnm_dn8 = assign66120_e102862_d_n8;
        locals.var_dnm_dn9 = assign66120_e102862_d_n9;
        locals.var_dnm_dn10 = assign66120_e102862_d_n10;
        locals.var_dnm_dn13 = assign66120_e102862_d_n13;

        let (assign66130_e102868, assign66130_e102868_d_n0, assign66130_e102868_d_n2, assign66130_e102868_d_n4, assign66130_e102868_d_n5, assign66130_e102868_d_n6, assign66130_e102868_d_n7, assign66130_e102868_d_n8, assign66130_e102868_d_n9, assign66130_e102868_d_n10, assign66130_e102868_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66130_e102866: f64 = (locals.var_xp * locals.var_x2);
        (assign66130_e102866, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66130_e102868;
        locals.var_xp_dn0 = assign66130_e102868_d_n0;
        locals.var_xp_dn2 = assign66130_e102868_d_n2;
        locals.var_xp_dn4 = assign66130_e102868_d_n4;
        locals.var_xp_dn5 = assign66130_e102868_d_n5;
        locals.var_xp_dn6 = assign66130_e102868_d_n6;
        locals.var_xp_dn7 = assign66130_e102868_d_n7;
        locals.var_xp_dn8 = assign66130_e102868_d_n8;
        locals.var_xp_dn9 = assign66130_e102868_d_n9;
        locals.var_xp_dn10 = assign66130_e102868_d_n10;
        locals.var_xp_dn13 = assign66130_e102868_d_n13;

        let (assign66140_e102874, assign66140_e102874_d_n0, assign66140_e102874_d_n2, assign66140_e102874_d_n4, assign66140_e102874_d_n5, assign66140_e102874_d_n6, assign66140_e102874_d_n7, assign66140_e102874_d_n8, assign66140_e102874_d_n9, assign66140_e102874_d_n10, assign66140_e102874_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66140_e102872: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66140_e102872, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66140_e102874;
        locals.var_xmp_dn0 = assign66140_e102874_d_n0;
        locals.var_xmp_dn2 = assign66140_e102874_d_n2;
        locals.var_xmp_dn4 = assign66140_e102874_d_n4;
        locals.var_xmp_dn5 = assign66140_e102874_d_n5;
        locals.var_xmp_dn6 = assign66140_e102874_d_n6;
        locals.var_xmp_dn7 = assign66140_e102874_d_n7;
        locals.var_xmp_dn8 = assign66140_e102874_d_n8;
        locals.var_xmp_dn9 = assign66140_e102874_d_n9;
        locals.var_xmp_dn10 = assign66140_e102874_d_n10;
        locals.var_xmp_dn13 = assign66140_e102874_d_n13;

        let (assign66150_e102880, assign66150_e102880_d_n0, assign66150_e102880_d_n2, assign66150_e102880_d_n4, assign66150_e102880_d_n5, assign66150_e102880_d_n6, assign66150_e102880_d_n7, assign66150_e102880_d_n8, assign66150_e102880_d_n9, assign66150_e102880_d_n10, assign66150_e102880_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66150_e102878: f64 = (locals.var_xp * locals.var_x2);
        (assign66150_e102878, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66150_e102880;
        locals.var_xp_dn0 = assign66150_e102880_d_n0;
        locals.var_xp_dn2 = assign66150_e102880_d_n2;
        locals.var_xp_dn4 = assign66150_e102880_d_n4;
        locals.var_xp_dn5 = assign66150_e102880_d_n5;
        locals.var_xp_dn6 = assign66150_e102880_d_n6;
        locals.var_xp_dn7 = assign66150_e102880_d_n7;
        locals.var_xp_dn8 = assign66150_e102880_d_n8;
        locals.var_xp_dn9 = assign66150_e102880_d_n9;
        locals.var_xp_dn10 = assign66150_e102880_d_n10;
        locals.var_xp_dn13 = assign66150_e102880_d_n13;

    }

    pub(super) fn stamp_transient_block_226(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign66160_e102886, assign66160_e102886_d_n0, assign66160_e102886_d_n2, assign66160_e102886_d_n4, assign66160_e102886_d_n5, assign66160_e102886_d_n6, assign66160_e102886_d_n7, assign66160_e102886_d_n8, assign66160_e102886_d_n9, assign66160_e102886_d_n10, assign66160_e102886_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66160_e102884: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66160_e102884, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66160_e102886;
        locals.var_xmp_dn0 = assign66160_e102886_d_n0;
        locals.var_xmp_dn2 = assign66160_e102886_d_n2;
        locals.var_xmp_dn4 = assign66160_e102886_d_n4;
        locals.var_xmp_dn5 = assign66160_e102886_d_n5;
        locals.var_xmp_dn6 = assign66160_e102886_d_n6;
        locals.var_xmp_dn7 = assign66160_e102886_d_n7;
        locals.var_xmp_dn8 = assign66160_e102886_d_n8;
        locals.var_xmp_dn9 = assign66160_e102886_d_n9;
        locals.var_xmp_dn10 = assign66160_e102886_d_n10;
        locals.var_xmp_dn13 = assign66160_e102886_d_n13;

        let (assign66170_e102892, assign66170_e102892_d_n0, assign66170_e102892_d_n2, assign66170_e102892_d_n4, assign66170_e102892_d_n5, assign66170_e102892_d_n6, assign66170_e102892_d_n7, assign66170_e102892_d_n8, assign66170_e102892_d_n9, assign66170_e102892_d_n10, assign66170_e102892_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66170_e102890: f64 = (locals.var_xp + locals.var_xmp);
        (assign66170_e102890, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign66170_e102892;
        locals.var_arg_dn0 = assign66170_e102892_d_n0;
        locals.var_arg_dn2 = assign66170_e102892_d_n2;
        locals.var_arg_dn4 = assign66170_e102892_d_n4;
        locals.var_arg_dn5 = assign66170_e102892_d_n5;
        locals.var_arg_dn6 = assign66170_e102892_d_n6;
        locals.var_arg_dn7 = assign66170_e102892_d_n7;
        locals.var_arg_dn8 = assign66170_e102892_d_n8;
        locals.var_arg_dn9 = assign66170_e102892_d_n9;
        locals.var_arg_dn10 = assign66170_e102892_d_n10;
        locals.var_arg_dn13 = assign66170_e102892_d_n13;

        let (assign66180_e102896, assign66180_e102896_d_n0, assign66180_e102896_d_n2, assign66180_e102896_d_n4, assign66180_e102896_d_n5, assign66180_e102896_d_n6, assign66180_e102896_d_n7, assign66180_e102896_d_n8, assign66180_e102896_d_n9, assign66180_e102896_d_n10, assign66180_e102896_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66180_e102896;
        locals.var_dnm_dn0 = assign66180_e102896_d_n0;
        locals.var_dnm_dn2 = assign66180_e102896_d_n2;
        locals.var_dnm_dn4 = assign66180_e102896_d_n4;
        locals.var_dnm_dn5 = assign66180_e102896_d_n5;
        locals.var_dnm_dn6 = assign66180_e102896_d_n6;
        locals.var_dnm_dn7 = assign66180_e102896_d_n7;
        locals.var_dnm_dn8 = assign66180_e102896_d_n8;
        locals.var_dnm_dn9 = assign66180_e102896_d_n9;
        locals.var_dnm_dn10 = assign66180_e102896_d_n10;
        locals.var_dnm_dn13 = assign66180_e102896_d_n13;

        let assign66190_e102911: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1575 = assign66190_e102911;

        let assign66200_e102914: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1576 = assign66200_e102914;

        let (assign66210_e102922,) = {
    if (((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_guard1576 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66210_e102922;

        let assign66220_e102925: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1577 = assign66220_e102925;

        let (assign66230_e102936,) = {
    if ((((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_guard1576 == 0.0)) && (locals.var_guard1577 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66230_e102936;

        let assign66240_e102939: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1578 = assign66240_e102939;

        let (assign66250_e102953,) = {
    if (((((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_guard1576 == 0.0)) && (locals.var_guard1577 == 0.0)) && (locals.var_guard1578 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66250_e102953;

        let assign66260_e102956: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1579 = assign66260_e102956;

        let (assign66270_e102973,) = {
    if ((((((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_guard1576 == 0.0)) && (locals.var_guard1577 == 0.0)) && (locals.var_guard1578 == 0.0)) && (locals.var_guard1579 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66270_e102973;

        let (assign66280_e102979,) = {
    if ((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66280_e102979;

        let mut assign66290_loop_guard: usize = 0;
        while {
            let assign66290_cond_e102986: f64 = if (((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign66290_cond_e102986 != 0.0
        } {
            assign66290_loop_guard += 1;
            assert!(assign66290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign66290_body0_e102993, assign66290_body0_e102993_d_n0, assign66290_body0_e102993_d_n2, assign66290_body0_e102993_d_n4, assign66290_body0_e102993_d_n5, assign66290_body0_e102993_d_n6, assign66290_body0_e102993_d_n7, assign66290_body0_e102993_d_n8, assign66290_body0_e102993_d_n9, assign66290_body0_e102993_d_n10, assign66290_body0_e102993_d_n13,) = {
    if ((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) {
        let assign66290_body0_e102991: f64 = (locals.var_dnm).sqrt();
        (assign66290_body0_e102991, (locals.var_dnm_dn0 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn2 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn4 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn5 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn6 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn7 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn8 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn9 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn10 / (2.0 * assign66290_body0_e102991)), (locals.var_dnm_dn13 / (2.0 * assign66290_body0_e102991)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign66290_body0_e102993;
            locals.var_dnm_dn0 = assign66290_body0_e102993_d_n0;
            locals.var_dnm_dn2 = assign66290_body0_e102993_d_n2;
            locals.var_dnm_dn4 = assign66290_body0_e102993_d_n4;
            locals.var_dnm_dn5 = assign66290_body0_e102993_d_n5;
            locals.var_dnm_dn6 = assign66290_body0_e102993_d_n6;
            locals.var_dnm_dn7 = assign66290_body0_e102993_d_n7;
            locals.var_dnm_dn8 = assign66290_body0_e102993_d_n8;
            locals.var_dnm_dn9 = assign66290_body0_e102993_d_n9;
            locals.var_dnm_dn10 = assign66290_body0_e102993_d_n10;
            locals.var_dnm_dn13 = assign66290_body0_e102993_d_n13;
            let (assign66290_body1_e103001,) = {
    if ((locals.var_guard1574 != 0.0) && (locals.var_guard1575 != 0.0)) {
        let assign66290_body1_e102999: f64 = (locals.var_m0 + 1.0);
        (assign66290_body1_e102999,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign66290_body1_e103001;
        }

        let (assign66300_e103019, assign66300_e103019_d_n0, assign66300_e103019_d_n2, assign66300_e103019_d_n4, assign66300_e103019_d_n5, assign66300_e103019_d_n6, assign66300_e103019_d_n7, assign66300_e103019_d_n8, assign66300_e103019_d_n9, assign66300_e103019_d_n10, assign66300_e103019_d_n13,) = {
    if ((locals.var_guard1574 != 0.0) && (locals.var_guard1575 == 0.0)) {
        let (assign66300_e103017, assign66300_e103017_d_n0, assign66300_e103017_d_n2, assign66300_e103017_d_n4, assign66300_e103017_d_n5, assign66300_e103017_d_n6, assign66300_e103017_d_n7, assign66300_e103017_d_n8, assign66300_e103017_d_n9, assign66300_e103017_d_n10, assign66300_e103017_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign66300_e103014: f64 = (2.0 * 2.0);
                let assign66300_e103015: f64 = (1.0 / assign66300_e103014);
                let assign66300_e103016: f64 = (locals.var_dnm).powf(assign66300_e103015);
                (assign66300_e103016, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn0)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn2)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn4)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn5)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn6)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn7)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn8)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn9)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn10)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66300_e103015) as f64).is_finite() && ((assign66300_e103015) as f64).fract() == 0.0 { if assign66300_e103015 == 0.0 { 0.0 } else { (assign66300_e103015 * ((locals.var_dnm).powf(assign66300_e103015 - 1.0) * locals.var_dnm_dn13)) } } else { (assign66300_e103016 * (assign66300_e103015 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign66300_e103017, assign66300_e103017_d_n0, assign66300_e103017_d_n2, assign66300_e103017_d_n4, assign66300_e103017_d_n5, assign66300_e103017_d_n6, assign66300_e103017_d_n7, assign66300_e103017_d_n8, assign66300_e103017_d_n9, assign66300_e103017_d_n10, assign66300_e103017_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66300_e103019;
        locals.var_dnm_dn0 = assign66300_e103019_d_n0;
        locals.var_dnm_dn2 = assign66300_e103019_d_n2;
        locals.var_dnm_dn4 = assign66300_e103019_d_n4;
        locals.var_dnm_dn5 = assign66300_e103019_d_n5;
        locals.var_dnm_dn6 = assign66300_e103019_d_n6;
        locals.var_dnm_dn7 = assign66300_e103019_d_n7;
        locals.var_dnm_dn8 = assign66300_e103019_d_n8;
        locals.var_dnm_dn9 = assign66300_e103019_d_n9;
        locals.var_dnm_dn10 = assign66300_e103019_d_n10;
        locals.var_dnm_dn13 = assign66300_e103019_d_n13;

        let (assign66310_e103025, assign66310_e103025_d_n0, assign66310_e103025_d_n2, assign66310_e103025_d_n4, assign66310_e103025_d_n5, assign66310_e103025_d_n6, assign66310_e103025_d_n7, assign66310_e103025_d_n8, assign66310_e103025_d_n9, assign66310_e103025_d_n10, assign66310_e103025_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66310_e103023: f64 = (1.0 / locals.var_dnm);
        (assign66310_e103023, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66310_e103025;
        locals.var_dnm_dn0 = assign66310_e103025_d_n0;
        locals.var_dnm_dn2 = assign66310_e103025_d_n2;
        locals.var_dnm_dn4 = assign66310_e103025_d_n4;
        locals.var_dnm_dn5 = assign66310_e103025_d_n5;
        locals.var_dnm_dn6 = assign66310_e103025_d_n6;
        locals.var_dnm_dn7 = assign66310_e103025_d_n7;
        locals.var_dnm_dn8 = assign66310_e103025_d_n8;
        locals.var_dnm_dn9 = assign66310_e103025_d_n9;
        locals.var_dnm_dn10 = assign66310_e103025_d_n10;
        locals.var_dnm_dn13 = assign66310_e103025_d_n13;

        let (assign66320_e103035, assign66320_e103035_d_n0, assign66320_e103035_d_n2, assign66320_e103035_d_n4, assign66320_e103035_d_n5, assign66320_e103035_d_n6, assign66320_e103035_d_n7, assign66320_e103035_d_n8, assign66320_e103035_d_n9, assign66320_e103035_d_n10, assign66320_e103035_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66320_e103030: f64 = (10.0 * 2.220446049250313e-16);
        let assign66320_e103031: f64 = (locals.var_tmf1 * assign66320_e103030);
        let assign66320_e103033: f64 = (assign66320_e103031 * locals.var_dnm);
        (assign66320_e103033, (((locals.var_tmf1_dn0 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign66320_e103030) * locals.var_dnm) + (assign66320_e103031 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign66320_e103035;
        locals.var_tmf0_dn0 = assign66320_e103035_d_n0;
        locals.var_tmf0_dn2 = assign66320_e103035_d_n2;
        locals.var_tmf0_dn4 = assign66320_e103035_d_n4;
        locals.var_tmf0_dn5 = assign66320_e103035_d_n5;
        locals.var_tmf0_dn6 = assign66320_e103035_d_n6;
        locals.var_tmf0_dn7 = assign66320_e103035_d_n7;
        locals.var_tmf0_dn8 = assign66320_e103035_d_n8;
        locals.var_tmf0_dn9 = assign66320_e103035_d_n9;
        locals.var_tmf0_dn10 = assign66320_e103035_d_n10;
        locals.var_tmf0_dn13 = assign66320_e103035_d_n13;

        let (assign66330_e103047, assign66330_e103047_d_n0, assign66330_e103047_d_n2, assign66330_e103047_d_n4, assign66330_e103047_d_n5, assign66330_e103047_d_n6, assign66330_e103047_d_n7, assign66330_e103047_d_n8, assign66330_e103047_d_n9, assign66330_e103047_d_n10, assign66330_e103047_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66330_e103039: f64 = (10.0 * 2.220446049250313e-16);
        let assign66330_e103041: f64 = (assign66330_e103039 * locals.var_xmp);
        let assign66330_e103043: f64 = (assign66330_e103041 * locals.var_dnm);
        let assign66330_e103045: f64 = (assign66330_e103043 / locals.var_arg);
        (assign66330_e103045, ((((((assign66330_e103039 * locals.var_xmp_dn0) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn0)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn2) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn2)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn4) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn4)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn5) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn5)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn6) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn6)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn7) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn7)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn8) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn8)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn9) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn9)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn10) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn10)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign66330_e103039 * locals.var_xmp_dn13) * locals.var_dnm) + (assign66330_e103041 * locals.var_dnm_dn13)) * locals.var_arg) - (assign66330_e103043 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66330_e103047;
        locals.var_t0_dn0 = assign66330_e103047_d_n0;
        locals.var_t0_dn2 = assign66330_e103047_d_n2;
        locals.var_t0_dn4 = assign66330_e103047_d_n4;
        locals.var_t0_dn5 = assign66330_e103047_d_n5;
        locals.var_t0_dn6 = assign66330_e103047_d_n6;
        locals.var_t0_dn7 = assign66330_e103047_d_n7;
        locals.var_t0_dn8 = assign66330_e103047_d_n8;
        locals.var_t0_dn9 = assign66330_e103047_d_n9;
        locals.var_t0_dn10 = assign66330_e103047_d_n10;
        locals.var_t0_dn13 = assign66330_e103047_d_n13;

        let (assign66340_e103059, assign66340_e103059_d_n0, assign66340_e103059_d_n2, assign66340_e103059_d_n4, assign66340_e103059_d_n5, assign66340_e103059_d_n6, assign66340_e103059_d_n7, assign66340_e103059_d_n8, assign66340_e103059_d_n9, assign66340_e103059_d_n10, assign66340_e103059_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        let assign66340_e103051: f64 = (10.0 * 2.220446049250313e-16);
        let assign66340_e103054: f64 = (10.0 * 2.220446049250313e-16);
        let assign66340_e103055: f64 = (assign66340_e103051 + assign66340_e103054);
        let assign66340_e103057: f64 = (assign66340_e103055 - locals.var_tmf0);
        (assign66340_e103057, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    }
};
        locals.var_pzadd = assign66340_e103059;
        locals.var_pzadd_dn0 = assign66340_e103059_d_n0;
        locals.var_pzadd_dn2 = assign66340_e103059_d_n2;
        locals.var_pzadd_dn4 = assign66340_e103059_d_n4;
        locals.var_pzadd_dn5 = assign66340_e103059_d_n5;
        locals.var_pzadd_dn6 = assign66340_e103059_d_n6;
        locals.var_pzadd_dn7 = assign66340_e103059_d_n7;
        locals.var_pzadd_dn8 = assign66340_e103059_d_n8;
        locals.var_pzadd_dn9 = assign66340_e103059_d_n9;
        locals.var_pzadd_dn10 = assign66340_e103059_d_n10;
        locals.var_pzadd_dn13 = assign66340_e103059_d_n13;

        let (assign66350_e103063, assign66350_e103063_d_n0, assign66350_e103063_d_n2, assign66350_e103063_d_n4, assign66350_e103063_d_n5, assign66350_e103063_d_n6, assign66350_e103063_d_n7, assign66350_e103063_d_n8, assign66350_e103063_d_n9, assign66350_e103063_d_n10, assign66350_e103063_d_n13,) = {
    if (locals.var_guard1574 != 0.0) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66350_e103063;
        locals.var_t0_dn0 = assign66350_e103063_d_n0;
        locals.var_t0_dn2 = assign66350_e103063_d_n2;
        locals.var_t0_dn4 = assign66350_e103063_d_n4;
        locals.var_t0_dn5 = assign66350_e103063_d_n5;
        locals.var_t0_dn6 = assign66350_e103063_d_n6;
        locals.var_t0_dn7 = assign66350_e103063_d_n7;
        locals.var_t0_dn8 = assign66350_e103063_d_n8;
        locals.var_t0_dn9 = assign66350_e103063_d_n9;
        locals.var_t0_dn10 = assign66350_e103063_d_n10;
        locals.var_t0_dn13 = assign66350_e103063_d_n13;

        let (assign66360_e103068, assign66360_e103068_d_n0, assign66360_e103068_d_n2, assign66360_e103068_d_n4, assign66360_e103068_d_n5, assign66360_e103068_d_n6, assign66360_e103068_d_n7, assign66360_e103068_d_n8, assign66360_e103068_d_n9, assign66360_e103068_d_n10, assign66360_e103068_d_n13,) = {
    if (locals.var_guard1574 == 0.0) {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn13,)
    }
};
        locals.var_pzadd = assign66360_e103068;
        locals.var_pzadd_dn0 = assign66360_e103068_d_n0;
        locals.var_pzadd_dn2 = assign66360_e103068_d_n2;
        locals.var_pzadd_dn4 = assign66360_e103068_d_n4;
        locals.var_pzadd_dn5 = assign66360_e103068_d_n5;
        locals.var_pzadd_dn6 = assign66360_e103068_d_n6;
        locals.var_pzadd_dn7 = assign66360_e103068_d_n7;
        locals.var_pzadd_dn8 = assign66360_e103068_d_n8;
        locals.var_pzadd_dn9 = assign66360_e103068_d_n9;
        locals.var_pzadd_dn10 = assign66360_e103068_d_n10;
        locals.var_pzadd_dn13 = assign66360_e103068_d_n13;

        let (assign66370_e103073, assign66370_e103073_d_n0, assign66370_e103073_d_n2, assign66370_e103073_d_n4, assign66370_e103073_d_n5, assign66370_e103073_d_n6, assign66370_e103073_d_n7, assign66370_e103073_d_n8, assign66370_e103073_d_n9, assign66370_e103073_d_n10, assign66370_e103073_d_n13,) = {
    if (locals.var_guard1574 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66370_e103073;
        locals.var_t0_dn0 = assign66370_e103073_d_n0;
        locals.var_t0_dn2 = assign66370_e103073_d_n2;
        locals.var_t0_dn4 = assign66370_e103073_d_n4;
        locals.var_t0_dn5 = assign66370_e103073_d_n5;
        locals.var_t0_dn6 = assign66370_e103073_d_n6;
        locals.var_t0_dn7 = assign66370_e103073_d_n7;
        locals.var_t0_dn8 = assign66370_e103073_d_n8;
        locals.var_t0_dn9 = assign66370_e103073_d_n9;
        locals.var_t0_dn10 = assign66370_e103073_d_n10;
        locals.var_t0_dn13 = assign66370_e103073_d_n13;

        let assign66380_e103076: f64 = (locals.var_ps0 + locals.var_pzadd);
        locals.var_ps0z = assign66380_e103076;
        locals.var_ps0z_dn0 = (locals.var_ps0_dn0 + locals.var_pzadd_dn0);
        locals.var_ps0z_dn2 = (locals.var_ps0_dn2 + locals.var_pzadd_dn2);
        locals.var_ps0z_dn4 = (locals.var_ps0_dn4 + locals.var_pzadd_dn4);
        locals.var_ps0z_dn5 = (locals.var_ps0_dn5 + locals.var_pzadd_dn5);
        locals.var_ps0z_dn6 = (locals.var_ps0_dn6 + locals.var_pzadd_dn6);
        locals.var_ps0z_dn7 = (locals.var_ps0_dn7 + locals.var_pzadd_dn7);
        locals.var_ps0z_dn8 = (locals.var_ps0_dn8 + locals.var_pzadd_dn8);
        locals.var_ps0z_dn9 = (locals.var_ps0_dn9 + locals.var_pzadd_dn9);
        locals.var_ps0z_dn10 = (locals.var_ps0_dn10 + locals.var_pzadd_dn10);
        locals.var_ps0z_dn13 = (locals.var_ps0_dn13 + locals.var_pzadd_dn13);

        let assign66390_e103080: f64 = (locals.var_weff / locals.var_leff);
        let assign66390_e103082: f64 = (assign66390_e103080 * p.p435);
        let assign66390_e103084: f64 = (assign66390_e103082 * locals.var_vds);
        let assign66390_e103085: f64 = (locals.var_ids + assign66390_e103084);
        locals.var_ids = assign66390_e103085;
        locals.var_ids_dn0 = (locals.var_ids_dn0 + (assign66390_e103082 * locals.var_vds_dn0));
        locals.var_ids_dn2 = (locals.var_ids_dn2 + (assign66390_e103082 * locals.var_vds_dn2));
        locals.var_ids_dn4 = (locals.var_ids_dn4 + (assign66390_e103082 * locals.var_vds_dn4));
        locals.var_ids_dn5 = (locals.var_ids_dn5 + (assign66390_e103082 * locals.var_vds_dn5));
        locals.var_ids_dn6 = (locals.var_ids_dn6 + (assign66390_e103082 * locals.var_vds_dn6));
        locals.var_ids_dn7 = (locals.var_ids_dn7 + (assign66390_e103082 * locals.var_vds_dn7));
        locals.var_ids_dn8 = (locals.var_ids_dn8 + (assign66390_e103082 * locals.var_vds_dn8));
        locals.var_ids_dn9 = (locals.var_ids_dn9 + (assign66390_e103082 * locals.var_vds_dn9));
        locals.var_ids_dn10 = (locals.var_ids_dn10 + (assign66390_e103082 * locals.var_vds_dn10));
        locals.var_ids_dn13 = (locals.var_ids_dn13 + (assign66390_e103082 * locals.var_vds_dn13));

        let assign66400_e103088: f64 = if p.p23 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1580 = assign66400_e103088;

        let (assign66410_e103092, assign66410_e103092_d_n0, assign66410_e103092_d_n2, assign66410_e103092_d_n4, assign66410_e103092_d_n5, assign66410_e103092_d_n6, assign66410_e103092_d_n7, assign66410_e103092_d_n8, assign66410_e103092_d_n9, assign66410_e103092_d_n10, assign66410_e103092_d_n13,) = {
    if (locals.var_guard1580 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13,)
    }
};
        locals.var_isub = assign66410_e103092;
        locals.var_isub_dn0 = assign66410_e103092_d_n0;
        locals.var_isub_dn2 = assign66410_e103092_d_n2;
        locals.var_isub_dn4 = assign66410_e103092_d_n4;
        locals.var_isub_dn5 = assign66410_e103092_d_n5;
        locals.var_isub_dn6 = assign66410_e103092_d_n6;
        locals.var_isub_dn7 = assign66410_e103092_d_n7;
        locals.var_isub_dn8 = assign66410_e103092_d_n8;
        locals.var_isub_dn9 = assign66410_e103092_d_n9;
        locals.var_isub_dn10 = assign66410_e103092_d_n10;
        locals.var_isub_dn13 = assign66410_e103092_d_n13;

        let (assign66420_e103096, assign66420_e103096_d_n0, assign66420_e103096_d_n2, assign66420_e103096_d_n4, assign66420_e103096_d_n5, assign66420_e103096_d_n6, assign66420_e103096_d_n7, assign66420_e103096_d_n8, assign66420_e103096_d_n9, assign66420_e103096_d_n10, assign66420_e103096_d_n13,) = {
    if (locals.var_guard1580 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wk_ii, locals.var_wk_ii_dn0, locals.var_wk_ii_dn2, locals.var_wk_ii_dn4, locals.var_wk_ii_dn5, locals.var_wk_ii_dn6, locals.var_wk_ii_dn7, locals.var_wk_ii_dn8, locals.var_wk_ii_dn9, locals.var_wk_ii_dn10, locals.var_wk_ii_dn13,)
    }
};
        locals.var_wk_ii = assign66420_e103096;
        locals.var_wk_ii_dn0 = assign66420_e103096_d_n0;
        locals.var_wk_ii_dn2 = assign66420_e103096_d_n2;
        locals.var_wk_ii_dn4 = assign66420_e103096_d_n4;
        locals.var_wk_ii_dn5 = assign66420_e103096_d_n5;
        locals.var_wk_ii_dn6 = assign66420_e103096_d_n6;
        locals.var_wk_ii_dn7 = assign66420_e103096_d_n7;
        locals.var_wk_ii_dn8 = assign66420_e103096_d_n8;
        locals.var_wk_ii_dn9 = assign66420_e103096_d_n9;
        locals.var_wk_ii_dn10 = assign66420_e103096_d_n10;
        locals.var_wk_ii_dn13 = assign66420_e103096_d_n13;

        let assign66430_e103103: f64 = if ((locals.var_uc_sub1 > 0.0) && (locals.var_uc_vmax > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1581 = assign66430_e103103;

        let (assign66440_e103112, assign66440_e103112_d_n0, assign66440_e103112_d_n2, assign66440_e103112_d_n4, assign66440_e103112_d_n5, assign66440_e103112_d_n6, assign66440_e103112_d_n7, assign66440_e103112_d_n8, assign66440_e103112_d_n9, assign66440_e103112_d_n10, assign66440_e103112_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66440_e103110: f64 = (locals.var_vg2const * locals.var_vgp);
        (assign66440_e103110, ((locals.var_vg2const_dn0 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn0)), ((locals.var_vg2const_dn2 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn2)), ((locals.var_vg2const_dn4 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn4)), ((locals.var_vg2const_dn5 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn5)), ((locals.var_vg2const_dn6 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn6)), ((locals.var_vg2const_dn7 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn7)), ((locals.var_vg2const_dn8 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn8)), ((locals.var_vg2const_dn9 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn9)), ((locals.var_vg2const_dn10 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn10)), ((locals.var_vg2const_dn13 * locals.var_vgp) + (locals.var_vg2const * locals.var_vgp_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign66440_e103112;
        locals.var_t1_dn0 = assign66440_e103112_d_n0;
        locals.var_t1_dn2 = assign66440_e103112_d_n2;
        locals.var_t1_dn4 = assign66440_e103112_d_n4;
        locals.var_t1_dn5 = assign66440_e103112_d_n5;
        locals.var_t1_dn6 = assign66440_e103112_d_n6;
        locals.var_t1_dn7 = assign66440_e103112_d_n7;
        locals.var_t1_dn8 = assign66440_e103112_d_n8;
        locals.var_t1_dn9 = assign66440_e103112_d_n9;
        locals.var_t1_dn10 = assign66440_e103112_d_n10;
        locals.var_t1_dn13 = assign66440_e103112_d_n13;

        let (assign66450_e103123, assign66450_e103123_d_n0, assign66450_e103123_d_n2, assign66450_e103123_d_n4, assign66450_e103123_d_n5, assign66450_e103123_d_n6, assign66450_e103123_d_n7, assign66450_e103123_d_n8, assign66450_e103123_d_n9, assign66450_e103123_d_n10, assign66450_e103123_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66450_e103120: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign66450_e103121: f64 = (locals.var_qnsub_esi / assign66450_e103120);
        (assign66450_e103121, (locals.var_qnsub_esi_dn0 / assign66450_e103120), (locals.var_qnsub_esi_dn2 / assign66450_e103120), (locals.var_qnsub_esi_dn4 / assign66450_e103120), (locals.var_qnsub_esi_dn5 / assign66450_e103120), (locals.var_qnsub_esi_dn6 / assign66450_e103120), (locals.var_qnsub_esi_dn7 / assign66450_e103120), (locals.var_qnsub_esi_dn8 / assign66450_e103120), (locals.var_qnsub_esi_dn9 / assign66450_e103120), (locals.var_qnsub_esi_dn10 / assign66450_e103120), (locals.var_qnsub_esi_dn13 / assign66450_e103120),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign66450_e103123;
        locals.var_t3_dn0 = assign66450_e103123_d_n0;
        locals.var_t3_dn2 = assign66450_e103123_d_n2;
        locals.var_t3_dn4 = assign66450_e103123_d_n4;
        locals.var_t3_dn5 = assign66450_e103123_d_n5;
        locals.var_t3_dn6 = assign66450_e103123_d_n6;
        locals.var_t3_dn7 = assign66450_e103123_d_n7;
        locals.var_t3_dn8 = assign66450_e103123_d_n8;
        locals.var_t3_dn9 = assign66450_e103123_d_n9;
        locals.var_t3_dn10 = assign66450_e103123_d_n10;
        locals.var_t3_dn13 = assign66450_e103123_d_n13;

        let (assign66460_e103136, assign66460_e103136_d_n0, assign66460_e103136_d_n2, assign66460_e103136_d_n4, assign66460_e103136_d_n5, assign66460_e103136_d_n6, assign66460_e103136_d_n7, assign66460_e103136_d_n8, assign66460_e103136_d_n9, assign66460_e103136_d_n10, assign66460_e103136_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66460_e103130: f64 = (2.0 / locals.var_qnsub_esi);
        let assign66460_e103133: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign66460_e103134: f64 = (assign66460_e103130 * assign66460_e103133);
        (assign66460_e103134, ((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn9) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133), ((-((2.0 * locals.var_qnsub_esi_dn13) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign66460_e103133),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign66460_e103136;
        locals.var_t4_dn0 = assign66460_e103136_d_n0;
        locals.var_t4_dn2 = assign66460_e103136_d_n2;
        locals.var_t4_dn4 = assign66460_e103136_d_n4;
        locals.var_t4_dn5 = assign66460_e103136_d_n5;
        locals.var_t4_dn6 = assign66460_e103136_d_n6;
        locals.var_t4_dn7 = assign66460_e103136_d_n7;
        locals.var_t4_dn8 = assign66460_e103136_d_n8;
        locals.var_t4_dn9 = assign66460_e103136_d_n9;
        locals.var_t4_dn10 = assign66460_e103136_d_n10;
        locals.var_t4_dn13 = assign66460_e103136_d_n13;

        let (assign66470_e103149, assign66470_e103149_d_n0, assign66470_e103149_d_n2, assign66470_e103149_d_n4, assign66470_e103149_d_n5, assign66470_e103149_d_n6, assign66470_e103149_d_n7, assign66470_e103149_d_n8, assign66470_e103149_d_n9, assign66470_e103149_d_n10, assign66470_e103149_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66470_e103143: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign66470_e103146: f64 = (locals.var_xvbs * locals.var_vbsz__blk438);
        let assign66470_e103147: f64 = (assign66470_e103143 - assign66470_e103146);
        (assign66470_e103147, ((locals.var_t1_dn0 - locals.var_beta_inv_dn0) - (locals.var_xvbs * locals.var_vbsz__blk438_dn0)), ((locals.var_t1_dn2 - locals.var_beta_inv_dn2) - (locals.var_xvbs * locals.var_vbsz__blk438_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs * locals.var_vbsz__blk438_dn4)), ((locals.var_t1_dn5 - locals.var_beta_inv_dn5) - (locals.var_xvbs * locals.var_vbsz__blk438_dn5)), ((locals.var_t1_dn6 - locals.var_beta_inv_dn6) - (locals.var_xvbs * locals.var_vbsz__blk438_dn6)), ((locals.var_t1_dn7 - locals.var_beta_inv_dn7) - (locals.var_xvbs * locals.var_vbsz__blk438_dn7)), ((locals.var_t1_dn8 - locals.var_beta_inv_dn8) - (locals.var_xvbs * locals.var_vbsz__blk438_dn8)), ((locals.var_t1_dn9 - locals.var_beta_inv_dn9) - (locals.var_xvbs * locals.var_vbsz__blk438_dn9)), ((locals.var_t1_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbsz__blk438_dn10)), ((locals.var_t1_dn13 - locals.var_beta_inv_dn13) - (locals.var_xvbs * locals.var_vbsz__blk438_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign66470_e103149;
        locals.var_t5_dn0 = assign66470_e103149_d_n0;
        locals.var_t5_dn2 = assign66470_e103149_d_n2;
        locals.var_t5_dn4 = assign66470_e103149_d_n4;
        locals.var_t5_dn5 = assign66470_e103149_d_n5;
        locals.var_t5_dn6 = assign66470_e103149_d_n6;
        locals.var_t5_dn7 = assign66470_e103149_d_n7;
        locals.var_t5_dn8 = assign66470_e103149_d_n8;
        locals.var_t5_dn9 = assign66470_e103149_d_n9;
        locals.var_t5_dn10 = assign66470_e103149_d_n10;
        locals.var_t5_dn13 = assign66470_e103149_d_n13;

        let (assign66480_e103160, assign66480_e103160_d_n0, assign66480_e103160_d_n2, assign66480_e103160_d_n4, assign66480_e103160_d_n5, assign66480_e103160_d_n6, assign66480_e103160_d_n7, assign66480_e103160_d_n8, assign66480_e103160_d_n9, assign66480_e103160_d_n10, assign66480_e103160_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66480_e103157: f64 = (locals.var_t4 * locals.var_t5);
        let assign66480_e103158: f64 = (1.0 + assign66480_e103157);
        (assign66480_e103158, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn13 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign66480_e103160;
        locals.var_t6_dn0 = assign66480_e103160_d_n0;
        locals.var_t6_dn2 = assign66480_e103160_d_n2;
        locals.var_t6_dn4 = assign66480_e103160_d_n4;
        locals.var_t6_dn5 = assign66480_e103160_d_n5;
        locals.var_t6_dn6 = assign66480_e103160_d_n6;
        locals.var_t6_dn7 = assign66480_e103160_d_n7;
        locals.var_t6_dn8 = assign66480_e103160_d_n8;
        locals.var_t6_dn9 = assign66480_e103160_d_n9;
        locals.var_t6_dn10 = assign66480_e103160_d_n10;
        locals.var_t6_dn13 = assign66480_e103160_d_n13;

    }

    pub(super) fn stamp_transient_block_227(
        locals: &mut StampLocals,
    ) {
        let (assign66490_e103171, assign66490_e103171_d_n0, assign66490_e103171_d_n2, assign66490_e103171_d_n4, assign66490_e103171_d_n5, assign66490_e103171_d_n6, assign66490_e103171_d_n7, assign66490_e103171_d_n8, assign66490_e103171_d_n9, assign66490_e103171_d_n10, assign66490_e103171_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66490_e103168: f64 = (1.0 + locals.var_t4);
        let assign66490_e103169: f64 = (2.0 * assign66490_e103168);
        (assign66490_e103169, (2.0 * locals.var_t4_dn0), (2.0 * locals.var_t4_dn2), (2.0 * locals.var_t4_dn4), (2.0 * locals.var_t4_dn5), (2.0 * locals.var_t4_dn6), (2.0 * locals.var_t4_dn7), (2.0 * locals.var_t4_dn8), (2.0 * locals.var_t4_dn9), (2.0 * locals.var_t4_dn10), (2.0 * locals.var_t4_dn13),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign66490_e103171;
        locals.var_t7_dn0 = assign66490_e103171_d_n0;
        locals.var_t7_dn2 = assign66490_e103171_d_n2;
        locals.var_t7_dn4 = assign66490_e103171_d_n4;
        locals.var_t7_dn5 = assign66490_e103171_d_n5;
        locals.var_t7_dn6 = assign66490_e103171_d_n6;
        locals.var_t7_dn7 = assign66490_e103171_d_n7;
        locals.var_t7_dn8 = assign66490_e103171_d_n8;
        locals.var_t7_dn9 = assign66490_e103171_d_n9;
        locals.var_t7_dn10 = assign66490_e103171_d_n10;
        locals.var_t7_dn13 = assign66490_e103171_d_n13;

        let assign66500_e103175: f64 = (1e-6 + locals.var_t7);
        let assign66500_e103180: f64 = if ((locals.var_t6 < assign66500_e103175) && (locals.var_t7 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1582 = assign66500_e103180;

        let (assign66510_e103193, assign66510_e103193_d_n0, assign66510_e103193_d_n2, assign66510_e103193_d_n4, assign66510_e103193_d_n5, assign66510_e103193_d_n6, assign66510_e103193_d_n7, assign66510_e103193_d_n8, assign66510_e103193_d_n9, assign66510_e103193_d_n10, assign66510_e103193_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66510_e103189: f64 = (1e-6 + locals.var_t7);
        let assign66510_e103191: f64 = (assign66510_e103189 - locals.var_t6);
        (assign66510_e103191, (locals.var_t7_dn0 - locals.var_t6_dn0), (locals.var_t7_dn2 - locals.var_t6_dn2), (locals.var_t7_dn4 - locals.var_t6_dn4), (locals.var_t7_dn5 - locals.var_t6_dn5), (locals.var_t7_dn6 - locals.var_t6_dn6), (locals.var_t7_dn7 - locals.var_t6_dn7), (locals.var_t7_dn8 - locals.var_t6_dn8), (locals.var_t7_dn9 - locals.var_t6_dn9), (locals.var_t7_dn10 - locals.var_t6_dn10), (locals.var_t7_dn13 - locals.var_t6_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign66510_e103193;
        locals.var_tmf1_dn0 = assign66510_e103193_d_n0;
        locals.var_tmf1_dn2 = assign66510_e103193_d_n2;
        locals.var_tmf1_dn4 = assign66510_e103193_d_n4;
        locals.var_tmf1_dn5 = assign66510_e103193_d_n5;
        locals.var_tmf1_dn6 = assign66510_e103193_d_n6;
        locals.var_tmf1_dn7 = assign66510_e103193_d_n7;
        locals.var_tmf1_dn8 = assign66510_e103193_d_n8;
        locals.var_tmf1_dn9 = assign66510_e103193_d_n9;
        locals.var_tmf1_dn10 = assign66510_e103193_d_n10;
        locals.var_tmf1_dn13 = assign66510_e103193_d_n13;

        let (assign66520_e103204, assign66520_e103204_d_n0, assign66520_e103204_d_n2, assign66520_e103204_d_n4, assign66520_e103204_d_n5, assign66520_e103204_d_n6, assign66520_e103204_d_n7, assign66520_e103204_d_n8, assign66520_e103204_d_n9, assign66520_e103204_d_n10, assign66520_e103204_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66520_e103202: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign66520_e103202, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign66520_e103204;
        locals.var_x2_dn0 = assign66520_e103204_d_n0;
        locals.var_x2_dn2 = assign66520_e103204_d_n2;
        locals.var_x2_dn4 = assign66520_e103204_d_n4;
        locals.var_x2_dn5 = assign66520_e103204_d_n5;
        locals.var_x2_dn6 = assign66520_e103204_d_n6;
        locals.var_x2_dn7 = assign66520_e103204_d_n7;
        locals.var_x2_dn8 = assign66520_e103204_d_n8;
        locals.var_x2_dn9 = assign66520_e103204_d_n9;
        locals.var_x2_dn10 = assign66520_e103204_d_n10;
        locals.var_x2_dn13 = assign66520_e103204_d_n13;

        let (assign66530_e103215, assign66530_e103215_d_n0, assign66530_e103215_d_n2, assign66530_e103215_d_n4, assign66530_e103215_d_n5, assign66530_e103215_d_n6, assign66530_e103215_d_n7, assign66530_e103215_d_n8, assign66530_e103215_d_n9, assign66530_e103215_d_n10, assign66530_e103215_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66530_e103213: f64 = (locals.var_t7 * locals.var_t7);
        (assign66530_e103213, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign66530_e103215;
        locals.var_xmax2_dn0 = assign66530_e103215_d_n0;
        locals.var_xmax2_dn2 = assign66530_e103215_d_n2;
        locals.var_xmax2_dn4 = assign66530_e103215_d_n4;
        locals.var_xmax2_dn5 = assign66530_e103215_d_n5;
        locals.var_xmax2_dn6 = assign66530_e103215_d_n6;
        locals.var_xmax2_dn7 = assign66530_e103215_d_n7;
        locals.var_xmax2_dn8 = assign66530_e103215_d_n8;
        locals.var_xmax2_dn9 = assign66530_e103215_d_n9;
        locals.var_xmax2_dn10 = assign66530_e103215_d_n10;
        locals.var_xmax2_dn13 = assign66530_e103215_d_n13;

        let (assign66540_e103224, assign66540_e103224_d_n0, assign66540_e103224_d_n2, assign66540_e103224_d_n4, assign66540_e103224_d_n5, assign66540_e103224_d_n6, assign66540_e103224_d_n7, assign66540_e103224_d_n8, assign66540_e103224_d_n9, assign66540_e103224_d_n10, assign66540_e103224_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66540_e103224;
        locals.var_xp_dn0 = assign66540_e103224_d_n0;
        locals.var_xp_dn2 = assign66540_e103224_d_n2;
        locals.var_xp_dn4 = assign66540_e103224_d_n4;
        locals.var_xp_dn5 = assign66540_e103224_d_n5;
        locals.var_xp_dn6 = assign66540_e103224_d_n6;
        locals.var_xp_dn7 = assign66540_e103224_d_n7;
        locals.var_xp_dn8 = assign66540_e103224_d_n8;
        locals.var_xp_dn9 = assign66540_e103224_d_n9;
        locals.var_xp_dn10 = assign66540_e103224_d_n10;
        locals.var_xp_dn13 = assign66540_e103224_d_n13;

        let (assign66550_e103233, assign66550_e103233_d_n0, assign66550_e103233_d_n2, assign66550_e103233_d_n4, assign66550_e103233_d_n5, assign66550_e103233_d_n6, assign66550_e103233_d_n7, assign66550_e103233_d_n8, assign66550_e103233_d_n9, assign66550_e103233_d_n10, assign66550_e103233_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66550_e103233;
        locals.var_xmp_dn0 = assign66550_e103233_d_n0;
        locals.var_xmp_dn2 = assign66550_e103233_d_n2;
        locals.var_xmp_dn4 = assign66550_e103233_d_n4;
        locals.var_xmp_dn5 = assign66550_e103233_d_n5;
        locals.var_xmp_dn6 = assign66550_e103233_d_n6;
        locals.var_xmp_dn7 = assign66550_e103233_d_n7;
        locals.var_xmp_dn8 = assign66550_e103233_d_n8;
        locals.var_xmp_dn9 = assign66550_e103233_d_n9;
        locals.var_xmp_dn10 = assign66550_e103233_d_n10;
        locals.var_xmp_dn13 = assign66550_e103233_d_n13;

        let (assign66560_e103242,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66560_e103242;

        let (assign66570_e103251,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66570_e103251;

        let (assign66580_e103260, assign66580_e103260_d_n0, assign66580_e103260_d_n2, assign66580_e103260_d_n4, assign66580_e103260_d_n5, assign66580_e103260_d_n6, assign66580_e103260_d_n7, assign66580_e103260_d_n8, assign66580_e103260_d_n9, assign66580_e103260_d_n10, assign66580_e103260_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign66580_e103260;
        locals.var_arg_dn0 = assign66580_e103260_d_n0;
        locals.var_arg_dn2 = assign66580_e103260_d_n2;
        locals.var_arg_dn4 = assign66580_e103260_d_n4;
        locals.var_arg_dn5 = assign66580_e103260_d_n5;
        locals.var_arg_dn6 = assign66580_e103260_d_n6;
        locals.var_arg_dn7 = assign66580_e103260_d_n7;
        locals.var_arg_dn8 = assign66580_e103260_d_n8;
        locals.var_arg_dn9 = assign66580_e103260_d_n9;
        locals.var_arg_dn10 = assign66580_e103260_d_n10;
        locals.var_arg_dn13 = assign66580_e103260_d_n13;

        let (assign66590_e103269, assign66590_e103269_d_n0, assign66590_e103269_d_n2, assign66590_e103269_d_n4, assign66590_e103269_d_n5, assign66590_e103269_d_n6, assign66590_e103269_d_n7, assign66590_e103269_d_n8, assign66590_e103269_d_n9, assign66590_e103269_d_n10, assign66590_e103269_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66590_e103269;
        locals.var_dnm_dn0 = assign66590_e103269_d_n0;
        locals.var_dnm_dn2 = assign66590_e103269_d_n2;
        locals.var_dnm_dn4 = assign66590_e103269_d_n4;
        locals.var_dnm_dn5 = assign66590_e103269_d_n5;
        locals.var_dnm_dn6 = assign66590_e103269_d_n6;
        locals.var_dnm_dn7 = assign66590_e103269_d_n7;
        locals.var_dnm_dn8 = assign66590_e103269_d_n8;
        locals.var_dnm_dn9 = assign66590_e103269_d_n9;
        locals.var_dnm_dn10 = assign66590_e103269_d_n10;
        locals.var_dnm_dn13 = assign66590_e103269_d_n13;

        let (assign66600_e103280, assign66600_e103280_d_n0, assign66600_e103280_d_n2, assign66600_e103280_d_n4, assign66600_e103280_d_n5, assign66600_e103280_d_n6, assign66600_e103280_d_n7, assign66600_e103280_d_n8, assign66600_e103280_d_n9, assign66600_e103280_d_n10, assign66600_e103280_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66600_e103278: f64 = (locals.var_xp * locals.var_x2);
        (assign66600_e103278, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66600_e103280;
        locals.var_xp_dn0 = assign66600_e103280_d_n0;
        locals.var_xp_dn2 = assign66600_e103280_d_n2;
        locals.var_xp_dn4 = assign66600_e103280_d_n4;
        locals.var_xp_dn5 = assign66600_e103280_d_n5;
        locals.var_xp_dn6 = assign66600_e103280_d_n6;
        locals.var_xp_dn7 = assign66600_e103280_d_n7;
        locals.var_xp_dn8 = assign66600_e103280_d_n8;
        locals.var_xp_dn9 = assign66600_e103280_d_n9;
        locals.var_xp_dn10 = assign66600_e103280_d_n10;
        locals.var_xp_dn13 = assign66600_e103280_d_n13;

        let (assign66610_e103291, assign66610_e103291_d_n0, assign66610_e103291_d_n2, assign66610_e103291_d_n4, assign66610_e103291_d_n5, assign66610_e103291_d_n6, assign66610_e103291_d_n7, assign66610_e103291_d_n8, assign66610_e103291_d_n9, assign66610_e103291_d_n10, assign66610_e103291_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66610_e103289: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66610_e103289, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66610_e103291;
        locals.var_xmp_dn0 = assign66610_e103291_d_n0;
        locals.var_xmp_dn2 = assign66610_e103291_d_n2;
        locals.var_xmp_dn4 = assign66610_e103291_d_n4;
        locals.var_xmp_dn5 = assign66610_e103291_d_n5;
        locals.var_xmp_dn6 = assign66610_e103291_d_n6;
        locals.var_xmp_dn7 = assign66610_e103291_d_n7;
        locals.var_xmp_dn8 = assign66610_e103291_d_n8;
        locals.var_xmp_dn9 = assign66610_e103291_d_n9;
        locals.var_xmp_dn10 = assign66610_e103291_d_n10;
        locals.var_xmp_dn13 = assign66610_e103291_d_n13;

        let (assign66620_e103302, assign66620_e103302_d_n0, assign66620_e103302_d_n2, assign66620_e103302_d_n4, assign66620_e103302_d_n5, assign66620_e103302_d_n6, assign66620_e103302_d_n7, assign66620_e103302_d_n8, assign66620_e103302_d_n9, assign66620_e103302_d_n10, assign66620_e103302_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66620_e103300: f64 = (locals.var_xp * locals.var_x2);
        (assign66620_e103300, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66620_e103302;
        locals.var_xp_dn0 = assign66620_e103302_d_n0;
        locals.var_xp_dn2 = assign66620_e103302_d_n2;
        locals.var_xp_dn4 = assign66620_e103302_d_n4;
        locals.var_xp_dn5 = assign66620_e103302_d_n5;
        locals.var_xp_dn6 = assign66620_e103302_d_n6;
        locals.var_xp_dn7 = assign66620_e103302_d_n7;
        locals.var_xp_dn8 = assign66620_e103302_d_n8;
        locals.var_xp_dn9 = assign66620_e103302_d_n9;
        locals.var_xp_dn10 = assign66620_e103302_d_n10;
        locals.var_xp_dn13 = assign66620_e103302_d_n13;

        let (assign66630_e103313, assign66630_e103313_d_n0, assign66630_e103313_d_n2, assign66630_e103313_d_n4, assign66630_e103313_d_n5, assign66630_e103313_d_n6, assign66630_e103313_d_n7, assign66630_e103313_d_n8, assign66630_e103313_d_n9, assign66630_e103313_d_n10, assign66630_e103313_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66630_e103311: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66630_e103311, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66630_e103313;
        locals.var_xmp_dn0 = assign66630_e103313_d_n0;
        locals.var_xmp_dn2 = assign66630_e103313_d_n2;
        locals.var_xmp_dn4 = assign66630_e103313_d_n4;
        locals.var_xmp_dn5 = assign66630_e103313_d_n5;
        locals.var_xmp_dn6 = assign66630_e103313_d_n6;
        locals.var_xmp_dn7 = assign66630_e103313_d_n7;
        locals.var_xmp_dn8 = assign66630_e103313_d_n8;
        locals.var_xmp_dn9 = assign66630_e103313_d_n9;
        locals.var_xmp_dn10 = assign66630_e103313_d_n10;
        locals.var_xmp_dn13 = assign66630_e103313_d_n13;

        let (assign66640_e103324, assign66640_e103324_d_n0, assign66640_e103324_d_n2, assign66640_e103324_d_n4, assign66640_e103324_d_n5, assign66640_e103324_d_n6, assign66640_e103324_d_n7, assign66640_e103324_d_n8, assign66640_e103324_d_n9, assign66640_e103324_d_n10, assign66640_e103324_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66640_e103322: f64 = (locals.var_xp * locals.var_x2);
        (assign66640_e103322, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66640_e103324;
        locals.var_xp_dn0 = assign66640_e103324_d_n0;
        locals.var_xp_dn2 = assign66640_e103324_d_n2;
        locals.var_xp_dn4 = assign66640_e103324_d_n4;
        locals.var_xp_dn5 = assign66640_e103324_d_n5;
        locals.var_xp_dn6 = assign66640_e103324_d_n6;
        locals.var_xp_dn7 = assign66640_e103324_d_n7;
        locals.var_xp_dn8 = assign66640_e103324_d_n8;
        locals.var_xp_dn9 = assign66640_e103324_d_n9;
        locals.var_xp_dn10 = assign66640_e103324_d_n10;
        locals.var_xp_dn13 = assign66640_e103324_d_n13;

        let (assign66650_e103335, assign66650_e103335_d_n0, assign66650_e103335_d_n2, assign66650_e103335_d_n4, assign66650_e103335_d_n5, assign66650_e103335_d_n6, assign66650_e103335_d_n7, assign66650_e103335_d_n8, assign66650_e103335_d_n9, assign66650_e103335_d_n10, assign66650_e103335_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66650_e103333: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66650_e103333, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66650_e103335;
        locals.var_xmp_dn0 = assign66650_e103335_d_n0;
        locals.var_xmp_dn2 = assign66650_e103335_d_n2;
        locals.var_xmp_dn4 = assign66650_e103335_d_n4;
        locals.var_xmp_dn5 = assign66650_e103335_d_n5;
        locals.var_xmp_dn6 = assign66650_e103335_d_n6;
        locals.var_xmp_dn7 = assign66650_e103335_d_n7;
        locals.var_xmp_dn8 = assign66650_e103335_d_n8;
        locals.var_xmp_dn9 = assign66650_e103335_d_n9;
        locals.var_xmp_dn10 = assign66650_e103335_d_n10;
        locals.var_xmp_dn13 = assign66650_e103335_d_n13;

        let (assign66660_e103346, assign66660_e103346_d_n0, assign66660_e103346_d_n2, assign66660_e103346_d_n4, assign66660_e103346_d_n5, assign66660_e103346_d_n6, assign66660_e103346_d_n7, assign66660_e103346_d_n8, assign66660_e103346_d_n9, assign66660_e103346_d_n10, assign66660_e103346_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66660_e103344: f64 = (locals.var_xp * locals.var_x2);
        (assign66660_e103344, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign66660_e103346;
        locals.var_xp_dn0 = assign66660_e103346_d_n0;
        locals.var_xp_dn2 = assign66660_e103346_d_n2;
        locals.var_xp_dn4 = assign66660_e103346_d_n4;
        locals.var_xp_dn5 = assign66660_e103346_d_n5;
        locals.var_xp_dn6 = assign66660_e103346_d_n6;
        locals.var_xp_dn7 = assign66660_e103346_d_n7;
        locals.var_xp_dn8 = assign66660_e103346_d_n8;
        locals.var_xp_dn9 = assign66660_e103346_d_n9;
        locals.var_xp_dn10 = assign66660_e103346_d_n10;
        locals.var_xp_dn13 = assign66660_e103346_d_n13;

        let (assign66670_e103357, assign66670_e103357_d_n0, assign66670_e103357_d_n2, assign66670_e103357_d_n4, assign66670_e103357_d_n5, assign66670_e103357_d_n6, assign66670_e103357_d_n7, assign66670_e103357_d_n8, assign66670_e103357_d_n9, assign66670_e103357_d_n10, assign66670_e103357_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66670_e103355: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign66670_e103355, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign66670_e103357;
        locals.var_xmp_dn0 = assign66670_e103357_d_n0;
        locals.var_xmp_dn2 = assign66670_e103357_d_n2;
        locals.var_xmp_dn4 = assign66670_e103357_d_n4;
        locals.var_xmp_dn5 = assign66670_e103357_d_n5;
        locals.var_xmp_dn6 = assign66670_e103357_d_n6;
        locals.var_xmp_dn7 = assign66670_e103357_d_n7;
        locals.var_xmp_dn8 = assign66670_e103357_d_n8;
        locals.var_xmp_dn9 = assign66670_e103357_d_n9;
        locals.var_xmp_dn10 = assign66670_e103357_d_n10;
        locals.var_xmp_dn13 = assign66670_e103357_d_n13;

        let (assign66680_e103368, assign66680_e103368_d_n0, assign66680_e103368_d_n2, assign66680_e103368_d_n4, assign66680_e103368_d_n5, assign66680_e103368_d_n6, assign66680_e103368_d_n7, assign66680_e103368_d_n8, assign66680_e103368_d_n9, assign66680_e103368_d_n10, assign66680_e103368_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66680_e103366: f64 = (locals.var_xp + locals.var_xmp);
        (assign66680_e103366, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign66680_e103368;
        locals.var_arg_dn0 = assign66680_e103368_d_n0;
        locals.var_arg_dn2 = assign66680_e103368_d_n2;
        locals.var_arg_dn4 = assign66680_e103368_d_n4;
        locals.var_arg_dn5 = assign66680_e103368_d_n5;
        locals.var_arg_dn6 = assign66680_e103368_d_n6;
        locals.var_arg_dn7 = assign66680_e103368_d_n7;
        locals.var_arg_dn8 = assign66680_e103368_d_n8;
        locals.var_arg_dn9 = assign66680_e103368_d_n9;
        locals.var_arg_dn10 = assign66680_e103368_d_n10;
        locals.var_arg_dn13 = assign66680_e103368_d_n13;

        let (assign66690_e103377, assign66690_e103377_d_n0, assign66690_e103377_d_n2, assign66690_e103377_d_n4, assign66690_e103377_d_n5, assign66690_e103377_d_n6, assign66690_e103377_d_n7, assign66690_e103377_d_n8, assign66690_e103377_d_n9, assign66690_e103377_d_n10, assign66690_e103377_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66690_e103377;
        locals.var_dnm_dn0 = assign66690_e103377_d_n0;
        locals.var_dnm_dn2 = assign66690_e103377_d_n2;
        locals.var_dnm_dn4 = assign66690_e103377_d_n4;
        locals.var_dnm_dn5 = assign66690_e103377_d_n5;
        locals.var_dnm_dn6 = assign66690_e103377_d_n6;
        locals.var_dnm_dn7 = assign66690_e103377_d_n7;
        locals.var_dnm_dn8 = assign66690_e103377_d_n8;
        locals.var_dnm_dn9 = assign66690_e103377_d_n9;
        locals.var_dnm_dn10 = assign66690_e103377_d_n10;
        locals.var_dnm_dn13 = assign66690_e103377_d_n13;

        let assign66700_e103392: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1583 = assign66700_e103392;

        let assign66710_e103395: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1584 = assign66710_e103395;

        let (assign66720_e103408,) = {
    if (((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66720_e103408;

        let assign66730_e103411: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1585 = assign66730_e103411;

        let (assign66740_e103427,) = {
    if ((((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 == 0.0)) && (locals.var_guard1585 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66740_e103427;

        let assign66750_e103430: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1586 = assign66750_e103430;

        let (assign66760_e103449,) = {
    if (((((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 == 0.0)) && (locals.var_guard1585 == 0.0)) && (locals.var_guard1586 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66760_e103449;

        let assign66770_e103452: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1587 = assign66770_e103452;

        let (assign66780_e103474,) = {
    if ((((((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_guard1584 == 0.0)) && (locals.var_guard1585 == 0.0)) && (locals.var_guard1586 == 0.0)) && (locals.var_guard1587 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign66780_e103474;

        let (assign66790_e103485,) = {
    if ((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign66790_e103485;

        let mut assign66800_loop_guard: usize = 0;
        while {
            let assign66800_cond_e103497: f64 = if (((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign66800_cond_e103497 != 0.0
        } {
            assign66800_loop_guard += 1;
            assert!(assign66800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign66800_body0_e103509, assign66800_body0_e103509_d_n0, assign66800_body0_e103509_d_n2, assign66800_body0_e103509_d_n4, assign66800_body0_e103509_d_n5, assign66800_body0_e103509_d_n6, assign66800_body0_e103509_d_n7, assign66800_body0_e103509_d_n8, assign66800_body0_e103509_d_n9, assign66800_body0_e103509_d_n10, assign66800_body0_e103509_d_n13,) = {
    if ((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) {
        let assign66800_body0_e103507: f64 = (locals.var_dnm).sqrt();
        (assign66800_body0_e103507, (locals.var_dnm_dn0 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn2 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn4 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn5 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn6 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn7 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn8 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn9 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn10 / (2.0 * assign66800_body0_e103507)), (locals.var_dnm_dn13 / (2.0 * assign66800_body0_e103507)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign66800_body0_e103509;
            locals.var_dnm_dn0 = assign66800_body0_e103509_d_n0;
            locals.var_dnm_dn2 = assign66800_body0_e103509_d_n2;
            locals.var_dnm_dn4 = assign66800_body0_e103509_d_n4;
            locals.var_dnm_dn5 = assign66800_body0_e103509_d_n5;
            locals.var_dnm_dn6 = assign66800_body0_e103509_d_n6;
            locals.var_dnm_dn7 = assign66800_body0_e103509_d_n7;
            locals.var_dnm_dn8 = assign66800_body0_e103509_d_n8;
            locals.var_dnm_dn9 = assign66800_body0_e103509_d_n9;
            locals.var_dnm_dn10 = assign66800_body0_e103509_d_n10;
            locals.var_dnm_dn13 = assign66800_body0_e103509_d_n13;
            let (assign66800_body1_e103522,) = {
    if ((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 != 0.0)) {
        let assign66800_body1_e103520: f64 = (locals.var_m0 + 1.0);
        (assign66800_body1_e103520,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign66800_body1_e103522;
        }

        let (assign66810_e103545, assign66810_e103545_d_n0, assign66810_e103545_d_n2, assign66810_e103545_d_n4, assign66810_e103545_d_n5, assign66810_e103545_d_n6, assign66810_e103545_d_n7, assign66810_e103545_d_n8, assign66810_e103545_d_n9, assign66810_e103545_d_n10, assign66810_e103545_d_n13,) = {
    if ((((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) && (locals.var_guard1583 == 0.0)) {
        let (assign66810_e103543, assign66810_e103543_d_n0, assign66810_e103543_d_n2, assign66810_e103543_d_n4, assign66810_e103543_d_n5, assign66810_e103543_d_n6, assign66810_e103543_d_n7, assign66810_e103543_d_n8, assign66810_e103543_d_n9, assign66810_e103543_d_n10, assign66810_e103543_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign66810_e103540: f64 = (2.0 * 4.0);
                let assign66810_e103541: f64 = (1.0 / assign66810_e103540);
                let assign66810_e103542: f64 = (locals.var_dnm).powf(assign66810_e103541);
                (assign66810_e103542, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn0)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn2)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn4)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn5)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn6)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn7)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn8)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn9)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn10)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign66810_e103541) as f64).is_finite() && ((assign66810_e103541) as f64).fract() == 0.0 { if assign66810_e103541 == 0.0 { 0.0 } else { (assign66810_e103541 * ((locals.var_dnm).powf(assign66810_e103541 - 1.0) * locals.var_dnm_dn13)) } } else { (assign66810_e103542 * (assign66810_e103541 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign66810_e103543, assign66810_e103543_d_n0, assign66810_e103543_d_n2, assign66810_e103543_d_n4, assign66810_e103543_d_n5, assign66810_e103543_d_n6, assign66810_e103543_d_n7, assign66810_e103543_d_n8, assign66810_e103543_d_n9, assign66810_e103543_d_n10, assign66810_e103543_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66810_e103545;
        locals.var_dnm_dn0 = assign66810_e103545_d_n0;
        locals.var_dnm_dn2 = assign66810_e103545_d_n2;
        locals.var_dnm_dn4 = assign66810_e103545_d_n4;
        locals.var_dnm_dn5 = assign66810_e103545_d_n5;
        locals.var_dnm_dn6 = assign66810_e103545_d_n6;
        locals.var_dnm_dn7 = assign66810_e103545_d_n7;
        locals.var_dnm_dn8 = assign66810_e103545_d_n8;
        locals.var_dnm_dn9 = assign66810_e103545_d_n9;
        locals.var_dnm_dn10 = assign66810_e103545_d_n10;
        locals.var_dnm_dn13 = assign66810_e103545_d_n13;

    }

    pub(super) fn stamp_transient_block_228(
        locals: &mut StampLocals,
    ) {
        let (assign66820_e103556, assign66820_e103556_d_n0, assign66820_e103556_d_n2, assign66820_e103556_d_n4, assign66820_e103556_d_n5, assign66820_e103556_d_n6, assign66820_e103556_d_n7, assign66820_e103556_d_n8, assign66820_e103556_d_n9, assign66820_e103556_d_n10, assign66820_e103556_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66820_e103554: f64 = (1.0 / locals.var_dnm);
        (assign66820_e103554, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign66820_e103556;
        locals.var_dnm_dn0 = assign66820_e103556_d_n0;
        locals.var_dnm_dn2 = assign66820_e103556_d_n2;
        locals.var_dnm_dn4 = assign66820_e103556_d_n4;
        locals.var_dnm_dn5 = assign66820_e103556_d_n5;
        locals.var_dnm_dn6 = assign66820_e103556_d_n6;
        locals.var_dnm_dn7 = assign66820_e103556_d_n7;
        locals.var_dnm_dn8 = assign66820_e103556_d_n8;
        locals.var_dnm_dn9 = assign66820_e103556_d_n9;
        locals.var_dnm_dn10 = assign66820_e103556_d_n10;
        locals.var_dnm_dn13 = assign66820_e103556_d_n13;

        let (assign66830_e103569, assign66830_e103569_d_n0, assign66830_e103569_d_n2, assign66830_e103569_d_n4, assign66830_e103569_d_n5, assign66830_e103569_d_n6, assign66830_e103569_d_n7, assign66830_e103569_d_n8, assign66830_e103569_d_n9, assign66830_e103569_d_n10, assign66830_e103569_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66830_e103565: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign66830_e103567: f64 = (assign66830_e103565 * locals.var_dnm);
        (assign66830_e103567, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn13)) * locals.var_dnm) + (assign66830_e103565 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign66830_e103569;
        locals.var_tmf0_dn0 = assign66830_e103569_d_n0;
        locals.var_tmf0_dn2 = assign66830_e103569_d_n2;
        locals.var_tmf0_dn4 = assign66830_e103569_d_n4;
        locals.var_tmf0_dn5 = assign66830_e103569_d_n5;
        locals.var_tmf0_dn6 = assign66830_e103569_d_n6;
        locals.var_tmf0_dn7 = assign66830_e103569_d_n7;
        locals.var_tmf0_dn8 = assign66830_e103569_d_n8;
        locals.var_tmf0_dn9 = assign66830_e103569_d_n9;
        locals.var_tmf0_dn10 = assign66830_e103569_d_n10;
        locals.var_tmf0_dn13 = assign66830_e103569_d_n13;

        let (assign66840_e103584, assign66840_e103584_d_n0, assign66840_e103584_d_n2, assign66840_e103584_d_n4, assign66840_e103584_d_n5, assign66840_e103584_d_n6, assign66840_e103584_d_n7, assign66840_e103584_d_n8, assign66840_e103584_d_n9, assign66840_e103584_d_n10, assign66840_e103584_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66840_e103578: f64 = (locals.var_t7 * locals.var_xmp);
        let assign66840_e103580: f64 = (assign66840_e103578 * locals.var_dnm);
        let assign66840_e103582: f64 = (assign66840_e103580 / locals.var_arg);
        (assign66840_e103582, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn0)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn2)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn4)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn5)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn6)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn7)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn8)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn9)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn10)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn13 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign66840_e103578 * locals.var_dnm_dn13)) * locals.var_arg) - (assign66840_e103580 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66840_e103584;
        locals.var_t0_dn0 = assign66840_e103584_d_n0;
        locals.var_t0_dn2 = assign66840_e103584_d_n2;
        locals.var_t0_dn4 = assign66840_e103584_d_n4;
        locals.var_t0_dn5 = assign66840_e103584_d_n5;
        locals.var_t0_dn6 = assign66840_e103584_d_n6;
        locals.var_t0_dn7 = assign66840_e103584_d_n7;
        locals.var_t0_dn8 = assign66840_e103584_d_n8;
        locals.var_t0_dn9 = assign66840_e103584_d_n9;
        locals.var_t0_dn10 = assign66840_e103584_d_n10;
        locals.var_t0_dn13 = assign66840_e103584_d_n13;

        let (assign66850_e103597, assign66850_e103597_d_n0, assign66850_e103597_d_n2, assign66850_e103597_d_n4, assign66850_e103597_d_n5, assign66850_e103597_d_n6, assign66850_e103597_d_n7, assign66850_e103597_d_n8, assign66850_e103597_d_n9, assign66850_e103597_d_n10, assign66850_e103597_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        let assign66850_e103593: f64 = (1e-6 + locals.var_t7);
        let assign66850_e103595: f64 = (assign66850_e103593 - locals.var_tmf0);
        (assign66850_e103595, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign66850_e103597;
        locals.var_t6_dn0 = assign66850_e103597_d_n0;
        locals.var_t6_dn2 = assign66850_e103597_d_n2;
        locals.var_t6_dn4 = assign66850_e103597_d_n4;
        locals.var_t6_dn5 = assign66850_e103597_d_n5;
        locals.var_t6_dn6 = assign66850_e103597_d_n6;
        locals.var_t6_dn7 = assign66850_e103597_d_n7;
        locals.var_t6_dn8 = assign66850_e103597_d_n8;
        locals.var_t6_dn9 = assign66850_e103597_d_n9;
        locals.var_t6_dn10 = assign66850_e103597_d_n10;
        locals.var_t6_dn13 = assign66850_e103597_d_n13;

        let (assign66860_e103606, assign66860_e103606_d_n0, assign66860_e103606_d_n2, assign66860_e103606_d_n4, assign66860_e103606_d_n5, assign66860_e103606_d_n6, assign66860_e103606_d_n7, assign66860_e103606_d_n8, assign66860_e103606_d_n9, assign66860_e103606_d_n10, assign66860_e103606_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66860_e103606;
        locals.var_t0_dn0 = assign66860_e103606_d_n0;
        locals.var_t0_dn2 = assign66860_e103606_d_n2;
        locals.var_t0_dn4 = assign66860_e103606_d_n4;
        locals.var_t0_dn5 = assign66860_e103606_d_n5;
        locals.var_t0_dn6 = assign66860_e103606_d_n6;
        locals.var_t0_dn7 = assign66860_e103606_d_n7;
        locals.var_t0_dn8 = assign66860_e103606_d_n8;
        locals.var_t0_dn9 = assign66860_e103606_d_n9;
        locals.var_t0_dn10 = assign66860_e103606_d_n10;
        locals.var_t0_dn13 = assign66860_e103606_d_n13;

        let (assign66870_e103616, assign66870_e103616_d_n0, assign66870_e103616_d_n2, assign66870_e103616_d_n4, assign66870_e103616_d_n5, assign66870_e103616_d_n6, assign66870_e103616_d_n7, assign66870_e103616_d_n8, assign66870_e103616_d_n9, assign66870_e103616_d_n10, assign66870_e103616_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign66870_e103616;
        locals.var_t6_dn0 = assign66870_e103616_d_n0;
        locals.var_t6_dn2 = assign66870_e103616_d_n2;
        locals.var_t6_dn4 = assign66870_e103616_d_n4;
        locals.var_t6_dn5 = assign66870_e103616_d_n5;
        locals.var_t6_dn6 = assign66870_e103616_d_n6;
        locals.var_t6_dn7 = assign66870_e103616_d_n7;
        locals.var_t6_dn8 = assign66870_e103616_d_n8;
        locals.var_t6_dn9 = assign66870_e103616_d_n9;
        locals.var_t6_dn10 = assign66870_e103616_d_n10;
        locals.var_t6_dn13 = assign66870_e103616_d_n13;

        let (assign66880_e103626, assign66880_e103626_d_n0, assign66880_e103626_d_n2, assign66880_e103626_d_n4, assign66880_e103626_d_n5, assign66880_e103626_d_n6, assign66880_e103626_d_n7, assign66880_e103626_d_n8, assign66880_e103626_d_n9, assign66880_e103626_d_n10, assign66880_e103626_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1582 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign66880_e103626;
        locals.var_t0_dn0 = assign66880_e103626_d_n0;
        locals.var_t0_dn2 = assign66880_e103626_d_n2;
        locals.var_t0_dn4 = assign66880_e103626_d_n4;
        locals.var_t0_dn5 = assign66880_e103626_d_n5;
        locals.var_t0_dn6 = assign66880_e103626_d_n6;
        locals.var_t0_dn7 = assign66880_e103626_d_n7;
        locals.var_t0_dn8 = assign66880_e103626_d_n8;
        locals.var_t0_dn9 = assign66880_e103626_d_n9;
        locals.var_t0_dn10 = assign66880_e103626_d_n10;
        locals.var_t0_dn13 = assign66880_e103626_d_n13;

        let (assign66890_e103634, assign66890_e103634_d_n0, assign66890_e103634_d_n2, assign66890_e103634_d_n4, assign66890_e103634_d_n5, assign66890_e103634_d_n6, assign66890_e103634_d_n7, assign66890_e103634_d_n8, assign66890_e103634_d_n9, assign66890_e103634_d_n10, assign66890_e103634_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66890_e103632: f64 = (locals.var_t6).sqrt();
        (assign66890_e103632, (locals.var_t6_dn0 / (2.0 * assign66890_e103632)), (locals.var_t6_dn2 / (2.0 * assign66890_e103632)), (locals.var_t6_dn4 / (2.0 * assign66890_e103632)), (locals.var_t6_dn5 / (2.0 * assign66890_e103632)), (locals.var_t6_dn6 / (2.0 * assign66890_e103632)), (locals.var_t6_dn7 / (2.0 * assign66890_e103632)), (locals.var_t6_dn8 / (2.0 * assign66890_e103632)), (locals.var_t6_dn9 / (2.0 * assign66890_e103632)), (locals.var_t6_dn10 / (2.0 * assign66890_e103632)), (locals.var_t6_dn13 / (2.0 * assign66890_e103632)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign66890_e103634;
        locals.var_t6_dn0 = assign66890_e103634_d_n0;
        locals.var_t6_dn2 = assign66890_e103634_d_n2;
        locals.var_t6_dn4 = assign66890_e103634_d_n4;
        locals.var_t6_dn5 = assign66890_e103634_d_n5;
        locals.var_t6_dn6 = assign66890_e103634_d_n6;
        locals.var_t6_dn7 = assign66890_e103634_d_n7;
        locals.var_t6_dn8 = assign66890_e103634_d_n8;
        locals.var_t6_dn9 = assign66890_e103634_d_n9;
        locals.var_t6_dn10 = assign66890_e103634_d_n10;
        locals.var_t6_dn13 = assign66890_e103634_d_n13;

        let (assign66900_e103647, assign66900_e103647_d_n0, assign66900_e103647_d_n2, assign66900_e103647_d_n4, assign66900_e103647_d_n5, assign66900_e103647_d_n6, assign66900_e103647_d_n7, assign66900_e103647_d_n8, assign66900_e103647_d_n9, assign66900_e103647_d_n10, assign66900_e103647_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66900_e103643: f64 = (1.0 - locals.var_t6);
        let assign66900_e103644: f64 = (locals.var_t3 * assign66900_e103643);
        let assign66900_e103645: f64 = (locals.var_t1 + assign66900_e103644);
        (assign66900_e103645, (locals.var_t1_dn0 + ((locals.var_t3_dn0 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn0)))), (locals.var_t1_dn2 + ((locals.var_t3_dn2 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn2)))), (locals.var_t1_dn4 + ((locals.var_t3_dn4 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn4)))), (locals.var_t1_dn5 + ((locals.var_t3_dn5 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn5)))), (locals.var_t1_dn6 + ((locals.var_t3_dn6 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn6)))), (locals.var_t1_dn7 + ((locals.var_t3_dn7 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn7)))), (locals.var_t1_dn8 + ((locals.var_t3_dn8 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn8)))), (locals.var_t1_dn9 + ((locals.var_t3_dn9 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn9)))), (locals.var_t1_dn10 + ((locals.var_t3_dn10 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn10)))), (locals.var_t1_dn13 + ((locals.var_t3_dn13 * assign66900_e103643) + (locals.var_t3 * (-locals.var_t6_dn13)))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn7, locals.var_psislsat_dn8, locals.var_psislsat_dn9, locals.var_psislsat_dn10, locals.var_psislsat_dn13,)
    }
};
        locals.var_psislsat = assign66900_e103647;
        locals.var_psislsat_dn0 = assign66900_e103647_d_n0;
        locals.var_psislsat_dn2 = assign66900_e103647_d_n2;
        locals.var_psislsat_dn4 = assign66900_e103647_d_n4;
        locals.var_psislsat_dn5 = assign66900_e103647_d_n5;
        locals.var_psislsat_dn6 = assign66900_e103647_d_n6;
        locals.var_psislsat_dn7 = assign66900_e103647_d_n7;
        locals.var_psislsat_dn8 = assign66900_e103647_d_n8;
        locals.var_psislsat_dn9 = assign66900_e103647_d_n9;
        locals.var_psislsat_dn10 = assign66900_e103647_d_n10;
        locals.var_psislsat_dn13 = assign66900_e103647_d_n13;

        let (assign66910_e103658, assign66910_e103658_d_n0, assign66910_e103658_d_n2, assign66910_e103658_d_n4, assign66910_e103658_d_n5, assign66910_e103658_d_n6, assign66910_e103658_d_n7, assign66910_e103658_d_n8, assign66910_e103658_d_n9, assign66910_e103658_d_n10, assign66910_e103658_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66910_e103655: f64 = (locals.var_xgate + locals.var_lgate);
        let assign66910_e103656: f64 = (locals.var_lgate / assign66910_e103655);
        (assign66910_e103656, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign66910_e103658;
        locals.var_t2_dn0 = assign66910_e103658_d_n0;
        locals.var_t2_dn2 = assign66910_e103658_d_n2;
        locals.var_t2_dn4 = assign66910_e103658_d_n4;
        locals.var_t2_dn5 = assign66910_e103658_d_n5;
        locals.var_t2_dn6 = assign66910_e103658_d_n6;
        locals.var_t2_dn7 = assign66910_e103658_d_n7;
        locals.var_t2_dn8 = assign66910_e103658_d_n8;
        locals.var_t2_dn9 = assign66910_e103658_d_n9;
        locals.var_t2_dn10 = assign66910_e103658_d_n10;
        locals.var_t2_dn13 = assign66910_e103658_d_n13;

        let (assign66920_e103673, assign66920_e103673_d_n0, assign66920_e103673_d_n2, assign66920_e103673_d_n4, assign66920_e103673_d_n5, assign66920_e103673_d_n6, assign66920_e103673_d_n7, assign66920_e103673_d_n8, assign66920_e103673_d_n9, assign66920_e103673_d_n10, assign66920_e103673_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66920_e103665: f64 = (locals.var_uc_svds * locals.var_vdsz__blk439);
        let assign66920_e103667: f64 = (assign66920_e103665 + locals.var_ps0z);
        let assign66920_e103670: f64 = (locals.var_t2 * locals.var_psislsat);
        let assign66920_e103671: f64 = (assign66920_e103667 - assign66920_e103670);
        (assign66920_e103671, (((locals.var_uc_svds * locals.var_vdsz__blk439_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2_dn0 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn0))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2_dn2 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn2))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn4) + locals.var_ps0z_dn4) - ((locals.var_t2_dn4 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn4))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn5) + locals.var_ps0z_dn5) - ((locals.var_t2_dn5 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn5))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2_dn6 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn6))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2_dn7 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn7))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn8) + locals.var_ps0z_dn8) - ((locals.var_t2_dn8 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn8))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn9) + locals.var_ps0z_dn9) - ((locals.var_t2_dn9 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn9))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2_dn10 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn10))), (((locals.var_uc_svds * locals.var_vdsz__blk439_dn13) + locals.var_ps0z_dn13) - ((locals.var_t2_dn13 * locals.var_psislsat) + (locals.var_t2 * locals.var_psislsat_dn13))),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign66920_e103673;
        locals.var_psisubsat_dn0 = assign66920_e103673_d_n0;
        locals.var_psisubsat_dn2 = assign66920_e103673_d_n2;
        locals.var_psisubsat_dn4 = assign66920_e103673_d_n4;
        locals.var_psisubsat_dn5 = assign66920_e103673_d_n5;
        locals.var_psisubsat_dn6 = assign66920_e103673_d_n6;
        locals.var_psisubsat_dn7 = assign66920_e103673_d_n7;
        locals.var_psisubsat_dn8 = assign66920_e103673_d_n8;
        locals.var_psisubsat_dn9 = assign66920_e103673_d_n9;
        locals.var_psisubsat_dn10 = assign66920_e103673_d_n10;
        locals.var_psisubsat_dn13 = assign66920_e103673_d_n13;

        let (assign66930_e103689, assign66930_e103689_d_n0, assign66930_e103689_d_n2, assign66930_e103689_d_n4, assign66930_e103689_d_n5, assign66930_e103689_d_n6, assign66930_e103689_d_n7, assign66930_e103689_d_n8, assign66930_e103689_d_n9, assign66930_e103689_d_n10, assign66930_e103689_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66930_e103680: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign66930_e103683: f64 = (4.0 * 0.001);
        let assign66930_e103685: f64 = (assign66930_e103683 * 0.001);
        let assign66930_e103686: f64 = (assign66930_e103680 + assign66930_e103685);
        let assign66930_e103687: f64 = (assign66930_e103686).sqrt();
        (assign66930_e103687, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn7 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn7)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn9 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn9)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign66930_e103687)), (((locals.var_psisubsat_dn13 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn13)) / (2.0 * assign66930_e103687)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign66930_e103689;
        locals.var_tmf2_dn0 = assign66930_e103689_d_n0;
        locals.var_tmf2_dn2 = assign66930_e103689_d_n2;
        locals.var_tmf2_dn4 = assign66930_e103689_d_n4;
        locals.var_tmf2_dn5 = assign66930_e103689_d_n5;
        locals.var_tmf2_dn6 = assign66930_e103689_d_n6;
        locals.var_tmf2_dn7 = assign66930_e103689_d_n7;
        locals.var_tmf2_dn8 = assign66930_e103689_d_n8;
        locals.var_tmf2_dn9 = assign66930_e103689_d_n9;
        locals.var_tmf2_dn10 = assign66930_e103689_d_n10;
        locals.var_tmf2_dn13 = assign66930_e103689_d_n13;

        let (assign66940_e103702, assign66940_e103702_d_n0, assign66940_e103702_d_n2, assign66940_e103702_d_n4, assign66940_e103702_d_n5, assign66940_e103702_d_n6, assign66940_e103702_d_n7, assign66940_e103702_d_n8, assign66940_e103702_d_n9, assign66940_e103702_d_n10, assign66940_e103702_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66940_e103698: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign66940_e103699: f64 = (1.0 + assign66940_e103698);
        let assign66940_e103700: f64 = (0.5 * assign66940_e103699);
        (assign66940_e103700, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn7 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn9 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn13 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign66940_e103702;
        locals.var_t9_dn0 = assign66940_e103702_d_n0;
        locals.var_t9_dn2 = assign66940_e103702_d_n2;
        locals.var_t9_dn4 = assign66940_e103702_d_n4;
        locals.var_t9_dn5 = assign66940_e103702_d_n5;
        locals.var_t9_dn6 = assign66940_e103702_d_n6;
        locals.var_t9_dn7 = assign66940_e103702_d_n7;
        locals.var_t9_dn8 = assign66940_e103702_d_n8;
        locals.var_t9_dn9 = assign66940_e103702_d_n9;
        locals.var_t9_dn10 = assign66940_e103702_d_n10;
        locals.var_t9_dn13 = assign66940_e103702_d_n13;

        let (assign66950_e103713, assign66950_e103713_d_n0, assign66950_e103713_d_n2, assign66950_e103713_d_n4, assign66950_e103713_d_n5, assign66950_e103713_d_n6, assign66950_e103713_d_n7, assign66950_e103713_d_n8, assign66950_e103713_d_n9, assign66950_e103713_d_n10, assign66950_e103713_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66950_e103710: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign66950_e103711: f64 = (0.5 * assign66950_e103710);
        (assign66950_e103711, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign66950_e103713;
        locals.var_psisubsat_dn0 = assign66950_e103713_d_n0;
        locals.var_psisubsat_dn2 = assign66950_e103713_d_n2;
        locals.var_psisubsat_dn4 = assign66950_e103713_d_n4;
        locals.var_psisubsat_dn5 = assign66950_e103713_d_n5;
        locals.var_psisubsat_dn6 = assign66950_e103713_d_n6;
        locals.var_psisubsat_dn7 = assign66950_e103713_d_n7;
        locals.var_psisubsat_dn8 = assign66950_e103713_d_n8;
        locals.var_psisubsat_dn9 = assign66950_e103713_d_n9;
        locals.var_psisubsat_dn10 = assign66950_e103713_d_n10;
        locals.var_psisubsat_dn13 = assign66950_e103713_d_n13;

        let assign66960_e103716: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1588 = assign66960_e103716;

        let (assign66970_e103725, assign66970_e103725_d_n0, assign66970_e103725_d_n2, assign66970_e103725_d_n4, assign66970_e103725_d_n5, assign66970_e103725_d_n6, assign66970_e103725_d_n7, assign66970_e103725_d_n8, assign66970_e103725_d_n9, assign66970_e103725_d_n10, assign66970_e103725_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1588 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign66970_e103725;
        locals.var_psisubsat_dn0 = assign66970_e103725_d_n0;
        locals.var_psisubsat_dn2 = assign66970_e103725_d_n2;
        locals.var_psisubsat_dn4 = assign66970_e103725_d_n4;
        locals.var_psisubsat_dn5 = assign66970_e103725_d_n5;
        locals.var_psisubsat_dn6 = assign66970_e103725_d_n6;
        locals.var_psisubsat_dn7 = assign66970_e103725_d_n7;
        locals.var_psisubsat_dn8 = assign66970_e103725_d_n8;
        locals.var_psisubsat_dn9 = assign66970_e103725_d_n9;
        locals.var_psisubsat_dn10 = assign66970_e103725_d_n10;
        locals.var_psisubsat_dn13 = assign66970_e103725_d_n13;

        let (assign66980_e103734, assign66980_e103734_d_n0, assign66980_e103734_d_n2, assign66980_e103734_d_n4, assign66980_e103734_d_n5, assign66980_e103734_d_n6, assign66980_e103734_d_n7, assign66980_e103734_d_n8, assign66980_e103734_d_n9, assign66980_e103734_d_n10, assign66980_e103734_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) && (locals.var_guard1588 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign66980_e103734;
        locals.var_t9_dn0 = assign66980_e103734_d_n0;
        locals.var_t9_dn2 = assign66980_e103734_d_n2;
        locals.var_t9_dn4 = assign66980_e103734_d_n4;
        locals.var_t9_dn5 = assign66980_e103734_d_n5;
        locals.var_t9_dn6 = assign66980_e103734_d_n6;
        locals.var_t9_dn7 = assign66980_e103734_d_n7;
        locals.var_t9_dn8 = assign66980_e103734_d_n8;
        locals.var_t9_dn9 = assign66980_e103734_d_n9;
        locals.var_t9_dn10 = assign66980_e103734_d_n10;
        locals.var_t9_dn13 = assign66980_e103734_d_n13;

        let (assign66990_e103743, assign66990_e103743_d_n0, assign66990_e103743_d_n2, assign66990_e103743_d_n4, assign66990_e103743_d_n5, assign66990_e103743_d_n6, assign66990_e103743_d_n7, assign66990_e103743_d_n8, assign66990_e103743_d_n9, assign66990_e103743_d_n10, assign66990_e103743_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign66990_e103741: f64 = (locals.var_psisubsat + 1e-25);
        (assign66990_e103741, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn7, locals.var_psisubsat_dn8, locals.var_psisubsat_dn9, locals.var_psisubsat_dn10, locals.var_psisubsat_dn13,)
    }
};
        locals.var_psisubsat = assign66990_e103743;
        locals.var_psisubsat_dn0 = assign66990_e103743_d_n0;
        locals.var_psisubsat_dn2 = assign66990_e103743_d_n2;
        locals.var_psisubsat_dn4 = assign66990_e103743_d_n4;
        locals.var_psisubsat_dn5 = assign66990_e103743_d_n5;
        locals.var_psisubsat_dn6 = assign66990_e103743_d_n6;
        locals.var_psisubsat_dn7 = assign66990_e103743_d_n7;
        locals.var_psisubsat_dn8 = assign66990_e103743_d_n8;
        locals.var_psisubsat_dn9 = assign66990_e103743_d_n9;
        locals.var_psisubsat_dn10 = assign66990_e103743_d_n10;
        locals.var_psisubsat_dn13 = assign66990_e103743_d_n13;

        let (assign67000_e103756, assign67000_e103756_d_n0, assign67000_e103756_d_n2, assign67000_e103756_d_n4, assign67000_e103756_d_n5, assign67000_e103756_d_n6, assign67000_e103756_d_n7, assign67000_e103756_d_n8, assign67000_e103756_d_n9, assign67000_e103756_d_n10, assign67000_e103756_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67000_e103752: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign67000_e103753: f64 = (locals.var_uc_subtmp * assign67000_e103752);
        let assign67000_e103754: f64 = (1.0 + assign67000_e103753);
        (assign67000_e103754, (locals.var_uc_subtmp * locals.var_ttemp_dn0), (locals.var_uc_subtmp * locals.var_ttemp_dn2), (locals.var_uc_subtmp * locals.var_ttemp_dn4), (locals.var_uc_subtmp * locals.var_ttemp_dn5), (locals.var_uc_subtmp * locals.var_ttemp_dn6), (locals.var_uc_subtmp * locals.var_ttemp_dn7), (locals.var_uc_subtmp * locals.var_ttemp_dn8), (locals.var_uc_subtmp * locals.var_ttemp_dn9), (locals.var_uc_subtmp * locals.var_ttemp_dn10), (locals.var_uc_subtmp * locals.var_ttemp_dn13),)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
    }
};
        locals.var_xsubtmp = assign67000_e103756;
        locals.var_xsubtmp_dn0 = assign67000_e103756_d_n0;
        locals.var_xsubtmp_dn2 = assign67000_e103756_d_n2;
        locals.var_xsubtmp_dn4 = assign67000_e103756_d_n4;
        locals.var_xsubtmp_dn5 = assign67000_e103756_d_n5;
        locals.var_xsubtmp_dn6 = assign67000_e103756_d_n6;
        locals.var_xsubtmp_dn7 = assign67000_e103756_d_n7;
        locals.var_xsubtmp_dn8 = assign67000_e103756_d_n8;
        locals.var_xsubtmp_dn9 = assign67000_e103756_d_n9;
        locals.var_xsubtmp_dn10 = assign67000_e103756_d_n10;
        locals.var_xsubtmp_dn13 = assign67000_e103756_d_n13;

        let (assign67010_e103768, assign67010_e103768_d_n0, assign67010_e103768_d_n2, assign67010_e103768_d_n4, assign67010_e103768_d_n5, assign67010_e103768_d_n6, assign67010_e103768_d_n7, assign67010_e103768_d_n8, assign67010_e103768_d_n9, assign67010_e103768_d_n10, assign67010_e103768_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let (assign67010_e103766, assign67010_e103766_d_n0, assign67010_e103766_d_n2, assign67010_e103766_d_n4, assign67010_e103766_d_n5, assign67010_e103766_d_n6, assign67010_e103766_d_n7, assign67010_e103766_d_n8, assign67010_e103766_d_n9, assign67010_e103766_d_n10, assign67010_e103766_d_n13,) = {
            if (locals.var_xsubtmp <= 0.001) {
                (0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
            }
        };
        (assign67010_e103766, assign67010_e103766_d_n0, assign67010_e103766_d_n2, assign67010_e103766_d_n4, assign67010_e103766_d_n5, assign67010_e103766_d_n6, assign67010_e103766_d_n7, assign67010_e103766_d_n8, assign67010_e103766_d_n9, assign67010_e103766_d_n10, assign67010_e103766_d_n13,)
    } else {
        (locals.var_xsubtmp, locals.var_xsubtmp_dn0, locals.var_xsubtmp_dn2, locals.var_xsubtmp_dn4, locals.var_xsubtmp_dn5, locals.var_xsubtmp_dn6, locals.var_xsubtmp_dn7, locals.var_xsubtmp_dn8, locals.var_xsubtmp_dn9, locals.var_xsubtmp_dn10, locals.var_xsubtmp_dn13,)
    }
};
        locals.var_xsubtmp = assign67010_e103768;
        locals.var_xsubtmp_dn0 = assign67010_e103768_d_n0;
        locals.var_xsubtmp_dn2 = assign67010_e103768_d_n2;
        locals.var_xsubtmp_dn4 = assign67010_e103768_d_n4;
        locals.var_xsubtmp_dn5 = assign67010_e103768_d_n5;
        locals.var_xsubtmp_dn6 = assign67010_e103768_d_n6;
        locals.var_xsubtmp_dn7 = assign67010_e103768_d_n7;
        locals.var_xsubtmp_dn8 = assign67010_e103768_d_n8;
        locals.var_xsubtmp_dn9 = assign67010_e103768_d_n9;
        locals.var_xsubtmp_dn10 = assign67010_e103768_d_n10;
        locals.var_xsubtmp_dn13 = assign67010_e103768_d_n13;

        let (assign67020_e103777, assign67020_e103777_d_n0, assign67020_e103777_d_n2, assign67020_e103777_d_n4, assign67020_e103777_d_n5, assign67020_e103777_d_n6, assign67020_e103777_d_n7, assign67020_e103777_d_n8, assign67020_e103777_d_n9, assign67020_e103777_d_n10, assign67020_e103777_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67020_e103775: f64 = (locals.var_xsub1 / locals.var_xsubtmp);
        (assign67020_e103775, (-((locals.var_xsub1 * locals.var_xsubtmp_dn0) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn2) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn4) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn5) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn6) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn7) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn8) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn9) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn10) / (locals.var_xsubtmp * locals.var_xsubtmp))), (-((locals.var_xsub1 * locals.var_xsubtmp_dn13) / (locals.var_xsubtmp * locals.var_xsubtmp))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign67020_e103777;
        locals.var_t5_dn0 = assign67020_e103777_d_n0;
        locals.var_t5_dn2 = assign67020_e103777_d_n2;
        locals.var_t5_dn4 = assign67020_e103777_d_n4;
        locals.var_t5_dn5 = assign67020_e103777_d_n5;
        locals.var_t5_dn6 = assign67020_e103777_d_n6;
        locals.var_t5_dn7 = assign67020_e103777_d_n7;
        locals.var_t5_dn8 = assign67020_e103777_d_n8;
        locals.var_t5_dn9 = assign67020_e103777_d_n9;
        locals.var_t5_dn10 = assign67020_e103777_d_n10;
        locals.var_t5_dn13 = assign67020_e103777_d_n13;

        let (assign67030_e103786, assign67030_e103786_d_n0, assign67030_e103786_d_n2, assign67030_e103786_d_n4, assign67030_e103786_d_n5, assign67030_e103786_d_n6, assign67030_e103786_d_n7, assign67030_e103786_d_n8, assign67030_e103786_d_n9, assign67030_e103786_d_n10, assign67030_e103786_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67030_e103784: f64 = (locals.var_xsub2 * locals.var_xsubtmp);
        (assign67030_e103784, (locals.var_xsub2 * locals.var_xsubtmp_dn0), (locals.var_xsub2 * locals.var_xsubtmp_dn2), (locals.var_xsub2 * locals.var_xsubtmp_dn4), (locals.var_xsub2 * locals.var_xsubtmp_dn5), (locals.var_xsub2 * locals.var_xsubtmp_dn6), (locals.var_xsub2 * locals.var_xsubtmp_dn7), (locals.var_xsub2 * locals.var_xsubtmp_dn8), (locals.var_xsub2 * locals.var_xsubtmp_dn9), (locals.var_xsub2 * locals.var_xsubtmp_dn10), (locals.var_xsub2 * locals.var_xsubtmp_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign67030_e103786;
        locals.var_t6_dn0 = assign67030_e103786_d_n0;
        locals.var_t6_dn2 = assign67030_e103786_d_n2;
        locals.var_t6_dn4 = assign67030_e103786_d_n4;
        locals.var_t6_dn5 = assign67030_e103786_d_n5;
        locals.var_t6_dn6 = assign67030_e103786_d_n6;
        locals.var_t6_dn7 = assign67030_e103786_d_n7;
        locals.var_t6_dn8 = assign67030_e103786_d_n8;
        locals.var_t6_dn9 = assign67030_e103786_d_n9;
        locals.var_t6_dn10 = assign67030_e103786_d_n10;
        locals.var_t6_dn13 = assign67030_e103786_d_n13;

        let (assign67040_e103797, assign67040_e103797_d_n0, assign67040_e103797_d_n2, assign67040_e103797_d_n4, assign67040_e103797_d_n5, assign67040_e103797_d_n6, assign67040_e103797_d_n7, assign67040_e103797_d_n8, assign67040_e103797_d_n9, assign67040_e103797_d_n10, assign67040_e103797_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67040_e103792: f64 = (-locals.var_t6);
        let assign67040_e103794: f64 = (assign67040_e103792 / locals.var_psisubsat);
        let assign67040_e103795: f64 = (assign67040_e103794).exp();
        (assign67040_e103795, (assign67040_e103795 * ((((-locals.var_t6_dn0) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn0)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn2) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn2)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn4) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn4)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn5) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn5)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn6) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn6)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn7) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn7)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn8) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn8)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn9) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn9)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn10) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn10)) / (locals.var_psisubsat * locals.var_psisubsat))), (assign67040_e103795 * ((((-locals.var_t6_dn13) * locals.var_psisubsat) - (assign67040_e103792 * locals.var_psisubsat_dn13)) / (locals.var_psisubsat * locals.var_psisubsat))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67040_e103797;
        locals.var_t2_dn0 = assign67040_e103797_d_n0;
        locals.var_t2_dn2 = assign67040_e103797_d_n2;
        locals.var_t2_dn4 = assign67040_e103797_d_n4;
        locals.var_t2_dn5 = assign67040_e103797_d_n5;
        locals.var_t2_dn6 = assign67040_e103797_d_n6;
        locals.var_t2_dn7 = assign67040_e103797_d_n7;
        locals.var_t2_dn8 = assign67040_e103797_d_n8;
        locals.var_t2_dn9 = assign67040_e103797_d_n9;
        locals.var_t2_dn10 = assign67040_e103797_d_n10;
        locals.var_t2_dn13 = assign67040_e103797_d_n13;

        let (assign67050_e103810, assign67050_e103810_d_n0, assign67050_e103810_d_n2, assign67050_e103810_d_n4, assign67050_e103810_d_n5, assign67050_e103810_d_n6, assign67050_e103810_d_n7, assign67050_e103810_d_n8, assign67050_e103810_d_n9, assign67050_e103810_d_n10, assign67050_e103810_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67050_e103804: f64 = (locals.var_t5 * locals.var_psisubsat);
        let assign67050_e103806: f64 = (assign67050_e103804 * locals.var_ids);
        let assign67050_e103808: f64 = (assign67050_e103806 * locals.var_t2);
        (assign67050_e103808, ((((((locals.var_t5_dn0 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn0)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn0)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn0)), ((((((locals.var_t5_dn2 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn2)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn2)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn2)), ((((((locals.var_t5_dn4 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn4)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn4)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn4)), ((((((locals.var_t5_dn5 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn5)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn5)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn5)), ((((((locals.var_t5_dn6 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn6)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn6)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn6)), ((((((locals.var_t5_dn7 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn7)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn7)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn7)), ((((((locals.var_t5_dn8 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn8)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn8)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn8)), ((((((locals.var_t5_dn9 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn9)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn9)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn9)), ((((((locals.var_t5_dn10 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn10)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn10)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn10)), ((((((locals.var_t5_dn13 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn13)) * locals.var_ids) + (assign67050_e103804 * locals.var_ids_dn13)) * locals.var_t2) + (assign67050_e103806 * locals.var_t2_dn13)),)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13,)
    }
};
        locals.var_isub = assign67050_e103810;
        locals.var_isub_dn0 = assign67050_e103810_d_n0;
        locals.var_isub_dn2 = assign67050_e103810_d_n2;
        locals.var_isub_dn4 = assign67050_e103810_d_n4;
        locals.var_isub_dn5 = assign67050_e103810_d_n5;
        locals.var_isub_dn6 = assign67050_e103810_d_n6;
        locals.var_isub_dn7 = assign67050_e103810_d_n7;
        locals.var_isub_dn8 = assign67050_e103810_d_n8;
        locals.var_isub_dn9 = assign67050_e103810_d_n9;
        locals.var_isub_dn10 = assign67050_e103810_d_n10;
        locals.var_isub_dn13 = assign67050_e103810_d_n13;

        let (assign67060_e103821, assign67060_e103821_d_n0, assign67060_e103821_d_n2, assign67060_e103821_d_n4, assign67060_e103821_d_n5, assign67060_e103821_d_n6, assign67060_e103821_d_n7, assign67060_e103821_d_n8, assign67060_e103821_d_n9, assign67060_e103821_d_n10, assign67060_e103821_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 != 0.0)) {
        let assign67060_e103817: f64 = (locals.var_t5 * locals.var_psisubsat);
        let assign67060_e103819: f64 = (assign67060_e103817 * locals.var_t2);
        (assign67060_e103819, ((((locals.var_t5_dn0 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn0)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn0)), ((((locals.var_t5_dn2 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn2)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn2)), ((((locals.var_t5_dn4 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn4)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn4)), ((((locals.var_t5_dn5 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn5)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn5)), ((((locals.var_t5_dn6 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn6)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn6)), ((((locals.var_t5_dn7 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn7)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn7)), ((((locals.var_t5_dn8 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn8)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn8)), ((((locals.var_t5_dn9 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn9)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn9)), ((((locals.var_t5_dn10 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn10)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn10)), ((((locals.var_t5_dn13 * locals.var_psisubsat) + (locals.var_t5 * locals.var_psisubsat_dn13)) * locals.var_t2) + (assign67060_e103817 * locals.var_t2_dn13)),)
    } else {
        (locals.var_wk_ii, locals.var_wk_ii_dn0, locals.var_wk_ii_dn2, locals.var_wk_ii_dn4, locals.var_wk_ii_dn5, locals.var_wk_ii_dn6, locals.var_wk_ii_dn7, locals.var_wk_ii_dn8, locals.var_wk_ii_dn9, locals.var_wk_ii_dn10, locals.var_wk_ii_dn13,)
    }
};
        locals.var_wk_ii = assign67060_e103821;
        locals.var_wk_ii_dn0 = assign67060_e103821_d_n0;
        locals.var_wk_ii_dn2 = assign67060_e103821_d_n2;
        locals.var_wk_ii_dn4 = assign67060_e103821_d_n4;
        locals.var_wk_ii_dn5 = assign67060_e103821_d_n5;
        locals.var_wk_ii_dn6 = assign67060_e103821_d_n6;
        locals.var_wk_ii_dn7 = assign67060_e103821_d_n7;
        locals.var_wk_ii_dn8 = assign67060_e103821_d_n8;
        locals.var_wk_ii_dn9 = assign67060_e103821_d_n9;
        locals.var_wk_ii_dn10 = assign67060_e103821_d_n10;
        locals.var_wk_ii_dn13 = assign67060_e103821_d_n13;

    }

    pub(super) fn stamp_transient_block_229(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67070_e103829, assign67070_e103829_d_n0, assign67070_e103829_d_n2, assign67070_e103829_d_n4, assign67070_e103829_d_n5, assign67070_e103829_d_n6, assign67070_e103829_d_n7, assign67070_e103829_d_n8, assign67070_e103829_d_n9, assign67070_e103829_d_n10, assign67070_e103829_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1581 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13,)
    }
};
        locals.var_isub = assign67070_e103829;
        locals.var_isub_dn0 = assign67070_e103829_d_n0;
        locals.var_isub_dn2 = assign67070_e103829_d_n2;
        locals.var_isub_dn4 = assign67070_e103829_d_n4;
        locals.var_isub_dn5 = assign67070_e103829_d_n5;
        locals.var_isub_dn6 = assign67070_e103829_d_n6;
        locals.var_isub_dn7 = assign67070_e103829_d_n7;
        locals.var_isub_dn8 = assign67070_e103829_d_n8;
        locals.var_isub_dn9 = assign67070_e103829_d_n9;
        locals.var_isub_dn10 = assign67070_e103829_d_n10;
        locals.var_isub_dn13 = assign67070_e103829_d_n13;

        let assign67080_e103832: f64 = if locals.var_uc_subld1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1589 = assign67080_e103832;

        let (assign67090_e103839, assign67090_e103839_d_n0, assign67090_e103839_d_n2, assign67090_e103839_d_n4, assign67090_e103839_d_n5, assign67090_e103839_d_n6, assign67090_e103839_d_n7, assign67090_e103839_d_n8, assign67090_e103839_d_n9, assign67090_e103839_d_n10, assign67090_e103839_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        (locals.var_vddp, locals.var_vddp_dn0, 0.0, 0.0, locals.var_vddp_dn5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67090_e103839;
        locals.var_t0_dn0 = assign67090_e103839_d_n0;
        locals.var_t0_dn2 = assign67090_e103839_d_n2;
        locals.var_t0_dn4 = assign67090_e103839_d_n4;
        locals.var_t0_dn5 = assign67090_e103839_d_n5;
        locals.var_t0_dn6 = assign67090_e103839_d_n6;
        locals.var_t0_dn7 = assign67090_e103839_d_n7;
        locals.var_t0_dn8 = assign67090_e103839_d_n8;
        locals.var_t0_dn9 = assign67090_e103839_d_n9;
        locals.var_t0_dn10 = assign67090_e103839_d_n10;
        locals.var_t0_dn13 = assign67090_e103839_d_n13;

        let (assign67100_e103855, assign67100_e103855_d_n0, assign67100_e103855_d_n2, assign67100_e103855_d_n4, assign67100_e103855_d_n5, assign67100_e103855_d_n6, assign67100_e103855_d_n7, assign67100_e103855_d_n8, assign67100_e103855_d_n9, assign67100_e103855_d_n10, assign67100_e103855_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67100_e103846: f64 = (locals.var_t0 * locals.var_t0);
        let assign67100_e103849: f64 = (4.0 * 1e-6);
        let assign67100_e103851: f64 = (assign67100_e103849 * 1e-6);
        let assign67100_e103852: f64 = (assign67100_e103846 + assign67100_e103851);
        let assign67100_e103853: f64 = (assign67100_e103852).sqrt();
        (assign67100_e103853, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign67100_e103853)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign67100_e103853)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign67100_e103855;
        locals.var_tmf2_dn0 = assign67100_e103855_d_n0;
        locals.var_tmf2_dn2 = assign67100_e103855_d_n2;
        locals.var_tmf2_dn4 = assign67100_e103855_d_n4;
        locals.var_tmf2_dn5 = assign67100_e103855_d_n5;
        locals.var_tmf2_dn6 = assign67100_e103855_d_n6;
        locals.var_tmf2_dn7 = assign67100_e103855_d_n7;
        locals.var_tmf2_dn8 = assign67100_e103855_d_n8;
        locals.var_tmf2_dn9 = assign67100_e103855_d_n9;
        locals.var_tmf2_dn10 = assign67100_e103855_d_n10;
        locals.var_tmf2_dn13 = assign67100_e103855_d_n13;

        let (assign67110_e103868, assign67110_e103868_d_n0, assign67110_e103868_d_n2, assign67110_e103868_d_n4, assign67110_e103868_d_n5, assign67110_e103868_d_n6, assign67110_e103868_d_n7, assign67110_e103868_d_n8, assign67110_e103868_d_n9, assign67110_e103868_d_n10, assign67110_e103868_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67110_e103864: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign67110_e103865: f64 = (1.0 + assign67110_e103864);
        let assign67110_e103866: f64 = (0.5 * assign67110_e103865);
        (assign67110_e103866, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67110_e103868;
        locals.var_t1_dn0 = assign67110_e103868_d_n0;
        locals.var_t1_dn2 = assign67110_e103868_d_n2;
        locals.var_t1_dn4 = assign67110_e103868_d_n4;
        locals.var_t1_dn5 = assign67110_e103868_d_n5;
        locals.var_t1_dn6 = assign67110_e103868_d_n6;
        locals.var_t1_dn7 = assign67110_e103868_d_n7;
        locals.var_t1_dn8 = assign67110_e103868_d_n8;
        locals.var_t1_dn9 = assign67110_e103868_d_n9;
        locals.var_t1_dn10 = assign67110_e103868_d_n10;
        locals.var_t1_dn13 = assign67110_e103868_d_n13;

        let (assign67120_e103879, assign67120_e103879_d_n0, assign67120_e103879_d_n2, assign67120_e103879_d_n4, assign67120_e103879_d_n5, assign67120_e103879_d_n6, assign67120_e103879_d_n7, assign67120_e103879_d_n8, assign67120_e103879_d_n9, assign67120_e103879_d_n10, assign67120_e103879_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67120_e103876: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign67120_e103877: f64 = (0.5 * assign67120_e103876);
        (assign67120_e103877, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67120_e103879;
        locals.var_t0_dn0 = assign67120_e103879_d_n0;
        locals.var_t0_dn2 = assign67120_e103879_d_n2;
        locals.var_t0_dn4 = assign67120_e103879_d_n4;
        locals.var_t0_dn5 = assign67120_e103879_d_n5;
        locals.var_t0_dn6 = assign67120_e103879_d_n6;
        locals.var_t0_dn7 = assign67120_e103879_d_n7;
        locals.var_t0_dn8 = assign67120_e103879_d_n8;
        locals.var_t0_dn9 = assign67120_e103879_d_n9;
        locals.var_t0_dn10 = assign67120_e103879_d_n10;
        locals.var_t0_dn13 = assign67120_e103879_d_n13;

        let assign67130_e103882: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1590 = assign67130_e103882;

        let (assign67140_e103891, assign67140_e103891_d_n0, assign67140_e103891_d_n2, assign67140_e103891_d_n4, assign67140_e103891_d_n5, assign67140_e103891_d_n6, assign67140_e103891_d_n7, assign67140_e103891_d_n8, assign67140_e103891_d_n9, assign67140_e103891_d_n10, assign67140_e103891_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1590 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67140_e103891;
        locals.var_t0_dn0 = assign67140_e103891_d_n0;
        locals.var_t0_dn2 = assign67140_e103891_d_n2;
        locals.var_t0_dn4 = assign67140_e103891_d_n4;
        locals.var_t0_dn5 = assign67140_e103891_d_n5;
        locals.var_t0_dn6 = assign67140_e103891_d_n6;
        locals.var_t0_dn7 = assign67140_e103891_d_n7;
        locals.var_t0_dn8 = assign67140_e103891_d_n8;
        locals.var_t0_dn9 = assign67140_e103891_d_n9;
        locals.var_t0_dn10 = assign67140_e103891_d_n10;
        locals.var_t0_dn13 = assign67140_e103891_d_n13;

        let (assign67150_e103900, assign67150_e103900_d_n0, assign67150_e103900_d_n2, assign67150_e103900_d_n4, assign67150_e103900_d_n5, assign67150_e103900_d_n6, assign67150_e103900_d_n7, assign67150_e103900_d_n8, assign67150_e103900_d_n9, assign67150_e103900_d_n10, assign67150_e103900_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1590 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67150_e103900;
        locals.var_t1_dn0 = assign67150_e103900_d_n0;
        locals.var_t1_dn2 = assign67150_e103900_d_n2;
        locals.var_t1_dn4 = assign67150_e103900_d_n4;
        locals.var_t1_dn5 = assign67150_e103900_d_n5;
        locals.var_t1_dn6 = assign67150_e103900_d_n6;
        locals.var_t1_dn7 = assign67150_e103900_d_n7;
        locals.var_t1_dn8 = assign67150_e103900_d_n8;
        locals.var_t1_dn9 = assign67150_e103900_d_n9;
        locals.var_t1_dn10 = assign67150_e103900_d_n10;
        locals.var_t1_dn13 = assign67150_e103900_d_n13;

        let (assign67160_e103910, assign67160_e103910_d_n0, assign67160_e103910_d_n2, assign67160_e103910_d_n4, assign67160_e103910_d_n5, assign67160_e103910_d_n6, assign67160_e103910_d_n7, assign67160_e103910_d_n8, assign67160_e103910_d_n9, assign67160_e103910_d_n10, assign67160_e103910_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67160_e103907: f64 = (locals.var_vgvt + 1e-25);
        let assign67160_e103908: f64 = (assign67160_e103907).sqrt();
        (assign67160_e103908, (locals.var_vgvt_dn0 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn2 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn4 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn5 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn6 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn7 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn8 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn9 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn10 / (2.0 * assign67160_e103908)), (locals.var_vgvt_dn13 / (2.0 * assign67160_e103908)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67160_e103910;
        locals.var_t1_dn0 = assign67160_e103910_d_n0;
        locals.var_t1_dn2 = assign67160_e103910_d_n2;
        locals.var_t1_dn4 = assign67160_e103910_d_n4;
        locals.var_t1_dn5 = assign67160_e103910_d_n5;
        locals.var_t1_dn6 = assign67160_e103910_d_n6;
        locals.var_t1_dn7 = assign67160_e103910_d_n7;
        locals.var_t1_dn8 = assign67160_e103910_d_n8;
        locals.var_t1_dn9 = assign67160_e103910_d_n9;
        locals.var_t1_dn10 = assign67160_e103910_d_n10;
        locals.var_t1_dn13 = assign67160_e103910_d_n13;

        let (assign67170_e103921, assign67170_e103921_d_n0, assign67170_e103921_d_n2, assign67170_e103921_d_n4, assign67170_e103921_d_n5, assign67170_e103921_d_n6, assign67170_e103921_d_n7, assign67170_e103921_d_n8, assign67170_e103921_d_n9, assign67170_e103921_d_n10, assign67170_e103921_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67170_e103918: f64 = (2.0 * locals.var_t1);
        let assign67170_e103919: f64 = (1.0 / assign67170_e103918);
        (assign67170_e103919, (-((2.0 * locals.var_t1_dn0) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn2) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn4) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn5) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn6) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn7) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn8) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn9) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn10) / (assign67170_e103918 * assign67170_e103918))), (-((2.0 * locals.var_t1_dn13) / (assign67170_e103918 * assign67170_e103918))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign67170_e103921;
        locals.var_t3_dn0 = assign67170_e103921_d_n0;
        locals.var_t3_dn2 = assign67170_e103921_d_n2;
        locals.var_t3_dn4 = assign67170_e103921_d_n4;
        locals.var_t3_dn5 = assign67170_e103921_d_n5;
        locals.var_t3_dn6 = assign67170_e103921_d_n6;
        locals.var_t3_dn7 = assign67170_e103921_d_n7;
        locals.var_t3_dn8 = assign67170_e103921_d_n8;
        locals.var_t3_dn9 = assign67170_e103921_d_n9;
        locals.var_t3_dn10 = assign67170_e103921_d_n10;
        locals.var_t3_dn13 = assign67170_e103921_d_n13;

        let (assign67180_e103936, assign67180_e103936_d_n0, assign67180_e103936_d_n2, assign67180_e103936_d_n4, assign67180_e103936_d_n5, assign67180_e103936_d_n6, assign67180_e103936_d_n7, assign67180_e103936_d_n8, assign67180_e103936_d_n9, assign67180_e103936_d_n10, assign67180_e103936_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67180_e103931: f64 = (p.p106 * locals.var_vgs);
        let assign67180_e103932: f64 = (1.0 + assign67180_e103931);
        let assign67180_e103933: f64 = (p.p105 * assign67180_e103932);
        let assign67180_e103934: f64 = (locals.var_t0 - assign67180_e103933);
        (assign67180_e103934, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, (locals.var_t0_dn5 - (p.p105 * (p.p106 * locals.var_vgs_dn5))), (locals.var_t0_dn6 - (p.p105 * (p.p106 * locals.var_vgs_dn6))), (locals.var_t0_dn7 - (p.p105 * (p.p106 * locals.var_vgs_dn7))), locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67180_e103936;
        locals.var_t4_dn0 = assign67180_e103936_d_n0;
        locals.var_t4_dn2 = assign67180_e103936_d_n2;
        locals.var_t4_dn4 = assign67180_e103936_d_n4;
        locals.var_t4_dn5 = assign67180_e103936_d_n5;
        locals.var_t4_dn6 = assign67180_e103936_d_n6;
        locals.var_t4_dn7 = assign67180_e103936_d_n7;
        locals.var_t4_dn8 = assign67180_e103936_d_n8;
        locals.var_t4_dn9 = assign67180_e103936_d_n9;
        locals.var_t4_dn10 = assign67180_e103936_d_n10;
        locals.var_t4_dn13 = assign67180_e103936_d_n13;

        let (assign67190_e103952, assign67190_e103952_d_n0, assign67190_e103952_d_n2, assign67190_e103952_d_n4, assign67190_e103952_d_n5, assign67190_e103952_d_n6, assign67190_e103952_d_n7, assign67190_e103952_d_n8, assign67190_e103952_d_n9, assign67190_e103952_d_n10, assign67190_e103952_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67190_e103943: f64 = (locals.var_t4 * locals.var_t4);
        let assign67190_e103946: f64 = (4.0 * 0.01);
        let assign67190_e103948: f64 = (assign67190_e103946 * 0.01);
        let assign67190_e103949: f64 = (assign67190_e103943 + assign67190_e103948);
        let assign67190_e103950: f64 = (assign67190_e103949).sqrt();
        (assign67190_e103950, (((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) / (2.0 * assign67190_e103950)), (((locals.var_t4_dn13 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn13)) / (2.0 * assign67190_e103950)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign67190_e103952;
        locals.var_tmf2_dn0 = assign67190_e103952_d_n0;
        locals.var_tmf2_dn2 = assign67190_e103952_d_n2;
        locals.var_tmf2_dn4 = assign67190_e103952_d_n4;
        locals.var_tmf2_dn5 = assign67190_e103952_d_n5;
        locals.var_tmf2_dn6 = assign67190_e103952_d_n6;
        locals.var_tmf2_dn7 = assign67190_e103952_d_n7;
        locals.var_tmf2_dn8 = assign67190_e103952_d_n8;
        locals.var_tmf2_dn9 = assign67190_e103952_d_n9;
        locals.var_tmf2_dn10 = assign67190_e103952_d_n10;
        locals.var_tmf2_dn13 = assign67190_e103952_d_n13;

        let (assign67200_e103965, assign67200_e103965_d_n0, assign67200_e103965_d_n2, assign67200_e103965_d_n4, assign67200_e103965_d_n5, assign67200_e103965_d_n6, assign67200_e103965_d_n7, assign67200_e103965_d_n8, assign67200_e103965_d_n9, assign67200_e103965_d_n10, assign67200_e103965_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67200_e103961: f64 = (locals.var_t4 / locals.var_tmf2);
        let assign67200_e103962: f64 = (1.0 + assign67200_e103961);
        let assign67200_e103963: f64 = (0.5 * assign67200_e103962);
        (assign67200_e103963, (0.5 * (((locals.var_t4_dn0 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn2 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn4 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn5 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn6 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn7 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn8 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn9 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn10 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4_dn13 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign67200_e103965;
        locals.var_t9_dn0 = assign67200_e103965_d_n0;
        locals.var_t9_dn2 = assign67200_e103965_d_n2;
        locals.var_t9_dn4 = assign67200_e103965_d_n4;
        locals.var_t9_dn5 = assign67200_e103965_d_n5;
        locals.var_t9_dn6 = assign67200_e103965_d_n6;
        locals.var_t9_dn7 = assign67200_e103965_d_n7;
        locals.var_t9_dn8 = assign67200_e103965_d_n8;
        locals.var_t9_dn9 = assign67200_e103965_d_n9;
        locals.var_t9_dn10 = assign67200_e103965_d_n10;
        locals.var_t9_dn13 = assign67200_e103965_d_n13;

        let (assign67210_e103976, assign67210_e103976_d_n0, assign67210_e103976_d_n2, assign67210_e103976_d_n4, assign67210_e103976_d_n5, assign67210_e103976_d_n6, assign67210_e103976_d_n7, assign67210_e103976_d_n8, assign67210_e103976_d_n9, assign67210_e103976_d_n10, assign67210_e103976_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67210_e103973: f64 = (locals.var_t4 + locals.var_tmf2);
        let assign67210_e103974: f64 = (0.5 * assign67210_e103973);
        (assign67210_e103974, (0.5 * (locals.var_t4_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t4_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t4_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t4_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t4_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t4_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t4_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t4_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t4_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t4_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67210_e103976;
        locals.var_t4_dn0 = assign67210_e103976_d_n0;
        locals.var_t4_dn2 = assign67210_e103976_d_n2;
        locals.var_t4_dn4 = assign67210_e103976_d_n4;
        locals.var_t4_dn5 = assign67210_e103976_d_n5;
        locals.var_t4_dn6 = assign67210_e103976_d_n6;
        locals.var_t4_dn7 = assign67210_e103976_d_n7;
        locals.var_t4_dn8 = assign67210_e103976_d_n8;
        locals.var_t4_dn9 = assign67210_e103976_d_n9;
        locals.var_t4_dn10 = assign67210_e103976_d_n10;
        locals.var_t4_dn13 = assign67210_e103976_d_n13;

        let assign67220_e103979: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1591 = assign67220_e103979;

        let (assign67230_e103988, assign67230_e103988_d_n0, assign67230_e103988_d_n2, assign67230_e103988_d_n4, assign67230_e103988_d_n5, assign67230_e103988_d_n6, assign67230_e103988_d_n7, assign67230_e103988_d_n8, assign67230_e103988_d_n9, assign67230_e103988_d_n10, assign67230_e103988_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1591 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67230_e103988;
        locals.var_t4_dn0 = assign67230_e103988_d_n0;
        locals.var_t4_dn2 = assign67230_e103988_d_n2;
        locals.var_t4_dn4 = assign67230_e103988_d_n4;
        locals.var_t4_dn5 = assign67230_e103988_d_n5;
        locals.var_t4_dn6 = assign67230_e103988_d_n6;
        locals.var_t4_dn7 = assign67230_e103988_d_n7;
        locals.var_t4_dn8 = assign67230_e103988_d_n8;
        locals.var_t4_dn9 = assign67230_e103988_d_n9;
        locals.var_t4_dn10 = assign67230_e103988_d_n10;
        locals.var_t4_dn13 = assign67230_e103988_d_n13;

        let (assign67240_e103997, assign67240_e103997_d_n0, assign67240_e103997_d_n2, assign67240_e103997_d_n4, assign67240_e103997_d_n5, assign67240_e103997_d_n6, assign67240_e103997_d_n7, assign67240_e103997_d_n8, assign67240_e103997_d_n9, assign67240_e103997_d_n10, assign67240_e103997_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1591 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign67240_e103997;
        locals.var_t9_dn0 = assign67240_e103997_d_n0;
        locals.var_t9_dn2 = assign67240_e103997_d_n2;
        locals.var_t9_dn4 = assign67240_e103997_d_n4;
        locals.var_t9_dn5 = assign67240_e103997_d_n5;
        locals.var_t9_dn6 = assign67240_e103997_d_n6;
        locals.var_t9_dn7 = assign67240_e103997_d_n7;
        locals.var_t9_dn8 = assign67240_e103997_d_n8;
        locals.var_t9_dn9 = assign67240_e103997_d_n9;
        locals.var_t9_dn10 = assign67240_e103997_d_n10;
        locals.var_t9_dn13 = assign67240_e103997_d_n13;

        let (assign67250_e104006, assign67250_e104006_d_n0, assign67250_e104006_d_n2, assign67250_e104006_d_n4, assign67250_e104006_d_n5, assign67250_e104006_d_n6, assign67250_e104006_d_n7, assign67250_e104006_d_n8, assign67250_e104006_d_n9, assign67250_e104006_d_n10, assign67250_e104006_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67250_e104004: f64 = (locals.var_t4 + 1e-25);
        (assign67250_e104004, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67250_e104006;
        locals.var_t4_dn0 = assign67250_e104006_d_n0;
        locals.var_t4_dn2 = assign67250_e104006_d_n2;
        locals.var_t4_dn4 = assign67250_e104006_d_n4;
        locals.var_t4_dn5 = assign67250_e104006_d_n5;
        locals.var_t4_dn6 = assign67250_e104006_d_n6;
        locals.var_t4_dn7 = assign67250_e104006_d_n7;
        locals.var_t4_dn8 = assign67250_e104006_d_n8;
        locals.var_t4_dn9 = assign67250_e104006_d_n9;
        locals.var_t4_dn10 = assign67250_e104006_d_n10;
        locals.var_t4_dn13 = assign67250_e104006_d_n13;

        let (assign67260_e104021, assign67260_e104021_d_n0, assign67260_e104021_d_n2, assign67260_e104021_d_n4, assign67260_e104021_d_n5, assign67260_e104021_d_n6, assign67260_e104021_d_n7, assign67260_e104021_d_n8, assign67260_e104021_d_n9, assign67260_e104021_d_n10, assign67260_e104021_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67260_e104013: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign67260_e104015: f64 = (-1.0);
        let assign67260_e104017: f64 = (assign67260_e104015 / locals.var_t4);
        let assign67260_e104018: f64 = (assign67260_e104017).exp();
        let assign67260_e104019: f64 = (assign67260_e104013 * assign67260_e104018);
        (assign67260_e104019, (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn0) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn2) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))), (assign67260_e104013 * (assign67260_e104018 * (-((assign67260_e104015 * locals.var_t4_dn13) / (locals.var_t4 * locals.var_t4))))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign67260_e104021;
        locals.var_t10_dn0 = assign67260_e104021_d_n0;
        locals.var_t10_dn2 = assign67260_e104021_d_n2;
        locals.var_t10_dn4 = assign67260_e104021_d_n4;
        locals.var_t10_dn5 = assign67260_e104021_d_n5;
        locals.var_t10_dn6 = assign67260_e104021_d_n6;
        locals.var_t10_dn7 = assign67260_e104021_d_n7;
        locals.var_t10_dn8 = assign67260_e104021_d_n8;
        locals.var_t10_dn9 = assign67260_e104021_d_n9;
        locals.var_t10_dn10 = assign67260_e104021_d_n10;
        locals.var_t10_dn13 = assign67260_e104021_d_n13;

        let (assign67270_e104034, assign67270_e104034_d_n0, assign67270_e104034_d_n2, assign67270_e104034_d_n4, assign67270_e104034_d_n5, assign67270_e104034_d_n6, assign67270_e104034_d_n7, assign67270_e104034_d_n8, assign67270_e104034_d_n9, assign67270_e104034_d_n10, assign67270_e104034_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67270_e104030: f64 = (1.0 / locals.var_t4);
        let assign67270_e104031: f64 = (1.0 + assign67270_e104030);
        let assign67270_e104032: f64 = (locals.var_t10 * assign67270_e104031);
        (assign67270_e104032, ((locals.var_t10_dn0 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn2 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn4 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn5 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn6 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn7 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn8 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn9 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn10 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))))), ((locals.var_t10_dn13 * assign67270_e104031) + (locals.var_t10 * (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))))),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign67270_e104034;
        locals.var_t11_dn0 = assign67270_e104034_d_n0;
        locals.var_t11_dn2 = assign67270_e104034_d_n2;
        locals.var_t11_dn4 = assign67270_e104034_d_n4;
        locals.var_t11_dn5 = assign67270_e104034_d_n5;
        locals.var_t11_dn6 = assign67270_e104034_d_n6;
        locals.var_t11_dn7 = assign67270_e104034_d_n7;
        locals.var_t11_dn8 = assign67270_e104034_d_n8;
        locals.var_t11_dn9 = assign67270_e104034_d_n9;
        locals.var_t11_dn10 = assign67270_e104034_d_n10;
        locals.var_t11_dn13 = assign67270_e104034_d_n13;

        let (assign67280_e104043, assign67280_e104043_d_n0, assign67280_e104043_d_n2, assign67280_e104043_d_n4, assign67280_e104043_d_n5, assign67280_e104043_d_n6, assign67280_e104043_d_n7, assign67280_e104043_d_n8, assign67280_e104043_d_n9, assign67280_e104043_d_n10, assign67280_e104043_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67280_e104041: f64 = (locals.var_t4 * locals.var_t10);
        (assign67280_e104041, ((locals.var_t4_dn0 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn0)), ((locals.var_t4_dn2 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn2)), ((locals.var_t4_dn4 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn4)), ((locals.var_t4_dn5 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn5)), ((locals.var_t4_dn6 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn6)), ((locals.var_t4_dn7 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn7)), ((locals.var_t4_dn8 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn8)), ((locals.var_t4_dn9 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn9)), ((locals.var_t4_dn10 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn10)), ((locals.var_t4_dn13 * locals.var_t10) + (locals.var_t4 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign67280_e104043;
        locals.var_t3_dn0 = assign67280_e104043_d_n0;
        locals.var_t3_dn2 = assign67280_e104043_d_n2;
        locals.var_t3_dn4 = assign67280_e104043_d_n4;
        locals.var_t3_dn5 = assign67280_e104043_d_n5;
        locals.var_t3_dn6 = assign67280_e104043_d_n6;
        locals.var_t3_dn7 = assign67280_e104043_d_n7;
        locals.var_t3_dn8 = assign67280_e104043_d_n8;
        locals.var_t3_dn9 = assign67280_e104043_d_n9;
        locals.var_t3_dn10 = assign67280_e104043_d_n10;
        locals.var_t3_dn13 = assign67280_e104043_d_n13;

        let (assign67290_e104052, assign67290_e104052_d_n0, assign67290_e104052_d_n2, assign67290_e104052_d_n4, assign67290_e104052_d_n5, assign67290_e104052_d_n6, assign67290_e104052_d_n7, assign67290_e104052_d_n8, assign67290_e104052_d_n9, assign67290_e104052_d_n10, assign67290_e104052_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67290_e104050: f64 = (locals.var_t0 - locals.var_t3);
        (assign67290_e104050, (locals.var_t0_dn0 - locals.var_t3_dn0), (locals.var_t0_dn2 - locals.var_t3_dn2), (locals.var_t0_dn4 - locals.var_t3_dn4), (locals.var_t0_dn5 - locals.var_t3_dn5), (locals.var_t0_dn6 - locals.var_t3_dn6), (locals.var_t0_dn7 - locals.var_t3_dn7), (locals.var_t0_dn8 - locals.var_t3_dn8), (locals.var_t0_dn9 - locals.var_t3_dn9), (locals.var_t0_dn10 - locals.var_t3_dn10), (locals.var_t0_dn13 - locals.var_t3_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67290_e104052;
        locals.var_t0_dn0 = assign67290_e104052_d_n0;
        locals.var_t0_dn2 = assign67290_e104052_d_n2;
        locals.var_t0_dn4 = assign67290_e104052_d_n4;
        locals.var_t0_dn5 = assign67290_e104052_d_n5;
        locals.var_t0_dn6 = assign67290_e104052_d_n6;
        locals.var_t0_dn7 = assign67290_e104052_d_n7;
        locals.var_t0_dn8 = assign67290_e104052_d_n8;
        locals.var_t0_dn9 = assign67290_e104052_d_n9;
        locals.var_t0_dn10 = assign67290_e104052_d_n10;
        locals.var_t0_dn13 = assign67290_e104052_d_n13;

        let (assign67300_e104068, assign67300_e104068_d_n0, assign67300_e104068_d_n2, assign67300_e104068_d_n4, assign67300_e104068_d_n5, assign67300_e104068_d_n6, assign67300_e104068_d_n7, assign67300_e104068_d_n8, assign67300_e104068_d_n9, assign67300_e104068_d_n10, assign67300_e104068_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67300_e104059: f64 = (locals.var_t0 * locals.var_t0);
        let assign67300_e104062: f64 = (4.0 * 0.01);
        let assign67300_e104064: f64 = (assign67300_e104062 * 0.01);
        let assign67300_e104065: f64 = (assign67300_e104059 + assign67300_e104064);
        let assign67300_e104066: f64 = (assign67300_e104065).sqrt();
        (assign67300_e104066, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign67300_e104066)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign67300_e104066)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign67300_e104068;
        locals.var_tmf2_dn0 = assign67300_e104068_d_n0;
        locals.var_tmf2_dn2 = assign67300_e104068_d_n2;
        locals.var_tmf2_dn4 = assign67300_e104068_d_n4;
        locals.var_tmf2_dn5 = assign67300_e104068_d_n5;
        locals.var_tmf2_dn6 = assign67300_e104068_d_n6;
        locals.var_tmf2_dn7 = assign67300_e104068_d_n7;
        locals.var_tmf2_dn8 = assign67300_e104068_d_n8;
        locals.var_tmf2_dn9 = assign67300_e104068_d_n9;
        locals.var_tmf2_dn10 = assign67300_e104068_d_n10;
        locals.var_tmf2_dn13 = assign67300_e104068_d_n13;

        let (assign67310_e104081, assign67310_e104081_d_n0, assign67310_e104081_d_n2, assign67310_e104081_d_n4, assign67310_e104081_d_n5, assign67310_e104081_d_n6, assign67310_e104081_d_n7, assign67310_e104081_d_n8, assign67310_e104081_d_n9, assign67310_e104081_d_n10, assign67310_e104081_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67310_e104077: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign67310_e104078: f64 = (1.0 + assign67310_e104077);
        let assign67310_e104079: f64 = (0.5 * assign67310_e104078);
        (assign67310_e104079, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign67310_e104081;
        locals.var_t9_dn0 = assign67310_e104081_d_n0;
        locals.var_t9_dn2 = assign67310_e104081_d_n2;
        locals.var_t9_dn4 = assign67310_e104081_d_n4;
        locals.var_t9_dn5 = assign67310_e104081_d_n5;
        locals.var_t9_dn6 = assign67310_e104081_d_n6;
        locals.var_t9_dn7 = assign67310_e104081_d_n7;
        locals.var_t9_dn8 = assign67310_e104081_d_n8;
        locals.var_t9_dn9 = assign67310_e104081_d_n9;
        locals.var_t9_dn10 = assign67310_e104081_d_n10;
        locals.var_t9_dn13 = assign67310_e104081_d_n13;

        let (assign67320_e104092, assign67320_e104092_d_n0, assign67320_e104092_d_n2, assign67320_e104092_d_n4, assign67320_e104092_d_n5, assign67320_e104092_d_n6, assign67320_e104092_d_n7, assign67320_e104092_d_n8, assign67320_e104092_d_n9, assign67320_e104092_d_n10, assign67320_e104092_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67320_e104089: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign67320_e104090: f64 = (0.5 * assign67320_e104089);
        (assign67320_e104090, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67320_e104092;
        locals.var_t0_dn0 = assign67320_e104092_d_n0;
        locals.var_t0_dn2 = assign67320_e104092_d_n2;
        locals.var_t0_dn4 = assign67320_e104092_d_n4;
        locals.var_t0_dn5 = assign67320_e104092_d_n5;
        locals.var_t0_dn6 = assign67320_e104092_d_n6;
        locals.var_t0_dn7 = assign67320_e104092_d_n7;
        locals.var_t0_dn8 = assign67320_e104092_d_n8;
        locals.var_t0_dn9 = assign67320_e104092_d_n9;
        locals.var_t0_dn10 = assign67320_e104092_d_n10;
        locals.var_t0_dn13 = assign67320_e104092_d_n13;

        let assign67330_e104095: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1592 = assign67330_e104095;

    }

    pub(super) fn stamp_transient_block_230(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67340_e104104, assign67340_e104104_d_n0, assign67340_e104104_d_n2, assign67340_e104104_d_n4, assign67340_e104104_d_n5, assign67340_e104104_d_n6, assign67340_e104104_d_n7, assign67340_e104104_d_n8, assign67340_e104104_d_n9, assign67340_e104104_d_n10, assign67340_e104104_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1592 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67340_e104104;
        locals.var_t0_dn0 = assign67340_e104104_d_n0;
        locals.var_t0_dn2 = assign67340_e104104_d_n2;
        locals.var_t0_dn4 = assign67340_e104104_d_n4;
        locals.var_t0_dn5 = assign67340_e104104_d_n5;
        locals.var_t0_dn6 = assign67340_e104104_d_n6;
        locals.var_t0_dn7 = assign67340_e104104_d_n7;
        locals.var_t0_dn8 = assign67340_e104104_d_n8;
        locals.var_t0_dn9 = assign67340_e104104_d_n9;
        locals.var_t0_dn10 = assign67340_e104104_d_n10;
        locals.var_t0_dn13 = assign67340_e104104_d_n13;

        let (assign67350_e104113, assign67350_e104113_d_n0, assign67350_e104113_d_n2, assign67350_e104113_d_n4, assign67350_e104113_d_n5, assign67350_e104113_d_n6, assign67350_e104113_d_n7, assign67350_e104113_d_n8, assign67350_e104113_d_n9, assign67350_e104113_d_n10, assign67350_e104113_d_n13,) = {
    if (((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) && (locals.var_guard1592 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign67350_e104113;
        locals.var_t9_dn0 = assign67350_e104113_d_n0;
        locals.var_t9_dn2 = assign67350_e104113_d_n2;
        locals.var_t9_dn4 = assign67350_e104113_d_n4;
        locals.var_t9_dn5 = assign67350_e104113_d_n5;
        locals.var_t9_dn6 = assign67350_e104113_d_n6;
        locals.var_t9_dn7 = assign67350_e104113_d_n7;
        locals.var_t9_dn8 = assign67350_e104113_d_n8;
        locals.var_t9_dn9 = assign67350_e104113_d_n9;
        locals.var_t9_dn10 = assign67350_e104113_d_n10;
        locals.var_t9_dn13 = assign67350_e104113_d_n13;

        let (assign67360_e104122, assign67360_e104122_d_n0, assign67360_e104122_d_n2, assign67360_e104122_d_n4, assign67360_e104122_d_n5, assign67360_e104122_d_n6, assign67360_e104122_d_n7, assign67360_e104122_d_n8, assign67360_e104122_d_n9, assign67360_e104122_d_n10, assign67360_e104122_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67360_e104120: f64 = (locals.var_t0 + 1e-25);
        (assign67360_e104120, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67360_e104122;
        locals.var_t0_dn0 = assign67360_e104122_d_n0;
        locals.var_t0_dn2 = assign67360_e104122_d_n2;
        locals.var_t0_dn4 = assign67360_e104122_d_n4;
        locals.var_t0_dn5 = assign67360_e104122_d_n5;
        locals.var_t0_dn6 = assign67360_e104122_d_n6;
        locals.var_t0_dn7 = assign67360_e104122_d_n7;
        locals.var_t0_dn8 = assign67360_e104122_d_n8;
        locals.var_t0_dn9 = assign67360_e104122_d_n9;
        locals.var_t0_dn10 = assign67360_e104122_d_n10;
        locals.var_t0_dn13 = assign67360_e104122_d_n13;

        let (assign67370_e104133, assign67370_e104133_d_n0, assign67370_e104133_d_n2, assign67370_e104133_d_n4, assign67370_e104133_d_n5, assign67370_e104133_d_n6, assign67370_e104133_d_n7, assign67370_e104133_d_n8, assign67370_e104133_d_n9, assign67370_e104133_d_n10, assign67370_e104133_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67370_e104130: f64 = (locals.var_t0 * locals.var_t1);
        let assign67370_e104131: f64 = (1.0 / assign67370_e104130);
        (assign67370_e104131, (-(((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) / (assign67370_e104130 * assign67370_e104130))), (-(((locals.var_t0_dn13 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn13)) / (assign67370_e104130 * assign67370_e104130))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign67370_e104133;
        locals.var_t4_dn0 = assign67370_e104133_d_n0;
        locals.var_t4_dn2 = assign67370_e104133_d_n2;
        locals.var_t4_dn4 = assign67370_e104133_d_n4;
        locals.var_t4_dn5 = assign67370_e104133_d_n5;
        locals.var_t4_dn6 = assign67370_e104133_d_n6;
        locals.var_t4_dn7 = assign67370_e104133_d_n7;
        locals.var_t4_dn8 = assign67370_e104133_d_n8;
        locals.var_t4_dn9 = assign67370_e104133_d_n9;
        locals.var_t4_dn10 = assign67370_e104133_d_n10;
        locals.var_t4_dn13 = assign67370_e104133_d_n13;

        let (assign67380_e104142, assign67380_e104142_d_n0, assign67380_e104142_d_n2, assign67380_e104142_d_n4, assign67380_e104142_d_n5, assign67380_e104142_d_n6, assign67380_e104142_d_n7, assign67380_e104142_d_n8, assign67380_e104142_d_n9, assign67380_e104142_d_n10, assign67380_e104142_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67380_e104140: f64 = (locals.var_ldrift0 * locals.var_mks_subld2);
        (assign67380_e104140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign67380_e104142;
        locals.var_t7_dn0 = assign67380_e104142_d_n0;
        locals.var_t7_dn2 = assign67380_e104142_d_n2;
        locals.var_t7_dn4 = assign67380_e104142_d_n4;
        locals.var_t7_dn5 = assign67380_e104142_d_n5;
        locals.var_t7_dn6 = assign67380_e104142_d_n6;
        locals.var_t7_dn7 = assign67380_e104142_d_n7;
        locals.var_t7_dn8 = assign67380_e104142_d_n8;
        locals.var_t7_dn9 = assign67380_e104142_d_n9;
        locals.var_t7_dn10 = assign67380_e104142_d_n10;
        locals.var_t7_dn13 = assign67380_e104142_d_n13;

        let (assign67390_e104153, assign67390_e104153_d_n0, assign67390_e104153_d_n2, assign67390_e104153_d_n4, assign67390_e104153_d_n5, assign67390_e104153_d_n6, assign67390_e104153_d_n7, assign67390_e104153_d_n8, assign67390_e104153_d_n9, assign67390_e104153_d_n10, assign67390_e104153_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67390_e104148: f64 = (-locals.var_t7);
        let assign67390_e104150: f64 = (assign67390_e104148 * locals.var_t4);
        let assign67390_e104151: f64 = (assign67390_e104150).exp();
        (assign67390_e104151, (assign67390_e104151 * (((-locals.var_t7_dn0) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn0))), (assign67390_e104151 * (((-locals.var_t7_dn2) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn2))), (assign67390_e104151 * (((-locals.var_t7_dn4) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn4))), (assign67390_e104151 * (((-locals.var_t7_dn5) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn5))), (assign67390_e104151 * (((-locals.var_t7_dn6) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn6))), (assign67390_e104151 * (((-locals.var_t7_dn7) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn7))), (assign67390_e104151 * (((-locals.var_t7_dn8) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn8))), (assign67390_e104151 * (((-locals.var_t7_dn9) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn9))), (assign67390_e104151 * (((-locals.var_t7_dn10) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn10))), (assign67390_e104151 * (((-locals.var_t7_dn13) * locals.var_t4) + (assign67390_e104148 * locals.var_t4_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67390_e104153;
        locals.var_t2_dn0 = assign67390_e104153_d_n0;
        locals.var_t2_dn2 = assign67390_e104153_d_n2;
        locals.var_t2_dn4 = assign67390_e104153_d_n4;
        locals.var_t2_dn5 = assign67390_e104153_d_n5;
        locals.var_t2_dn6 = assign67390_e104153_d_n6;
        locals.var_t2_dn7 = assign67390_e104153_d_n7;
        locals.var_t2_dn8 = assign67390_e104153_d_n8;
        locals.var_t2_dn9 = assign67390_e104153_d_n9;
        locals.var_t2_dn10 = assign67390_e104153_d_n10;
        locals.var_t2_dn13 = assign67390_e104153_d_n13;

        let (assign67400_e104166, assign67400_e104166_d_n0, assign67400_e104166_d_n2, assign67400_e104166_d_n4, assign67400_e104166_d_n5, assign67400_e104166_d_n6, assign67400_e104166_d_n7, assign67400_e104166_d_n8, assign67400_e104166_d_n9, assign67400_e104166_d_n10, assign67400_e104166_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67400_e104160: f64 = (locals.var_t7 * locals.var_t2);
        let assign67400_e104162: f64 = (assign67400_e104160 * locals.var_t4);
        let assign67400_e104164: f64 = (assign67400_e104162 * locals.var_t4);
        (assign67400_e104164, ((((((locals.var_t7_dn0 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn0)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn0)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn0)), ((((((locals.var_t7_dn2 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn2)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn2)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn2)), ((((((locals.var_t7_dn4 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn4)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn4)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn4)), ((((((locals.var_t7_dn5 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn5)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn5)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn5)), ((((((locals.var_t7_dn6 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn6)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn6)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn6)), ((((((locals.var_t7_dn7 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn7)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn7)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn7)), ((((((locals.var_t7_dn8 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn8)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn8)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn8)), ((((((locals.var_t7_dn9 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn9)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn9)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn9)), ((((((locals.var_t7_dn10 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn10)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn10)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn10)), ((((((locals.var_t7_dn13 * locals.var_t2) + (locals.var_t7 * locals.var_t2_dn13)) * locals.var_t4) + (assign67400_e104160 * locals.var_t4_dn13)) * locals.var_t4) + (assign67400_e104162 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign67400_e104166;
        locals.var_t6_dn0 = assign67400_e104166_d_n0;
        locals.var_t6_dn2 = assign67400_e104166_d_n2;
        locals.var_t6_dn4 = assign67400_e104166_d_n4;
        locals.var_t6_dn5 = assign67400_e104166_d_n5;
        locals.var_t6_dn6 = assign67400_e104166_d_n6;
        locals.var_t6_dn7 = assign67400_e104166_d_n7;
        locals.var_t6_dn8 = assign67400_e104166_d_n8;
        locals.var_t6_dn9 = assign67400_e104166_d_n9;
        locals.var_t6_dn10 = assign67400_e104166_d_n10;
        locals.var_t6_dn13 = assign67400_e104166_d_n13;

        let (assign67410_e104179, assign67410_e104179_d_n0, assign67410_e104179_d_n2, assign67410_e104179_d_n4, assign67410_e104179_d_n5, assign67410_e104179_d_n6, assign67410_e104179_d_n7, assign67410_e104179_d_n8, assign67410_e104179_d_n9, assign67410_e104179_d_n10, assign67410_e104179_d_n13,) = {
    if ((locals.var_guard1580 == 0.0) && (locals.var_guard1589 != 0.0)) {
        let assign67410_e104173: f64 = (locals.var_uc_subld1 * locals.var_ids);
        let assign67410_e104175: f64 = (assign67410_e104173 * locals.var_t0);
        let assign67410_e104177: f64 = (assign67410_e104175 * locals.var_t2);
        (assign67410_e104177, (((((locals.var_uc_subld1 * locals.var_ids_dn0) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn0)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn0)), (((((locals.var_uc_subld1 * locals.var_ids_dn2) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn2)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn2)), (((((locals.var_uc_subld1 * locals.var_ids_dn4) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn4)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn4)), (((((locals.var_uc_subld1 * locals.var_ids_dn5) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn5)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn5)), (((((locals.var_uc_subld1 * locals.var_ids_dn6) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn6)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn6)), (((((locals.var_uc_subld1 * locals.var_ids_dn7) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn7)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn7)), (((((locals.var_uc_subld1 * locals.var_ids_dn8) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn8)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn8)), (((((locals.var_uc_subld1 * locals.var_ids_dn9) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn9)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn9)), (((((locals.var_uc_subld1 * locals.var_ids_dn10) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn10)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn10)), (((((locals.var_uc_subld1 * locals.var_ids_dn13) * locals.var_t0) + (assign67410_e104173 * locals.var_t0_dn13)) * locals.var_t2) + (assign67410_e104175 * locals.var_t2_dn13)),)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn13,)
    }
};
        locals.var_isubld = assign67410_e104179;
        locals.var_isubld_dn0 = assign67410_e104179_d_n0;
        locals.var_isubld_dn2 = assign67410_e104179_d_n2;
        locals.var_isubld_dn4 = assign67410_e104179_d_n4;
        locals.var_isubld_dn5 = assign67410_e104179_d_n5;
        locals.var_isubld_dn6 = assign67410_e104179_d_n6;
        locals.var_isubld_dn7 = assign67410_e104179_d_n7;
        locals.var_isubld_dn8 = assign67410_e104179_d_n8;
        locals.var_isubld_dn9 = assign67410_e104179_d_n9;
        locals.var_isubld_dn10 = assign67410_e104179_d_n10;
        locals.var_isubld_dn13 = assign67410_e104179_d_n13;

        let assign67420_e104182: f64 = if p.p45 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1593 = assign67420_e104182;

        let (assign67430_e104186, assign67430_e104186_d_n0, assign67430_e104186_d_n2, assign67430_e104186_d_n4, assign67430_e104186_d_n5, assign67430_e104186_d_n6, assign67430_e104186_d_n7, assign67430_e104186_d_n8, assign67430_e104186_d_n9, assign67430_e104186_d_n10, assign67430_e104186_d_n13,) = {
    if (locals.var_guard1593 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn13,)
    }
};
        locals.var_ibreakhe = assign67430_e104186;
        locals.var_ibreakhe_dn0 = assign67430_e104186_d_n0;
        locals.var_ibreakhe_dn2 = assign67430_e104186_d_n2;
        locals.var_ibreakhe_dn4 = assign67430_e104186_d_n4;
        locals.var_ibreakhe_dn5 = assign67430_e104186_d_n5;
        locals.var_ibreakhe_dn6 = assign67430_e104186_d_n6;
        locals.var_ibreakhe_dn7 = assign67430_e104186_d_n7;
        locals.var_ibreakhe_dn8 = assign67430_e104186_d_n8;
        locals.var_ibreakhe_dn9 = assign67430_e104186_d_n9;
        locals.var_ibreakhe_dn10 = assign67430_e104186_d_n10;
        locals.var_ibreakhe_dn13 = assign67430_e104186_d_n13;

        let assign67440_e104190: f64 = (locals.var_vgse - p.p446);
        let assign67440_e104191: f64 = (p.p45 * assign67440_e104190);
        let assign67440_e104193: f64 = if assign67440_e104191 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1594 = assign67440_e104193;

        let (assign67450_e104200, assign67450_e104200_d_n0, assign67450_e104200_d_n2, assign67450_e104200_d_n4, assign67450_e104200_d_n5, assign67450_e104200_d_n6, assign67450_e104200_d_n7, assign67450_e104200_d_n8, assign67450_e104200_d_n9, assign67450_e104200_d_n10, assign67450_e104200_d_n13,) = {
    if ((locals.var_guard1593 == 0.0) && (locals.var_guard1594 != 0.0)) {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn13,)
    } else {
        (locals.var_hbdv, locals.var_hbdv_dn0, locals.var_hbdv_dn2, locals.var_hbdv_dn4, locals.var_hbdv_dn5, locals.var_hbdv_dn6, locals.var_hbdv_dn7, locals.var_hbdv_dn8, locals.var_hbdv_dn9, locals.var_hbdv_dn10, locals.var_hbdv_dn13,)
    }
};
        locals.var_hbdv = assign67450_e104200;
        locals.var_hbdv_dn0 = assign67450_e104200_d_n0;
        locals.var_hbdv_dn2 = assign67450_e104200_d_n2;
        locals.var_hbdv_dn4 = assign67450_e104200_d_n4;
        locals.var_hbdv_dn5 = assign67450_e104200_d_n5;
        locals.var_hbdv_dn6 = assign67450_e104200_d_n6;
        locals.var_hbdv_dn7 = assign67450_e104200_d_n7;
        locals.var_hbdv_dn8 = assign67450_e104200_d_n8;
        locals.var_hbdv_dn9 = assign67450_e104200_d_n9;
        locals.var_hbdv_dn10 = assign67450_e104200_d_n10;
        locals.var_hbdv_dn13 = assign67450_e104200_d_n13;

        let (assign67460_e104216, assign67460_e104216_d_n0, assign67460_e104216_d_n2, assign67460_e104216_d_n4, assign67460_e104216_d_n5, assign67460_e104216_d_n6, assign67460_e104216_d_n7, assign67460_e104216_d_n8, assign67460_e104216_d_n9, assign67460_e104216_d_n10, assign67460_e104216_d_n13,) = {
    if ((locals.var_guard1593 == 0.0) && (locals.var_guard1594 == 0.0)) {
        let assign67460_e104209: f64 = (locals.var_vgse - p.p446);
        let assign67460_e104211: f64 = (assign67460_e104209).powf(2.0);
        let assign67460_e104212: f64 = (p.p445 * assign67460_e104211);
        let assign67460_e104214: f64 = (assign67460_e104212 + locals.var_hbdceff);
        (assign67460_e104214, ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67460_e104209).powf(2.0 - 1.0) * locals.var_vgse_dn0)) } } else { (assign67460_e104211 * (2.0 * (locals.var_vgse_dn0 / assign67460_e104209))) }) + locals.var_hbdceff_dn0), ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67460_e104209).powf(2.0 - 1.0) * locals.var_vgse_dn2)) } } else { (assign67460_e104211 * (2.0 * (locals.var_vgse_dn2 / assign67460_e104209))) }) + locals.var_hbdceff_dn2), locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, ((p.p445 * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign67460_e104209).powf(2.0 - 1.0) * locals.var_vgse_dn6)) } } else { (assign67460_e104211 * (2.0 * (locals.var_vgse_dn6 / assign67460_e104209))) }) + locals.var_hbdceff_dn6), locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn13,)
    } else {
        (locals.var_hbdv, locals.var_hbdv_dn0, locals.var_hbdv_dn2, locals.var_hbdv_dn4, locals.var_hbdv_dn5, locals.var_hbdv_dn6, locals.var_hbdv_dn7, locals.var_hbdv_dn8, locals.var_hbdv_dn9, locals.var_hbdv_dn10, locals.var_hbdv_dn13,)
    }
};
        locals.var_hbdv = assign67460_e104216;
        locals.var_hbdv_dn0 = assign67460_e104216_d_n0;
        locals.var_hbdv_dn2 = assign67460_e104216_d_n2;
        locals.var_hbdv_dn4 = assign67460_e104216_d_n4;
        locals.var_hbdv_dn5 = assign67460_e104216_d_n5;
        locals.var_hbdv_dn6 = assign67460_e104216_d_n6;
        locals.var_hbdv_dn7 = assign67460_e104216_d_n7;
        locals.var_hbdv_dn8 = assign67460_e104216_d_n8;
        locals.var_hbdv_dn9 = assign67460_e104216_d_n9;
        locals.var_hbdv_dn10 = assign67460_e104216_d_n10;
        locals.var_hbdv_dn13 = assign67460_e104216_d_n13;

        let (assign67470_e104228, assign67470_e104228_d_n0, assign67470_e104228_d_n2, assign67470_e104228_d_n4, assign67470_e104228_d_n5, assign67470_e104228_d_n6, assign67470_e104228_d_n7, assign67470_e104228_d_n8, assign67470_e104228_d_n9, assign67470_e104228_d_n10, assign67470_e104228_d_n13,) = {
    if (locals.var_guard1593 == 0.0) {
        let assign67470_e104223: f64 = (locals.var_vdse - locals.var_hbdv);
        let assign67470_e104224: f64 = (locals.var_beta * assign67470_e104223);
        let assign67470_e104225: f64 = { let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign67470_e104226: f64 = (p.p449 * assign67470_e104225);
        (assign67470_e104226, (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn0 * assign67470_e104223) + (locals.var_beta * (locals.var_vdse_dn0 - locals.var_hbdv_dn0))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn2 * assign67470_e104223) + (locals.var_beta * (locals.var_vdse_dn2 - locals.var_hbdv_dn2))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn4 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn4))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn5 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn5))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn6 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn6))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn7 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn7))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn8 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn8))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn9 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn9))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn10 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn10))))), (p.p449 * ({ let limited_exp_arg = assign67470_e104224; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_beta_dn13 * assign67470_e104223) + (locals.var_beta * (-locals.var_hbdv_dn13))))),)
    } else {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn13,)
    }
};
        locals.var_ibreakhe = assign67470_e104228;
        locals.var_ibreakhe_dn0 = assign67470_e104228_d_n0;
        locals.var_ibreakhe_dn2 = assign67470_e104228_d_n2;
        locals.var_ibreakhe_dn4 = assign67470_e104228_d_n4;
        locals.var_ibreakhe_dn5 = assign67470_e104228_d_n5;
        locals.var_ibreakhe_dn6 = assign67470_e104228_d_n6;
        locals.var_ibreakhe_dn7 = assign67470_e104228_d_n7;
        locals.var_ibreakhe_dn8 = assign67470_e104228_d_n8;
        locals.var_ibreakhe_dn9 = assign67470_e104228_d_n9;
        locals.var_ibreakhe_dn10 = assign67470_e104228_d_n10;
        locals.var_ibreakhe_dn13 = assign67470_e104228_d_n13;

        let assign67480_e104231: f64 = if locals.var_ibreakhe > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1595 = assign67480_e104231;

        let assign67490_e104235: f64 = (100000.0 - 50000.0);
        let assign67490_e104240: f64 = if ((locals.var_ibreakhe > assign67490_e104235) && (50000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1596 = assign67490_e104240;

        let (assign67500_e104250, assign67500_e104250_d_n0, assign67500_e104250_d_n2, assign67500_e104250_d_n4, assign67500_e104250_d_n5, assign67500_e104250_d_n6, assign67500_e104250_d_n7, assign67500_e104250_d_n8, assign67500_e104250_d_n9, assign67500_e104250_d_n10, assign67500_e104250_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67500_e104246: f64 = (locals.var_ibreakhe - 100000.0);
        let assign67500_e104248: f64 = (assign67500_e104246 + 50000.0);
        (assign67500_e104248, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign67500_e104250;
        locals.var_tmf1_dn0 = assign67500_e104250_d_n0;
        locals.var_tmf1_dn2 = assign67500_e104250_d_n2;
        locals.var_tmf1_dn4 = assign67500_e104250_d_n4;
        locals.var_tmf1_dn5 = assign67500_e104250_d_n5;
        locals.var_tmf1_dn6 = assign67500_e104250_d_n6;
        locals.var_tmf1_dn7 = assign67500_e104250_d_n7;
        locals.var_tmf1_dn8 = assign67500_e104250_d_n8;
        locals.var_tmf1_dn9 = assign67500_e104250_d_n9;
        locals.var_tmf1_dn10 = assign67500_e104250_d_n10;
        locals.var_tmf1_dn13 = assign67500_e104250_d_n13;

        let (assign67510_e104258, assign67510_e104258_d_n0, assign67510_e104258_d_n2, assign67510_e104258_d_n4, assign67510_e104258_d_n5, assign67510_e104258_d_n6, assign67510_e104258_d_n7, assign67510_e104258_d_n8, assign67510_e104258_d_n9, assign67510_e104258_d_n10, assign67510_e104258_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67510_e104256: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign67510_e104256, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign67510_e104258;
        locals.var_x2_dn0 = assign67510_e104258_d_n0;
        locals.var_x2_dn2 = assign67510_e104258_d_n2;
        locals.var_x2_dn4 = assign67510_e104258_d_n4;
        locals.var_x2_dn5 = assign67510_e104258_d_n5;
        locals.var_x2_dn6 = assign67510_e104258_d_n6;
        locals.var_x2_dn7 = assign67510_e104258_d_n7;
        locals.var_x2_dn8 = assign67510_e104258_d_n8;
        locals.var_x2_dn9 = assign67510_e104258_d_n9;
        locals.var_x2_dn10 = assign67510_e104258_d_n10;
        locals.var_x2_dn13 = assign67510_e104258_d_n13;

        let (assign67520_e104266, assign67520_e104266_d_n0, assign67520_e104266_d_n2, assign67520_e104266_d_n4, assign67520_e104266_d_n5, assign67520_e104266_d_n6, assign67520_e104266_d_n7, assign67520_e104266_d_n8, assign67520_e104266_d_n9, assign67520_e104266_d_n10, assign67520_e104266_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67520_e104264: f64 = (50000.0 * 50000.0);
        (assign67520_e104264, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign67520_e104266;
        locals.var_xmax2_dn0 = assign67520_e104266_d_n0;
        locals.var_xmax2_dn2 = assign67520_e104266_d_n2;
        locals.var_xmax2_dn4 = assign67520_e104266_d_n4;
        locals.var_xmax2_dn5 = assign67520_e104266_d_n5;
        locals.var_xmax2_dn6 = assign67520_e104266_d_n6;
        locals.var_xmax2_dn7 = assign67520_e104266_d_n7;
        locals.var_xmax2_dn8 = assign67520_e104266_d_n8;
        locals.var_xmax2_dn9 = assign67520_e104266_d_n9;
        locals.var_xmax2_dn10 = assign67520_e104266_d_n10;
        locals.var_xmax2_dn13 = assign67520_e104266_d_n13;

        let (assign67530_e104272, assign67530_e104272_d_n0, assign67530_e104272_d_n2, assign67530_e104272_d_n4, assign67530_e104272_d_n5, assign67530_e104272_d_n6, assign67530_e104272_d_n7, assign67530_e104272_d_n8, assign67530_e104272_d_n9, assign67530_e104272_d_n10, assign67530_e104272_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign67530_e104272;
        locals.var_xp_dn0 = assign67530_e104272_d_n0;
        locals.var_xp_dn2 = assign67530_e104272_d_n2;
        locals.var_xp_dn4 = assign67530_e104272_d_n4;
        locals.var_xp_dn5 = assign67530_e104272_d_n5;
        locals.var_xp_dn6 = assign67530_e104272_d_n6;
        locals.var_xp_dn7 = assign67530_e104272_d_n7;
        locals.var_xp_dn8 = assign67530_e104272_d_n8;
        locals.var_xp_dn9 = assign67530_e104272_d_n9;
        locals.var_xp_dn10 = assign67530_e104272_d_n10;
        locals.var_xp_dn13 = assign67530_e104272_d_n13;

        let (assign67540_e104278, assign67540_e104278_d_n0, assign67540_e104278_d_n2, assign67540_e104278_d_n4, assign67540_e104278_d_n5, assign67540_e104278_d_n6, assign67540_e104278_d_n7, assign67540_e104278_d_n8, assign67540_e104278_d_n9, assign67540_e104278_d_n10, assign67540_e104278_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign67540_e104278;
        locals.var_xmp_dn0 = assign67540_e104278_d_n0;
        locals.var_xmp_dn2 = assign67540_e104278_d_n2;
        locals.var_xmp_dn4 = assign67540_e104278_d_n4;
        locals.var_xmp_dn5 = assign67540_e104278_d_n5;
        locals.var_xmp_dn6 = assign67540_e104278_d_n6;
        locals.var_xmp_dn7 = assign67540_e104278_d_n7;
        locals.var_xmp_dn8 = assign67540_e104278_d_n8;
        locals.var_xmp_dn9 = assign67540_e104278_d_n9;
        locals.var_xmp_dn10 = assign67540_e104278_d_n10;
        locals.var_xmp_dn13 = assign67540_e104278_d_n13;

        let (assign67550_e104284,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign67550_e104284;

        let (assign67560_e104290,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67560_e104290;

        let (assign67570_e104296, assign67570_e104296_d_n0, assign67570_e104296_d_n2, assign67570_e104296_d_n4, assign67570_e104296_d_n5, assign67570_e104296_d_n6, assign67570_e104296_d_n7, assign67570_e104296_d_n8, assign67570_e104296_d_n9, assign67570_e104296_d_n10, assign67570_e104296_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign67570_e104296;
        locals.var_arg_dn0 = assign67570_e104296_d_n0;
        locals.var_arg_dn2 = assign67570_e104296_d_n2;
        locals.var_arg_dn4 = assign67570_e104296_d_n4;
        locals.var_arg_dn5 = assign67570_e104296_d_n5;
        locals.var_arg_dn6 = assign67570_e104296_d_n6;
        locals.var_arg_dn7 = assign67570_e104296_d_n7;
        locals.var_arg_dn8 = assign67570_e104296_d_n8;
        locals.var_arg_dn9 = assign67570_e104296_d_n9;
        locals.var_arg_dn10 = assign67570_e104296_d_n10;
        locals.var_arg_dn13 = assign67570_e104296_d_n13;

        let (assign67580_e104302, assign67580_e104302_d_n0, assign67580_e104302_d_n2, assign67580_e104302_d_n4, assign67580_e104302_d_n5, assign67580_e104302_d_n6, assign67580_e104302_d_n7, assign67580_e104302_d_n8, assign67580_e104302_d_n9, assign67580_e104302_d_n10, assign67580_e104302_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign67580_e104302;
        locals.var_dnm_dn0 = assign67580_e104302_d_n0;
        locals.var_dnm_dn2 = assign67580_e104302_d_n2;
        locals.var_dnm_dn4 = assign67580_e104302_d_n4;
        locals.var_dnm_dn5 = assign67580_e104302_d_n5;
        locals.var_dnm_dn6 = assign67580_e104302_d_n6;
        locals.var_dnm_dn7 = assign67580_e104302_d_n7;
        locals.var_dnm_dn8 = assign67580_e104302_d_n8;
        locals.var_dnm_dn9 = assign67580_e104302_d_n9;
        locals.var_dnm_dn10 = assign67580_e104302_d_n10;
        locals.var_dnm_dn13 = assign67580_e104302_d_n13;

        let (assign67590_e104310, assign67590_e104310_d_n0, assign67590_e104310_d_n2, assign67590_e104310_d_n4, assign67590_e104310_d_n5, assign67590_e104310_d_n6, assign67590_e104310_d_n7, assign67590_e104310_d_n8, assign67590_e104310_d_n9, assign67590_e104310_d_n10, assign67590_e104310_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67590_e104308: f64 = (locals.var_xp * locals.var_x2);
        (assign67590_e104308, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign67590_e104310;
        locals.var_xp_dn0 = assign67590_e104310_d_n0;
        locals.var_xp_dn2 = assign67590_e104310_d_n2;
        locals.var_xp_dn4 = assign67590_e104310_d_n4;
        locals.var_xp_dn5 = assign67590_e104310_d_n5;
        locals.var_xp_dn6 = assign67590_e104310_d_n6;
        locals.var_xp_dn7 = assign67590_e104310_d_n7;
        locals.var_xp_dn8 = assign67590_e104310_d_n8;
        locals.var_xp_dn9 = assign67590_e104310_d_n9;
        locals.var_xp_dn10 = assign67590_e104310_d_n10;
        locals.var_xp_dn13 = assign67590_e104310_d_n13;

        let (assign67600_e104318, assign67600_e104318_d_n0, assign67600_e104318_d_n2, assign67600_e104318_d_n4, assign67600_e104318_d_n5, assign67600_e104318_d_n6, assign67600_e104318_d_n7, assign67600_e104318_d_n8, assign67600_e104318_d_n9, assign67600_e104318_d_n10, assign67600_e104318_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67600_e104316: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign67600_e104316, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign67600_e104318;
        locals.var_xmp_dn0 = assign67600_e104318_d_n0;
        locals.var_xmp_dn2 = assign67600_e104318_d_n2;
        locals.var_xmp_dn4 = assign67600_e104318_d_n4;
        locals.var_xmp_dn5 = assign67600_e104318_d_n5;
        locals.var_xmp_dn6 = assign67600_e104318_d_n6;
        locals.var_xmp_dn7 = assign67600_e104318_d_n7;
        locals.var_xmp_dn8 = assign67600_e104318_d_n8;
        locals.var_xmp_dn9 = assign67600_e104318_d_n9;
        locals.var_xmp_dn10 = assign67600_e104318_d_n10;
        locals.var_xmp_dn13 = assign67600_e104318_d_n13;

        let (assign67610_e104326, assign67610_e104326_d_n0, assign67610_e104326_d_n2, assign67610_e104326_d_n4, assign67610_e104326_d_n5, assign67610_e104326_d_n6, assign67610_e104326_d_n7, assign67610_e104326_d_n8, assign67610_e104326_d_n9, assign67610_e104326_d_n10, assign67610_e104326_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67610_e104324: f64 = (locals.var_xp + locals.var_xmp);
        (assign67610_e104324, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign67610_e104326;
        locals.var_arg_dn0 = assign67610_e104326_d_n0;
        locals.var_arg_dn2 = assign67610_e104326_d_n2;
        locals.var_arg_dn4 = assign67610_e104326_d_n4;
        locals.var_arg_dn5 = assign67610_e104326_d_n5;
        locals.var_arg_dn6 = assign67610_e104326_d_n6;
        locals.var_arg_dn7 = assign67610_e104326_d_n7;
        locals.var_arg_dn8 = assign67610_e104326_d_n8;
        locals.var_arg_dn9 = assign67610_e104326_d_n9;
        locals.var_arg_dn10 = assign67610_e104326_d_n10;
        locals.var_arg_dn13 = assign67610_e104326_d_n13;

        let (assign67620_e104332, assign67620_e104332_d_n0, assign67620_e104332_d_n2, assign67620_e104332_d_n4, assign67620_e104332_d_n5, assign67620_e104332_d_n6, assign67620_e104332_d_n7, assign67620_e104332_d_n8, assign67620_e104332_d_n9, assign67620_e104332_d_n10, assign67620_e104332_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign67620_e104332;
        locals.var_dnm_dn0 = assign67620_e104332_d_n0;
        locals.var_dnm_dn2 = assign67620_e104332_d_n2;
        locals.var_dnm_dn4 = assign67620_e104332_d_n4;
        locals.var_dnm_dn5 = assign67620_e104332_d_n5;
        locals.var_dnm_dn6 = assign67620_e104332_d_n6;
        locals.var_dnm_dn7 = assign67620_e104332_d_n7;
        locals.var_dnm_dn8 = assign67620_e104332_d_n8;
        locals.var_dnm_dn9 = assign67620_e104332_d_n9;
        locals.var_dnm_dn10 = assign67620_e104332_d_n10;
        locals.var_dnm_dn13 = assign67620_e104332_d_n13;

        let assign67630_e104347: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1597 = assign67630_e104347;

        let assign67640_e104350: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1598 = assign67640_e104350;

    }

    pub(super) fn stamp_transient_block_231(
        locals: &mut StampLocals,
    ) {
        let (assign67650_e104360,) = {
    if ((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_guard1598 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67650_e104360;

        let assign67660_e104363: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1599 = assign67660_e104363;

        let (assign67670_e104376,) = {
    if (((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_guard1598 == 0.0)) && (locals.var_guard1599 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67670_e104376;

        let assign67680_e104379: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1600 = assign67680_e104379;

        let (assign67690_e104395,) = {
    if ((((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_guard1598 == 0.0)) && (locals.var_guard1599 == 0.0)) && (locals.var_guard1600 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67690_e104395;

        let assign67700_e104398: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1601 = assign67700_e104398;

        let (assign67710_e104417,) = {
    if (((((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_guard1598 == 0.0)) && (locals.var_guard1599 == 0.0)) && (locals.var_guard1600 == 0.0)) && (locals.var_guard1601 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign67710_e104417;

        let (assign67720_e104425,) = {
    if (((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign67720_e104425;

        let mut assign67730_loop_guard: usize = 0;
        while {
            let assign67730_cond_e104434: f64 = if ((((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign67730_cond_e104434 != 0.0
        } {
            assign67730_loop_guard += 1;
            assert!(assign67730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign67730_body0_e104443, assign67730_body0_e104443_d_n0, assign67730_body0_e104443_d_n2, assign67730_body0_e104443_d_n4, assign67730_body0_e104443_d_n5, assign67730_body0_e104443_d_n6, assign67730_body0_e104443_d_n7, assign67730_body0_e104443_d_n8, assign67730_body0_e104443_d_n9, assign67730_body0_e104443_d_n10, assign67730_body0_e104443_d_n13,) = {
    if (((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) {
        let assign67730_body0_e104441: f64 = (locals.var_dnm).sqrt();
        (assign67730_body0_e104441, (locals.var_dnm_dn0 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn2 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn4 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn5 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn6 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn7 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn8 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn9 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn10 / (2.0 * assign67730_body0_e104441)), (locals.var_dnm_dn13 / (2.0 * assign67730_body0_e104441)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign67730_body0_e104443;
            locals.var_dnm_dn0 = assign67730_body0_e104443_d_n0;
            locals.var_dnm_dn2 = assign67730_body0_e104443_d_n2;
            locals.var_dnm_dn4 = assign67730_body0_e104443_d_n4;
            locals.var_dnm_dn5 = assign67730_body0_e104443_d_n5;
            locals.var_dnm_dn6 = assign67730_body0_e104443_d_n6;
            locals.var_dnm_dn7 = assign67730_body0_e104443_d_n7;
            locals.var_dnm_dn8 = assign67730_body0_e104443_d_n8;
            locals.var_dnm_dn9 = assign67730_body0_e104443_d_n9;
            locals.var_dnm_dn10 = assign67730_body0_e104443_d_n10;
            locals.var_dnm_dn13 = assign67730_body0_e104443_d_n13;
            let (assign67730_body1_e104453,) = {
    if (((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 != 0.0)) {
        let assign67730_body1_e104451: f64 = (locals.var_m0 + 1.0);
        (assign67730_body1_e104451,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign67730_body1_e104453;
        }

        let (assign67740_e104473, assign67740_e104473_d_n0, assign67740_e104473_d_n2, assign67740_e104473_d_n4, assign67740_e104473_d_n5, assign67740_e104473_d_n6, assign67740_e104473_d_n7, assign67740_e104473_d_n8, assign67740_e104473_d_n9, assign67740_e104473_d_n10, assign67740_e104473_d_n13,) = {
    if (((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) && (locals.var_guard1597 == 0.0)) {
        let (assign67740_e104471, assign67740_e104471_d_n0, assign67740_e104471_d_n2, assign67740_e104471_d_n4, assign67740_e104471_d_n5, assign67740_e104471_d_n6, assign67740_e104471_d_n7, assign67740_e104471_d_n8, assign67740_e104471_d_n9, assign67740_e104471_d_n10, assign67740_e104471_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign67740_e104468: f64 = 2.0;
                let assign67740_e104469: f64 = (1.0 / assign67740_e104468);
                let assign67740_e104470: f64 = (locals.var_dnm).powf(assign67740_e104469);
                (assign67740_e104470, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn0)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn2)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn4)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn5)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn6)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn7)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn8)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn9)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn10)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign67740_e104469) as f64).is_finite() && ((assign67740_e104469) as f64).fract() == 0.0 { if assign67740_e104469 == 0.0 { 0.0 } else { (assign67740_e104469 * ((locals.var_dnm).powf(assign67740_e104469 - 1.0) * locals.var_dnm_dn13)) } } else { (assign67740_e104470 * (assign67740_e104469 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign67740_e104471, assign67740_e104471_d_n0, assign67740_e104471_d_n2, assign67740_e104471_d_n4, assign67740_e104471_d_n5, assign67740_e104471_d_n6, assign67740_e104471_d_n7, assign67740_e104471_d_n8, assign67740_e104471_d_n9, assign67740_e104471_d_n10, assign67740_e104471_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign67740_e104473;
        locals.var_dnm_dn0 = assign67740_e104473_d_n0;
        locals.var_dnm_dn2 = assign67740_e104473_d_n2;
        locals.var_dnm_dn4 = assign67740_e104473_d_n4;
        locals.var_dnm_dn5 = assign67740_e104473_d_n5;
        locals.var_dnm_dn6 = assign67740_e104473_d_n6;
        locals.var_dnm_dn7 = assign67740_e104473_d_n7;
        locals.var_dnm_dn8 = assign67740_e104473_d_n8;
        locals.var_dnm_dn9 = assign67740_e104473_d_n9;
        locals.var_dnm_dn10 = assign67740_e104473_d_n10;
        locals.var_dnm_dn13 = assign67740_e104473_d_n13;

        let (assign67750_e104481, assign67750_e104481_d_n0, assign67750_e104481_d_n2, assign67750_e104481_d_n4, assign67750_e104481_d_n5, assign67750_e104481_d_n6, assign67750_e104481_d_n7, assign67750_e104481_d_n8, assign67750_e104481_d_n9, assign67750_e104481_d_n10, assign67750_e104481_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67750_e104479: f64 = (1.0 / locals.var_dnm);
        (assign67750_e104479, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign67750_e104481;
        locals.var_dnm_dn0 = assign67750_e104481_d_n0;
        locals.var_dnm_dn2 = assign67750_e104481_d_n2;
        locals.var_dnm_dn4 = assign67750_e104481_d_n4;
        locals.var_dnm_dn5 = assign67750_e104481_d_n5;
        locals.var_dnm_dn6 = assign67750_e104481_d_n6;
        locals.var_dnm_dn7 = assign67750_e104481_d_n7;
        locals.var_dnm_dn8 = assign67750_e104481_d_n8;
        locals.var_dnm_dn9 = assign67750_e104481_d_n9;
        locals.var_dnm_dn10 = assign67750_e104481_d_n10;
        locals.var_dnm_dn13 = assign67750_e104481_d_n13;

        let (assign67760_e104491, assign67760_e104491_d_n0, assign67760_e104491_d_n2, assign67760_e104491_d_n4, assign67760_e104491_d_n5, assign67760_e104491_d_n6, assign67760_e104491_d_n7, assign67760_e104491_d_n8, assign67760_e104491_d_n9, assign67760_e104491_d_n10, assign67760_e104491_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67760_e104487: f64 = (locals.var_tmf1 * 50000.0);
        let assign67760_e104489: f64 = (assign67760_e104487 * locals.var_dnm);
        (assign67760_e104489, (((locals.var_tmf1_dn0 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 50000.0) * locals.var_dnm) + (assign67760_e104487 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign67760_e104491;
        locals.var_tmf0_dn0 = assign67760_e104491_d_n0;
        locals.var_tmf0_dn2 = assign67760_e104491_d_n2;
        locals.var_tmf0_dn4 = assign67760_e104491_d_n4;
        locals.var_tmf0_dn5 = assign67760_e104491_d_n5;
        locals.var_tmf0_dn6 = assign67760_e104491_d_n6;
        locals.var_tmf0_dn7 = assign67760_e104491_d_n7;
        locals.var_tmf0_dn8 = assign67760_e104491_d_n8;
        locals.var_tmf0_dn9 = assign67760_e104491_d_n9;
        locals.var_tmf0_dn10 = assign67760_e104491_d_n10;
        locals.var_tmf0_dn13 = assign67760_e104491_d_n13;

        let (assign67770_e104503, assign67770_e104503_d_n0, assign67770_e104503_d_n2, assign67770_e104503_d_n4, assign67770_e104503_d_n5, assign67770_e104503_d_n6, assign67770_e104503_d_n7, assign67770_e104503_d_n8, assign67770_e104503_d_n9, assign67770_e104503_d_n10, assign67770_e104503_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67770_e104497: f64 = (50000.0 * locals.var_xmp);
        let assign67770_e104499: f64 = (assign67770_e104497 * locals.var_dnm);
        let assign67770_e104501: f64 = (assign67770_e104499 / locals.var_arg);
        (assign67770_e104501, ((((((50000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn0)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn2)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn4)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn5)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn6)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn7)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn8)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn9)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn10)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((50000.0 * locals.var_xmp_dn13) * locals.var_dnm) + (assign67770_e104497 * locals.var_dnm_dn13)) * locals.var_arg) - (assign67770_e104499 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67770_e104503;
        locals.var_t0_dn0 = assign67770_e104503_d_n0;
        locals.var_t0_dn2 = assign67770_e104503_d_n2;
        locals.var_t0_dn4 = assign67770_e104503_d_n4;
        locals.var_t0_dn5 = assign67770_e104503_d_n5;
        locals.var_t0_dn6 = assign67770_e104503_d_n6;
        locals.var_t0_dn7 = assign67770_e104503_d_n7;
        locals.var_t0_dn8 = assign67770_e104503_d_n8;
        locals.var_t0_dn9 = assign67770_e104503_d_n9;
        locals.var_t0_dn10 = assign67770_e104503_d_n10;
        locals.var_t0_dn13 = assign67770_e104503_d_n13;

        let (assign67780_e104513, assign67780_e104513_d_n0, assign67780_e104513_d_n2, assign67780_e104513_d_n4, assign67780_e104513_d_n5, assign67780_e104513_d_n6, assign67780_e104513_d_n7, assign67780_e104513_d_n8, assign67780_e104513_d_n9, assign67780_e104513_d_n10, assign67780_e104513_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        let assign67780_e104509: f64 = (100000.0 - 50000.0);
        let assign67780_e104511: f64 = (assign67780_e104509 + locals.var_tmf0);
        (assign67780_e104511, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67780_e104513;
        locals.var_t2_dn0 = assign67780_e104513_d_n0;
        locals.var_t2_dn2 = assign67780_e104513_d_n2;
        locals.var_t2_dn4 = assign67780_e104513_d_n4;
        locals.var_t2_dn5 = assign67780_e104513_d_n5;
        locals.var_t2_dn6 = assign67780_e104513_d_n6;
        locals.var_t2_dn7 = assign67780_e104513_d_n7;
        locals.var_t2_dn8 = assign67780_e104513_d_n8;
        locals.var_t2_dn9 = assign67780_e104513_d_n9;
        locals.var_t2_dn10 = assign67780_e104513_d_n10;
        locals.var_t2_dn13 = assign67780_e104513_d_n13;

        let (assign67790_e104519, assign67790_e104519_d_n0, assign67790_e104519_d_n2, assign67790_e104519_d_n4, assign67790_e104519_d_n5, assign67790_e104519_d_n6, assign67790_e104519_d_n7, assign67790_e104519_d_n8, assign67790_e104519_d_n9, assign67790_e104519_d_n10, assign67790_e104519_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67790_e104519;
        locals.var_t0_dn0 = assign67790_e104519_d_n0;
        locals.var_t0_dn2 = assign67790_e104519_d_n2;
        locals.var_t0_dn4 = assign67790_e104519_d_n4;
        locals.var_t0_dn5 = assign67790_e104519_d_n5;
        locals.var_t0_dn6 = assign67790_e104519_d_n6;
        locals.var_t0_dn7 = assign67790_e104519_d_n7;
        locals.var_t0_dn8 = assign67790_e104519_d_n8;
        locals.var_t0_dn9 = assign67790_e104519_d_n9;
        locals.var_t0_dn10 = assign67790_e104519_d_n10;
        locals.var_t0_dn13 = assign67790_e104519_d_n13;

        let (assign67800_e104526, assign67800_e104526_d_n0, assign67800_e104526_d_n2, assign67800_e104526_d_n4, assign67800_e104526_d_n5, assign67800_e104526_d_n6, assign67800_e104526_d_n7, assign67800_e104526_d_n8, assign67800_e104526_d_n9, assign67800_e104526_d_n10, assign67800_e104526_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 == 0.0)) {
        (locals.var_ibreakhe, locals.var_ibreakhe_dn0, locals.var_ibreakhe_dn2, locals.var_ibreakhe_dn4, locals.var_ibreakhe_dn5, locals.var_ibreakhe_dn6, locals.var_ibreakhe_dn7, locals.var_ibreakhe_dn8, locals.var_ibreakhe_dn9, locals.var_ibreakhe_dn10, locals.var_ibreakhe_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67800_e104526;
        locals.var_t2_dn0 = assign67800_e104526_d_n0;
        locals.var_t2_dn2 = assign67800_e104526_d_n2;
        locals.var_t2_dn4 = assign67800_e104526_d_n4;
        locals.var_t2_dn5 = assign67800_e104526_d_n5;
        locals.var_t2_dn6 = assign67800_e104526_d_n6;
        locals.var_t2_dn7 = assign67800_e104526_d_n7;
        locals.var_t2_dn8 = assign67800_e104526_d_n8;
        locals.var_t2_dn9 = assign67800_e104526_d_n9;
        locals.var_t2_dn10 = assign67800_e104526_d_n10;
        locals.var_t2_dn13 = assign67800_e104526_d_n13;

        let (assign67810_e104533, assign67810_e104533_d_n0, assign67810_e104533_d_n2, assign67810_e104533_d_n4, assign67810_e104533_d_n5, assign67810_e104533_d_n6, assign67810_e104533_d_n7, assign67810_e104533_d_n8, assign67810_e104533_d_n9, assign67810_e104533_d_n10, assign67810_e104533_d_n13,) = {
    if ((locals.var_guard1595 != 0.0) && (locals.var_guard1596 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67810_e104533;
        locals.var_t0_dn0 = assign67810_e104533_d_n0;
        locals.var_t0_dn2 = assign67810_e104533_d_n2;
        locals.var_t0_dn4 = assign67810_e104533_d_n4;
        locals.var_t0_dn5 = assign67810_e104533_d_n5;
        locals.var_t0_dn6 = assign67810_e104533_d_n6;
        locals.var_t0_dn7 = assign67810_e104533_d_n7;
        locals.var_t0_dn8 = assign67810_e104533_d_n8;
        locals.var_t0_dn9 = assign67810_e104533_d_n9;
        locals.var_t0_dn10 = assign67810_e104533_d_n10;
        locals.var_t0_dn13 = assign67810_e104533_d_n13;

        let (assign67820_e104541, assign67820_e104541_d_n0, assign67820_e104541_d_n2, assign67820_e104541_d_n4, assign67820_e104541_d_n5, assign67820_e104541_d_n6, assign67820_e104541_d_n7, assign67820_e104541_d_n8, assign67820_e104541_d_n9, assign67820_e104541_d_n10, assign67820_e104541_d_n13,) = {
    if (locals.var_guard1595 != 0.0) {
        let assign67820_e104537: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign67820_e104539: f64 = (assign67820_e104537 * locals.var_t2);
        (assign67820_e104539, (assign67820_e104537 * locals.var_t2_dn0), (assign67820_e104537 * locals.var_t2_dn2), (assign67820_e104537 * locals.var_t2_dn4), (assign67820_e104537 * locals.var_t2_dn5), (assign67820_e104537 * locals.var_t2_dn6), (assign67820_e104537 * locals.var_t2_dn7), (assign67820_e104537 * locals.var_t2_dn8), (assign67820_e104537 * locals.var_t2_dn9), (assign67820_e104537 * locals.var_t2_dn10), (assign67820_e104537 * locals.var_t2_dn13),)
    } else {
        (locals.var_ibreake, locals.var_ibreake_dn0, locals.var_ibreake_dn2, locals.var_ibreake_dn4, locals.var_ibreake_dn5, locals.var_ibreake_dn6, locals.var_ibreake_dn7, locals.var_ibreake_dn8, locals.var_ibreake_dn9, locals.var_ibreake_dn10, locals.var_ibreake_dn13,)
    }
};
        locals.var_ibreake = assign67820_e104541;
        locals.var_ibreake_dn0 = assign67820_e104541_d_n0;
        locals.var_ibreake_dn2 = assign67820_e104541_d_n2;
        locals.var_ibreake_dn4 = assign67820_e104541_d_n4;
        locals.var_ibreake_dn5 = assign67820_e104541_d_n5;
        locals.var_ibreake_dn6 = assign67820_e104541_d_n6;
        locals.var_ibreake_dn7 = assign67820_e104541_d_n7;
        locals.var_ibreake_dn8 = assign67820_e104541_d_n8;
        locals.var_ibreake_dn9 = assign67820_e104541_d_n9;
        locals.var_ibreake_dn10 = assign67820_e104541_d_n10;
        locals.var_ibreake_dn13 = assign67820_e104541_d_n13;

        let (assign67830_e104546, assign67830_e104546_d_n0, assign67830_e104546_d_n2, assign67830_e104546_d_n4, assign67830_e104546_d_n5, assign67830_e104546_d_n6, assign67830_e104546_d_n7, assign67830_e104546_d_n8, assign67830_e104546_d_n9, assign67830_e104546_d_n10, assign67830_e104546_d_n13,) = {
    if (locals.var_guard1595 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibreake, locals.var_ibreake_dn0, locals.var_ibreake_dn2, locals.var_ibreake_dn4, locals.var_ibreake_dn5, locals.var_ibreake_dn6, locals.var_ibreake_dn7, locals.var_ibreake_dn8, locals.var_ibreake_dn9, locals.var_ibreake_dn10, locals.var_ibreake_dn13,)
    }
};
        locals.var_ibreake = assign67830_e104546;
        locals.var_ibreake_dn0 = assign67830_e104546_d_n0;
        locals.var_ibreake_dn2 = assign67830_e104546_d_n2;
        locals.var_ibreake_dn4 = assign67830_e104546_d_n4;
        locals.var_ibreake_dn5 = assign67830_e104546_d_n5;
        locals.var_ibreake_dn6 = assign67830_e104546_d_n6;
        locals.var_ibreake_dn7 = assign67830_e104546_d_n7;
        locals.var_ibreake_dn8 = assign67830_e104546_d_n8;
        locals.var_ibreake_dn9 = assign67830_e104546_d_n9;
        locals.var_ibreake_dn10 = assign67830_e104546_d_n10;
        locals.var_ibreake_dn13 = assign67830_e104546_d_n13;

        let assign67840_e104549: f64 = (locals.var_isub + locals.var_isubld);
        let assign67840_e104559: f64 = if (((assign67840_e104549 > 0.0) && (locals.var_uc_ibpc1 != 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1602 = assign67840_e104559;

        let (assign67850_e104567, assign67850_e104567_d_n0, assign67850_e104567_d_n2, assign67850_e104567_d_n4, assign67850_e104567_d_n5, assign67850_e104567_d_n6, assign67850_e104567_d_n7, assign67850_e104567_d_n8, assign67850_e104567_d_n9, assign67850_e104567_d_n10, assign67850_e104567_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67850_e104564: f64 = (locals.var_uc_ibpc2 * locals.var_dvth);
        let assign67850_e104565: f64 = (1.0 + assign67850_e104564);
        (assign67850_e104565, (locals.var_uc_ibpc2 * locals.var_dvth_dn0), (locals.var_uc_ibpc2 * locals.var_dvth_dn2), (locals.var_uc_ibpc2 * locals.var_dvth_dn4), (locals.var_uc_ibpc2 * locals.var_dvth_dn5), (locals.var_uc_ibpc2 * locals.var_dvth_dn6), (locals.var_uc_ibpc2 * locals.var_dvth_dn7), (locals.var_uc_ibpc2 * locals.var_dvth_dn8), (locals.var_uc_ibpc2 * locals.var_dvth_dn9), (locals.var_uc_ibpc2 * locals.var_dvth_dn10), (locals.var_uc_ibpc2 * locals.var_dvth_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign67850_e104567;
        locals.var_t0_dn0 = assign67850_e104567_d_n0;
        locals.var_t0_dn2 = assign67850_e104567_d_n2;
        locals.var_t0_dn4 = assign67850_e104567_d_n4;
        locals.var_t0_dn5 = assign67850_e104567_d_n5;
        locals.var_t0_dn6 = assign67850_e104567_d_n6;
        locals.var_t0_dn7 = assign67850_e104567_d_n7;
        locals.var_t0_dn8 = assign67850_e104567_d_n8;
        locals.var_t0_dn9 = assign67850_e104567_d_n9;
        locals.var_t0_dn10 = assign67850_e104567_d_n10;
        locals.var_t0_dn13 = assign67850_e104567_d_n13;

        let (assign67860_e104573, assign67860_e104573_d_n0, assign67860_e104573_d_n2, assign67860_e104573_d_n4, assign67860_e104573_d_n5, assign67860_e104573_d_n6, assign67860_e104573_d_n7, assign67860_e104573_d_n8, assign67860_e104573_d_n9, assign67860_e104573_d_n10, assign67860_e104573_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67860_e104571: f64 = (locals.var_isub + locals.var_isubld);
        (assign67860_e104571, (locals.var_isub_dn0 + locals.var_isubld_dn0), (locals.var_isub_dn2 + locals.var_isubld_dn2), (locals.var_isub_dn4 + locals.var_isubld_dn4), (locals.var_isub_dn5 + locals.var_isubld_dn5), (locals.var_isub_dn6 + locals.var_isubld_dn6), (locals.var_isub_dn7 + locals.var_isubld_dn7), (locals.var_isub_dn8 + locals.var_isubld_dn8), (locals.var_isub_dn9 + locals.var_isubld_dn9), (locals.var_isub_dn10 + locals.var_isubld_dn10), (locals.var_isub_dn13 + locals.var_isubld_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67860_e104573;
        locals.var_t1_dn0 = assign67860_e104573_d_n0;
        locals.var_t1_dn2 = assign67860_e104573_d_n2;
        locals.var_t1_dn4 = assign67860_e104573_d_n4;
        locals.var_t1_dn5 = assign67860_e104573_d_n5;
        locals.var_t1_dn6 = assign67860_e104573_d_n6;
        locals.var_t1_dn7 = assign67860_e104573_d_n7;
        locals.var_t1_dn8 = assign67860_e104573_d_n8;
        locals.var_t1_dn9 = assign67860_e104573_d_n9;
        locals.var_t1_dn10 = assign67860_e104573_d_n10;
        locals.var_t1_dn13 = assign67860_e104573_d_n13;

        let (assign67870_e104581, assign67870_e104581_d_n0, assign67870_e104581_d_n2, assign67870_e104581_d_n4, assign67870_e104581_d_n5, assign67870_e104581_d_n6, assign67870_e104581_d_n7, assign67870_e104581_d_n8, assign67870_e104581_d_n9, assign67870_e104581_d_n10, assign67870_e104581_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67870_e104577: f64 = (locals.var_uc_ibpc1 * locals.var_t0);
        let assign67870_e104579: f64 = (assign67870_e104577 * locals.var_t1);
        (assign67870_e104579, (((locals.var_uc_ibpc1 * locals.var_t0_dn0) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn0)), (((locals.var_uc_ibpc1 * locals.var_t0_dn2) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn2)), (((locals.var_uc_ibpc1 * locals.var_t0_dn4) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn4)), (((locals.var_uc_ibpc1 * locals.var_t0_dn5) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn5)), (((locals.var_uc_ibpc1 * locals.var_t0_dn6) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn6)), (((locals.var_uc_ibpc1 * locals.var_t0_dn7) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn7)), (((locals.var_uc_ibpc1 * locals.var_t0_dn8) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn8)), (((locals.var_uc_ibpc1 * locals.var_t0_dn9) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn9)), (((locals.var_uc_ibpc1 * locals.var_t0_dn10) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn10)), (((locals.var_uc_ibpc1 * locals.var_t0_dn13) * locals.var_t1) + (assign67870_e104577 * locals.var_t1_dn13)),)
    } else {
        (locals.var_dvbsibpc, locals.var_dvbsibpc_dn0, locals.var_dvbsibpc_dn2, locals.var_dvbsibpc_dn4, locals.var_dvbsibpc_dn5, locals.var_dvbsibpc_dn6, locals.var_dvbsibpc_dn7, locals.var_dvbsibpc_dn8, locals.var_dvbsibpc_dn9, locals.var_dvbsibpc_dn10, locals.var_dvbsibpc_dn13,)
    }
};
        locals.var_dvbsibpc = assign67870_e104581;
        locals.var_dvbsibpc_dn0 = assign67870_e104581_d_n0;
        locals.var_dvbsibpc_dn2 = assign67870_e104581_d_n2;
        locals.var_dvbsibpc_dn4 = assign67870_e104581_d_n4;
        locals.var_dvbsibpc_dn5 = assign67870_e104581_d_n5;
        locals.var_dvbsibpc_dn6 = assign67870_e104581_d_n6;
        locals.var_dvbsibpc_dn7 = assign67870_e104581_d_n7;
        locals.var_dvbsibpc_dn8 = assign67870_e104581_d_n8;
        locals.var_dvbsibpc_dn9 = assign67870_e104581_d_n9;
        locals.var_dvbsibpc_dn10 = assign67870_e104581_d_n10;
        locals.var_dvbsibpc_dn13 = assign67870_e104581_d_n13;

        let (assign67880_e104587, assign67880_e104587_d_n0, assign67880_e104587_d_n2, assign67880_e104587_d_n4, assign67880_e104587_d_n5, assign67880_e104587_d_n6, assign67880_e104587_d_n7, assign67880_e104587_d_n8, assign67880_e104587_d_n9, assign67880_e104587_d_n10, assign67880_e104587_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67880_e104585: f64 = (1.0 / locals.var_xi0);
        (assign67880_e104585, (-(locals.var_xi0_dn0 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn2 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn4 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn5 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn6 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn7 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn8 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn9 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn10 / (locals.var_xi0 * locals.var_xi0))), (-(locals.var_xi0_dn13 / (locals.var_xi0 * locals.var_xi0))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign67880_e104587;
        locals.var_t10_dn0 = assign67880_e104587_d_n0;
        locals.var_t10_dn2 = assign67880_e104587_d_n2;
        locals.var_t10_dn4 = assign67880_e104587_d_n4;
        locals.var_t10_dn5 = assign67880_e104587_d_n5;
        locals.var_t10_dn6 = assign67880_e104587_d_n6;
        locals.var_t10_dn7 = assign67880_e104587_d_n7;
        locals.var_t10_dn8 = assign67880_e104587_d_n8;
        locals.var_t10_dn9 = assign67880_e104587_d_n9;
        locals.var_t10_dn10 = assign67880_e104587_d_n10;
        locals.var_t10_dn13 = assign67880_e104587_d_n13;

        let (assign67890_e104595, assign67890_e104595_d_n0, assign67890_e104595_d_n2, assign67890_e104595_d_n4, assign67890_e104595_d_n5, assign67890_e104595_d_n6, assign67890_e104595_d_n7, assign67890_e104595_d_n8, assign67890_e104595_d_n9, assign67890_e104595_d_n10, assign67890_e104595_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67890_e104591: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign67890_e104593: f64 = (assign67890_e104591 * locals.var_t10);
        (assign67890_e104593, ((((locals.var_beta_dn0 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn0)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn0)), ((((locals.var_beta_dn2 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn2)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn2)), ((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn4)), ((((locals.var_beta_dn5 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn5)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn5)), ((((locals.var_beta_dn6 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn6)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn6)), ((((locals.var_beta_dn7 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn7)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn7)), ((((locals.var_beta_dn8 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn8)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn8)), ((((locals.var_beta_dn9 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn9)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn9)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn10)), ((((locals.var_beta_dn13 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn13)) * locals.var_t10) + (assign67890_e104591 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign67890_e104595;
        locals.var_t1_dn0 = assign67890_e104595_d_n0;
        locals.var_t1_dn2 = assign67890_e104595_d_n2;
        locals.var_t1_dn4 = assign67890_e104595_d_n4;
        locals.var_t1_dn5 = assign67890_e104595_d_n5;
        locals.var_t1_dn6 = assign67890_e104595_d_n6;
        locals.var_t1_dn7 = assign67890_e104595_d_n7;
        locals.var_t1_dn8 = assign67890_e104595_d_n8;
        locals.var_t1_dn9 = assign67890_e104595_d_n9;
        locals.var_t1_dn10 = assign67890_e104595_d_n10;
        locals.var_t1_dn13 = assign67890_e104595_d_n13;

        let (assign67900_e104601, assign67900_e104601_d_n0, assign67900_e104601_d_n2, assign67900_e104601_d_n4, assign67900_e104601_d_n5, assign67900_e104601_d_n6, assign67900_e104601_d_n7, assign67900_e104601_d_n8, assign67900_e104601_d_n9, assign67900_e104601_d_n10, assign67900_e104601_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67900_e104599: f64 = (locals.var_t10 * locals.var_t10);
        (assign67900_e104599, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn13 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign67900_e104601;
        locals.var_t11_dn0 = assign67900_e104601_d_n0;
        locals.var_t11_dn2 = assign67900_e104601_d_n2;
        locals.var_t11_dn4 = assign67900_e104601_d_n4;
        locals.var_t11_dn5 = assign67900_e104601_d_n5;
        locals.var_t11_dn6 = assign67900_e104601_d_n6;
        locals.var_t11_dn7 = assign67900_e104601_d_n7;
        locals.var_t11_dn8 = assign67900_e104601_d_n8;
        locals.var_t11_dn9 = assign67900_e104601_d_n9;
        locals.var_t11_dn10 = assign67900_e104601_d_n10;
        locals.var_t11_dn13 = assign67900_e104601_d_n13;

        let (assign67910_e104607, assign67910_e104607_d_n0, assign67910_e104607_d_n2, assign67910_e104607_d_n4, assign67910_e104607_d_n5, assign67910_e104607_d_n6, assign67910_e104607_d_n7, assign67910_e104607_d_n8, assign67910_e104607_d_n9, assign67910_e104607_d_n10, assign67910_e104607_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67910_e104605: f64 = (1.0 / locals.var_xil);
        (assign67910_e104605, (-(locals.var_xil_dn0 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn2 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn4 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn5 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn6 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn7 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn8 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn9 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn10 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn13 / (locals.var_xil * locals.var_xil))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign67910_e104607;
        locals.var_t10_dn0 = assign67910_e104607_d_n0;
        locals.var_t10_dn2 = assign67910_e104607_d_n2;
        locals.var_t10_dn4 = assign67910_e104607_d_n4;
        locals.var_t10_dn5 = assign67910_e104607_d_n5;
        locals.var_t10_dn6 = assign67910_e104607_d_n6;
        locals.var_t10_dn7 = assign67910_e104607_d_n7;
        locals.var_t10_dn8 = assign67910_e104607_d_n8;
        locals.var_t10_dn9 = assign67910_e104607_d_n9;
        locals.var_t10_dn10 = assign67910_e104607_d_n10;
        locals.var_t10_dn13 = assign67910_e104607_d_n13;

        let (assign67920_e104615, assign67920_e104615_d_n0, assign67920_e104615_d_n2, assign67920_e104615_d_n4, assign67920_e104615_d_n5, assign67920_e104615_d_n6, assign67920_e104615_d_n7, assign67920_e104615_d_n8, assign67920_e104615_d_n9, assign67920_e104615_d_n10, assign67920_e104615_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67920_e104611: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign67920_e104613: f64 = (assign67920_e104611 * locals.var_t10);
        (assign67920_e104613, ((((locals.var_beta_dn0 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn0)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn0)), ((((locals.var_beta_dn2 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn2)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn2)), ((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn4)), ((((locals.var_beta_dn5 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn5)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn5)), ((((locals.var_beta_dn6 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn6)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn6)), ((((locals.var_beta_dn7 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn7)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn7)), ((((locals.var_beta_dn8 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn8)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn8)), ((((locals.var_beta_dn9 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn9)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn9)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn10)), ((((locals.var_beta_dn13 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn13)) * locals.var_t10) + (assign67920_e104611 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign67920_e104615;
        locals.var_t2_dn0 = assign67920_e104615_d_n0;
        locals.var_t2_dn2 = assign67920_e104615_d_n2;
        locals.var_t2_dn4 = assign67920_e104615_d_n4;
        locals.var_t2_dn5 = assign67920_e104615_d_n5;
        locals.var_t2_dn6 = assign67920_e104615_d_n6;
        locals.var_t2_dn7 = assign67920_e104615_d_n7;
        locals.var_t2_dn8 = assign67920_e104615_d_n8;
        locals.var_t2_dn9 = assign67920_e104615_d_n9;
        locals.var_t2_dn10 = assign67920_e104615_d_n10;
        locals.var_t2_dn13 = assign67920_e104615_d_n13;

        let (assign67930_e104621, assign67930_e104621_d_n0, assign67930_e104621_d_n2, assign67930_e104621_d_n4, assign67930_e104621_d_n5, assign67930_e104621_d_n6, assign67930_e104621_d_n7, assign67930_e104621_d_n8, assign67930_e104621_d_n9, assign67930_e104621_d_n10, assign67930_e104621_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67930_e104619: f64 = (locals.var_t10 * locals.var_t10);
        (assign67930_e104619, ((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)), ((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)), ((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)), ((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)), ((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)), ((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)), ((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)), ((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)), ((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)), ((locals.var_t10_dn13 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign67930_e104621;
        locals.var_t11_dn0 = assign67930_e104621_d_n0;
        locals.var_t11_dn2 = assign67930_e104621_d_n2;
        locals.var_t11_dn4 = assign67930_e104621_d_n4;
        locals.var_t11_dn5 = assign67930_e104621_d_n5;
        locals.var_t11_dn6 = assign67930_e104621_d_n6;
        locals.var_t11_dn7 = assign67930_e104621_d_n7;
        locals.var_t11_dn8 = assign67930_e104621_d_n8;
        locals.var_t11_dn9 = assign67930_e104621_d_n9;
        locals.var_t11_dn10 = assign67930_e104621_d_n10;
        locals.var_t11_dn13 = assign67930_e104621_d_n13;

        let (assign67940_e104633, assign67940_e104633_d_n0, assign67940_e104633_d_n2, assign67940_e104633_d_n4, assign67940_e104633_d_n5, assign67940_e104633_d_n6, assign67940_e104633_d_n7, assign67940_e104633_d_n8, assign67940_e104633_d_n9, assign67940_e104633_d_n10, assign67940_e104633_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67940_e104626: f64 = (locals.var_xilp32 * locals.var_t2);
        let assign67940_e104629: f64 = (locals.var_xi0p32 * locals.var_t1);
        let assign67940_e104630: f64 = (assign67940_e104626 - assign67940_e104629);
        let assign67940_e104631: f64 = (locals.var_cnst0 * assign67940_e104630);
        (assign67940_e104631, ((locals.var_cnst0_dn0 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn0 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn0)) - ((locals.var_xi0p32_dn0 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn0))))), ((locals.var_cnst0_dn2 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn2 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn2)) - ((locals.var_xi0p32_dn2 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn2))))), ((locals.var_cnst0_dn4 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn4 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn4)) - ((locals.var_xi0p32_dn4 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn4))))), ((locals.var_cnst0_dn5 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn5 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn5)) - ((locals.var_xi0p32_dn5 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn5))))), ((locals.var_cnst0_dn6 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn6 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn6)) - ((locals.var_xi0p32_dn6 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn6))))), ((locals.var_cnst0_dn7 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn7 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn7)) - ((locals.var_xi0p32_dn7 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn7))))), ((locals.var_cnst0_dn8 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn8 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn8)) - ((locals.var_xi0p32_dn8 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn8))))), ((locals.var_cnst0_dn9 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn9 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn9)) - ((locals.var_xi0p32_dn9 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn9))))), ((locals.var_cnst0_dn10 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn10 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn10)) - ((locals.var_xi0p32_dn10 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn10))))), ((locals.var_cnst0_dn13 * assign67940_e104630) + (locals.var_cnst0 * (((locals.var_xilp32_dn13 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn13)) - ((locals.var_xi0p32_dn13 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn13))))),)
    } else {
        (locals.var_dg3, locals.var_dg3_dn0, locals.var_dg3_dn2, locals.var_dg3_dn4, locals.var_dg3_dn5, locals.var_dg3_dn6, locals.var_dg3_dn7, locals.var_dg3_dn8, locals.var_dg3_dn9, locals.var_dg3_dn10, locals.var_dg3_dn13,)
    }
};
        locals.var_dg3 = assign67940_e104633;
        locals.var_dg3_dn0 = assign67940_e104633_d_n0;
        locals.var_dg3_dn2 = assign67940_e104633_d_n2;
        locals.var_dg3_dn4 = assign67940_e104633_d_n4;
        locals.var_dg3_dn5 = assign67940_e104633_d_n5;
        locals.var_dg3_dn6 = assign67940_e104633_d_n6;
        locals.var_dg3_dn7 = assign67940_e104633_d_n7;
        locals.var_dg3_dn8 = assign67940_e104633_d_n8;
        locals.var_dg3_dn9 = assign67940_e104633_d_n9;
        locals.var_dg3_dn10 = assign67940_e104633_d_n10;
        locals.var_dg3_dn13 = assign67940_e104633_d_n13;

    }

    pub(super) fn stamp_transient_block_232(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign67950_e104648, assign67950_e104648_d_n0, assign67950_e104648_d_n2, assign67950_e104648_d_n4, assign67950_e104648_d_n5, assign67950_e104648_d_n6, assign67950_e104648_d_n7, assign67950_e104648_d_n8, assign67950_e104648_d_n9, assign67950_e104648_d_n10, assign67950_e104648_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67950_e104637: f64 = (locals.var_cnst0 * 0.5);
        let assign67950_e104639: f64 = (-locals.var_xilp12);
        let assign67950_e104641: f64 = (assign67950_e104639 * locals.var_t2);
        let assign67950_e104644: f64 = (locals.var_xi0p12 * locals.var_t1);
        let assign67950_e104645: f64 = (assign67950_e104641 + assign67950_e104644);
        let assign67950_e104646: f64 = (assign67950_e104637 * assign67950_e104645);
        (assign67950_e104646, (((locals.var_cnst0_dn0 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn0) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn0)) + ((locals.var_xi0p12_dn0 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn0))))), (((locals.var_cnst0_dn2 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn2) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn2)) + ((locals.var_xi0p12_dn2 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn2))))), (((locals.var_cnst0_dn4 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn4) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn4)) + ((locals.var_xi0p12_dn4 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn4))))), (((locals.var_cnst0_dn5 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn5) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn5)) + ((locals.var_xi0p12_dn5 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn5))))), (((locals.var_cnst0_dn6 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn6) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn6)) + ((locals.var_xi0p12_dn6 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn6))))), (((locals.var_cnst0_dn7 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn7) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn7)) + ((locals.var_xi0p12_dn7 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn7))))), (((locals.var_cnst0_dn8 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn8) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn8)) + ((locals.var_xi0p12_dn8 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn8))))), (((locals.var_cnst0_dn9 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn9) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn9)) + ((locals.var_xi0p12_dn9 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn9))))), (((locals.var_cnst0_dn10 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn10) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn10)) + ((locals.var_xi0p12_dn10 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn10))))), (((locals.var_cnst0_dn13 * 0.5) * assign67950_e104645) + (assign67950_e104637 * ((((-locals.var_xilp12_dn13) * locals.var_t2) + (assign67950_e104639 * locals.var_t2_dn13)) + ((locals.var_xi0p12_dn13 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn13))))),)
    } else {
        (locals.var_dg4, locals.var_dg4_dn0, locals.var_dg4_dn2, locals.var_dg4_dn4, locals.var_dg4_dn5, locals.var_dg4_dn6, locals.var_dg4_dn7, locals.var_dg4_dn8, locals.var_dg4_dn9, locals.var_dg4_dn10, locals.var_dg4_dn13,)
    }
};
        locals.var_dg4 = assign67950_e104648;
        locals.var_dg4_dn0 = assign67950_e104648_d_n0;
        locals.var_dg4_dn2 = assign67950_e104648_d_n2;
        locals.var_dg4_dn4 = assign67950_e104648_d_n4;
        locals.var_dg4_dn5 = assign67950_e104648_d_n5;
        locals.var_dg4_dn6 = assign67950_e104648_d_n6;
        locals.var_dg4_dn7 = assign67950_e104648_d_n7;
        locals.var_dg4_dn8 = assign67950_e104648_d_n8;
        locals.var_dg4_dn9 = assign67950_e104648_d_n9;
        locals.var_dg4_dn10 = assign67950_e104648_d_n10;
        locals.var_dg4_dn13 = assign67950_e104648_d_n13;

        let (assign67960_e104654, assign67960_e104654_d_n0, assign67960_e104654_d_n2, assign67960_e104654_d_n4, assign67960_e104654_d_n5, assign67960_e104654_d_n6, assign67960_e104654_d_n7, assign67960_e104654_d_n8, assign67960_e104654_d_n9, assign67960_e104654_d_n10, assign67960_e104654_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67960_e104652: f64 = (locals.var_dg3 + locals.var_dg4);
        (assign67960_e104652, (locals.var_dg3_dn0 + locals.var_dg4_dn0), (locals.var_dg3_dn2 + locals.var_dg4_dn2), (locals.var_dg3_dn4 + locals.var_dg4_dn4), (locals.var_dg3_dn5 + locals.var_dg4_dn5), (locals.var_dg3_dn6 + locals.var_dg4_dn6), (locals.var_dg3_dn7 + locals.var_dg4_dn7), (locals.var_dg3_dn8 + locals.var_dg4_dn8), (locals.var_dg3_dn9 + locals.var_dg4_dn9), (locals.var_dg3_dn10 + locals.var_dg4_dn10), (locals.var_dg3_dn13 + locals.var_dg4_dn13),)
    } else {
        (locals.var_didd, locals.var_didd_dn0, locals.var_didd_dn2, locals.var_didd_dn4, locals.var_didd_dn5, locals.var_didd_dn6, locals.var_didd_dn7, locals.var_didd_dn8, locals.var_didd_dn9, locals.var_didd_dn10, locals.var_didd_dn13,)
    }
};
        locals.var_didd = assign67960_e104654;
        locals.var_didd_dn0 = assign67960_e104654_d_n0;
        locals.var_didd_dn2 = assign67960_e104654_d_n2;
        locals.var_didd_dn4 = assign67960_e104654_d_n4;
        locals.var_didd_dn5 = assign67960_e104654_d_n5;
        locals.var_didd_dn6 = assign67960_e104654_d_n6;
        locals.var_didd_dn7 = assign67960_e104654_d_n7;
        locals.var_didd_dn8 = assign67960_e104654_d_n8;
        locals.var_didd_dn9 = assign67960_e104654_d_n9;
        locals.var_didd_dn10 = assign67960_e104654_d_n10;
        locals.var_didd_dn13 = assign67960_e104654_d_n13;

        let (assign67970_e104662, assign67970_e104662_d_n0, assign67970_e104662_d_n2, assign67970_e104662_d_n4, assign67970_e104662_d_n5, assign67970_e104662_d_n6, assign67970_e104662_d_n7, assign67970_e104662_d_n8, assign67970_e104662_d_n9, assign67970_e104662_d_n10, assign67970_e104662_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67970_e104658: f64 = (locals.var_betawl * locals.var_didd);
        let assign67970_e104660: f64 = (assign67970_e104658 * locals.var_mu);
        (assign67970_e104660, ((((locals.var_betawl_dn0 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn0)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn2)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn4)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn5)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn6)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn7)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn8)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn9)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn10)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn10)), ((((locals.var_betawl_dn13 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn13)) * locals.var_mu) + (assign67970_e104658 * locals.var_mu_dn13)),)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn13,)
    }
};
        locals.var_idsibpc = assign67970_e104662;
        locals.var_idsibpc_dn0 = assign67970_e104662_d_n0;
        locals.var_idsibpc_dn2 = assign67970_e104662_d_n2;
        locals.var_idsibpc_dn4 = assign67970_e104662_d_n4;
        locals.var_idsibpc_dn5 = assign67970_e104662_d_n5;
        locals.var_idsibpc_dn6 = assign67970_e104662_d_n6;
        locals.var_idsibpc_dn7 = assign67970_e104662_d_n7;
        locals.var_idsibpc_dn8 = assign67970_e104662_d_n8;
        locals.var_idsibpc_dn9 = assign67970_e104662_d_n9;
        locals.var_idsibpc_dn10 = assign67970_e104662_d_n10;
        locals.var_idsibpc_dn13 = assign67970_e104662_d_n13;

        let (assign67980_e104668, assign67980_e104668_d_n0, assign67980_e104668_d_n2, assign67980_e104668_d_n4, assign67980_e104668_d_n5, assign67980_e104668_d_n6, assign67980_e104668_d_n7, assign67980_e104668_d_n8, assign67980_e104668_d_n9, assign67980_e104668_d_n10, assign67980_e104668_d_n13,) = {
    if (locals.var_guard1602 != 0.0) {
        let assign67980_e104666: f64 = (locals.var_wk_ii * locals.var_idsibpc);
        (assign67980_e104666, ((locals.var_wk_ii_dn0 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn0)), ((locals.var_wk_ii_dn2 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn2)), ((locals.var_wk_ii_dn4 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn4)), ((locals.var_wk_ii_dn5 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn5)), ((locals.var_wk_ii_dn6 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn6)), ((locals.var_wk_ii_dn7 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn7)), ((locals.var_wk_ii_dn8 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn8)), ((locals.var_wk_ii_dn9 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn9)), ((locals.var_wk_ii_dn10 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn10)), ((locals.var_wk_ii_dn13 * locals.var_idsibpc) + (locals.var_wk_ii * locals.var_idsibpc_dn13)),)
    } else {
        (locals.var_isubibpc, locals.var_isubibpc_dn0, locals.var_isubibpc_dn2, locals.var_isubibpc_dn4, locals.var_isubibpc_dn5, locals.var_isubibpc_dn6, locals.var_isubibpc_dn7, locals.var_isubibpc_dn8, locals.var_isubibpc_dn9, locals.var_isubibpc_dn10, locals.var_isubibpc_dn13,)
    }
};
        locals.var_isubibpc = assign67980_e104668;
        locals.var_isubibpc_dn0 = assign67980_e104668_d_n0;
        locals.var_isubibpc_dn2 = assign67980_e104668_d_n2;
        locals.var_isubibpc_dn4 = assign67980_e104668_d_n4;
        locals.var_isubibpc_dn5 = assign67980_e104668_d_n5;
        locals.var_isubibpc_dn6 = assign67980_e104668_d_n6;
        locals.var_isubibpc_dn7 = assign67980_e104668_d_n7;
        locals.var_isubibpc_dn8 = assign67980_e104668_d_n8;
        locals.var_isubibpc_dn9 = assign67980_e104668_d_n9;
        locals.var_isubibpc_dn10 = assign67980_e104668_d_n10;
        locals.var_isubibpc_dn13 = assign67980_e104668_d_n13;

        let assign67990_e104671: f64 = if p.p24 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1603 = assign67990_e104671;

        let assign68000_e104674: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1604 = assign68000_e104674;

        let (assign68010_e104686, assign68010_e104686_d_n0, assign68010_e104686_d_n2, assign68010_e104686_d_n4, assign68010_e104686_d_n5, assign68010_e104686_d_n6, assign68010_e104686_d_n7, assign68010_e104686_d_n8, assign68010_e104686_d_n9, assign68010_e104686_d_n10, assign68010_e104686_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68010_e104680: f64 = (locals.var_ps0z + locals.var_vdsz__blk439);
        let assign68010_e104683: f64 = (10.0 * 2.220446049250313e-16);
        let assign68010_e104684: f64 = (assign68010_e104680 - assign68010_e104683);
        (assign68010_e104684, (locals.var_ps0z_dn0 + locals.var_vdsz__blk439_dn0), (locals.var_ps0z_dn2 + locals.var_vdsz__blk439_dn2), (locals.var_ps0z_dn4 + locals.var_vdsz__blk439_dn4), (locals.var_ps0z_dn5 + locals.var_vdsz__blk439_dn5), (locals.var_ps0z_dn6 + locals.var_vdsz__blk439_dn6), (locals.var_ps0z_dn7 + locals.var_vdsz__blk439_dn7), (locals.var_ps0z_dn8 + locals.var_vdsz__blk439_dn8), (locals.var_ps0z_dn9 + locals.var_vdsz__blk439_dn9), (locals.var_ps0z_dn10 + locals.var_vdsz__blk439_dn10), (locals.var_ps0z_dn13 + locals.var_vdsz__blk439_dn13),)
    } else {
        (locals.var_psdlz, locals.var_psdlz_dn0, locals.var_psdlz_dn2, locals.var_psdlz_dn4, locals.var_psdlz_dn5, locals.var_psdlz_dn6, locals.var_psdlz_dn7, locals.var_psdlz_dn8, locals.var_psdlz_dn9, locals.var_psdlz_dn10, locals.var_psdlz_dn13,)
    }
};
        locals.var_psdlz = assign68010_e104686;
        locals.var_psdlz_dn0 = assign68010_e104686_d_n0;
        locals.var_psdlz_dn2 = assign68010_e104686_d_n2;
        locals.var_psdlz_dn4 = assign68010_e104686_d_n4;
        locals.var_psdlz_dn5 = assign68010_e104686_d_n5;
        locals.var_psdlz_dn6 = assign68010_e104686_d_n6;
        locals.var_psdlz_dn7 = assign68010_e104686_d_n7;
        locals.var_psdlz_dn8 = assign68010_e104686_d_n8;
        locals.var_psdlz_dn9 = assign68010_e104686_d_n9;
        locals.var_psdlz_dn10 = assign68010_e104686_d_n10;
        locals.var_psdlz_dn13 = assign68010_e104686_d_n13;

        let (assign68020_e104706, assign68020_e104706_d_n0, assign68020_e104706_d_n2, assign68020_e104706_d_n4, assign68020_e104706_d_n5, assign68020_e104706_d_n6, assign68020_e104706_d_n7, assign68020_e104706_d_n8, assign68020_e104706_d_n9, assign68020_e104706_d_n10, assign68020_e104706_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68020_e104692: f64 = (locals.var_vgsz__blk440 - locals.var_vfb);
        let assign68020_e104696: f64 = (locals.var_dvth - locals.var_dppg);
        let assign68020_e104697: f64 = (locals.var_mks_gleak4 * assign68020_e104696);
        let assign68020_e104699: f64 = (assign68020_e104697 * locals.var_leff);
        let assign68020_e104700: f64 = (assign68020_e104692 + assign68020_e104699);
        let assign68020_e104703: f64 = (locals.var_psdlz * locals.var_uc_gleak3);
        let assign68020_e104704: f64 = (assign68020_e104700 - assign68020_e104703);
        (assign68020_e104704, ((locals.var_vgsz__blk440_dn0 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn0 - locals.var_dppg_dn0)) * locals.var_leff)) - (locals.var_psdlz_dn0 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn2 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn2 - locals.var_dppg_dn2)) * locals.var_leff)) - (locals.var_psdlz_dn2 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn4 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn4 - locals.var_dppg_dn4)) * locals.var_leff)) - (locals.var_psdlz_dn4 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn5 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn5 - locals.var_dppg_dn5)) * locals.var_leff)) - (locals.var_psdlz_dn5 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn6 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn6 - locals.var_dppg_dn6)) * locals.var_leff)) - (locals.var_psdlz_dn6 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn7 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn7 - locals.var_dppg_dn7)) * locals.var_leff)) - (locals.var_psdlz_dn7 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn8 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn8 - locals.var_dppg_dn8)) * locals.var_leff)) - (locals.var_psdlz_dn8 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn9 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn9 - locals.var_dppg_dn9)) * locals.var_leff)) - (locals.var_psdlz_dn9 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn10 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn10 - locals.var_dppg_dn10)) * locals.var_leff)) - (locals.var_psdlz_dn10 * locals.var_uc_gleak3)), ((locals.var_vgsz__blk440_dn13 + ((locals.var_mks_gleak4 * (locals.var_dvth_dn13 - locals.var_dppg_dn13)) * locals.var_leff)) - (locals.var_psdlz_dn13 * locals.var_uc_gleak3)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68020_e104706;
        locals.var_t1_dn0 = assign68020_e104706_d_n0;
        locals.var_t1_dn2 = assign68020_e104706_d_n2;
        locals.var_t1_dn4 = assign68020_e104706_d_n4;
        locals.var_t1_dn5 = assign68020_e104706_d_n5;
        locals.var_t1_dn6 = assign68020_e104706_d_n6;
        locals.var_t1_dn7 = assign68020_e104706_d_n7;
        locals.var_t1_dn8 = assign68020_e104706_d_n8;
        locals.var_t1_dn9 = assign68020_e104706_d_n9;
        locals.var_t1_dn10 = assign68020_e104706_d_n10;
        locals.var_t1_dn13 = assign68020_e104706_d_n13;

        let (assign68030_e104714, assign68030_e104714_d_n0, assign68030_e104714_d_n2, assign68030_e104714_d_n4, assign68030_e104714_d_n5, assign68030_e104714_d_n6, assign68030_e104714_d_n7, assign68030_e104714_d_n8, assign68030_e104714_d_n9, assign68030_e104714_d_n10, assign68030_e104714_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68030_e104712: f64 = (locals.var_t1 * locals.var_t1);
        (assign68030_e104712, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68030_e104714;
        locals.var_t1_dn0 = assign68030_e104714_d_n0;
        locals.var_t1_dn2 = assign68030_e104714_d_n2;
        locals.var_t1_dn4 = assign68030_e104714_d_n4;
        locals.var_t1_dn5 = assign68030_e104714_d_n5;
        locals.var_t1_dn6 = assign68030_e104714_d_n6;
        locals.var_t1_dn7 = assign68030_e104714_d_n7;
        locals.var_t1_dn8 = assign68030_e104714_d_n8;
        locals.var_t1_dn9 = assign68030_e104714_d_n9;
        locals.var_t1_dn10 = assign68030_e104714_d_n10;
        locals.var_t1_dn13 = assign68030_e104714_d_n13;

        let (assign68040_e104722, assign68040_e104722_d_n0, assign68040_e104722_d_n2, assign68040_e104722_d_n4, assign68040_e104722_d_n5, assign68040_e104722_d_n6, assign68040_e104722_d_n7, assign68040_e104722_d_n8, assign68040_e104722_d_n9, assign68040_e104722_d_n10, assign68040_e104722_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68040_e104720: f64 = (1.0 / locals.var_tox0);
        (assign68040_e104720, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68040_e104722;
        locals.var_t3_dn0 = assign68040_e104722_d_n0;
        locals.var_t3_dn2 = assign68040_e104722_d_n2;
        locals.var_t3_dn4 = assign68040_e104722_d_n4;
        locals.var_t3_dn5 = assign68040_e104722_d_n5;
        locals.var_t3_dn6 = assign68040_e104722_d_n6;
        locals.var_t3_dn7 = assign68040_e104722_d_n7;
        locals.var_t3_dn8 = assign68040_e104722_d_n8;
        locals.var_t3_dn9 = assign68040_e104722_d_n9;
        locals.var_t3_dn10 = assign68040_e104722_d_n10;
        locals.var_t3_dn13 = assign68040_e104722_d_n13;

        let (assign68050_e104730, assign68050_e104730_d_n0, assign68050_e104730_d_n2, assign68050_e104730_d_n4, assign68050_e104730_d_n5, assign68050_e104730_d_n6, assign68050_e104730_d_n7, assign68050_e104730_d_n8, assign68050_e104730_d_n9, assign68050_e104730_d_n10, assign68050_e104730_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68050_e104728: f64 = (locals.var_t1 * locals.var_t3);
        (assign68050_e104728, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68050_e104730;
        locals.var_t2_dn0 = assign68050_e104730_d_n0;
        locals.var_t2_dn2 = assign68050_e104730_d_n2;
        locals.var_t2_dn4 = assign68050_e104730_d_n4;
        locals.var_t2_dn5 = assign68050_e104730_d_n5;
        locals.var_t2_dn6 = assign68050_e104730_d_n6;
        locals.var_t2_dn7 = assign68050_e104730_d_n7;
        locals.var_t2_dn8 = assign68050_e104730_d_n8;
        locals.var_t2_dn9 = assign68050_e104730_d_n9;
        locals.var_t2_dn10 = assign68050_e104730_d_n10;
        locals.var_t2_dn13 = assign68050_e104730_d_n13;

        let (assign68060_e104738, assign68060_e104738_d_n0, assign68060_e104738_d_n2, assign68060_e104738_d_n4, assign68060_e104738_d_n5, assign68060_e104738_d_n6, assign68060_e104738_d_n7, assign68060_e104738_d_n8, assign68060_e104738_d_n9, assign68060_e104738_d_n10, assign68060_e104738_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68060_e104736: f64 = (1.0 / locals.var_mks_gleak5);
        (assign68060_e104736, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68060_e104738;
        locals.var_t3_dn0 = assign68060_e104738_d_n0;
        locals.var_t3_dn2 = assign68060_e104738_d_n2;
        locals.var_t3_dn4 = assign68060_e104738_d_n4;
        locals.var_t3_dn5 = assign68060_e104738_d_n5;
        locals.var_t3_dn6 = assign68060_e104738_d_n6;
        locals.var_t3_dn7 = assign68060_e104738_d_n7;
        locals.var_t3_dn8 = assign68060_e104738_d_n8;
        locals.var_t3_dn9 = assign68060_e104738_d_n9;
        locals.var_t3_dn10 = assign68060_e104738_d_n10;
        locals.var_t3_dn13 = assign68060_e104738_d_n13;

        let (assign68070_e104748, assign68070_e104748_d_n0, assign68070_e104748_d_n2, assign68070_e104748_d_n4, assign68070_e104748_d_n5, assign68070_e104748_d_n6, assign68070_e104748_d_n7, assign68070_e104748_d_n8, assign68070_e104748_d_n9, assign68070_e104748_d_n10, assign68070_e104748_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68070_e104745: f64 = (locals.var_ey * locals.var_t3);
        let assign68070_e104746: f64 = (1.0 + assign68070_e104745);
        (assign68070_e104746, ((locals.var_ey_dn0 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn0)), ((locals.var_ey_dn2 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn2)), ((locals.var_ey_dn4 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn4)), ((locals.var_ey_dn5 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn5)), ((locals.var_ey_dn6 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn6)), ((locals.var_ey_dn7 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn7)), ((locals.var_ey_dn8 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn8)), ((locals.var_ey_dn9 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn9)), ((locals.var_ey_dn10 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn10)), ((locals.var_ey_dn13 * locals.var_t3) + (locals.var_ey * locals.var_t3_dn13)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign68070_e104748;
        locals.var_t7_dn0 = assign68070_e104748_d_n0;
        locals.var_t7_dn2 = assign68070_e104748_d_n2;
        locals.var_t7_dn4 = assign68070_e104748_d_n4;
        locals.var_t7_dn5 = assign68070_e104748_d_n5;
        locals.var_t7_dn6 = assign68070_e104748_d_n6;
        locals.var_t7_dn7 = assign68070_e104748_d_n7;
        locals.var_t7_dn8 = assign68070_e104748_d_n8;
        locals.var_t7_dn9 = assign68070_e104748_d_n9;
        locals.var_t7_dn10 = assign68070_e104748_d_n10;
        locals.var_t7_dn13 = assign68070_e104748_d_n13;

        let (assign68080_e104756, assign68080_e104756_d_n0, assign68080_e104756_d_n2, assign68080_e104756_d_n4, assign68080_e104756_d_n5, assign68080_e104756_d_n6, assign68080_e104756_d_n7, assign68080_e104756_d_n8, assign68080_e104756_d_n9, assign68080_e104756_d_n10, assign68080_e104756_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68080_e104754: f64 = (locals.var_t2 * locals.var_t7);
        (assign68080_e104754, ((locals.var_t2_dn0 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn0)), ((locals.var_t2_dn2 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn2)), ((locals.var_t2_dn4 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn4)), ((locals.var_t2_dn5 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn5)), ((locals.var_t2_dn6 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn6)), ((locals.var_t2_dn7 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn7)), ((locals.var_t2_dn8 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn8)), ((locals.var_t2_dn9 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn9)), ((locals.var_t2_dn10 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn10)), ((locals.var_t2_dn13 * locals.var_t7) + (locals.var_t2 * locals.var_t7_dn13)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    }
};
        locals.var_etun = assign68080_e104756;
        locals.var_etun_dn0 = assign68080_e104756_d_n0;
        locals.var_etun_dn2 = assign68080_e104756_d_n2;
        locals.var_etun_dn4 = assign68080_e104756_d_n4;
        locals.var_etun_dn5 = assign68080_e104756_d_n5;
        locals.var_etun_dn6 = assign68080_e104756_d_n6;
        locals.var_etun_dn7 = assign68080_e104756_d_n7;
        locals.var_etun_dn8 = assign68080_e104756_d_n8;
        locals.var_etun_dn9 = assign68080_e104756_d_n9;
        locals.var_etun_dn10 = assign68080_e104756_d_n10;
        locals.var_etun_dn13 = assign68080_e104756_d_n13;

        let (assign68090_e104775, assign68090_e104775_d_n0, assign68090_e104775_d_n2, assign68090_e104775_d_n4, assign68090_e104775_d_n5, assign68090_e104775_d_n6, assign68090_e104775_d_n7, assign68090_e104775_d_n8, assign68090_e104775_d_n9, assign68090_e104775_d_n10, assign68090_e104775_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68090_e104762: f64 = (locals.var_etun * locals.var_etun);
        let assign68090_e104766: f64 = (0.01 / 0.01);
        let assign68090_e104767: f64 = (4.0 * assign68090_e104766);
        let assign68090_e104770: f64 = (0.01 / 0.01);
        let assign68090_e104771: f64 = (assign68090_e104767 * assign68090_e104770);
        let assign68090_e104772: f64 = (assign68090_e104762 + assign68090_e104771);
        let assign68090_e104773: f64 = (assign68090_e104772).sqrt();
        (assign68090_e104773, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign68090_e104773)), (((locals.var_etun_dn13 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn13)) / (2.0 * assign68090_e104773)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign68090_e104775;
        locals.var_tmf2_dn0 = assign68090_e104775_d_n0;
        locals.var_tmf2_dn2 = assign68090_e104775_d_n2;
        locals.var_tmf2_dn4 = assign68090_e104775_d_n4;
        locals.var_tmf2_dn5 = assign68090_e104775_d_n5;
        locals.var_tmf2_dn6 = assign68090_e104775_d_n6;
        locals.var_tmf2_dn7 = assign68090_e104775_d_n7;
        locals.var_tmf2_dn8 = assign68090_e104775_d_n8;
        locals.var_tmf2_dn9 = assign68090_e104775_d_n9;
        locals.var_tmf2_dn10 = assign68090_e104775_d_n10;
        locals.var_tmf2_dn13 = assign68090_e104775_d_n13;

        let (assign68100_e104787, assign68100_e104787_d_n0, assign68100_e104787_d_n2, assign68100_e104787_d_n4, assign68100_e104787_d_n5, assign68100_e104787_d_n6, assign68100_e104787_d_n7, assign68100_e104787_d_n8, assign68100_e104787_d_n9, assign68100_e104787_d_n10, assign68100_e104787_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68100_e104783: f64 = (locals.var_etun / locals.var_tmf2);
        let assign68100_e104784: f64 = (1.0 + assign68100_e104783);
        let assign68100_e104785: f64 = (0.5 * assign68100_e104784);
        (assign68100_e104785, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn7 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn9 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn13 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68100_e104787;
        locals.var_t5_dn0 = assign68100_e104787_d_n0;
        locals.var_t5_dn2 = assign68100_e104787_d_n2;
        locals.var_t5_dn4 = assign68100_e104787_d_n4;
        locals.var_t5_dn5 = assign68100_e104787_d_n5;
        locals.var_t5_dn6 = assign68100_e104787_d_n6;
        locals.var_t5_dn7 = assign68100_e104787_d_n7;
        locals.var_t5_dn8 = assign68100_e104787_d_n8;
        locals.var_t5_dn9 = assign68100_e104787_d_n9;
        locals.var_t5_dn10 = assign68100_e104787_d_n10;
        locals.var_t5_dn13 = assign68100_e104787_d_n13;

        let (assign68110_e104797, assign68110_e104797_d_n0, assign68110_e104797_d_n2, assign68110_e104797_d_n4, assign68110_e104797_d_n5, assign68110_e104797_d_n6, assign68110_e104797_d_n7, assign68110_e104797_d_n8, assign68110_e104797_d_n9, assign68110_e104797_d_n10, assign68110_e104797_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68110_e104794: f64 = (locals.var_etun + locals.var_tmf2);
        let assign68110_e104795: f64 = (0.5 * assign68110_e104794);
        (assign68110_e104795, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    }
};
        locals.var_etun = assign68110_e104797;
        locals.var_etun_dn0 = assign68110_e104797_d_n0;
        locals.var_etun_dn2 = assign68110_e104797_d_n2;
        locals.var_etun_dn4 = assign68110_e104797_d_n4;
        locals.var_etun_dn5 = assign68110_e104797_d_n5;
        locals.var_etun_dn6 = assign68110_e104797_d_n6;
        locals.var_etun_dn7 = assign68110_e104797_d_n7;
        locals.var_etun_dn8 = assign68110_e104797_d_n8;
        locals.var_etun_dn9 = assign68110_e104797_d_n9;
        locals.var_etun_dn10 = assign68110_e104797_d_n10;
        locals.var_etun_dn13 = assign68110_e104797_d_n13;

        let assign68120_e104800: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1605 = assign68120_e104800;

        let (assign68130_e104808, assign68130_e104808_d_n0, assign68130_e104808_d_n2, assign68130_e104808_d_n4, assign68130_e104808_d_n5, assign68130_e104808_d_n6, assign68130_e104808_d_n7, assign68130_e104808_d_n8, assign68130_e104808_d_n9, assign68130_e104808_d_n10, assign68130_e104808_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1605 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    }
};
        locals.var_etun = assign68130_e104808;
        locals.var_etun_dn0 = assign68130_e104808_d_n0;
        locals.var_etun_dn2 = assign68130_e104808_d_n2;
        locals.var_etun_dn4 = assign68130_e104808_d_n4;
        locals.var_etun_dn5 = assign68130_e104808_d_n5;
        locals.var_etun_dn6 = assign68130_e104808_d_n6;
        locals.var_etun_dn7 = assign68130_e104808_d_n7;
        locals.var_etun_dn8 = assign68130_e104808_d_n8;
        locals.var_etun_dn9 = assign68130_e104808_d_n9;
        locals.var_etun_dn10 = assign68130_e104808_d_n10;
        locals.var_etun_dn13 = assign68130_e104808_d_n13;

        let (assign68140_e104816, assign68140_e104816_d_n0, assign68140_e104816_d_n2, assign68140_e104816_d_n4, assign68140_e104816_d_n5, assign68140_e104816_d_n6, assign68140_e104816_d_n7, assign68140_e104816_d_n8, assign68140_e104816_d_n9, assign68140_e104816_d_n10, assign68140_e104816_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1605 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68140_e104816;
        locals.var_t5_dn0 = assign68140_e104816_d_n0;
        locals.var_t5_dn2 = assign68140_e104816_d_n2;
        locals.var_t5_dn4 = assign68140_e104816_d_n4;
        locals.var_t5_dn5 = assign68140_e104816_d_n5;
        locals.var_t5_dn6 = assign68140_e104816_d_n6;
        locals.var_t5_dn7 = assign68140_e104816_d_n7;
        locals.var_t5_dn8 = assign68140_e104816_d_n8;
        locals.var_t5_dn9 = assign68140_e104816_d_n9;
        locals.var_t5_dn10 = assign68140_e104816_d_n10;
        locals.var_t5_dn13 = assign68140_e104816_d_n13;

        let (assign68150_e104831, assign68150_e104831_d_n0, assign68150_e104831_d_n2, assign68150_e104831_d_n4, assign68150_e104831_d_n5, assign68150_e104831_d_n6, assign68150_e104831_d_n7, assign68150_e104831_d_n8, assign68150_e104831_d_n9, assign68150_e104831_d_n10, assign68150_e104831_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68150_e104822: f64 = (locals.var_vgsz__blk440 * locals.var_vgsz__blk440);
        let assign68150_e104825: f64 = (4.0 * 0.001);
        let assign68150_e104827: f64 = (assign68150_e104825 * 0.001);
        let assign68150_e104828: f64 = (assign68150_e104822 + assign68150_e104827);
        let assign68150_e104829: f64 = (assign68150_e104828).sqrt();
        (assign68150_e104829, (((locals.var_vgsz__blk440_dn0 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn0)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn2 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn2)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn4 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn4)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn5 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn5)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn6 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn6)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn7 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn7)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn8 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn8)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn9 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn9)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn10 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn10)) / (2.0 * assign68150_e104829)), (((locals.var_vgsz__blk440_dn13 * locals.var_vgsz__blk440) + (locals.var_vgsz__blk440 * locals.var_vgsz__blk440_dn13)) / (2.0 * assign68150_e104829)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign68150_e104831;
        locals.var_tmf2_dn0 = assign68150_e104831_d_n0;
        locals.var_tmf2_dn2 = assign68150_e104831_d_n2;
        locals.var_tmf2_dn4 = assign68150_e104831_d_n4;
        locals.var_tmf2_dn5 = assign68150_e104831_d_n5;
        locals.var_tmf2_dn6 = assign68150_e104831_d_n6;
        locals.var_tmf2_dn7 = assign68150_e104831_d_n7;
        locals.var_tmf2_dn8 = assign68150_e104831_d_n8;
        locals.var_tmf2_dn9 = assign68150_e104831_d_n9;
        locals.var_tmf2_dn10 = assign68150_e104831_d_n10;
        locals.var_tmf2_dn13 = assign68150_e104831_d_n13;

        let (assign68160_e104843, assign68160_e104843_d_n0, assign68160_e104843_d_n2, assign68160_e104843_d_n4, assign68160_e104843_d_n5, assign68160_e104843_d_n6, assign68160_e104843_d_n7, assign68160_e104843_d_n8, assign68160_e104843_d_n9, assign68160_e104843_d_n10, assign68160_e104843_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68160_e104839: f64 = (locals.var_vgsz__blk440 / locals.var_tmf2);
        let assign68160_e104840: f64 = (1.0 + assign68160_e104839);
        let assign68160_e104841: f64 = (0.5 * assign68160_e104840);
        (assign68160_e104841, (0.5 * (((locals.var_vgsz__blk440_dn0 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn2 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn4 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn5 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn6 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn7 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn8 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn9 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn10 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz__blk440_dn13 * locals.var_tmf2) - (locals.var_vgsz__blk440 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign68160_e104843;
        locals.var_t4_dn0 = assign68160_e104843_d_n0;
        locals.var_t4_dn2 = assign68160_e104843_d_n2;
        locals.var_t4_dn4 = assign68160_e104843_d_n4;
        locals.var_t4_dn5 = assign68160_e104843_d_n5;
        locals.var_t4_dn6 = assign68160_e104843_d_n6;
        locals.var_t4_dn7 = assign68160_e104843_d_n7;
        locals.var_t4_dn8 = assign68160_e104843_d_n8;
        locals.var_t4_dn9 = assign68160_e104843_d_n9;
        locals.var_t4_dn10 = assign68160_e104843_d_n10;
        locals.var_t4_dn13 = assign68160_e104843_d_n13;

        let (assign68170_e104853, assign68170_e104853_d_n0, assign68170_e104853_d_n2, assign68170_e104853_d_n4, assign68170_e104853_d_n5, assign68170_e104853_d_n6, assign68170_e104853_d_n7, assign68170_e104853_d_n8, assign68170_e104853_d_n9, assign68170_e104853_d_n10, assign68170_e104853_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68170_e104850: f64 = (locals.var_vgsz__blk440 + locals.var_tmf2);
        let assign68170_e104851: f64 = (0.5 * assign68170_e104850);
        (assign68170_e104851, (0.5 * (locals.var_vgsz__blk440_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vgsz__blk440_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vgsz__blk440_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vgsz__blk440_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vgsz__blk440_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vgsz__blk440_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vgsz__blk440_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vgsz__blk440_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vgsz__blk440_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vgsz__blk440_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68170_e104853;
        locals.var_t3_dn0 = assign68170_e104853_d_n0;
        locals.var_t3_dn2 = assign68170_e104853_d_n2;
        locals.var_t3_dn4 = assign68170_e104853_d_n4;
        locals.var_t3_dn5 = assign68170_e104853_d_n5;
        locals.var_t3_dn6 = assign68170_e104853_d_n6;
        locals.var_t3_dn7 = assign68170_e104853_d_n7;
        locals.var_t3_dn8 = assign68170_e104853_d_n8;
        locals.var_t3_dn9 = assign68170_e104853_d_n9;
        locals.var_t3_dn10 = assign68170_e104853_d_n10;
        locals.var_t3_dn13 = assign68170_e104853_d_n13;

        let assign68180_e104856: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1606 = assign68180_e104856;

        let (assign68190_e104864, assign68190_e104864_d_n0, assign68190_e104864_d_n2, assign68190_e104864_d_n4, assign68190_e104864_d_n5, assign68190_e104864_d_n6, assign68190_e104864_d_n7, assign68190_e104864_d_n8, assign68190_e104864_d_n9, assign68190_e104864_d_n10, assign68190_e104864_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1606 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68190_e104864;
        locals.var_t3_dn0 = assign68190_e104864_d_n0;
        locals.var_t3_dn2 = assign68190_e104864_d_n2;
        locals.var_t3_dn4 = assign68190_e104864_d_n4;
        locals.var_t3_dn5 = assign68190_e104864_d_n5;
        locals.var_t3_dn6 = assign68190_e104864_d_n6;
        locals.var_t3_dn7 = assign68190_e104864_d_n7;
        locals.var_t3_dn8 = assign68190_e104864_d_n8;
        locals.var_t3_dn9 = assign68190_e104864_d_n9;
        locals.var_t3_dn10 = assign68190_e104864_d_n10;
        locals.var_t3_dn13 = assign68190_e104864_d_n13;

        let (assign68200_e104872, assign68200_e104872_d_n0, assign68200_e104872_d_n2, assign68200_e104872_d_n4, assign68200_e104872_d_n5, assign68200_e104872_d_n6, assign68200_e104872_d_n7, assign68200_e104872_d_n8, assign68200_e104872_d_n9, assign68200_e104872_d_n10, assign68200_e104872_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1606 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign68200_e104872;
        locals.var_t4_dn0 = assign68200_e104872_d_n0;
        locals.var_t4_dn2 = assign68200_e104872_d_n2;
        locals.var_t4_dn4 = assign68200_e104872_d_n4;
        locals.var_t4_dn5 = assign68200_e104872_d_n5;
        locals.var_t4_dn6 = assign68200_e104872_d_n6;
        locals.var_t4_dn7 = assign68200_e104872_d_n7;
        locals.var_t4_dn8 = assign68200_e104872_d_n8;
        locals.var_t4_dn9 = assign68200_e104872_d_n9;
        locals.var_t4_dn10 = assign68200_e104872_d_n10;
        locals.var_t4_dn13 = assign68200_e104872_d_n13;

        let (assign68210_e104880, assign68210_e104880_d_n0, assign68210_e104880_d_n2, assign68210_e104880_d_n4, assign68210_e104880_d_n5, assign68210_e104880_d_n6, assign68210_e104880_d_n7, assign68210_e104880_d_n8, assign68210_e104880_d_n9, assign68210_e104880_d_n10, assign68210_e104880_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68210_e104878: f64 = (locals.var_t3 - p.p262);
        (assign68210_e104878, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68210_e104880;
        locals.var_t3_dn0 = assign68210_e104880_d_n0;
        locals.var_t3_dn2 = assign68210_e104880_d_n2;
        locals.var_t3_dn4 = assign68210_e104880_d_n4;
        locals.var_t3_dn5 = assign68210_e104880_d_n5;
        locals.var_t3_dn6 = assign68210_e104880_d_n6;
        locals.var_t3_dn7 = assign68210_e104880_d_n7;
        locals.var_t3_dn8 = assign68210_e104880_d_n8;
        locals.var_t3_dn9 = assign68210_e104880_d_n9;
        locals.var_t3_dn10 = assign68210_e104880_d_n10;
        locals.var_t3_dn13 = assign68210_e104880_d_n13;

    }

    pub(super) fn stamp_transient_block_233(
        locals: &mut StampLocals,
    ) {
        let (assign68220_e104888, assign68220_e104888_d_n0, assign68220_e104888_d_n2, assign68220_e104888_d_n4, assign68220_e104888_d_n5, assign68220_e104888_d_n6, assign68220_e104888_d_n7, assign68220_e104888_d_n8, assign68220_e104888_d_n9, assign68220_e104888_d_n10, assign68220_e104888_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68220_e104886: f64 = (locals.var_t3 / 0.1);
        (assign68220_e104886, (locals.var_t3_dn0 / 0.1), (locals.var_t3_dn2 / 0.1), (locals.var_t3_dn4 / 0.1), (locals.var_t3_dn5 / 0.1), (locals.var_t3_dn6 / 0.1), (locals.var_t3_dn7 / 0.1), (locals.var_t3_dn8 / 0.1), (locals.var_t3_dn9 / 0.1), (locals.var_t3_dn10 / 0.1), (locals.var_t3_dn13 / 0.1),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign68220_e104888;
        locals.var_tx_dn0 = assign68220_e104888_d_n0;
        locals.var_tx_dn2 = assign68220_e104888_d_n2;
        locals.var_tx_dn4 = assign68220_e104888_d_n4;
        locals.var_tx_dn5 = assign68220_e104888_d_n5;
        locals.var_tx_dn6 = assign68220_e104888_d_n6;
        locals.var_tx_dn7 = assign68220_e104888_d_n7;
        locals.var_tx_dn8 = assign68220_e104888_d_n8;
        locals.var_tx_dn9 = assign68220_e104888_d_n9;
        locals.var_tx_dn10 = assign68220_e104888_d_n10;
        locals.var_tx_dn13 = assign68220_e104888_d_n13;

        let (assign68230_e104898, assign68230_e104898_d_n0, assign68230_e104898_d_n2, assign68230_e104898_d_n4, assign68230_e104898_d_n5, assign68230_e104898_d_n6, assign68230_e104898_d_n7, assign68230_e104898_d_n8, assign68230_e104898_d_n9, assign68230_e104898_d_n10, assign68230_e104898_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68230_e104895: f64 = (locals.var_tx * locals.var_tx);
        let assign68230_e104896: f64 = (1.0 + assign68230_e104895);
        (assign68230_e104896, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn13 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68230_e104898;
        locals.var_t2_dn0 = assign68230_e104898_d_n0;
        locals.var_t2_dn2 = assign68230_e104898_d_n2;
        locals.var_t2_dn4 = assign68230_e104898_d_n4;
        locals.var_t2_dn5 = assign68230_e104898_d_n5;
        locals.var_t2_dn6 = assign68230_e104898_d_n6;
        locals.var_t2_dn7 = assign68230_e104898_d_n7;
        locals.var_t2_dn8 = assign68230_e104898_d_n8;
        locals.var_t2_dn9 = assign68230_e104898_d_n9;
        locals.var_t2_dn10 = assign68230_e104898_d_n10;
        locals.var_t2_dn13 = assign68230_e104898_d_n13;

        let (assign68240_e104908, assign68240_e104908_d_n0, assign68240_e104908_d_n2, assign68240_e104908_d_n4, assign68240_e104908_d_n5, assign68240_e104908_d_n6, assign68240_e104908_d_n7, assign68240_e104908_d_n8, assign68240_e104908_d_n9, assign68240_e104908_d_n10, assign68240_e104908_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68240_e104905: f64 = (1.0 / locals.var_t2);
        let assign68240_e104906: f64 = (1.0 - assign68240_e104905);
        (assign68240_e104906, (-(-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2)))), (-(-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68240_e104908;
        locals.var_t1_dn0 = assign68240_e104908_d_n0;
        locals.var_t1_dn2 = assign68240_e104908_d_n2;
        locals.var_t1_dn4 = assign68240_e104908_d_n4;
        locals.var_t1_dn5 = assign68240_e104908_d_n5;
        locals.var_t1_dn6 = assign68240_e104908_d_n6;
        locals.var_t1_dn7 = assign68240_e104908_d_n7;
        locals.var_t1_dn8 = assign68240_e104908_d_n8;
        locals.var_t1_dn9 = assign68240_e104908_d_n9;
        locals.var_t1_dn10 = assign68240_e104908_d_n10;
        locals.var_t1_dn13 = assign68240_e104908_d_n13;

        let (assign68250_e104916, assign68250_e104916_d_n0, assign68250_e104916_d_n2, assign68250_e104916_d_n4, assign68250_e104916_d_n5, assign68250_e104916_d_n6, assign68250_e104916_d_n7, assign68250_e104916_d_n8, assign68250_e104916_d_n9, assign68250_e104916_d_n10, assign68250_e104916_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68250_e104914: f64 = (locals.var_etun * locals.var_t1);
        (assign68250_e104914, ((locals.var_etun_dn0 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn0)), ((locals.var_etun_dn2 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn2)), ((locals.var_etun_dn4 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn4)), ((locals.var_etun_dn5 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn5)), ((locals.var_etun_dn6 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn6)), ((locals.var_etun_dn7 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn7)), ((locals.var_etun_dn8 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn8)), ((locals.var_etun_dn9 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn9)), ((locals.var_etun_dn10 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn10)), ((locals.var_etun_dn13 * locals.var_t1) + (locals.var_etun * locals.var_t1_dn13)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    }
};
        locals.var_etun = assign68250_e104916;
        locals.var_etun_dn0 = assign68250_e104916_d_n0;
        locals.var_etun_dn2 = assign68250_e104916_d_n2;
        locals.var_etun_dn4 = assign68250_e104916_d_n4;
        locals.var_etun_dn5 = assign68250_e104916_d_n5;
        locals.var_etun_dn6 = assign68250_e104916_d_n6;
        locals.var_etun_dn7 = assign68250_e104916_d_n7;
        locals.var_etun_dn8 = assign68250_e104916_d_n8;
        locals.var_etun_dn9 = assign68250_e104916_d_n9;
        locals.var_etun_dn10 = assign68250_e104916_d_n10;
        locals.var_etun_dn13 = assign68250_e104916_d_n13;

        let (assign68260_e104924, assign68260_e104924_d_n0, assign68260_e104924_d_n2, assign68260_e104924_d_n4, assign68260_e104924_d_n5, assign68260_e104924_d_n6, assign68260_e104924_d_n7, assign68260_e104924_d_n8, assign68260_e104924_d_n9, assign68260_e104924_d_n10, assign68260_e104924_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68260_e104922: f64 = (locals.var_leff * locals.var_weff_nf);
        (assign68260_e104922, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign68260_e104924;
        locals.var_t0_dn0 = assign68260_e104924_d_n0;
        locals.var_t0_dn2 = assign68260_e104924_d_n2;
        locals.var_t0_dn4 = assign68260_e104924_d_n4;
        locals.var_t0_dn5 = assign68260_e104924_d_n5;
        locals.var_t0_dn6 = assign68260_e104924_d_n6;
        locals.var_t0_dn7 = assign68260_e104924_d_n7;
        locals.var_t0_dn8 = assign68260_e104924_d_n8;
        locals.var_t0_dn9 = assign68260_e104924_d_n9;
        locals.var_t0_dn10 = assign68260_e104924_d_n10;
        locals.var_t0_dn13 = assign68260_e104924_d_n13;

        let (assign68270_e104934, assign68270_e104934_d_n0, assign68270_e104934_d_n2, assign68270_e104934_d_n4, assign68270_e104934_d_n5, assign68270_e104934_d_n6, assign68270_e104934_d_n7, assign68270_e104934_d_n8, assign68270_e104934_d_n9, assign68270_e104934_d_n10, assign68270_e104934_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68270_e104931: f64 = (locals.var_mks_gleak7 + locals.var_t0);
        let assign68270_e104932: f64 = (locals.var_mks_gleak7 / assign68270_e104931);
        (assign68270_e104932, (-((locals.var_mks_gleak7 * locals.var_t0_dn0) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn2) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn4) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn5) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn6) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn7) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn8) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn9) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn10) / (assign68270_e104931 * assign68270_e104931))), (-((locals.var_mks_gleak7 * locals.var_t0_dn13) / (assign68270_e104931 * assign68270_e104931))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign68270_e104934;
        locals.var_t7_dn0 = assign68270_e104934_d_n0;
        locals.var_t7_dn2 = assign68270_e104934_d_n2;
        locals.var_t7_dn4 = assign68270_e104934_d_n4;
        locals.var_t7_dn5 = assign68270_e104934_d_n5;
        locals.var_t7_dn6 = assign68270_e104934_d_n6;
        locals.var_t7_dn7 = assign68270_e104934_d_n7;
        locals.var_t7_dn8 = assign68270_e104934_d_n8;
        locals.var_t7_dn9 = assign68270_e104934_d_n9;
        locals.var_t7_dn10 = assign68270_e104934_d_n10;
        locals.var_t7_dn13 = assign68270_e104934_d_n13;

        let (assign68280_e104940, assign68280_e104940_d_n0, assign68280_e104940_d_n2, assign68280_e104940_d_n4, assign68280_e104940_d_n5, assign68280_e104940_d_n6, assign68280_e104940_d_n7, assign68280_e104940_d_n8, assign68280_e104940_d_n9, assign68280_e104940_d_n10, assign68280_e104940_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        (locals.var_uc_gleak6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign68280_e104940;
        locals.var_t6_dn0 = assign68280_e104940_d_n0;
        locals.var_t6_dn2 = assign68280_e104940_d_n2;
        locals.var_t6_dn4 = assign68280_e104940_d_n4;
        locals.var_t6_dn5 = assign68280_e104940_d_n5;
        locals.var_t6_dn6 = assign68280_e104940_d_n6;
        locals.var_t6_dn7 = assign68280_e104940_d_n7;
        locals.var_t6_dn8 = assign68280_e104940_d_n8;
        locals.var_t6_dn9 = assign68280_e104940_d_n9;
        locals.var_t6_dn10 = assign68280_e104940_d_n10;
        locals.var_t6_dn13 = assign68280_e104940_d_n13;

        let (assign68290_e104950, assign68290_e104950_d_n0, assign68290_e104950_d_n2, assign68290_e104950_d_n4, assign68290_e104950_d_n5, assign68290_e104950_d_n6, assign68290_e104950_d_n7, assign68290_e104950_d_n8, assign68290_e104950_d_n9, assign68290_e104950_d_n10, assign68290_e104950_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68290_e104947: f64 = (locals.var_t6 + locals.var_vdsz__blk439);
        let assign68290_e104948: f64 = (locals.var_t6 / assign68290_e104947);
        (assign68290_e104948, (((locals.var_t6_dn0 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn0 + locals.var_vdsz__blk439_dn0))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn2 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn2 + locals.var_vdsz__blk439_dn2))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn4 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn4 + locals.var_vdsz__blk439_dn4))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn5 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn5 + locals.var_vdsz__blk439_dn5))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn6 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn6 + locals.var_vdsz__blk439_dn6))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn7 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn7 + locals.var_vdsz__blk439_dn7))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn8 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn8 + locals.var_vdsz__blk439_dn8))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn9 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn9 + locals.var_vdsz__blk439_dn9))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn10 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn10 + locals.var_vdsz__blk439_dn10))) / (assign68290_e104947 * assign68290_e104947)), (((locals.var_t6_dn13 * assign68290_e104947) - (locals.var_t6 * (locals.var_t6_dn13 + locals.var_vdsz__blk439_dn13))) / (assign68290_e104947 * assign68290_e104947)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign68290_e104950;
        locals.var_t9_dn0 = assign68290_e104950_d_n0;
        locals.var_t9_dn2 = assign68290_e104950_d_n2;
        locals.var_t9_dn4 = assign68290_e104950_d_n4;
        locals.var_t9_dn5 = assign68290_e104950_d_n5;
        locals.var_t9_dn6 = assign68290_e104950_d_n6;
        locals.var_t9_dn7 = assign68290_e104950_d_n7;
        locals.var_t9_dn8 = assign68290_e104950_d_n8;
        locals.var_t9_dn9 = assign68290_e104950_d_n9;
        locals.var_t9_dn10 = assign68290_e104950_d_n10;
        locals.var_t9_dn13 = assign68290_e104950_d_n13;

        let (assign68300_e104960, assign68300_e104960_d_n0, assign68300_e104960_d_n2, assign68300_e104960_d_n4, assign68300_e104960_d_n5, assign68300_e104960_d_n6, assign68300_e104960_d_n7, assign68300_e104960_d_n8, assign68300_e104960_d_n9, assign68300_e104960_d_n10, assign68300_e104960_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68300_e104957: f64 = (locals.var_etun + 1e-25);
        let assign68300_e104958: f64 = (1.0 / assign68300_e104957);
        (assign68300_e104958, (-(locals.var_etun_dn0 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn2 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn4 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn5 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn6 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn7 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn8 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn9 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn10 / (assign68300_e104957 * assign68300_e104957))), (-(locals.var_etun_dn13 / (assign68300_e104957 * assign68300_e104957))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign68300_e104960;
        locals.var_t4_dn0 = assign68300_e104960_d_n0;
        locals.var_t4_dn2 = assign68300_e104960_d_n2;
        locals.var_t4_dn4 = assign68300_e104960_d_n4;
        locals.var_t4_dn5 = assign68300_e104960_d_n5;
        locals.var_t4_dn6 = assign68300_e104960_d_n6;
        locals.var_t4_dn7 = assign68300_e104960_d_n7;
        locals.var_t4_dn8 = assign68300_e104960_d_n8;
        locals.var_t4_dn9 = assign68300_e104960_d_n9;
        locals.var_t4_dn10 = assign68300_e104960_d_n10;
        locals.var_t4_dn13 = assign68300_e104960_d_n13;

        let (assign68310_e104971, assign68310_e104971_d_n0, assign68310_e104971_d_n2, assign68310_e104971_d_n4, assign68310_e104971_d_n5, assign68310_e104971_d_n6, assign68310_e104971_d_n7, assign68310_e104971_d_n8, assign68310_e104971_d_n9, assign68310_e104971_d_n10, assign68310_e104971_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68310_e104965: f64 = (-locals.var_uc_gleak2);
        let assign68310_e104967: f64 = (assign68310_e104965 * locals.var_egp32);
        let assign68310_e104969: f64 = (assign68310_e104967 * locals.var_t4);
        (assign68310_e104969, (((assign68310_e104965 * locals.var_egp32_dn0) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn0)), (((assign68310_e104965 * locals.var_egp32_dn2) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn2)), (((assign68310_e104965 * locals.var_egp32_dn4) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn4)), (((assign68310_e104965 * locals.var_egp32_dn5) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn5)), (((assign68310_e104965 * locals.var_egp32_dn6) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn6)), (((assign68310_e104965 * locals.var_egp32_dn7) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn7)), (((assign68310_e104965 * locals.var_egp32_dn8) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn8)), (((assign68310_e104965 * locals.var_egp32_dn9) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn9)), (((assign68310_e104965 * locals.var_egp32_dn10) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn10)), (((assign68310_e104965 * locals.var_egp32_dn13) * locals.var_t4) + (assign68310_e104967 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68310_e104971;
        locals.var_t1_dn0 = assign68310_e104971_d_n0;
        locals.var_t1_dn2 = assign68310_e104971_d_n2;
        locals.var_t1_dn4 = assign68310_e104971_d_n4;
        locals.var_t1_dn5 = assign68310_e104971_d_n5;
        locals.var_t1_dn6 = assign68310_e104971_d_n6;
        locals.var_t1_dn7 = assign68310_e104971_d_n7;
        locals.var_t1_dn8 = assign68310_e104971_d_n8;
        locals.var_t1_dn9 = assign68310_e104971_d_n9;
        locals.var_t1_dn10 = assign68310_e104971_d_n10;
        locals.var_t1_dn13 = assign68310_e104971_d_n13;

        let (assign68320_e104981, assign68320_e104981_d_n0, assign68320_e104981_d_n2, assign68320_e104981_d_n4, assign68320_e104981_d_n5, assign68320_e104981_d_n6, assign68320_e104981_d_n7, assign68320_e104981_d_n8, assign68320_e104981_d_n9, assign68320_e104981_d_n10, assign68320_e104981_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) {
        let assign68320_e104977: f64 = (locals.var_uc_gleak2 * locals.var_t4);
        let assign68320_e104979: f64 = (assign68320_e104977 * locals.var_t4);
        (assign68320_e104979, (((locals.var_uc_gleak2 * locals.var_t4_dn0) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn0)), (((locals.var_uc_gleak2 * locals.var_t4_dn2) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn2)), (((locals.var_uc_gleak2 * locals.var_t4_dn4) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn4)), (((locals.var_uc_gleak2 * locals.var_t4_dn5) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn5)), (((locals.var_uc_gleak2 * locals.var_t4_dn6) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn6)), (((locals.var_uc_gleak2 * locals.var_t4_dn7) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn7)), (((locals.var_uc_gleak2 * locals.var_t4_dn8) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn8)), (((locals.var_uc_gleak2 * locals.var_t4_dn9) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn9)), (((locals.var_uc_gleak2 * locals.var_t4_dn10) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn10)), (((locals.var_uc_gleak2 * locals.var_t4_dn13) * locals.var_t4) + (assign68320_e104977 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68320_e104981;
        locals.var_t3_dn0 = assign68320_e104981_d_n0;
        locals.var_t3_dn2 = assign68320_e104981_d_n2;
        locals.var_t3_dn4 = assign68320_e104981_d_n4;
        locals.var_t3_dn5 = assign68320_e104981_d_n5;
        locals.var_t3_dn6 = assign68320_e104981_d_n6;
        locals.var_t3_dn7 = assign68320_e104981_d_n7;
        locals.var_t3_dn8 = assign68320_e104981_d_n8;
        locals.var_t3_dn9 = assign68320_e104981_d_n9;
        locals.var_t3_dn10 = assign68320_e104981_d_n10;
        locals.var_t3_dn13 = assign68320_e104981_d_n13;

        let assign68330_e104984: f64 = (-34.0);
        let assign68330_e104985: f64 = if locals.var_t1 < assign68330_e104984 { 1.0 } else { 0.0 };
        locals.var_guard1607 = assign68330_e104985;

        let (assign68340_e104993, assign68340_e104993_d_n0, assign68340_e104993_d_n2, assign68340_e104993_d_n4, assign68340_e104993_d_n5, assign68340_e104993_d_n6, assign68340_e104993_d_n7, assign68340_e104993_d_n8, assign68340_e104993_d_n9, assign68340_e104993_d_n10, assign68340_e104993_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn4, locals.var_igate_dn5, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn8, locals.var_igate_dn9, locals.var_igate_dn10, locals.var_igate_dn13,)
    }
};
        locals.var_igate = assign68340_e104993;
        locals.var_igate_dn0 = assign68340_e104993_d_n0;
        locals.var_igate_dn2 = assign68340_e104993_d_n2;
        locals.var_igate_dn4 = assign68340_e104993_d_n4;
        locals.var_igate_dn5 = assign68340_e104993_d_n5;
        locals.var_igate_dn6 = assign68340_e104993_d_n6;
        locals.var_igate_dn7 = assign68340_e104993_d_n7;
        locals.var_igate_dn8 = assign68340_e104993_d_n8;
        locals.var_igate_dn9 = assign68340_e104993_d_n9;
        locals.var_igate_dn10 = assign68340_e104993_d_n10;
        locals.var_igate_dn13 = assign68340_e104993_d_n13;

        let (assign68350_e105003, assign68350_e105003_d_n0, assign68350_e105003_d_n2, assign68350_e105003_d_n4, assign68350_e105003_d_n5, assign68350_e105003_d_n6, assign68350_e105003_d_n7, assign68350_e105003_d_n8, assign68350_e105003_d_n9, assign68350_e105003_d_n10, assign68350_e105003_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 == 0.0)) {
        let assign68350_e105001: f64 = (locals.var_t1).exp();
        (assign68350_e105001, (assign68350_e105001 * locals.var_t1_dn0), (assign68350_e105001 * locals.var_t1_dn2), (assign68350_e105001 * locals.var_t1_dn4), (assign68350_e105001 * locals.var_t1_dn5), (assign68350_e105001 * locals.var_t1_dn6), (assign68350_e105001 * locals.var_t1_dn7), (assign68350_e105001 * locals.var_t1_dn8), (assign68350_e105001 * locals.var_t1_dn9), (assign68350_e105001 * locals.var_t1_dn10), (assign68350_e105001 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68350_e105003;
        locals.var_t2_dn0 = assign68350_e105003_d_n0;
        locals.var_t2_dn2 = assign68350_e105003_d_n2;
        locals.var_t2_dn4 = assign68350_e105003_d_n4;
        locals.var_t2_dn5 = assign68350_e105003_d_n5;
        locals.var_t2_dn6 = assign68350_e105003_d_n6;
        locals.var_t2_dn7 = assign68350_e105003_d_n7;
        locals.var_t2_dn8 = assign68350_e105003_d_n8;
        locals.var_t2_dn9 = assign68350_e105003_d_n9;
        locals.var_t2_dn10 = assign68350_e105003_d_n10;
        locals.var_t2_dn13 = assign68350_e105003_d_n13;

        let (assign68360_e105018, assign68360_e105018_d_n0, assign68360_e105018_d_n2, assign68360_e105018_d_n4, assign68360_e105018_d_n5, assign68360_e105018_d_n6, assign68360_e105018_d_n7, assign68360_e105018_d_n8, assign68360_e105018_d_n9, assign68360_e105018_d_n10, assign68360_e105018_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 == 0.0)) {
        let assign68360_e105012: f64 = (locals.var_uc_gleak1 / locals.var_egp12);
        let assign68360_e105014: f64 = (assign68360_e105012 * 1.6021918e-19);
        let assign68360_e105016: f64 = (assign68360_e105014 * locals.var_t0);
        (assign68360_e105016, ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn0)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn2)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn4)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn5)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn6)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn7)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn8)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn9) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn9)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn10)), ((((-((locals.var_uc_gleak1 * locals.var_egp12_dn13) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0) + (assign68360_e105014 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68360_e105018;
        locals.var_t3_dn0 = assign68360_e105018_d_n0;
        locals.var_t3_dn2 = assign68360_e105018_d_n2;
        locals.var_t3_dn4 = assign68360_e105018_d_n4;
        locals.var_t3_dn5 = assign68360_e105018_d_n5;
        locals.var_t3_dn6 = assign68360_e105018_d_n6;
        locals.var_t3_dn7 = assign68360_e105018_d_n7;
        locals.var_t3_dn8 = assign68360_e105018_d_n8;
        locals.var_t3_dn9 = assign68360_e105018_d_n9;
        locals.var_t3_dn10 = assign68360_e105018_d_n10;
        locals.var_t3_dn13 = assign68360_e105018_d_n13;

        let (assign68370_e105029, assign68370_e105029_d_n0, assign68370_e105029_d_n2, assign68370_e105029_d_n4, assign68370_e105029_d_n5, assign68370_e105029_d_n6, assign68370_e105029_d_n7, assign68370_e105029_d_n8, assign68370_e105029_d_n9, assign68370_e105029_d_n10, assign68370_e105029_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 == 0.0)) {
        let assign68370_e105027: f64 = (1.0 / locals.var_cnst0);
        (assign68370_e105027, (-(locals.var_cnst0_dn0 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn2 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn4 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn5 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn6 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn7 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn8 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn9 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn10 / (locals.var_cnst0 * locals.var_cnst0))), (-(locals.var_cnst0_dn13 / (locals.var_cnst0 * locals.var_cnst0))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68370_e105029;
        locals.var_t5_dn0 = assign68370_e105029_d_n0;
        locals.var_t5_dn2 = assign68370_e105029_d_n2;
        locals.var_t5_dn4 = assign68370_e105029_d_n4;
        locals.var_t5_dn5 = assign68370_e105029_d_n5;
        locals.var_t5_dn6 = assign68370_e105029_d_n6;
        locals.var_t5_dn7 = assign68370_e105029_d_n7;
        locals.var_t5_dn8 = assign68370_e105029_d_n8;
        locals.var_t5_dn9 = assign68370_e105029_d_n9;
        locals.var_t5_dn10 = assign68370_e105029_d_n10;
        locals.var_t5_dn13 = assign68370_e105029_d_n13;

        let (assign68380_e105045, assign68380_e105045_d_n0, assign68380_e105045_d_n2, assign68380_e105045_d_n4, assign68380_e105045_d_n5, assign68380_e105045_d_n6, assign68380_e105045_d_n7, assign68380_e105045_d_n8, assign68380_e105045_d_n9, assign68380_e105045_d_n10, assign68380_e105045_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 == 0.0)) {
        let assign68380_e105039: f64 = (locals.var_cox0 * 1e-12);
        let assign68380_e105040: f64 = (locals.var_qiu_noi + assign68380_e105039);
        let assign68380_e105042: f64 = (assign68380_e105040 * locals.var_t5);
        let assign68380_e105043: f64 = (assign68380_e105042).sqrt();
        (assign68380_e105043, (((locals.var_qiu_noi_dn0 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn0)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn2 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn2)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn4 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn4)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn5 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn5)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn6 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn6)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn7 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn7)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn8 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn8)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn9 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn9)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn10 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn10)) / (2.0 * assign68380_e105043)), (((locals.var_qiu_noi_dn13 * locals.var_t5) + (assign68380_e105040 * locals.var_t5_dn13)) / (2.0 * assign68380_e105043)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign68380_e105045;
        locals.var_t6_dn0 = assign68380_e105045_d_n0;
        locals.var_t6_dn2 = assign68380_e105045_d_n2;
        locals.var_t6_dn4 = assign68380_e105045_d_n4;
        locals.var_t6_dn5 = assign68380_e105045_d_n5;
        locals.var_t6_dn6 = assign68380_e105045_d_n6;
        locals.var_t6_dn7 = assign68380_e105045_d_n7;
        locals.var_t6_dn8 = assign68380_e105045_d_n8;
        locals.var_t6_dn9 = assign68380_e105045_d_n9;
        locals.var_t6_dn10 = assign68380_e105045_d_n10;
        locals.var_t6_dn13 = assign68380_e105045_d_n13;

        let (assign68390_e105058, assign68390_e105058_d_n0, assign68390_e105058_d_n2, assign68390_e105058_d_n4, assign68390_e105058_d_n5, assign68390_e105058_d_n6, assign68390_e105058_d_n7, assign68390_e105058_d_n8, assign68390_e105058_d_n9, assign68390_e105058_d_n10, assign68390_e105058_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 == 0.0)) {
        let assign68390_e105054: f64 = (locals.var_t2 * locals.var_t3);
        let assign68390_e105056: f64 = (assign68390_e105054 * locals.var_t6);
        (assign68390_e105056, ((((locals.var_t2_dn0 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn0)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn0)), ((((locals.var_t2_dn2 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn2)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn2)), ((((locals.var_t2_dn4 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn4)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn4)), ((((locals.var_t2_dn5 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn5)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn5)), ((((locals.var_t2_dn6 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn6)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn6)), ((((locals.var_t2_dn7 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn7)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn7)), ((((locals.var_t2_dn8 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn8)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn8)), ((((locals.var_t2_dn9 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn9)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn9)), ((((locals.var_t2_dn10 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn10)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn10)), ((((locals.var_t2_dn13 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn13)) * locals.var_t6) + (assign68390_e105054 * locals.var_t6_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign68390_e105058;
        locals.var_t4_dn0 = assign68390_e105058_d_n0;
        locals.var_t4_dn2 = assign68390_e105058_d_n2;
        locals.var_t4_dn4 = assign68390_e105058_d_n4;
        locals.var_t4_dn5 = assign68390_e105058_d_n5;
        locals.var_t4_dn6 = assign68390_e105058_d_n6;
        locals.var_t4_dn7 = assign68390_e105058_d_n7;
        locals.var_t4_dn8 = assign68390_e105058_d_n8;
        locals.var_t4_dn9 = assign68390_e105058_d_n9;
        locals.var_t4_dn10 = assign68390_e105058_d_n10;
        locals.var_t4_dn13 = assign68390_e105058_d_n13;

        let (assign68400_e105069, assign68400_e105069_d_n0, assign68400_e105069_d_n2, assign68400_e105069_d_n4, assign68400_e105069_d_n5, assign68400_e105069_d_n6, assign68400_e105069_d_n7, assign68400_e105069_d_n8, assign68400_e105069_d_n9, assign68400_e105069_d_n10, assign68400_e105069_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 == 0.0)) {
        let assign68400_e105067: f64 = (locals.var_t4 * locals.var_etun);
        (assign68400_e105067, ((locals.var_t4_dn0 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn0)), ((locals.var_t4_dn2 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn2)), ((locals.var_t4_dn4 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn4)), ((locals.var_t4_dn5 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn5)), ((locals.var_t4_dn6 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn6)), ((locals.var_t4_dn7 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn7)), ((locals.var_t4_dn8 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn8)), ((locals.var_t4_dn9 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn9)), ((locals.var_t4_dn10 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn10)), ((locals.var_t4_dn13 * locals.var_etun) + (locals.var_t4 * locals.var_etun_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68400_e105069;
        locals.var_t5_dn0 = assign68400_e105069_d_n0;
        locals.var_t5_dn2 = assign68400_e105069_d_n2;
        locals.var_t5_dn4 = assign68400_e105069_d_n4;
        locals.var_t5_dn5 = assign68400_e105069_d_n5;
        locals.var_t5_dn6 = assign68400_e105069_d_n6;
        locals.var_t5_dn7 = assign68400_e105069_d_n7;
        locals.var_t5_dn8 = assign68400_e105069_d_n8;
        locals.var_t5_dn9 = assign68400_e105069_d_n9;
        locals.var_t5_dn10 = assign68400_e105069_d_n10;
        locals.var_t5_dn13 = assign68400_e105069_d_n13;

        let (assign68410_e105080, assign68410_e105080_d_n0, assign68410_e105080_d_n2, assign68410_e105080_d_n4, assign68410_e105080_d_n5, assign68410_e105080_d_n6, assign68410_e105080_d_n7, assign68410_e105080_d_n8, assign68410_e105080_d_n9, assign68410_e105080_d_n10, assign68410_e105080_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 == 0.0)) {
        let assign68410_e105078: f64 = (locals.var_t5 * locals.var_etun);
        (assign68410_e105078, ((locals.var_t5_dn0 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn0)), ((locals.var_t5_dn2 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn2)), ((locals.var_t5_dn4 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn4)), ((locals.var_t5_dn5 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn5)), ((locals.var_t5_dn6 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn6)), ((locals.var_t5_dn7 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn7)), ((locals.var_t5_dn8 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn8)), ((locals.var_t5_dn9 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn9)), ((locals.var_t5_dn10 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn10)), ((locals.var_t5_dn13 * locals.var_etun) + (locals.var_t5 * locals.var_etun_dn13)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign68410_e105080;
        locals.var_t10_dn0 = assign68410_e105080_d_n0;
        locals.var_t10_dn2 = assign68410_e105080_d_n2;
        locals.var_t10_dn4 = assign68410_e105080_d_n4;
        locals.var_t10_dn5 = assign68410_e105080_d_n5;
        locals.var_t10_dn6 = assign68410_e105080_d_n6;
        locals.var_t10_dn7 = assign68410_e105080_d_n7;
        locals.var_t10_dn8 = assign68410_e105080_d_n8;
        locals.var_t10_dn9 = assign68410_e105080_d_n9;
        locals.var_t10_dn10 = assign68410_e105080_d_n10;
        locals.var_t10_dn13 = assign68410_e105080_d_n13;

        let (assign68420_e105093, assign68420_e105093_d_n0, assign68420_e105093_d_n2, assign68420_e105093_d_n4, assign68420_e105093_d_n5, assign68420_e105093_d_n6, assign68420_e105093_d_n7, assign68420_e105093_d_n8, assign68420_e105093_d_n9, assign68420_e105093_d_n10, assign68420_e105093_d_n13,) = {
    if (((locals.var_guard1603 != 0.0) && (locals.var_guard1604 != 0.0)) && (locals.var_guard1607 == 0.0)) {
        let assign68420_e105089: f64 = (locals.var_t7 * locals.var_t9);
        let assign68420_e105091: f64 = (assign68420_e105089 * locals.var_t10);
        (assign68420_e105091, ((((locals.var_t7_dn0 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn0)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn0)), ((((locals.var_t7_dn2 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn2)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn2)), ((((locals.var_t7_dn4 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn4)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn4)), ((((locals.var_t7_dn5 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn5)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn5)), ((((locals.var_t7_dn6 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn6)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn6)), ((((locals.var_t7_dn7 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn7)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn7)), ((((locals.var_t7_dn8 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn8)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn8)), ((((locals.var_t7_dn9 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn9)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn9)), ((((locals.var_t7_dn10 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn10)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn10)), ((((locals.var_t7_dn13 * locals.var_t9) + (locals.var_t7 * locals.var_t9_dn13)) * locals.var_t10) + (assign68420_e105089 * locals.var_t10_dn13)),)
    } else {
        (locals.var_igate, locals.var_igate_dn0, locals.var_igate_dn2, locals.var_igate_dn4, locals.var_igate_dn5, locals.var_igate_dn6, locals.var_igate_dn7, locals.var_igate_dn8, locals.var_igate_dn9, locals.var_igate_dn10, locals.var_igate_dn13,)
    }
};
        locals.var_igate = assign68420_e105093;
        locals.var_igate_dn0 = assign68420_e105093_d_n0;
        locals.var_igate_dn2 = assign68420_e105093_d_n2;
        locals.var_igate_dn4 = assign68420_e105093_d_n4;
        locals.var_igate_dn5 = assign68420_e105093_d_n5;
        locals.var_igate_dn6 = assign68420_e105093_d_n6;
        locals.var_igate_dn7 = assign68420_e105093_d_n7;
        locals.var_igate_dn8 = assign68420_e105093_d_n8;
        locals.var_igate_dn9 = assign68420_e105093_d_n9;
        locals.var_igate_dn10 = assign68420_e105093_d_n10;
        locals.var_igate_dn13 = assign68420_e105093_d_n13;

        let (assign68430_e105102, assign68430_e105102_d_n0, assign68430_e105102_d_n2, assign68430_e105102_d_n4, assign68430_e105102_d_n5, assign68430_e105102_d_n6, assign68430_e105102_d_n7, assign68430_e105102_d_n8, assign68430_e105102_d_n9, assign68430_e105102_d_n10, assign68430_e105102_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68430_e105096: f64 = (-locals.var_uc_glksd2);
        let assign68430_e105098: f64 = (assign68430_e105096 * locals.var_vgs);
        let assign68430_e105100: f64 = (assign68430_e105098 + locals.var_mks_glksd3);
        (assign68430_e105100, 0.0, 0.0, 0.0, (assign68430_e105096 * locals.var_vgs_dn5), (assign68430_e105096 * locals.var_vgs_dn6), (assign68430_e105096 * locals.var_vgs_dn7), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign68430_e105102;
        locals.var_t0_dn0 = assign68430_e105102_d_n0;
        locals.var_t0_dn2 = assign68430_e105102_d_n2;
        locals.var_t0_dn4 = assign68430_e105102_d_n4;
        locals.var_t0_dn5 = assign68430_e105102_d_n5;
        locals.var_t0_dn6 = assign68430_e105102_d_n6;
        locals.var_t0_dn7 = assign68430_e105102_d_n7;
        locals.var_t0_dn8 = assign68430_e105102_d_n8;
        locals.var_t0_dn9 = assign68430_e105102_d_n9;
        locals.var_t0_dn10 = assign68430_e105102_d_n10;
        locals.var_t0_dn13 = assign68430_e105102_d_n13;

        let (assign68440_e105109, assign68440_e105109_d_n0, assign68440_e105109_d_n2, assign68440_e105109_d_n4, assign68440_e105109_d_n5, assign68440_e105109_d_n6, assign68440_e105109_d_n7, assign68440_e105109_d_n8, assign68440_e105109_d_n9, assign68440_e105109_d_n10, assign68440_e105109_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68440_e105106: f64 = (locals.var_tox0 * locals.var_t0);
        let assign68440_e105107: f64 = (assign68440_e105106).exp();
        (assign68440_e105107, (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn0)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn2)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn4)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn5)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn6)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn7)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn8)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn9)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn10)), (assign68440_e105107 * (locals.var_tox0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68440_e105109;
        locals.var_t2_dn0 = assign68440_e105109_d_n0;
        locals.var_t2_dn2 = assign68440_e105109_d_n2;
        locals.var_t2_dn4 = assign68440_e105109_d_n4;
        locals.var_t2_dn5 = assign68440_e105109_d_n5;
        locals.var_t2_dn6 = assign68440_e105109_d_n6;
        locals.var_t2_dn7 = assign68440_e105109_d_n7;
        locals.var_t2_dn8 = assign68440_e105109_d_n8;
        locals.var_t2_dn9 = assign68440_e105109_d_n9;
        locals.var_t2_dn10 = assign68440_e105109_d_n10;
        locals.var_t2_dn13 = assign68440_e105109_d_n13;

        let (assign68450_e105117, assign68450_e105117_d_n0, assign68450_e105117_d_n2, assign68450_e105117_d_n4, assign68450_e105117_d_n5, assign68450_e105117_d_n6, assign68450_e105117_d_n7, assign68450_e105117_d_n8, assign68450_e105117_d_n9, assign68450_e105117_d_n10, assign68450_e105117_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tox0;
        let assign68450_e105113: f64 = (locals.var_vgs * __rspice_inv_cse_0);
        let assign68450_e105115: f64 = (assign68450_e105113 * __rspice_inv_cse_0);
        (assign68450_e105115, 0.0, 0.0, 0.0, ((locals.var_vgs_dn5 / locals.var_tox0) / locals.var_tox0), ((locals.var_vgs_dn6 / locals.var_tox0) / locals.var_tox0), ((locals.var_vgs_dn7 / locals.var_tox0) / locals.var_tox0), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign68450_e105117;
        locals.var_t0_dn0 = assign68450_e105117_d_n0;
        locals.var_t0_dn2 = assign68450_e105117_d_n2;
        locals.var_t0_dn4 = assign68450_e105117_d_n4;
        locals.var_t0_dn5 = assign68450_e105117_d_n5;
        locals.var_t0_dn6 = assign68450_e105117_d_n6;
        locals.var_t0_dn7 = assign68450_e105117_d_n7;
        locals.var_t0_dn8 = assign68450_e105117_d_n8;
        locals.var_t0_dn9 = assign68450_e105117_d_n9;
        locals.var_t0_dn10 = assign68450_e105117_d_n10;
        locals.var_t0_dn13 = assign68450_e105117_d_n13;

        let (assign68460_e105123, assign68460_e105123_d_n0, assign68460_e105123_d_n2, assign68460_e105123_d_n4, assign68460_e105123_d_n5, assign68460_e105123_d_n6, assign68460_e105123_d_n7, assign68460_e105123_d_n8, assign68460_e105123_d_n9, assign68460_e105123_d_n10, assign68460_e105123_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68460_e105121: f64 = (locals.var_vgs * locals.var_t0);
        (assign68460_e105121, (locals.var_vgs * locals.var_t0_dn0), (locals.var_vgs * locals.var_t0_dn2), (locals.var_vgs * locals.var_t0_dn4), ((locals.var_vgs_dn5 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn5)), ((locals.var_vgs_dn6 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn6)), ((locals.var_vgs_dn7 * locals.var_t0) + (locals.var_vgs * locals.var_t0_dn7)), (locals.var_vgs * locals.var_t0_dn8), (locals.var_vgs * locals.var_t0_dn9), (locals.var_vgs * locals.var_t0_dn10), (locals.var_vgs * locals.var_t0_dn13),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68460_e105123;
        locals.var_t3_dn0 = assign68460_e105123_d_n0;
        locals.var_t3_dn2 = assign68460_e105123_d_n2;
        locals.var_t3_dn4 = assign68460_e105123_d_n4;
        locals.var_t3_dn5 = assign68460_e105123_d_n5;
        locals.var_t3_dn6 = assign68460_e105123_d_n6;
        locals.var_t3_dn7 = assign68460_e105123_d_n7;
        locals.var_t3_dn8 = assign68460_e105123_d_n8;
        locals.var_t3_dn9 = assign68460_e105123_d_n9;
        locals.var_t3_dn10 = assign68460_e105123_d_n10;
        locals.var_t3_dn13 = assign68460_e105123_d_n13;

    }

    pub(super) fn stamp_transient_block_234(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68470_e105131, assign68470_e105131_d_n0, assign68470_e105131_d_n2, assign68470_e105131_d_n4, assign68470_e105131_d_n5, assign68470_e105131_d_n6, assign68470_e105131_d_n7, assign68470_e105131_d_n8, assign68470_e105131_d_n9, assign68470_e105131_d_n10, assign68470_e105131_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68470_e105127: f64 = (locals.var_uc_glksd1 / 1000000.0);
        let assign68470_e105129: f64 = (assign68470_e105127 * locals.var_weff_nf);
        (assign68470_e105129, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign68470_e105131;
        locals.var_t4_dn0 = assign68470_e105131_d_n0;
        locals.var_t4_dn2 = assign68470_e105131_d_n2;
        locals.var_t4_dn4 = assign68470_e105131_d_n4;
        locals.var_t4_dn5 = assign68470_e105131_d_n5;
        locals.var_t4_dn6 = assign68470_e105131_d_n6;
        locals.var_t4_dn7 = assign68470_e105131_d_n7;
        locals.var_t4_dn8 = assign68470_e105131_d_n8;
        locals.var_t4_dn9 = assign68470_e105131_d_n9;
        locals.var_t4_dn10 = assign68470_e105131_d_n10;
        locals.var_t4_dn13 = assign68470_e105131_d_n13;

        let (assign68480_e105139, assign68480_e105139_d_n0, assign68480_e105139_d_n2, assign68480_e105139_d_n4, assign68480_e105139_d_n5, assign68480_e105139_d_n6, assign68480_e105139_d_n7, assign68480_e105139_d_n8, assign68480_e105139_d_n9, assign68480_e105139_d_n10, assign68480_e105139_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68480_e105135: f64 = (locals.var_t4 * locals.var_t2);
        let assign68480_e105137: f64 = (assign68480_e105135 * locals.var_t3);
        (assign68480_e105137, ((((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn0)), ((((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn2)), ((((locals.var_t4_dn4 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn4)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn4)), ((((locals.var_t4_dn5 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn5)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn5)), ((((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn6)), ((((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn7)), ((((locals.var_t4_dn8 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn8)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn8)), ((((locals.var_t4_dn9 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn9)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn9)), ((((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn10)), ((((locals.var_t4_dn13 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn13)) * locals.var_t3) + (assign68480_e105135 * locals.var_t3_dn13)),)
    } else {
        (locals.var_igs, locals.var_igs_dn0, locals.var_igs_dn2, locals.var_igs_dn4, locals.var_igs_dn5, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn8, locals.var_igs_dn9, locals.var_igs_dn10, locals.var_igs_dn13,)
    }
};
        locals.var_igs = assign68480_e105139;
        locals.var_igs_dn0 = assign68480_e105139_d_n0;
        locals.var_igs_dn2 = assign68480_e105139_d_n2;
        locals.var_igs_dn4 = assign68480_e105139_d_n4;
        locals.var_igs_dn5 = assign68480_e105139_d_n5;
        locals.var_igs_dn6 = assign68480_e105139_d_n6;
        locals.var_igs_dn7 = assign68480_e105139_d_n7;
        locals.var_igs_dn8 = assign68480_e105139_d_n8;
        locals.var_igs_dn9 = assign68480_e105139_d_n9;
        locals.var_igs_dn10 = assign68480_e105139_d_n10;
        locals.var_igs_dn13 = assign68480_e105139_d_n13;

        let assign68490_e105142: f64 = if locals.var_vgs >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1608 = assign68490_e105142;

        let (assign68500_e105151, assign68500_e105151_d_n0, assign68500_e105151_d_n2, assign68500_e105151_d_n4, assign68500_e105151_d_n5, assign68500_e105151_d_n6, assign68500_e105151_d_n7, assign68500_e105151_d_n8, assign68500_e105151_d_n9, assign68500_e105151_d_n10, assign68500_e105151_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1608 != 0.0)) {
        let assign68500_e105148: f64 = (-1.0);
        let assign68500_e105149: f64 = (locals.var_igs * assign68500_e105148);
        (assign68500_e105149, (locals.var_igs_dn0 * assign68500_e105148), (locals.var_igs_dn2 * assign68500_e105148), (locals.var_igs_dn4 * assign68500_e105148), (locals.var_igs_dn5 * assign68500_e105148), (locals.var_igs_dn6 * assign68500_e105148), (locals.var_igs_dn7 * assign68500_e105148), (locals.var_igs_dn8 * assign68500_e105148), (locals.var_igs_dn9 * assign68500_e105148), (locals.var_igs_dn10 * assign68500_e105148), (locals.var_igs_dn13 * assign68500_e105148),)
    } else {
        (locals.var_igs, locals.var_igs_dn0, locals.var_igs_dn2, locals.var_igs_dn4, locals.var_igs_dn5, locals.var_igs_dn6, locals.var_igs_dn7, locals.var_igs_dn8, locals.var_igs_dn9, locals.var_igs_dn10, locals.var_igs_dn13,)
    }
};
        locals.var_igs = assign68500_e105151;
        locals.var_igs_dn0 = assign68500_e105151_d_n0;
        locals.var_igs_dn2 = assign68500_e105151_d_n2;
        locals.var_igs_dn4 = assign68500_e105151_d_n4;
        locals.var_igs_dn5 = assign68500_e105151_d_n5;
        locals.var_igs_dn6 = assign68500_e105151_d_n6;
        locals.var_igs_dn7 = assign68500_e105151_d_n7;
        locals.var_igs_dn8 = assign68500_e105151_d_n8;
        locals.var_igs_dn9 = assign68500_e105151_d_n9;
        locals.var_igs_dn10 = assign68500_e105151_d_n10;
        locals.var_igs_dn13 = assign68500_e105151_d_n13;

        let (assign68510_e105157, assign68510_e105157_d_n0, assign68510_e105157_d_n2, assign68510_e105157_d_n4, assign68510_e105157_d_n5, assign68510_e105157_d_n6, assign68510_e105157_d_n7, assign68510_e105157_d_n8, assign68510_e105157_d_n9, assign68510_e105157_d_n10, assign68510_e105157_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68510_e105155: f64 = (locals.var_vgs - locals.var_vds);
        (assign68510_e105155, (-locals.var_vds_dn0), (-locals.var_vds_dn2), (-locals.var_vds_dn4), (locals.var_vgs_dn5 - locals.var_vds_dn5), (locals.var_vgs_dn6 - locals.var_vds_dn6), (locals.var_vgs_dn7 - locals.var_vds_dn7), (-locals.var_vds_dn8), (-locals.var_vds_dn9), (-locals.var_vds_dn10), (-locals.var_vds_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68510_e105157;
        locals.var_t1_dn0 = assign68510_e105157_d_n0;
        locals.var_t1_dn2 = assign68510_e105157_d_n2;
        locals.var_t1_dn4 = assign68510_e105157_d_n4;
        locals.var_t1_dn5 = assign68510_e105157_d_n5;
        locals.var_t1_dn6 = assign68510_e105157_d_n6;
        locals.var_t1_dn7 = assign68510_e105157_d_n7;
        locals.var_t1_dn8 = assign68510_e105157_d_n8;
        locals.var_t1_dn9 = assign68510_e105157_d_n9;
        locals.var_t1_dn10 = assign68510_e105157_d_n10;
        locals.var_t1_dn13 = assign68510_e105157_d_n13;

        let (assign68520_e105166, assign68520_e105166_d_n0, assign68520_e105166_d_n2, assign68520_e105166_d_n4, assign68520_e105166_d_n5, assign68520_e105166_d_n6, assign68520_e105166_d_n7, assign68520_e105166_d_n8, assign68520_e105166_d_n9, assign68520_e105166_d_n10, assign68520_e105166_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68520_e105160: f64 = (-locals.var_uc_glksd2);
        let assign68520_e105162: f64 = (assign68520_e105160 * locals.var_t1);
        let assign68520_e105164: f64 = (assign68520_e105162 + locals.var_mks_glksd3);
        (assign68520_e105164, (assign68520_e105160 * locals.var_t1_dn0), (assign68520_e105160 * locals.var_t1_dn2), (assign68520_e105160 * locals.var_t1_dn4), (assign68520_e105160 * locals.var_t1_dn5), (assign68520_e105160 * locals.var_t1_dn6), (assign68520_e105160 * locals.var_t1_dn7), (assign68520_e105160 * locals.var_t1_dn8), (assign68520_e105160 * locals.var_t1_dn9), (assign68520_e105160 * locals.var_t1_dn10), (assign68520_e105160 * locals.var_t1_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign68520_e105166;
        locals.var_t0_dn0 = assign68520_e105166_d_n0;
        locals.var_t0_dn2 = assign68520_e105166_d_n2;
        locals.var_t0_dn4 = assign68520_e105166_d_n4;
        locals.var_t0_dn5 = assign68520_e105166_d_n5;
        locals.var_t0_dn6 = assign68520_e105166_d_n6;
        locals.var_t0_dn7 = assign68520_e105166_d_n7;
        locals.var_t0_dn8 = assign68520_e105166_d_n8;
        locals.var_t0_dn9 = assign68520_e105166_d_n9;
        locals.var_t0_dn10 = assign68520_e105166_d_n10;
        locals.var_t0_dn13 = assign68520_e105166_d_n13;

        let (assign68530_e105173, assign68530_e105173_d_n0, assign68530_e105173_d_n2, assign68530_e105173_d_n4, assign68530_e105173_d_n5, assign68530_e105173_d_n6, assign68530_e105173_d_n7, assign68530_e105173_d_n8, assign68530_e105173_d_n9, assign68530_e105173_d_n10, assign68530_e105173_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68530_e105170: f64 = (locals.var_tox0 * locals.var_t0);
        let assign68530_e105171: f64 = (assign68530_e105170).exp();
        (assign68530_e105171, (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn0)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn2)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn4)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn5)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn6)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn7)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn8)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn9)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn10)), (assign68530_e105171 * (locals.var_tox0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68530_e105173;
        locals.var_t2_dn0 = assign68530_e105173_d_n0;
        locals.var_t2_dn2 = assign68530_e105173_d_n2;
        locals.var_t2_dn4 = assign68530_e105173_d_n4;
        locals.var_t2_dn5 = assign68530_e105173_d_n5;
        locals.var_t2_dn6 = assign68530_e105173_d_n6;
        locals.var_t2_dn7 = assign68530_e105173_d_n7;
        locals.var_t2_dn8 = assign68530_e105173_d_n8;
        locals.var_t2_dn9 = assign68530_e105173_d_n9;
        locals.var_t2_dn10 = assign68530_e105173_d_n10;
        locals.var_t2_dn13 = assign68530_e105173_d_n13;

        let (assign68540_e105181, assign68540_e105181_d_n0, assign68540_e105181_d_n2, assign68540_e105181_d_n4, assign68540_e105181_d_n5, assign68540_e105181_d_n6, assign68540_e105181_d_n7, assign68540_e105181_d_n8, assign68540_e105181_d_n9, assign68540_e105181_d_n10, assign68540_e105181_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tox0;
        let assign68540_e105177: f64 = (locals.var_t1 * __rspice_inv_cse_0);
        let assign68540_e105179: f64 = (assign68540_e105177 * __rspice_inv_cse_0);
        (assign68540_e105179, ((locals.var_t1_dn0 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn2 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn4 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn5 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn6 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn7 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn8 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn9 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn10 / locals.var_tox0) / locals.var_tox0), ((locals.var_t1_dn13 / locals.var_tox0) / locals.var_tox0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign68540_e105181;
        locals.var_t0_dn0 = assign68540_e105181_d_n0;
        locals.var_t0_dn2 = assign68540_e105181_d_n2;
        locals.var_t0_dn4 = assign68540_e105181_d_n4;
        locals.var_t0_dn5 = assign68540_e105181_d_n5;
        locals.var_t0_dn6 = assign68540_e105181_d_n6;
        locals.var_t0_dn7 = assign68540_e105181_d_n7;
        locals.var_t0_dn8 = assign68540_e105181_d_n8;
        locals.var_t0_dn9 = assign68540_e105181_d_n9;
        locals.var_t0_dn10 = assign68540_e105181_d_n10;
        locals.var_t0_dn13 = assign68540_e105181_d_n13;

        let (assign68550_e105187, assign68550_e105187_d_n0, assign68550_e105187_d_n2, assign68550_e105187_d_n4, assign68550_e105187_d_n5, assign68550_e105187_d_n6, assign68550_e105187_d_n7, assign68550_e105187_d_n8, assign68550_e105187_d_n9, assign68550_e105187_d_n10, assign68550_e105187_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68550_e105185: f64 = (locals.var_t1 * locals.var_t0);
        (assign68550_e105185, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn13 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68550_e105187;
        locals.var_t3_dn0 = assign68550_e105187_d_n0;
        locals.var_t3_dn2 = assign68550_e105187_d_n2;
        locals.var_t3_dn4 = assign68550_e105187_d_n4;
        locals.var_t3_dn5 = assign68550_e105187_d_n5;
        locals.var_t3_dn6 = assign68550_e105187_d_n6;
        locals.var_t3_dn7 = assign68550_e105187_d_n7;
        locals.var_t3_dn8 = assign68550_e105187_d_n8;
        locals.var_t3_dn9 = assign68550_e105187_d_n9;
        locals.var_t3_dn10 = assign68550_e105187_d_n10;
        locals.var_t3_dn13 = assign68550_e105187_d_n13;

        let (assign68560_e105195, assign68560_e105195_d_n0, assign68560_e105195_d_n2, assign68560_e105195_d_n4, assign68560_e105195_d_n5, assign68560_e105195_d_n6, assign68560_e105195_d_n7, assign68560_e105195_d_n8, assign68560_e105195_d_n9, assign68560_e105195_d_n10, assign68560_e105195_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68560_e105191: f64 = (locals.var_uc_glksd1 / 1000000.0);
        let assign68560_e105193: f64 = (assign68560_e105191 * locals.var_weff_nf);
        (assign68560_e105193, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign68560_e105195;
        locals.var_t4_dn0 = assign68560_e105195_d_n0;
        locals.var_t4_dn2 = assign68560_e105195_d_n2;
        locals.var_t4_dn4 = assign68560_e105195_d_n4;
        locals.var_t4_dn5 = assign68560_e105195_d_n5;
        locals.var_t4_dn6 = assign68560_e105195_d_n6;
        locals.var_t4_dn7 = assign68560_e105195_d_n7;
        locals.var_t4_dn8 = assign68560_e105195_d_n8;
        locals.var_t4_dn9 = assign68560_e105195_d_n9;
        locals.var_t4_dn10 = assign68560_e105195_d_n10;
        locals.var_t4_dn13 = assign68560_e105195_d_n13;

        let (assign68570_e105203, assign68570_e105203_d_n0, assign68570_e105203_d_n2, assign68570_e105203_d_n4, assign68570_e105203_d_n5, assign68570_e105203_d_n6, assign68570_e105203_d_n7, assign68570_e105203_d_n8, assign68570_e105203_d_n9, assign68570_e105203_d_n10, assign68570_e105203_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68570_e105199: f64 = (locals.var_t4 * locals.var_t2);
        let assign68570_e105201: f64 = (assign68570_e105199 * locals.var_t3);
        (assign68570_e105201, ((((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn0)), ((((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn2)), ((((locals.var_t4_dn4 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn4)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn4)), ((((locals.var_t4_dn5 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn5)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn5)), ((((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn6)), ((((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn7)), ((((locals.var_t4_dn8 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn8)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn8)), ((((locals.var_t4_dn9 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn9)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn9)), ((((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn10)), ((((locals.var_t4_dn13 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn13)) * locals.var_t3) + (assign68570_e105199 * locals.var_t3_dn13)),)
    } else {
        (locals.var_igd, locals.var_igd_dn0, locals.var_igd_dn2, locals.var_igd_dn4, locals.var_igd_dn5, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn8, locals.var_igd_dn9, locals.var_igd_dn10, locals.var_igd_dn13,)
    }
};
        locals.var_igd = assign68570_e105203;
        locals.var_igd_dn0 = assign68570_e105203_d_n0;
        locals.var_igd_dn2 = assign68570_e105203_d_n2;
        locals.var_igd_dn4 = assign68570_e105203_d_n4;
        locals.var_igd_dn5 = assign68570_e105203_d_n5;
        locals.var_igd_dn6 = assign68570_e105203_d_n6;
        locals.var_igd_dn7 = assign68570_e105203_d_n7;
        locals.var_igd_dn8 = assign68570_e105203_d_n8;
        locals.var_igd_dn9 = assign68570_e105203_d_n9;
        locals.var_igd_dn10 = assign68570_e105203_d_n10;
        locals.var_igd_dn13 = assign68570_e105203_d_n13;

        let assign68580_e105206: f64 = if locals.var_t1 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1609 = assign68580_e105206;

        let (assign68590_e105215, assign68590_e105215_d_n0, assign68590_e105215_d_n2, assign68590_e105215_d_n4, assign68590_e105215_d_n5, assign68590_e105215_d_n6, assign68590_e105215_d_n7, assign68590_e105215_d_n8, assign68590_e105215_d_n9, assign68590_e105215_d_n10, assign68590_e105215_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1609 != 0.0)) {
        let assign68590_e105212: f64 = (-1.0);
        let assign68590_e105213: f64 = (locals.var_igd * assign68590_e105212);
        (assign68590_e105213, (locals.var_igd_dn0 * assign68590_e105212), (locals.var_igd_dn2 * assign68590_e105212), (locals.var_igd_dn4 * assign68590_e105212), (locals.var_igd_dn5 * assign68590_e105212), (locals.var_igd_dn6 * assign68590_e105212), (locals.var_igd_dn7 * assign68590_e105212), (locals.var_igd_dn8 * assign68590_e105212), (locals.var_igd_dn9 * assign68590_e105212), (locals.var_igd_dn10 * assign68590_e105212), (locals.var_igd_dn13 * assign68590_e105212),)
    } else {
        (locals.var_igd, locals.var_igd_dn0, locals.var_igd_dn2, locals.var_igd_dn4, locals.var_igd_dn5, locals.var_igd_dn6, locals.var_igd_dn7, locals.var_igd_dn8, locals.var_igd_dn9, locals.var_igd_dn10, locals.var_igd_dn13,)
    }
};
        locals.var_igd = assign68590_e105215;
        locals.var_igd_dn0 = assign68590_e105215_d_n0;
        locals.var_igd_dn2 = assign68590_e105215_d_n2;
        locals.var_igd_dn4 = assign68590_e105215_d_n4;
        locals.var_igd_dn5 = assign68590_e105215_d_n5;
        locals.var_igd_dn6 = assign68590_e105215_d_n6;
        locals.var_igd_dn7 = assign68590_e105215_d_n7;
        locals.var_igd_dn8 = assign68590_e105215_d_n8;
        locals.var_igd_dn9 = assign68590_e105215_d_n9;
        locals.var_igd_dn10 = assign68590_e105215_d_n10;
        locals.var_igd_dn13 = assign68590_e105215_d_n13;

        let (assign68600_e105228, assign68600_e105228_d_n0, assign68600_e105228_d_n2, assign68600_e105228_d_n4, assign68600_e105228_d_n5, assign68600_e105228_d_n6, assign68600_e105228_d_n7, assign68600_e105228_d_n8, assign68600_e105228_d_n9, assign68600_e105228_d_n10, assign68600_e105228_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68600_e105219: f64 = (locals.var_vgs - locals.var_vbs);
        let assign68600_e105220: f64 = (-assign68600_e105219);
        let assign68600_e105222: f64 = (assign68600_e105220 + locals.var_vfb);
        let assign68600_e105224: f64 = (assign68600_e105222 + p.p258);
        let assign68600_e105226: f64 = (assign68600_e105224 / locals.var_tox0);
        (assign68600_e105226, 0.0, 0.0, 0.0, ((-(locals.var_vgs_dn5 - locals.var_vbs_dn5)) / locals.var_tox0), ((-locals.var_vgs_dn6) / locals.var_tox0), ((-(locals.var_vgs_dn7 - locals.var_vbs_dn7)) / locals.var_tox0), ((-(-locals.var_vbs_dn8)) / locals.var_tox0), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    }
};
        locals.var_etun = assign68600_e105228;
        locals.var_etun_dn0 = assign68600_e105228_d_n0;
        locals.var_etun_dn2 = assign68600_e105228_d_n2;
        locals.var_etun_dn4 = assign68600_e105228_d_n4;
        locals.var_etun_dn5 = assign68600_e105228_d_n5;
        locals.var_etun_dn6 = assign68600_e105228_d_n6;
        locals.var_etun_dn7 = assign68600_e105228_d_n7;
        locals.var_etun_dn8 = assign68600_e105228_d_n8;
        locals.var_etun_dn9 = assign68600_e105228_d_n9;
        locals.var_etun_dn10 = assign68600_e105228_d_n10;
        locals.var_etun_dn13 = assign68600_e105228_d_n13;

        let (assign68610_e105245, assign68610_e105245_d_n0, assign68610_e105245_d_n2, assign68610_e105245_d_n4, assign68610_e105245_d_n5, assign68610_e105245_d_n6, assign68610_e105245_d_n7, assign68610_e105245_d_n8, assign68610_e105245_d_n9, assign68610_e105245_d_n10, assign68610_e105245_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68610_e105232: f64 = (locals.var_etun * locals.var_etun);
        let assign68610_e105236: f64 = (0.01 / 0.01);
        let assign68610_e105237: f64 = (4.0 * assign68610_e105236);
        let assign68610_e105240: f64 = (0.01 / 0.01);
        let assign68610_e105241: f64 = (assign68610_e105237 * assign68610_e105240);
        let assign68610_e105242: f64 = (assign68610_e105232 + assign68610_e105241);
        let assign68610_e105243: f64 = (assign68610_e105242).sqrt();
        (assign68610_e105243, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign68610_e105243)), (((locals.var_etun_dn13 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn13)) / (2.0 * assign68610_e105243)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign68610_e105245;
        locals.var_tmf2_dn0 = assign68610_e105245_d_n0;
        locals.var_tmf2_dn2 = assign68610_e105245_d_n2;
        locals.var_tmf2_dn4 = assign68610_e105245_d_n4;
        locals.var_tmf2_dn5 = assign68610_e105245_d_n5;
        locals.var_tmf2_dn6 = assign68610_e105245_d_n6;
        locals.var_tmf2_dn7 = assign68610_e105245_d_n7;
        locals.var_tmf2_dn8 = assign68610_e105245_d_n8;
        locals.var_tmf2_dn9 = assign68610_e105245_d_n9;
        locals.var_tmf2_dn10 = assign68610_e105245_d_n10;
        locals.var_tmf2_dn13 = assign68610_e105245_d_n13;

        let (assign68620_e105255, assign68620_e105255_d_n0, assign68620_e105255_d_n2, assign68620_e105255_d_n4, assign68620_e105255_d_n5, assign68620_e105255_d_n6, assign68620_e105255_d_n7, assign68620_e105255_d_n8, assign68620_e105255_d_n9, assign68620_e105255_d_n10, assign68620_e105255_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68620_e105251: f64 = (locals.var_etun / locals.var_tmf2);
        let assign68620_e105252: f64 = (1.0 + assign68620_e105251);
        let assign68620_e105253: f64 = (0.5 * assign68620_e105252);
        (assign68620_e105253, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn7 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn9 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn13 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68620_e105255;
        locals.var_t5_dn0 = assign68620_e105255_d_n0;
        locals.var_t5_dn2 = assign68620_e105255_d_n2;
        locals.var_t5_dn4 = assign68620_e105255_d_n4;
        locals.var_t5_dn5 = assign68620_e105255_d_n5;
        locals.var_t5_dn6 = assign68620_e105255_d_n6;
        locals.var_t5_dn7 = assign68620_e105255_d_n7;
        locals.var_t5_dn8 = assign68620_e105255_d_n8;
        locals.var_t5_dn9 = assign68620_e105255_d_n9;
        locals.var_t5_dn10 = assign68620_e105255_d_n10;
        locals.var_t5_dn13 = assign68620_e105255_d_n13;

        let (assign68630_e105263, assign68630_e105263_d_n0, assign68630_e105263_d_n2, assign68630_e105263_d_n4, assign68630_e105263_d_n5, assign68630_e105263_d_n6, assign68630_e105263_d_n7, assign68630_e105263_d_n8, assign68630_e105263_d_n9, assign68630_e105263_d_n10, assign68630_e105263_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68630_e105260: f64 = (locals.var_etun + locals.var_tmf2);
        let assign68630_e105261: f64 = (0.5 * assign68630_e105260);
        (assign68630_e105261, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    }
};
        locals.var_etun = assign68630_e105263;
        locals.var_etun_dn0 = assign68630_e105263_d_n0;
        locals.var_etun_dn2 = assign68630_e105263_d_n2;
        locals.var_etun_dn4 = assign68630_e105263_d_n4;
        locals.var_etun_dn5 = assign68630_e105263_d_n5;
        locals.var_etun_dn6 = assign68630_e105263_d_n6;
        locals.var_etun_dn7 = assign68630_e105263_d_n7;
        locals.var_etun_dn8 = assign68630_e105263_d_n8;
        locals.var_etun_dn9 = assign68630_e105263_d_n9;
        locals.var_etun_dn10 = assign68630_e105263_d_n10;
        locals.var_etun_dn13 = assign68630_e105263_d_n13;

        let assign68640_e105266: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1610 = assign68640_e105266;

        let (assign68650_e105272, assign68650_e105272_d_n0, assign68650_e105272_d_n2, assign68650_e105272_d_n4, assign68650_e105272_d_n5, assign68650_e105272_d_n6, assign68650_e105272_d_n7, assign68650_e105272_d_n8, assign68650_e105272_d_n9, assign68650_e105272_d_n10, assign68650_e105272_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1610 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    }
};
        locals.var_etun = assign68650_e105272;
        locals.var_etun_dn0 = assign68650_e105272_d_n0;
        locals.var_etun_dn2 = assign68650_e105272_d_n2;
        locals.var_etun_dn4 = assign68650_e105272_d_n4;
        locals.var_etun_dn5 = assign68650_e105272_d_n5;
        locals.var_etun_dn6 = assign68650_e105272_d_n6;
        locals.var_etun_dn7 = assign68650_e105272_d_n7;
        locals.var_etun_dn8 = assign68650_e105272_d_n8;
        locals.var_etun_dn9 = assign68650_e105272_d_n9;
        locals.var_etun_dn10 = assign68650_e105272_d_n10;
        locals.var_etun_dn13 = assign68650_e105272_d_n13;

        let (assign68660_e105278, assign68660_e105278_d_n0, assign68660_e105278_d_n2, assign68660_e105278_d_n4, assign68660_e105278_d_n5, assign68660_e105278_d_n6, assign68660_e105278_d_n7, assign68660_e105278_d_n8, assign68660_e105278_d_n9, assign68660_e105278_d_n10, assign68660_e105278_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1610 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68660_e105278;
        locals.var_t5_dn0 = assign68660_e105278_d_n0;
        locals.var_t5_dn2 = assign68660_e105278_d_n2;
        locals.var_t5_dn4 = assign68660_e105278_d_n4;
        locals.var_t5_dn5 = assign68660_e105278_d_n5;
        locals.var_t5_dn6 = assign68660_e105278_d_n6;
        locals.var_t5_dn7 = assign68660_e105278_d_n7;
        locals.var_t5_dn8 = assign68660_e105278_d_n8;
        locals.var_t5_dn9 = assign68660_e105278_d_n9;
        locals.var_t5_dn10 = assign68660_e105278_d_n10;
        locals.var_t5_dn13 = assign68660_e105278_d_n13;

        let (assign68670_e105284, assign68670_e105284_d_n0, assign68670_e105284_d_n2, assign68670_e105284_d_n4, assign68670_e105284_d_n5, assign68670_e105284_d_n6, assign68670_e105284_d_n7, assign68670_e105284_d_n8, assign68670_e105284_d_n9, assign68670_e105284_d_n10, assign68670_e105284_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68670_e105282: f64 = (locals.var_etun + 1e-25);
        (assign68670_e105282, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn8, locals.var_etun_dn9, locals.var_etun_dn10, locals.var_etun_dn13,)
    }
};
        locals.var_etun = assign68670_e105284;
        locals.var_etun_dn0 = assign68670_e105284_d_n0;
        locals.var_etun_dn2 = assign68670_e105284_d_n2;
        locals.var_etun_dn4 = assign68670_e105284_d_n4;
        locals.var_etun_dn5 = assign68670_e105284_d_n5;
        locals.var_etun_dn6 = assign68670_e105284_d_n6;
        locals.var_etun_dn7 = assign68670_e105284_d_n7;
        locals.var_etun_dn8 = assign68670_e105284_d_n8;
        locals.var_etun_dn9 = assign68670_e105284_d_n9;
        locals.var_etun_dn10 = assign68670_e105284_d_n10;
        locals.var_etun_dn13 = assign68670_e105284_d_n13;

        let (assign68680_e105291, assign68680_e105291_d_n0, assign68680_e105291_d_n2, assign68680_e105291_d_n4, assign68680_e105291_d_n5, assign68680_e105291_d_n6, assign68680_e105291_d_n7, assign68680_e105291_d_n8, assign68680_e105291_d_n9, assign68680_e105291_d_n10, assign68680_e105291_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68680_e105287: f64 = (-locals.var_uc_glkb2);
        let assign68680_e105289: f64 = (assign68680_e105287 / locals.var_etun);
        (assign68680_e105289, (-((assign68680_e105287 * locals.var_etun_dn0) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn2) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn4) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn5) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn6) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn7) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn8) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn9) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn10) / (locals.var_etun * locals.var_etun))), (-((assign68680_e105287 * locals.var_etun_dn13) / (locals.var_etun * locals.var_etun))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68680_e105291;
        locals.var_t1_dn0 = assign68680_e105291_d_n0;
        locals.var_t1_dn2 = assign68680_e105291_d_n2;
        locals.var_t1_dn4 = assign68680_e105291_d_n4;
        locals.var_t1_dn5 = assign68680_e105291_d_n5;
        locals.var_t1_dn6 = assign68680_e105291_d_n6;
        locals.var_t1_dn7 = assign68680_e105291_d_n7;
        locals.var_t1_dn8 = assign68680_e105291_d_n8;
        locals.var_t1_dn9 = assign68680_e105291_d_n9;
        locals.var_t1_dn10 = assign68680_e105291_d_n10;
        locals.var_t1_dn13 = assign68680_e105291_d_n13;

        let assign68690_e105294: f64 = (-34.0);
        let assign68690_e105295: f64 = if locals.var_t1 < assign68690_e105294 { 1.0 } else { 0.0 };
        locals.var_guard1611 = assign68690_e105295;

        let (assign68700_e105301, assign68700_e105301_d_n0, assign68700_e105301_d_n2, assign68700_e105301_d_n4, assign68700_e105301_d_n5, assign68700_e105301_d_n6, assign68700_e105301_d_n7, assign68700_e105301_d_n8, assign68700_e105301_d_n9, assign68700_e105301_d_n10, assign68700_e105301_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1611 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn4, locals.var_igb_dn5, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn8, locals.var_igb_dn9, locals.var_igb_dn10, locals.var_igb_dn13,)
    }
};
        locals.var_igb = assign68700_e105301;
        locals.var_igb_dn0 = assign68700_e105301_d_n0;
        locals.var_igb_dn2 = assign68700_e105301_d_n2;
        locals.var_igb_dn4 = assign68700_e105301_d_n4;
        locals.var_igb_dn5 = assign68700_e105301_d_n5;
        locals.var_igb_dn6 = assign68700_e105301_d_n6;
        locals.var_igb_dn7 = assign68700_e105301_d_n7;
        locals.var_igb_dn8 = assign68700_e105301_d_n8;
        locals.var_igb_dn9 = assign68700_e105301_d_n9;
        locals.var_igb_dn10 = assign68700_e105301_d_n10;
        locals.var_igb_dn13 = assign68700_e105301_d_n13;

        let (assign68710_e105309, assign68710_e105309_d_n0, assign68710_e105309_d_n2, assign68710_e105309_d_n4, assign68710_e105309_d_n5, assign68710_e105309_d_n6, assign68710_e105309_d_n7, assign68710_e105309_d_n8, assign68710_e105309_d_n9, assign68710_e105309_d_n10, assign68710_e105309_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1611 == 0.0)) {
        let assign68710_e105307: f64 = (locals.var_t1).exp();
        (assign68710_e105307, (assign68710_e105307 * locals.var_t1_dn0), (assign68710_e105307 * locals.var_t1_dn2), (assign68710_e105307 * locals.var_t1_dn4), (assign68710_e105307 * locals.var_t1_dn5), (assign68710_e105307 * locals.var_t1_dn6), (assign68710_e105307 * locals.var_t1_dn7), (assign68710_e105307 * locals.var_t1_dn8), (assign68710_e105307 * locals.var_t1_dn9), (assign68710_e105307 * locals.var_t1_dn10), (assign68710_e105307 * locals.var_t1_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68710_e105309;
        locals.var_t2_dn0 = assign68710_e105309_d_n0;
        locals.var_t2_dn2 = assign68710_e105309_d_n2;
        locals.var_t2_dn4 = assign68710_e105309_d_n4;
        locals.var_t2_dn5 = assign68710_e105309_d_n5;
        locals.var_t2_dn6 = assign68710_e105309_d_n6;
        locals.var_t2_dn7 = assign68710_e105309_d_n7;
        locals.var_t2_dn8 = assign68710_e105309_d_n8;
        locals.var_t2_dn9 = assign68710_e105309_d_n9;
        locals.var_t2_dn10 = assign68710_e105309_d_n10;
        locals.var_t2_dn13 = assign68710_e105309_d_n13;

        let (assign68720_e105322, assign68720_e105322_d_n0, assign68720_e105322_d_n2, assign68720_e105322_d_n4, assign68720_e105322_d_n5, assign68720_e105322_d_n6, assign68720_e105322_d_n7, assign68720_e105322_d_n8, assign68720_e105322_d_n9, assign68720_e105322_d_n10, assign68720_e105322_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1611 == 0.0)) {
        let assign68720_e105317: f64 = (locals.var_etun * locals.var_etun);
        let assign68720_e105318: f64 = (locals.var_uc_glkb2 / assign68720_e105317);
        let assign68720_e105320: f64 = (assign68720_e105318 * locals.var_t2);
        (assign68720_e105320, (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn0)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn2)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn4)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn5)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn6)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn7)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn8)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn9 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn9))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn9)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn10)), (((-((locals.var_uc_glkb2 * ((locals.var_etun_dn13 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn13))) / (assign68720_e105317 * assign68720_e105317))) * locals.var_t2) + (assign68720_e105318 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68720_e105322;
        locals.var_t3_dn0 = assign68720_e105322_d_n0;
        locals.var_t3_dn2 = assign68720_e105322_d_n2;
        locals.var_t3_dn4 = assign68720_e105322_d_n4;
        locals.var_t3_dn5 = assign68720_e105322_d_n5;
        locals.var_t3_dn6 = assign68720_e105322_d_n6;
        locals.var_t3_dn7 = assign68720_e105322_d_n7;
        locals.var_t3_dn8 = assign68720_e105322_d_n8;
        locals.var_t3_dn9 = assign68720_e105322_d_n9;
        locals.var_t3_dn10 = assign68720_e105322_d_n10;
        locals.var_t3_dn13 = assign68720_e105322_d_n13;

        let (assign68730_e105333, assign68730_e105333_d_n0, assign68730_e105333_d_n2, assign68730_e105333_d_n4, assign68730_e105333_d_n5, assign68730_e105333_d_n6, assign68730_e105333_d_n7, assign68730_e105333_d_n8, assign68730_e105333_d_n9, assign68730_e105333_d_n10, assign68730_e105333_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1611 == 0.0)) {
        let assign68730_e105329: f64 = (locals.var_uc_glkb1 * locals.var_weff_nf);
        let assign68730_e105331: f64 = (assign68730_e105329 * locals.var_leff);
        (assign68730_e105331, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68730_e105333;
        locals.var_t3_dn0 = assign68730_e105333_d_n0;
        locals.var_t3_dn2 = assign68730_e105333_d_n2;
        locals.var_t3_dn4 = assign68730_e105333_d_n4;
        locals.var_t3_dn5 = assign68730_e105333_d_n5;
        locals.var_t3_dn6 = assign68730_e105333_d_n6;
        locals.var_t3_dn7 = assign68730_e105333_d_n7;
        locals.var_t3_dn8 = assign68730_e105333_d_n8;
        locals.var_t3_dn9 = assign68730_e105333_d_n9;
        locals.var_t3_dn10 = assign68730_e105333_d_n10;
        locals.var_t3_dn13 = assign68730_e105333_d_n13;

    }

    pub(super) fn stamp_transient_block_235(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign68740_e105346, assign68740_e105346_d_n0, assign68740_e105346_d_n2, assign68740_e105346_d_n4, assign68740_e105346_d_n5, assign68740_e105346_d_n6, assign68740_e105346_d_n7, assign68740_e105346_d_n8, assign68740_e105346_d_n9, assign68740_e105346_d_n10, assign68740_e105346_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1611 == 0.0)) {
        let assign68740_e105340: f64 = (locals.var_t3 * locals.var_etun);
        let assign68740_e105342: f64 = (assign68740_e105340 * locals.var_etun);
        let assign68740_e105344: f64 = (assign68740_e105342 * locals.var_t2);
        (assign68740_e105344, ((((((locals.var_t3_dn0 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn0)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn0)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn0)), ((((((locals.var_t3_dn2 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn2)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn2)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn2)), ((((((locals.var_t3_dn4 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn4)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn4)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn4)), ((((((locals.var_t3_dn5 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn5)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn5)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn5)), ((((((locals.var_t3_dn6 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn6)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn6)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn6)), ((((((locals.var_t3_dn7 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn7)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn7)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn7)), ((((((locals.var_t3_dn8 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn8)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn8)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn8)), ((((((locals.var_t3_dn9 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn9)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn9)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn9)), ((((((locals.var_t3_dn10 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn10)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn10)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn10)), ((((((locals.var_t3_dn13 * locals.var_etun) + (locals.var_t3 * locals.var_etun_dn13)) * locals.var_etun) + (assign68740_e105340 * locals.var_etun_dn13)) * locals.var_t2) + (assign68740_e105342 * locals.var_t2_dn13)),)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn4, locals.var_igb_dn5, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn8, locals.var_igb_dn9, locals.var_igb_dn10, locals.var_igb_dn13,)
    }
};
        locals.var_igb = assign68740_e105346;
        locals.var_igb_dn0 = assign68740_e105346_d_n0;
        locals.var_igb_dn2 = assign68740_e105346_d_n2;
        locals.var_igb_dn4 = assign68740_e105346_d_n4;
        locals.var_igb_dn5 = assign68740_e105346_d_n5;
        locals.var_igb_dn6 = assign68740_e105346_d_n6;
        locals.var_igb_dn7 = assign68740_e105346_d_n7;
        locals.var_igb_dn8 = assign68740_e105346_d_n8;
        locals.var_igb_dn9 = assign68740_e105346_d_n9;
        locals.var_igb_dn10 = assign68740_e105346_d_n10;
        locals.var_igb_dn13 = assign68740_e105346_d_n13;

        let (assign68750_e105350, assign68750_e105350_d_n0, assign68750_e105350_d_n2, assign68750_e105350_d_n4, assign68750_e105350_d_n5, assign68750_e105350_d_n6, assign68750_e105350_d_n7, assign68750_e105350_d_n8, assign68750_e105350_d_n9, assign68750_e105350_d_n10, assign68750_e105350_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn13,)
    } else {
        (locals.var_eg12, locals.var_eg12_dn0, locals.var_eg12_dn2, locals.var_eg12_dn4, locals.var_eg12_dn5, locals.var_eg12_dn6, locals.var_eg12_dn7, locals.var_eg12_dn8, locals.var_eg12_dn9, locals.var_eg12_dn10, locals.var_eg12_dn13,)
    }
};
        locals.var_eg12 = assign68750_e105350;
        locals.var_eg12_dn0 = assign68750_e105350_d_n0;
        locals.var_eg12_dn2 = assign68750_e105350_d_n2;
        locals.var_eg12_dn4 = assign68750_e105350_d_n4;
        locals.var_eg12_dn5 = assign68750_e105350_d_n5;
        locals.var_eg12_dn6 = assign68750_e105350_d_n6;
        locals.var_eg12_dn7 = assign68750_e105350_d_n7;
        locals.var_eg12_dn8 = assign68750_e105350_d_n8;
        locals.var_eg12_dn9 = assign68750_e105350_d_n9;
        locals.var_eg12_dn10 = assign68750_e105350_d_n10;
        locals.var_eg12_dn13 = assign68750_e105350_d_n13;

        let (assign68760_e105356, assign68760_e105356_d_n0, assign68760_e105356_d_n2, assign68760_e105356_d_n4, assign68760_e105356_d_n5, assign68760_e105356_d_n6, assign68760_e105356_d_n7, assign68760_e105356_d_n8, assign68760_e105356_d_n9, assign68760_e105356_d_n10, assign68760_e105356_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68760_e105354: f64 = (locals.var_eg * locals.var_eg12);
        (assign68760_e105354, ((locals.var_eg_dn0 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn0)), ((locals.var_eg_dn2 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn2)), ((locals.var_eg_dn4 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn4)), ((locals.var_eg_dn5 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn5)), ((locals.var_eg_dn6 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn6)), ((locals.var_eg_dn7 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn7)), ((locals.var_eg_dn8 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn8)), ((locals.var_eg_dn9 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn9)), ((locals.var_eg_dn10 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn10)), ((locals.var_eg_dn13 * locals.var_eg12) + (locals.var_eg * locals.var_eg12_dn13)),)
    } else {
        (locals.var_eg32, locals.var_eg32_dn0, locals.var_eg32_dn2, locals.var_eg32_dn4, locals.var_eg32_dn5, locals.var_eg32_dn6, locals.var_eg32_dn7, locals.var_eg32_dn8, locals.var_eg32_dn9, locals.var_eg32_dn10, locals.var_eg32_dn13,)
    }
};
        locals.var_eg32 = assign68760_e105356;
        locals.var_eg32_dn0 = assign68760_e105356_d_n0;
        locals.var_eg32_dn2 = assign68760_e105356_d_n2;
        locals.var_eg32_dn4 = assign68760_e105356_d_n4;
        locals.var_eg32_dn5 = assign68760_e105356_d_n5;
        locals.var_eg32_dn6 = assign68760_e105356_d_n6;
        locals.var_eg32_dn7 = assign68760_e105356_d_n7;
        locals.var_eg32_dn8 = assign68760_e105356_d_n8;
        locals.var_eg32_dn9 = assign68760_e105356_d_n9;
        locals.var_eg32_dn10 = assign68760_e105356_d_n10;
        locals.var_eg32_dn13 = assign68760_e105356_d_n13;

        let (assign68770_e105373, assign68770_e105373_d_n0, assign68770_e105373_d_n2, assign68770_e105373_d_n4, assign68770_e105373_d_n5, assign68770_e105373_d_n6, assign68770_e105373_d_n7, assign68770_e105373_d_n8, assign68770_e105373_d_n9, assign68770_e105373_d_n10, assign68770_e105373_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68770_e105360: f64 = (locals.var_uc_fvbs * locals.var_vbsz__blk438);
        let assign68770_e105362: f64 = (assign68770_e105360 - locals.var_vgsz__blk440);
        let assign68770_e105364: f64 = (assign68770_e105362 + locals.var_dvthsc);
        let assign68770_e105366: f64 = (assign68770_e105364 + locals.var_dvthlp);
        let assign68770_e105368: f64 = (assign68770_e105366 - locals.var_uc_fn3);
        let assign68770_e105369: f64 = (-assign68770_e105368);
        let assign68770_e105371: f64 = (assign68770_e105369 / locals.var_tox0);
        (assign68770_e105371, ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn0) - locals.var_vgsz__blk440_dn0) + locals.var_dvthsc_dn0) + locals.var_dvthlp_dn0)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn2) - locals.var_vgsz__blk440_dn2) + locals.var_dvthsc_dn2) + locals.var_dvthlp_dn2)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn4) - locals.var_vgsz__blk440_dn4) + locals.var_dvthsc_dn4) + locals.var_dvthlp_dn4)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn5) - locals.var_vgsz__blk440_dn5) + locals.var_dvthsc_dn5) + locals.var_dvthlp_dn5)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn6) - locals.var_vgsz__blk440_dn6) + locals.var_dvthsc_dn6) + locals.var_dvthlp_dn6)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn7) - locals.var_vgsz__blk440_dn7) + locals.var_dvthsc_dn7) + locals.var_dvthlp_dn7)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn8) - locals.var_vgsz__blk440_dn8) + locals.var_dvthsc_dn8) + locals.var_dvthlp_dn8)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn9) - locals.var_vgsz__blk440_dn9) + locals.var_dvthsc_dn9) + locals.var_dvthlp_dn9)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn10) - locals.var_vgsz__blk440_dn10) + locals.var_dvthsc_dn10) + locals.var_dvthlp_dn10)) / locals.var_tox0), ((-((((locals.var_uc_fvbs * locals.var_vbsz__blk438_dn13) - locals.var_vgsz__blk440_dn13) + locals.var_dvthsc_dn13) + locals.var_dvthlp_dn13)) / locals.var_tox0),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68770_e105373;
        locals.var_t2_dn0 = assign68770_e105373_d_n0;
        locals.var_t2_dn2 = assign68770_e105373_d_n2;
        locals.var_t2_dn4 = assign68770_e105373_d_n4;
        locals.var_t2_dn5 = assign68770_e105373_d_n5;
        locals.var_t2_dn6 = assign68770_e105373_d_n6;
        locals.var_t2_dn7 = assign68770_e105373_d_n7;
        locals.var_t2_dn8 = assign68770_e105373_d_n8;
        locals.var_t2_dn9 = assign68770_e105373_d_n9;
        locals.var_t2_dn10 = assign68770_e105373_d_n10;
        locals.var_t2_dn13 = assign68770_e105373_d_n13;

        let (assign68780_e105379, assign68780_e105379_d_n0, assign68780_e105379_d_n2, assign68780_e105379_d_n4, assign68780_e105379_d_n5, assign68780_e105379_d_n6, assign68780_e105379_d_n7, assign68780_e105379_d_n8, assign68780_e105379_d_n9, assign68780_e105379_d_n10, assign68780_e105379_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68780_e105377: f64 = (locals.var_t2 * locals.var_t2);
        (assign68780_e105377, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign68780_e105379;
        locals.var_t0_dn0 = assign68780_e105379_d_n0;
        locals.var_t0_dn2 = assign68780_e105379_d_n2;
        locals.var_t0_dn4 = assign68780_e105379_d_n4;
        locals.var_t0_dn5 = assign68780_e105379_d_n5;
        locals.var_t0_dn6 = assign68780_e105379_d_n6;
        locals.var_t0_dn7 = assign68780_e105379_d_n7;
        locals.var_t0_dn8 = assign68780_e105379_d_n8;
        locals.var_t0_dn9 = assign68780_e105379_d_n9;
        locals.var_t0_dn10 = assign68780_e105379_d_n10;
        locals.var_t0_dn13 = assign68780_e105379_d_n13;

        let (assign68790_e105385, assign68790_e105385_d_n0, assign68790_e105385_d_n2, assign68790_e105385_d_n4, assign68790_e105385_d_n5, assign68790_e105385_d_n6, assign68790_e105385_d_n7, assign68790_e105385_d_n8, assign68790_e105385_d_n9, assign68790_e105385_d_n10, assign68790_e105385_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68790_e105383: f64 = (locals.var_uc_fn2 * locals.var_eg32);
        (assign68790_e105383, (locals.var_uc_fn2 * locals.var_eg32_dn0), (locals.var_uc_fn2 * locals.var_eg32_dn2), (locals.var_uc_fn2 * locals.var_eg32_dn4), (locals.var_uc_fn2 * locals.var_eg32_dn5), (locals.var_uc_fn2 * locals.var_eg32_dn6), (locals.var_uc_fn2 * locals.var_eg32_dn7), (locals.var_uc_fn2 * locals.var_eg32_dn8), (locals.var_uc_fn2 * locals.var_eg32_dn9), (locals.var_uc_fn2 * locals.var_eg32_dn10), (locals.var_uc_fn2 * locals.var_eg32_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68790_e105385;
        locals.var_t1_dn0 = assign68790_e105385_d_n0;
        locals.var_t1_dn2 = assign68790_e105385_d_n2;
        locals.var_t1_dn4 = assign68790_e105385_d_n4;
        locals.var_t1_dn5 = assign68790_e105385_d_n5;
        locals.var_t1_dn6 = assign68790_e105385_d_n6;
        locals.var_t1_dn7 = assign68790_e105385_d_n7;
        locals.var_t1_dn8 = assign68790_e105385_d_n8;
        locals.var_t1_dn9 = assign68790_e105385_d_n9;
        locals.var_t1_dn10 = assign68790_e105385_d_n10;
        locals.var_t1_dn13 = assign68790_e105385_d_n13;

        let (assign68800_e105392, assign68800_e105392_d_n0, assign68800_e105392_d_n2, assign68800_e105392_d_n4, assign68800_e105392_d_n5, assign68800_e105392_d_n6, assign68800_e105392_d_n7, assign68800_e105392_d_n8, assign68800_e105392_d_n9, assign68800_e105392_d_n10, assign68800_e105392_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68800_e105388: f64 = (-locals.var_t1);
        let assign68800_e105390: f64 = (assign68800_e105388 / locals.var_t2);
        (assign68800_e105390, ((((-locals.var_t1_dn0) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn2) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn4) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn5) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn6) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn7) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn8) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn9) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn10) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), ((((-locals.var_t1_dn13) * locals.var_t2) - (assign68800_e105388 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign68800_e105392;
        locals.var_t3_dn0 = assign68800_e105392_d_n0;
        locals.var_t3_dn2 = assign68800_e105392_d_n2;
        locals.var_t3_dn4 = assign68800_e105392_d_n4;
        locals.var_t3_dn5 = assign68800_e105392_d_n5;
        locals.var_t3_dn6 = assign68800_e105392_d_n6;
        locals.var_t3_dn7 = assign68800_e105392_d_n7;
        locals.var_t3_dn8 = assign68800_e105392_d_n8;
        locals.var_t3_dn9 = assign68800_e105392_d_n9;
        locals.var_t3_dn10 = assign68800_e105392_d_n10;
        locals.var_t3_dn13 = assign68800_e105392_d_n13;

        let assign68810_e105395: f64 = (-34.0);
        let assign68810_e105396: f64 = if locals.var_t3 < assign68810_e105395 { 1.0 } else { 0.0 };
        locals.var_guard1612 = assign68810_e105396;

        let (assign68820_e105402, assign68820_e105402_d_n0, assign68820_e105402_d_n2, assign68820_e105402_d_n4, assign68820_e105402_d_n5, assign68820_e105402_d_n6, assign68820_e105402_d_n7, assign68820_e105402_d_n8, assign68820_e105402_d_n9, assign68820_e105402_d_n10, assign68820_e105402_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1612 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68820_e105402;
        locals.var_t5_dn0 = assign68820_e105402_d_n0;
        locals.var_t5_dn2 = assign68820_e105402_d_n2;
        locals.var_t5_dn4 = assign68820_e105402_d_n4;
        locals.var_t5_dn5 = assign68820_e105402_d_n5;
        locals.var_t5_dn6 = assign68820_e105402_d_n6;
        locals.var_t5_dn7 = assign68820_e105402_d_n7;
        locals.var_t5_dn8 = assign68820_e105402_d_n8;
        locals.var_t5_dn9 = assign68820_e105402_d_n9;
        locals.var_t5_dn10 = assign68820_e105402_d_n10;
        locals.var_t5_dn13 = assign68820_e105402_d_n13;

        let (assign68830_e105410, assign68830_e105410_d_n0, assign68830_e105410_d_n2, assign68830_e105410_d_n4, assign68830_e105410_d_n5, assign68830_e105410_d_n6, assign68830_e105410_d_n7, assign68830_e105410_d_n8, assign68830_e105410_d_n9, assign68830_e105410_d_n10, assign68830_e105410_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1612 == 0.0)) {
        let assign68830_e105408: f64 = (locals.var_t3).exp();
        (assign68830_e105408, (assign68830_e105408 * locals.var_t3_dn0), (assign68830_e105408 * locals.var_t3_dn2), (assign68830_e105408 * locals.var_t3_dn4), (assign68830_e105408 * locals.var_t3_dn5), (assign68830_e105408 * locals.var_t3_dn6), (assign68830_e105408 * locals.var_t3_dn7), (assign68830_e105408 * locals.var_t3_dn8), (assign68830_e105408 * locals.var_t3_dn9), (assign68830_e105408 * locals.var_t3_dn10), (assign68830_e105408 * locals.var_t3_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68830_e105410;
        locals.var_t5_dn0 = assign68830_e105410_d_n0;
        locals.var_t5_dn2 = assign68830_e105410_d_n2;
        locals.var_t5_dn4 = assign68830_e105410_d_n4;
        locals.var_t5_dn5 = assign68830_e105410_d_n5;
        locals.var_t5_dn6 = assign68830_e105410_d_n6;
        locals.var_t5_dn7 = assign68830_e105410_d_n7;
        locals.var_t5_dn8 = assign68830_e105410_d_n8;
        locals.var_t5_dn9 = assign68830_e105410_d_n9;
        locals.var_t5_dn10 = assign68830_e105410_d_n10;
        locals.var_t5_dn13 = assign68830_e105410_d_n13;

        let (assign68840_e105422, assign68840_e105422_d_n0, assign68840_e105422_d_n2, assign68840_e105422_d_n4, assign68840_e105422_d_n5, assign68840_e105422_d_n6, assign68840_e105422_d_n7, assign68840_e105422_d_n8, assign68840_e105422_d_n9, assign68840_e105422_d_n10, assign68840_e105422_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68840_e105414: f64 = (1.6021918e-19 * locals.var_uc_fn1);
        let assign68840_e105416: f64 = (assign68840_e105414 * locals.var_weff_nf);
        let assign68840_e105418: f64 = (assign68840_e105416 * locals.var_lgate);
        let assign68840_e105420: f64 = (assign68840_e105418 / locals.var_eg12);
        (assign68840_e105420, (-((assign68840_e105418 * locals.var_eg12_dn0) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn2) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn4) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn5) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn6) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn7) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn8) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn9) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn10) / (locals.var_eg12 * locals.var_eg12))), (-((assign68840_e105418 * locals.var_eg12_dn13) / (locals.var_eg12 * locals.var_eg12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign68840_e105422;
        locals.var_t4_dn0 = assign68840_e105422_d_n0;
        locals.var_t4_dn2 = assign68840_e105422_d_n2;
        locals.var_t4_dn4 = assign68840_e105422_d_n4;
        locals.var_t4_dn5 = assign68840_e105422_d_n5;
        locals.var_t4_dn6 = assign68840_e105422_d_n6;
        locals.var_t4_dn7 = assign68840_e105422_d_n7;
        locals.var_t4_dn8 = assign68840_e105422_d_n8;
        locals.var_t4_dn9 = assign68840_e105422_d_n9;
        locals.var_t4_dn10 = assign68840_e105422_d_n10;
        locals.var_t4_dn13 = assign68840_e105422_d_n13;

        let assign68850_e105425: f64 = (2.0 * locals.var_t2);
        let assign68850_e105427: f64 = (assign68850_e105425 + locals.var_t1);
        let assign68850_e105429: f64 = if assign68850_e105427 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1613 = assign68850_e105429;

        let (assign68860_e105443, assign68860_e105443_d_n0, assign68860_e105443_d_n2, assign68860_e105443_d_n4, assign68860_e105443_d_n5, assign68860_e105443_d_n6, assign68860_e105443_d_n7, assign68860_e105443_d_n8, assign68860_e105443_d_n9, assign68860_e105443_d_n10, assign68860_e105443_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1613 != 0.0)) {
        let assign68860_e105435: f64 = (0.25 * locals.var_t4);
        let assign68860_e105437: f64 = (assign68860_e105435 * locals.var_t1);
        let assign68860_e105439: f64 = (assign68860_e105437 * locals.var_t1);
        let assign68860_e105441: f64 = (assign68860_e105439 * 7.38905609893065);
        (assign68860_e105441, ((((((0.25 * locals.var_t4_dn0) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn0)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn0)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn2) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn2)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn2)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn4) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn4)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn4)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn5) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn5)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn5)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn6) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn6)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn6)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn7) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn7)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn7)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn8) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn8)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn8)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn9) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn9)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn9)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn10) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn10)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn10)) * 7.38905609893065), ((((((0.25 * locals.var_t4_dn13) * locals.var_t1) + (assign68860_e105435 * locals.var_t1_dn13)) * locals.var_t1) + (assign68860_e105437 * locals.var_t1_dn13)) * 7.38905609893065),)
    } else {
        (locals.var_ifn, locals.var_ifn_dn0, locals.var_ifn_dn2, locals.var_ifn_dn4, locals.var_ifn_dn5, locals.var_ifn_dn6, locals.var_ifn_dn7, locals.var_ifn_dn8, locals.var_ifn_dn9, locals.var_ifn_dn10, locals.var_ifn_dn13,)
    }
};
        locals.var_ifn = assign68860_e105443;
        locals.var_ifn_dn0 = assign68860_e105443_d_n0;
        locals.var_ifn_dn2 = assign68860_e105443_d_n2;
        locals.var_ifn_dn4 = assign68860_e105443_d_n4;
        locals.var_ifn_dn5 = assign68860_e105443_d_n5;
        locals.var_ifn_dn6 = assign68860_e105443_d_n6;
        locals.var_ifn_dn7 = assign68860_e105443_d_n7;
        locals.var_ifn_dn8 = assign68860_e105443_d_n8;
        locals.var_ifn_dn9 = assign68860_e105443_d_n9;
        locals.var_ifn_dn10 = assign68860_e105443_d_n10;
        locals.var_ifn_dn13 = assign68860_e105443_d_n13;

        let (assign68870_e105454, assign68870_e105454_d_n0, assign68870_e105454_d_n2, assign68870_e105454_d_n4, assign68870_e105454_d_n5, assign68870_e105454_d_n6, assign68870_e105454_d_n7, assign68870_e105454_d_n8, assign68870_e105454_d_n9, assign68870_e105454_d_n10, assign68870_e105454_d_n13,) = {
    if ((locals.var_guard1603 != 0.0) && (locals.var_guard1613 == 0.0)) {
        let assign68870_e105450: f64 = (locals.var_t4 * locals.var_t0);
        let assign68870_e105452: f64 = (assign68870_e105450 * locals.var_t5);
        (assign68870_e105452, ((((locals.var_t4_dn0 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn0)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn0)), ((((locals.var_t4_dn2 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn2)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn2)), ((((locals.var_t4_dn4 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn4)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn4)), ((((locals.var_t4_dn5 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn5)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn5)), ((((locals.var_t4_dn6 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn6)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn6)), ((((locals.var_t4_dn7 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn7)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn7)), ((((locals.var_t4_dn8 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn8)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn8)), ((((locals.var_t4_dn9 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn9)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn9)), ((((locals.var_t4_dn10 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn10)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn10)), ((((locals.var_t4_dn13 * locals.var_t0) + (locals.var_t4 * locals.var_t0_dn13)) * locals.var_t5) + (assign68870_e105450 * locals.var_t5_dn13)),)
    } else {
        (locals.var_ifn, locals.var_ifn_dn0, locals.var_ifn_dn2, locals.var_ifn_dn4, locals.var_ifn_dn5, locals.var_ifn_dn6, locals.var_ifn_dn7, locals.var_ifn_dn8, locals.var_ifn_dn9, locals.var_ifn_dn10, locals.var_ifn_dn13,)
    }
};
        locals.var_ifn = assign68870_e105454;
        locals.var_ifn_dn0 = assign68870_e105454_d_n0;
        locals.var_ifn_dn2 = assign68870_e105454_d_n2;
        locals.var_ifn_dn4 = assign68870_e105454_d_n4;
        locals.var_ifn_dn5 = assign68870_e105454_d_n5;
        locals.var_ifn_dn6 = assign68870_e105454_d_n6;
        locals.var_ifn_dn7 = assign68870_e105454_d_n7;
        locals.var_ifn_dn8 = assign68870_e105454_d_n8;
        locals.var_ifn_dn9 = assign68870_e105454_d_n9;
        locals.var_ifn_dn10 = assign68870_e105454_d_n10;
        locals.var_ifn_dn13 = assign68870_e105454_d_n13;

        let (assign68880_e105460, assign68880_e105460_d_n0, assign68880_e105460_d_n2, assign68880_e105460_d_n4, assign68880_e105460_d_n5, assign68880_e105460_d_n6, assign68880_e105460_d_n7, assign68880_e105460_d_n8, assign68880_e105460_d_n9, assign68880_e105460_d_n10, assign68880_e105460_d_n13,) = {
    if (locals.var_guard1603 != 0.0) {
        let assign68880_e105458: f64 = (locals.var_igb - locals.var_ifn);
        (assign68880_e105458, (locals.var_igb_dn0 - locals.var_ifn_dn0), (locals.var_igb_dn2 - locals.var_ifn_dn2), (locals.var_igb_dn4 - locals.var_ifn_dn4), (locals.var_igb_dn5 - locals.var_ifn_dn5), (locals.var_igb_dn6 - locals.var_ifn_dn6), (locals.var_igb_dn7 - locals.var_ifn_dn7), (locals.var_igb_dn8 - locals.var_ifn_dn8), (locals.var_igb_dn9 - locals.var_ifn_dn9), (locals.var_igb_dn10 - locals.var_ifn_dn10), (locals.var_igb_dn13 - locals.var_ifn_dn13),)
    } else {
        (locals.var_igb, locals.var_igb_dn0, locals.var_igb_dn2, locals.var_igb_dn4, locals.var_igb_dn5, locals.var_igb_dn6, locals.var_igb_dn7, locals.var_igb_dn8, locals.var_igb_dn9, locals.var_igb_dn10, locals.var_igb_dn13,)
    }
};
        locals.var_igb = assign68880_e105460;
        locals.var_igb_dn0 = assign68880_e105460_d_n0;
        locals.var_igb_dn2 = assign68880_e105460_d_n2;
        locals.var_igb_dn4 = assign68880_e105460_d_n4;
        locals.var_igb_dn5 = assign68880_e105460_d_n5;
        locals.var_igb_dn6 = assign68880_e105460_d_n6;
        locals.var_igb_dn7 = assign68880_e105460_d_n7;
        locals.var_igb_dn8 = assign68880_e105460_d_n8;
        locals.var_igb_dn9 = assign68880_e105460_d_n9;
        locals.var_igb_dn10 = assign68880_e105460_d_n10;
        locals.var_igb_dn13 = assign68880_e105460_d_n13;

        let assign68890_e105463: f64 = if p.p25 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1614 = assign68890_e105463;

        let (assign68900_e105475, assign68900_e105475_d_n0, assign68900_e105475_d_n2, assign68900_e105475_d_n4, assign68900_e105475_d_n5, assign68900_e105475_d_n6, assign68900_e105475_d_n7, assign68900_e105475_d_n8, assign68900_e105475_d_n9, assign68900_e105475_d_n10, assign68900_e105475_d_n13,) = {
    if (locals.var_guard1614 != 0.0) {
        let assign68900_e105469: f64 = (100.0 * locals.var_vds);
        let assign68900_e105470: f64 = (1.0 - assign68900_e105469);
        let assign68900_e105471: f64 = (locals.var_vds * assign68900_e105470);
        let assign68900_e105473: f64 = (assign68900_e105471 - 1e-5);
        (assign68900_e105473, ((locals.var_vds_dn0 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn0)))), ((locals.var_vds_dn2 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn2)))), ((locals.var_vds_dn4 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn4)))), ((locals.var_vds_dn5 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn5)))), ((locals.var_vds_dn6 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn6)))), ((locals.var_vds_dn7 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn7)))), ((locals.var_vds_dn8 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn8)))), ((locals.var_vds_dn9 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn9)))), ((locals.var_vds_dn10 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn10)))), ((locals.var_vds_dn13 * assign68900_e105470) + (locals.var_vds * (-(100.0 * locals.var_vds_dn13)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68900_e105475;
        locals.var_t1_dn0 = assign68900_e105475_d_n0;
        locals.var_t1_dn2 = assign68900_e105475_d_n2;
        locals.var_t1_dn4 = assign68900_e105475_d_n4;
        locals.var_t1_dn5 = assign68900_e105475_d_n5;
        locals.var_t1_dn6 = assign68900_e105475_d_n6;
        locals.var_t1_dn7 = assign68900_e105475_d_n7;
        locals.var_t1_dn8 = assign68900_e105475_d_n8;
        locals.var_t1_dn9 = assign68900_e105475_d_n9;
        locals.var_t1_dn10 = assign68900_e105475_d_n10;
        locals.var_t1_dn13 = assign68900_e105475_d_n13;

        let (assign68910_e105488, assign68910_e105488_d_n0, assign68910_e105488_d_n2, assign68910_e105488_d_n4, assign68910_e105488_d_n5, assign68910_e105488_d_n6, assign68910_e105488_d_n7, assign68910_e105488_d_n8, assign68910_e105488_d_n9, assign68910_e105488_d_n10, assign68910_e105488_d_n13,) = {
    if (locals.var_guard1614 != 0.0) {
        let assign68910_e105479: f64 = (locals.var_t1 * locals.var_t1);
        let assign68910_e105482: f64 = (4.0 * 1e-5);
        let assign68910_e105484: f64 = (assign68910_e105482 * locals.var_vds);
        let assign68910_e105485: f64 = (assign68910_e105479 + assign68910_e105484);
        let assign68910_e105486: f64 = (assign68910_e105485).sqrt();
        (assign68910_e105486, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (assign68910_e105482 * locals.var_vds_dn0)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (assign68910_e105482 * locals.var_vds_dn2)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (assign68910_e105482 * locals.var_vds_dn4)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (assign68910_e105482 * locals.var_vds_dn5)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (assign68910_e105482 * locals.var_vds_dn6)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (assign68910_e105482 * locals.var_vds_dn7)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (assign68910_e105482 * locals.var_vds_dn8)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (assign68910_e105482 * locals.var_vds_dn9)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (assign68910_e105482 * locals.var_vds_dn10)) / (2.0 * assign68910_e105486)), ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + (assign68910_e105482 * locals.var_vds_dn13)) / (2.0 * assign68910_e105486)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68910_e105488;
        locals.var_t2_dn0 = assign68910_e105488_d_n0;
        locals.var_t2_dn2 = assign68910_e105488_d_n2;
        locals.var_t2_dn4 = assign68910_e105488_d_n4;
        locals.var_t2_dn5 = assign68910_e105488_d_n5;
        locals.var_t2_dn6 = assign68910_e105488_d_n6;
        locals.var_t2_dn7 = assign68910_e105488_d_n7;
        locals.var_t2_dn8 = assign68910_e105488_d_n8;
        locals.var_t2_dn9 = assign68910_e105488_d_n9;
        locals.var_t2_dn10 = assign68910_e105488_d_n10;
        locals.var_t2_dn13 = assign68910_e105488_d_n13;

        let (assign68920_e105498, assign68920_e105498_d_n0, assign68920_e105498_d_n2, assign68920_e105498_d_n4, assign68920_e105498_d_n5, assign68920_e105498_d_n6, assign68920_e105498_d_n7, assign68920_e105498_d_n8, assign68920_e105498_d_n9, assign68920_e105498_d_n10, assign68920_e105498_d_n13,) = {
    if (locals.var_guard1614 != 0.0) {
        let assign68920_e105494: f64 = (locals.var_t1 + locals.var_t2);
        let assign68920_e105495: f64 = (0.5 * assign68920_e105494);
        let assign68920_e105496: f64 = (locals.var_vds - assign68920_e105495);
        (assign68920_e105496, (locals.var_vds_dn0 - (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))), (locals.var_vds_dn2 - (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))), (locals.var_vds_dn4 - (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))), (locals.var_vds_dn5 - (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))), (locals.var_vds_dn6 - (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))), (locals.var_vds_dn7 - (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7))), (locals.var_vds_dn8 - (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))), (locals.var_vds_dn9 - (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9))), (locals.var_vds_dn10 - (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))), (locals.var_vds_dn13 - (0.5 * (locals.var_t1_dn13 + locals.var_t2_dn13))),)
    } else {
        (locals.var_vdsp, locals.var_vdsp_dn0, locals.var_vdsp_dn2, locals.var_vdsp_dn4, locals.var_vdsp_dn5, locals.var_vdsp_dn6, locals.var_vdsp_dn7, locals.var_vdsp_dn8, locals.var_vdsp_dn9, locals.var_vdsp_dn10, locals.var_vdsp_dn13,)
    }
};
        locals.var_vdsp = assign68920_e105498;
        locals.var_vdsp_dn0 = assign68920_e105498_d_n0;
        locals.var_vdsp_dn2 = assign68920_e105498_d_n2;
        locals.var_vdsp_dn4 = assign68920_e105498_d_n4;
        locals.var_vdsp_dn5 = assign68920_e105498_d_n5;
        locals.var_vdsp_dn6 = assign68920_e105498_d_n6;
        locals.var_vdsp_dn7 = assign68920_e105498_d_n7;
        locals.var_vdsp_dn8 = assign68920_e105498_d_n8;
        locals.var_vdsp_dn9 = assign68920_e105498_d_n9;
        locals.var_vdsp_dn10 = assign68920_e105498_d_n10;
        locals.var_vdsp_dn13 = assign68920_e105498_d_n13;

        let assign68930_e105501: f64 = if p.p25 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1615 = assign68930_e105501;

        let (assign68940_e105505, assign68940_e105505_d_n0, assign68940_e105505_d_n2, assign68940_e105505_d_n4, assign68940_e105505_d_n5, assign68940_e105505_d_n6, assign68940_e105505_d_n7, assign68940_e105505_d_n8, assign68940_e105505_d_n9, assign68940_e105505_d_n10, assign68940_e105505_d_n13,) = {
    if (locals.var_guard1615 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn13,)
    }
};
        locals.var_igidl = assign68940_e105505;
        locals.var_igidl_dn0 = assign68940_e105505_d_n0;
        locals.var_igidl_dn2 = assign68940_e105505_d_n2;
        locals.var_igidl_dn4 = assign68940_e105505_d_n4;
        locals.var_igidl_dn5 = assign68940_e105505_d_n5;
        locals.var_igidl_dn6 = assign68940_e105505_d_n6;
        locals.var_igidl_dn7 = assign68940_e105505_d_n7;
        locals.var_igidl_dn8 = assign68940_e105505_d_n8;
        locals.var_igidl_dn9 = assign68940_e105505_d_n9;
        locals.var_igidl_dn10 = assign68940_e105505_d_n10;
        locals.var_igidl_dn13 = assign68940_e105505_d_n13;

        let (assign68950_e105522, assign68950_e105522_d_n0, assign68950_e105522_d_n2, assign68950_e105522_d_n4, assign68950_e105522_d_n5, assign68950_e105522_d_n6, assign68950_e105522_d_n7, assign68950_e105522_d_n8, assign68950_e105522_d_n9, assign68950_e105522_d_n10, assign68950_e105522_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign68950_e105511: f64 = (locals.var_vdsp + p.p243);
        let assign68950_e105512: f64 = (p.p242 * assign68950_e105511);
        let assign68950_e105514: f64 = (assign68950_e105512 - locals.var_vgs);
        let assign68950_e105517: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign68950_e105519: f64 = (assign68950_e105517 * p.p244);
        let assign68950_e105520: f64 = (assign68950_e105514 + assign68950_e105519);
        (assign68950_e105520, ((p.p242 * locals.var_vdsp_dn0) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p244)), ((p.p242 * locals.var_vdsp_dn2) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p244)), ((p.p242 * locals.var_vdsp_dn4) + ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p244)), (((p.p242 * locals.var_vdsp_dn5) - locals.var_vgs_dn5) + ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p244)), (((p.p242 * locals.var_vdsp_dn6) - locals.var_vgs_dn6) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p244)), (((p.p242 * locals.var_vdsp_dn7) - locals.var_vgs_dn7) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p244)), ((p.p242 * locals.var_vdsp_dn8) + ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p244)), ((p.p242 * locals.var_vdsp_dn9) + ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) * p.p244)), ((p.p242 * locals.var_vdsp_dn10) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p244)), ((p.p242 * locals.var_vdsp_dn13) + ((locals.var_dvthsc_dn13 + locals.var_dvthlp_dn13) * p.p244)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign68950_e105522;
        locals.var_t1_dn0 = assign68950_e105522_d_n0;
        locals.var_t1_dn2 = assign68950_e105522_d_n2;
        locals.var_t1_dn4 = assign68950_e105522_d_n4;
        locals.var_t1_dn5 = assign68950_e105522_d_n5;
        locals.var_t1_dn6 = assign68950_e105522_d_n6;
        locals.var_t1_dn7 = assign68950_e105522_d_n7;
        locals.var_t1_dn8 = assign68950_e105522_d_n8;
        locals.var_t1_dn9 = assign68950_e105522_d_n9;
        locals.var_t1_dn10 = assign68950_e105522_d_n10;
        locals.var_t1_dn13 = assign68950_e105522_d_n13;

        let (assign68960_e105529, assign68960_e105529_d_n0, assign68960_e105529_d_n2, assign68960_e105529_d_n4, assign68960_e105529_d_n5, assign68960_e105529_d_n6, assign68960_e105529_d_n7, assign68960_e105529_d_n8, assign68960_e105529_d_n9, assign68960_e105529_d_n10, assign68960_e105529_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign68960_e105527: f64 = (1.0 / locals.var_tox0);
        (assign68960_e105527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign68960_e105529;
        locals.var_t2_dn0 = assign68960_e105529_d_n0;
        locals.var_t2_dn2 = assign68960_e105529_d_n2;
        locals.var_t2_dn4 = assign68960_e105529_d_n4;
        locals.var_t2_dn5 = assign68960_e105529_d_n5;
        locals.var_t2_dn6 = assign68960_e105529_d_n6;
        locals.var_t2_dn7 = assign68960_e105529_d_n7;
        locals.var_t2_dn8 = assign68960_e105529_d_n8;
        locals.var_t2_dn9 = assign68960_e105529_d_n9;
        locals.var_t2_dn10 = assign68960_e105529_d_n10;
        locals.var_t2_dn13 = assign68960_e105529_d_n13;

        let (assign68970_e105536, assign68970_e105536_d_n0, assign68970_e105536_d_n2, assign68970_e105536_d_n4, assign68970_e105536_d_n5, assign68970_e105536_d_n6, assign68970_e105536_d_n7, assign68970_e105536_d_n8, assign68970_e105536_d_n9, assign68970_e105536_d_n10, assign68970_e105536_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign68970_e105534: f64 = (locals.var_t1 * locals.var_t2);
        (assign68970_e105534, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn13,)
    }
};
        locals.var_e1 = assign68970_e105536;
        locals.var_e1_dn0 = assign68970_e105536_d_n0;
        locals.var_e1_dn2 = assign68970_e105536_d_n2;
        locals.var_e1_dn4 = assign68970_e105536_d_n4;
        locals.var_e1_dn5 = assign68970_e105536_d_n5;
        locals.var_e1_dn6 = assign68970_e105536_d_n6;
        locals.var_e1_dn7 = assign68970_e105536_d_n7;
        locals.var_e1_dn8 = assign68970_e105536_d_n8;
        locals.var_e1_dn9 = assign68970_e105536_d_n9;
        locals.var_e1_dn10 = assign68970_e105536_d_n10;
        locals.var_e1_dn13 = assign68970_e105536_d_n13;

        let (assign68980_e105554, assign68980_e105554_d_n0, assign68980_e105554_d_n2, assign68980_e105554_d_n4, assign68980_e105554_d_n5, assign68980_e105554_d_n6, assign68980_e105554_d_n7, assign68980_e105554_d_n8, assign68980_e105554_d_n9, assign68980_e105554_d_n10, assign68980_e105554_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign68980_e105541: f64 = (locals.var_e1 * locals.var_e1);
        let assign68980_e105545: f64 = (0.01 / 0.01);
        let assign68980_e105546: f64 = (4.0 * assign68980_e105545);
        let assign68980_e105549: f64 = (0.01 / 0.01);
        let assign68980_e105550: f64 = (assign68980_e105546 * assign68980_e105549);
        let assign68980_e105551: f64 = (assign68980_e105541 + assign68980_e105550);
        let assign68980_e105552: f64 = (assign68980_e105551).sqrt();
        (assign68980_e105552, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn9 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn9)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign68980_e105552)), (((locals.var_e1_dn13 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn13)) / (2.0 * assign68980_e105552)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign68980_e105554;
        locals.var_tmf2_dn0 = assign68980_e105554_d_n0;
        locals.var_tmf2_dn2 = assign68980_e105554_d_n2;
        locals.var_tmf2_dn4 = assign68980_e105554_d_n4;
        locals.var_tmf2_dn5 = assign68980_e105554_d_n5;
        locals.var_tmf2_dn6 = assign68980_e105554_d_n6;
        locals.var_tmf2_dn7 = assign68980_e105554_d_n7;
        locals.var_tmf2_dn8 = assign68980_e105554_d_n8;
        locals.var_tmf2_dn9 = assign68980_e105554_d_n9;
        locals.var_tmf2_dn10 = assign68980_e105554_d_n10;
        locals.var_tmf2_dn13 = assign68980_e105554_d_n13;

        let (assign68990_e105565, assign68990_e105565_d_n0, assign68990_e105565_d_n2, assign68990_e105565_d_n4, assign68990_e105565_d_n5, assign68990_e105565_d_n6, assign68990_e105565_d_n7, assign68990_e105565_d_n8, assign68990_e105565_d_n9, assign68990_e105565_d_n10, assign68990_e105565_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign68990_e105561: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign68990_e105562: f64 = (1.0 + assign68990_e105561);
        let assign68990_e105563: f64 = (0.5 * assign68990_e105562);
        (assign68990_e105563, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn7 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn9 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn13 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign68990_e105565;
        locals.var_t5_dn0 = assign68990_e105565_d_n0;
        locals.var_t5_dn2 = assign68990_e105565_d_n2;
        locals.var_t5_dn4 = assign68990_e105565_d_n4;
        locals.var_t5_dn5 = assign68990_e105565_d_n5;
        locals.var_t5_dn6 = assign68990_e105565_d_n6;
        locals.var_t5_dn7 = assign68990_e105565_d_n7;
        locals.var_t5_dn8 = assign68990_e105565_d_n8;
        locals.var_t5_dn9 = assign68990_e105565_d_n9;
        locals.var_t5_dn10 = assign68990_e105565_d_n10;
        locals.var_t5_dn13 = assign68990_e105565_d_n13;

        let (assign69000_e105574, assign69000_e105574_d_n0, assign69000_e105574_d_n2, assign69000_e105574_d_n4, assign69000_e105574_d_n5, assign69000_e105574_d_n6, assign69000_e105574_d_n7, assign69000_e105574_d_n8, assign69000_e105574_d_n9, assign69000_e105574_d_n10, assign69000_e105574_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign69000_e105571: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign69000_e105572: f64 = (0.5 * assign69000_e105571);
        (assign69000_e105572, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn8, locals.var_egidl_dn9, locals.var_egidl_dn10, locals.var_egidl_dn13,)
    }
};
        locals.var_egidl = assign69000_e105574;
        locals.var_egidl_dn0 = assign69000_e105574_d_n0;
        locals.var_egidl_dn2 = assign69000_e105574_d_n2;
        locals.var_egidl_dn4 = assign69000_e105574_d_n4;
        locals.var_egidl_dn5 = assign69000_e105574_d_n5;
        locals.var_egidl_dn6 = assign69000_e105574_d_n6;
        locals.var_egidl_dn7 = assign69000_e105574_d_n7;
        locals.var_egidl_dn8 = assign69000_e105574_d_n8;
        locals.var_egidl_dn9 = assign69000_e105574_d_n9;
        locals.var_egidl_dn10 = assign69000_e105574_d_n10;
        locals.var_egidl_dn13 = assign69000_e105574_d_n13;

    }

    pub(super) fn stamp_transient_block_236(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign69010_e105577: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1616 = assign69010_e105577;

        let (assign69020_e105584, assign69020_e105584_d_n0, assign69020_e105584_d_n2, assign69020_e105584_d_n4, assign69020_e105584_d_n5, assign69020_e105584_d_n6, assign69020_e105584_d_n7, assign69020_e105584_d_n8, assign69020_e105584_d_n9, assign69020_e105584_d_n10, assign69020_e105584_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1616 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn4, locals.var_egidl_dn5, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn8, locals.var_egidl_dn9, locals.var_egidl_dn10, locals.var_egidl_dn13,)
    }
};
        locals.var_egidl = assign69020_e105584;
        locals.var_egidl_dn0 = assign69020_e105584_d_n0;
        locals.var_egidl_dn2 = assign69020_e105584_d_n2;
        locals.var_egidl_dn4 = assign69020_e105584_d_n4;
        locals.var_egidl_dn5 = assign69020_e105584_d_n5;
        locals.var_egidl_dn6 = assign69020_e105584_d_n6;
        locals.var_egidl_dn7 = assign69020_e105584_d_n7;
        locals.var_egidl_dn8 = assign69020_e105584_d_n8;
        locals.var_egidl_dn9 = assign69020_e105584_d_n9;
        locals.var_egidl_dn10 = assign69020_e105584_d_n10;
        locals.var_egidl_dn13 = assign69020_e105584_d_n13;

        let (assign69030_e105591, assign69030_e105591_d_n0, assign69030_e105591_d_n2, assign69030_e105591_d_n4, assign69030_e105591_d_n5, assign69030_e105591_d_n6, assign69030_e105591_d_n7, assign69030_e105591_d_n8, assign69030_e105591_d_n9, assign69030_e105591_d_n10, assign69030_e105591_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1616 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69030_e105591;
        locals.var_t5_dn0 = assign69030_e105591_d_n0;
        locals.var_t5_dn2 = assign69030_e105591_d_n2;
        locals.var_t5_dn4 = assign69030_e105591_d_n4;
        locals.var_t5_dn5 = assign69030_e105591_d_n5;
        locals.var_t5_dn6 = assign69030_e105591_d_n6;
        locals.var_t5_dn7 = assign69030_e105591_d_n7;
        locals.var_t5_dn8 = assign69030_e105591_d_n8;
        locals.var_t5_dn9 = assign69030_e105591_d_n9;
        locals.var_t5_dn10 = assign69030_e105591_d_n10;
        locals.var_t5_dn13 = assign69030_e105591_d_n13;

        let (assign69040_e105600, assign69040_e105600_d_n0, assign69040_e105600_d_n2, assign69040_e105600_d_n4, assign69040_e105600_d_n5, assign69040_e105600_d_n6, assign69040_e105600_d_n7, assign69040_e105600_d_n8, assign69040_e105600_d_n9, assign69040_e105600_d_n10, assign69040_e105600_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign69040_e105597: f64 = (locals.var_egidl + 1e-25);
        let assign69040_e105598: f64 = (1.0 / assign69040_e105597);
        (assign69040_e105598, (-(locals.var_egidl_dn0 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn2 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn4 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn5 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn6 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn7 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn8 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn9 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn10 / (assign69040_e105597 * assign69040_e105597))), (-(locals.var_egidl_dn13 / (assign69040_e105597 * assign69040_e105597))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign69040_e105600;
        locals.var_t3_dn0 = assign69040_e105600_d_n0;
        locals.var_t3_dn2 = assign69040_e105600_d_n2;
        locals.var_t3_dn4 = assign69040_e105600_d_n4;
        locals.var_t3_dn5 = assign69040_e105600_d_n5;
        locals.var_t3_dn6 = assign69040_e105600_d_n6;
        locals.var_t3_dn7 = assign69040_e105600_d_n7;
        locals.var_t3_dn8 = assign69040_e105600_d_n8;
        locals.var_t3_dn9 = assign69040_e105600_d_n9;
        locals.var_t3_dn10 = assign69040_e105600_d_n10;
        locals.var_t3_dn13 = assign69040_e105600_d_n13;

        let (assign69050_e105610, assign69050_e105610_d_n0, assign69050_e105610_d_n2, assign69050_e105610_d_n4, assign69050_e105610_d_n5, assign69050_e105610_d_n6, assign69050_e105610_d_n7, assign69050_e105610_d_n8, assign69050_e105610_d_n9, assign69050_e105610_d_n10, assign69050_e105610_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign69050_e105604: f64 = (-locals.var_uc_gidl2);
        let assign69050_e105606: f64 = (assign69050_e105604 * locals.var_egp32);
        let assign69050_e105608: f64 = (assign69050_e105606 * locals.var_t3);
        (assign69050_e105608, (((assign69050_e105604 * locals.var_egp32_dn0) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn0)), (((assign69050_e105604 * locals.var_egp32_dn2) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn2)), (((assign69050_e105604 * locals.var_egp32_dn4) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn4)), (((assign69050_e105604 * locals.var_egp32_dn5) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn5)), (((assign69050_e105604 * locals.var_egp32_dn6) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn6)), (((assign69050_e105604 * locals.var_egp32_dn7) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn7)), (((assign69050_e105604 * locals.var_egp32_dn8) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn8)), (((assign69050_e105604 * locals.var_egp32_dn9) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn9)), (((assign69050_e105604 * locals.var_egp32_dn10) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn10)), (((assign69050_e105604 * locals.var_egp32_dn13) * locals.var_t3) + (assign69050_e105606 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign69050_e105610;
        locals.var_t0_dn0 = assign69050_e105610_d_n0;
        locals.var_t0_dn2 = assign69050_e105610_d_n2;
        locals.var_t0_dn4 = assign69050_e105610_d_n4;
        locals.var_t0_dn5 = assign69050_e105610_d_n5;
        locals.var_t0_dn6 = assign69050_e105610_d_n6;
        locals.var_t0_dn7 = assign69050_e105610_d_n7;
        locals.var_t0_dn8 = assign69050_e105610_d_n8;
        locals.var_t0_dn9 = assign69050_e105610_d_n9;
        locals.var_t0_dn10 = assign69050_e105610_d_n10;
        locals.var_t0_dn13 = assign69050_e105610_d_n13;

        let assign69060_e105613: f64 = (-34.0);
        let assign69060_e105614: f64 = if locals.var_t0 < assign69060_e105613 { 1.0 } else { 0.0 };
        locals.var_guard1617 = assign69060_e105614;

        let (assign69070_e105621, assign69070_e105621_d_n0, assign69070_e105621_d_n2, assign69070_e105621_d_n4, assign69070_e105621_d_n5, assign69070_e105621_d_n6, assign69070_e105621_d_n7, assign69070_e105621_d_n8, assign69070_e105621_d_n9, assign69070_e105621_d_n10, assign69070_e105621_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1617 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn13,)
    }
};
        locals.var_igidl = assign69070_e105621;
        locals.var_igidl_dn0 = assign69070_e105621_d_n0;
        locals.var_igidl_dn2 = assign69070_e105621_d_n2;
        locals.var_igidl_dn4 = assign69070_e105621_d_n4;
        locals.var_igidl_dn5 = assign69070_e105621_d_n5;
        locals.var_igidl_dn6 = assign69070_e105621_d_n6;
        locals.var_igidl_dn7 = assign69070_e105621_d_n7;
        locals.var_igidl_dn8 = assign69070_e105621_d_n8;
        locals.var_igidl_dn9 = assign69070_e105621_d_n9;
        locals.var_igidl_dn10 = assign69070_e105621_d_n10;
        locals.var_igidl_dn13 = assign69070_e105621_d_n13;

        let (assign69080_e105630, assign69080_e105630_d_n0, assign69080_e105630_d_n2, assign69080_e105630_d_n4, assign69080_e105630_d_n5, assign69080_e105630_d_n6, assign69080_e105630_d_n7, assign69080_e105630_d_n8, assign69080_e105630_d_n9, assign69080_e105630_d_n10, assign69080_e105630_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1617 == 0.0)) {
        let assign69080_e105628: f64 = (locals.var_t0).exp();
        (assign69080_e105628, (assign69080_e105628 * locals.var_t0_dn0), (assign69080_e105628 * locals.var_t0_dn2), (assign69080_e105628 * locals.var_t0_dn4), (assign69080_e105628 * locals.var_t0_dn5), (assign69080_e105628 * locals.var_t0_dn6), (assign69080_e105628 * locals.var_t0_dn7), (assign69080_e105628 * locals.var_t0_dn8), (assign69080_e105628 * locals.var_t0_dn9), (assign69080_e105628 * locals.var_t0_dn10), (assign69080_e105628 * locals.var_t0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign69080_e105630;
        locals.var_t1_dn0 = assign69080_e105630_d_n0;
        locals.var_t1_dn2 = assign69080_e105630_d_n2;
        locals.var_t1_dn4 = assign69080_e105630_d_n4;
        locals.var_t1_dn5 = assign69080_e105630_d_n5;
        locals.var_t1_dn6 = assign69080_e105630_d_n6;
        locals.var_t1_dn7 = assign69080_e105630_d_n7;
        locals.var_t1_dn8 = assign69080_e105630_d_n8;
        locals.var_t1_dn9 = assign69080_e105630_d_n9;
        locals.var_t1_dn10 = assign69080_e105630_d_n10;
        locals.var_t1_dn13 = assign69080_e105630_d_n13;

        let (assign69090_e105644, assign69090_e105644_d_n0, assign69090_e105644_d_n2, assign69090_e105644_d_n4, assign69090_e105644_d_n5, assign69090_e105644_d_n6, assign69090_e105644_d_n7, assign69090_e105644_d_n8, assign69090_e105644_d_n9, assign69090_e105644_d_n10, assign69090_e105644_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1617 == 0.0)) {
        let assign69090_e105638: f64 = (locals.var_uc_gidl1 / locals.var_egp12);
        let assign69090_e105640: f64 = (assign69090_e105638 * 1.6021918e-19);
        let assign69090_e105642: f64 = (assign69090_e105640 * locals.var_weff_nf);
        (assign69090_e105642, (((-((locals.var_uc_gidl1 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn9) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((locals.var_uc_gidl1 * locals.var_egp12_dn13) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69090_e105644;
        locals.var_t2_dn0 = assign69090_e105644_d_n0;
        locals.var_t2_dn2 = assign69090_e105644_d_n2;
        locals.var_t2_dn4 = assign69090_e105644_d_n4;
        locals.var_t2_dn5 = assign69090_e105644_d_n5;
        locals.var_t2_dn6 = assign69090_e105644_d_n6;
        locals.var_t2_dn7 = assign69090_e105644_d_n7;
        locals.var_t2_dn8 = assign69090_e105644_d_n8;
        locals.var_t2_dn9 = assign69090_e105644_d_n9;
        locals.var_t2_dn10 = assign69090_e105644_d_n10;
        locals.var_t2_dn13 = assign69090_e105644_d_n13;

        let (assign69100_e105658, assign69100_e105658_d_n0, assign69100_e105658_d_n2, assign69100_e105658_d_n4, assign69100_e105658_d_n5, assign69100_e105658_d_n6, assign69100_e105658_d_n7, assign69100_e105658_d_n8, assign69100_e105658_d_n9, assign69100_e105658_d_n10, assign69100_e105658_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1617 == 0.0)) {
        let assign69100_e105652: f64 = (locals.var_t2 * locals.var_egidl);
        let assign69100_e105654: f64 = (assign69100_e105652 * locals.var_egidl);
        let assign69100_e105656: f64 = (assign69100_e105654 * locals.var_t1);
        (assign69100_e105656, ((((((locals.var_t2_dn0 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn0)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn0)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn0)), ((((((locals.var_t2_dn2 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn2)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn2)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn2)), ((((((locals.var_t2_dn4 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn4)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn4)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn4)), ((((((locals.var_t2_dn5 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn5)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn5)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn5)), ((((((locals.var_t2_dn6 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn6)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn6)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn6)), ((((((locals.var_t2_dn7 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn7)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn7)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn7)), ((((((locals.var_t2_dn8 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn8)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn8)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn8)), ((((((locals.var_t2_dn9 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn9)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn9)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn9)), ((((((locals.var_t2_dn10 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn10)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn10)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn10)), ((((((locals.var_t2_dn13 * locals.var_egidl) + (locals.var_t2 * locals.var_egidl_dn13)) * locals.var_egidl) + (assign69100_e105652 * locals.var_egidl_dn13)) * locals.var_t1) + (assign69100_e105654 * locals.var_t1_dn13)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn13,)
    }
};
        locals.var_igidl = assign69100_e105658;
        locals.var_igidl_dn0 = assign69100_e105658_d_n0;
        locals.var_igidl_dn2 = assign69100_e105658_d_n2;
        locals.var_igidl_dn4 = assign69100_e105658_d_n4;
        locals.var_igidl_dn5 = assign69100_e105658_d_n5;
        locals.var_igidl_dn6 = assign69100_e105658_d_n6;
        locals.var_igidl_dn7 = assign69100_e105658_d_n7;
        locals.var_igidl_dn8 = assign69100_e105658_d_n8;
        locals.var_igidl_dn9 = assign69100_e105658_d_n9;
        locals.var_igidl_dn10 = assign69100_e105658_d_n10;
        locals.var_igidl_dn13 = assign69100_e105658_d_n13;

        let (assign69110_e105665, assign69110_e105665_d_n0, assign69110_e105665_d_n2, assign69110_e105665_d_n4, assign69110_e105665_d_n5, assign69110_e105665_d_n6, assign69110_e105665_d_n7, assign69110_e105665_d_n8, assign69110_e105665_d_n9, assign69110_e105665_d_n10, assign69110_e105665_d_n13,) = {
    if (locals.var_guard1615 == 0.0) {
        let assign69110_e105663: f64 = (locals.var_vds - locals.var_vbs);
        (assign69110_e105663, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, (locals.var_vds_dn5 - locals.var_vbs_dn5), locals.var_vds_dn6, (locals.var_vds_dn7 - locals.var_vbs_dn7), (locals.var_vds_dn8 - locals.var_vbs_dn8), locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    } else {
        (locals.var_vdb, locals.var_vdb_dn0, locals.var_vdb_dn2, locals.var_vdb_dn4, locals.var_vdb_dn5, locals.var_vdb_dn6, locals.var_vdb_dn7, locals.var_vdb_dn8, locals.var_vdb_dn9, locals.var_vdb_dn10, locals.var_vdb_dn13,)
    }
};
        locals.var_vdb = assign69110_e105665;
        locals.var_vdb_dn0 = assign69110_e105665_d_n0;
        locals.var_vdb_dn2 = assign69110_e105665_d_n2;
        locals.var_vdb_dn4 = assign69110_e105665_d_n4;
        locals.var_vdb_dn5 = assign69110_e105665_d_n5;
        locals.var_vdb_dn6 = assign69110_e105665_d_n6;
        locals.var_vdb_dn7 = assign69110_e105665_d_n7;
        locals.var_vdb_dn8 = assign69110_e105665_d_n8;
        locals.var_vdb_dn9 = assign69110_e105665_d_n9;
        locals.var_vdb_dn10 = assign69110_e105665_d_n10;
        locals.var_vdb_dn13 = assign69110_e105665_d_n13;

        let assign69120_e105668: f64 = if locals.var_vdb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1618 = assign69120_e105668;

        let (assign69130_e105677, assign69130_e105677_d_n0, assign69130_e105677_d_n2, assign69130_e105677_d_n4, assign69130_e105677_d_n5, assign69130_e105677_d_n6, assign69130_e105677_d_n7, assign69130_e105677_d_n8, assign69130_e105677_d_n9, assign69130_e105677_d_n10, assign69130_e105677_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69130_e105675: f64 = (locals.var_vdb * locals.var_vdb);
        (assign69130_e105675, ((locals.var_vdb_dn0 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn0)), ((locals.var_vdb_dn2 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn2)), ((locals.var_vdb_dn4 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn4)), ((locals.var_vdb_dn5 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn5)), ((locals.var_vdb_dn6 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn6)), ((locals.var_vdb_dn7 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn7)), ((locals.var_vdb_dn8 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn8)), ((locals.var_vdb_dn9 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn9)), ((locals.var_vdb_dn10 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn10)), ((locals.var_vdb_dn13 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69130_e105677;
        locals.var_t2_dn0 = assign69130_e105677_d_n0;
        locals.var_t2_dn2 = assign69130_e105677_d_n2;
        locals.var_t2_dn4 = assign69130_e105677_d_n4;
        locals.var_t2_dn5 = assign69130_e105677_d_n5;
        locals.var_t2_dn6 = assign69130_e105677_d_n6;
        locals.var_t2_dn7 = assign69130_e105677_d_n7;
        locals.var_t2_dn8 = assign69130_e105677_d_n8;
        locals.var_t2_dn9 = assign69130_e105677_d_n9;
        locals.var_t2_dn10 = assign69130_e105677_d_n10;
        locals.var_t2_dn13 = assign69130_e105677_d_n13;

        let (assign69140_e105686, assign69140_e105686_d_n0, assign69140_e105686_d_n2, assign69140_e105686_d_n4, assign69140_e105686_d_n5, assign69140_e105686_d_n6, assign69140_e105686_d_n7, assign69140_e105686_d_n8, assign69140_e105686_d_n9, assign69140_e105686_d_n10, assign69140_e105686_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69140_e105684: f64 = (locals.var_t2 * locals.var_vdb);
        (assign69140_e105684, ((locals.var_t2_dn0 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn0)), ((locals.var_t2_dn2 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn2)), ((locals.var_t2_dn4 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn4)), ((locals.var_t2_dn5 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn5)), ((locals.var_t2_dn6 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn6)), ((locals.var_t2_dn7 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn7)), ((locals.var_t2_dn8 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn8)), ((locals.var_t2_dn9 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn9)), ((locals.var_t2_dn10 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn10)), ((locals.var_t2_dn13 * locals.var_vdb) + (locals.var_t2 * locals.var_vdb_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign69140_e105686;
        locals.var_t4_dn0 = assign69140_e105686_d_n0;
        locals.var_t4_dn2 = assign69140_e105686_d_n2;
        locals.var_t4_dn4 = assign69140_e105686_d_n4;
        locals.var_t4_dn5 = assign69140_e105686_d_n5;
        locals.var_t4_dn6 = assign69140_e105686_d_n6;
        locals.var_t4_dn7 = assign69140_e105686_d_n7;
        locals.var_t4_dn8 = assign69140_e105686_d_n8;
        locals.var_t4_dn9 = assign69140_e105686_d_n9;
        locals.var_t4_dn10 = assign69140_e105686_d_n10;
        locals.var_t4_dn13 = assign69140_e105686_d_n13;

        let (assign69150_e105695, assign69150_e105695_d_n0, assign69150_e105695_d_n2, assign69150_e105695_d_n4, assign69150_e105695_d_n5, assign69150_e105695_d_n6, assign69150_e105695_d_n7, assign69150_e105695_d_n8, assign69150_e105695_d_n9, assign69150_e105695_d_n10, assign69150_e105695_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69150_e105693: f64 = (locals.var_t4 + 0.5);
        (assign69150_e105693, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign69150_e105695;
        locals.var_t0_dn0 = assign69150_e105695_d_n0;
        locals.var_t0_dn2 = assign69150_e105695_d_n2;
        locals.var_t0_dn4 = assign69150_e105695_d_n4;
        locals.var_t0_dn5 = assign69150_e105695_d_n5;
        locals.var_t0_dn6 = assign69150_e105695_d_n6;
        locals.var_t0_dn7 = assign69150_e105695_d_n7;
        locals.var_t0_dn8 = assign69150_e105695_d_n8;
        locals.var_t0_dn9 = assign69150_e105695_d_n9;
        locals.var_t0_dn10 = assign69150_e105695_d_n10;
        locals.var_t0_dn13 = assign69150_e105695_d_n13;

        let (assign69160_e105704, assign69160_e105704_d_n0, assign69160_e105704_d_n2, assign69160_e105704_d_n4, assign69160_e105704_d_n5, assign69160_e105704_d_n6, assign69160_e105704_d_n7, assign69160_e105704_d_n8, assign69160_e105704_d_n9, assign69160_e105704_d_n10, assign69160_e105704_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69160_e105702: f64 = (locals.var_t4 / locals.var_t0);
        (assign69160_e105702, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn13 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69160_e105704;
        locals.var_t5_dn0 = assign69160_e105704_d_n0;
        locals.var_t5_dn2 = assign69160_e105704_d_n2;
        locals.var_t5_dn4 = assign69160_e105704_d_n4;
        locals.var_t5_dn5 = assign69160_e105704_d_n5;
        locals.var_t5_dn6 = assign69160_e105704_d_n6;
        locals.var_t5_dn7 = assign69160_e105704_d_n7;
        locals.var_t5_dn8 = assign69160_e105704_d_n8;
        locals.var_t5_dn9 = assign69160_e105704_d_n9;
        locals.var_t5_dn10 = assign69160_e105704_d_n10;
        locals.var_t5_dn13 = assign69160_e105704_d_n13;

        let (assign69170_e105725, assign69170_e105725_d_n0, assign69170_e105725_d_n2, assign69170_e105725_d_n4, assign69170_e105725_d_n5, assign69170_e105725_d_n6, assign69170_e105725_d_n7, assign69170_e105725_d_n8, assign69170_e105725_d_n9, assign69170_e105725_d_n10, assign69170_e105725_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69170_e105711: f64 = (3.0 * locals.var_t2);
        let assign69170_e105713: f64 = (assign69170_e105711 * locals.var_t0);
        let assign69170_e105716: f64 = (locals.var_t4 * 3.0);
        let assign69170_e105718: f64 = (assign69170_e105716 * locals.var_t2);
        let assign69170_e105719: f64 = (assign69170_e105713 - assign69170_e105718);
        let assign69170_e105722: f64 = (locals.var_t0 * locals.var_t0);
        let assign69170_e105723: f64 = (assign69170_e105719 / assign69170_e105722);
        (assign69170_e105723, (((((((3.0 * locals.var_t2_dn0) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn0)) - (((locals.var_t4_dn0 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn0))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn2) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn2)) - (((locals.var_t4_dn2 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn2))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn4) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn4)) - (((locals.var_t4_dn4 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn4))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn5) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn5)) - (((locals.var_t4_dn5 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn5))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn6) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn6)) - (((locals.var_t4_dn6 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn6))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn7) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn7)) - (((locals.var_t4_dn7 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn7))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn8) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn8)) - (((locals.var_t4_dn8 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn8))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn9) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn9)) - (((locals.var_t4_dn9 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn9))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn10) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn10)) - (((locals.var_t4_dn10 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn10))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)))) / (assign69170_e105722 * assign69170_e105722)), (((((((3.0 * locals.var_t2_dn13) * locals.var_t0) + (assign69170_e105711 * locals.var_t0_dn13)) - (((locals.var_t4_dn13 * 3.0) * locals.var_t2) + (assign69170_e105716 * locals.var_t2_dn13))) * assign69170_e105722) - (assign69170_e105719 * ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)))) / (assign69170_e105722 * assign69170_e105722)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign69170_e105725;
        locals.var_t7_dn0 = assign69170_e105725_d_n0;
        locals.var_t7_dn2 = assign69170_e105725_d_n2;
        locals.var_t7_dn4 = assign69170_e105725_d_n4;
        locals.var_t7_dn5 = assign69170_e105725_d_n5;
        locals.var_t7_dn6 = assign69170_e105725_d_n6;
        locals.var_t7_dn7 = assign69170_e105725_d_n7;
        locals.var_t7_dn8 = assign69170_e105725_d_n8;
        locals.var_t7_dn9 = assign69170_e105725_d_n9;
        locals.var_t7_dn10 = assign69170_e105725_d_n10;
        locals.var_t7_dn13 = assign69170_e105725_d_n13;

        let (assign69180_e105734, assign69180_e105734_d_n0, assign69180_e105734_d_n2, assign69180_e105734_d_n4, assign69180_e105734_d_n5, assign69180_e105734_d_n6, assign69180_e105734_d_n7, assign69180_e105734_d_n8, assign69180_e105734_d_n9, assign69180_e105734_d_n10, assign69180_e105734_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 != 0.0)) {
        let assign69180_e105732: f64 = (locals.var_igidl * locals.var_t5);
        (assign69180_e105732, ((locals.var_igidl_dn0 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn0)), ((locals.var_igidl_dn2 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn2)), ((locals.var_igidl_dn4 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn4)), ((locals.var_igidl_dn5 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn5)), ((locals.var_igidl_dn6 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn6)), ((locals.var_igidl_dn7 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn7)), ((locals.var_igidl_dn8 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn8)), ((locals.var_igidl_dn9 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn9)), ((locals.var_igidl_dn10 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn10)), ((locals.var_igidl_dn13 * locals.var_t5) + (locals.var_igidl * locals.var_t5_dn13)),)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn13,)
    }
};
        locals.var_igidl = assign69180_e105734;
        locals.var_igidl_dn0 = assign69180_e105734_d_n0;
        locals.var_igidl_dn2 = assign69180_e105734_d_n2;
        locals.var_igidl_dn4 = assign69180_e105734_d_n4;
        locals.var_igidl_dn5 = assign69180_e105734_d_n5;
        locals.var_igidl_dn6 = assign69180_e105734_d_n6;
        locals.var_igidl_dn7 = assign69180_e105734_d_n7;
        locals.var_igidl_dn8 = assign69180_e105734_d_n8;
        locals.var_igidl_dn9 = assign69180_e105734_d_n9;
        locals.var_igidl_dn10 = assign69180_e105734_d_n10;
        locals.var_igidl_dn13 = assign69180_e105734_d_n13;

        let (assign69190_e105742, assign69190_e105742_d_n0, assign69190_e105742_d_n2, assign69190_e105742_d_n4, assign69190_e105742_d_n5, assign69190_e105742_d_n6, assign69190_e105742_d_n7, assign69190_e105742_d_n8, assign69190_e105742_d_n9, assign69190_e105742_d_n10, assign69190_e105742_d_n13,) = {
    if ((locals.var_guard1615 == 0.0) && (locals.var_guard1618 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl, locals.var_igidl_dn0, locals.var_igidl_dn2, locals.var_igidl_dn4, locals.var_igidl_dn5, locals.var_igidl_dn6, locals.var_igidl_dn7, locals.var_igidl_dn8, locals.var_igidl_dn9, locals.var_igidl_dn10, locals.var_igidl_dn13,)
    }
};
        locals.var_igidl = assign69190_e105742;
        locals.var_igidl_dn0 = assign69190_e105742_d_n0;
        locals.var_igidl_dn2 = assign69190_e105742_d_n2;
        locals.var_igidl_dn4 = assign69190_e105742_d_n4;
        locals.var_igidl_dn5 = assign69190_e105742_d_n5;
        locals.var_igidl_dn6 = assign69190_e105742_d_n6;
        locals.var_igidl_dn7 = assign69190_e105742_d_n7;
        locals.var_igidl_dn8 = assign69190_e105742_d_n8;
        locals.var_igidl_dn9 = assign69190_e105742_d_n9;
        locals.var_igidl_dn10 = assign69190_e105742_d_n10;
        locals.var_igidl_dn13 = assign69190_e105742_d_n13;

        let assign69200_e105745: f64 = if p.p25 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1619 = assign69200_e105745;

        let (assign69210_e105749, assign69210_e105749_d_n0, assign69210_e105749_d_n2, assign69210_e105749_d_n4, assign69210_e105749_d_n5, assign69210_e105749_d_n6, assign69210_e105749_d_n7, assign69210_e105749_d_n8, assign69210_e105749_d_n9, assign69210_e105749_d_n10, assign69210_e105749_d_n13,) = {
    if (locals.var_guard1619 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn13,)
    }
};
        locals.var_igisl = assign69210_e105749;
        locals.var_igisl_dn0 = assign69210_e105749_d_n0;
        locals.var_igisl_dn2 = assign69210_e105749_d_n2;
        locals.var_igisl_dn4 = assign69210_e105749_d_n4;
        locals.var_igisl_dn5 = assign69210_e105749_d_n5;
        locals.var_igisl_dn6 = assign69210_e105749_d_n6;
        locals.var_igisl_dn7 = assign69210_e105749_d_n7;
        locals.var_igisl_dn8 = assign69210_e105749_d_n8;
        locals.var_igisl_dn9 = assign69210_e105749_d_n9;
        locals.var_igisl_dn10 = assign69210_e105749_d_n10;
        locals.var_igisl_dn13 = assign69210_e105749_d_n13;

        let (assign69220_e105769, assign69220_e105769_d_n0, assign69220_e105769_d_n2, assign69220_e105769_d_n4, assign69220_e105769_d_n5, assign69220_e105769_d_n6, assign69220_e105769_d_n7, assign69220_e105769_d_n8, assign69220_e105769_d_n9, assign69220_e105769_d_n10, assign69220_e105769_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69220_e105754: f64 = (-locals.var_vdsp);
        let assign69220_e105756: f64 = (assign69220_e105754 + p.p243);
        let assign69220_e105757: f64 = (p.p242 * assign69220_e105756);
        let assign69220_e105760: f64 = (locals.var_vgs - locals.var_vdsp);
        let assign69220_e105761: f64 = (assign69220_e105757 - assign69220_e105760);
        let assign69220_e105764: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign69220_e105766: f64 = (assign69220_e105764 * p.p244);
        let assign69220_e105767: f64 = (assign69220_e105761 + assign69220_e105766);
        (assign69220_e105767, (((p.p242 * (-locals.var_vdsp_dn0)) - (-locals.var_vdsp_dn0)) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn2)) - (-locals.var_vdsp_dn2)) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn4)) - (-locals.var_vdsp_dn4)) + ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn5)) - (locals.var_vgs_dn5 - locals.var_vdsp_dn5)) + ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn6)) - (locals.var_vgs_dn6 - locals.var_vdsp_dn6)) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn7)) - (locals.var_vgs_dn7 - locals.var_vdsp_dn7)) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn8)) - (-locals.var_vdsp_dn8)) + ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn9)) - (-locals.var_vdsp_dn9)) + ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn10)) - (-locals.var_vdsp_dn10)) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p244)), (((p.p242 * (-locals.var_vdsp_dn13)) - (-locals.var_vdsp_dn13)) + ((locals.var_dvthsc_dn13 + locals.var_dvthlp_dn13) * p.p244)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign69220_e105769;
        locals.var_t1_dn0 = assign69220_e105769_d_n0;
        locals.var_t1_dn2 = assign69220_e105769_d_n2;
        locals.var_t1_dn4 = assign69220_e105769_d_n4;
        locals.var_t1_dn5 = assign69220_e105769_d_n5;
        locals.var_t1_dn6 = assign69220_e105769_d_n6;
        locals.var_t1_dn7 = assign69220_e105769_d_n7;
        locals.var_t1_dn8 = assign69220_e105769_d_n8;
        locals.var_t1_dn9 = assign69220_e105769_d_n9;
        locals.var_t1_dn10 = assign69220_e105769_d_n10;
        locals.var_t1_dn13 = assign69220_e105769_d_n13;

        let (assign69230_e105776, assign69230_e105776_d_n0, assign69230_e105776_d_n2, assign69230_e105776_d_n4, assign69230_e105776_d_n5, assign69230_e105776_d_n6, assign69230_e105776_d_n7, assign69230_e105776_d_n8, assign69230_e105776_d_n9, assign69230_e105776_d_n10, assign69230_e105776_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69230_e105774: f64 = (1.0 / locals.var_tox0);
        (assign69230_e105774, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69230_e105776;
        locals.var_t2_dn0 = assign69230_e105776_d_n0;
        locals.var_t2_dn2 = assign69230_e105776_d_n2;
        locals.var_t2_dn4 = assign69230_e105776_d_n4;
        locals.var_t2_dn5 = assign69230_e105776_d_n5;
        locals.var_t2_dn6 = assign69230_e105776_d_n6;
        locals.var_t2_dn7 = assign69230_e105776_d_n7;
        locals.var_t2_dn8 = assign69230_e105776_d_n8;
        locals.var_t2_dn9 = assign69230_e105776_d_n9;
        locals.var_t2_dn10 = assign69230_e105776_d_n10;
        locals.var_t2_dn13 = assign69230_e105776_d_n13;

        let (assign69240_e105783, assign69240_e105783_d_n0, assign69240_e105783_d_n2, assign69240_e105783_d_n4, assign69240_e105783_d_n5, assign69240_e105783_d_n6, assign69240_e105783_d_n7, assign69240_e105783_d_n8, assign69240_e105783_d_n9, assign69240_e105783_d_n10, assign69240_e105783_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69240_e105781: f64 = (locals.var_t1 * locals.var_t2);
        (assign69240_e105781, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn13,)
    }
};
        locals.var_e1 = assign69240_e105783;
        locals.var_e1_dn0 = assign69240_e105783_d_n0;
        locals.var_e1_dn2 = assign69240_e105783_d_n2;
        locals.var_e1_dn4 = assign69240_e105783_d_n4;
        locals.var_e1_dn5 = assign69240_e105783_d_n5;
        locals.var_e1_dn6 = assign69240_e105783_d_n6;
        locals.var_e1_dn7 = assign69240_e105783_d_n7;
        locals.var_e1_dn8 = assign69240_e105783_d_n8;
        locals.var_e1_dn9 = assign69240_e105783_d_n9;
        locals.var_e1_dn10 = assign69240_e105783_d_n10;
        locals.var_e1_dn13 = assign69240_e105783_d_n13;

        let (assign69250_e105801, assign69250_e105801_d_n0, assign69250_e105801_d_n2, assign69250_e105801_d_n4, assign69250_e105801_d_n5, assign69250_e105801_d_n6, assign69250_e105801_d_n7, assign69250_e105801_d_n8, assign69250_e105801_d_n9, assign69250_e105801_d_n10, assign69250_e105801_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69250_e105788: f64 = (locals.var_e1 * locals.var_e1);
        let assign69250_e105792: f64 = (0.01 / 0.01);
        let assign69250_e105793: f64 = (4.0 * assign69250_e105792);
        let assign69250_e105796: f64 = (0.01 / 0.01);
        let assign69250_e105797: f64 = (assign69250_e105793 * assign69250_e105796);
        let assign69250_e105798: f64 = (assign69250_e105788 + assign69250_e105797);
        let assign69250_e105799: f64 = (assign69250_e105798).sqrt();
        (assign69250_e105799, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn4 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn4)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn5 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn5)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn8 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn8)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn9 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn9)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign69250_e105799)), (((locals.var_e1_dn13 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn13)) / (2.0 * assign69250_e105799)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign69250_e105801;
        locals.var_tmf2_dn0 = assign69250_e105801_d_n0;
        locals.var_tmf2_dn2 = assign69250_e105801_d_n2;
        locals.var_tmf2_dn4 = assign69250_e105801_d_n4;
        locals.var_tmf2_dn5 = assign69250_e105801_d_n5;
        locals.var_tmf2_dn6 = assign69250_e105801_d_n6;
        locals.var_tmf2_dn7 = assign69250_e105801_d_n7;
        locals.var_tmf2_dn8 = assign69250_e105801_d_n8;
        locals.var_tmf2_dn9 = assign69250_e105801_d_n9;
        locals.var_tmf2_dn10 = assign69250_e105801_d_n10;
        locals.var_tmf2_dn13 = assign69250_e105801_d_n13;

        let (assign69260_e105812, assign69260_e105812_d_n0, assign69260_e105812_d_n2, assign69260_e105812_d_n4, assign69260_e105812_d_n5, assign69260_e105812_d_n6, assign69260_e105812_d_n7, assign69260_e105812_d_n8, assign69260_e105812_d_n9, assign69260_e105812_d_n10, assign69260_e105812_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69260_e105808: f64 = (locals.var_e1 / locals.var_tmf2);
        let assign69260_e105809: f64 = (1.0 + assign69260_e105808);
        let assign69260_e105810: f64 = (0.5 * assign69260_e105809);
        (assign69260_e105810, (0.5 * (((locals.var_e1_dn0 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn2 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn4 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn5 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn6 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn7 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn8 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn9 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn10 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_e1_dn13 * locals.var_tmf2) - (locals.var_e1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69260_e105812;
        locals.var_t5_dn0 = assign69260_e105812_d_n0;
        locals.var_t5_dn2 = assign69260_e105812_d_n2;
        locals.var_t5_dn4 = assign69260_e105812_d_n4;
        locals.var_t5_dn5 = assign69260_e105812_d_n5;
        locals.var_t5_dn6 = assign69260_e105812_d_n6;
        locals.var_t5_dn7 = assign69260_e105812_d_n7;
        locals.var_t5_dn8 = assign69260_e105812_d_n8;
        locals.var_t5_dn9 = assign69260_e105812_d_n9;
        locals.var_t5_dn10 = assign69260_e105812_d_n10;
        locals.var_t5_dn13 = assign69260_e105812_d_n13;

        let (assign69270_e105821, assign69270_e105821_d_n0, assign69270_e105821_d_n2, assign69270_e105821_d_n4, assign69270_e105821_d_n5, assign69270_e105821_d_n6, assign69270_e105821_d_n7, assign69270_e105821_d_n8, assign69270_e105821_d_n9, assign69270_e105821_d_n10, assign69270_e105821_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69270_e105818: f64 = (locals.var_e1 + locals.var_tmf2);
        let assign69270_e105819: f64 = (0.5 * assign69270_e105818);
        (assign69270_e105819, (0.5 * (locals.var_e1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_e1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_e1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_e1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_e1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_e1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn8, locals.var_egisl_dn9, locals.var_egisl_dn10, locals.var_egisl_dn13,)
    }
};
        locals.var_egisl = assign69270_e105821;
        locals.var_egisl_dn0 = assign69270_e105821_d_n0;
        locals.var_egisl_dn2 = assign69270_e105821_d_n2;
        locals.var_egisl_dn4 = assign69270_e105821_d_n4;
        locals.var_egisl_dn5 = assign69270_e105821_d_n5;
        locals.var_egisl_dn6 = assign69270_e105821_d_n6;
        locals.var_egisl_dn7 = assign69270_e105821_d_n7;
        locals.var_egisl_dn8 = assign69270_e105821_d_n8;
        locals.var_egisl_dn9 = assign69270_e105821_d_n9;
        locals.var_egisl_dn10 = assign69270_e105821_d_n10;
        locals.var_egisl_dn13 = assign69270_e105821_d_n13;

        let assign69280_e105824: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1620 = assign69280_e105824;

    }

    pub(super) fn stamp_transient_block_237(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69290_e105831, assign69290_e105831_d_n0, assign69290_e105831_d_n2, assign69290_e105831_d_n4, assign69290_e105831_d_n5, assign69290_e105831_d_n6, assign69290_e105831_d_n7, assign69290_e105831_d_n8, assign69290_e105831_d_n9, assign69290_e105831_d_n10, assign69290_e105831_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1620 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn4, locals.var_egisl_dn5, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn8, locals.var_egisl_dn9, locals.var_egisl_dn10, locals.var_egisl_dn13,)
    }
};
        locals.var_egisl = assign69290_e105831;
        locals.var_egisl_dn0 = assign69290_e105831_d_n0;
        locals.var_egisl_dn2 = assign69290_e105831_d_n2;
        locals.var_egisl_dn4 = assign69290_e105831_d_n4;
        locals.var_egisl_dn5 = assign69290_e105831_d_n5;
        locals.var_egisl_dn6 = assign69290_e105831_d_n6;
        locals.var_egisl_dn7 = assign69290_e105831_d_n7;
        locals.var_egisl_dn8 = assign69290_e105831_d_n8;
        locals.var_egisl_dn9 = assign69290_e105831_d_n9;
        locals.var_egisl_dn10 = assign69290_e105831_d_n10;
        locals.var_egisl_dn13 = assign69290_e105831_d_n13;

        let (assign69300_e105838, assign69300_e105838_d_n0, assign69300_e105838_d_n2, assign69300_e105838_d_n4, assign69300_e105838_d_n5, assign69300_e105838_d_n6, assign69300_e105838_d_n7, assign69300_e105838_d_n8, assign69300_e105838_d_n9, assign69300_e105838_d_n10, assign69300_e105838_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1620 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69300_e105838;
        locals.var_t5_dn0 = assign69300_e105838_d_n0;
        locals.var_t5_dn2 = assign69300_e105838_d_n2;
        locals.var_t5_dn4 = assign69300_e105838_d_n4;
        locals.var_t5_dn5 = assign69300_e105838_d_n5;
        locals.var_t5_dn6 = assign69300_e105838_d_n6;
        locals.var_t5_dn7 = assign69300_e105838_d_n7;
        locals.var_t5_dn8 = assign69300_e105838_d_n8;
        locals.var_t5_dn9 = assign69300_e105838_d_n9;
        locals.var_t5_dn10 = assign69300_e105838_d_n10;
        locals.var_t5_dn13 = assign69300_e105838_d_n13;

        let (assign69310_e105847, assign69310_e105847_d_n0, assign69310_e105847_d_n2, assign69310_e105847_d_n4, assign69310_e105847_d_n5, assign69310_e105847_d_n6, assign69310_e105847_d_n7, assign69310_e105847_d_n8, assign69310_e105847_d_n9, assign69310_e105847_d_n10, assign69310_e105847_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69310_e105844: f64 = (locals.var_egisl + 1e-25);
        let assign69310_e105845: f64 = (1.0 / assign69310_e105844);
        (assign69310_e105845, (-(locals.var_egisl_dn0 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn2 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn4 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn5 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn6 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn7 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn8 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn9 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn10 / (assign69310_e105844 * assign69310_e105844))), (-(locals.var_egisl_dn13 / (assign69310_e105844 * assign69310_e105844))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign69310_e105847;
        locals.var_t3_dn0 = assign69310_e105847_d_n0;
        locals.var_t3_dn2 = assign69310_e105847_d_n2;
        locals.var_t3_dn4 = assign69310_e105847_d_n4;
        locals.var_t3_dn5 = assign69310_e105847_d_n5;
        locals.var_t3_dn6 = assign69310_e105847_d_n6;
        locals.var_t3_dn7 = assign69310_e105847_d_n7;
        locals.var_t3_dn8 = assign69310_e105847_d_n8;
        locals.var_t3_dn9 = assign69310_e105847_d_n9;
        locals.var_t3_dn10 = assign69310_e105847_d_n10;
        locals.var_t3_dn13 = assign69310_e105847_d_n13;

        let (assign69320_e105857, assign69320_e105857_d_n0, assign69320_e105857_d_n2, assign69320_e105857_d_n4, assign69320_e105857_d_n5, assign69320_e105857_d_n6, assign69320_e105857_d_n7, assign69320_e105857_d_n8, assign69320_e105857_d_n9, assign69320_e105857_d_n10, assign69320_e105857_d_n13,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69320_e105851: f64 = (-locals.var_uc_gidl2);
        let assign69320_e105853: f64 = (assign69320_e105851 * locals.var_egp32);
        let assign69320_e105855: f64 = (assign69320_e105853 * locals.var_t3);
        (assign69320_e105855, (((assign69320_e105851 * locals.var_egp32_dn0) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn0)), (((assign69320_e105851 * locals.var_egp32_dn2) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn2)), (((assign69320_e105851 * locals.var_egp32_dn4) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn4)), (((assign69320_e105851 * locals.var_egp32_dn5) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn5)), (((assign69320_e105851 * locals.var_egp32_dn6) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn6)), (((assign69320_e105851 * locals.var_egp32_dn7) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn7)), (((assign69320_e105851 * locals.var_egp32_dn8) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn8)), (((assign69320_e105851 * locals.var_egp32_dn9) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn9)), (((assign69320_e105851 * locals.var_egp32_dn10) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn10)), (((assign69320_e105851 * locals.var_egp32_dn13) * locals.var_t3) + (assign69320_e105853 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign69320_e105857;
        locals.var_t0_dn0 = assign69320_e105857_d_n0;
        locals.var_t0_dn2 = assign69320_e105857_d_n2;
        locals.var_t0_dn4 = assign69320_e105857_d_n4;
        locals.var_t0_dn5 = assign69320_e105857_d_n5;
        locals.var_t0_dn6 = assign69320_e105857_d_n6;
        locals.var_t0_dn7 = assign69320_e105857_d_n7;
        locals.var_t0_dn8 = assign69320_e105857_d_n8;
        locals.var_t0_dn9 = assign69320_e105857_d_n9;
        locals.var_t0_dn10 = assign69320_e105857_d_n10;
        locals.var_t0_dn13 = assign69320_e105857_d_n13;

        let assign69330_e105860: f64 = (-34.0);
        let assign69330_e105861: f64 = if locals.var_t0 < assign69330_e105860 { 1.0 } else { 0.0 };
        locals.var_guard1621 = assign69330_e105861;

        let (assign69340_e105868, assign69340_e105868_d_n0, assign69340_e105868_d_n2, assign69340_e105868_d_n4, assign69340_e105868_d_n5, assign69340_e105868_d_n6, assign69340_e105868_d_n7, assign69340_e105868_d_n8, assign69340_e105868_d_n9, assign69340_e105868_d_n10, assign69340_e105868_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn13,)
    }
};
        locals.var_igisl = assign69340_e105868;
        locals.var_igisl_dn0 = assign69340_e105868_d_n0;
        locals.var_igisl_dn2 = assign69340_e105868_d_n2;
        locals.var_igisl_dn4 = assign69340_e105868_d_n4;
        locals.var_igisl_dn5 = assign69340_e105868_d_n5;
        locals.var_igisl_dn6 = assign69340_e105868_d_n6;
        locals.var_igisl_dn7 = assign69340_e105868_d_n7;
        locals.var_igisl_dn8 = assign69340_e105868_d_n8;
        locals.var_igisl_dn9 = assign69340_e105868_d_n9;
        locals.var_igisl_dn10 = assign69340_e105868_d_n10;
        locals.var_igisl_dn13 = assign69340_e105868_d_n13;

        let (assign69350_e105877, assign69350_e105877_d_n0, assign69350_e105877_d_n2, assign69350_e105877_d_n4, assign69350_e105877_d_n5, assign69350_e105877_d_n6, assign69350_e105877_d_n7, assign69350_e105877_d_n8, assign69350_e105877_d_n9, assign69350_e105877_d_n10, assign69350_e105877_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69350_e105875: f64 = (locals.var_t0).exp();
        (assign69350_e105875, (assign69350_e105875 * locals.var_t0_dn0), (assign69350_e105875 * locals.var_t0_dn2), (assign69350_e105875 * locals.var_t0_dn4), (assign69350_e105875 * locals.var_t0_dn5), (assign69350_e105875 * locals.var_t0_dn6), (assign69350_e105875 * locals.var_t0_dn7), (assign69350_e105875 * locals.var_t0_dn8), (assign69350_e105875 * locals.var_t0_dn9), (assign69350_e105875 * locals.var_t0_dn10), (assign69350_e105875 * locals.var_t0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign69350_e105877;
        locals.var_t1_dn0 = assign69350_e105877_d_n0;
        locals.var_t1_dn2 = assign69350_e105877_d_n2;
        locals.var_t1_dn4 = assign69350_e105877_d_n4;
        locals.var_t1_dn5 = assign69350_e105877_d_n5;
        locals.var_t1_dn6 = assign69350_e105877_d_n6;
        locals.var_t1_dn7 = assign69350_e105877_d_n7;
        locals.var_t1_dn8 = assign69350_e105877_d_n8;
        locals.var_t1_dn9 = assign69350_e105877_d_n9;
        locals.var_t1_dn10 = assign69350_e105877_d_n10;
        locals.var_t1_dn13 = assign69350_e105877_d_n13;

        let (assign69360_e105887, assign69360_e105887_d_n0, assign69360_e105887_d_n2, assign69360_e105887_d_n4, assign69360_e105887_d_n5, assign69360_e105887_d_n6, assign69360_e105887_d_n7, assign69360_e105887_d_n8, assign69360_e105887_d_n9, assign69360_e105887_d_n10, assign69360_e105887_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69360_e105885: f64 = (1.0 / locals.var_egp12);
        (assign69360_e105885, (-(locals.var_egp12_dn0 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn2 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn4 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn5 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn6 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn7 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn8 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn9 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn10 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn13 / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign69360_e105887;
        locals.var_t3_dn0 = assign69360_e105887_d_n0;
        locals.var_t3_dn2 = assign69360_e105887_d_n2;
        locals.var_t3_dn4 = assign69360_e105887_d_n4;
        locals.var_t3_dn5 = assign69360_e105887_d_n5;
        locals.var_t3_dn6 = assign69360_e105887_d_n6;
        locals.var_t3_dn7 = assign69360_e105887_d_n7;
        locals.var_t3_dn8 = assign69360_e105887_d_n8;
        locals.var_t3_dn9 = assign69360_e105887_d_n9;
        locals.var_t3_dn10 = assign69360_e105887_d_n10;
        locals.var_t3_dn13 = assign69360_e105887_d_n13;

        let (assign69370_e105901, assign69370_e105901_d_n0, assign69370_e105901_d_n2, assign69370_e105901_d_n4, assign69370_e105901_d_n5, assign69370_e105901_d_n6, assign69370_e105901_d_n7, assign69370_e105901_d_n8, assign69370_e105901_d_n9, assign69370_e105901_d_n10, assign69370_e105901_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69370_e105895: f64 = (locals.var_uc_gidl1 * locals.var_t3);
        let assign69370_e105897: f64 = (assign69370_e105895 * 1.6021918e-19);
        let assign69370_e105899: f64 = (assign69370_e105897 * locals.var_weff_nf);
        (assign69370_e105899, (((locals.var_uc_gidl1 * locals.var_t3_dn0) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn2) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn4) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn5) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn6) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn7) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn8) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn9) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn10) * 1.6021918e-19) * locals.var_weff_nf), (((locals.var_uc_gidl1 * locals.var_t3_dn13) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69370_e105901;
        locals.var_t2_dn0 = assign69370_e105901_d_n0;
        locals.var_t2_dn2 = assign69370_e105901_d_n2;
        locals.var_t2_dn4 = assign69370_e105901_d_n4;
        locals.var_t2_dn5 = assign69370_e105901_d_n5;
        locals.var_t2_dn6 = assign69370_e105901_d_n6;
        locals.var_t2_dn7 = assign69370_e105901_d_n7;
        locals.var_t2_dn8 = assign69370_e105901_d_n8;
        locals.var_t2_dn9 = assign69370_e105901_d_n9;
        locals.var_t2_dn10 = assign69370_e105901_d_n10;
        locals.var_t2_dn13 = assign69370_e105901_d_n13;

        let (assign69380_e105915, assign69380_e105915_d_n0, assign69380_e105915_d_n2, assign69380_e105915_d_n4, assign69380_e105915_d_n5, assign69380_e105915_d_n6, assign69380_e105915_d_n7, assign69380_e105915_d_n8, assign69380_e105915_d_n9, assign69380_e105915_d_n10, assign69380_e105915_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1621 == 0.0)) {
        let assign69380_e105909: f64 = (locals.var_t2 * locals.var_egisl);
        let assign69380_e105911: f64 = (assign69380_e105909 * locals.var_egisl);
        let assign69380_e105913: f64 = (assign69380_e105911 * locals.var_t1);
        (assign69380_e105913, ((((((locals.var_t2_dn0 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn0)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn0)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn0)), ((((((locals.var_t2_dn2 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn2)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn2)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn2)), ((((((locals.var_t2_dn4 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn4)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn4)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn4)), ((((((locals.var_t2_dn5 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn5)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn5)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn5)), ((((((locals.var_t2_dn6 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn6)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn6)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn6)), ((((((locals.var_t2_dn7 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn7)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn7)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn7)), ((((((locals.var_t2_dn8 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn8)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn8)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn8)), ((((((locals.var_t2_dn9 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn9)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn9)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn9)), ((((((locals.var_t2_dn10 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn10)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn10)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn10)), ((((((locals.var_t2_dn13 * locals.var_egisl) + (locals.var_t2 * locals.var_egisl_dn13)) * locals.var_egisl) + (assign69380_e105909 * locals.var_egisl_dn13)) * locals.var_t1) + (assign69380_e105911 * locals.var_t1_dn13)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn13,)
    }
};
        locals.var_igisl = assign69380_e105915;
        locals.var_igisl_dn0 = assign69380_e105915_d_n0;
        locals.var_igisl_dn2 = assign69380_e105915_d_n2;
        locals.var_igisl_dn4 = assign69380_e105915_d_n4;
        locals.var_igisl_dn5 = assign69380_e105915_d_n5;
        locals.var_igisl_dn6 = assign69380_e105915_d_n6;
        locals.var_igisl_dn7 = assign69380_e105915_d_n7;
        locals.var_igisl_dn8 = assign69380_e105915_d_n8;
        locals.var_igisl_dn9 = assign69380_e105915_d_n9;
        locals.var_igisl_dn10 = assign69380_e105915_d_n10;
        locals.var_igisl_dn13 = assign69380_e105915_d_n13;

        let (assign69390_e105921, assign69390_e105921_d_n5, assign69390_e105921_d_n7, assign69390_e105921_d_n8,) = {
    if (locals.var_guard1619 == 0.0) {
        let assign69390_e105919: f64 = (-locals.var_vbs);
        (assign69390_e105919, (-locals.var_vbs_dn5), (-locals.var_vbs_dn7), (-locals.var_vbs_dn8),)
    } else {
        (locals.var_vsb, locals.var_vsb_dn5, locals.var_vsb_dn7, locals.var_vsb_dn8,)
    }
};
        locals.var_vsb = assign69390_e105921;
        locals.var_vsb_dn5 = assign69390_e105921_d_n5;
        locals.var_vsb_dn7 = assign69390_e105921_d_n7;
        locals.var_vsb_dn8 = assign69390_e105921_d_n8;

        let assign69400_e105924: f64 = if locals.var_vsb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1622 = assign69400_e105924;

        let (assign69410_e105933, assign69410_e105933_d_n0, assign69410_e105933_d_n2, assign69410_e105933_d_n4, assign69410_e105933_d_n5, assign69410_e105933_d_n6, assign69410_e105933_d_n7, assign69410_e105933_d_n8, assign69410_e105933_d_n9, assign69410_e105933_d_n10, assign69410_e105933_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69410_e105931: f64 = (locals.var_vsb * locals.var_vsb);
        (assign69410_e105931, 0.0, 0.0, 0.0, ((locals.var_vsb_dn5 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn5)), 0.0, ((locals.var_vsb_dn7 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn7)), ((locals.var_vsb_dn8 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn8)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69410_e105933;
        locals.var_t2_dn0 = assign69410_e105933_d_n0;
        locals.var_t2_dn2 = assign69410_e105933_d_n2;
        locals.var_t2_dn4 = assign69410_e105933_d_n4;
        locals.var_t2_dn5 = assign69410_e105933_d_n5;
        locals.var_t2_dn6 = assign69410_e105933_d_n6;
        locals.var_t2_dn7 = assign69410_e105933_d_n7;
        locals.var_t2_dn8 = assign69410_e105933_d_n8;
        locals.var_t2_dn9 = assign69410_e105933_d_n9;
        locals.var_t2_dn10 = assign69410_e105933_d_n10;
        locals.var_t2_dn13 = assign69410_e105933_d_n13;

        let (assign69420_e105942, assign69420_e105942_d_n0, assign69420_e105942_d_n2, assign69420_e105942_d_n4, assign69420_e105942_d_n5, assign69420_e105942_d_n6, assign69420_e105942_d_n7, assign69420_e105942_d_n8, assign69420_e105942_d_n9, assign69420_e105942_d_n10, assign69420_e105942_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69420_e105940: f64 = (locals.var_t2 * locals.var_vsb);
        (assign69420_e105940, (locals.var_t2_dn0 * locals.var_vsb), (locals.var_t2_dn2 * locals.var_vsb), (locals.var_t2_dn4 * locals.var_vsb), ((locals.var_t2_dn5 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn5)), (locals.var_t2_dn6 * locals.var_vsb), ((locals.var_t2_dn7 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn7)), ((locals.var_t2_dn8 * locals.var_vsb) + (locals.var_t2 * locals.var_vsb_dn8)), (locals.var_t2_dn9 * locals.var_vsb), (locals.var_t2_dn10 * locals.var_vsb), (locals.var_t2_dn13 * locals.var_vsb),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign69420_e105942;
        locals.var_t4_dn0 = assign69420_e105942_d_n0;
        locals.var_t4_dn2 = assign69420_e105942_d_n2;
        locals.var_t4_dn4 = assign69420_e105942_d_n4;
        locals.var_t4_dn5 = assign69420_e105942_d_n5;
        locals.var_t4_dn6 = assign69420_e105942_d_n6;
        locals.var_t4_dn7 = assign69420_e105942_d_n7;
        locals.var_t4_dn8 = assign69420_e105942_d_n8;
        locals.var_t4_dn9 = assign69420_e105942_d_n9;
        locals.var_t4_dn10 = assign69420_e105942_d_n10;
        locals.var_t4_dn13 = assign69420_e105942_d_n13;

        let (assign69430_e105951, assign69430_e105951_d_n0, assign69430_e105951_d_n2, assign69430_e105951_d_n4, assign69430_e105951_d_n5, assign69430_e105951_d_n6, assign69430_e105951_d_n7, assign69430_e105951_d_n8, assign69430_e105951_d_n9, assign69430_e105951_d_n10, assign69430_e105951_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69430_e105949: f64 = (locals.var_t4 + 0.5);
        (assign69430_e105949, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign69430_e105951;
        locals.var_t0_dn0 = assign69430_e105951_d_n0;
        locals.var_t0_dn2 = assign69430_e105951_d_n2;
        locals.var_t0_dn4 = assign69430_e105951_d_n4;
        locals.var_t0_dn5 = assign69430_e105951_d_n5;
        locals.var_t0_dn6 = assign69430_e105951_d_n6;
        locals.var_t0_dn7 = assign69430_e105951_d_n7;
        locals.var_t0_dn8 = assign69430_e105951_d_n8;
        locals.var_t0_dn9 = assign69430_e105951_d_n9;
        locals.var_t0_dn10 = assign69430_e105951_d_n10;
        locals.var_t0_dn13 = assign69430_e105951_d_n13;

        let (assign69440_e105960, assign69440_e105960_d_n0, assign69440_e105960_d_n2, assign69440_e105960_d_n4, assign69440_e105960_d_n5, assign69440_e105960_d_n6, assign69440_e105960_d_n7, assign69440_e105960_d_n8, assign69440_e105960_d_n9, assign69440_e105960_d_n10, assign69440_e105960_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69440_e105958: f64 = (locals.var_t4 / locals.var_t0);
        (assign69440_e105958, (((locals.var_t4_dn0 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn2 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn4 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn5 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn6 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn7 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn8 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn9 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn10 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t4_dn13 * locals.var_t0) - (locals.var_t4 * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign69440_e105960;
        locals.var_t5_dn0 = assign69440_e105960_d_n0;
        locals.var_t5_dn2 = assign69440_e105960_d_n2;
        locals.var_t5_dn4 = assign69440_e105960_d_n4;
        locals.var_t5_dn5 = assign69440_e105960_d_n5;
        locals.var_t5_dn6 = assign69440_e105960_d_n6;
        locals.var_t5_dn7 = assign69440_e105960_d_n7;
        locals.var_t5_dn8 = assign69440_e105960_d_n8;
        locals.var_t5_dn9 = assign69440_e105960_d_n9;
        locals.var_t5_dn10 = assign69440_e105960_d_n10;
        locals.var_t5_dn13 = assign69440_e105960_d_n13;

        let (assign69450_e105981, assign69450_e105981_d_n0, assign69450_e105981_d_n2, assign69450_e105981_d_n4, assign69450_e105981_d_n5, assign69450_e105981_d_n6, assign69450_e105981_d_n7, assign69450_e105981_d_n8, assign69450_e105981_d_n9, assign69450_e105981_d_n10, assign69450_e105981_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69450_e105967: f64 = (3.0 * locals.var_t2);
        let assign69450_e105969: f64 = (assign69450_e105967 * locals.var_t0);
        let assign69450_e105972: f64 = (locals.var_t4 * 3.0);
        let assign69450_e105974: f64 = (assign69450_e105972 * locals.var_t2);
        let assign69450_e105975: f64 = (assign69450_e105969 - assign69450_e105974);
        let assign69450_e105978: f64 = (locals.var_t0 * locals.var_t0);
        let assign69450_e105979: f64 = (assign69450_e105975 / assign69450_e105978);
        (assign69450_e105979, (((((((3.0 * locals.var_t2_dn0) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn0)) - (((locals.var_t4_dn0 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn0))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn2) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn2)) - (((locals.var_t4_dn2 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn2))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn4) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn4)) - (((locals.var_t4_dn4 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn4))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn5) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn5)) - (((locals.var_t4_dn5 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn5))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn6) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn6)) - (((locals.var_t4_dn6 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn6))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn7) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn7)) - (((locals.var_t4_dn7 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn7))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn8) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn8)) - (((locals.var_t4_dn8 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn8))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn9) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn9)) - (((locals.var_t4_dn9 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn9))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn10) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn10)) - (((locals.var_t4_dn10 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn10))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)))) / (assign69450_e105978 * assign69450_e105978)), (((((((3.0 * locals.var_t2_dn13) * locals.var_t0) + (assign69450_e105967 * locals.var_t0_dn13)) - (((locals.var_t4_dn13 * 3.0) * locals.var_t2) + (assign69450_e105972 * locals.var_t2_dn13))) * assign69450_e105978) - (assign69450_e105975 * ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)))) / (assign69450_e105978 * assign69450_e105978)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign69450_e105981;
        locals.var_t7_dn0 = assign69450_e105981_d_n0;
        locals.var_t7_dn2 = assign69450_e105981_d_n2;
        locals.var_t7_dn4 = assign69450_e105981_d_n4;
        locals.var_t7_dn5 = assign69450_e105981_d_n5;
        locals.var_t7_dn6 = assign69450_e105981_d_n6;
        locals.var_t7_dn7 = assign69450_e105981_d_n7;
        locals.var_t7_dn8 = assign69450_e105981_d_n8;
        locals.var_t7_dn9 = assign69450_e105981_d_n9;
        locals.var_t7_dn10 = assign69450_e105981_d_n10;
        locals.var_t7_dn13 = assign69450_e105981_d_n13;

        let (assign69460_e105990, assign69460_e105990_d_n0, assign69460_e105990_d_n2, assign69460_e105990_d_n4, assign69460_e105990_d_n5, assign69460_e105990_d_n6, assign69460_e105990_d_n7, assign69460_e105990_d_n8, assign69460_e105990_d_n9, assign69460_e105990_d_n10, assign69460_e105990_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 != 0.0)) {
        let assign69460_e105988: f64 = (locals.var_igisl * locals.var_t5);
        (assign69460_e105988, ((locals.var_igisl_dn0 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn0)), ((locals.var_igisl_dn2 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn2)), ((locals.var_igisl_dn4 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn4)), ((locals.var_igisl_dn5 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn5)), ((locals.var_igisl_dn6 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn6)), ((locals.var_igisl_dn7 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn7)), ((locals.var_igisl_dn8 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn8)), ((locals.var_igisl_dn9 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn9)), ((locals.var_igisl_dn10 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn10)), ((locals.var_igisl_dn13 * locals.var_t5) + (locals.var_igisl * locals.var_t5_dn13)),)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn13,)
    }
};
        locals.var_igisl = assign69460_e105990;
        locals.var_igisl_dn0 = assign69460_e105990_d_n0;
        locals.var_igisl_dn2 = assign69460_e105990_d_n2;
        locals.var_igisl_dn4 = assign69460_e105990_d_n4;
        locals.var_igisl_dn5 = assign69460_e105990_d_n5;
        locals.var_igisl_dn6 = assign69460_e105990_d_n6;
        locals.var_igisl_dn7 = assign69460_e105990_d_n7;
        locals.var_igisl_dn8 = assign69460_e105990_d_n8;
        locals.var_igisl_dn9 = assign69460_e105990_d_n9;
        locals.var_igisl_dn10 = assign69460_e105990_d_n10;
        locals.var_igisl_dn13 = assign69460_e105990_d_n13;

        let (assign69470_e105998, assign69470_e105998_d_n0, assign69470_e105998_d_n2, assign69470_e105998_d_n4, assign69470_e105998_d_n5, assign69470_e105998_d_n6, assign69470_e105998_d_n7, assign69470_e105998_d_n8, assign69470_e105998_d_n9, assign69470_e105998_d_n10, assign69470_e105998_d_n13,) = {
    if ((locals.var_guard1619 == 0.0) && (locals.var_guard1622 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl, locals.var_igisl_dn0, locals.var_igisl_dn2, locals.var_igisl_dn4, locals.var_igisl_dn5, locals.var_igisl_dn6, locals.var_igisl_dn7, locals.var_igisl_dn8, locals.var_igisl_dn9, locals.var_igisl_dn10, locals.var_igisl_dn13,)
    }
};
        locals.var_igisl = assign69470_e105998;
        locals.var_igisl_dn0 = assign69470_e105998_d_n0;
        locals.var_igisl_dn2 = assign69470_e105998_d_n2;
        locals.var_igisl_dn4 = assign69470_e105998_d_n4;
        locals.var_igisl_dn5 = assign69470_e105998_d_n5;
        locals.var_igisl_dn6 = assign69470_e105998_d_n6;
        locals.var_igisl_dn7 = assign69470_e105998_d_n7;
        locals.var_igisl_dn8 = assign69470_e105998_d_n8;
        locals.var_igisl_dn9 = assign69470_e105998_d_n9;
        locals.var_igisl_dn10 = assign69470_e105998_d_n10;
        locals.var_igisl_dn13 = assign69470_e105998_d_n13;

        locals.var_flg_coovlps = 0.0;

        locals.var_flg_coovlp = 0.0;

        locals.var_flg_calcqover = 0.0;

        locals.var_flg_never_reach_vfbover = 0.0;

        locals.var_flg_calcqover = 0.0;

        let assign69540_e106007: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1627 = assign69540_e106007;

        let assign69550_e106010: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1628 = assign69550_e106010;

        let assign69560_e106013: f64 = if 1.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1629 = assign69560_e106013;

        let assign69570_e106016: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1630 = assign69570_e106016;

        let assign69580_e106027: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1631 = assign69580_e106027;

        let (assign69590_e106033,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69590_e106033;

        let (assign69600_e106039,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign69600_e106039;

        let (assign69610_e106047, assign69610_e106047_d_n2, assign69610_e106047_d_n6, assign69610_e106047_d_n7, assign69610_e106047_d_n8,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        let assign69610_e106045: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign69610_e106045, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign69610_e106047;
        locals.var_vgbgmt_dn2 = assign69610_e106047_d_n2;
        locals.var_vgbgmt_dn6 = assign69610_e106047_d_n6;
        locals.var_vgbgmt_dn7 = assign69610_e106047_d_n7;
        locals.var_vgbgmt_dn8 = assign69610_e106047_d_n8;

        let (assign69620_e106054, assign69620_e106054_d_n0, assign69620_e106054_d_n2, assign69620_e106054_d_n4, assign69620_e106054_d_n5, assign69620_e106054_d_n6, assign69620_e106054_d_n7, assign69620_e106054_d_n8, assign69620_e106054_d_n9, assign69620_e106054_d_n10, assign69620_e106054_d_n13,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        let assign69620_e106052: f64 = (-locals.var_vbsi);
        (assign69620_e106052, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69620_e106054;
        locals.var_vxbgmt_dn0 = assign69620_e106054_d_n0;
        locals.var_vxbgmt_dn2 = assign69620_e106054_d_n2;
        locals.var_vxbgmt_dn4 = assign69620_e106054_d_n4;
        locals.var_vxbgmt_dn5 = assign69620_e106054_d_n5;
        locals.var_vxbgmt_dn6 = assign69620_e106054_d_n6;
        locals.var_vxbgmt_dn7 = assign69620_e106054_d_n7;
        locals.var_vxbgmt_dn8 = assign69620_e106054_d_n8;
        locals.var_vxbgmt_dn9 = assign69620_e106054_d_n9;
        locals.var_vxbgmt_dn10 = assign69620_e106054_d_n10;
        locals.var_vxbgmt_dn13 = assign69620_e106054_d_n13;

        let (assign69630_e106060,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign69630_e106060;

        let (assign69640_e106066, assign69640_e106066_d_n0, assign69640_e106066_d_n2, assign69640_e106066_d_n4, assign69640_e106066_d_n5, assign69640_e106066_d_n6, assign69640_e106066_d_n7, assign69640_e106066_d_n8, assign69640_e106066_d_n9, assign69640_e106066_d_n10, assign69640_e106066_d_n13,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign69640_e106066;
        locals.var_lover_func_dn0 = assign69640_e106066_d_n0;
        locals.var_lover_func_dn2 = assign69640_e106066_d_n2;
        locals.var_lover_func_dn4 = assign69640_e106066_d_n4;
        locals.var_lover_func_dn5 = assign69640_e106066_d_n5;
        locals.var_lover_func_dn6 = assign69640_e106066_d_n6;
        locals.var_lover_func_dn7 = assign69640_e106066_d_n7;
        locals.var_lover_func_dn8 = assign69640_e106066_d_n8;
        locals.var_lover_func_dn9 = assign69640_e106066_d_n9;
        locals.var_lover_func_dn10 = assign69640_e106066_d_n10;
        locals.var_lover_func_dn13 = assign69640_e106066_d_n13;

        let (assign69650_e106072, assign69650_e106072_d_n0, assign69650_e106072_d_n2, assign69650_e106072_d_n4, assign69650_e106072_d_n5, assign69650_e106072_d_n6, assign69650_e106072_d_n7, assign69650_e106072_d_n8, assign69650_e106072_d_n9, assign69650_e106072_d_n10, assign69650_e106072_d_n13,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign69650_e106072;
        locals.var_wdep_func_dn0 = assign69650_e106072_d_n0;
        locals.var_wdep_func_dn2 = assign69650_e106072_d_n2;
        locals.var_wdep_func_dn4 = assign69650_e106072_d_n4;
        locals.var_wdep_func_dn5 = assign69650_e106072_d_n5;
        locals.var_wdep_func_dn6 = assign69650_e106072_d_n6;
        locals.var_wdep_func_dn7 = assign69650_e106072_d_n7;
        locals.var_wdep_func_dn8 = assign69650_e106072_d_n8;
        locals.var_wdep_func_dn9 = assign69650_e106072_d_n9;
        locals.var_wdep_func_dn10 = assign69650_e106072_d_n10;
        locals.var_wdep_func_dn13 = assign69650_e106072_d_n13;

        let (assign69660_e106078, assign69660_e106078_d_n0, assign69660_e106078_d_n2, assign69660_e106078_d_n4, assign69660_e106078_d_n5, assign69660_e106078_d_n6, assign69660_e106078_d_n7, assign69660_e106078_d_n8, assign69660_e106078_d_n9, assign69660_e106078_d_n10, assign69660_e106078_d_n13,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign69660_e106078;
        locals.var_cnst0over_func_dn0 = assign69660_e106078_d_n0;
        locals.var_cnst0over_func_dn2 = assign69660_e106078_d_n2;
        locals.var_cnst0over_func_dn4 = assign69660_e106078_d_n4;
        locals.var_cnst0over_func_dn5 = assign69660_e106078_d_n5;
        locals.var_cnst0over_func_dn6 = assign69660_e106078_d_n6;
        locals.var_cnst0over_func_dn7 = assign69660_e106078_d_n7;
        locals.var_cnst0over_func_dn8 = assign69660_e106078_d_n8;
        locals.var_cnst0over_func_dn9 = assign69660_e106078_d_n9;
        locals.var_cnst0over_func_dn10 = assign69660_e106078_d_n10;
        locals.var_cnst0over_func_dn13 = assign69660_e106078_d_n13;

        let (assign69670_e106084,) = {
    if ((locals.var_guard1627 != 0.0) && (locals.var_guard1631 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign69670_e106084;

        let assign69680_e106103: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1632 = assign69680_e106103;

    }

    pub(super) fn stamp_transient_block_238(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign69690_e106112,) = {
    if (((locals.var_guard1628 != 0.0) && (locals.var_guard1627 == 0.0)) && (locals.var_guard1632 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69690_e106112;

        let (assign69700_e106123, assign69700_e106123_d_n2, assign69700_e106123_d_n6, assign69700_e106123_d_n7, assign69700_e106123_d_n8,) = {
    if (((locals.var_guard1628 != 0.0) && (locals.var_guard1627 == 0.0)) && (locals.var_guard1632 != 0.0)) {
        let assign69700_e106121: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign69700_e106121, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign69700_e106123;
        locals.var_vgbgmt_dn2 = assign69700_e106123_d_n2;
        locals.var_vgbgmt_dn6 = assign69700_e106123_d_n6;
        locals.var_vgbgmt_dn7 = assign69700_e106123_d_n7;
        locals.var_vgbgmt_dn8 = assign69700_e106123_d_n8;

        let (assign69710_e106133, assign69710_e106133_d_n0, assign69710_e106133_d_n2, assign69710_e106133_d_n4, assign69710_e106133_d_n5, assign69710_e106133_d_n6, assign69710_e106133_d_n7, assign69710_e106133_d_n8, assign69710_e106133_d_n9, assign69710_e106133_d_n10, assign69710_e106133_d_n13,) = {
    if (((locals.var_guard1628 != 0.0) && (locals.var_guard1627 == 0.0)) && (locals.var_guard1632 != 0.0)) {
        let assign69710_e106131: f64 = (-locals.var_vbsei);
        (assign69710_e106131, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69710_e106133;
        locals.var_vxbgmt_dn0 = assign69710_e106133_d_n0;
        locals.var_vxbgmt_dn2 = assign69710_e106133_d_n2;
        locals.var_vxbgmt_dn4 = assign69710_e106133_d_n4;
        locals.var_vxbgmt_dn5 = assign69710_e106133_d_n5;
        locals.var_vxbgmt_dn6 = assign69710_e106133_d_n6;
        locals.var_vxbgmt_dn7 = assign69710_e106133_d_n7;
        locals.var_vxbgmt_dn8 = assign69710_e106133_d_n8;
        locals.var_vxbgmt_dn9 = assign69710_e106133_d_n9;
        locals.var_vxbgmt_dn10 = assign69710_e106133_d_n10;
        locals.var_vxbgmt_dn13 = assign69710_e106133_d_n13;

        let assign69720_e106144: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1633 = assign69720_e106144;

        let (assign69730_e106155,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign69730_e106155;

        let (assign69740_e106166,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign69740_e106166;

        let (assign69750_e106179, assign69750_e106179_d_n2, assign69750_e106179_d_n6, assign69750_e106179_d_n7, assign69750_e106179_d_n8,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        let assign69750_e106177: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign69750_e106177, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign69750_e106179;
        locals.var_vgbgmt_dn2 = assign69750_e106179_d_n2;
        locals.var_vgbgmt_dn6 = assign69750_e106179_d_n6;
        locals.var_vgbgmt_dn7 = assign69750_e106179_d_n7;
        locals.var_vgbgmt_dn8 = assign69750_e106179_d_n8;

        let (assign69760_e106192, assign69760_e106192_d_n0, assign69760_e106192_d_n2, assign69760_e106192_d_n4, assign69760_e106192_d_n5, assign69760_e106192_d_n6, assign69760_e106192_d_n7, assign69760_e106192_d_n8, assign69760_e106192_d_n9, assign69760_e106192_d_n10, assign69760_e106192_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        let assign69760_e106190: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign69760_e106190, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, (locals.var_vdsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69760_e106192;
        locals.var_vxbgmt_dn0 = assign69760_e106192_d_n0;
        locals.var_vxbgmt_dn2 = assign69760_e106192_d_n2;
        locals.var_vxbgmt_dn4 = assign69760_e106192_d_n4;
        locals.var_vxbgmt_dn5 = assign69760_e106192_d_n5;
        locals.var_vxbgmt_dn6 = assign69760_e106192_d_n6;
        locals.var_vxbgmt_dn7 = assign69760_e106192_d_n7;
        locals.var_vxbgmt_dn8 = assign69760_e106192_d_n8;
        locals.var_vxbgmt_dn9 = assign69760_e106192_d_n9;
        locals.var_vxbgmt_dn10 = assign69760_e106192_d_n10;
        locals.var_vxbgmt_dn13 = assign69760_e106192_d_n13;

        let (assign69770_e106203,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign69770_e106203;

        let (assign69780_e106218, assign69780_e106218_d_n0, assign69780_e106218_d_n2, assign69780_e106218_d_n4, assign69780_e106218_d_n5, assign69780_e106218_d_n6, assign69780_e106218_d_n7, assign69780_e106218_d_n8, assign69780_e106218_d_n9, assign69780_e106218_d_n10, assign69780_e106218_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        let assign69780_e106215: f64 = (p.p64 * p.p55);
        let assign69780_e106216: f64 = (p.p63 + assign69780_e106215);
        (assign69780_e106216, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign69780_e106218;
        locals.var_lover_func_dn0 = assign69780_e106218_d_n0;
        locals.var_lover_func_dn2 = assign69780_e106218_d_n2;
        locals.var_lover_func_dn4 = assign69780_e106218_d_n4;
        locals.var_lover_func_dn5 = assign69780_e106218_d_n5;
        locals.var_lover_func_dn6 = assign69780_e106218_d_n6;
        locals.var_lover_func_dn7 = assign69780_e106218_d_n7;
        locals.var_lover_func_dn8 = assign69780_e106218_d_n8;
        locals.var_lover_func_dn9 = assign69780_e106218_d_n9;
        locals.var_lover_func_dn10 = assign69780_e106218_d_n10;
        locals.var_lover_func_dn13 = assign69780_e106218_d_n13;

        let (assign69790_e106229, assign69790_e106229_d_n0, assign69790_e106229_d_n2, assign69790_e106229_d_n4, assign69790_e106229_d_n5, assign69790_e106229_d_n6, assign69790_e106229_d_n7, assign69790_e106229_d_n8, assign69790_e106229_d_n9, assign69790_e106229_d_n10, assign69790_e106229_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign69790_e106229;
        locals.var_wdep_func_dn0 = assign69790_e106229_d_n0;
        locals.var_wdep_func_dn2 = assign69790_e106229_d_n2;
        locals.var_wdep_func_dn4 = assign69790_e106229_d_n4;
        locals.var_wdep_func_dn5 = assign69790_e106229_d_n5;
        locals.var_wdep_func_dn6 = assign69790_e106229_d_n6;
        locals.var_wdep_func_dn7 = assign69790_e106229_d_n7;
        locals.var_wdep_func_dn8 = assign69790_e106229_d_n8;
        locals.var_wdep_func_dn9 = assign69790_e106229_d_n9;
        locals.var_wdep_func_dn10 = assign69790_e106229_d_n10;
        locals.var_wdep_func_dn13 = assign69790_e106229_d_n13;

        let (assign69800_e106240, assign69800_e106240_d_n0, assign69800_e106240_d_n2, assign69800_e106240_d_n4, assign69800_e106240_d_n5, assign69800_e106240_d_n6, assign69800_e106240_d_n7, assign69800_e106240_d_n8, assign69800_e106240_d_n9, assign69800_e106240_d_n10, assign69800_e106240_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign69800_e106240;
        locals.var_cnst0over_func_dn0 = assign69800_e106240_d_n0;
        locals.var_cnst0over_func_dn2 = assign69800_e106240_d_n2;
        locals.var_cnst0over_func_dn4 = assign69800_e106240_d_n4;
        locals.var_cnst0over_func_dn5 = assign69800_e106240_d_n5;
        locals.var_cnst0over_func_dn6 = assign69800_e106240_d_n6;
        locals.var_cnst0over_func_dn7 = assign69800_e106240_d_n7;
        locals.var_cnst0over_func_dn8 = assign69800_e106240_d_n8;
        locals.var_cnst0over_func_dn9 = assign69800_e106240_d_n9;
        locals.var_cnst0over_func_dn10 = assign69800_e106240_d_n10;
        locals.var_cnst0over_func_dn13 = assign69800_e106240_d_n13;

        let (assign69810_e106251,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign69810_e106251;

        let (assign69820_e106263, assign69820_e106263_d_n0, assign69820_e106263_d_n2, assign69820_e106263_d_n4, assign69820_e106263_d_n5, assign69820_e106263_d_n6, assign69820_e106263_d_n7, assign69820_e106263_d_n8, assign69820_e106263_d_n9, assign69820_e106263_d_n10, assign69820_e106263_d_n13,) = {
    if (((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) {
        let assign69820_e106261: f64 = (-locals.var_lover_func);
        (assign69820_e106261, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign69820_e106263;
        locals.var_lover_func_dn0 = assign69820_e106263_d_n0;
        locals.var_lover_func_dn2 = assign69820_e106263_d_n2;
        locals.var_lover_func_dn4 = assign69820_e106263_d_n4;
        locals.var_lover_func_dn5 = assign69820_e106263_d_n5;
        locals.var_lover_func_dn6 = assign69820_e106263_d_n6;
        locals.var_lover_func_dn7 = assign69820_e106263_d_n7;
        locals.var_lover_func_dn8 = assign69820_e106263_d_n8;
        locals.var_lover_func_dn9 = assign69820_e106263_d_n9;
        locals.var_lover_func_dn10 = assign69820_e106263_d_n10;
        locals.var_lover_func_dn13 = assign69820_e106263_d_n13;

        let assign69830_e106274: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1634 = assign69830_e106274;

        let (assign69840_e106288, assign69840_e106288_d_n0, assign69840_e106288_d_n2, assign69840_e106288_d_n4, assign69840_e106288_d_n5, assign69840_e106288_d_n6, assign69840_e106288_d_n7, assign69840_e106288_d_n8, assign69840_e106288_d_n9, assign69840_e106288_d_n10, assign69840_e106288_d_n13,) = {
    if ((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) {
        let assign69840_e106286: f64 = (-locals.var_lover_func);
        (assign69840_e106286, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign69840_e106288;
        locals.var_lover_func_dn0 = assign69840_e106288_d_n0;
        locals.var_lover_func_dn2 = assign69840_e106288_d_n2;
        locals.var_lover_func_dn4 = assign69840_e106288_d_n4;
        locals.var_lover_func_dn5 = assign69840_e106288_d_n5;
        locals.var_lover_func_dn6 = assign69840_e106288_d_n6;
        locals.var_lover_func_dn7 = assign69840_e106288_d_n7;
        locals.var_lover_func_dn8 = assign69840_e106288_d_n8;
        locals.var_lover_func_dn9 = assign69840_e106288_d_n9;
        locals.var_lover_func_dn10 = assign69840_e106288_d_n10;
        locals.var_lover_func_dn13 = assign69840_e106288_d_n13;

        let (assign69850_e106301, assign69850_e106301_d_n0, assign69850_e106301_d_n2, assign69850_e106301_d_n4, assign69850_e106301_d_n5, assign69850_e106301_d_n6, assign69850_e106301_d_n7, assign69850_e106301_d_n8, assign69850_e106301_d_n9, assign69850_e106301_d_n10, assign69850_e106301_d_n13,) = {
    if ((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign69850_e106301;
        locals.var_t1_dn0 = assign69850_e106301_d_n0;
        locals.var_t1_dn2 = assign69850_e106301_d_n2;
        locals.var_t1_dn4 = assign69850_e106301_d_n4;
        locals.var_t1_dn5 = assign69850_e106301_d_n5;
        locals.var_t1_dn6 = assign69850_e106301_d_n6;
        locals.var_t1_dn7 = assign69850_e106301_d_n7;
        locals.var_t1_dn8 = assign69850_e106301_d_n8;
        locals.var_t1_dn9 = assign69850_e106301_d_n9;
        locals.var_t1_dn10 = assign69850_e106301_d_n10;
        locals.var_t1_dn13 = assign69850_e106301_d_n13;

        let (assign69860_e106320, assign69860_e106320_d_n0, assign69860_e106320_d_n2, assign69860_e106320_d_n4, assign69860_e106320_d_n5, assign69860_e106320_d_n6, assign69860_e106320_d_n7, assign69860_e106320_d_n8, assign69860_e106320_d_n9, assign69860_e106320_d_n10, assign69860_e106320_d_n13,) = {
    if ((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) {
        let assign69860_e106314: f64 = (locals.var_t1 * locals.var_t1);
        let assign69860_e106316: f64 = (assign69860_e106314 / locals.var_kjunc);
        let assign69860_e106318: f64 = (assign69860_e106316 - p.p137);
        (assign69860_e106318, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) * locals.var_kjunc) - (assign69860_e106314 * locals.var_kjunc_dn13)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn13,)
    }
};
        locals.var_vxb_lim = assign69860_e106320;
        locals.var_vxb_lim_dn0 = assign69860_e106320_d_n0;
        locals.var_vxb_lim_dn2 = assign69860_e106320_d_n2;
        locals.var_vxb_lim_dn4 = assign69860_e106320_d_n4;
        locals.var_vxb_lim_dn5 = assign69860_e106320_d_n5;
        locals.var_vxb_lim_dn6 = assign69860_e106320_d_n6;
        locals.var_vxb_lim_dn7 = assign69860_e106320_d_n7;
        locals.var_vxb_lim_dn8 = assign69860_e106320_d_n8;
        locals.var_vxb_lim_dn9 = assign69860_e106320_d_n9;
        locals.var_vxb_lim_dn10 = assign69860_e106320_d_n10;
        locals.var_vxb_lim_dn13 = assign69860_e106320_d_n13;

        let assign69870_e106323: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1635 = assign69870_e106323;

        let assign69880_e106330: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1636 = assign69880_e106330;

        let (assign69890_e106347, assign69890_e106347_d_n0, assign69890_e106347_d_n2, assign69890_e106347_d_n4, assign69890_e106347_d_n5, assign69890_e106347_d_n6, assign69890_e106347_d_n7, assign69890_e106347_d_n8, assign69890_e106347_d_n9, assign69890_e106347_d_n10, assign69890_e106347_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69890_e106347;
        locals.var_vxbgmt_dn0 = assign69890_e106347_d_n0;
        locals.var_vxbgmt_dn2 = assign69890_e106347_d_n2;
        locals.var_vxbgmt_dn4 = assign69890_e106347_d_n4;
        locals.var_vxbgmt_dn5 = assign69890_e106347_d_n5;
        locals.var_vxbgmt_dn6 = assign69890_e106347_d_n6;
        locals.var_vxbgmt_dn7 = assign69890_e106347_d_n7;
        locals.var_vxbgmt_dn8 = assign69890_e106347_d_n8;
        locals.var_vxbgmt_dn9 = assign69890_e106347_d_n9;
        locals.var_vxbgmt_dn10 = assign69890_e106347_d_n10;
        locals.var_vxbgmt_dn13 = assign69890_e106347_d_n13;

        let (assign69900_e106371, assign69900_e106371_d_n0, assign69900_e106371_d_n2, assign69900_e106371_d_n4, assign69900_e106371_d_n5, assign69900_e106371_d_n6, assign69900_e106371_d_n7, assign69900_e106371_d_n8, assign69900_e106371_d_n9, assign69900_e106371_d_n10, assign69900_e106371_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let (assign69900_e106369,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign69900_e106367: f64 = (-1.0);
                (assign69900_e106367,)
            } else {
                (1.0,)
            }
        };
        (assign69900_e106369, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign69900_e106371;
        locals.var_tmf3_dn0 = assign69900_e106371_d_n0;
        locals.var_tmf3_dn2 = assign69900_e106371_d_n2;
        locals.var_tmf3_dn4 = assign69900_e106371_d_n4;
        locals.var_tmf3_dn5 = assign69900_e106371_d_n5;
        locals.var_tmf3_dn6 = assign69900_e106371_d_n6;
        locals.var_tmf3_dn7 = assign69900_e106371_d_n7;
        locals.var_tmf3_dn8 = assign69900_e106371_d_n8;
        locals.var_tmf3_dn9 = assign69900_e106371_d_n9;
        locals.var_tmf3_dn10 = assign69900_e106371_d_n10;
        locals.var_tmf3_dn13 = assign69900_e106371_d_n13;

        let (assign69910_e106391, assign69910_e106391_d_n0, assign69910_e106391_d_n2, assign69910_e106391_d_n4, assign69910_e106391_d_n5, assign69910_e106391_d_n6, assign69910_e106391_d_n7, assign69910_e106391_d_n8, assign69910_e106391_d_n9, assign69910_e106391_d_n10, assign69910_e106391_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let assign69910_e106389: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign69910_e106389, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn13 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign69910_e106391;
        locals.var_tmf4_dn0 = assign69910_e106391_d_n0;
        locals.var_tmf4_dn2 = assign69910_e106391_d_n2;
        locals.var_tmf4_dn4 = assign69910_e106391_d_n4;
        locals.var_tmf4_dn5 = assign69910_e106391_d_n5;
        locals.var_tmf4_dn6 = assign69910_e106391_d_n6;
        locals.var_tmf4_dn7 = assign69910_e106391_d_n7;
        locals.var_tmf4_dn8 = assign69910_e106391_d_n8;
        locals.var_tmf4_dn9 = assign69910_e106391_d_n9;
        locals.var_tmf4_dn10 = assign69910_e106391_d_n10;
        locals.var_tmf4_dn13 = assign69910_e106391_d_n13;

        let (assign69920_e106415, assign69920_e106415_d_n0, assign69920_e106415_d_n2, assign69920_e106415_d_n4, assign69920_e106415_d_n5, assign69920_e106415_d_n6, assign69920_e106415_d_n7, assign69920_e106415_d_n8, assign69920_e106415_d_n9, assign69920_e106415_d_n10, assign69920_e106415_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let assign69920_e106410: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign69920_e106412: f64 = (assign69920_e106410).powf(p.p113);
        let assign69920_e106413: f64 = (1.0 + assign69920_e106412);
        (assign69920_e106413, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign69920_e106410).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign69920_e106412 * (p.p113 * ((((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign69920_e106410))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign69920_e106415;
        locals.var_tmf1_dn0 = assign69920_e106415_d_n0;
        locals.var_tmf1_dn2 = assign69920_e106415_d_n2;
        locals.var_tmf1_dn4 = assign69920_e106415_d_n4;
        locals.var_tmf1_dn5 = assign69920_e106415_d_n5;
        locals.var_tmf1_dn6 = assign69920_e106415_d_n6;
        locals.var_tmf1_dn7 = assign69920_e106415_d_n7;
        locals.var_tmf1_dn8 = assign69920_e106415_d_n8;
        locals.var_tmf1_dn9 = assign69920_e106415_d_n9;
        locals.var_tmf1_dn10 = assign69920_e106415_d_n10;
        locals.var_tmf1_dn13 = assign69920_e106415_d_n13;

        let (assign69930_e106437, assign69930_e106437_d_n0, assign69930_e106437_d_n2, assign69930_e106437_d_n4, assign69930_e106437_d_n5, assign69930_e106437_d_n6, assign69930_e106437_d_n7, assign69930_e106437_d_n8, assign69930_e106437_d_n9, assign69930_e106437_d_n10, assign69930_e106437_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let assign69930_e106434: f64 = (1.0 / p.p113);
        let assign69930_e106435: f64 = (locals.var_tmf1).powf(assign69930_e106434);
        (assign69930_e106435, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign69930_e106434) as f64).is_finite() && ((assign69930_e106434) as f64).fract() == 0.0 { if assign69930_e106434 == 0.0 { 0.0 } else { (assign69930_e106434 * ((locals.var_tmf1).powf(assign69930_e106434 - 1.0) * locals.var_tmf1_dn13)) } } else { (assign69930_e106435 * (assign69930_e106434 * (locals.var_tmf1_dn13 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign69930_e106437;
        locals.var_tmf2_dn0 = assign69930_e106437_d_n0;
        locals.var_tmf2_dn2 = assign69930_e106437_d_n2;
        locals.var_tmf2_dn4 = assign69930_e106437_d_n4;
        locals.var_tmf2_dn5 = assign69930_e106437_d_n5;
        locals.var_tmf2_dn6 = assign69930_e106437_d_n6;
        locals.var_tmf2_dn7 = assign69930_e106437_d_n7;
        locals.var_tmf2_dn8 = assign69930_e106437_d_n8;
        locals.var_tmf2_dn9 = assign69930_e106437_d_n9;
        locals.var_tmf2_dn10 = assign69930_e106437_d_n10;
        locals.var_tmf2_dn13 = assign69930_e106437_d_n13;

        let (assign69940_e106459, assign69940_e106459_d_n0, assign69940_e106459_d_n2, assign69940_e106459_d_n4, assign69940_e106459_d_n5, assign69940_e106459_d_n6, assign69940_e106459_d_n7, assign69940_e106459_d_n8, assign69940_e106459_d_n9, assign69940_e106459_d_n10, assign69940_e106459_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1636 == 0.0)) {
        let assign69940_e106455: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign69940_e106457: f64 = (assign69940_e106455 / locals.var_tmf2);
        (assign69940_e106457, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn13 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn13)) * locals.var_tmf2) - (assign69940_e106455 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign69940_e106459;
        locals.var_vxbgmt_dn0 = assign69940_e106459_d_n0;
        locals.var_vxbgmt_dn2 = assign69940_e106459_d_n2;
        locals.var_vxbgmt_dn4 = assign69940_e106459_d_n4;
        locals.var_vxbgmt_dn5 = assign69940_e106459_d_n5;
        locals.var_vxbgmt_dn6 = assign69940_e106459_d_n6;
        locals.var_vxbgmt_dn7 = assign69940_e106459_d_n7;
        locals.var_vxbgmt_dn8 = assign69940_e106459_d_n8;
        locals.var_vxbgmt_dn9 = assign69940_e106459_d_n9;
        locals.var_vxbgmt_dn10 = assign69940_e106459_d_n10;
        locals.var_vxbgmt_dn13 = assign69940_e106459_d_n13;

        let (assign69950_e106487, assign69950_e106487_d_n0, assign69950_e106487_d_n2, assign69950_e106487_d_n4, assign69950_e106487_d_n5, assign69950_e106487_d_n6, assign69950_e106487_d_n7, assign69950_e106487_d_n8, assign69950_e106487_d_n9, assign69950_e106487_d_n10, assign69950_e106487_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign69950_e106474: f64 = (locals.var_vxbgmt + p.p137);
        let assign69950_e106477: f64 = (locals.var_vxbgmt + p.p137);
        let assign69950_e106478: f64 = (assign69950_e106474 * assign69950_e106477);
        let assign69950_e106481: f64 = (4.0 * 0.1);
        let assign69950_e106483: f64 = (assign69950_e106481 * 0.1);
        let assign69950_e106484: f64 = (assign69950_e106478 + assign69950_e106483);
        let assign69950_e106485: f64 = (assign69950_e106484).sqrt();
        (assign69950_e106485, (((locals.var_vxbgmt_dn0 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn0)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn2 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn2)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn4 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn4)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn5 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn5)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn6 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn6)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn7 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn7)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn8 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn8)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn9 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn9)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn10 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn10)) / (2.0 * assign69950_e106485)), (((locals.var_vxbgmt_dn13 * assign69950_e106477) + (assign69950_e106474 * locals.var_vxbgmt_dn13)) / (2.0 * assign69950_e106485)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign69950_e106487;
        locals.var_tmf2_dn0 = assign69950_e106487_d_n0;
        locals.var_tmf2_dn2 = assign69950_e106487_d_n2;
        locals.var_tmf2_dn4 = assign69950_e106487_d_n4;
        locals.var_tmf2_dn5 = assign69950_e106487_d_n5;
        locals.var_tmf2_dn6 = assign69950_e106487_d_n6;
        locals.var_tmf2_dn7 = assign69950_e106487_d_n7;
        locals.var_tmf2_dn8 = assign69950_e106487_d_n8;
        locals.var_tmf2_dn9 = assign69950_e106487_d_n9;
        locals.var_tmf2_dn10 = assign69950_e106487_d_n10;
        locals.var_tmf2_dn13 = assign69950_e106487_d_n13;

        let (assign69960_e106510, assign69960_e106510_d_n0, assign69960_e106510_d_n2, assign69960_e106510_d_n4, assign69960_e106510_d_n5, assign69960_e106510_d_n6, assign69960_e106510_d_n7, assign69960_e106510_d_n8, assign69960_e106510_d_n9, assign69960_e106510_d_n10, assign69960_e106510_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign69960_e106504: f64 = (locals.var_vxbgmt + p.p137);
        let assign69960_e106506: f64 = (assign69960_e106504 / locals.var_tmf2);
        let assign69960_e106507: f64 = (1.0 + assign69960_e106506);
        let assign69960_e106508: f64 = (0.5 * assign69960_e106507);
        (assign69960_e106508, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn13 * locals.var_tmf2) - (assign69960_e106504 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign69960_e106510;
        locals.var_t9_dn0 = assign69960_e106510_d_n0;
        locals.var_t9_dn2 = assign69960_e106510_d_n2;
        locals.var_t9_dn4 = assign69960_e106510_d_n4;
        locals.var_t9_dn5 = assign69960_e106510_d_n5;
        locals.var_t9_dn6 = assign69960_e106510_d_n6;
        locals.var_t9_dn7 = assign69960_e106510_d_n7;
        locals.var_t9_dn8 = assign69960_e106510_d_n8;
        locals.var_t9_dn9 = assign69960_e106510_d_n9;
        locals.var_t9_dn10 = assign69960_e106510_d_n10;
        locals.var_t9_dn13 = assign69960_e106510_d_n13;

        let (assign69970_e106531, assign69970_e106531_d_n0, assign69970_e106531_d_n2, assign69970_e106531_d_n4, assign69970_e106531_d_n5, assign69970_e106531_d_n6, assign69970_e106531_d_n7, assign69970_e106531_d_n8, assign69970_e106531_d_n9, assign69970_e106531_d_n10, assign69970_e106531_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign69970_e106526: f64 = (locals.var_vxbgmt + p.p137);
        let assign69970_e106528: f64 = (assign69970_e106526 + locals.var_tmf2);
        let assign69970_e106529: f64 = (0.5 * assign69970_e106528);
        (assign69970_e106529, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69970_e106531;
        locals.var_t2_dn0 = assign69970_e106531_d_n0;
        locals.var_t2_dn2 = assign69970_e106531_d_n2;
        locals.var_t2_dn4 = assign69970_e106531_d_n4;
        locals.var_t2_dn5 = assign69970_e106531_d_n5;
        locals.var_t2_dn6 = assign69970_e106531_d_n6;
        locals.var_t2_dn7 = assign69970_e106531_d_n7;
        locals.var_t2_dn8 = assign69970_e106531_d_n8;
        locals.var_t2_dn9 = assign69970_e106531_d_n9;
        locals.var_t2_dn10 = assign69970_e106531_d_n10;
        locals.var_t2_dn13 = assign69970_e106531_d_n13;

        let assign69980_e106534: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1637 = assign69980_e106534;

        let (assign69990_e106551, assign69990_e106551_d_n0, assign69990_e106551_d_n2, assign69990_e106551_d_n4, assign69990_e106551_d_n5, assign69990_e106551_d_n6, assign69990_e106551_d_n7, assign69990_e106551_d_n8, assign69990_e106551_d_n9, assign69990_e106551_d_n10, assign69990_e106551_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign69990_e106551;
        locals.var_t2_dn0 = assign69990_e106551_d_n0;
        locals.var_t2_dn2 = assign69990_e106551_d_n2;
        locals.var_t2_dn4 = assign69990_e106551_d_n4;
        locals.var_t2_dn5 = assign69990_e106551_d_n5;
        locals.var_t2_dn6 = assign69990_e106551_d_n6;
        locals.var_t2_dn7 = assign69990_e106551_d_n7;
        locals.var_t2_dn8 = assign69990_e106551_d_n8;
        locals.var_t2_dn9 = assign69990_e106551_d_n9;
        locals.var_t2_dn10 = assign69990_e106551_d_n10;
        locals.var_t2_dn13 = assign69990_e106551_d_n13;

        let (assign70000_e106568, assign70000_e106568_d_n0, assign70000_e106568_d_n2, assign70000_e106568_d_n4, assign70000_e106568_d_n5, assign70000_e106568_d_n6, assign70000_e106568_d_n7, assign70000_e106568_d_n8, assign70000_e106568_d_n9, assign70000_e106568_d_n10, assign70000_e106568_d_n13,) = {
    if ((((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) && (locals.var_guard1637 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign70000_e106568;
        locals.var_t9_dn0 = assign70000_e106568_d_n0;
        locals.var_t9_dn2 = assign70000_e106568_d_n2;
        locals.var_t9_dn4 = assign70000_e106568_d_n4;
        locals.var_t9_dn5 = assign70000_e106568_d_n5;
        locals.var_t9_dn6 = assign70000_e106568_d_n6;
        locals.var_t9_dn7 = assign70000_e106568_d_n7;
        locals.var_t9_dn8 = assign70000_e106568_d_n8;
        locals.var_t9_dn9 = assign70000_e106568_d_n9;
        locals.var_t9_dn10 = assign70000_e106568_d_n10;
        locals.var_t9_dn13 = assign70000_e106568_d_n13;

    }

    pub(super) fn stamp_transient_block_239(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign70010_e106588, assign70010_e106588_d_n0, assign70010_e106588_d_n2, assign70010_e106588_d_n4, assign70010_e106588_d_n5, assign70010_e106588_d_n6, assign70010_e106588_d_n7, assign70010_e106588_d_n8, assign70010_e106588_d_n9, assign70010_e106588_d_n10, assign70010_e106588_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign70010_e106583: f64 = (locals.var_kjunc * locals.var_t2);
        let assign70010_e106584: f64 = (assign70010_e106583).sqrt();
        let assign70010_e106586: f64 = (assign70010_e106584 * p.p432);
        (assign70010_e106586, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign70010_e106584)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign70010_e106584)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign70010_e106588;
        locals.var_wjunc0_dn0 = assign70010_e106588_d_n0;
        locals.var_wjunc0_dn2 = assign70010_e106588_d_n2;
        locals.var_wjunc0_dn4 = assign70010_e106588_d_n4;
        locals.var_wjunc0_dn5 = assign70010_e106588_d_n5;
        locals.var_wjunc0_dn6 = assign70010_e106588_d_n6;
        locals.var_wjunc0_dn7 = assign70010_e106588_d_n7;
        locals.var_wjunc0_dn8 = assign70010_e106588_d_n8;
        locals.var_wjunc0_dn9 = assign70010_e106588_d_n9;
        locals.var_wjunc0_dn10 = assign70010_e106588_d_n10;
        locals.var_wjunc0_dn13 = assign70010_e106588_d_n13;

        let (assign70020_e106605, assign70020_e106605_d_n0, assign70020_e106605_d_n2, assign70020_e106605_d_n4, assign70020_e106605_d_n5, assign70020_e106605_d_n6, assign70020_e106605_d_n7, assign70020_e106605_d_n8, assign70020_e106605_d_n9, assign70020_e106605_d_n10, assign70020_e106605_d_n13,) = {
    if (((((locals.var_guard1629 != 0.0) && (!((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)))) && (locals.var_guard1633 != 0.0)) && (locals.var_guard1634 != 0.0)) && (locals.var_guard1635 != 0.0)) {
        let assign70020_e106603: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign70020_e106603, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn13 - locals.var_wjunc0_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign70020_e106605;
        locals.var_lover_func_dn0 = assign70020_e106605_d_n0;
        locals.var_lover_func_dn2 = assign70020_e106605_d_n2;
        locals.var_lover_func_dn4 = assign70020_e106605_d_n4;
        locals.var_lover_func_dn5 = assign70020_e106605_d_n5;
        locals.var_lover_func_dn6 = assign70020_e106605_d_n6;
        locals.var_lover_func_dn7 = assign70020_e106605_d_n7;
        locals.var_lover_func_dn8 = assign70020_e106605_d_n8;
        locals.var_lover_func_dn9 = assign70020_e106605_d_n9;
        locals.var_lover_func_dn10 = assign70020_e106605_d_n10;
        locals.var_lover_func_dn13 = assign70020_e106605_d_n13;

        let assign70030_e106624: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1638 = assign70030_e106624;

        let (assign70040_e106637,) = {
    if (((locals.var_guard1630 != 0.0) && (!(((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)) || (locals.var_guard1629 != 0.0)))) && (locals.var_guard1638 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign70040_e106637;

        let (assign70050_e106652, assign70050_e106652_d_n2, assign70050_e106652_d_n6, assign70050_e106652_d_n7, assign70050_e106652_d_n8,) = {
    if (((locals.var_guard1630 != 0.0) && (!(((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)) || (locals.var_guard1629 != 0.0)))) && (locals.var_guard1638 != 0.0)) {
        let assign70050_e106650: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign70050_e106650, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign70050_e106652;
        locals.var_vgbgmt_dn2 = assign70050_e106652_d_n2;
        locals.var_vgbgmt_dn6 = assign70050_e106652_d_n6;
        locals.var_vgbgmt_dn7 = assign70050_e106652_d_n7;
        locals.var_vgbgmt_dn8 = assign70050_e106652_d_n8;

        let (assign70060_e106667, assign70060_e106667_d_n0, assign70060_e106667_d_n2, assign70060_e106667_d_n4, assign70060_e106667_d_n5, assign70060_e106667_d_n6, assign70060_e106667_d_n7, assign70060_e106667_d_n8, assign70060_e106667_d_n9, assign70060_e106667_d_n10, assign70060_e106667_d_n13,) = {
    if (((locals.var_guard1630 != 0.0) && (!(((locals.var_guard1627 != 0.0) || (locals.var_guard1628 != 0.0)) || (locals.var_guard1629 != 0.0)))) && (locals.var_guard1638 != 0.0)) {
        let assign70060_e106665: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign70060_e106665, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign70060_e106667;
        locals.var_vxbgmt_dn0 = assign70060_e106667_d_n0;
        locals.var_vxbgmt_dn2 = assign70060_e106667_d_n2;
        locals.var_vxbgmt_dn4 = assign70060_e106667_d_n4;
        locals.var_vxbgmt_dn5 = assign70060_e106667_d_n5;
        locals.var_vxbgmt_dn6 = assign70060_e106667_d_n6;
        locals.var_vxbgmt_dn7 = assign70060_e106667_d_n7;
        locals.var_vxbgmt_dn8 = assign70060_e106667_d_n8;
        locals.var_vxbgmt_dn9 = assign70060_e106667_d_n9;
        locals.var_vxbgmt_dn10 = assign70060_e106667_d_n10;
        locals.var_vxbgmt_dn13 = assign70060_e106667_d_n13;

        let (assign70070_e106671, assign70070_e106671_d_n0, assign70070_e106671_d_n2, assign70070_e106671_d_n4, assign70070_e106671_d_n5, assign70070_e106671_d_n6, assign70070_e106671_d_n7, assign70070_e106671_d_n8, assign70070_e106671_d_n9, assign70070_e106671_d_n10, assign70070_e106671_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn13,)
    }
};
        locals.var_vbs_bnd_over = assign70070_e106671;
        locals.var_vbs_bnd_over_dn0 = assign70070_e106671_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70070_e106671_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70070_e106671_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70070_e106671_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70070_e106671_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70070_e106671_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70070_e106671_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70070_e106671_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70070_e106671_d_n10;
        locals.var_vbs_bnd_over_dn13 = assign70070_e106671_d_n13;

        let (assign70090_e106679,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign70090_e106679;

        let (assign70100_e106683, assign70100_e106683_d_n0, assign70100_e106683_d_n2, assign70100_e106683_d_n4, assign70100_e106683_d_n5, assign70100_e106683_d_n6, assign70100_e106683_d_n7, assign70100_e106683_d_n8, assign70100_e106683_d_n9, assign70100_e106683_d_n10, assign70100_e106683_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign70100_e106683;
        locals.var_fb_dn0 = assign70100_e106683_d_n0;
        locals.var_fb_dn2 = assign70100_e106683_d_n2;
        locals.var_fb_dn4 = assign70100_e106683_d_n4;
        locals.var_fb_dn5 = assign70100_e106683_d_n5;
        locals.var_fb_dn6 = assign70100_e106683_d_n6;
        locals.var_fb_dn7 = assign70100_e106683_d_n7;
        locals.var_fb_dn8 = assign70100_e106683_d_n8;
        locals.var_fb_dn9 = assign70100_e106683_d_n9;
        locals.var_fb_dn10 = assign70100_e106683_d_n10;
        locals.var_fb_dn13 = assign70100_e106683_d_n13;

        let (assign70110_e106687, assign70110_e106687_d_n0, assign70110_e106687_d_n2, assign70110_e106687_d_n4, assign70110_e106687_d_n5, assign70110_e106687_d_n6, assign70110_e106687_d_n7, assign70110_e106687_d_n8, assign70110_e106687_d_n9, assign70110_e106687_d_n10, assign70110_e106687_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
        locals.var_fs01 = assign70110_e106687;
        locals.var_fs01_dn0 = assign70110_e106687_d_n0;
        locals.var_fs01_dn2 = assign70110_e106687_d_n2;
        locals.var_fs01_dn4 = assign70110_e106687_d_n4;
        locals.var_fs01_dn5 = assign70110_e106687_d_n5;
        locals.var_fs01_dn6 = assign70110_e106687_d_n6;
        locals.var_fs01_dn7 = assign70110_e106687_d_n7;
        locals.var_fs01_dn8 = assign70110_e106687_d_n8;
        locals.var_fs01_dn9 = assign70110_e106687_d_n9;
        locals.var_fs01_dn10 = assign70110_e106687_d_n10;
        locals.var_fs01_dn13 = assign70110_e106687_d_n13;

        let (assign70120_e106691, assign70120_e106691_d_n0, assign70120_e106691_d_n2, assign70120_e106691_d_n4, assign70120_e106691_d_n5, assign70120_e106691_d_n6, assign70120_e106691_d_n7, assign70120_e106691_d_n8, assign70120_e106691_d_n9, assign70120_e106691_d_n10, assign70120_e106691_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
        locals.var_fs02 = assign70120_e106691;
        locals.var_fs02_dn0 = assign70120_e106691_d_n0;
        locals.var_fs02_dn2 = assign70120_e106691_d_n2;
        locals.var_fs02_dn4 = assign70120_e106691_d_n4;
        locals.var_fs02_dn5 = assign70120_e106691_d_n5;
        locals.var_fs02_dn6 = assign70120_e106691_d_n6;
        locals.var_fs02_dn7 = assign70120_e106691_d_n7;
        locals.var_fs02_dn8 = assign70120_e106691_d_n8;
        locals.var_fs02_dn9 = assign70120_e106691_d_n9;
        locals.var_fs02_dn10 = assign70120_e106691_d_n10;
        locals.var_fs02_dn13 = assign70120_e106691_d_n13;

        let (assign70130_e106695, assign70130_e106695_d_n0, assign70130_e106695_d_n2, assign70130_e106695_d_n4, assign70130_e106695_d_n5, assign70130_e106695_d_n6, assign70130_e106695_d_n7, assign70130_e106695_d_n8, assign70130_e106695_d_n9, assign70130_e106695_d_n10, assign70130_e106695_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
        locals.var_fs0 = assign70130_e106695;
        locals.var_fs0_dn0 = assign70130_e106695_d_n0;
        locals.var_fs0_dn2 = assign70130_e106695_d_n2;
        locals.var_fs0_dn4 = assign70130_e106695_d_n4;
        locals.var_fs0_dn5 = assign70130_e106695_d_n5;
        locals.var_fs0_dn6 = assign70130_e106695_d_n6;
        locals.var_fs0_dn7 = assign70130_e106695_d_n7;
        locals.var_fs0_dn8 = assign70130_e106695_d_n8;
        locals.var_fs0_dn9 = assign70130_e106695_d_n9;
        locals.var_fs0_dn10 = assign70130_e106695_d_n10;
        locals.var_fs0_dn13 = assign70130_e106695_d_n13;

        let (assign70140_e106699, assign70140_e106699_d_n0, assign70140_e106699_d_n2, assign70140_e106699_d_n4, assign70140_e106699_d_n5, assign70140_e106699_d_n6, assign70140_e106699_d_n7, assign70140_e106699_d_n8, assign70140_e106699_d_n9, assign70140_e106699_d_n10, assign70140_e106699_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
        locals.var_dps0 = assign70140_e106699;
        locals.var_dps0_dn0 = assign70140_e106699_d_n0;
        locals.var_dps0_dn2 = assign70140_e106699_d_n2;
        locals.var_dps0_dn4 = assign70140_e106699_d_n4;
        locals.var_dps0_dn5 = assign70140_e106699_d_n5;
        locals.var_dps0_dn6 = assign70140_e106699_d_n6;
        locals.var_dps0_dn7 = assign70140_e106699_d_n7;
        locals.var_dps0_dn8 = assign70140_e106699_d_n8;
        locals.var_dps0_dn9 = assign70140_e106699_d_n9;
        locals.var_dps0_dn10 = assign70140_e106699_d_n10;
        locals.var_dps0_dn13 = assign70140_e106699_d_n13;

        let (assign70150_e106703, assign70150_e106703_d_n0, assign70150_e106703_d_n2, assign70150_e106703_d_n4, assign70150_e106703_d_n5, assign70150_e106703_d_n6, assign70150_e106703_d_n7, assign70150_e106703_d_n8, assign70150_e106703_d_n9, assign70150_e106703_d_n10, assign70150_e106703_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
        locals.var_fs0_dps0 = assign70150_e106703;
        locals.var_fs0_dps0_dn0 = assign70150_e106703_d_n0;
        locals.var_fs0_dps0_dn2 = assign70150_e106703_d_n2;
        locals.var_fs0_dps0_dn4 = assign70150_e106703_d_n4;
        locals.var_fs0_dps0_dn5 = assign70150_e106703_d_n5;
        locals.var_fs0_dps0_dn6 = assign70150_e106703_d_n6;
        locals.var_fs0_dps0_dn7 = assign70150_e106703_d_n7;
        locals.var_fs0_dps0_dn8 = assign70150_e106703_d_n8;
        locals.var_fs0_dps0_dn9 = assign70150_e106703_d_n9;
        locals.var_fs0_dps0_dn10 = assign70150_e106703_d_n10;
        locals.var_fs0_dps0_dn13 = assign70150_e106703_d_n13;

        let (assign70160_e106707, assign70160_e106707_d_n0, assign70160_e106707_d_n2, assign70160_e106707_d_n4, assign70160_e106707_d_n5, assign70160_e106707_d_n6, assign70160_e106707_d_n7, assign70160_e106707_d_n8, assign70160_e106707_d_n9, assign70160_e106707_d_n10, assign70160_e106707_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
        locals.var_fs02_dps0 = assign70160_e106707;
        locals.var_fs02_dps0_dn0 = assign70160_e106707_d_n0;
        locals.var_fs02_dps0_dn2 = assign70160_e106707_d_n2;
        locals.var_fs02_dps0_dn4 = assign70160_e106707_d_n4;
        locals.var_fs02_dps0_dn5 = assign70160_e106707_d_n5;
        locals.var_fs02_dps0_dn6 = assign70160_e106707_d_n6;
        locals.var_fs02_dps0_dn7 = assign70160_e106707_d_n7;
        locals.var_fs02_dps0_dn8 = assign70160_e106707_d_n8;
        locals.var_fs02_dps0_dn9 = assign70160_e106707_d_n9;
        locals.var_fs02_dps0_dn10 = assign70160_e106707_d_n10;
        locals.var_fs02_dps0_dn13 = assign70160_e106707_d_n13;

        let (assign70170_e106711, assign70170_e106711_d_n0, assign70170_e106711_d_n2, assign70170_e106711_d_n4, assign70170_e106711_d_n5, assign70170_e106711_d_n6, assign70170_e106711_d_n7, assign70170_e106711_d_n8, assign70170_e106711_d_n9, assign70170_e106711_d_n10, assign70170_e106711_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
        locals.var_fb_dpss = assign70170_e106711;
        locals.var_fb_dpss_dn0 = assign70170_e106711_d_n0;
        locals.var_fb_dpss_dn2 = assign70170_e106711_d_n2;
        locals.var_fb_dpss_dn4 = assign70170_e106711_d_n4;
        locals.var_fb_dpss_dn5 = assign70170_e106711_d_n5;
        locals.var_fb_dpss_dn6 = assign70170_e106711_d_n6;
        locals.var_fb_dpss_dn7 = assign70170_e106711_d_n7;
        locals.var_fb_dpss_dn8 = assign70170_e106711_d_n8;
        locals.var_fb_dpss_dn9 = assign70170_e106711_d_n9;
        locals.var_fb_dpss_dn10 = assign70170_e106711_d_n10;
        locals.var_fb_dpss_dn13 = assign70170_e106711_d_n13;

        let (assign70180_e106715, assign70180_e106715_d_n0, assign70180_e106715_d_n2, assign70180_e106715_d_n4, assign70180_e106715_d_n5, assign70180_e106715_d_n6, assign70180_e106715_d_n7, assign70180_e106715_d_n8, assign70180_e106715_d_n9, assign70180_e106715_d_n10, assign70180_e106715_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
        locals.var_fs01_dps0 = assign70180_e106715;
        locals.var_fs01_dps0_dn0 = assign70180_e106715_d_n0;
        locals.var_fs01_dps0_dn2 = assign70180_e106715_d_n2;
        locals.var_fs01_dps0_dn4 = assign70180_e106715_d_n4;
        locals.var_fs01_dps0_dn5 = assign70180_e106715_d_n5;
        locals.var_fs01_dps0_dn6 = assign70180_e106715_d_n6;
        locals.var_fs01_dps0_dn7 = assign70180_e106715_d_n7;
        locals.var_fs01_dps0_dn8 = assign70180_e106715_d_n8;
        locals.var_fs01_dps0_dn9 = assign70180_e106715_d_n9;
        locals.var_fs01_dps0_dn10 = assign70180_e106715_d_n10;
        locals.var_fs01_dps0_dn13 = assign70180_e106715_d_n13;

        let (assign70190_e106719, assign70190_e106719_d_n0, assign70190_e106719_d_n2, assign70190_e106719_d_n4, assign70190_e106719_d_n5, assign70190_e106719_d_n6, assign70190_e106719_d_n7, assign70190_e106719_d_n8, assign70190_e106719_d_n9, assign70190_e106719_d_n10, assign70190_e106719_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign70190_e106719;
        locals.var_chi_1_dn0 = assign70190_e106719_d_n0;
        locals.var_chi_1_dn2 = assign70190_e106719_d_n2;
        locals.var_chi_1_dn4 = assign70190_e106719_d_n4;
        locals.var_chi_1_dn5 = assign70190_e106719_d_n5;
        locals.var_chi_1_dn6 = assign70190_e106719_d_n6;
        locals.var_chi_1_dn7 = assign70190_e106719_d_n7;
        locals.var_chi_1_dn8 = assign70190_e106719_d_n8;
        locals.var_chi_1_dn9 = assign70190_e106719_d_n9;
        locals.var_chi_1_dn10 = assign70190_e106719_d_n10;
        locals.var_chi_1_dn13 = assign70190_e106719_d_n13;

        let (assign70200_e106723, assign70200_e106723_d_n0, assign70200_e106723_d_n2, assign70200_e106723_d_n4, assign70200_e106723_d_n5, assign70200_e106723_d_n6, assign70200_e106723_d_n7, assign70200_e106723_d_n8, assign70200_e106723_d_n9, assign70200_e106723_d_n10, assign70200_e106723_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign70200_e106723;
        locals.var_chi_a_dn0 = assign70200_e106723_d_n0;
        locals.var_chi_a_dn2 = assign70200_e106723_d_n2;
        locals.var_chi_a_dn4 = assign70200_e106723_d_n4;
        locals.var_chi_a_dn5 = assign70200_e106723_d_n5;
        locals.var_chi_a_dn6 = assign70200_e106723_d_n6;
        locals.var_chi_a_dn7 = assign70200_e106723_d_n7;
        locals.var_chi_a_dn8 = assign70200_e106723_d_n8;
        locals.var_chi_a_dn9 = assign70200_e106723_d_n9;
        locals.var_chi_a_dn10 = assign70200_e106723_d_n10;
        locals.var_chi_a_dn13 = assign70200_e106723_d_n13;

        let (assign70210_e106727, assign70210_e106727_d_n0, assign70210_e106727_d_n2, assign70210_e106727_d_n4, assign70210_e106727_d_n5, assign70210_e106727_d_n6, assign70210_e106727_d_n7, assign70210_e106727_d_n8, assign70210_e106727_d_n9, assign70210_e106727_d_n10, assign70210_e106727_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign70210_e106727;
        locals.var_chi_b_dn0 = assign70210_e106727_d_n0;
        locals.var_chi_b_dn2 = assign70210_e106727_d_n2;
        locals.var_chi_b_dn4 = assign70210_e106727_d_n4;
        locals.var_chi_b_dn5 = assign70210_e106727_d_n5;
        locals.var_chi_b_dn6 = assign70210_e106727_d_n6;
        locals.var_chi_b_dn7 = assign70210_e106727_d_n7;
        locals.var_chi_b_dn8 = assign70210_e106727_d_n8;
        locals.var_chi_b_dn9 = assign70210_e106727_d_n9;
        locals.var_chi_b_dn10 = assign70210_e106727_d_n10;
        locals.var_chi_b_dn13 = assign70210_e106727_d_n13;

        let (assign70220_e106732,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70220_e106730: f64 = (-1.0);
        (assign70220_e106730,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign70220_e106732;

        let (assign70230_e106736, assign70230_e106736_d_n0, assign70230_e106736_d_n2, assign70230_e106736_d_n4, assign70230_e106736_d_n5, assign70230_e106736_d_n6, assign70230_e106736_d_n7, assign70230_e106736_d_n8, assign70230_e106736_d_n9, assign70230_e106736_d_n10, assign70230_e106736_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn13,)
    }
};
        locals.var_ps0ld_ini = assign70230_e106736;
        locals.var_ps0ld_ini_dn0 = assign70230_e106736_d_n0;
        locals.var_ps0ld_ini_dn2 = assign70230_e106736_d_n2;
        locals.var_ps0ld_ini_dn4 = assign70230_e106736_d_n4;
        locals.var_ps0ld_ini_dn5 = assign70230_e106736_d_n5;
        locals.var_ps0ld_ini_dn6 = assign70230_e106736_d_n6;
        locals.var_ps0ld_ini_dn7 = assign70230_e106736_d_n7;
        locals.var_ps0ld_ini_dn8 = assign70230_e106736_d_n8;
        locals.var_ps0ld_ini_dn9 = assign70230_e106736_d_n9;
        locals.var_ps0ld_ini_dn10 = assign70230_e106736_d_n10;
        locals.var_ps0ld_ini_dn13 = assign70230_e106736_d_n13;

        let (assign70240_e106740, assign70240_e106740_d_n0, assign70240_e106740_d_n2, assign70240_e106740_d_n4, assign70240_e106740_d_n5, assign70240_e106740_d_n6, assign70240_e106740_d_n7, assign70240_e106740_d_n8, assign70240_e106740_d_n9, assign70240_e106740_d_n10, assign70240_e106740_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq, locals.var_fbsq_dn0, locals.var_fbsq_dn2, locals.var_fbsq_dn4, locals.var_fbsq_dn5, locals.var_fbsq_dn6, locals.var_fbsq_dn7, locals.var_fbsq_dn8, locals.var_fbsq_dn9, locals.var_fbsq_dn10, locals.var_fbsq_dn13,)
    }
};
        locals.var_fbsq = assign70240_e106740;
        locals.var_fbsq_dn0 = assign70240_e106740_d_n0;
        locals.var_fbsq_dn2 = assign70240_e106740_d_n2;
        locals.var_fbsq_dn4 = assign70240_e106740_d_n4;
        locals.var_fbsq_dn5 = assign70240_e106740_d_n5;
        locals.var_fbsq_dn6 = assign70240_e106740_d_n6;
        locals.var_fbsq_dn7 = assign70240_e106740_d_n7;
        locals.var_fbsq_dn8 = assign70240_e106740_d_n8;
        locals.var_fbsq_dn9 = assign70240_e106740_d_n9;
        locals.var_fbsq_dn10 = assign70240_e106740_d_n10;
        locals.var_fbsq_dn13 = assign70240_e106740_d_n13;

        let (assign70250_e106751, assign70250_e106751_d_n0, assign70250_e106751_d_n2, assign70250_e106751_d_n4, assign70250_e106751_d_n5, assign70250_e106751_d_n6, assign70250_e106751_d_n7, assign70250_e106751_d_n8, assign70250_e106751_d_n9, assign70250_e106751_d_n10, assign70250_e106751_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70250_e106744: f64 = (2.0 * locals.var_beta_inv);
        let assign70250_e106747: f64 = (locals.var_nover_func / locals.var_nin);
        let assign70250_e106748: f64 = (assign70250_e106747).ln();
        let assign70250_e106749: f64 = (assign70250_e106744 * assign70250_e106748);
        (assign70250_e106749, (((2.0 * locals.var_beta_inv_dn0) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn2) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn4) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn5) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn6) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn7) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn8) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn9) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn10) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))), (((2.0 * locals.var_beta_inv_dn13) * assign70250_e106748) + (assign70250_e106744 * ((-((locals.var_nover_func * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) / assign70250_e106747))),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn4, locals.var_pb2over_dn5, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn8, locals.var_pb2over_dn9, locals.var_pb2over_dn10, locals.var_pb2over_dn13,)
    }
};
        locals.var_pb2over = assign70250_e106751;
        locals.var_pb2over_dn0 = assign70250_e106751_d_n0;
        locals.var_pb2over_dn2 = assign70250_e106751_d_n2;
        locals.var_pb2over_dn4 = assign70250_e106751_d_n4;
        locals.var_pb2over_dn5 = assign70250_e106751_d_n5;
        locals.var_pb2over_dn6 = assign70250_e106751_d_n6;
        locals.var_pb2over_dn7 = assign70250_e106751_d_n7;
        locals.var_pb2over_dn8 = assign70250_e106751_d_n8;
        locals.var_pb2over_dn9 = assign70250_e106751_d_n9;
        locals.var_pb2over_dn10 = assign70250_e106751_d_n10;
        locals.var_pb2over_dn13 = assign70250_e106751_d_n13;

        let (assign70260_e106759, assign70260_e106759_d_n0, assign70260_e106759_d_n2, assign70260_e106759_d_n4, assign70260_e106759_d_n5, assign70260_e106759_d_n6, assign70260_e106759_d_n7, assign70260_e106759_d_n8, assign70260_e106759_d_n9, assign70260_e106759_d_n10, assign70260_e106759_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70260_e106755: f64 = (0.8 - locals.var_pb2over);
        let assign70260_e106757: f64 = (assign70260_e106755 - 0.1);
        (assign70260_e106757, (-locals.var_pb2over_dn0), (-locals.var_pb2over_dn2), (-locals.var_pb2over_dn4), (-locals.var_pb2over_dn5), (-locals.var_pb2over_dn6), (-locals.var_pb2over_dn7), (-locals.var_pb2over_dn8), (-locals.var_pb2over_dn9), (-locals.var_pb2over_dn10), (-locals.var_pb2over_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign70260_e106759;
        locals.var_tmf1_dn0 = assign70260_e106759_d_n0;
        locals.var_tmf1_dn2 = assign70260_e106759_d_n2;
        locals.var_tmf1_dn4 = assign70260_e106759_d_n4;
        locals.var_tmf1_dn5 = assign70260_e106759_d_n5;
        locals.var_tmf1_dn6 = assign70260_e106759_d_n6;
        locals.var_tmf1_dn7 = assign70260_e106759_d_n7;
        locals.var_tmf1_dn8 = assign70260_e106759_d_n8;
        locals.var_tmf1_dn9 = assign70260_e106759_d_n9;
        locals.var_tmf1_dn10 = assign70260_e106759_d_n10;
        locals.var_tmf1_dn13 = assign70260_e106759_d_n13;

        let (assign70270_e106767, assign70270_e106767_d_n0, assign70270_e106767_d_n2, assign70270_e106767_d_n4, assign70270_e106767_d_n5, assign70270_e106767_d_n6, assign70270_e106767_d_n7, assign70270_e106767_d_n8, assign70270_e106767_d_n9, assign70270_e106767_d_n10, assign70270_e106767_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70270_e106763: f64 = (4.0 * 0.8);
        let assign70270_e106765: f64 = (assign70270_e106763 * 0.1);
        (assign70270_e106765, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70270_e106767;
        locals.var_tmf2_dn0 = assign70270_e106767_d_n0;
        locals.var_tmf2_dn2 = assign70270_e106767_d_n2;
        locals.var_tmf2_dn4 = assign70270_e106767_d_n4;
        locals.var_tmf2_dn5 = assign70270_e106767_d_n5;
        locals.var_tmf2_dn6 = assign70270_e106767_d_n6;
        locals.var_tmf2_dn7 = assign70270_e106767_d_n7;
        locals.var_tmf2_dn8 = assign70270_e106767_d_n8;
        locals.var_tmf2_dn9 = assign70270_e106767_d_n9;
        locals.var_tmf2_dn10 = assign70270_e106767_d_n10;
        locals.var_tmf2_dn13 = assign70270_e106767_d_n13;

        let (assign70280_e106777, assign70280_e106777_d_n0, assign70280_e106777_d_n2, assign70280_e106777_d_n4, assign70280_e106777_d_n5, assign70280_e106777_d_n6, assign70280_e106777_d_n7, assign70280_e106777_d_n8, assign70280_e106777_d_n9, assign70280_e106777_d_n10, assign70280_e106777_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign70280_e106775, assign70280_e106775_d_n0, assign70280_e106775_d_n2, assign70280_e106775_d_n4, assign70280_e106775_d_n5, assign70280_e106775_d_n6, assign70280_e106775_d_n7, assign70280_e106775_d_n8, assign70280_e106775_d_n9, assign70280_e106775_d_n10, assign70280_e106775_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign70280_e106774: f64 = (-locals.var_tmf2);
                (assign70280_e106774, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign70280_e106775, assign70280_e106775_d_n0, assign70280_e106775_d_n2, assign70280_e106775_d_n4, assign70280_e106775_d_n5, assign70280_e106775_d_n6, assign70280_e106775_d_n7, assign70280_e106775_d_n8, assign70280_e106775_d_n9, assign70280_e106775_d_n10, assign70280_e106775_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70280_e106777;
        locals.var_tmf2_dn0 = assign70280_e106777_d_n0;
        locals.var_tmf2_dn2 = assign70280_e106777_d_n2;
        locals.var_tmf2_dn4 = assign70280_e106777_d_n4;
        locals.var_tmf2_dn5 = assign70280_e106777_d_n5;
        locals.var_tmf2_dn6 = assign70280_e106777_d_n6;
        locals.var_tmf2_dn7 = assign70280_e106777_d_n7;
        locals.var_tmf2_dn8 = assign70280_e106777_d_n8;
        locals.var_tmf2_dn9 = assign70280_e106777_d_n9;
        locals.var_tmf2_dn10 = assign70280_e106777_d_n10;
        locals.var_tmf2_dn13 = assign70280_e106777_d_n13;

        let (assign70290_e106786, assign70290_e106786_d_n0, assign70290_e106786_d_n2, assign70290_e106786_d_n4, assign70290_e106786_d_n5, assign70290_e106786_d_n6, assign70290_e106786_d_n7, assign70290_e106786_d_n8, assign70290_e106786_d_n9, assign70290_e106786_d_n10, assign70290_e106786_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70290_e106781: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign70290_e106783: f64 = (assign70290_e106781 + locals.var_tmf2);
        let assign70290_e106784: f64 = (assign70290_e106783).sqrt();
        (assign70290_e106784, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign70290_e106784)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign70290_e106784)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign70290_e106786;
        locals.var_tmf2_dn0 = assign70290_e106786_d_n0;
        locals.var_tmf2_dn2 = assign70290_e106786_d_n2;
        locals.var_tmf2_dn4 = assign70290_e106786_d_n4;
        locals.var_tmf2_dn5 = assign70290_e106786_d_n5;
        locals.var_tmf2_dn6 = assign70290_e106786_d_n6;
        locals.var_tmf2_dn7 = assign70290_e106786_d_n7;
        locals.var_tmf2_dn8 = assign70290_e106786_d_n8;
        locals.var_tmf2_dn9 = assign70290_e106786_d_n9;
        locals.var_tmf2_dn10 = assign70290_e106786_d_n10;
        locals.var_tmf2_dn13 = assign70290_e106786_d_n13;

    }
}
