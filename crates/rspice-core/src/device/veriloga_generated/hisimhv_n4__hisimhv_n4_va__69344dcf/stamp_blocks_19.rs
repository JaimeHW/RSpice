#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_304(
        locals: &mut StampLocals,
    ) {
        let (assign86530_e132185, assign86530_e132185_d_n0, assign86530_e132185_d_n2, assign86530_e132185_d_n4, assign86530_e132185_d_n5, assign86530_e132185_d_n6, assign86530_e132185_d_n7, assign86530_e132185_d_n8, assign86530_e132185_d_n9, assign86530_e132185_d_n10, assign86530_e132185_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86530_e132181: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign86530_e132182: f64 = (0.5 * assign86530_e132181);
        let assign86530_e132183: f64 = (0.5 + assign86530_e132182);
        (assign86530_e132183, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86530_e132185;
        locals.var_t1_dn0 = assign86530_e132185_d_n0;
        locals.var_t1_dn2 = assign86530_e132185_d_n2;
        locals.var_t1_dn4 = assign86530_e132185_d_n4;
        locals.var_t1_dn5 = assign86530_e132185_d_n5;
        locals.var_t1_dn6 = assign86530_e132185_d_n6;
        locals.var_t1_dn7 = assign86530_e132185_d_n7;
        locals.var_t1_dn8 = assign86530_e132185_d_n8;
        locals.var_t1_dn9 = assign86530_e132185_d_n9;
        locals.var_t1_dn10 = assign86530_e132185_d_n10;
        locals.var_t1_dn13 = assign86530_e132185_d_n13;

        let assign86540_e132188: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86540_e132191: f64 = (-locals.var_t1);
        let assign86540_e132196: f64 = if ((assign86540_e132188 > assign86540_e132191) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2022 = assign86540_e132196;

        let (assign86550_e132210, assign86550_e132210_d_n0, assign86550_e132210_d_n2, assign86550_e132210_d_n4, assign86550_e132210_d_n5, assign86550_e132210_d_n6, assign86550_e132210_d_n7, assign86550_e132210_d_n8, assign86550_e132210_d_n9, assign86550_e132210_d_n10, assign86550_e132210_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86550_e132204: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86550_e132206: f64 = assign86550_e132204;
        let assign86550_e132208: f64 = (assign86550_e132206 + locals.var_t1);
        (assign86550_e132208, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), ((locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6) + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), (locals.var_vxbgmtcl_dn9 + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn13 + locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign86550_e132210;
        locals.var_tmf1_dn0 = assign86550_e132210_d_n0;
        locals.var_tmf1_dn2 = assign86550_e132210_d_n2;
        locals.var_tmf1_dn4 = assign86550_e132210_d_n4;
        locals.var_tmf1_dn5 = assign86550_e132210_d_n5;
        locals.var_tmf1_dn6 = assign86550_e132210_d_n6;
        locals.var_tmf1_dn7 = assign86550_e132210_d_n7;
        locals.var_tmf1_dn8 = assign86550_e132210_d_n8;
        locals.var_tmf1_dn9 = assign86550_e132210_d_n9;
        locals.var_tmf1_dn10 = assign86550_e132210_d_n10;
        locals.var_tmf1_dn13 = assign86550_e132210_d_n13;

        let (assign86560_e132220, assign86560_e132220_d_n0, assign86560_e132220_d_n2, assign86560_e132220_d_n4, assign86560_e132220_d_n5, assign86560_e132220_d_n6, assign86560_e132220_d_n7, assign86560_e132220_d_n8, assign86560_e132220_d_n9, assign86560_e132220_d_n10, assign86560_e132220_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86560_e132218: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign86560_e132218, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign86560_e132220;
        locals.var_x2_dn0 = assign86560_e132220_d_n0;
        locals.var_x2_dn2 = assign86560_e132220_d_n2;
        locals.var_x2_dn4 = assign86560_e132220_d_n4;
        locals.var_x2_dn5 = assign86560_e132220_d_n5;
        locals.var_x2_dn6 = assign86560_e132220_d_n6;
        locals.var_x2_dn7 = assign86560_e132220_d_n7;
        locals.var_x2_dn8 = assign86560_e132220_d_n8;
        locals.var_x2_dn9 = assign86560_e132220_d_n9;
        locals.var_x2_dn10 = assign86560_e132220_d_n10;
        locals.var_x2_dn13 = assign86560_e132220_d_n13;

        let (assign86570_e132230, assign86570_e132230_d_n0, assign86570_e132230_d_n2, assign86570_e132230_d_n4, assign86570_e132230_d_n5, assign86570_e132230_d_n6, assign86570_e132230_d_n7, assign86570_e132230_d_n8, assign86570_e132230_d_n9, assign86570_e132230_d_n10, assign86570_e132230_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86570_e132228: f64 = (locals.var_t1 * locals.var_t1);
        (assign86570_e132228, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign86570_e132230;
        locals.var_xmax2_dn0 = assign86570_e132230_d_n0;
        locals.var_xmax2_dn2 = assign86570_e132230_d_n2;
        locals.var_xmax2_dn4 = assign86570_e132230_d_n4;
        locals.var_xmax2_dn5 = assign86570_e132230_d_n5;
        locals.var_xmax2_dn6 = assign86570_e132230_d_n6;
        locals.var_xmax2_dn7 = assign86570_e132230_d_n7;
        locals.var_xmax2_dn8 = assign86570_e132230_d_n8;
        locals.var_xmax2_dn9 = assign86570_e132230_d_n9;
        locals.var_xmax2_dn10 = assign86570_e132230_d_n10;
        locals.var_xmax2_dn13 = assign86570_e132230_d_n13;

        let (assign86580_e132238, assign86580_e132238_d_n0, assign86580_e132238_d_n2, assign86580_e132238_d_n4, assign86580_e132238_d_n5, assign86580_e132238_d_n6, assign86580_e132238_d_n7, assign86580_e132238_d_n8, assign86580_e132238_d_n9, assign86580_e132238_d_n10, assign86580_e132238_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign86580_e132238;
        locals.var_xp_dn0 = assign86580_e132238_d_n0;
        locals.var_xp_dn2 = assign86580_e132238_d_n2;
        locals.var_xp_dn4 = assign86580_e132238_d_n4;
        locals.var_xp_dn5 = assign86580_e132238_d_n5;
        locals.var_xp_dn6 = assign86580_e132238_d_n6;
        locals.var_xp_dn7 = assign86580_e132238_d_n7;
        locals.var_xp_dn8 = assign86580_e132238_d_n8;
        locals.var_xp_dn9 = assign86580_e132238_d_n9;
        locals.var_xp_dn10 = assign86580_e132238_d_n10;
        locals.var_xp_dn13 = assign86580_e132238_d_n13;

        let (assign86590_e132246, assign86590_e132246_d_n0, assign86590_e132246_d_n2, assign86590_e132246_d_n4, assign86590_e132246_d_n5, assign86590_e132246_d_n6, assign86590_e132246_d_n7, assign86590_e132246_d_n8, assign86590_e132246_d_n9, assign86590_e132246_d_n10, assign86590_e132246_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign86590_e132246;
        locals.var_xmp_dn0 = assign86590_e132246_d_n0;
        locals.var_xmp_dn2 = assign86590_e132246_d_n2;
        locals.var_xmp_dn4 = assign86590_e132246_d_n4;
        locals.var_xmp_dn5 = assign86590_e132246_d_n5;
        locals.var_xmp_dn6 = assign86590_e132246_d_n6;
        locals.var_xmp_dn7 = assign86590_e132246_d_n7;
        locals.var_xmp_dn8 = assign86590_e132246_d_n8;
        locals.var_xmp_dn9 = assign86590_e132246_d_n9;
        locals.var_xmp_dn10 = assign86590_e132246_d_n10;
        locals.var_xmp_dn13 = assign86590_e132246_d_n13;

        let (assign86600_e132254,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign86600_e132254;

        let (assign86610_e132262,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86610_e132262;

        let (assign86620_e132270, assign86620_e132270_d_n0, assign86620_e132270_d_n2, assign86620_e132270_d_n4, assign86620_e132270_d_n5, assign86620_e132270_d_n6, assign86620_e132270_d_n7, assign86620_e132270_d_n8, assign86620_e132270_d_n9, assign86620_e132270_d_n10, assign86620_e132270_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign86620_e132270;
        locals.var_arg_dn0 = assign86620_e132270_d_n0;
        locals.var_arg_dn2 = assign86620_e132270_d_n2;
        locals.var_arg_dn4 = assign86620_e132270_d_n4;
        locals.var_arg_dn5 = assign86620_e132270_d_n5;
        locals.var_arg_dn6 = assign86620_e132270_d_n6;
        locals.var_arg_dn7 = assign86620_e132270_d_n7;
        locals.var_arg_dn8 = assign86620_e132270_d_n8;
        locals.var_arg_dn9 = assign86620_e132270_d_n9;
        locals.var_arg_dn10 = assign86620_e132270_d_n10;
        locals.var_arg_dn13 = assign86620_e132270_d_n13;

        let (assign86630_e132278, assign86630_e132278_d_n0, assign86630_e132278_d_n2, assign86630_e132278_d_n4, assign86630_e132278_d_n5, assign86630_e132278_d_n6, assign86630_e132278_d_n7, assign86630_e132278_d_n8, assign86630_e132278_d_n9, assign86630_e132278_d_n10, assign86630_e132278_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign86630_e132278;
        locals.var_dnm_dn0 = assign86630_e132278_d_n0;
        locals.var_dnm_dn2 = assign86630_e132278_d_n2;
        locals.var_dnm_dn4 = assign86630_e132278_d_n4;
        locals.var_dnm_dn5 = assign86630_e132278_d_n5;
        locals.var_dnm_dn6 = assign86630_e132278_d_n6;
        locals.var_dnm_dn7 = assign86630_e132278_d_n7;
        locals.var_dnm_dn8 = assign86630_e132278_d_n8;
        locals.var_dnm_dn9 = assign86630_e132278_d_n9;
        locals.var_dnm_dn10 = assign86630_e132278_d_n10;
        locals.var_dnm_dn13 = assign86630_e132278_d_n13;

        let (assign86640_e132288, assign86640_e132288_d_n0, assign86640_e132288_d_n2, assign86640_e132288_d_n4, assign86640_e132288_d_n5, assign86640_e132288_d_n6, assign86640_e132288_d_n7, assign86640_e132288_d_n8, assign86640_e132288_d_n9, assign86640_e132288_d_n10, assign86640_e132288_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86640_e132286: f64 = (locals.var_xp * locals.var_x2);
        (assign86640_e132286, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign86640_e132288;
        locals.var_xp_dn0 = assign86640_e132288_d_n0;
        locals.var_xp_dn2 = assign86640_e132288_d_n2;
        locals.var_xp_dn4 = assign86640_e132288_d_n4;
        locals.var_xp_dn5 = assign86640_e132288_d_n5;
        locals.var_xp_dn6 = assign86640_e132288_d_n6;
        locals.var_xp_dn7 = assign86640_e132288_d_n7;
        locals.var_xp_dn8 = assign86640_e132288_d_n8;
        locals.var_xp_dn9 = assign86640_e132288_d_n9;
        locals.var_xp_dn10 = assign86640_e132288_d_n10;
        locals.var_xp_dn13 = assign86640_e132288_d_n13;

        let (assign86650_e132298, assign86650_e132298_d_n0, assign86650_e132298_d_n2, assign86650_e132298_d_n4, assign86650_e132298_d_n5, assign86650_e132298_d_n6, assign86650_e132298_d_n7, assign86650_e132298_d_n8, assign86650_e132298_d_n9, assign86650_e132298_d_n10, assign86650_e132298_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86650_e132296: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign86650_e132296, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign86650_e132298;
        locals.var_xmp_dn0 = assign86650_e132298_d_n0;
        locals.var_xmp_dn2 = assign86650_e132298_d_n2;
        locals.var_xmp_dn4 = assign86650_e132298_d_n4;
        locals.var_xmp_dn5 = assign86650_e132298_d_n5;
        locals.var_xmp_dn6 = assign86650_e132298_d_n6;
        locals.var_xmp_dn7 = assign86650_e132298_d_n7;
        locals.var_xmp_dn8 = assign86650_e132298_d_n8;
        locals.var_xmp_dn9 = assign86650_e132298_d_n9;
        locals.var_xmp_dn10 = assign86650_e132298_d_n10;
        locals.var_xmp_dn13 = assign86650_e132298_d_n13;

        let (assign86660_e132308, assign86660_e132308_d_n0, assign86660_e132308_d_n2, assign86660_e132308_d_n4, assign86660_e132308_d_n5, assign86660_e132308_d_n6, assign86660_e132308_d_n7, assign86660_e132308_d_n8, assign86660_e132308_d_n9, assign86660_e132308_d_n10, assign86660_e132308_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86660_e132306: f64 = (locals.var_xp + locals.var_xmp);
        (assign86660_e132306, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign86660_e132308;
        locals.var_arg_dn0 = assign86660_e132308_d_n0;
        locals.var_arg_dn2 = assign86660_e132308_d_n2;
        locals.var_arg_dn4 = assign86660_e132308_d_n4;
        locals.var_arg_dn5 = assign86660_e132308_d_n5;
        locals.var_arg_dn6 = assign86660_e132308_d_n6;
        locals.var_arg_dn7 = assign86660_e132308_d_n7;
        locals.var_arg_dn8 = assign86660_e132308_d_n8;
        locals.var_arg_dn9 = assign86660_e132308_d_n9;
        locals.var_arg_dn10 = assign86660_e132308_d_n10;
        locals.var_arg_dn13 = assign86660_e132308_d_n13;

        let (assign86670_e132316, assign86670_e132316_d_n0, assign86670_e132316_d_n2, assign86670_e132316_d_n4, assign86670_e132316_d_n5, assign86670_e132316_d_n6, assign86670_e132316_d_n7, assign86670_e132316_d_n8, assign86670_e132316_d_n9, assign86670_e132316_d_n10, assign86670_e132316_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign86670_e132316;
        locals.var_dnm_dn0 = assign86670_e132316_d_n0;
        locals.var_dnm_dn2 = assign86670_e132316_d_n2;
        locals.var_dnm_dn4 = assign86670_e132316_d_n4;
        locals.var_dnm_dn5 = assign86670_e132316_d_n5;
        locals.var_dnm_dn6 = assign86670_e132316_d_n6;
        locals.var_dnm_dn7 = assign86670_e132316_d_n7;
        locals.var_dnm_dn8 = assign86670_e132316_d_n8;
        locals.var_dnm_dn9 = assign86670_e132316_d_n9;
        locals.var_dnm_dn10 = assign86670_e132316_d_n10;
        locals.var_dnm_dn13 = assign86670_e132316_d_n13;

        let assign86680_e132331: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2023 = assign86680_e132331;

        let assign86690_e132334: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2024 = assign86690_e132334;

        let (assign86700_e132346,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_guard2024 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86700_e132346;

        let assign86710_e132349: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2025 = assign86710_e132349;

        let (assign86720_e132364,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_guard2024 == 0.0)) && (locals.var_guard2025 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86720_e132364;

        let assign86730_e132367: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2026 = assign86730_e132367;

        let (assign86740_e132385,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_guard2024 == 0.0)) && (locals.var_guard2025 == 0.0)) && (locals.var_guard2026 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86740_e132385;

        let assign86750_e132388: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2027 = assign86750_e132388;

        let (assign86760_e132409,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_guard2024 == 0.0)) && (locals.var_guard2025 == 0.0)) && (locals.var_guard2026 == 0.0)) && (locals.var_guard2027 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign86760_e132409;

        let (assign86770_e132419,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign86770_e132419;

        let mut assign86780_loop_guard: usize = 0;
        while {
            let assign86780_cond_e132430: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign86780_cond_e132430 != 0.0
        } {
            assign86780_loop_guard += 1;
            assert!(assign86780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign86780_body0_e132441, assign86780_body0_e132441_d_n0, assign86780_body0_e132441_d_n2, assign86780_body0_e132441_d_n4, assign86780_body0_e132441_d_n5, assign86780_body0_e132441_d_n6, assign86780_body0_e132441_d_n7, assign86780_body0_e132441_d_n8, assign86780_body0_e132441_d_n9, assign86780_body0_e132441_d_n10, assign86780_body0_e132441_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) {
        let assign86780_body0_e132439: f64 = (locals.var_dnm).sqrt();
        (assign86780_body0_e132439, (locals.var_dnm_dn0 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn2 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn4 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn5 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn6 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn7 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn8 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn9 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn10 / (2.0 * assign86780_body0_e132439)), (locals.var_dnm_dn13 / (2.0 * assign86780_body0_e132439)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign86780_body0_e132441;
            locals.var_dnm_dn0 = assign86780_body0_e132441_d_n0;
            locals.var_dnm_dn2 = assign86780_body0_e132441_d_n2;
            locals.var_dnm_dn4 = assign86780_body0_e132441_d_n4;
            locals.var_dnm_dn5 = assign86780_body0_e132441_d_n5;
            locals.var_dnm_dn6 = assign86780_body0_e132441_d_n6;
            locals.var_dnm_dn7 = assign86780_body0_e132441_d_n7;
            locals.var_dnm_dn8 = assign86780_body0_e132441_d_n8;
            locals.var_dnm_dn9 = assign86780_body0_e132441_d_n9;
            locals.var_dnm_dn10 = assign86780_body0_e132441_d_n10;
            locals.var_dnm_dn13 = assign86780_body0_e132441_d_n13;
            let (assign86780_body1_e132453,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 != 0.0)) {
        let assign86780_body1_e132451: f64 = (locals.var_m0 + 1.0);
        (assign86780_body1_e132451,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign86780_body1_e132453;
        }

        let (assign86790_e132475, assign86790_e132475_d_n0, assign86790_e132475_d_n2, assign86790_e132475_d_n4, assign86790_e132475_d_n5, assign86790_e132475_d_n6, assign86790_e132475_d_n7, assign86790_e132475_d_n8, assign86790_e132475_d_n9, assign86790_e132475_d_n10, assign86790_e132475_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) && (locals.var_guard2023 == 0.0)) {
        let (assign86790_e132473, assign86790_e132473_d_n0, assign86790_e132473_d_n2, assign86790_e132473_d_n4, assign86790_e132473_d_n5, assign86790_e132473_d_n6, assign86790_e132473_d_n7, assign86790_e132473_d_n8, assign86790_e132473_d_n9, assign86790_e132473_d_n10, assign86790_e132473_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign86790_e132470: f64 = 2.0;
                let assign86790_e132471: f64 = (1.0 / assign86790_e132470);
                let assign86790_e132472: f64 = (locals.var_dnm).powf(assign86790_e132471);
                (assign86790_e132472, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn0)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn2)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn4)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn5)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn6)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn7)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn8)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn9)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn10)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign86790_e132471) as f64).is_finite() && ((assign86790_e132471) as f64).fract() == 0.0 { if assign86790_e132471 == 0.0 { 0.0 } else { (assign86790_e132471 * ((locals.var_dnm).powf(assign86790_e132471 - 1.0) * locals.var_dnm_dn13)) } } else { (assign86790_e132472 * (assign86790_e132471 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign86790_e132473, assign86790_e132473_d_n0, assign86790_e132473_d_n2, assign86790_e132473_d_n4, assign86790_e132473_d_n5, assign86790_e132473_d_n6, assign86790_e132473_d_n7, assign86790_e132473_d_n8, assign86790_e132473_d_n9, assign86790_e132473_d_n10, assign86790_e132473_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign86790_e132475;
        locals.var_dnm_dn0 = assign86790_e132475_d_n0;
        locals.var_dnm_dn2 = assign86790_e132475_d_n2;
        locals.var_dnm_dn4 = assign86790_e132475_d_n4;
        locals.var_dnm_dn5 = assign86790_e132475_d_n5;
        locals.var_dnm_dn6 = assign86790_e132475_d_n6;
        locals.var_dnm_dn7 = assign86790_e132475_d_n7;
        locals.var_dnm_dn8 = assign86790_e132475_d_n8;
        locals.var_dnm_dn9 = assign86790_e132475_d_n9;
        locals.var_dnm_dn10 = assign86790_e132475_d_n10;
        locals.var_dnm_dn13 = assign86790_e132475_d_n13;

        let (assign86800_e132485, assign86800_e132485_d_n0, assign86800_e132485_d_n2, assign86800_e132485_d_n4, assign86800_e132485_d_n5, assign86800_e132485_d_n6, assign86800_e132485_d_n7, assign86800_e132485_d_n8, assign86800_e132485_d_n9, assign86800_e132485_d_n10, assign86800_e132485_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86800_e132483: f64 = (1.0 / locals.var_dnm);
        (assign86800_e132483, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign86800_e132485;
        locals.var_dnm_dn0 = assign86800_e132485_d_n0;
        locals.var_dnm_dn2 = assign86800_e132485_d_n2;
        locals.var_dnm_dn4 = assign86800_e132485_d_n4;
        locals.var_dnm_dn5 = assign86800_e132485_d_n5;
        locals.var_dnm_dn6 = assign86800_e132485_d_n6;
        locals.var_dnm_dn7 = assign86800_e132485_d_n7;
        locals.var_dnm_dn8 = assign86800_e132485_d_n8;
        locals.var_dnm_dn9 = assign86800_e132485_d_n9;
        locals.var_dnm_dn10 = assign86800_e132485_d_n10;
        locals.var_dnm_dn13 = assign86800_e132485_d_n13;

        let (assign86810_e132497, assign86810_e132497_d_n0, assign86810_e132497_d_n2, assign86810_e132497_d_n4, assign86810_e132497_d_n5, assign86810_e132497_d_n6, assign86810_e132497_d_n7, assign86810_e132497_d_n8, assign86810_e132497_d_n9, assign86810_e132497_d_n10, assign86810_e132497_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86810_e132493: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign86810_e132495: f64 = (assign86810_e132493 * locals.var_dnm);
        (assign86810_e132495, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn13)) * locals.var_dnm) + (assign86810_e132493 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign86810_e132497;
        locals.var_tmf0_dn0 = assign86810_e132497_d_n0;
        locals.var_tmf0_dn2 = assign86810_e132497_d_n2;
        locals.var_tmf0_dn4 = assign86810_e132497_d_n4;
        locals.var_tmf0_dn5 = assign86810_e132497_d_n5;
        locals.var_tmf0_dn6 = assign86810_e132497_d_n6;
        locals.var_tmf0_dn7 = assign86810_e132497_d_n7;
        locals.var_tmf0_dn8 = assign86810_e132497_d_n8;
        locals.var_tmf0_dn9 = assign86810_e132497_d_n9;
        locals.var_tmf0_dn10 = assign86810_e132497_d_n10;
        locals.var_tmf0_dn13 = assign86810_e132497_d_n13;

        let (assign86820_e132511, assign86820_e132511_d_n0, assign86820_e132511_d_n2, assign86820_e132511_d_n4, assign86820_e132511_d_n5, assign86820_e132511_d_n6, assign86820_e132511_d_n7, assign86820_e132511_d_n8, assign86820_e132511_d_n9, assign86820_e132511_d_n10, assign86820_e132511_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86820_e132505: f64 = (locals.var_t1 * locals.var_xmp);
        let assign86820_e132507: f64 = (assign86820_e132505 * locals.var_dnm);
        let assign86820_e132509: f64 = (assign86820_e132507 / locals.var_arg);
        (assign86820_e132509, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn0)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn2)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn4)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn5)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn6)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn7)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn8)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn9)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn10)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn13 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign86820_e132505 * locals.var_dnm_dn13)) * locals.var_arg) - (assign86820_e132507 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86820_e132511;
        locals.var_t0_dn0 = assign86820_e132511_d_n0;
        locals.var_t0_dn2 = assign86820_e132511_d_n2;
        locals.var_t0_dn4 = assign86820_e132511_d_n4;
        locals.var_t0_dn5 = assign86820_e132511_d_n5;
        locals.var_t0_dn6 = assign86820_e132511_d_n6;
        locals.var_t0_dn7 = assign86820_e132511_d_n7;
        locals.var_t0_dn8 = assign86820_e132511_d_n8;
        locals.var_t0_dn9 = assign86820_e132511_d_n9;
        locals.var_t0_dn10 = assign86820_e132511_d_n10;
        locals.var_t0_dn13 = assign86820_e132511_d_n13;

        let (assign86830_e132523, assign86830_e132523_d_n0, assign86830_e132523_d_n2, assign86830_e132523_d_n4, assign86830_e132523_d_n5, assign86830_e132523_d_n6, assign86830_e132523_d_n7, assign86830_e132523_d_n8, assign86830_e132523_d_n9, assign86830_e132523_d_n10, assign86830_e132523_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        let assign86830_e132519: f64 = (-locals.var_t1);
        let assign86830_e132521: f64 = (assign86830_e132519 + locals.var_tmf0);
        (assign86830_e132521, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86830_e132523;
        locals.var_t1_dn0 = assign86830_e132523_d_n0;
        locals.var_t1_dn2 = assign86830_e132523_d_n2;
        locals.var_t1_dn4 = assign86830_e132523_d_n4;
        locals.var_t1_dn5 = assign86830_e132523_d_n5;
        locals.var_t1_dn6 = assign86830_e132523_d_n6;
        locals.var_t1_dn7 = assign86830_e132523_d_n7;
        locals.var_t1_dn8 = assign86830_e132523_d_n8;
        locals.var_t1_dn9 = assign86830_e132523_d_n9;
        locals.var_t1_dn10 = assign86830_e132523_d_n10;
        locals.var_t1_dn13 = assign86830_e132523_d_n13;

        let (assign86840_e132531, assign86840_e132531_d_n0, assign86840_e132531_d_n2, assign86840_e132531_d_n4, assign86840_e132531_d_n5, assign86840_e132531_d_n6, assign86840_e132531_d_n7, assign86840_e132531_d_n8, assign86840_e132531_d_n9, assign86840_e132531_d_n10, assign86840_e132531_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86840_e132531;
        locals.var_t0_dn0 = assign86840_e132531_d_n0;
        locals.var_t0_dn2 = assign86840_e132531_d_n2;
        locals.var_t0_dn4 = assign86840_e132531_d_n4;
        locals.var_t0_dn5 = assign86840_e132531_d_n5;
        locals.var_t0_dn6 = assign86840_e132531_d_n6;
        locals.var_t0_dn7 = assign86840_e132531_d_n7;
        locals.var_t0_dn8 = assign86840_e132531_d_n8;
        locals.var_t0_dn9 = assign86840_e132531_d_n9;
        locals.var_t0_dn10 = assign86840_e132531_d_n10;
        locals.var_t0_dn13 = assign86840_e132531_d_n13;

        let (assign86850_e132542, assign86850_e132542_d_n0, assign86850_e132542_d_n2, assign86850_e132542_d_n4, assign86850_e132542_d_n5, assign86850_e132542_d_n6, assign86850_e132542_d_n7, assign86850_e132542_d_n8, assign86850_e132542_d_n9, assign86850_e132542_d_n10, assign86850_e132542_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 == 0.0)) {
        let assign86850_e132540: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign86850_e132540, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86850_e132542;
        locals.var_t1_dn0 = assign86850_e132542_d_n0;
        locals.var_t1_dn2 = assign86850_e132542_d_n2;
        locals.var_t1_dn4 = assign86850_e132542_d_n4;
        locals.var_t1_dn5 = assign86850_e132542_d_n5;
        locals.var_t1_dn6 = assign86850_e132542_d_n6;
        locals.var_t1_dn7 = assign86850_e132542_d_n7;
        locals.var_t1_dn8 = assign86850_e132542_d_n8;
        locals.var_t1_dn9 = assign86850_e132542_d_n9;
        locals.var_t1_dn10 = assign86850_e132542_d_n10;
        locals.var_t1_dn13 = assign86850_e132542_d_n13;

    }

    pub(super) fn stamp_transient_block_305(
        locals: &mut StampLocals,
    ) {
        let (assign86860_e132551, assign86860_e132551_d_n0, assign86860_e132551_d_n2, assign86860_e132551_d_n4, assign86860_e132551_d_n5, assign86860_e132551_d_n6, assign86860_e132551_d_n7, assign86860_e132551_d_n8, assign86860_e132551_d_n9, assign86860_e132551_d_n10, assign86860_e132551_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard2022 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86860_e132551;
        locals.var_t0_dn0 = assign86860_e132551_d_n0;
        locals.var_t0_dn2 = assign86860_e132551_d_n2;
        locals.var_t0_dn4 = assign86860_e132551_d_n4;
        locals.var_t0_dn5 = assign86860_e132551_d_n5;
        locals.var_t0_dn6 = assign86860_e132551_d_n6;
        locals.var_t0_dn7 = assign86860_e132551_d_n7;
        locals.var_t0_dn8 = assign86860_e132551_d_n8;
        locals.var_t0_dn9 = assign86860_e132551_d_n9;
        locals.var_t0_dn10 = assign86860_e132551_d_n10;
        locals.var_t0_dn13 = assign86860_e132551_d_n13;

        let (assign86870_e132559, assign86870_e132559_d_n0, assign86870_e132559_d_n2, assign86870_e132559_d_n4, assign86870_e132559_d_n5, assign86870_e132559_d_n6, assign86870_e132559_d_n7, assign86870_e132559_d_n8, assign86870_e132559_d_n9, assign86870_e132559_d_n10, assign86870_e132559_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86870_e132557: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign86870_e132557, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, (locals.var_t1_dn6 - locals.var_vgpld_dn6), (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign86870_e132559;
        locals.var_vxbgmtcl_dn0 = assign86870_e132559_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86870_e132559_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86870_e132559_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86870_e132559_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86870_e132559_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86870_e132559_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86870_e132559_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86870_e132559_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86870_e132559_d_n10;
        locals.var_vxbgmtcl_dn13 = assign86870_e132559_d_n13;

        let (assign86880_e132570,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86880_e132564: f64 = (-locals.var_vxbgmtcl);
        let assign86880_e132567: f64 = (10.0 * 2.220446049250313e-16);
        let assign86880_e132568: f64 = (assign86880_e132564 + assign86880_e132567);
        (assign86880_e132568,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign86880_e132570;

        let assign86890_e132573: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard2028 = assign86890_e132573;

        let (assign86910_e132594, assign86910_e132594_d_n0, assign86910_e132594_d_n2, assign86910_e132594_d_n4, assign86910_e132594_d_n5, assign86910_e132594_d_n6, assign86910_e132594_d_n7, assign86910_e132594_d_n8, assign86910_e132594_d_n9, assign86910_e132594_d_n10, assign86910_e132594_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86910_e132586: f64 = (2.0 * locals.var_beta_inv);
        let assign86910_e132588: f64 = (-locals.var_vgs_min);
        let assign86910_e132590: f64 = (assign86910_e132588 / locals.var_fac1);
        let assign86910_e132591: f64 = (assign86910_e132590).ln();
        let assign86910_e132592: f64 = (assign86910_e132586 * assign86910_e132591);
        (assign86910_e132592, (((2.0 * locals.var_beta_inv_dn0) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn2) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn4) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn5) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn6) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn7) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn8) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn9) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn10) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))), (((2.0 * locals.var_beta_inv_dn13) * assign86910_e132591) + (assign86910_e132586 * ((-((assign86910_e132588 * locals.var_fac1_dn13) / (locals.var_fac1 * locals.var_fac1))) / assign86910_e132590))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn13,)
    }
};
        locals.var_ps0_min = assign86910_e132594;
        locals.var_ps0_min_dn0 = assign86910_e132594_d_n0;
        locals.var_ps0_min_dn2 = assign86910_e132594_d_n2;
        locals.var_ps0_min_dn4 = assign86910_e132594_d_n4;
        locals.var_ps0_min_dn5 = assign86910_e132594_d_n5;
        locals.var_ps0_min_dn6 = assign86910_e132594_d_n6;
        locals.var_ps0_min_dn7 = assign86910_e132594_d_n7;
        locals.var_ps0_min_dn8 = assign86910_e132594_d_n8;
        locals.var_ps0_min_dn9 = assign86910_e132594_d_n9;
        locals.var_ps0_min_dn10 = assign86910_e132594_d_n10;
        locals.var_ps0_min_dn13 = assign86910_e132594_d_n13;

        let (assign86920_e132604, assign86920_e132604_d_n0, assign86920_e132604_d_n2, assign86920_e132604_d_n4, assign86920_e132604_d_n5, assign86920_e132604_d_n6, assign86920_e132604_d_n7, assign86920_e132604_d_n8, assign86920_e132604_d_n9, assign86920_e132604_d_n10, assign86920_e132604_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86920_e132601: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign86920_e132602: f64 = (locals.var_beta * assign86920_e132601);
        (assign86920_e132602, ((locals.var_beta_dn0 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign86920_e132601) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign86920_e132601) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign86920_e132601) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign86920_e132601) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn9)), ((locals.var_beta_dn10 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn13 * assign86920_e132601) + (locals.var_beta * locals.var_vxbgmtcl_dn13)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign86920_e132604;
        locals.var_tx_dn0 = assign86920_e132604_d_n0;
        locals.var_tx_dn2 = assign86920_e132604_d_n2;
        locals.var_tx_dn4 = assign86920_e132604_d_n4;
        locals.var_tx_dn5 = assign86920_e132604_d_n5;
        locals.var_tx_dn6 = assign86920_e132604_d_n6;
        locals.var_tx_dn7 = assign86920_e132604_d_n7;
        locals.var_tx_dn8 = assign86920_e132604_d_n8;
        locals.var_tx_dn9 = assign86920_e132604_d_n9;
        locals.var_tx_dn10 = assign86920_e132604_d_n10;
        locals.var_tx_dn13 = assign86920_e132604_d_n13;

        let (assign86930_e132614, assign86930_e132614_d_n0, assign86930_e132614_d_n2, assign86930_e132614_d_n4, assign86930_e132614_d_n5, assign86930_e132614_d_n6, assign86930_e132614_d_n7, assign86930_e132614_d_n8, assign86930_e132614_d_n9, assign86930_e132614_d_n10, assign86930_e132614_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86930_e132611: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign86930_e132612: f64 = (1.0 / assign86930_e132611);
        (assign86930_e132612, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign86930_e132611 * assign86930_e132611))), (-(((locals.var_beta_dn13 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn13)) / (assign86930_e132611 * assign86930_e132611))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86930_e132614;
        locals.var_t1_dn0 = assign86930_e132614_d_n0;
        locals.var_t1_dn2 = assign86930_e132614_d_n2;
        locals.var_t1_dn4 = assign86930_e132614_d_n4;
        locals.var_t1_dn5 = assign86930_e132614_d_n5;
        locals.var_t1_dn6 = assign86930_e132614_d_n6;
        locals.var_t1_dn7 = assign86930_e132614_d_n7;
        locals.var_t1_dn8 = assign86930_e132614_d_n8;
        locals.var_t1_dn9 = assign86930_e132614_d_n9;
        locals.var_t1_dn10 = assign86930_e132614_d_n10;
        locals.var_t1_dn13 = assign86930_e132614_d_n13;

        let (assign86940_e132622, assign86940_e132622_d_n0, assign86940_e132622_d_n2, assign86940_e132622_d_n4, assign86940_e132622_d_n5, assign86940_e132622_d_n6, assign86940_e132622_d_n7, assign86940_e132622_d_n8, assign86940_e132622_d_n9, assign86940_e132622_d_n10, assign86940_e132622_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86940_e132620: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign86940_e132620, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn13 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign86940_e132622;
        locals.var_ty_dn0 = assign86940_e132622_d_n0;
        locals.var_ty_dn2 = assign86940_e132622_d_n2;
        locals.var_ty_dn4 = assign86940_e132622_d_n4;
        locals.var_ty_dn5 = assign86940_e132622_d_n5;
        locals.var_ty_dn6 = assign86940_e132622_d_n6;
        locals.var_ty_dn7 = assign86940_e132622_d_n7;
        locals.var_ty_dn8 = assign86940_e132622_d_n8;
        locals.var_ty_dn9 = assign86940_e132622_d_n9;
        locals.var_ty_dn10 = assign86940_e132622_d_n10;
        locals.var_ty_dn13 = assign86940_e132622_d_n13;

        let (assign86950_e132634, assign86950_e132634_d_n0, assign86950_e132634_d_n2, assign86950_e132634_d_n4, assign86950_e132634_d_n5, assign86950_e132634_d_n6, assign86950_e132634_d_n7, assign86950_e132634_d_n8, assign86950_e132634_d_n9, assign86950_e132634_d_n10, assign86950_e132634_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86950_e132629: f64 = (3.0 * 1.414213562373095);
        let assign86950_e132631: f64 = (assign86950_e132629 * locals.var_ty);
        let assign86950_e132632: f64 = (2.0 + assign86950_e132631);
        (assign86950_e132632, (assign86950_e132629 * locals.var_ty_dn0), (assign86950_e132629 * locals.var_ty_dn2), (assign86950_e132629 * locals.var_ty_dn4), (assign86950_e132629 * locals.var_ty_dn5), (assign86950_e132629 * locals.var_ty_dn6), (assign86950_e132629 * locals.var_ty_dn7), (assign86950_e132629 * locals.var_ty_dn8), (assign86950_e132629 * locals.var_ty_dn9), (assign86950_e132629 * locals.var_ty_dn10), (assign86950_e132629 * locals.var_ty_dn13),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn13,)
    }
};
        locals.var_ac41 = assign86950_e132634;
        locals.var_ac41_dn0 = assign86950_e132634_d_n0;
        locals.var_ac41_dn2 = assign86950_e132634_d_n2;
        locals.var_ac41_dn4 = assign86950_e132634_d_n4;
        locals.var_ac41_dn5 = assign86950_e132634_d_n5;
        locals.var_ac41_dn6 = assign86950_e132634_d_n6;
        locals.var_ac41_dn7 = assign86950_e132634_d_n7;
        locals.var_ac41_dn8 = assign86950_e132634_d_n8;
        locals.var_ac41_dn9 = assign86950_e132634_d_n9;
        locals.var_ac41_dn10 = assign86950_e132634_d_n10;
        locals.var_ac41_dn13 = assign86950_e132634_d_n13;

        let (assign86960_e132646, assign86960_e132646_d_n0, assign86960_e132646_d_n2, assign86960_e132646_d_n4, assign86960_e132646_d_n5, assign86960_e132646_d_n6, assign86960_e132646_d_n7, assign86960_e132646_d_n8, assign86960_e132646_d_n9, assign86960_e132646_d_n10, assign86960_e132646_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86960_e132640: f64 = (8.0 * locals.var_ac41);
        let assign86960_e132642: f64 = (assign86960_e132640 * locals.var_ac41);
        let assign86960_e132644: f64 = (assign86960_e132642 * locals.var_ac41);
        (assign86960_e132644, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn13) * locals.var_ac41) + (assign86960_e132640 * locals.var_ac41_dn13)) * locals.var_ac41) + (assign86960_e132642 * locals.var_ac41_dn13)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn13,)
    }
};
        locals.var_ac4 = assign86960_e132646;
        locals.var_ac4_dn0 = assign86960_e132646_d_n0;
        locals.var_ac4_dn2 = assign86960_e132646_d_n2;
        locals.var_ac4_dn4 = assign86960_e132646_d_n4;
        locals.var_ac4_dn5 = assign86960_e132646_d_n5;
        locals.var_ac4_dn6 = assign86960_e132646_d_n6;
        locals.var_ac4_dn7 = assign86960_e132646_d_n7;
        locals.var_ac4_dn8 = assign86960_e132646_d_n8;
        locals.var_ac4_dn9 = assign86960_e132646_d_n9;
        locals.var_ac4_dn10 = assign86960_e132646_d_n10;
        locals.var_ac4_dn13 = assign86960_e132646_d_n13;

        let (assign86970_e132662, assign86970_e132662_d_n0, assign86970_e132662_d_n2, assign86970_e132662_d_n4, assign86970_e132662_d_n5, assign86970_e132662_d_n6, assign86970_e132662_d_n7, assign86970_e132662_d_n8, assign86970_e132662_d_n9, assign86970_e132662_d_n10, assign86970_e132662_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86970_e132652: f64 = (7.0 * 1.414213562373095);
        let assign86970_e132655: f64 = (9.0 * locals.var_ty);
        let assign86970_e132658: f64 = (locals.var_tx - 2.0);
        let assign86970_e132659: f64 = (assign86970_e132655 * assign86970_e132658);
        let assign86970_e132660: f64 = (assign86970_e132652 - assign86970_e132659);
        (assign86970_e132660, (-(((9.0 * locals.var_ty_dn0) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn13) * assign86970_e132658) + (assign86970_e132655 * locals.var_tx_dn13))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn13,)
    }
};
        locals.var_ac31 = assign86970_e132662;
        locals.var_ac31_dn0 = assign86970_e132662_d_n0;
        locals.var_ac31_dn2 = assign86970_e132662_d_n2;
        locals.var_ac31_dn4 = assign86970_e132662_d_n4;
        locals.var_ac31_dn5 = assign86970_e132662_d_n5;
        locals.var_ac31_dn6 = assign86970_e132662_d_n6;
        locals.var_ac31_dn7 = assign86970_e132662_d_n7;
        locals.var_ac31_dn8 = assign86970_e132662_d_n8;
        locals.var_ac31_dn9 = assign86970_e132662_d_n9;
        locals.var_ac31_dn10 = assign86970_e132662_d_n10;
        locals.var_ac31_dn13 = assign86970_e132662_d_n13;

        let (assign86980_e132670, assign86980_e132670_d_n0, assign86980_e132670_d_n2, assign86980_e132670_d_n4, assign86980_e132670_d_n5, assign86980_e132670_d_n6, assign86980_e132670_d_n7, assign86980_e132670_d_n8, assign86980_e132670_d_n9, assign86980_e132670_d_n10, assign86980_e132670_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign86980_e132668: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign86980_e132668, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn13 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn13)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn13,)
    }
};
        locals.var_ac3 = assign86980_e132670;
        locals.var_ac3_dn0 = assign86980_e132670_d_n0;
        locals.var_ac3_dn2 = assign86980_e132670_d_n2;
        locals.var_ac3_dn4 = assign86980_e132670_d_n4;
        locals.var_ac3_dn5 = assign86980_e132670_d_n5;
        locals.var_ac3_dn6 = assign86980_e132670_d_n6;
        locals.var_ac3_dn7 = assign86980_e132670_d_n7;
        locals.var_ac3_dn8 = assign86980_e132670_d_n8;
        locals.var_ac3_dn9 = assign86980_e132670_d_n9;
        locals.var_ac3_dn10 = assign86980_e132670_d_n10;
        locals.var_ac3_dn13 = assign86980_e132670_d_n13;

        let assign86990_e132674: f64 = (locals.var_ac3 * 1e-8);
        let assign86990_e132675: f64 = if locals.var_ac4 < assign86990_e132674 { 1.0 } else { 0.0 };
        locals.var_guard2029 = assign86990_e132675;

        let (assign87010_e132696, assign87010_e132696_d_n0, assign87010_e132696_d_n2, assign87010_e132696_d_n4, assign87010_e132696_d_n5, assign87010_e132696_d_n6, assign87010_e132696_d_n7, assign87010_e132696_d_n8, assign87010_e132696_d_n9, assign87010_e132696_d_n10, assign87010_e132696_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) && (locals.var_guard2029 != 0.0)) {
        let assign87010_e132692: f64 = (0.5 * locals.var_ac4);
        let assign87010_e132694: f64 = (assign87010_e132692 / locals.var_ac31);
        (assign87010_e132694, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn13) * locals.var_ac31) - (assign87010_e132692 * locals.var_ac31_dn13)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign87010_e132696;
        locals.var_ac1_dn0 = assign87010_e132696_d_n0;
        locals.var_ac1_dn2 = assign87010_e132696_d_n2;
        locals.var_ac1_dn4 = assign87010_e132696_d_n4;
        locals.var_ac1_dn5 = assign87010_e132696_d_n5;
        locals.var_ac1_dn6 = assign87010_e132696_d_n6;
        locals.var_ac1_dn7 = assign87010_e132696_d_n7;
        locals.var_ac1_dn8 = assign87010_e132696_d_n8;
        locals.var_ac1_dn9 = assign87010_e132696_d_n9;
        locals.var_ac1_dn10 = assign87010_e132696_d_n10;
        locals.var_ac1_dn13 = assign87010_e132696_d_n13;

        let (assign87020_e132708, assign87020_e132708_d_n0, assign87020_e132708_d_n2, assign87020_e132708_d_n4, assign87020_e132708_d_n5, assign87020_e132708_d_n6, assign87020_e132708_d_n7, assign87020_e132708_d_n8, assign87020_e132708_d_n9, assign87020_e132708_d_n10, assign87020_e132708_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) && (locals.var_guard2029 == 0.0)) {
        let assign87020_e132705: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign87020_e132706: f64 = (assign87020_e132705).sqrt();
        (assign87020_e132706, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign87020_e132706)), ((locals.var_ac4_dn13 + locals.var_ac3_dn13) / (2.0 * assign87020_e132706)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn13,)
    }
};
        locals.var_ac2 = assign87020_e132708;
        locals.var_ac2_dn0 = assign87020_e132708_d_n0;
        locals.var_ac2_dn2 = assign87020_e132708_d_n2;
        locals.var_ac2_dn4 = assign87020_e132708_d_n4;
        locals.var_ac2_dn5 = assign87020_e132708_d_n5;
        locals.var_ac2_dn6 = assign87020_e132708_d_n6;
        locals.var_ac2_dn7 = assign87020_e132708_d_n7;
        locals.var_ac2_dn8 = assign87020_e132708_d_n8;
        locals.var_ac2_dn9 = assign87020_e132708_d_n9;
        locals.var_ac2_dn10 = assign87020_e132708_d_n10;
        locals.var_ac2_dn13 = assign87020_e132708_d_n13;

        let (assign87030_e132720, assign87030_e132720_d_n0, assign87030_e132720_d_n2, assign87030_e132720_d_n4, assign87030_e132720_d_n5, assign87030_e132720_d_n6, assign87030_e132720_d_n7, assign87030_e132720_d_n8, assign87030_e132720_d_n9, assign87030_e132720_d_n10, assign87030_e132720_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) && (locals.var_guard2029 == 0.0)) {
        let assign87030_e132716: f64 = (-locals.var_ac31);
        let assign87030_e132718: f64 = (assign87030_e132716 + locals.var_ac2);
        (assign87030_e132718, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn13) + locals.var_ac2_dn13),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn13,)
    }
};
        locals.var_ac1 = assign87030_e132720;
        locals.var_ac1_dn0 = assign87030_e132720_d_n0;
        locals.var_ac1_dn2 = assign87030_e132720_d_n2;
        locals.var_ac1_dn4 = assign87030_e132720_d_n4;
        locals.var_ac1_dn5 = assign87030_e132720_d_n5;
        locals.var_ac1_dn6 = assign87030_e132720_d_n6;
        locals.var_ac1_dn7 = assign87030_e132720_d_n7;
        locals.var_ac1_dn8 = assign87030_e132720_d_n8;
        locals.var_ac1_dn9 = assign87030_e132720_d_n9;
        locals.var_ac1_dn10 = assign87030_e132720_d_n10;
        locals.var_ac1_dn13 = assign87030_e132720_d_n13;

        let (assign87040_e132728, assign87040_e132728_d_n0, assign87040_e132728_d_n2, assign87040_e132728_d_n4, assign87040_e132728_d_n5, assign87040_e132728_d_n6, assign87040_e132728_d_n7, assign87040_e132728_d_n8, assign87040_e132728_d_n9, assign87040_e132728_d_n10, assign87040_e132728_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87040_e132726: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign87040_e132726, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn13)) } } else { (assign87040_e132726 * (0.3333333333333333 * (locals.var_ac1_dn13 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn13,)
    }
};
        locals.var_acd = assign87040_e132728;
        locals.var_acd_dn0 = assign87040_e132728_d_n0;
        locals.var_acd_dn2 = assign87040_e132728_d_n2;
        locals.var_acd_dn4 = assign87040_e132728_d_n4;
        locals.var_acd_dn5 = assign87040_e132728_d_n5;
        locals.var_acd_dn6 = assign87040_e132728_d_n6;
        locals.var_acd_dn7 = assign87040_e132728_d_n7;
        locals.var_acd_dn8 = assign87040_e132728_d_n8;
        locals.var_acd_dn9 = assign87040_e132728_d_n9;
        locals.var_acd_dn10 = assign87040_e132728_d_n10;
        locals.var_acd_dn13 = assign87040_e132728_d_n13;

        let (assign87050_e132751, assign87050_e132751_d_n0, assign87050_e132751_d_n2, assign87050_e132751_d_n4, assign87050_e132751_d_n5, assign87050_e132751_d_n6, assign87050_e132751_d_n7, assign87050_e132751_d_n8, assign87050_e132751_d_n9, assign87050_e132751_d_n10, assign87050_e132751_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87050_e132733: f64 = (-4.0);
        let assign87050_e132735: f64 = (assign87050_e132733 * 1.414213562373095);
        let assign87050_e132738: f64 = (12.0 * locals.var_ty);
        let assign87050_e132739: f64 = (assign87050_e132735 - assign87050_e132738);
        let assign87050_e132742: f64 = (2.0 * locals.var_acd);
        let assign87050_e132743: f64 = (assign87050_e132739 + assign87050_e132742);
        let assign87050_e132746: f64 = (1.414213562373095 * locals.var_acd);
        let assign87050_e132748: f64 = (assign87050_e132746 * locals.var_acd);
        let assign87050_e132749: f64 = (assign87050_e132743 + assign87050_e132748);
        (assign87050_e132749, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn13)) + (2.0 * locals.var_acd_dn13)) + (((1.414213562373095 * locals.var_acd_dn13) * locals.var_acd) + (assign87050_e132746 * locals.var_acd_dn13))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn13,)
    }
};
        locals.var_acn = assign87050_e132751;
        locals.var_acn_dn0 = assign87050_e132751_d_n0;
        locals.var_acn_dn2 = assign87050_e132751_d_n2;
        locals.var_acn_dn4 = assign87050_e132751_d_n4;
        locals.var_acn_dn5 = assign87050_e132751_d_n5;
        locals.var_acn_dn6 = assign87050_e132751_d_n6;
        locals.var_acn_dn7 = assign87050_e132751_d_n7;
        locals.var_acn_dn8 = assign87050_e132751_d_n8;
        locals.var_acn_dn9 = assign87050_e132751_d_n9;
        locals.var_acn_dn10 = assign87050_e132751_d_n10;
        locals.var_acn_dn13 = assign87050_e132751_d_n13;

        let (assign87060_e132759, assign87060_e132759_d_n0, assign87060_e132759_d_n2, assign87060_e132759_d_n4, assign87060_e132759_d_n5, assign87060_e132759_d_n6, assign87060_e132759_d_n7, assign87060_e132759_d_n8, assign87060_e132759_d_n9, assign87060_e132759_d_n10, assign87060_e132759_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87060_e132757: f64 = (locals.var_acn / locals.var_acd);
        (assign87060_e132757, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn13 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn13)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87060_e132759;
        locals.var_chi_dn0 = assign87060_e132759_d_n0;
        locals.var_chi_dn2 = assign87060_e132759_d_n2;
        locals.var_chi_dn4 = assign87060_e132759_d_n4;
        locals.var_chi_dn5 = assign87060_e132759_d_n5;
        locals.var_chi_dn6 = assign87060_e132759_d_n6;
        locals.var_chi_dn7 = assign87060_e132759_d_n7;
        locals.var_chi_dn8 = assign87060_e132759_d_n8;
        locals.var_chi_dn9 = assign87060_e132759_d_n9;
        locals.var_chi_dn10 = assign87060_e132759_d_n10;
        locals.var_chi_dn13 = assign87060_e132759_d_n13;

        let (assign87070_e132767, assign87070_e132767_d_n0, assign87070_e132767_d_n2, assign87070_e132767_d_n4, assign87070_e132767_d_n5, assign87070_e132767_d_n6, assign87070_e132767_d_n7, assign87070_e132767_d_n8, assign87070_e132767_d_n9, assign87070_e132767_d_n10, assign87070_e132767_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87070_e132765: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign87070_e132765, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87070_e132767;
        locals.var_t1_dn0 = assign87070_e132767_d_n0;
        locals.var_t1_dn2 = assign87070_e132767_d_n2;
        locals.var_t1_dn4 = assign87070_e132767_d_n4;
        locals.var_t1_dn5 = assign87070_e132767_d_n5;
        locals.var_t1_dn6 = assign87070_e132767_d_n6;
        locals.var_t1_dn7 = assign87070_e132767_d_n7;
        locals.var_t1_dn8 = assign87070_e132767_d_n8;
        locals.var_t1_dn9 = assign87070_e132767_d_n9;
        locals.var_t1_dn10 = assign87070_e132767_d_n10;
        locals.var_t1_dn13 = assign87070_e132767_d_n13;

        let (assign87080_e132775, assign87080_e132775_d_n0, assign87080_e132775_d_n2, assign87080_e132775_d_n4, assign87080_e132775_d_n5, assign87080_e132775_d_n6, assign87080_e132775_d_n7, assign87080_e132775_d_n8, assign87080_e132775_d_n9, assign87080_e132775_d_n10, assign87080_e132775_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87080_e132773: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign87080_e132773, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn13 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn13)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign87080_e132775;
        locals.var_t2_dn0 = assign87080_e132775_d_n0;
        locals.var_t2_dn2 = assign87080_e132775_d_n2;
        locals.var_t2_dn4 = assign87080_e132775_d_n4;
        locals.var_t2_dn5 = assign87080_e132775_d_n5;
        locals.var_t2_dn6 = assign87080_e132775_d_n6;
        locals.var_t2_dn7 = assign87080_e132775_d_n7;
        locals.var_t2_dn8 = assign87080_e132775_d_n8;
        locals.var_t2_dn9 = assign87080_e132775_d_n9;
        locals.var_t2_dn10 = assign87080_e132775_d_n10;
        locals.var_t2_dn13 = assign87080_e132775_d_n13;

        let (assign87090_e132786, assign87090_e132786_d_n0, assign87090_e132786_d_n2, assign87090_e132786_d_n4, assign87090_e132786_d_n5, assign87090_e132786_d_n6, assign87090_e132786_d_n7, assign87090_e132786_d_n8, assign87090_e132786_d_n9, assign87090_e132786_d_n10, assign87090_e132786_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87090_e132782: f64 = (locals.var_t2 * locals.var_t2);
        let assign87090_e132783: f64 = (1.0 + assign87090_e132782);
        let assign87090_e132784: f64 = (assign87090_e132783).sqrt();
        (assign87090_e132784, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign87090_e132784)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign87090_e132784)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign87090_e132786;
        locals.var_t3_dn0 = assign87090_e132786_d_n0;
        locals.var_t3_dn2 = assign87090_e132786_d_n2;
        locals.var_t3_dn4 = assign87090_e132786_d_n4;
        locals.var_t3_dn5 = assign87090_e132786_d_n5;
        locals.var_t3_dn6 = assign87090_e132786_d_n6;
        locals.var_t3_dn7 = assign87090_e132786_d_n7;
        locals.var_t3_dn8 = assign87090_e132786_d_n8;
        locals.var_t3_dn9 = assign87090_e132786_d_n9;
        locals.var_t3_dn10 = assign87090_e132786_d_n10;
        locals.var_t3_dn13 = assign87090_e132786_d_n13;

        let (assign87100_e132796, assign87100_e132796_d_n0, assign87100_e132796_d_n2, assign87100_e132796_d_n4, assign87100_e132796_d_n5, assign87100_e132796_d_n6, assign87100_e132796_d_n7, assign87100_e132796_d_n8, assign87100_e132796_d_n9, assign87100_e132796_d_n10, assign87100_e132796_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87100_e132792: f64 = (locals.var_t1 / locals.var_t3);
        let assign87100_e132794: f64 = (assign87100_e132792 - locals.var_vxbgmtcl);
        (assign87100_e132794, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn13 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign87100_e132796;
        locals.var_ps0ld_dn0 = assign87100_e132796_d_n0;
        locals.var_ps0ld_dn2 = assign87100_e132796_d_n2;
        locals.var_ps0ld_dn4 = assign87100_e132796_d_n4;
        locals.var_ps0ld_dn5 = assign87100_e132796_d_n5;
        locals.var_ps0ld_dn6 = assign87100_e132796_d_n6;
        locals.var_ps0ld_dn7 = assign87100_e132796_d_n7;
        locals.var_ps0ld_dn8 = assign87100_e132796_d_n8;
        locals.var_ps0ld_dn9 = assign87100_e132796_d_n9;
        locals.var_ps0ld_dn10 = assign87100_e132796_d_n10;
        locals.var_ps0ld_dn13 = assign87100_e132796_d_n13;

        let (assign87110_e132804, assign87110_e132804_d_n0, assign87110_e132804_d_n2, assign87110_e132804_d_n4, assign87110_e132804_d_n5, assign87110_e132804_d_n6, assign87110_e132804_d_n7, assign87110_e132804_d_n8, assign87110_e132804_d_n9, assign87110_e132804_d_n10, assign87110_e132804_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87110_e132802: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign87110_e132802, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign87110_e132804;
        locals.var_t2_dn0 = assign87110_e132804_d_n0;
        locals.var_t2_dn2 = assign87110_e132804_d_n2;
        locals.var_t2_dn4 = assign87110_e132804_d_n4;
        locals.var_t2_dn5 = assign87110_e132804_d_n5;
        locals.var_t2_dn6 = assign87110_e132804_d_n6;
        locals.var_t2_dn7 = assign87110_e132804_d_n7;
        locals.var_t2_dn8 = assign87110_e132804_d_n8;
        locals.var_t2_dn9 = assign87110_e132804_d_n9;
        locals.var_t2_dn10 = assign87110_e132804_d_n10;
        locals.var_t2_dn13 = assign87110_e132804_d_n13;

        let (assign87120_e132812, assign87120_e132812_d_n0, assign87120_e132812_d_n2, assign87120_e132812_d_n4, assign87120_e132812_d_n5, assign87120_e132812_d_n6, assign87120_e132812_d_n7, assign87120_e132812_d_n8, assign87120_e132812_d_n9, assign87120_e132812_d_n10, assign87120_e132812_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        let assign87120_e132810: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign87120_e132810, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign87120_e132812;
        locals.var_qsuld_dn0 = assign87120_e132812_d_n0;
        locals.var_qsuld_dn2 = assign87120_e132812_d_n2;
        locals.var_qsuld_dn4 = assign87120_e132812_d_n4;
        locals.var_qsuld_dn5 = assign87120_e132812_d_n5;
        locals.var_qsuld_dn6 = assign87120_e132812_d_n6;
        locals.var_qsuld_dn7 = assign87120_e132812_d_n7;
        locals.var_qsuld_dn8 = assign87120_e132812_d_n8;
        locals.var_qsuld_dn9 = assign87120_e132812_d_n9;
        locals.var_qsuld_dn10 = assign87120_e132812_d_n10;
        locals.var_qsuld_dn13 = assign87120_e132812_d_n13;

        let (assign87130_e132818, assign87130_e132818_d_n0, assign87130_e132818_d_n2, assign87130_e132818_d_n4, assign87130_e132818_d_n5, assign87130_e132818_d_n6, assign87130_e132818_d_n7, assign87130_e132818_d_n8, assign87130_e132818_d_n9, assign87130_e132818_d_n10, assign87130_e132818_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign87130_e132818;
        locals.var_qbuld_dn0 = assign87130_e132818_d_n0;
        locals.var_qbuld_dn2 = assign87130_e132818_d_n2;
        locals.var_qbuld_dn4 = assign87130_e132818_d_n4;
        locals.var_qbuld_dn5 = assign87130_e132818_d_n5;
        locals.var_qbuld_dn6 = assign87130_e132818_d_n6;
        locals.var_qbuld_dn7 = assign87130_e132818_d_n7;
        locals.var_qbuld_dn8 = assign87130_e132818_d_n8;
        locals.var_qbuld_dn9 = assign87130_e132818_d_n9;
        locals.var_qbuld_dn10 = assign87130_e132818_d_n10;
        locals.var_qbuld_dn13 = assign87130_e132818_d_n13;

    }

    pub(super) fn stamp_transient_block_306(
        locals: &mut StampLocals,
    ) {
        let (assign87140_e132824, assign87140_e132824_d_n0, assign87140_e132824_d_n2, assign87140_e132824_d_n4, assign87140_e132824_d_n5, assign87140_e132824_d_n6, assign87140_e132824_d_n7, assign87140_e132824_d_n8, assign87140_e132824_d_n9, assign87140_e132824_d_n10, assign87140_e132824_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk2011, locals.var_ps0ld_ini__blk2011_dn0, locals.var_ps0ld_ini__blk2011_dn2, locals.var_ps0ld_ini__blk2011_dn4, locals.var_ps0ld_ini__blk2011_dn5, locals.var_ps0ld_ini__blk2011_dn6, locals.var_ps0ld_ini__blk2011_dn7, locals.var_ps0ld_ini__blk2011_dn8, locals.var_ps0ld_ini__blk2011_dn9, locals.var_ps0ld_ini__blk2011_dn10, locals.var_ps0ld_ini__blk2011_dn13,)
    }
};
        locals.var_ps0ld_ini__blk2011 = assign87140_e132824;
        locals.var_ps0ld_ini__blk2011_dn0 = assign87140_e132824_d_n0;
        locals.var_ps0ld_ini__blk2011_dn2 = assign87140_e132824_d_n2;
        locals.var_ps0ld_ini__blk2011_dn4 = assign87140_e132824_d_n4;
        locals.var_ps0ld_ini__blk2011_dn5 = assign87140_e132824_d_n5;
        locals.var_ps0ld_ini__blk2011_dn6 = assign87140_e132824_d_n6;
        locals.var_ps0ld_ini__blk2011_dn7 = assign87140_e132824_d_n7;
        locals.var_ps0ld_ini__blk2011_dn8 = assign87140_e132824_d_n8;
        locals.var_ps0ld_ini__blk2011_dn9 = assign87140_e132824_d_n9;
        locals.var_ps0ld_ini__blk2011_dn10 = assign87140_e132824_d_n10;
        locals.var_ps0ld_ini__blk2011_dn13 = assign87140_e132824_d_n13;

        let assign87150_e132828: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87150_e132829: f64 = (locals.var_beta * assign87150_e132828);
        let assign87150_e132833: f64 = (10.0 * 2.220446049250313e-16);
        let assign87150_e132835: f64 = (assign87150_e132833 - 1.0);
        let assign87150_e132837: f64 = (assign87150_e132835 * locals.var_fac1p2);
        let assign87150_e132839: f64 = (assign87150_e132837 * locals.var_beta2);
        let assign87150_e132841: f64 = (assign87150_e132839 / 4.0);
        let assign87150_e132842: f64 = (1.0 + assign87150_e132841);
        let assign87150_e132843: f64 = if assign87150_e132829 < assign87150_e132842 { 1.0 } else { 0.0 };
        locals.var_guard2030 = assign87150_e132843;

        let (assign87160_e132858, assign87160_e132858_d_n0, assign87160_e132858_d_n2, assign87160_e132858_d_n4, assign87160_e132858_d_n5, assign87160_e132858_d_n6, assign87160_e132858_d_n7, assign87160_e132858_d_n8, assign87160_e132858_d_n9, assign87160_e132858_d_n10, assign87160_e132858_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2030 != 0.0)) {
        let assign87160_e132853: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87160_e132855: f64 = (assign87160_e132853 / 2.0);
        let assign87160_e132856: f64 = (locals.var_vgpld + assign87160_e132855);
        (assign87160_e132856, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (locals.var_vgpld_dn6 + (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0)), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87160_e132858;
        locals.var_ps0_inia_dn0 = assign87160_e132858_d_n0;
        locals.var_ps0_inia_dn2 = assign87160_e132858_d_n2;
        locals.var_ps0_inia_dn4 = assign87160_e132858_d_n4;
        locals.var_ps0_inia_dn5 = assign87160_e132858_d_n5;
        locals.var_ps0_inia_dn6 = assign87160_e132858_d_n6;
        locals.var_ps0_inia_dn7 = assign87160_e132858_d_n7;
        locals.var_ps0_inia_dn8 = assign87160_e132858_d_n8;
        locals.var_ps0_inia_dn9 = assign87160_e132858_d_n9;
        locals.var_ps0_inia_dn10 = assign87160_e132858_d_n10;
        locals.var_ps0_inia_dn13 = assign87160_e132858_d_n13;

        let (assign87170_e132882, assign87170_e132882_d_n0, assign87170_e132882_d_n2, assign87170_e132882_d_n4, assign87170_e132882_d_n5, assign87170_e132882_d_n6, assign87170_e132882_d_n7, assign87170_e132882_d_n8, assign87170_e132882_d_n9, assign87170_e132882_d_n10, assign87170_e132882_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2030 == 0.0)) {
        let assign87170_e132871: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87170_e132872: f64 = (locals.var_beta * assign87170_e132871);
        let assign87170_e132874: f64 = (assign87170_e132872 - 1.0);
        let assign87170_e132875: f64 = (4.0 * assign87170_e132874);
        let assign87170_e132878: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87170_e132879: f64 = (assign87170_e132875 / assign87170_e132878);
        let assign87170_e132880: f64 = (1.0 + assign87170_e132879);
        (assign87170_e132880, ((((4.0 * ((locals.var_beta_dn0 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn2 * assign87170_e132871) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn4 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn5 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn6 * assign87170_e132871) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn7 * assign87170_e132871) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn8 * assign87170_e132871) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn9 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn9))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn10 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87170_e132878 * assign87170_e132878)), ((((4.0 * ((locals.var_beta_dn13 * assign87170_e132871) + (locals.var_beta * locals.var_vxbgmtcl_dn13))) * assign87170_e132878) - (assign87170_e132875 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign87170_e132878 * assign87170_e132878)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign87170_e132882;
        locals.var_tx_dn0 = assign87170_e132882_d_n0;
        locals.var_tx_dn2 = assign87170_e132882_d_n2;
        locals.var_tx_dn4 = assign87170_e132882_d_n4;
        locals.var_tx_dn5 = assign87170_e132882_d_n5;
        locals.var_tx_dn6 = assign87170_e132882_d_n6;
        locals.var_tx_dn7 = assign87170_e132882_d_n7;
        locals.var_tx_dn8 = assign87170_e132882_d_n8;
        locals.var_tx_dn9 = assign87170_e132882_d_n9;
        locals.var_tx_dn10 = assign87170_e132882_d_n10;
        locals.var_tx_dn13 = assign87170_e132882_d_n13;

        let (assign87180_e132903, assign87180_e132903_d_n0, assign87180_e132903_d_n2, assign87180_e132903_d_n4, assign87180_e132903_d_n5, assign87180_e132903_d_n6, assign87180_e132903_d_n7, assign87180_e132903_d_n8, assign87180_e132903_d_n9, assign87180_e132903_d_n10, assign87180_e132903_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2030 == 0.0)) {
        let assign87180_e132893: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87180_e132895: f64 = (assign87180_e132893 / 2.0);
        let assign87180_e132898: f64 = (locals.var_tx).sqrt();
        let assign87180_e132899: f64 = (1.0 - assign87180_e132898);
        let assign87180_e132900: f64 = (assign87180_e132895 * assign87180_e132899);
        let assign87180_e132901: f64 = (locals.var_vgpld + assign87180_e132900);
        (assign87180_e132901, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn0 / (2.0 * assign87180_e132898))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn2 / (2.0 * assign87180_e132898)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn4 / (2.0 * assign87180_e132898))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn5 / (2.0 * assign87180_e132898))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn6 / (2.0 * assign87180_e132898)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn7 / (2.0 * assign87180_e132898)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn8 / (2.0 * assign87180_e132898)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn9 / (2.0 * assign87180_e132898))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn10 / (2.0 * assign87180_e132898))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign87180_e132899) + (assign87180_e132895 * (-(locals.var_tx_dn13 / (2.0 * assign87180_e132898))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87180_e132903;
        locals.var_ps0_inia_dn0 = assign87180_e132903_d_n0;
        locals.var_ps0_inia_dn2 = assign87180_e132903_d_n2;
        locals.var_ps0_inia_dn4 = assign87180_e132903_d_n4;
        locals.var_ps0_inia_dn5 = assign87180_e132903_d_n5;
        locals.var_ps0_inia_dn6 = assign87180_e132903_d_n6;
        locals.var_ps0_inia_dn7 = assign87180_e132903_d_n7;
        locals.var_ps0_inia_dn8 = assign87180_e132903_d_n8;
        locals.var_ps0_inia_dn9 = assign87180_e132903_d_n9;
        locals.var_ps0_inia_dn10 = assign87180_e132903_d_n10;
        locals.var_ps0_inia_dn13 = assign87180_e132903_d_n13;

        let (assign87190_e132914, assign87190_e132914_d_n0, assign87190_e132914_d_n2, assign87190_e132914_d_n4, assign87190_e132914_d_n5, assign87190_e132914_d_n6, assign87190_e132914_d_n7, assign87190_e132914_d_n8, assign87190_e132914_d_n9, assign87190_e132914_d_n10, assign87190_e132914_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign87190_e132911: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87190_e132912: f64 = (locals.var_beta * assign87190_e132911);
        (assign87190_e132912, ((locals.var_beta_dn0 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign87190_e132911) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87190_e132914;
        locals.var_chi_dn0 = assign87190_e132914_d_n0;
        locals.var_chi_dn2 = assign87190_e132914_d_n2;
        locals.var_chi_dn4 = assign87190_e132914_d_n4;
        locals.var_chi_dn5 = assign87190_e132914_d_n5;
        locals.var_chi_dn6 = assign87190_e132914_d_n6;
        locals.var_chi_dn7 = assign87190_e132914_d_n7;
        locals.var_chi_dn8 = assign87190_e132914_d_n8;
        locals.var_chi_dn9 = assign87190_e132914_d_n9;
        locals.var_chi_dn10 = assign87190_e132914_d_n10;
        locals.var_chi_dn13 = assign87190_e132914_d_n13;

        let assign87200_e132917: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard2031 = assign87200_e132917;

        let (assign87220_e132937, assign87220_e132937_d_n0, assign87220_e132937_d_n2, assign87220_e132937_d_n4, assign87220_e132937_d_n5, assign87220_e132937_d_n6, assign87220_e132937_d_n7, assign87220_e132937_d_n8, assign87220_e132937_d_n9, assign87220_e132937_d_n10, assign87220_e132937_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87220_e132934: f64 = (-locals.var_chi);
        let assign87220_e132935: f64 = (assign87220_e132934).exp();
        (assign87220_e132935, (assign87220_e132935 * (-locals.var_chi_dn0)), (assign87220_e132935 * (-locals.var_chi_dn2)), (assign87220_e132935 * (-locals.var_chi_dn4)), (assign87220_e132935 * (-locals.var_chi_dn5)), (assign87220_e132935 * (-locals.var_chi_dn6)), (assign87220_e132935 * (-locals.var_chi_dn7)), (assign87220_e132935 * (-locals.var_chi_dn8)), (assign87220_e132935 * (-locals.var_chi_dn9)), (assign87220_e132935 * (-locals.var_chi_dn10)), (assign87220_e132935 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign87220_e132937;
        locals.var_ty_dn0 = assign87220_e132937_d_n0;
        locals.var_ty_dn2 = assign87220_e132937_d_n2;
        locals.var_ty_dn4 = assign87220_e132937_d_n4;
        locals.var_ty_dn5 = assign87220_e132937_d_n5;
        locals.var_ty_dn6 = assign87220_e132937_d_n6;
        locals.var_ty_dn7 = assign87220_e132937_d_n7;
        locals.var_ty_dn8 = assign87220_e132937_d_n8;
        locals.var_ty_dn9 = assign87220_e132937_d_n9;
        locals.var_ty_dn10 = assign87220_e132937_d_n10;
        locals.var_ty_dn13 = assign87220_e132937_d_n13;

        let (assign87230_e132962, assign87230_e132962_d_n0, assign87230_e132962_d_n2, assign87230_e132962_d_n4, assign87230_e132962_d_n5, assign87230_e132962_d_n6, assign87230_e132962_d_n7, assign87230_e132962_d_n8, assign87230_e132962_d_n9, assign87230_e132962_d_n10, assign87230_e132962_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87230_e132949: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87230_e132950: f64 = (locals.var_beta * assign87230_e132949);
        let assign87230_e132952: f64 = (assign87230_e132950 - 1.0);
        let assign87230_e132954: f64 = (assign87230_e132952 + locals.var_ty);
        let assign87230_e132955: f64 = (4.0 * assign87230_e132954);
        let assign87230_e132958: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87230_e132959: f64 = (assign87230_e132955 / assign87230_e132958);
        let assign87230_e132960: f64 = (1.0 + assign87230_e132959);
        (assign87230_e132960, ((((4.0 * (((locals.var_beta_dn0 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn2 * assign87230_e132949) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn4 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn5 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn6 * assign87230_e132949) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn7 * assign87230_e132949) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn8 * assign87230_e132949) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn9 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn10 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87230_e132958 * assign87230_e132958)), ((((4.0 * (((locals.var_beta_dn13 * assign87230_e132949) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign87230_e132958) - (assign87230_e132955 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign87230_e132958 * assign87230_e132958)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign87230_e132962;
        locals.var_tx_dn0 = assign87230_e132962_d_n0;
        locals.var_tx_dn2 = assign87230_e132962_d_n2;
        locals.var_tx_dn4 = assign87230_e132962_d_n4;
        locals.var_tx_dn5 = assign87230_e132962_d_n5;
        locals.var_tx_dn6 = assign87230_e132962_d_n6;
        locals.var_tx_dn7 = assign87230_e132962_d_n7;
        locals.var_tx_dn8 = assign87230_e132962_d_n8;
        locals.var_tx_dn9 = assign87230_e132962_d_n9;
        locals.var_tx_dn10 = assign87230_e132962_d_n10;
        locals.var_tx_dn13 = assign87230_e132962_d_n13;

        let (assign87240_e132982, assign87240_e132982_d_n0, assign87240_e132982_d_n2, assign87240_e132982_d_n4, assign87240_e132982_d_n5, assign87240_e132982_d_n6, assign87240_e132982_d_n7, assign87240_e132982_d_n8, assign87240_e132982_d_n9, assign87240_e132982_d_n10, assign87240_e132982_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87240_e132972: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87240_e132974: f64 = (assign87240_e132972 / 2.0);
        let assign87240_e132977: f64 = (locals.var_tx).sqrt();
        let assign87240_e132978: f64 = (1.0 - assign87240_e132977);
        let assign87240_e132979: f64 = (assign87240_e132974 * assign87240_e132978);
        let assign87240_e132980: f64 = (locals.var_vgpld + assign87240_e132979);
        (assign87240_e132980, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn0 / (2.0 * assign87240_e132977))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn2 / (2.0 * assign87240_e132977)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn4 / (2.0 * assign87240_e132977))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn5 / (2.0 * assign87240_e132977))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn6 / (2.0 * assign87240_e132977)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn7 / (2.0 * assign87240_e132977)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn8 / (2.0 * assign87240_e132977)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn9 / (2.0 * assign87240_e132977))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn10 / (2.0 * assign87240_e132977))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign87240_e132978) + (assign87240_e132974 * (-(locals.var_tx_dn13 / (2.0 * assign87240_e132977))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87240_e132982;
        locals.var_ps0_inia_dn0 = assign87240_e132982_d_n0;
        locals.var_ps0_inia_dn2 = assign87240_e132982_d_n2;
        locals.var_ps0_inia_dn4 = assign87240_e132982_d_n4;
        locals.var_ps0_inia_dn5 = assign87240_e132982_d_n5;
        locals.var_ps0_inia_dn6 = assign87240_e132982_d_n6;
        locals.var_ps0_inia_dn7 = assign87240_e132982_d_n7;
        locals.var_ps0_inia_dn8 = assign87240_e132982_d_n8;
        locals.var_ps0_inia_dn9 = assign87240_e132982_d_n9;
        locals.var_ps0_inia_dn10 = assign87240_e132982_d_n10;
        locals.var_ps0_inia_dn13 = assign87240_e132982_d_n13;

        let (assign87250_e132995, assign87250_e132995_d_n0, assign87250_e132995_d_n2, assign87250_e132995_d_n4, assign87250_e132995_d_n5, assign87250_e132995_d_n6, assign87250_e132995_d_n7, assign87250_e132995_d_n8, assign87250_e132995_d_n9, assign87250_e132995_d_n10, assign87250_e132995_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87250_e132992: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87250_e132993: f64 = (locals.var_beta * assign87250_e132992);
        (assign87250_e132993, ((locals.var_beta_dn0 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign87250_e132992) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87250_e132995;
        locals.var_chi_dn0 = assign87250_e132995_d_n0;
        locals.var_chi_dn2 = assign87250_e132995_d_n2;
        locals.var_chi_dn4 = assign87250_e132995_d_n4;
        locals.var_chi_dn5 = assign87250_e132995_d_n5;
        locals.var_chi_dn6 = assign87250_e132995_d_n6;
        locals.var_chi_dn7 = assign87250_e132995_d_n7;
        locals.var_chi_dn8 = assign87250_e132995_d_n8;
        locals.var_chi_dn9 = assign87250_e132995_d_n9;
        locals.var_chi_dn10 = assign87250_e132995_d_n10;
        locals.var_chi_dn13 = assign87250_e132995_d_n13;

        let (assign87260_e133006, assign87260_e133006_d_n0, assign87260_e133006_d_n2, assign87260_e133006_d_n4, assign87260_e133006_d_n5, assign87260_e133006_d_n6, assign87260_e133006_d_n7, assign87260_e133006_d_n8, assign87260_e133006_d_n9, assign87260_e133006_d_n10, assign87260_e133006_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87260_e133003: f64 = (-locals.var_chi);
        let assign87260_e133004: f64 = (assign87260_e133003).exp();
        (assign87260_e133004, (assign87260_e133004 * (-locals.var_chi_dn0)), (assign87260_e133004 * (-locals.var_chi_dn2)), (assign87260_e133004 * (-locals.var_chi_dn4)), (assign87260_e133004 * (-locals.var_chi_dn5)), (assign87260_e133004 * (-locals.var_chi_dn6)), (assign87260_e133004 * (-locals.var_chi_dn7)), (assign87260_e133004 * (-locals.var_chi_dn8)), (assign87260_e133004 * (-locals.var_chi_dn9)), (assign87260_e133004 * (-locals.var_chi_dn10)), (assign87260_e133004 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign87260_e133006;
        locals.var_ty_dn0 = assign87260_e133006_d_n0;
        locals.var_ty_dn2 = assign87260_e133006_d_n2;
        locals.var_ty_dn4 = assign87260_e133006_d_n4;
        locals.var_ty_dn5 = assign87260_e133006_d_n5;
        locals.var_ty_dn6 = assign87260_e133006_d_n6;
        locals.var_ty_dn7 = assign87260_e133006_d_n7;
        locals.var_ty_dn8 = assign87260_e133006_d_n8;
        locals.var_ty_dn9 = assign87260_e133006_d_n9;
        locals.var_ty_dn10 = assign87260_e133006_d_n10;
        locals.var_ty_dn13 = assign87260_e133006_d_n13;

        let (assign87270_e133031, assign87270_e133031_d_n0, assign87270_e133031_d_n2, assign87270_e133031_d_n4, assign87270_e133031_d_n5, assign87270_e133031_d_n6, assign87270_e133031_d_n7, assign87270_e133031_d_n8, assign87270_e133031_d_n9, assign87270_e133031_d_n10, assign87270_e133031_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87270_e133018: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87270_e133019: f64 = (locals.var_beta * assign87270_e133018);
        let assign87270_e133021: f64 = (assign87270_e133019 - 1.0);
        let assign87270_e133023: f64 = (assign87270_e133021 + locals.var_ty);
        let assign87270_e133024: f64 = (4.0 * assign87270_e133023);
        let assign87270_e133027: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign87270_e133028: f64 = (assign87270_e133024 / assign87270_e133027);
        let assign87270_e133029: f64 = (1.0 + assign87270_e133028);
        (assign87270_e133029, ((((4.0 * (((locals.var_beta_dn0 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn2 * assign87270_e133018) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn4 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn5 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn6 * assign87270_e133018) + (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6))) + locals.var_ty_dn6)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn7 * assign87270_e133018) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn8 * assign87270_e133018) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn9 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn9)) + locals.var_ty_dn9)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn10 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign87270_e133027 * assign87270_e133027)), ((((4.0 * (((locals.var_beta_dn13 * assign87270_e133018) + (locals.var_beta * locals.var_vxbgmtcl_dn13)) + locals.var_ty_dn13)) * assign87270_e133027) - (assign87270_e133024 * ((locals.var_fac1p2_dn13 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn13)))) / (assign87270_e133027 * assign87270_e133027)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign87270_e133031;
        locals.var_tx_dn0 = assign87270_e133031_d_n0;
        locals.var_tx_dn2 = assign87270_e133031_d_n2;
        locals.var_tx_dn4 = assign87270_e133031_d_n4;
        locals.var_tx_dn5 = assign87270_e133031_d_n5;
        locals.var_tx_dn6 = assign87270_e133031_d_n6;
        locals.var_tx_dn7 = assign87270_e133031_d_n7;
        locals.var_tx_dn8 = assign87270_e133031_d_n8;
        locals.var_tx_dn9 = assign87270_e133031_d_n9;
        locals.var_tx_dn10 = assign87270_e133031_d_n10;
        locals.var_tx_dn13 = assign87270_e133031_d_n13;

        let (assign87280_e133051, assign87280_e133051_d_n0, assign87280_e133051_d_n2, assign87280_e133051_d_n4, assign87280_e133051_d_n5, assign87280_e133051_d_n6, assign87280_e133051_d_n7, assign87280_e133051_d_n8, assign87280_e133051_d_n9, assign87280_e133051_d_n10, assign87280_e133051_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87280_e133041: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign87280_e133043: f64 = (assign87280_e133041 / 2.0);
        let assign87280_e133046: f64 = (locals.var_tx).sqrt();
        let assign87280_e133047: f64 = (1.0 - assign87280_e133046);
        let assign87280_e133048: f64 = (assign87280_e133043 * assign87280_e133047);
        let assign87280_e133049: f64 = (locals.var_vgpld + assign87280_e133048);
        (assign87280_e133049, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn0 / (2.0 * assign87280_e133046))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn2 / (2.0 * assign87280_e133046)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn4 / (2.0 * assign87280_e133046))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn5 / (2.0 * assign87280_e133046))))), (locals.var_vgpld_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn6 / (2.0 * assign87280_e133046)))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn7 / (2.0 * assign87280_e133046)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn8 / (2.0 * assign87280_e133046)))))), (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn9 / (2.0 * assign87280_e133046))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn10 / (2.0 * assign87280_e133046))))), (((((locals.var_fac1p2_dn13 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn13)) / 2.0) * assign87280_e133047) + (assign87280_e133043 * (-(locals.var_tx_dn13 / (2.0 * assign87280_e133046))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87280_e133051;
        locals.var_ps0_inia_dn0 = assign87280_e133051_d_n0;
        locals.var_ps0_inia_dn2 = assign87280_e133051_d_n2;
        locals.var_ps0_inia_dn4 = assign87280_e133051_d_n4;
        locals.var_ps0_inia_dn5 = assign87280_e133051_d_n5;
        locals.var_ps0_inia_dn6 = assign87280_e133051_d_n6;
        locals.var_ps0_inia_dn7 = assign87280_e133051_d_n7;
        locals.var_ps0_inia_dn8 = assign87280_e133051_d_n8;
        locals.var_ps0_inia_dn9 = assign87280_e133051_d_n9;
        locals.var_ps0_inia_dn10 = assign87280_e133051_d_n10;
        locals.var_ps0_inia_dn13 = assign87280_e133051_d_n13;

        let (assign87290_e133064, assign87290_e133064_d_n0, assign87290_e133064_d_n2, assign87290_e133064_d_n4, assign87290_e133064_d_n5, assign87290_e133064_d_n6, assign87290_e133064_d_n7, assign87290_e133064_d_n8, assign87290_e133064_d_n9, assign87290_e133064_d_n10, assign87290_e133064_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 != 0.0)) {
        let assign87290_e133061: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign87290_e133062: f64 = (locals.var_beta * assign87290_e133061);
        (assign87290_e133062, ((locals.var_beta_dn0 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign87290_e133061) + (locals.var_beta * (locals.var_ps0_inia_dn13 + locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87290_e133064;
        locals.var_chi_dn0 = assign87290_e133064_d_n0;
        locals.var_chi_dn2 = assign87290_e133064_d_n2;
        locals.var_chi_dn4 = assign87290_e133064_d_n4;
        locals.var_chi_dn5 = assign87290_e133064_d_n5;
        locals.var_chi_dn6 = assign87290_e133064_d_n6;
        locals.var_chi_dn7 = assign87290_e133064_d_n7;
        locals.var_chi_dn8 = assign87290_e133064_d_n8;
        locals.var_chi_dn9 = assign87290_e133064_d_n9;
        locals.var_chi_dn10 = assign87290_e133064_d_n10;
        locals.var_chi_dn13 = assign87290_e133064_d_n13;

        let (assign87310_e133106,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87310_e133085: f64 = (2.0_f64).sqrt();
        let assign87310_e133086: f64 = (9.0 * assign87310_e133085);
        let assign87310_e133087: f64 = (1.0 / assign87310_e133086);
        let assign87310_e133091: f64 = (-3.0);
        let assign87310_e133092: f64 = (assign87310_e133091).exp();
        let assign87310_e133093: f64 = (7.0 * assign87310_e133092);
        let assign87310_e133094: f64 = (5.0 + assign87310_e133093);
        let assign87310_e133098: f64 = (-3.0);
        let assign87310_e133099: f64 = (assign87310_e133098).exp();
        let assign87310_e133100: f64 = (2.0 + assign87310_e133099);
        let assign87310_e133101: f64 = (assign87310_e133100).sqrt();
        let assign87310_e133102: f64 = (54.0 * assign87310_e133101);
        let assign87310_e133103: f64 = (assign87310_e133094 / assign87310_e133102);
        let assign87310_e133104: f64 = (assign87310_e133087 - assign87310_e133103);
        (assign87310_e133104,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign87310_e133106;

        let (assign87320_e133134,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87320_e133116: f64 = (-3.0);
        let assign87320_e133117: f64 = (assign87320_e133116).exp();
        let assign87320_e133118: f64 = (1.0 + assign87320_e133117);
        let assign87320_e133122: f64 = (-3.0);
        let assign87320_e133123: f64 = (assign87320_e133122).exp();
        let assign87320_e133124: f64 = (2.0 + assign87320_e133123);
        let assign87320_e133125: f64 = (assign87320_e133124).sqrt();
        let assign87320_e133126: f64 = (2.0 * assign87320_e133125);
        let assign87320_e133127: f64 = (assign87320_e133118 / assign87320_e133126);
        let assign87320_e133129: f64 = (2.0_f64).sqrt();
        let assign87320_e133131: f64 = (assign87320_e133129 / 3.0);
        let assign87320_e133132: f64 = (assign87320_e133127 - assign87320_e133131);
        (assign87320_e133132,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign87320_e133134;

        let (assign87330_e133153, assign87330_e133153_d_n0, assign87330_e133153_d_n2, assign87330_e133153_d_n4, assign87330_e133153_d_n5, assign87330_e133153_d_n6, assign87330_e133153_d_n7, assign87330_e133153_d_n8, assign87330_e133153_d_n9, assign87330_e133153_d_n10, assign87330_e133153_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87330_e133144: f64 = (2.0_f64).sqrt();
        let assign87330_e133145: f64 = (1.0 / assign87330_e133144);
        let assign87330_e133149: f64 = (locals.var_beta * locals.var_fac1);
        let assign87330_e133150: f64 = (1.0 / assign87330_e133149);
        let assign87330_e133151: f64 = (assign87330_e133145 + assign87330_e133150);
        (assign87330_e133151, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign87330_e133149 * assign87330_e133149))), (-(((locals.var_beta_dn13 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn13)) / (assign87330_e133149 * assign87330_e133149))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn13,)
    }
};
        locals.var_tc = assign87330_e133153;
        locals.var_tc_dn0 = assign87330_e133153_d_n0;
        locals.var_tc_dn2 = assign87330_e133153_d_n2;
        locals.var_tc_dn4 = assign87330_e133153_d_n4;
        locals.var_tc_dn5 = assign87330_e133153_d_n5;
        locals.var_tc_dn6 = assign87330_e133153_d_n6;
        locals.var_tc_dn7 = assign87330_e133153_d_n7;
        locals.var_tc_dn8 = assign87330_e133153_d_n8;
        locals.var_tc_dn9 = assign87330_e133153_d_n9;
        locals.var_tc_dn10 = assign87330_e133153_d_n10;
        locals.var_tc_dn13 = assign87330_e133153_d_n13;

        let (assign87340_e133168, assign87340_e133168_d_n0, assign87340_e133168_d_n2, assign87340_e133168_d_n4, assign87340_e133168_d_n5, assign87340_e133168_d_n6, assign87340_e133168_d_n7, assign87340_e133168_d_n8, assign87340_e133168_d_n9, assign87340_e133168_d_n10, assign87340_e133168_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87340_e133163: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87340_e133164: f64 = (-assign87340_e133163);
        let assign87340_e133166: f64 = (assign87340_e133164 / locals.var_fac1);
        (assign87340_e133166, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn9) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn13) * locals.var_fac1) - (assign87340_e133164 * locals.var_fac1_dn13)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn13,)
    }
};
        locals.var_td = assign87340_e133168;
        locals.var_td_dn0 = assign87340_e133168_d_n0;
        locals.var_td_dn2 = assign87340_e133168_d_n2;
        locals.var_td_dn4 = assign87340_e133168_d_n4;
        locals.var_td_dn5 = assign87340_e133168_d_n5;
        locals.var_td_dn6 = assign87340_e133168_d_n6;
        locals.var_td_dn7 = assign87340_e133168_d_n7;
        locals.var_td_dn8 = assign87340_e133168_d_n8;
        locals.var_td_dn9 = assign87340_e133168_d_n9;
        locals.var_td_dn10 = assign87340_e133168_d_n10;
        locals.var_td_dn13 = assign87340_e133168_d_n13;

        let (assign87350_e133206, assign87350_e133206_d_n0, assign87350_e133206_d_n2, assign87350_e133206_d_n4, assign87350_e133206_d_n5, assign87350_e133206_d_n6, assign87350_e133206_d_n7, assign87350_e133206_d_n8, assign87350_e133206_d_n9, assign87350_e133206_d_n10, assign87350_e133206_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87350_e133178: f64 = (locals.var_tb * locals.var_tb);
        let assign87350_e133180: f64 = (assign87350_e133178 * locals.var_tb);
        let assign87350_e133183: f64 = (27.0 * locals.var_ta);
        let assign87350_e133185: f64 = (assign87350_e133183 * locals.var_ta);
        let assign87350_e133187: f64 = (assign87350_e133185 * locals.var_ta);
        let assign87350_e133188: f64 = (assign87350_e133180 / assign87350_e133187);
        let assign87350_e133191: f64 = (locals.var_tb * locals.var_tc);
        let assign87350_e133194: f64 = (6.0 * locals.var_ta);
        let assign87350_e133196: f64 = (assign87350_e133194 * locals.var_ta);
        let assign87350_e133197: f64 = (assign87350_e133191 / assign87350_e133196);
        let assign87350_e133198: f64 = (assign87350_e133188 - assign87350_e133197);
        let assign87350_e133202: f64 = (2.0 * locals.var_ta);
        let assign87350_e133203: f64 = (locals.var_td / assign87350_e133202);
        let assign87350_e133204: f64 = (assign87350_e133198 + assign87350_e133203);
        (assign87350_e133204, ((-((locals.var_tb * locals.var_tc_dn0) / assign87350_e133196)) + (locals.var_td_dn0 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn2) / assign87350_e133196)) + (locals.var_td_dn2 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn4) / assign87350_e133196)) + (locals.var_td_dn4 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn5) / assign87350_e133196)) + (locals.var_td_dn5 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn6) / assign87350_e133196)) + (locals.var_td_dn6 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn7) / assign87350_e133196)) + (locals.var_td_dn7 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn8) / assign87350_e133196)) + (locals.var_td_dn8 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn9) / assign87350_e133196)) + (locals.var_td_dn9 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn10) / assign87350_e133196)) + (locals.var_td_dn10 / assign87350_e133202)), ((-((locals.var_tb * locals.var_tc_dn13) / assign87350_e133196)) + (locals.var_td_dn13 / assign87350_e133202)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn13,)
    }
};
        locals.var_tq = assign87350_e133206;
        locals.var_tq_dn0 = assign87350_e133206_d_n0;
        locals.var_tq_dn2 = assign87350_e133206_d_n2;
        locals.var_tq_dn4 = assign87350_e133206_d_n4;
        locals.var_tq_dn5 = assign87350_e133206_d_n5;
        locals.var_tq_dn6 = assign87350_e133206_d_n6;
        locals.var_tq_dn7 = assign87350_e133206_d_n7;
        locals.var_tq_dn8 = assign87350_e133206_d_n8;
        locals.var_tq_dn9 = assign87350_e133206_d_n9;
        locals.var_tq_dn10 = assign87350_e133206_d_n10;
        locals.var_tq_dn13 = assign87350_e133206_d_n13;

        let (assign87360_e133230, assign87360_e133230_d_n0, assign87360_e133230_d_n2, assign87360_e133230_d_n4, assign87360_e133230_d_n5, assign87360_e133230_d_n6, assign87360_e133230_d_n7, assign87360_e133230_d_n8, assign87360_e133230_d_n9, assign87360_e133230_d_n10, assign87360_e133230_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87360_e133216: f64 = (3.0 * locals.var_ta);
        let assign87360_e133218: f64 = (assign87360_e133216 * locals.var_tc);
        let assign87360_e133221: f64 = (locals.var_tb * locals.var_tb);
        let assign87360_e133222: f64 = (assign87360_e133218 - assign87360_e133221);
        let assign87360_e133225: f64 = (9.0 * locals.var_ta);
        let assign87360_e133227: f64 = (assign87360_e133225 * locals.var_ta);
        let assign87360_e133228: f64 = (assign87360_e133222 / assign87360_e133227);
        (assign87360_e133228, ((assign87360_e133216 * locals.var_tc_dn0) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn2) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn4) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn5) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn6) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn7) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn8) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn9) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn10) / assign87360_e133227), ((assign87360_e133216 * locals.var_tc_dn13) / assign87360_e133227),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn13,)
    }
};
        locals.var_tp = assign87360_e133230;
        locals.var_tp_dn0 = assign87360_e133230_d_n0;
        locals.var_tp_dn2 = assign87360_e133230_d_n2;
        locals.var_tp_dn4 = assign87360_e133230_d_n4;
        locals.var_tp_dn5 = assign87360_e133230_d_n5;
        locals.var_tp_dn6 = assign87360_e133230_d_n6;
        locals.var_tp_dn7 = assign87360_e133230_d_n7;
        locals.var_tp_dn8 = assign87360_e133230_d_n8;
        locals.var_tp_dn9 = assign87360_e133230_d_n9;
        locals.var_tp_dn10 = assign87360_e133230_d_n10;
        locals.var_tp_dn13 = assign87360_e133230_d_n13;

        let (assign87370_e133249, assign87370_e133249_d_n0, assign87370_e133249_d_n2, assign87370_e133249_d_n4, assign87370_e133249_d_n5, assign87370_e133249_d_n6, assign87370_e133249_d_n7, assign87370_e133249_d_n8, assign87370_e133249_d_n9, assign87370_e133249_d_n10, assign87370_e133249_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87370_e133240: f64 = (locals.var_tq * locals.var_tq);
        let assign87370_e133243: f64 = (locals.var_tp * locals.var_tp);
        let assign87370_e133245: f64 = (assign87370_e133243 * locals.var_tp);
        let assign87370_e133246: f64 = (assign87370_e133240 + assign87370_e133245);
        let assign87370_e133247: f64 = (assign87370_e133246).sqrt();
        (assign87370_e133247, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn0))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn2))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn4))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn5))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn6))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn7))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn8))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn9))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn10))) / (2.0 * assign87370_e133247)), ((((locals.var_tq_dn13 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn13)) + ((((locals.var_tp_dn13 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn13)) * locals.var_tp) + (assign87370_e133243 * locals.var_tp_dn13))) / (2.0 * assign87370_e133247)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign87370_e133249;
        locals.var_t5_dn0 = assign87370_e133249_d_n0;
        locals.var_t5_dn2 = assign87370_e133249_d_n2;
        locals.var_t5_dn4 = assign87370_e133249_d_n4;
        locals.var_t5_dn5 = assign87370_e133249_d_n5;
        locals.var_t5_dn6 = assign87370_e133249_d_n6;
        locals.var_t5_dn7 = assign87370_e133249_d_n7;
        locals.var_t5_dn8 = assign87370_e133249_d_n8;
        locals.var_t5_dn9 = assign87370_e133249_d_n9;
        locals.var_t5_dn10 = assign87370_e133249_d_n10;
        locals.var_t5_dn13 = assign87370_e133249_d_n13;

        let (assign87380_e133264, assign87380_e133264_d_n0, assign87380_e133264_d_n2, assign87380_e133264_d_n4, assign87380_e133264_d_n5, assign87380_e133264_d_n6, assign87380_e133264_d_n7, assign87380_e133264_d_n8, assign87380_e133264_d_n9, assign87380_e133264_d_n10, assign87380_e133264_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87380_e133258: f64 = (-locals.var_tq);
        let assign87380_e133260: f64 = (assign87380_e133258 + locals.var_t5);
        let assign87380_e133262: f64 = (assign87380_e133260).powf(0.3333333333333333);
        (assign87380_e133262, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign87380_e133260))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87380_e133260).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn13) + locals.var_t5_dn13))) } } else { (assign87380_e133262 * (0.3333333333333333 * (((-locals.var_tq_dn13) + locals.var_t5_dn13) / assign87380_e133260))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn13,)
    }
};
        locals.var_tu = assign87380_e133264;
        locals.var_tu_dn0 = assign87380_e133264_d_n0;
        locals.var_tu_dn2 = assign87380_e133264_d_n2;
        locals.var_tu_dn4 = assign87380_e133264_d_n4;
        locals.var_tu_dn5 = assign87380_e133264_d_n5;
        locals.var_tu_dn6 = assign87380_e133264_d_n6;
        locals.var_tu_dn7 = assign87380_e133264_d_n7;
        locals.var_tu_dn8 = assign87380_e133264_d_n8;
        locals.var_tu_dn9 = assign87380_e133264_d_n9;
        locals.var_tu_dn10 = assign87380_e133264_d_n10;
        locals.var_tu_dn13 = assign87380_e133264_d_n13;

    }

    pub(super) fn stamp_transient_block_307(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87390_e133279, assign87390_e133279_d_n0, assign87390_e133279_d_n2, assign87390_e133279_d_n4, assign87390_e133279_d_n5, assign87390_e133279_d_n6, assign87390_e133279_d_n7, assign87390_e133279_d_n8, assign87390_e133279_d_n9, assign87390_e133279_d_n10, assign87390_e133279_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87390_e133274: f64 = (locals.var_tq + locals.var_t5);
        let assign87390_e133276: f64 = (assign87390_e133274).powf(0.3333333333333333);
        let assign87390_e133277: f64 = (-assign87390_e133276);
        (assign87390_e133277, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign87390_e133274))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign87390_e133274).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn13 + locals.var_t5_dn13))) } } else { (assign87390_e133276 * (0.3333333333333333 * ((locals.var_tq_dn13 + locals.var_t5_dn13) / assign87390_e133274))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn13,)
    }
};
        locals.var_tv = assign87390_e133279;
        locals.var_tv_dn0 = assign87390_e133279_d_n0;
        locals.var_tv_dn2 = assign87390_e133279_d_n2;
        locals.var_tv_dn4 = assign87390_e133279_d_n4;
        locals.var_tv_dn5 = assign87390_e133279_d_n5;
        locals.var_tv_dn6 = assign87390_e133279_d_n6;
        locals.var_tv_dn7 = assign87390_e133279_d_n7;
        locals.var_tv_dn8 = assign87390_e133279_d_n8;
        locals.var_tv_dn9 = assign87390_e133279_d_n9;
        locals.var_tv_dn10 = assign87390_e133279_d_n10;
        locals.var_tv_dn13 = assign87390_e133279_d_n13;

        let (assign87400_e133297, assign87400_e133297_d_n0, assign87400_e133297_d_n2, assign87400_e133297_d_n4, assign87400_e133297_d_n5, assign87400_e133297_d_n6, assign87400_e133297_d_n7, assign87400_e133297_d_n8, assign87400_e133297_d_n9, assign87400_e133297_d_n10, assign87400_e133297_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87400_e133289: f64 = (locals.var_tu + locals.var_tv);
        let assign87400_e133293: f64 = (3.0 * locals.var_ta);
        let assign87400_e133294: f64 = (locals.var_tb / assign87400_e133293);
        let assign87400_e133295: f64 = (assign87400_e133289 - assign87400_e133294);
        (assign87400_e133295, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn13 + locals.var_tv_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87400_e133297;
        locals.var_chi_dn0 = assign87400_e133297_d_n0;
        locals.var_chi_dn2 = assign87400_e133297_d_n2;
        locals.var_chi_dn4 = assign87400_e133297_d_n4;
        locals.var_chi_dn5 = assign87400_e133297_d_n5;
        locals.var_chi_dn6 = assign87400_e133297_d_n6;
        locals.var_chi_dn7 = assign87400_e133297_d_n7;
        locals.var_chi_dn8 = assign87400_e133297_d_n8;
        locals.var_chi_dn9 = assign87400_e133297_d_n9;
        locals.var_chi_dn10 = assign87400_e133297_d_n10;
        locals.var_chi_dn13 = assign87400_e133297_d_n13;

        let (assign87410_e133311, assign87410_e133311_d_n0, assign87410_e133311_d_n2, assign87410_e133311_d_n4, assign87410_e133311_d_n5, assign87410_e133311_d_n6, assign87410_e133311_d_n7, assign87410_e133311_d_n8, assign87410_e133311_d_n9, assign87410_e133311_d_n10, assign87410_e133311_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2031 == 0.0)) {
        let assign87410_e133307: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign87410_e133309: f64 = (assign87410_e133307 - locals.var_vxbgmtcl);
        (assign87410_e133309, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn13,)
    }
};
        locals.var_ps0_inia = assign87410_e133311;
        locals.var_ps0_inia_dn0 = assign87410_e133311_d_n0;
        locals.var_ps0_inia_dn2 = assign87410_e133311_d_n2;
        locals.var_ps0_inia_dn4 = assign87410_e133311_d_n4;
        locals.var_ps0_inia_dn5 = assign87410_e133311_d_n5;
        locals.var_ps0_inia_dn6 = assign87410_e133311_d_n6;
        locals.var_ps0_inia_dn7 = assign87410_e133311_d_n7;
        locals.var_ps0_inia_dn8 = assign87410_e133311_d_n8;
        locals.var_ps0_inia_dn9 = assign87410_e133311_d_n9;
        locals.var_ps0_inia_dn10 = assign87410_e133311_d_n10;
        locals.var_ps0_inia_dn13 = assign87410_e133311_d_n13;

        let assign87420_e133314: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2032 = assign87420_e133314;

        let (assign87430_e133327, assign87430_e133327_d_n0, assign87430_e133327_d_n2, assign87430_e133327_d_n4, assign87430_e133327_d_n5, assign87430_e133327_d_n6, assign87430_e133327_d_n7, assign87430_e133327_d_n8, assign87430_e133327_d_n9, assign87430_e133327_d_n10, assign87430_e133327_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87430_e133323: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign87430_e133325: f64 = (assign87430_e133323 + 0.1);
        (assign87430_e133325, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn13,)
    }
};
        locals.var_vgpld_shift = assign87430_e133327;
        locals.var_vgpld_shift_dn0 = assign87430_e133327_d_n0;
        locals.var_vgpld_shift_dn2 = assign87430_e133327_d_n2;
        locals.var_vgpld_shift_dn4 = assign87430_e133327_d_n4;
        locals.var_vgpld_shift_dn5 = assign87430_e133327_d_n5;
        locals.var_vgpld_shift_dn6 = assign87430_e133327_d_n6;
        locals.var_vgpld_shift_dn7 = assign87430_e133327_d_n7;
        locals.var_vgpld_shift_dn8 = assign87430_e133327_d_n8;
        locals.var_vgpld_shift_dn9 = assign87430_e133327_d_n9;
        locals.var_vgpld_shift_dn10 = assign87430_e133327_d_n10;
        locals.var_vgpld_shift_dn13 = assign87430_e133327_d_n13;

        let (assign87440_e133338, assign87440_e133338_d_n0, assign87440_e133338_d_n2, assign87440_e133338_d_n4, assign87440_e133338_d_n5, assign87440_e133338_d_n6, assign87440_e133338_d_n7, assign87440_e133338_d_n8, assign87440_e133338_d_n9, assign87440_e133338_d_n10, assign87440_e133338_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87440_e133336: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign87440_e133336, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign87440_e133338;
        locals.var_cfs1_dn0 = assign87440_e133338_d_n0;
        locals.var_cfs1_dn2 = assign87440_e133338_d_n2;
        locals.var_cfs1_dn4 = assign87440_e133338_d_n4;
        locals.var_cfs1_dn5 = assign87440_e133338_d_n5;
        locals.var_cfs1_dn6 = assign87440_e133338_d_n6;
        locals.var_cfs1_dn7 = assign87440_e133338_d_n7;
        locals.var_cfs1_dn8 = assign87440_e133338_d_n8;
        locals.var_cfs1_dn9 = assign87440_e133338_d_n9;
        locals.var_cfs1_dn10 = assign87440_e133338_d_n10;
        locals.var_cfs1_dn13 = assign87440_e133338_d_n13;

        let (assign87450_e133349, assign87450_e133349_d_n0, assign87450_e133349_d_n2, assign87450_e133349_d_n4, assign87450_e133349_d_n5, assign87450_e133349_d_n6, assign87450_e133349_d_n7, assign87450_e133349_d_n8, assign87450_e133349_d_n9, assign87450_e133349_d_n10, assign87450_e133349_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87450_e133347: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign87450_e133347, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn13,)
    }
};
        locals.var_gammachi = assign87450_e133349;
        locals.var_gammachi_dn0 = assign87450_e133349_d_n0;
        locals.var_gammachi_dn2 = assign87450_e133349_d_n2;
        locals.var_gammachi_dn4 = assign87450_e133349_d_n4;
        locals.var_gammachi_dn5 = assign87450_e133349_d_n5;
        locals.var_gammachi_dn6 = assign87450_e133349_d_n6;
        locals.var_gammachi_dn7 = assign87450_e133349_d_n7;
        locals.var_gammachi_dn8 = assign87450_e133349_d_n8;
        locals.var_gammachi_dn9 = assign87450_e133349_d_n9;
        locals.var_gammachi_dn10 = assign87450_e133349_d_n10;
        locals.var_gammachi_dn13 = assign87450_e133349_d_n13;

        let (assign87460_e133360, assign87460_e133360_d_n0, assign87460_e133360_d_n2, assign87460_e133360_d_n4, assign87460_e133360_d_n5, assign87460_e133360_d_n6, assign87460_e133360_d_n7, assign87460_e133360_d_n8, assign87460_e133360_d_n9, assign87460_e133360_d_n10, assign87460_e133360_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87460_e133358: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign87460_e133358, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn13 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign87460_e133360;
        locals.var_t0_dn0 = assign87460_e133360_d_n0;
        locals.var_t0_dn2 = assign87460_e133360_d_n2;
        locals.var_t0_dn4 = assign87460_e133360_d_n4;
        locals.var_t0_dn5 = assign87460_e133360_d_n5;
        locals.var_t0_dn6 = assign87460_e133360_d_n6;
        locals.var_t0_dn7 = assign87460_e133360_d_n7;
        locals.var_t0_dn8 = assign87460_e133360_d_n8;
        locals.var_t0_dn9 = assign87460_e133360_d_n9;
        locals.var_t0_dn10 = assign87460_e133360_d_n10;
        locals.var_t0_dn13 = assign87460_e133360_d_n13;

        let (assign87470_e133371, assign87470_e133371_d_n0, assign87470_e133371_d_n2, assign87470_e133371_d_n4, assign87470_e133371_d_n5, assign87470_e133371_d_n6, assign87470_e133371_d_n7, assign87470_e133371_d_n8, assign87470_e133371_d_n9, assign87470_e133371_d_n10, assign87470_e133371_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87470_e133369: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign87470_e133369, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn13 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn13)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign87470_e133371;
        locals.var_psi_dn0 = assign87470_e133371_d_n0;
        locals.var_psi_dn2 = assign87470_e133371_d_n2;
        locals.var_psi_dn4 = assign87470_e133371_d_n4;
        locals.var_psi_dn5 = assign87470_e133371_d_n5;
        locals.var_psi_dn6 = assign87470_e133371_d_n6;
        locals.var_psi_dn7 = assign87470_e133371_d_n7;
        locals.var_psi_dn8 = assign87470_e133371_d_n8;
        locals.var_psi_dn9 = assign87470_e133371_d_n9;
        locals.var_psi_dn10 = assign87470_e133371_d_n10;
        locals.var_psi_dn13 = assign87470_e133371_d_n13;

        let (assign87480_e133396, assign87480_e133396_d_n0, assign87480_e133396_d_n2, assign87480_e133396_d_n4, assign87480_e133396_d_n5, assign87480_e133396_d_n6, assign87480_e133396_d_n7, assign87480_e133396_d_n8, assign87480_e133396_d_n9, assign87480_e133396_d_n10, assign87480_e133396_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87480_e133380: f64 = (locals.var_gammachi * locals.var_t0);
        let assign87480_e133383: f64 = (locals.var_psi * locals.var_psi);
        let assign87480_e133384: f64 = (assign87480_e133380 + assign87480_e133383);
        let assign87480_e133385: f64 = (assign87480_e133384).ln();
        let assign87480_e133388: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign87480_e133389: f64 = (assign87480_e133388).ln();
        let assign87480_e133390: f64 = (assign87480_e133385 - assign87480_e133389);
        let assign87480_e133393: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign87480_e133394: f64 = (assign87480_e133390 + assign87480_e133393);
        (assign87480_e133394, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign87480_e133384) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign87480_e133388)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign87480_e133384) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign87480_e133388)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign87480_e133384) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign87480_e133388)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign87480_e133384) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign87480_e133388)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign87480_e133384) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign87480_e133388)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign87480_e133384) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign87480_e133388)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign87480_e133384) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign87480_e133388)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign87480_e133384) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign87480_e133388)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign87480_e133384) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign87480_e133388)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign87480_e133384) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign87480_e133388)) + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign87480_e133396;
        locals.var_chi_1_dn0 = assign87480_e133396_d_n0;
        locals.var_chi_1_dn2 = assign87480_e133396_d_n2;
        locals.var_chi_1_dn4 = assign87480_e133396_d_n4;
        locals.var_chi_1_dn5 = assign87480_e133396_d_n5;
        locals.var_chi_1_dn6 = assign87480_e133396_d_n6;
        locals.var_chi_1_dn7 = assign87480_e133396_d_n7;
        locals.var_chi_1_dn8 = assign87480_e133396_d_n8;
        locals.var_chi_1_dn9 = assign87480_e133396_d_n9;
        locals.var_chi_1_dn10 = assign87480_e133396_d_n10;
        locals.var_chi_1_dn13 = assign87480_e133396_d_n13;

        let assign87490_e133399: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2033 = assign87490_e133399;

        let (assign87500_e133414, assign87500_e133414_d_n0, assign87500_e133414_d_n2, assign87500_e133414_d_n4, assign87500_e133414_d_n5, assign87500_e133414_d_n6, assign87500_e133414_d_n7, assign87500_e133414_d_n8, assign87500_e133414_d_n9, assign87500_e133414_d_n10, assign87500_e133414_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87500_e133410: f64 = (locals.var_psi - locals.var_chi_1);
        let assign87500_e133412: f64 = (assign87500_e133410 - 1.0);
        (assign87500_e133412, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign87500_e133414;
        locals.var_tmf1_dn0 = assign87500_e133414_d_n0;
        locals.var_tmf1_dn2 = assign87500_e133414_d_n2;
        locals.var_tmf1_dn4 = assign87500_e133414_d_n4;
        locals.var_tmf1_dn5 = assign87500_e133414_d_n5;
        locals.var_tmf1_dn6 = assign87500_e133414_d_n6;
        locals.var_tmf1_dn7 = assign87500_e133414_d_n7;
        locals.var_tmf1_dn8 = assign87500_e133414_d_n8;
        locals.var_tmf1_dn9 = assign87500_e133414_d_n9;
        locals.var_tmf1_dn10 = assign87500_e133414_d_n10;
        locals.var_tmf1_dn13 = assign87500_e133414_d_n13;

        let (assign87510_e133429, assign87510_e133429_d_n0, assign87510_e133429_d_n2, assign87510_e133429_d_n4, assign87510_e133429_d_n5, assign87510_e133429_d_n6, assign87510_e133429_d_n7, assign87510_e133429_d_n8, assign87510_e133429_d_n9, assign87510_e133429_d_n10, assign87510_e133429_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87510_e133425: f64 = (4.0 * locals.var_psi);
        let assign87510_e133427: f64 = assign87510_e133425;
        (assign87510_e133427, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn13),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign87510_e133429;
        locals.var_tmf2_dn0 = assign87510_e133429_d_n0;
        locals.var_tmf2_dn2 = assign87510_e133429_d_n2;
        locals.var_tmf2_dn4 = assign87510_e133429_d_n4;
        locals.var_tmf2_dn5 = assign87510_e133429_d_n5;
        locals.var_tmf2_dn6 = assign87510_e133429_d_n6;
        locals.var_tmf2_dn7 = assign87510_e133429_d_n7;
        locals.var_tmf2_dn8 = assign87510_e133429_d_n8;
        locals.var_tmf2_dn9 = assign87510_e133429_d_n9;
        locals.var_tmf2_dn10 = assign87510_e133429_d_n10;
        locals.var_tmf2_dn13 = assign87510_e133429_d_n13;

        let (assign87520_e133446, assign87520_e133446_d_n0, assign87520_e133446_d_n2, assign87520_e133446_d_n4, assign87520_e133446_d_n5, assign87520_e133446_d_n6, assign87520_e133446_d_n7, assign87520_e133446_d_n8, assign87520_e133446_d_n9, assign87520_e133446_d_n10, assign87520_e133446_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let (assign87520_e133444, assign87520_e133444_d_n0, assign87520_e133444_d_n2, assign87520_e133444_d_n4, assign87520_e133444_d_n5, assign87520_e133444_d_n6, assign87520_e133444_d_n7, assign87520_e133444_d_n8, assign87520_e133444_d_n9, assign87520_e133444_d_n10, assign87520_e133444_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign87520_e133443: f64 = (-locals.var_tmf2);
                (assign87520_e133443, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign87520_e133444, assign87520_e133444_d_n0, assign87520_e133444_d_n2, assign87520_e133444_d_n4, assign87520_e133444_d_n5, assign87520_e133444_d_n6, assign87520_e133444_d_n7, assign87520_e133444_d_n8, assign87520_e133444_d_n9, assign87520_e133444_d_n10, assign87520_e133444_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign87520_e133446;
        locals.var_tmf2_dn0 = assign87520_e133446_d_n0;
        locals.var_tmf2_dn2 = assign87520_e133446_d_n2;
        locals.var_tmf2_dn4 = assign87520_e133446_d_n4;
        locals.var_tmf2_dn5 = assign87520_e133446_d_n5;
        locals.var_tmf2_dn6 = assign87520_e133446_d_n6;
        locals.var_tmf2_dn7 = assign87520_e133446_d_n7;
        locals.var_tmf2_dn8 = assign87520_e133446_d_n8;
        locals.var_tmf2_dn9 = assign87520_e133446_d_n9;
        locals.var_tmf2_dn10 = assign87520_e133446_d_n10;
        locals.var_tmf2_dn13 = assign87520_e133446_d_n13;

        let (assign87530_e133462, assign87530_e133462_d_n0, assign87530_e133462_d_n2, assign87530_e133462_d_n4, assign87530_e133462_d_n5, assign87530_e133462_d_n6, assign87530_e133462_d_n7, assign87530_e133462_d_n8, assign87530_e133462_d_n9, assign87530_e133462_d_n10, assign87530_e133462_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87530_e133457: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign87530_e133459: f64 = (assign87530_e133457 + locals.var_tmf2);
        let assign87530_e133460: f64 = (assign87530_e133459).sqrt();
        (assign87530_e133460, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign87530_e133460)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign87530_e133460)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign87530_e133462;
        locals.var_tmf2_dn0 = assign87530_e133462_d_n0;
        locals.var_tmf2_dn2 = assign87530_e133462_d_n2;
        locals.var_tmf2_dn4 = assign87530_e133462_d_n4;
        locals.var_tmf2_dn5 = assign87530_e133462_d_n5;
        locals.var_tmf2_dn6 = assign87530_e133462_d_n6;
        locals.var_tmf2_dn7 = assign87530_e133462_d_n7;
        locals.var_tmf2_dn8 = assign87530_e133462_d_n8;
        locals.var_tmf2_dn9 = assign87530_e133462_d_n9;
        locals.var_tmf2_dn10 = assign87530_e133462_d_n10;
        locals.var_tmf2_dn13 = assign87530_e133462_d_n13;

        let (assign87540_e133479, assign87540_e133479_d_n0, assign87540_e133479_d_n2, assign87540_e133479_d_n4, assign87540_e133479_d_n5, assign87540_e133479_d_n6, assign87540_e133479_d_n7, assign87540_e133479_d_n8, assign87540_e133479_d_n9, assign87540_e133479_d_n10, assign87540_e133479_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87540_e133475: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign87540_e133476: f64 = (1.0 + assign87540_e133475);
        let assign87540_e133477: f64 = (0.5 * assign87540_e133476);
        (assign87540_e133477, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87540_e133479;
        locals.var_t1_dn0 = assign87540_e133479_d_n0;
        locals.var_t1_dn2 = assign87540_e133479_d_n2;
        locals.var_t1_dn4 = assign87540_e133479_d_n4;
        locals.var_t1_dn5 = assign87540_e133479_d_n5;
        locals.var_t1_dn6 = assign87540_e133479_d_n6;
        locals.var_t1_dn7 = assign87540_e133479_d_n7;
        locals.var_t1_dn8 = assign87540_e133479_d_n8;
        locals.var_t1_dn9 = assign87540_e133479_d_n9;
        locals.var_t1_dn10 = assign87540_e133479_d_n10;
        locals.var_t1_dn13 = assign87540_e133479_d_n13;

        let (assign87550_e133496, assign87550_e133496_d_n0, assign87550_e133496_d_n2, assign87550_e133496_d_n4, assign87550_e133496_d_n5, assign87550_e133496_d_n6, assign87550_e133496_d_n7, assign87550_e133496_d_n8, assign87550_e133496_d_n9, assign87550_e133496_d_n10, assign87550_e133496_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 != 0.0)) {
        let assign87550_e133492: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign87550_e133493: f64 = (0.5 * assign87550_e133492);
        let assign87550_e133494: f64 = (locals.var_psi - assign87550_e133493);
        (assign87550_e133494, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign87550_e133496;
        locals.var_chi_1_dn0 = assign87550_e133496_d_n0;
        locals.var_chi_1_dn2 = assign87550_e133496_d_n2;
        locals.var_chi_1_dn4 = assign87550_e133496_d_n4;
        locals.var_chi_1_dn5 = assign87550_e133496_d_n5;
        locals.var_chi_1_dn6 = assign87550_e133496_d_n6;
        locals.var_chi_1_dn7 = assign87550_e133496_d_n7;
        locals.var_chi_1_dn8 = assign87550_e133496_d_n8;
        locals.var_chi_1_dn9 = assign87550_e133496_d_n9;
        locals.var_chi_1_dn10 = assign87550_e133496_d_n10;
        locals.var_chi_1_dn13 = assign87550_e133496_d_n13;

        let (assign87560_e133513, assign87560_e133513_d_n0, assign87560_e133513_d_n2, assign87560_e133513_d_n4, assign87560_e133513_d_n5, assign87560_e133513_d_n6, assign87560_e133513_d_n7, assign87560_e133513_d_n8, assign87560_e133513_d_n9, assign87560_e133513_d_n10, assign87560_e133513_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2033 == 0.0)) {
        let (assign87560_e133511, assign87560_e133511_d_n0, assign87560_e133511_d_n2, assign87560_e133511_d_n4, assign87560_e133511_d_n5, assign87560_e133511_d_n6, assign87560_e133511_d_n7, assign87560_e133511_d_n8, assign87560_e133511_d_n9, assign87560_e133511_d_n10, assign87560_e133511_d_n13,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
            }
        };
        (assign87560_e133511, assign87560_e133511_d_n0, assign87560_e133511_d_n2, assign87560_e133511_d_n4, assign87560_e133511_d_n5, assign87560_e133511_d_n6, assign87560_e133511_d_n7, assign87560_e133511_d_n8, assign87560_e133511_d_n9, assign87560_e133511_d_n10, assign87560_e133511_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign87560_e133513;
        locals.var_chi_1_dn0 = assign87560_e133513_d_n0;
        locals.var_chi_1_dn2 = assign87560_e133513_d_n2;
        locals.var_chi_1_dn4 = assign87560_e133513_d_n4;
        locals.var_chi_1_dn5 = assign87560_e133513_d_n5;
        locals.var_chi_1_dn6 = assign87560_e133513_d_n6;
        locals.var_chi_1_dn7 = assign87560_e133513_d_n7;
        locals.var_chi_1_dn8 = assign87560_e133513_d_n8;
        locals.var_chi_1_dn9 = assign87560_e133513_d_n9;
        locals.var_chi_1_dn10 = assign87560_e133513_d_n10;
        locals.var_chi_1_dn13 = assign87560_e133513_d_n13;

        let (assign87570_e133527, assign87570_e133527_d_n0, assign87570_e133527_d_n2, assign87570_e133527_d_n4, assign87570_e133527_d_n5, assign87570_e133527_d_n6, assign87570_e133527_d_n7, assign87570_e133527_d_n8, assign87570_e133527_d_n9, assign87570_e133527_d_n10, assign87570_e133527_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let (assign87570_e133525, assign87570_e133525_d_n0, assign87570_e133525_d_n2, assign87570_e133525_d_n4, assign87570_e133525_d_n5, assign87570_e133525_d_n6, assign87570_e133525_d_n7, assign87570_e133525_d_n8, assign87570_e133525_d_n9, assign87570_e133525_d_n10, assign87570_e133525_d_n13,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign87570_e133525, assign87570_e133525_d_n0, assign87570_e133525_d_n2, assign87570_e133525_d_n4, assign87570_e133525_d_n5, assign87570_e133525_d_n6, assign87570_e133525_d_n7, assign87570_e133525_d_n8, assign87570_e133525_d_n9, assign87570_e133525_d_n10, assign87570_e133525_d_n13,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign87570_e133527;
        locals.var_chi_1_dn0 = assign87570_e133527_d_n0;
        locals.var_chi_1_dn2 = assign87570_e133527_d_n2;
        locals.var_chi_1_dn4 = assign87570_e133527_d_n4;
        locals.var_chi_1_dn5 = assign87570_e133527_d_n5;
        locals.var_chi_1_dn6 = assign87570_e133527_d_n6;
        locals.var_chi_1_dn7 = assign87570_e133527_d_n7;
        locals.var_chi_1_dn8 = assign87570_e133527_d_n8;
        locals.var_chi_1_dn9 = assign87570_e133527_d_n9;
        locals.var_chi_1_dn10 = assign87570_e133527_d_n10;
        locals.var_chi_1_dn13 = assign87570_e133527_d_n13;

        let (assign87580_e133538, assign87580_e133538_d_n0, assign87580_e133538_d_n2, assign87580_e133538_d_n4, assign87580_e133538_d_n5, assign87580_e133538_d_n6, assign87580_e133538_d_n7, assign87580_e133538_d_n8, assign87580_e133538_d_n9, assign87580_e133538_d_n10, assign87580_e133538_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87580_e133536: f64 = (locals.var_psi - locals.var_chi_1);
        (assign87580_e133536, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn13 - locals.var_chi_1_dn13),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign87580_e133538;
        locals.var_psi_dn0 = assign87580_e133538_d_n0;
        locals.var_psi_dn2 = assign87580_e133538_d_n2;
        locals.var_psi_dn4 = assign87580_e133538_d_n4;
        locals.var_psi_dn5 = assign87580_e133538_d_n5;
        locals.var_psi_dn6 = assign87580_e133538_d_n6;
        locals.var_psi_dn7 = assign87580_e133538_d_n7;
        locals.var_psi_dn8 = assign87580_e133538_d_n8;
        locals.var_psi_dn9 = assign87580_e133538_d_n9;
        locals.var_psi_dn10 = assign87580_e133538_d_n10;
        locals.var_psi_dn13 = assign87580_e133538_d_n13;

        let (assign87590_e133551, assign87590_e133551_d_n0, assign87590_e133551_d_n2, assign87590_e133551_d_n4, assign87590_e133551_d_n5, assign87590_e133551_d_n6, assign87590_e133551_d_n7, assign87590_e133551_d_n8, assign87590_e133551_d_n9, assign87590_e133551_d_n10, assign87590_e133551_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87590_e133548: f64 = (locals.var_beta * 0.1);
        let assign87590_e133549: f64 = (locals.var_psi + assign87590_e133548);
        (assign87590_e133549, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn13 + (locals.var_beta_dn13 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn13,)
    }
};
        locals.var_psi = assign87590_e133551;
        locals.var_psi_dn0 = assign87590_e133551_d_n0;
        locals.var_psi_dn2 = assign87590_e133551_d_n2;
        locals.var_psi_dn4 = assign87590_e133551_d_n4;
        locals.var_psi_dn5 = assign87590_e133551_d_n5;
        locals.var_psi_dn6 = assign87590_e133551_d_n6;
        locals.var_psi_dn7 = assign87590_e133551_d_n7;
        locals.var_psi_dn8 = assign87590_e133551_d_n8;
        locals.var_psi_dn9 = assign87590_e133551_d_n9;
        locals.var_psi_dn10 = assign87590_e133551_d_n10;
        locals.var_psi_dn13 = assign87590_e133551_d_n13;

        let (assign87600_e133572, assign87600_e133572_d_n0, assign87600_e133572_d_n2, assign87600_e133572_d_n4, assign87600_e133572_d_n5, assign87600_e133572_d_n6, assign87600_e133572_d_n7, assign87600_e133572_d_n8, assign87600_e133572_d_n9, assign87600_e133572_d_n10, assign87600_e133572_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87600_e133560: f64 = (locals.var_gammachi * locals.var_t0);
        let assign87600_e133563: f64 = (locals.var_psi * locals.var_psi);
        let assign87600_e133564: f64 = (assign87600_e133560 + assign87600_e133563);
        let assign87600_e133565: f64 = (assign87600_e133564).ln();
        let assign87600_e133568: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign87600_e133569: f64 = (assign87600_e133568).ln();
        let assign87600_e133570: f64 = (assign87600_e133565 - assign87600_e133569);
        (assign87600_e133570, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign87600_e133564) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign87600_e133568)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign87600_e133564) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign87600_e133568)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign87600_e133564) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign87600_e133568)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign87600_e133564) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign87600_e133568)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign87600_e133564) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign87600_e133568)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign87600_e133564) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign87600_e133568)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign87600_e133564) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign87600_e133568)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign87600_e133564) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign87600_e133568)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign87600_e133564) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign87600_e133568)), (((((locals.var_gammachi_dn13 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn13)) + ((locals.var_psi_dn13 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn13))) / assign87600_e133564) - (((locals.var_cnst1over_dn13 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn13)) / assign87600_e133568)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87600_e133572;
        locals.var_t1_dn0 = assign87600_e133572_d_n0;
        locals.var_t1_dn2 = assign87600_e133572_d_n2;
        locals.var_t1_dn4 = assign87600_e133572_d_n4;
        locals.var_t1_dn5 = assign87600_e133572_d_n5;
        locals.var_t1_dn6 = assign87600_e133572_d_n6;
        locals.var_t1_dn7 = assign87600_e133572_d_n7;
        locals.var_t1_dn8 = assign87600_e133572_d_n8;
        locals.var_t1_dn9 = assign87600_e133572_d_n9;
        locals.var_t1_dn10 = assign87600_e133572_d_n10;
        locals.var_t1_dn13 = assign87600_e133572_d_n13;

        let (assign87610_e133585, assign87610_e133585_d_n0, assign87610_e133585_d_n2, assign87610_e133585_d_n4, assign87610_e133585_d_n5, assign87610_e133585_d_n6, assign87610_e133585_d_n7, assign87610_e133585_d_n8, assign87610_e133585_d_n9, assign87610_e133585_d_n10, assign87610_e133585_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let assign87610_e133582: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign87610_e133583: f64 = (locals.var_t1 + assign87610_e133582);
        (assign87610_e133583, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn13 + ((locals.var_beta_dn13 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign87610_e133585;
        locals.var_chi_b_dn0 = assign87610_e133585_d_n0;
        locals.var_chi_b_dn2 = assign87610_e133585_d_n2;
        locals.var_chi_b_dn4 = assign87610_e133585_d_n4;
        locals.var_chi_b_dn5 = assign87610_e133585_d_n5;
        locals.var_chi_b_dn6 = assign87610_e133585_d_n6;
        locals.var_chi_b_dn7 = assign87610_e133585_d_n7;
        locals.var_chi_b_dn8 = assign87610_e133585_d_n8;
        locals.var_chi_b_dn9 = assign87610_e133585_d_n9;
        locals.var_chi_b_dn10 = assign87610_e133585_d_n10;
        locals.var_chi_b_dn13 = assign87610_e133585_d_n13;

        let (assign87620_e133599, assign87620_e133599_d_n0, assign87620_e133599_d_n2, assign87620_e133599_d_n4, assign87620_e133599_d_n5, assign87620_e133599_d_n6, assign87620_e133599_d_n7, assign87620_e133599_d_n8, assign87620_e133599_d_n9, assign87620_e133599_d_n10, assign87620_e133599_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        let (assign87620_e133597, assign87620_e133597_d_n0, assign87620_e133597_d_n2, assign87620_e133597_d_n4, assign87620_e133597_d_n5, assign87620_e133597_d_n6, assign87620_e133597_d_n7, assign87620_e133597_d_n8, assign87620_e133597_d_n9, assign87620_e133597_d_n10, assign87620_e133597_d_n13,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign87620_e133597, assign87620_e133597_d_n0, assign87620_e133597_d_n2, assign87620_e133597_d_n4, assign87620_e133597_d_n5, assign87620_e133597_d_n6, assign87620_e133597_d_n7, assign87620_e133597_d_n8, assign87620_e133597_d_n9, assign87620_e133597_d_n10, assign87620_e133597_d_n13,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign87620_e133599;
        locals.var_chi_b_dn0 = assign87620_e133599_d_n0;
        locals.var_chi_b_dn2 = assign87620_e133599_d_n2;
        locals.var_chi_b_dn4 = assign87620_e133599_d_n4;
        locals.var_chi_b_dn5 = assign87620_e133599_d_n5;
        locals.var_chi_b_dn6 = assign87620_e133599_d_n6;
        locals.var_chi_b_dn7 = assign87620_e133599_d_n7;
        locals.var_chi_b_dn8 = assign87620_e133599_d_n8;
        locals.var_chi_b_dn9 = assign87620_e133599_d_n9;
        locals.var_chi_b_dn10 = assign87620_e133599_d_n10;
        locals.var_chi_b_dn13 = assign87620_e133599_d_n13;

    }

    pub(super) fn stamp_transient_block_308(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87630_e133608, assign87630_e133608_d_n0, assign87630_e133608_d_n2, assign87630_e133608_d_n4, assign87630_e133608_d_n5, assign87630_e133608_d_n6, assign87630_e133608_d_n7, assign87630_e133608_d_n8, assign87630_e133608_d_n9, assign87630_e133608_d_n10, assign87630_e133608_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign87630_e133608;
        locals.var_chi_a_dn0 = assign87630_e133608_d_n0;
        locals.var_chi_a_dn2 = assign87630_e133608_d_n2;
        locals.var_chi_a_dn4 = assign87630_e133608_d_n4;
        locals.var_chi_a_dn5 = assign87630_e133608_d_n5;
        locals.var_chi_a_dn6 = assign87630_e133608_d_n6;
        locals.var_chi_a_dn7 = assign87630_e133608_d_n7;
        locals.var_chi_a_dn8 = assign87630_e133608_d_n8;
        locals.var_chi_a_dn9 = assign87630_e133608_d_n9;
        locals.var_chi_a_dn10 = assign87630_e133608_d_n10;
        locals.var_chi_a_dn13 = assign87630_e133608_d_n13;

        let assign87640_e133611: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2034 = assign87640_e133611;

        let assign87650_e133616: f64 = (0.2 * locals.var_chi_b);
        let assign87650_e133617: f64 = (locals.var_chi_b - assign87650_e133616);
        let assign87650_e133621: f64 = (0.2 * locals.var_chi_b);
        let assign87650_e133624: f64 = if ((locals.var_chi_a > assign87650_e133617) && (assign87650_e133621 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2035 = assign87650_e133624;

        let (assign87660_e133643, assign87660_e133643_d_n0, assign87660_e133643_d_n2, assign87660_e133643_d_n4, assign87660_e133643_d_n5, assign87660_e133643_d_n6, assign87660_e133643_d_n7, assign87660_e133643_d_n8, assign87660_e133643_d_n9, assign87660_e133643_d_n10, assign87660_e133643_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87660_e133637: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign87660_e133640: f64 = (0.2 * locals.var_chi_b);
        let assign87660_e133641: f64 = (assign87660_e133637 + assign87660_e133640);
        (assign87660_e133641, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn13 - locals.var_chi_b_dn13) + (0.2 * locals.var_chi_b_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign87660_e133643;
        locals.var_tmf1_dn0 = assign87660_e133643_d_n0;
        locals.var_tmf1_dn2 = assign87660_e133643_d_n2;
        locals.var_tmf1_dn4 = assign87660_e133643_d_n4;
        locals.var_tmf1_dn5 = assign87660_e133643_d_n5;
        locals.var_tmf1_dn6 = assign87660_e133643_d_n6;
        locals.var_tmf1_dn7 = assign87660_e133643_d_n7;
        locals.var_tmf1_dn8 = assign87660_e133643_d_n8;
        locals.var_tmf1_dn9 = assign87660_e133643_d_n9;
        locals.var_tmf1_dn10 = assign87660_e133643_d_n10;
        locals.var_tmf1_dn13 = assign87660_e133643_d_n13;

        let (assign87670_e133658, assign87670_e133658_d_n0, assign87670_e133658_d_n2, assign87670_e133658_d_n4, assign87670_e133658_d_n5, assign87670_e133658_d_n6, assign87670_e133658_d_n7, assign87670_e133658_d_n8, assign87670_e133658_d_n9, assign87670_e133658_d_n10, assign87670_e133658_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87670_e133656: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign87670_e133656, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign87670_e133658;
        locals.var_x2_dn0 = assign87670_e133658_d_n0;
        locals.var_x2_dn2 = assign87670_e133658_d_n2;
        locals.var_x2_dn4 = assign87670_e133658_d_n4;
        locals.var_x2_dn5 = assign87670_e133658_d_n5;
        locals.var_x2_dn6 = assign87670_e133658_d_n6;
        locals.var_x2_dn7 = assign87670_e133658_d_n7;
        locals.var_x2_dn8 = assign87670_e133658_d_n8;
        locals.var_x2_dn9 = assign87670_e133658_d_n9;
        locals.var_x2_dn10 = assign87670_e133658_d_n10;
        locals.var_x2_dn13 = assign87670_e133658_d_n13;

        let (assign87680_e133677, assign87680_e133677_d_n0, assign87680_e133677_d_n2, assign87680_e133677_d_n4, assign87680_e133677_d_n5, assign87680_e133677_d_n6, assign87680_e133677_d_n7, assign87680_e133677_d_n8, assign87680_e133677_d_n9, assign87680_e133677_d_n10, assign87680_e133677_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87680_e133671: f64 = (0.2 * locals.var_chi_b);
        let assign87680_e133674: f64 = (0.2 * locals.var_chi_b);
        let assign87680_e133675: f64 = (assign87680_e133671 * assign87680_e133674);
        (assign87680_e133675, (((0.2 * locals.var_chi_b_dn0) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn13) * assign87680_e133674) + (assign87680_e133671 * (0.2 * locals.var_chi_b_dn13))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign87680_e133677;
        locals.var_xmax2_dn0 = assign87680_e133677_d_n0;
        locals.var_xmax2_dn2 = assign87680_e133677_d_n2;
        locals.var_xmax2_dn4 = assign87680_e133677_d_n4;
        locals.var_xmax2_dn5 = assign87680_e133677_d_n5;
        locals.var_xmax2_dn6 = assign87680_e133677_d_n6;
        locals.var_xmax2_dn7 = assign87680_e133677_d_n7;
        locals.var_xmax2_dn8 = assign87680_e133677_d_n8;
        locals.var_xmax2_dn9 = assign87680_e133677_d_n9;
        locals.var_xmax2_dn10 = assign87680_e133677_d_n10;
        locals.var_xmax2_dn13 = assign87680_e133677_d_n13;

        let (assign87690_e133690, assign87690_e133690_d_n0, assign87690_e133690_d_n2, assign87690_e133690_d_n4, assign87690_e133690_d_n5, assign87690_e133690_d_n6, assign87690_e133690_d_n7, assign87690_e133690_d_n8, assign87690_e133690_d_n9, assign87690_e133690_d_n10, assign87690_e133690_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign87690_e133690;
        locals.var_xp_dn0 = assign87690_e133690_d_n0;
        locals.var_xp_dn2 = assign87690_e133690_d_n2;
        locals.var_xp_dn4 = assign87690_e133690_d_n4;
        locals.var_xp_dn5 = assign87690_e133690_d_n5;
        locals.var_xp_dn6 = assign87690_e133690_d_n6;
        locals.var_xp_dn7 = assign87690_e133690_d_n7;
        locals.var_xp_dn8 = assign87690_e133690_d_n8;
        locals.var_xp_dn9 = assign87690_e133690_d_n9;
        locals.var_xp_dn10 = assign87690_e133690_d_n10;
        locals.var_xp_dn13 = assign87690_e133690_d_n13;

        let (assign87700_e133703, assign87700_e133703_d_n0, assign87700_e133703_d_n2, assign87700_e133703_d_n4, assign87700_e133703_d_n5, assign87700_e133703_d_n6, assign87700_e133703_d_n7, assign87700_e133703_d_n8, assign87700_e133703_d_n9, assign87700_e133703_d_n10, assign87700_e133703_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign87700_e133703;
        locals.var_xmp_dn0 = assign87700_e133703_d_n0;
        locals.var_xmp_dn2 = assign87700_e133703_d_n2;
        locals.var_xmp_dn4 = assign87700_e133703_d_n4;
        locals.var_xmp_dn5 = assign87700_e133703_d_n5;
        locals.var_xmp_dn6 = assign87700_e133703_d_n6;
        locals.var_xmp_dn7 = assign87700_e133703_d_n7;
        locals.var_xmp_dn8 = assign87700_e133703_d_n8;
        locals.var_xmp_dn9 = assign87700_e133703_d_n9;
        locals.var_xmp_dn10 = assign87700_e133703_d_n10;
        locals.var_xmp_dn13 = assign87700_e133703_d_n13;

        let (assign87710_e133716,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign87710_e133716;

        let (assign87720_e133729,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87720_e133729;

        let (assign87730_e133742, assign87730_e133742_d_n0, assign87730_e133742_d_n2, assign87730_e133742_d_n4, assign87730_e133742_d_n5, assign87730_e133742_d_n6, assign87730_e133742_d_n7, assign87730_e133742_d_n8, assign87730_e133742_d_n9, assign87730_e133742_d_n10, assign87730_e133742_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign87730_e133742;
        locals.var_arg_dn0 = assign87730_e133742_d_n0;
        locals.var_arg_dn2 = assign87730_e133742_d_n2;
        locals.var_arg_dn4 = assign87730_e133742_d_n4;
        locals.var_arg_dn5 = assign87730_e133742_d_n5;
        locals.var_arg_dn6 = assign87730_e133742_d_n6;
        locals.var_arg_dn7 = assign87730_e133742_d_n7;
        locals.var_arg_dn8 = assign87730_e133742_d_n8;
        locals.var_arg_dn9 = assign87730_e133742_d_n9;
        locals.var_arg_dn10 = assign87730_e133742_d_n10;
        locals.var_arg_dn13 = assign87730_e133742_d_n13;

        let (assign87740_e133755, assign87740_e133755_d_n0, assign87740_e133755_d_n2, assign87740_e133755_d_n4, assign87740_e133755_d_n5, assign87740_e133755_d_n6, assign87740_e133755_d_n7, assign87740_e133755_d_n8, assign87740_e133755_d_n9, assign87740_e133755_d_n10, assign87740_e133755_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign87740_e133755;
        locals.var_dnm_dn0 = assign87740_e133755_d_n0;
        locals.var_dnm_dn2 = assign87740_e133755_d_n2;
        locals.var_dnm_dn4 = assign87740_e133755_d_n4;
        locals.var_dnm_dn5 = assign87740_e133755_d_n5;
        locals.var_dnm_dn6 = assign87740_e133755_d_n6;
        locals.var_dnm_dn7 = assign87740_e133755_d_n7;
        locals.var_dnm_dn8 = assign87740_e133755_d_n8;
        locals.var_dnm_dn9 = assign87740_e133755_d_n9;
        locals.var_dnm_dn10 = assign87740_e133755_d_n10;
        locals.var_dnm_dn13 = assign87740_e133755_d_n13;

        let (assign87750_e133770, assign87750_e133770_d_n0, assign87750_e133770_d_n2, assign87750_e133770_d_n4, assign87750_e133770_d_n5, assign87750_e133770_d_n6, assign87750_e133770_d_n7, assign87750_e133770_d_n8, assign87750_e133770_d_n9, assign87750_e133770_d_n10, assign87750_e133770_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87750_e133768: f64 = (locals.var_xp * locals.var_x2);
        (assign87750_e133768, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign87750_e133770;
        locals.var_xp_dn0 = assign87750_e133770_d_n0;
        locals.var_xp_dn2 = assign87750_e133770_d_n2;
        locals.var_xp_dn4 = assign87750_e133770_d_n4;
        locals.var_xp_dn5 = assign87750_e133770_d_n5;
        locals.var_xp_dn6 = assign87750_e133770_d_n6;
        locals.var_xp_dn7 = assign87750_e133770_d_n7;
        locals.var_xp_dn8 = assign87750_e133770_d_n8;
        locals.var_xp_dn9 = assign87750_e133770_d_n9;
        locals.var_xp_dn10 = assign87750_e133770_d_n10;
        locals.var_xp_dn13 = assign87750_e133770_d_n13;

        let (assign87760_e133785, assign87760_e133785_d_n0, assign87760_e133785_d_n2, assign87760_e133785_d_n4, assign87760_e133785_d_n5, assign87760_e133785_d_n6, assign87760_e133785_d_n7, assign87760_e133785_d_n8, assign87760_e133785_d_n9, assign87760_e133785_d_n10, assign87760_e133785_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87760_e133783: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign87760_e133783, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign87760_e133785;
        locals.var_xmp_dn0 = assign87760_e133785_d_n0;
        locals.var_xmp_dn2 = assign87760_e133785_d_n2;
        locals.var_xmp_dn4 = assign87760_e133785_d_n4;
        locals.var_xmp_dn5 = assign87760_e133785_d_n5;
        locals.var_xmp_dn6 = assign87760_e133785_d_n6;
        locals.var_xmp_dn7 = assign87760_e133785_d_n7;
        locals.var_xmp_dn8 = assign87760_e133785_d_n8;
        locals.var_xmp_dn9 = assign87760_e133785_d_n9;
        locals.var_xmp_dn10 = assign87760_e133785_d_n10;
        locals.var_xmp_dn13 = assign87760_e133785_d_n13;

        let (assign87770_e133800, assign87770_e133800_d_n0, assign87770_e133800_d_n2, assign87770_e133800_d_n4, assign87770_e133800_d_n5, assign87770_e133800_d_n6, assign87770_e133800_d_n7, assign87770_e133800_d_n8, assign87770_e133800_d_n9, assign87770_e133800_d_n10, assign87770_e133800_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87770_e133798: f64 = (locals.var_xp * locals.var_x2);
        (assign87770_e133798, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign87770_e133800;
        locals.var_xp_dn0 = assign87770_e133800_d_n0;
        locals.var_xp_dn2 = assign87770_e133800_d_n2;
        locals.var_xp_dn4 = assign87770_e133800_d_n4;
        locals.var_xp_dn5 = assign87770_e133800_d_n5;
        locals.var_xp_dn6 = assign87770_e133800_d_n6;
        locals.var_xp_dn7 = assign87770_e133800_d_n7;
        locals.var_xp_dn8 = assign87770_e133800_d_n8;
        locals.var_xp_dn9 = assign87770_e133800_d_n9;
        locals.var_xp_dn10 = assign87770_e133800_d_n10;
        locals.var_xp_dn13 = assign87770_e133800_d_n13;

        let (assign87780_e133815, assign87780_e133815_d_n0, assign87780_e133815_d_n2, assign87780_e133815_d_n4, assign87780_e133815_d_n5, assign87780_e133815_d_n6, assign87780_e133815_d_n7, assign87780_e133815_d_n8, assign87780_e133815_d_n9, assign87780_e133815_d_n10, assign87780_e133815_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87780_e133813: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign87780_e133813, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign87780_e133815;
        locals.var_xmp_dn0 = assign87780_e133815_d_n0;
        locals.var_xmp_dn2 = assign87780_e133815_d_n2;
        locals.var_xmp_dn4 = assign87780_e133815_d_n4;
        locals.var_xmp_dn5 = assign87780_e133815_d_n5;
        locals.var_xmp_dn6 = assign87780_e133815_d_n6;
        locals.var_xmp_dn7 = assign87780_e133815_d_n7;
        locals.var_xmp_dn8 = assign87780_e133815_d_n8;
        locals.var_xmp_dn9 = assign87780_e133815_d_n9;
        locals.var_xmp_dn10 = assign87780_e133815_d_n10;
        locals.var_xmp_dn13 = assign87780_e133815_d_n13;

        let (assign87790_e133830, assign87790_e133830_d_n0, assign87790_e133830_d_n2, assign87790_e133830_d_n4, assign87790_e133830_d_n5, assign87790_e133830_d_n6, assign87790_e133830_d_n7, assign87790_e133830_d_n8, assign87790_e133830_d_n9, assign87790_e133830_d_n10, assign87790_e133830_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87790_e133828: f64 = (locals.var_xp + locals.var_xmp);
        (assign87790_e133828, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign87790_e133830;
        locals.var_arg_dn0 = assign87790_e133830_d_n0;
        locals.var_arg_dn2 = assign87790_e133830_d_n2;
        locals.var_arg_dn4 = assign87790_e133830_d_n4;
        locals.var_arg_dn5 = assign87790_e133830_d_n5;
        locals.var_arg_dn6 = assign87790_e133830_d_n6;
        locals.var_arg_dn7 = assign87790_e133830_d_n7;
        locals.var_arg_dn8 = assign87790_e133830_d_n8;
        locals.var_arg_dn9 = assign87790_e133830_d_n9;
        locals.var_arg_dn10 = assign87790_e133830_d_n10;
        locals.var_arg_dn13 = assign87790_e133830_d_n13;

        let (assign87800_e133843, assign87800_e133843_d_n0, assign87800_e133843_d_n2, assign87800_e133843_d_n4, assign87800_e133843_d_n5, assign87800_e133843_d_n6, assign87800_e133843_d_n7, assign87800_e133843_d_n8, assign87800_e133843_d_n9, assign87800_e133843_d_n10, assign87800_e133843_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign87800_e133843;
        locals.var_dnm_dn0 = assign87800_e133843_d_n0;
        locals.var_dnm_dn2 = assign87800_e133843_d_n2;
        locals.var_dnm_dn4 = assign87800_e133843_d_n4;
        locals.var_dnm_dn5 = assign87800_e133843_d_n5;
        locals.var_dnm_dn6 = assign87800_e133843_d_n6;
        locals.var_dnm_dn7 = assign87800_e133843_d_n7;
        locals.var_dnm_dn8 = assign87800_e133843_d_n8;
        locals.var_dnm_dn9 = assign87800_e133843_d_n9;
        locals.var_dnm_dn10 = assign87800_e133843_d_n10;
        locals.var_dnm_dn13 = assign87800_e133843_d_n13;

        let assign87810_e133858: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2036 = assign87810_e133858;

        let assign87820_e133861: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2037 = assign87820_e133861;

        let (assign87830_e133878,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87830_e133878;

        let assign87840_e133881: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2038 = assign87840_e133881;

        let (assign87850_e133901,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 == 0.0)) && (locals.var_guard2038 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87850_e133901;

        let assign87860_e133904: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2039 = assign87860_e133904;

        let (assign87870_e133927,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 == 0.0)) && (locals.var_guard2038 == 0.0)) && (locals.var_guard2039 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87870_e133927;

        let assign87880_e133930: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2040 = assign87880_e133930;

        let (assign87890_e133956,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_guard2037 == 0.0)) && (locals.var_guard2038 == 0.0)) && (locals.var_guard2039 == 0.0)) && (locals.var_guard2040 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign87890_e133956;

        let (assign87900_e133971,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign87900_e133971;

        let mut assign87910_loop_guard: usize = 0;
        while {
            let assign87910_cond_e133987: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign87910_cond_e133987 != 0.0
        } {
            assign87910_loop_guard += 1;
            assert!(assign87910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign87910_body0_e134003, assign87910_body0_e134003_d_n0, assign87910_body0_e134003_d_n2, assign87910_body0_e134003_d_n4, assign87910_body0_e134003_d_n5, assign87910_body0_e134003_d_n6, assign87910_body0_e134003_d_n7, assign87910_body0_e134003_d_n8, assign87910_body0_e134003_d_n9, assign87910_body0_e134003_d_n10, assign87910_body0_e134003_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) {
        let assign87910_body0_e134001: f64 = (locals.var_dnm).sqrt();
        (assign87910_body0_e134001, (locals.var_dnm_dn0 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn2 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn4 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn5 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn6 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn7 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn8 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn9 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn10 / (2.0 * assign87910_body0_e134001)), (locals.var_dnm_dn13 / (2.0 * assign87910_body0_e134001)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign87910_body0_e134003;
            locals.var_dnm_dn0 = assign87910_body0_e134003_d_n0;
            locals.var_dnm_dn2 = assign87910_body0_e134003_d_n2;
            locals.var_dnm_dn4 = assign87910_body0_e134003_d_n4;
            locals.var_dnm_dn5 = assign87910_body0_e134003_d_n5;
            locals.var_dnm_dn6 = assign87910_body0_e134003_d_n6;
            locals.var_dnm_dn7 = assign87910_body0_e134003_d_n7;
            locals.var_dnm_dn8 = assign87910_body0_e134003_d_n8;
            locals.var_dnm_dn9 = assign87910_body0_e134003_d_n9;
            locals.var_dnm_dn10 = assign87910_body0_e134003_d_n10;
            locals.var_dnm_dn13 = assign87910_body0_e134003_d_n13;
            let (assign87910_body1_e134020,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 != 0.0)) {
        let assign87910_body1_e134018: f64 = (locals.var_m0 + 1.0);
        (assign87910_body1_e134018,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign87910_body1_e134020;
        }

        let (assign87920_e134047, assign87920_e134047_d_n0, assign87920_e134047_d_n2, assign87920_e134047_d_n4, assign87920_e134047_d_n5, assign87920_e134047_d_n6, assign87920_e134047_d_n7, assign87920_e134047_d_n8, assign87920_e134047_d_n9, assign87920_e134047_d_n10, assign87920_e134047_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) && (locals.var_guard2036 == 0.0)) {
        let (assign87920_e134045, assign87920_e134045_d_n0, assign87920_e134045_d_n2, assign87920_e134045_d_n4, assign87920_e134045_d_n5, assign87920_e134045_d_n6, assign87920_e134045_d_n7, assign87920_e134045_d_n8, assign87920_e134045_d_n9, assign87920_e134045_d_n10, assign87920_e134045_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign87920_e134042: f64 = (2.0 * 2.0);
                let assign87920_e134043: f64 = (1.0 / assign87920_e134042);
                let assign87920_e134044: f64 = (locals.var_dnm).powf(assign87920_e134043);
                (assign87920_e134044, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn0)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn2)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn4)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn5)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn6)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn7)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn8)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn9)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn10)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign87920_e134043) as f64).is_finite() && ((assign87920_e134043) as f64).fract() == 0.0 { if assign87920_e134043 == 0.0 { 0.0 } else { (assign87920_e134043 * ((locals.var_dnm).powf(assign87920_e134043 - 1.0) * locals.var_dnm_dn13)) } } else { (assign87920_e134044 * (assign87920_e134043 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign87920_e134045, assign87920_e134045_d_n0, assign87920_e134045_d_n2, assign87920_e134045_d_n4, assign87920_e134045_d_n5, assign87920_e134045_d_n6, assign87920_e134045_d_n7, assign87920_e134045_d_n8, assign87920_e134045_d_n9, assign87920_e134045_d_n10, assign87920_e134045_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign87920_e134047;
        locals.var_dnm_dn0 = assign87920_e134047_d_n0;
        locals.var_dnm_dn2 = assign87920_e134047_d_n2;
        locals.var_dnm_dn4 = assign87920_e134047_d_n4;
        locals.var_dnm_dn5 = assign87920_e134047_d_n5;
        locals.var_dnm_dn6 = assign87920_e134047_d_n6;
        locals.var_dnm_dn7 = assign87920_e134047_d_n7;
        locals.var_dnm_dn8 = assign87920_e134047_d_n8;
        locals.var_dnm_dn9 = assign87920_e134047_d_n9;
        locals.var_dnm_dn10 = assign87920_e134047_d_n10;
        locals.var_dnm_dn13 = assign87920_e134047_d_n13;

        let (assign87930_e134062, assign87930_e134062_d_n0, assign87930_e134062_d_n2, assign87930_e134062_d_n4, assign87930_e134062_d_n5, assign87930_e134062_d_n6, assign87930_e134062_d_n7, assign87930_e134062_d_n8, assign87930_e134062_d_n9, assign87930_e134062_d_n10, assign87930_e134062_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87930_e134060: f64 = (1.0 / locals.var_dnm);
        (assign87930_e134060, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign87930_e134062;
        locals.var_dnm_dn0 = assign87930_e134062_d_n0;
        locals.var_dnm_dn2 = assign87930_e134062_d_n2;
        locals.var_dnm_dn4 = assign87930_e134062_d_n4;
        locals.var_dnm_dn5 = assign87930_e134062_d_n5;
        locals.var_dnm_dn6 = assign87930_e134062_d_n6;
        locals.var_dnm_dn7 = assign87930_e134062_d_n7;
        locals.var_dnm_dn8 = assign87930_e134062_d_n8;
        locals.var_dnm_dn9 = assign87930_e134062_d_n9;
        locals.var_dnm_dn10 = assign87930_e134062_d_n10;
        locals.var_dnm_dn13 = assign87930_e134062_d_n13;

        let (assign87940_e134081, assign87940_e134081_d_n0, assign87940_e134081_d_n2, assign87940_e134081_d_n4, assign87940_e134081_d_n5, assign87940_e134081_d_n6, assign87940_e134081_d_n7, assign87940_e134081_d_n8, assign87940_e134081_d_n9, assign87940_e134081_d_n10, assign87940_e134081_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87940_e134076: f64 = (0.2 * locals.var_chi_b);
        let assign87940_e134077: f64 = (locals.var_tmf1 * assign87940_e134076);
        let assign87940_e134079: f64 = (assign87940_e134077 * locals.var_dnm);
        (assign87940_e134079, ((((locals.var_tmf1_dn0 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign87940_e134076) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn13))) * locals.var_dnm) + (assign87940_e134077 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign87940_e134081;
        locals.var_tmf0_dn0 = assign87940_e134081_d_n0;
        locals.var_tmf0_dn2 = assign87940_e134081_d_n2;
        locals.var_tmf0_dn4 = assign87940_e134081_d_n4;
        locals.var_tmf0_dn5 = assign87940_e134081_d_n5;
        locals.var_tmf0_dn6 = assign87940_e134081_d_n6;
        locals.var_tmf0_dn7 = assign87940_e134081_d_n7;
        locals.var_tmf0_dn8 = assign87940_e134081_d_n8;
        locals.var_tmf0_dn9 = assign87940_e134081_d_n9;
        locals.var_tmf0_dn10 = assign87940_e134081_d_n10;
        locals.var_tmf0_dn13 = assign87940_e134081_d_n13;

        let (assign87950_e134102, assign87950_e134102_d_n0, assign87950_e134102_d_n2, assign87950_e134102_d_n4, assign87950_e134102_d_n5, assign87950_e134102_d_n6, assign87950_e134102_d_n7, assign87950_e134102_d_n8, assign87950_e134102_d_n9, assign87950_e134102_d_n10, assign87950_e134102_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87950_e134094: f64 = (0.2 * locals.var_chi_b);
        let assign87950_e134096: f64 = (assign87950_e134094 * locals.var_xmp);
        let assign87950_e134098: f64 = (assign87950_e134096 * locals.var_dnm);
        let assign87950_e134100: f64 = (assign87950_e134098 / locals.var_arg);
        (assign87950_e134100, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn0)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn2)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn4)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn5)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn6)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn7)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn8)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn9)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn10)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn13) * locals.var_xmp) + (assign87950_e134094 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign87950_e134096 * locals.var_dnm_dn13)) * locals.var_arg) - (assign87950_e134098 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87950_e134102;
        locals.var_t1_dn0 = assign87950_e134102_d_n0;
        locals.var_t1_dn2 = assign87950_e134102_d_n2;
        locals.var_t1_dn4 = assign87950_e134102_d_n4;
        locals.var_t1_dn5 = assign87950_e134102_d_n5;
        locals.var_t1_dn6 = assign87950_e134102_d_n6;
        locals.var_t1_dn7 = assign87950_e134102_d_n7;
        locals.var_t1_dn8 = assign87950_e134102_d_n8;
        locals.var_t1_dn9 = assign87950_e134102_d_n9;
        locals.var_t1_dn10 = assign87950_e134102_d_n10;
        locals.var_t1_dn13 = assign87950_e134102_d_n13;

    }

    pub(super) fn stamp_transient_block_309(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign87960_e134121, assign87960_e134121_d_n0, assign87960_e134121_d_n2, assign87960_e134121_d_n4, assign87960_e134121_d_n5, assign87960_e134121_d_n6, assign87960_e134121_d_n7, assign87960_e134121_d_n8, assign87960_e134121_d_n9, assign87960_e134121_d_n10, assign87960_e134121_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        let assign87960_e134116: f64 = (0.2 * locals.var_chi_b);
        let assign87960_e134117: f64 = (locals.var_chi_b - assign87960_e134116);
        let assign87960_e134119: f64 = (assign87960_e134117 + locals.var_tmf0);
        (assign87960_e134119, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn13 - (0.2 * locals.var_chi_b_dn13)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87960_e134121;
        locals.var_chi_dn0 = assign87960_e134121_d_n0;
        locals.var_chi_dn2 = assign87960_e134121_d_n2;
        locals.var_chi_dn4 = assign87960_e134121_d_n4;
        locals.var_chi_dn5 = assign87960_e134121_d_n5;
        locals.var_chi_dn6 = assign87960_e134121_d_n6;
        locals.var_chi_dn7 = assign87960_e134121_d_n7;
        locals.var_chi_dn8 = assign87960_e134121_d_n8;
        locals.var_chi_dn9 = assign87960_e134121_d_n9;
        locals.var_chi_dn10 = assign87960_e134121_d_n10;
        locals.var_chi_dn13 = assign87960_e134121_d_n13;

        let (assign87970_e134134, assign87970_e134134_d_n0, assign87970_e134134_d_n2, assign87970_e134134_d_n4, assign87970_e134134_d_n5, assign87970_e134134_d_n6, assign87970_e134134_d_n7, assign87970_e134134_d_n8, assign87970_e134134_d_n9, assign87970_e134134_d_n10, assign87970_e134134_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87970_e134134;
        locals.var_t1_dn0 = assign87970_e134134_d_n0;
        locals.var_t1_dn2 = assign87970_e134134_d_n2;
        locals.var_t1_dn4 = assign87970_e134134_d_n4;
        locals.var_t1_dn5 = assign87970_e134134_d_n5;
        locals.var_t1_dn6 = assign87970_e134134_d_n6;
        locals.var_t1_dn7 = assign87970_e134134_d_n7;
        locals.var_t1_dn8 = assign87970_e134134_d_n8;
        locals.var_t1_dn9 = assign87970_e134134_d_n9;
        locals.var_t1_dn10 = assign87970_e134134_d_n10;
        locals.var_t1_dn13 = assign87970_e134134_d_n13;

        let (assign87980_e134148, assign87980_e134148_d_n0, assign87980_e134148_d_n2, assign87980_e134148_d_n4, assign87980_e134148_d_n5, assign87980_e134148_d_n6, assign87980_e134148_d_n7, assign87980_e134148_d_n8, assign87980_e134148_d_n9, assign87980_e134148_d_n10, assign87980_e134148_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign87980_e134148;
        locals.var_chi_dn0 = assign87980_e134148_d_n0;
        locals.var_chi_dn2 = assign87980_e134148_d_n2;
        locals.var_chi_dn4 = assign87980_e134148_d_n4;
        locals.var_chi_dn5 = assign87980_e134148_d_n5;
        locals.var_chi_dn6 = assign87980_e134148_d_n6;
        locals.var_chi_dn7 = assign87980_e134148_d_n7;
        locals.var_chi_dn8 = assign87980_e134148_d_n8;
        locals.var_chi_dn9 = assign87980_e134148_d_n9;
        locals.var_chi_dn10 = assign87980_e134148_d_n10;
        locals.var_chi_dn13 = assign87980_e134148_d_n13;

        let (assign87990_e134162, assign87990_e134162_d_n0, assign87990_e134162_d_n2, assign87990_e134162_d_n4, assign87990_e134162_d_n5, assign87990_e134162_d_n6, assign87990_e134162_d_n7, assign87990_e134162_d_n8, assign87990_e134162_d_n9, assign87990_e134162_d_n10, assign87990_e134162_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 != 0.0)) && (locals.var_guard2035 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign87990_e134162;
        locals.var_t1_dn0 = assign87990_e134162_d_n0;
        locals.var_t1_dn2 = assign87990_e134162_d_n2;
        locals.var_t1_dn4 = assign87990_e134162_d_n4;
        locals.var_t1_dn5 = assign87990_e134162_d_n5;
        locals.var_t1_dn6 = assign87990_e134162_d_n6;
        locals.var_t1_dn7 = assign87990_e134162_d_n7;
        locals.var_t1_dn8 = assign87990_e134162_d_n8;
        locals.var_t1_dn9 = assign87990_e134162_d_n9;
        locals.var_t1_dn10 = assign87990_e134162_d_n10;
        locals.var_t1_dn13 = assign87990_e134162_d_n13;

        let (assign88000_e134179, assign88000_e134179_d_n0, assign88000_e134179_d_n2, assign88000_e134179_d_n4, assign88000_e134179_d_n5, assign88000_e134179_d_n6, assign88000_e134179_d_n7, assign88000_e134179_d_n8, assign88000_e134179_d_n9, assign88000_e134179_d_n10, assign88000_e134179_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2032 != 0.0)) && (locals.var_guard2034 == 0.0)) {
        let (assign88000_e134177, assign88000_e134177_d_n0, assign88000_e134177_d_n2, assign88000_e134177_d_n4, assign88000_e134177_d_n5, assign88000_e134177_d_n6, assign88000_e134177_d_n7, assign88000_e134177_d_n8, assign88000_e134177_d_n9, assign88000_e134177_d_n10, assign88000_e134177_d_n13,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            }
        };
        (assign88000_e134177, assign88000_e134177_d_n0, assign88000_e134177_d_n2, assign88000_e134177_d_n4, assign88000_e134177_d_n5, assign88000_e134177_d_n6, assign88000_e134177_d_n7, assign88000_e134177_d_n8, assign88000_e134177_d_n9, assign88000_e134177_d_n10, assign88000_e134177_d_n13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign88000_e134179;
        locals.var_chi_dn0 = assign88000_e134179_d_n0;
        locals.var_chi_dn2 = assign88000_e134179_d_n2;
        locals.var_chi_dn4 = assign88000_e134179_d_n4;
        locals.var_chi_dn5 = assign88000_e134179_d_n5;
        locals.var_chi_dn6 = assign88000_e134179_d_n6;
        locals.var_chi_dn7 = assign88000_e134179_d_n7;
        locals.var_chi_dn8 = assign88000_e134179_d_n8;
        locals.var_chi_dn9 = assign88000_e134179_d_n9;
        locals.var_chi_dn10 = assign88000_e134179_d_n10;
        locals.var_chi_dn13 = assign88000_e134179_d_n13;

        let assign88010_e134182: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2041 = assign88010_e134182;

        let (assign88020_e134195, assign88020_e134195_d_n0, assign88020_e134195_d_n2, assign88020_e134195_d_n4, assign88020_e134195_d_n5, assign88020_e134195_d_n6, assign88020_e134195_d_n7, assign88020_e134195_d_n8, assign88020_e134195_d_n9, assign88020_e134195_d_n10, assign88020_e134195_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88020_e134191: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign88020_e134193: f64 = (assign88020_e134191 - locals.var_vxbgmtcl);
        (assign88020_e134193, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign88020_e134195;
        locals.var_ps0ld_dn0 = assign88020_e134195_d_n0;
        locals.var_ps0ld_dn2 = assign88020_e134195_d_n2;
        locals.var_ps0ld_dn4 = assign88020_e134195_d_n4;
        locals.var_ps0ld_dn5 = assign88020_e134195_d_n5;
        locals.var_ps0ld_dn6 = assign88020_e134195_d_n6;
        locals.var_ps0ld_dn7 = assign88020_e134195_d_n7;
        locals.var_ps0ld_dn8 = assign88020_e134195_d_n8;
        locals.var_ps0ld_dn9 = assign88020_e134195_d_n9;
        locals.var_ps0ld_dn10 = assign88020_e134195_d_n10;
        locals.var_ps0ld_dn13 = assign88020_e134195_d_n13;

        let assign88030_e134198: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2042 = assign88030_e134198;

        let (assign88040_e134211, assign88040_e134211_d_n0, assign88040_e134211_d_n2, assign88040_e134211_d_n4, assign88040_e134211_d_n5, assign88040_e134211_d_n6, assign88040_e134211_d_n7, assign88040_e134211_d_n8, assign88040_e134211_d_n9, assign88040_e134211_d_n10, assign88040_e134211_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 != 0.0)) {
        let assign88040_e134209: f64 = (p.p334 - locals.var_wdep_func);
        (assign88040_e134209, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88040_e134211;
        locals.var_t2_dn0 = assign88040_e134211_d_n0;
        locals.var_t2_dn2 = assign88040_e134211_d_n2;
        locals.var_t2_dn4 = assign88040_e134211_d_n4;
        locals.var_t2_dn5 = assign88040_e134211_d_n5;
        locals.var_t2_dn6 = assign88040_e134211_d_n6;
        locals.var_t2_dn7 = assign88040_e134211_d_n7;
        locals.var_t2_dn8 = assign88040_e134211_d_n8;
        locals.var_t2_dn9 = assign88040_e134211_d_n9;
        locals.var_t2_dn10 = assign88040_e134211_d_n10;
        locals.var_t2_dn13 = assign88040_e134211_d_n13;

        let (assign88050_e134236, assign88050_e134236_d_n0, assign88050_e134236_d_n2, assign88050_e134236_d_n4, assign88050_e134236_d_n5, assign88050_e134236_d_n6, assign88050_e134236_d_n7, assign88050_e134236_d_n8, assign88050_e134236_d_n9, assign88050_e134236_d_n10, assign88050_e134236_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88050_e134223: f64 = (locals.var_vdsi + p.p137);
        let assign88050_e134226: f64 = (locals.var_vdsi + p.p137);
        let assign88050_e134227: f64 = (assign88050_e134223 * assign88050_e134226);
        let assign88050_e134230: f64 = (4.0 * 0.1);
        let assign88050_e134232: f64 = (assign88050_e134230 * 0.1);
        let assign88050_e134233: f64 = (assign88050_e134227 + assign88050_e134232);
        let assign88050_e134234: f64 = (assign88050_e134233).sqrt();
        (assign88050_e134234, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign88050_e134226) + (assign88050_e134223 * locals.var_vdsi_dn5)) / (2.0 * assign88050_e134234)), 0.0, (((locals.var_vdsi_dn7 * assign88050_e134226) + (assign88050_e134223 * locals.var_vdsi_dn7)) / (2.0 * assign88050_e134234)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign88050_e134236;
        locals.var_tmf2_dn0 = assign88050_e134236_d_n0;
        locals.var_tmf2_dn2 = assign88050_e134236_d_n2;
        locals.var_tmf2_dn4 = assign88050_e134236_d_n4;
        locals.var_tmf2_dn5 = assign88050_e134236_d_n5;
        locals.var_tmf2_dn6 = assign88050_e134236_d_n6;
        locals.var_tmf2_dn7 = assign88050_e134236_d_n7;
        locals.var_tmf2_dn8 = assign88050_e134236_d_n8;
        locals.var_tmf2_dn9 = assign88050_e134236_d_n9;
        locals.var_tmf2_dn10 = assign88050_e134236_d_n10;
        locals.var_tmf2_dn13 = assign88050_e134236_d_n13;

        let (assign88060_e134256, assign88060_e134256_d_n0, assign88060_e134256_d_n2, assign88060_e134256_d_n4, assign88060_e134256_d_n5, assign88060_e134256_d_n6, assign88060_e134256_d_n7, assign88060_e134256_d_n8, assign88060_e134256_d_n9, assign88060_e134256_d_n10, assign88060_e134256_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88060_e134250: f64 = (locals.var_vdsi + p.p137);
        let assign88060_e134252: f64 = (assign88060_e134250 / locals.var_tmf2);
        let assign88060_e134253: f64 = (1.0 + assign88060_e134252);
        let assign88060_e134254: f64 = (0.5 * assign88060_e134253);
        (assign88060_e134254, (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign88060_e134250 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign88060_e134250 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign88060_e134250 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88060_e134256;
        locals.var_t9_dn0 = assign88060_e134256_d_n0;
        locals.var_t9_dn2 = assign88060_e134256_d_n2;
        locals.var_t9_dn4 = assign88060_e134256_d_n4;
        locals.var_t9_dn5 = assign88060_e134256_d_n5;
        locals.var_t9_dn6 = assign88060_e134256_d_n6;
        locals.var_t9_dn7 = assign88060_e134256_d_n7;
        locals.var_t9_dn8 = assign88060_e134256_d_n8;
        locals.var_t9_dn9 = assign88060_e134256_d_n9;
        locals.var_t9_dn10 = assign88060_e134256_d_n10;
        locals.var_t9_dn13 = assign88060_e134256_d_n13;

        let (assign88070_e134274, assign88070_e134274_d_n0, assign88070_e134274_d_n2, assign88070_e134274_d_n4, assign88070_e134274_d_n5, assign88070_e134274_d_n6, assign88070_e134274_d_n7, assign88070_e134274_d_n8, assign88070_e134274_d_n9, assign88070_e134274_d_n10, assign88070_e134274_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88070_e134269: f64 = (locals.var_vdsi + p.p137);
        let assign88070_e134271: f64 = (assign88070_e134269 + locals.var_tmf2);
        let assign88070_e134272: f64 = (0.5 * assign88070_e134271);
        (assign88070_e134272, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88070_e134274;
        locals.var_t2_dn0 = assign88070_e134274_d_n0;
        locals.var_t2_dn2 = assign88070_e134274_d_n2;
        locals.var_t2_dn4 = assign88070_e134274_d_n4;
        locals.var_t2_dn5 = assign88070_e134274_d_n5;
        locals.var_t2_dn6 = assign88070_e134274_d_n6;
        locals.var_t2_dn7 = assign88070_e134274_d_n7;
        locals.var_t2_dn8 = assign88070_e134274_d_n8;
        locals.var_t2_dn9 = assign88070_e134274_d_n9;
        locals.var_t2_dn10 = assign88070_e134274_d_n10;
        locals.var_t2_dn13 = assign88070_e134274_d_n13;

        let assign88080_e134277: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2043 = assign88080_e134277;

        let (assign88090_e134291, assign88090_e134291_d_n0, assign88090_e134291_d_n2, assign88090_e134291_d_n4, assign88090_e134291_d_n5, assign88090_e134291_d_n6, assign88090_e134291_d_n7, assign88090_e134291_d_n8, assign88090_e134291_d_n9, assign88090_e134291_d_n10, assign88090_e134291_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88090_e134291;
        locals.var_t2_dn0 = assign88090_e134291_d_n0;
        locals.var_t2_dn2 = assign88090_e134291_d_n2;
        locals.var_t2_dn4 = assign88090_e134291_d_n4;
        locals.var_t2_dn5 = assign88090_e134291_d_n5;
        locals.var_t2_dn6 = assign88090_e134291_d_n6;
        locals.var_t2_dn7 = assign88090_e134291_d_n7;
        locals.var_t2_dn8 = assign88090_e134291_d_n8;
        locals.var_t2_dn9 = assign88090_e134291_d_n9;
        locals.var_t2_dn10 = assign88090_e134291_d_n10;
        locals.var_t2_dn13 = assign88090_e134291_d_n13;

        let (assign88100_e134305, assign88100_e134305_d_n0, assign88100_e134305_d_n2, assign88100_e134305_d_n4, assign88100_e134305_d_n5, assign88100_e134305_d_n6, assign88100_e134305_d_n7, assign88100_e134305_d_n8, assign88100_e134305_d_n9, assign88100_e134305_d_n10, assign88100_e134305_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) && (locals.var_guard2043 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88100_e134305;
        locals.var_t9_dn0 = assign88100_e134305_d_n0;
        locals.var_t9_dn2 = assign88100_e134305_d_n2;
        locals.var_t9_dn4 = assign88100_e134305_d_n4;
        locals.var_t9_dn5 = assign88100_e134305_d_n5;
        locals.var_t9_dn6 = assign88100_e134305_d_n6;
        locals.var_t9_dn7 = assign88100_e134305_d_n7;
        locals.var_t9_dn8 = assign88100_e134305_d_n8;
        locals.var_t9_dn9 = assign88100_e134305_d_n9;
        locals.var_t9_dn10 = assign88100_e134305_d_n10;
        locals.var_t9_dn13 = assign88100_e134305_d_n13;

        let (assign88110_e134322, assign88110_e134322_d_n0, assign88110_e134322_d_n2, assign88110_e134322_d_n4, assign88110_e134322_d_n5, assign88110_e134322_d_n6, assign88110_e134322_d_n7, assign88110_e134322_d_n8, assign88110_e134322_d_n9, assign88110_e134322_d_n10, assign88110_e134322_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88110_e134317: f64 = (locals.var_kjunc * locals.var_t2);
        let assign88110_e134318: f64 = (assign88110_e134317).sqrt();
        let assign88110_e134320: f64 = (assign88110_e134318 * p.p432);
        (assign88110_e134320, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign88110_e134318)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign88110_e134318)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign88110_e134322;
        locals.var_wjunc0_dn0 = assign88110_e134322_d_n0;
        locals.var_wjunc0_dn2 = assign88110_e134322_d_n2;
        locals.var_wjunc0_dn4 = assign88110_e134322_d_n4;
        locals.var_wjunc0_dn5 = assign88110_e134322_d_n5;
        locals.var_wjunc0_dn6 = assign88110_e134322_d_n6;
        locals.var_wjunc0_dn7 = assign88110_e134322_d_n7;
        locals.var_wjunc0_dn8 = assign88110_e134322_d_n8;
        locals.var_wjunc0_dn9 = assign88110_e134322_d_n9;
        locals.var_wjunc0_dn10 = assign88110_e134322_d_n10;
        locals.var_wjunc0_dn13 = assign88110_e134322_d_n13;

        let (assign88120_e134336, assign88120_e134336_d_n0, assign88120_e134336_d_n2, assign88120_e134336_d_n4, assign88120_e134336_d_n5, assign88120_e134336_d_n6, assign88120_e134336_d_n7, assign88120_e134336_d_n8, assign88120_e134336_d_n9, assign88120_e134336_d_n10, assign88120_e134336_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2042 == 0.0)) {
        let assign88120_e134334: f64 = (p.p334 - locals.var_wjunc0);
        (assign88120_e134334, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88120_e134336;
        locals.var_t2_dn0 = assign88120_e134336_d_n0;
        locals.var_t2_dn2 = assign88120_e134336_d_n2;
        locals.var_t2_dn4 = assign88120_e134336_d_n4;
        locals.var_t2_dn5 = assign88120_e134336_d_n5;
        locals.var_t2_dn6 = assign88120_e134336_d_n6;
        locals.var_t2_dn7 = assign88120_e134336_d_n7;
        locals.var_t2_dn8 = assign88120_e134336_d_n8;
        locals.var_t2_dn9 = assign88120_e134336_d_n9;
        locals.var_t2_dn10 = assign88120_e134336_d_n10;
        locals.var_t2_dn13 = assign88120_e134336_d_n13;

        let (assign88130_e134358, assign88130_e134358_d_n0, assign88130_e134358_d_n2, assign88130_e134358_d_n4, assign88130_e134358_d_n5, assign88130_e134358_d_n6, assign88130_e134358_d_n7, assign88130_e134358_d_n8, assign88130_e134358_d_n9, assign88130_e134358_d_n10, assign88130_e134358_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88130_e134345: f64 = (locals.var_t2 * locals.var_t2);
        let assign88130_e134349: f64 = (p.p334 * 0.01);
        let assign88130_e134350: f64 = (4.0 * assign88130_e134349);
        let assign88130_e134353: f64 = (p.p334 * 0.01);
        let assign88130_e134354: f64 = (assign88130_e134350 * assign88130_e134353);
        let assign88130_e134355: f64 = (assign88130_e134345 + assign88130_e134354);
        let assign88130_e134356: f64 = (assign88130_e134355).sqrt();
        (assign88130_e134356, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign88130_e134356)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign88130_e134356)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign88130_e134358;
        locals.var_tmf2_dn0 = assign88130_e134358_d_n0;
        locals.var_tmf2_dn2 = assign88130_e134358_d_n2;
        locals.var_tmf2_dn4 = assign88130_e134358_d_n4;
        locals.var_tmf2_dn5 = assign88130_e134358_d_n5;
        locals.var_tmf2_dn6 = assign88130_e134358_d_n6;
        locals.var_tmf2_dn7 = assign88130_e134358_d_n7;
        locals.var_tmf2_dn8 = assign88130_e134358_d_n8;
        locals.var_tmf2_dn9 = assign88130_e134358_d_n9;
        locals.var_tmf2_dn10 = assign88130_e134358_d_n10;
        locals.var_tmf2_dn13 = assign88130_e134358_d_n13;

        let (assign88140_e134373, assign88140_e134373_d_n0, assign88140_e134373_d_n2, assign88140_e134373_d_n4, assign88140_e134373_d_n5, assign88140_e134373_d_n6, assign88140_e134373_d_n7, assign88140_e134373_d_n8, assign88140_e134373_d_n9, assign88140_e134373_d_n10, assign88140_e134373_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88140_e134369: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign88140_e134370: f64 = (1.0 + assign88140_e134369);
        let assign88140_e134371: f64 = (0.5 * assign88140_e134370);
        (assign88140_e134371, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88140_e134373;
        locals.var_t9_dn0 = assign88140_e134373_d_n0;
        locals.var_t9_dn2 = assign88140_e134373_d_n2;
        locals.var_t9_dn4 = assign88140_e134373_d_n4;
        locals.var_t9_dn5 = assign88140_e134373_d_n5;
        locals.var_t9_dn6 = assign88140_e134373_d_n6;
        locals.var_t9_dn7 = assign88140_e134373_d_n7;
        locals.var_t9_dn8 = assign88140_e134373_d_n8;
        locals.var_t9_dn9 = assign88140_e134373_d_n9;
        locals.var_t9_dn10 = assign88140_e134373_d_n10;
        locals.var_t9_dn13 = assign88140_e134373_d_n13;

        let (assign88150_e134386, assign88150_e134386_d_n0, assign88150_e134386_d_n2, assign88150_e134386_d_n4, assign88150_e134386_d_n5, assign88150_e134386_d_n6, assign88150_e134386_d_n7, assign88150_e134386_d_n8, assign88150_e134386_d_n9, assign88150_e134386_d_n10, assign88150_e134386_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88150_e134383: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign88150_e134384: f64 = (0.5 * assign88150_e134383);
        (assign88150_e134384, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88150_e134386;
        locals.var_t2_dn0 = assign88150_e134386_d_n0;
        locals.var_t2_dn2 = assign88150_e134386_d_n2;
        locals.var_t2_dn4 = assign88150_e134386_d_n4;
        locals.var_t2_dn5 = assign88150_e134386_d_n5;
        locals.var_t2_dn6 = assign88150_e134386_d_n6;
        locals.var_t2_dn7 = assign88150_e134386_d_n7;
        locals.var_t2_dn8 = assign88150_e134386_d_n8;
        locals.var_t2_dn9 = assign88150_e134386_d_n9;
        locals.var_t2_dn10 = assign88150_e134386_d_n10;
        locals.var_t2_dn13 = assign88150_e134386_d_n13;

        let assign88160_e134389: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2044 = assign88160_e134389;

        let (assign88170_e134400, assign88170_e134400_d_n0, assign88170_e134400_d_n2, assign88170_e134400_d_n4, assign88170_e134400_d_n5, assign88170_e134400_d_n6, assign88170_e134400_d_n7, assign88170_e134400_d_n8, assign88170_e134400_d_n9, assign88170_e134400_d_n10, assign88170_e134400_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2044 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88170_e134400;
        locals.var_t2_dn0 = assign88170_e134400_d_n0;
        locals.var_t2_dn2 = assign88170_e134400_d_n2;
        locals.var_t2_dn4 = assign88170_e134400_d_n4;
        locals.var_t2_dn5 = assign88170_e134400_d_n5;
        locals.var_t2_dn6 = assign88170_e134400_d_n6;
        locals.var_t2_dn7 = assign88170_e134400_d_n7;
        locals.var_t2_dn8 = assign88170_e134400_d_n8;
        locals.var_t2_dn9 = assign88170_e134400_d_n9;
        locals.var_t2_dn10 = assign88170_e134400_d_n10;
        locals.var_t2_dn13 = assign88170_e134400_d_n13;

        let (assign88180_e134411, assign88180_e134411_d_n0, assign88180_e134411_d_n2, assign88180_e134411_d_n4, assign88180_e134411_d_n5, assign88180_e134411_d_n6, assign88180_e134411_d_n7, assign88180_e134411_d_n8, assign88180_e134411_d_n9, assign88180_e134411_d_n10, assign88180_e134411_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2044 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign88180_e134411;
        locals.var_t9_dn0 = assign88180_e134411_d_n0;
        locals.var_t9_dn2 = assign88180_e134411_d_n2;
        locals.var_t9_dn4 = assign88180_e134411_d_n4;
        locals.var_t9_dn5 = assign88180_e134411_d_n5;
        locals.var_t9_dn6 = assign88180_e134411_d_n6;
        locals.var_t9_dn7 = assign88180_e134411_d_n7;
        locals.var_t9_dn8 = assign88180_e134411_d_n8;
        locals.var_t9_dn9 = assign88180_e134411_d_n9;
        locals.var_t9_dn10 = assign88180_e134411_d_n10;
        locals.var_t9_dn13 = assign88180_e134411_d_n13;

        let (assign88190_e134420, assign88190_e134420_d_n0, assign88190_e134420_d_n2, assign88190_e134420_d_n4, assign88190_e134420_d_n5, assign88190_e134420_d_n6, assign88190_e134420_d_n7, assign88190_e134420_d_n8, assign88190_e134420_d_n9, assign88190_e134420_d_n10, assign88190_e134420_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign88190_e134420;
        locals.var_ddriftldc_dn0 = assign88190_e134420_d_n0;
        locals.var_ddriftldc_dn2 = assign88190_e134420_d_n2;
        locals.var_ddriftldc_dn4 = assign88190_e134420_d_n4;
        locals.var_ddriftldc_dn5 = assign88190_e134420_d_n5;
        locals.var_ddriftldc_dn6 = assign88190_e134420_d_n6;
        locals.var_ddriftldc_dn7 = assign88190_e134420_d_n7;
        locals.var_ddriftldc_dn8 = assign88190_e134420_d_n8;
        locals.var_ddriftldc_dn9 = assign88190_e134420_d_n9;
        locals.var_ddriftldc_dn10 = assign88190_e134420_d_n10;
        locals.var_ddriftldc_dn13 = assign88190_e134420_d_n13;

        let (assign88200_e134437, assign88200_e134437_d_n0, assign88200_e134437_d_n2, assign88200_e134437_d_n4, assign88200_e134437_d_n5, assign88200_e134437_d_n6, assign88200_e134437_d_n7, assign88200_e134437_d_n8, assign88200_e134437_d_n9, assign88200_e134437_d_n10, assign88200_e134437_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88200_e134429: f64 = (locals.var_q_nsubld__blk2004 * locals.var_ddriftldc);
        let assign88200_e134431: f64 = (assign88200_e134429 * locals.var_ddriftldc);
        let assign88200_e134433: f64 = (assign88200_e134431 / 2.0);
        let assign88200_e134435: f64 = (assign88200_e134433 / 1.034943e-10);
        (assign88200_e134435, (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign88200_e134429 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign88200_e134437;
        locals.var_dphi_sb_dn0 = assign88200_e134437_d_n0;
        locals.var_dphi_sb_dn2 = assign88200_e134437_d_n2;
        locals.var_dphi_sb_dn4 = assign88200_e134437_d_n4;
        locals.var_dphi_sb_dn5 = assign88200_e134437_d_n5;
        locals.var_dphi_sb_dn6 = assign88200_e134437_d_n6;
        locals.var_dphi_sb_dn7 = assign88200_e134437_d_n7;
        locals.var_dphi_sb_dn8 = assign88200_e134437_d_n8;
        locals.var_dphi_sb_dn9 = assign88200_e134437_d_n9;
        locals.var_dphi_sb_dn10 = assign88200_e134437_d_n10;
        locals.var_dphi_sb_dn13 = assign88200_e134437_d_n13;

        let (assign88210_e134451, assign88210_e134451_d_n0, assign88210_e134451_d_n2, assign88210_e134451_d_n4, assign88210_e134451_d_n5, assign88210_e134451_d_n6, assign88210_e134451_d_n7, assign88210_e134451_d_n8, assign88210_e134451_d_n9, assign88210_e134451_d_n10, assign88210_e134451_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88210_e134446: f64 = (2.0 * locals.var_beta);
        let assign88210_e134448: f64 = (assign88210_e134446 * locals.var_dphi_sb);
        let assign88210_e134449: f64 = (assign88210_e134448).sqrt();
        (assign88210_e134449, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn0)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn2)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn4)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn5)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn6)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn7)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn8)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn9)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn10)) / (2.0 * assign88210_e134449)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign88210_e134446 * locals.var_dphi_sb_dn13)) / (2.0 * assign88210_e134449)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88210_e134451;
        locals.var_t0_dn0 = assign88210_e134451_d_n0;
        locals.var_t0_dn2 = assign88210_e134451_d_n2;
        locals.var_t0_dn4 = assign88210_e134451_d_n4;
        locals.var_t0_dn5 = assign88210_e134451_d_n5;
        locals.var_t0_dn6 = assign88210_e134451_d_n6;
        locals.var_t0_dn7 = assign88210_e134451_d_n7;
        locals.var_t0_dn8 = assign88210_e134451_d_n8;
        locals.var_t0_dn9 = assign88210_e134451_d_n9;
        locals.var_t0_dn10 = assign88210_e134451_d_n10;
        locals.var_t0_dn13 = assign88210_e134451_d_n13;

        let (assign88220_e134467, assign88220_e134467_d_n0, assign88220_e134467_d_n2, assign88220_e134467_d_n4, assign88220_e134467_d_n5, assign88220_e134467_d_n6, assign88220_e134467_d_n7, assign88220_e134467_d_n8, assign88220_e134467_d_n9, assign88220_e134467_d_n10, assign88220_e134467_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88220_e134459: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign88220_e134461: f64 = (-locals.var_t0);
        let assign88220_e134462: f64 = { let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign88220_e134463: f64 = (assign88220_e134459 + assign88220_e134462);
        let assign88220_e134465: f64 = (assign88220_e134463 / 2.0);
        (assign88220_e134465, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign88220_e134461; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88220_e134467;
        locals.var_t1_dn0 = assign88220_e134467_d_n0;
        locals.var_t1_dn2 = assign88220_e134467_d_n2;
        locals.var_t1_dn4 = assign88220_e134467_d_n4;
        locals.var_t1_dn5 = assign88220_e134467_d_n5;
        locals.var_t1_dn6 = assign88220_e134467_d_n6;
        locals.var_t1_dn7 = assign88220_e134467_d_n7;
        locals.var_t1_dn8 = assign88220_e134467_d_n8;
        locals.var_t1_dn9 = assign88220_e134467_d_n9;
        locals.var_t1_dn10 = assign88220_e134467_d_n10;
        locals.var_t1_dn13 = assign88220_e134467_d_n13;

    }

    pub(super) fn stamp_transient_block_310(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign88230_e134479, assign88230_e134479_d_n0, assign88230_e134479_d_n2, assign88230_e134479_d_n4, assign88230_e134479_d_n5, assign88230_e134479_d_n6, assign88230_e134479_d_n7, assign88230_e134479_d_n8, assign88230_e134479_d_n9, assign88230_e134479_d_n10, assign88230_e134479_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88230_e134475: f64 = (locals.var_t1).ln();
        let assign88230_e134477: f64 = (assign88230_e134475 / locals.var_dphi_sb);
        (assign88230_e134477, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign88230_e134475 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign88230_e134479;
        locals.var_c_sb_dn0 = assign88230_e134479_d_n0;
        locals.var_c_sb_dn2 = assign88230_e134479_d_n2;
        locals.var_c_sb_dn4 = assign88230_e134479_d_n4;
        locals.var_c_sb_dn5 = assign88230_e134479_d_n5;
        locals.var_c_sb_dn6 = assign88230_e134479_d_n6;
        locals.var_c_sb_dn7 = assign88230_e134479_d_n7;
        locals.var_c_sb_dn8 = assign88230_e134479_d_n8;
        locals.var_c_sb_dn9 = assign88230_e134479_d_n9;
        locals.var_c_sb_dn10 = assign88230_e134479_d_n10;
        locals.var_c_sb_dn13 = assign88230_e134479_d_n13;

        let (assign88240_e134490, assign88240_e134490_d_n0, assign88240_e134490_d_n2, assign88240_e134490_d_n4, assign88240_e134490_d_n5, assign88240_e134490_d_n6, assign88240_e134490_d_n7, assign88240_e134490_d_n8, assign88240_e134490_d_n9, assign88240_e134490_d_n10, assign88240_e134490_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88240_e134488: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign88240_e134488, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
        locals.var_ps0ld_vxb = assign88240_e134490;
        locals.var_ps0ld_vxb_dn0 = assign88240_e134490_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign88240_e134490_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign88240_e134490_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign88240_e134490_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign88240_e134490_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign88240_e134490_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign88240_e134490_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign88240_e134490_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign88240_e134490_d_n10;
        locals.var_ps0ld_vxb_dn13 = assign88240_e134490_d_n13;

        let (assign88250_e134503, assign88250_e134503_d_n0, assign88250_e134503_d_n2, assign88250_e134503_d_n4, assign88250_e134503_d_n5, assign88250_e134503_d_n6, assign88250_e134503_d_n7, assign88250_e134503_d_n8, assign88250_e134503_d_n9, assign88250_e134503_d_n10, assign88250_e134503_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88250_e134500: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign88250_e134501: f64 = (locals.var_c_sb * assign88250_e134500);
        (assign88250_e134501, ((locals.var_c_sb_dn0 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign88250_e134500) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign88250_e134503;
        locals.var_ty_dn0 = assign88250_e134503_d_n0;
        locals.var_ty_dn2 = assign88250_e134503_d_n2;
        locals.var_ty_dn4 = assign88250_e134503_d_n4;
        locals.var_ty_dn5 = assign88250_e134503_d_n5;
        locals.var_ty_dn6 = assign88250_e134503_d_n6;
        locals.var_ty_dn7 = assign88250_e134503_d_n7;
        locals.var_ty_dn8 = assign88250_e134503_d_n8;
        locals.var_ty_dn9 = assign88250_e134503_d_n9;
        locals.var_ty_dn10 = assign88250_e134503_d_n10;
        locals.var_ty_dn13 = assign88250_e134503_d_n13;

        let assign88260_e134506: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard2045 = assign88260_e134506;

        let (assign88270_e134518, assign88270_e134518_d_n0, assign88270_e134518_d_n2, assign88270_e134518_d_n4, assign88270_e134518_d_n5, assign88270_e134518_d_n6, assign88270_e134518_d_n7, assign88270_e134518_d_n8, assign88270_e134518_d_n9, assign88270_e134518_d_n10, assign88270_e134518_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88270_e134516: f64 = (locals.var_ty).exp();
        (assign88270_e134516, (assign88270_e134516 * locals.var_ty_dn0), (assign88270_e134516 * locals.var_ty_dn2), (assign88270_e134516 * locals.var_ty_dn4), (assign88270_e134516 * locals.var_ty_dn5), (assign88270_e134516 * locals.var_ty_dn6), (assign88270_e134516 * locals.var_ty_dn7), (assign88270_e134516 * locals.var_ty_dn8), (assign88270_e134516 * locals.var_ty_dn9), (assign88270_e134516 * locals.var_ty_dn10), (assign88270_e134516 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88270_e134518;
        locals.var_t1_dn0 = assign88270_e134518_d_n0;
        locals.var_t1_dn2 = assign88270_e134518_d_n2;
        locals.var_t1_dn4 = assign88270_e134518_d_n4;
        locals.var_t1_dn5 = assign88270_e134518_d_n5;
        locals.var_t1_dn6 = assign88270_e134518_d_n6;
        locals.var_t1_dn7 = assign88270_e134518_d_n7;
        locals.var_t1_dn8 = assign88270_e134518_d_n8;
        locals.var_t1_dn9 = assign88270_e134518_d_n9;
        locals.var_t1_dn10 = assign88270_e134518_d_n10;
        locals.var_t1_dn13 = assign88270_e134518_d_n13;

        let (assign88280_e134533, assign88280_e134533_d_n0, assign88280_e134533_d_n2, assign88280_e134533_d_n4, assign88280_e134533_d_n5, assign88280_e134533_d_n6, assign88280_e134533_d_n7, assign88280_e134533_d_n8, assign88280_e134533_d_n9, assign88280_e134533_d_n10, assign88280_e134533_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88280_e134528: f64 = (-locals.var_c_sb);
        let assign88280_e134530: f64 = (assign88280_e134528 * locals.var_dphi_sb);
        let assign88280_e134531: f64 = (assign88280_e134530).exp();
        (assign88280_e134531, (assign88280_e134531 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn0))), (assign88280_e134531 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn2))), (assign88280_e134531 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn4))), (assign88280_e134531 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn5))), (assign88280_e134531 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn6))), (assign88280_e134531 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn7))), (assign88280_e134531 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn8))), (assign88280_e134531 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn9))), (assign88280_e134531 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn10))), (assign88280_e134531 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign88280_e134528 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88280_e134533;
        locals.var_t0_dn0 = assign88280_e134533_d_n0;
        locals.var_t0_dn2 = assign88280_e134533_d_n2;
        locals.var_t0_dn4 = assign88280_e134533_d_n4;
        locals.var_t0_dn5 = assign88280_e134533_d_n5;
        locals.var_t0_dn6 = assign88280_e134533_d_n6;
        locals.var_t0_dn7 = assign88280_e134533_d_n7;
        locals.var_t0_dn8 = assign88280_e134533_d_n8;
        locals.var_t0_dn9 = assign88280_e134533_d_n9;
        locals.var_t0_dn10 = assign88280_e134533_d_n10;
        locals.var_t0_dn13 = assign88280_e134533_d_n13;

        let (assign88290_e134546, assign88290_e134546_d_n0, assign88290_e134546_d_n2, assign88290_e134546_d_n4, assign88290_e134546_d_n5, assign88290_e134546_d_n6, assign88290_e134546_d_n7, assign88290_e134546_d_n8, assign88290_e134546_d_n9, assign88290_e134546_d_n10, assign88290_e134546_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88290_e134544: f64 = (locals.var_t1 - locals.var_t0);
        (assign88290_e134544, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88290_e134546;
        locals.var_t2_dn0 = assign88290_e134546_d_n0;
        locals.var_t2_dn2 = assign88290_e134546_d_n2;
        locals.var_t2_dn4 = assign88290_e134546_d_n4;
        locals.var_t2_dn5 = assign88290_e134546_d_n5;
        locals.var_t2_dn6 = assign88290_e134546_d_n6;
        locals.var_t2_dn7 = assign88290_e134546_d_n7;
        locals.var_t2_dn8 = assign88290_e134546_d_n8;
        locals.var_t2_dn9 = assign88290_e134546_d_n9;
        locals.var_t2_dn10 = assign88290_e134546_d_n10;
        locals.var_t2_dn13 = assign88290_e134546_d_n13;

        let (assign88300_e134562, assign88300_e134562_d_n0, assign88300_e134562_d_n2, assign88300_e134562_d_n4, assign88300_e134562_d_n5, assign88300_e134562_d_n6, assign88300_e134562_d_n7, assign88300_e134562_d_n8, assign88300_e134562_d_n9, assign88300_e134562_d_n10, assign88300_e134562_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 != 0.0)) {
        let assign88300_e134557: f64 = (1.0 + locals.var_t2);
        let assign88300_e134558: f64 = (assign88300_e134557).ln();
        let assign88300_e134560: f64 = (assign88300_e134558 / locals.var_c_sb);
        (assign88300_e134560, ((((locals.var_t2_dn0 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign88300_e134557) * locals.var_c_sb) - (assign88300_e134558 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign88300_e134562;
        locals.var_phi_b_dn0 = assign88300_e134562_d_n0;
        locals.var_phi_b_dn2 = assign88300_e134562_d_n2;
        locals.var_phi_b_dn4 = assign88300_e134562_d_n4;
        locals.var_phi_b_dn5 = assign88300_e134562_d_n5;
        locals.var_phi_b_dn6 = assign88300_e134562_d_n6;
        locals.var_phi_b_dn7 = assign88300_e134562_d_n7;
        locals.var_phi_b_dn8 = assign88300_e134562_d_n8;
        locals.var_phi_b_dn9 = assign88300_e134562_d_n9;
        locals.var_phi_b_dn10 = assign88300_e134562_d_n10;
        locals.var_phi_b_dn13 = assign88300_e134562_d_n13;

        let (assign88310_e134576, assign88310_e134576_d_n0, assign88310_e134576_d_n2, assign88310_e134576_d_n4, assign88310_e134576_d_n5, assign88310_e134576_d_n6, assign88310_e134576_d_n7, assign88310_e134576_d_n8, assign88310_e134576_d_n9, assign88310_e134576_d_n10, assign88310_e134576_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2045 == 0.0)) {
        let assign88310_e134574: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign88310_e134574, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign88310_e134576;
        locals.var_phi_b_dn0 = assign88310_e134576_d_n0;
        locals.var_phi_b_dn2 = assign88310_e134576_d_n2;
        locals.var_phi_b_dn4 = assign88310_e134576_d_n4;
        locals.var_phi_b_dn5 = assign88310_e134576_d_n5;
        locals.var_phi_b_dn6 = assign88310_e134576_d_n6;
        locals.var_phi_b_dn7 = assign88310_e134576_d_n7;
        locals.var_phi_b_dn8 = assign88310_e134576_d_n8;
        locals.var_phi_b_dn9 = assign88310_e134576_d_n9;
        locals.var_phi_b_dn10 = assign88310_e134576_d_n10;
        locals.var_phi_b_dn13 = assign88310_e134576_d_n13;

        let (assign88320_e134587, assign88320_e134587_d_n0, assign88320_e134587_d_n2, assign88320_e134587_d_n4, assign88320_e134587_d_n5, assign88320_e134587_d_n6, assign88320_e134587_d_n7, assign88320_e134587_d_n8, assign88320_e134587_d_n9, assign88320_e134587_d_n10, assign88320_e134587_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) {
        let assign88320_e134585: f64 = (locals.var_beta * locals.var_phi_b);
        (assign88320_e134585, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
        locals.var_chib = assign88320_e134587;
        locals.var_chib_dn0 = assign88320_e134587_d_n0;
        locals.var_chib_dn2 = assign88320_e134587_d_n2;
        locals.var_chib_dn4 = assign88320_e134587_d_n4;
        locals.var_chib_dn5 = assign88320_e134587_d_n5;
        locals.var_chib_dn6 = assign88320_e134587_d_n6;
        locals.var_chib_dn7 = assign88320_e134587_d_n7;
        locals.var_chib_dn8 = assign88320_e134587_d_n8;
        locals.var_chib_dn9 = assign88320_e134587_d_n9;
        locals.var_chib_dn10 = assign88320_e134587_d_n10;
        locals.var_chib_dn13 = assign88320_e134587_d_n13;

        let assign88330_e134591: f64 = (locals.var_chi / 100.0);
        let assign88330_e134596: f64 = if ((locals.var_chib > assign88330_e134591) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2046 = assign88330_e134596;

        let (assign88340_e134609,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2046 != 0.0)) {
        let assign88340_e134607: f64 = (locals.var_flg_fd_mode__blk2010 + 1.0);
        (assign88340_e134607,)
    } else {
        (locals.var_flg_fd_mode__blk2010,)
    }
};
        locals.var_flg_fd_mode__blk2010 = assign88340_e134609;

        let (assign88350_e134620, assign88350_e134620_d_n0, assign88350_e134620_d_n2, assign88350_e134620_d_n4, assign88350_e134620_d_n5, assign88350_e134620_d_n6, assign88350_e134620_d_n7, assign88350_e134620_d_n8, assign88350_e134620_d_n9, assign88350_e134620_d_n10, assign88350_e134620_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2041 != 0.0)) && (locals.var_guard2046 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign88350_e134620;
        locals.var_chi_dn0 = assign88350_e134620_d_n0;
        locals.var_chi_dn2 = assign88350_e134620_d_n2;
        locals.var_chi_dn4 = assign88350_e134620_d_n4;
        locals.var_chi_dn5 = assign88350_e134620_d_n5;
        locals.var_chi_dn6 = assign88350_e134620_d_n6;
        locals.var_chi_dn7 = assign88350_e134620_d_n7;
        locals.var_chi_dn8 = assign88350_e134620_d_n8;
        locals.var_chi_dn9 = assign88350_e134620_d_n9;
        locals.var_chi_dn10 = assign88350_e134620_d_n10;
        locals.var_chi_dn13 = assign88350_e134620_d_n13;

        let (assign88360_e134631, assign88360_e134631_d_n0, assign88360_e134631_d_n2, assign88360_e134631_d_n4, assign88360_e134631_d_n5, assign88360_e134631_d_n6, assign88360_e134631_d_n7, assign88360_e134631_d_n8, assign88360_e134631_d_n9, assign88360_e134631_d_n10, assign88360_e134631_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign88360_e134627: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign88360_e134629: f64 = (assign88360_e134627 - locals.var_vxbgmtcl);
        (assign88360_e134629, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign88360_e134631;
        locals.var_ps0ld_dn0 = assign88360_e134631_d_n0;
        locals.var_ps0ld_dn2 = assign88360_e134631_d_n2;
        locals.var_ps0ld_dn4 = assign88360_e134631_d_n4;
        locals.var_ps0ld_dn5 = assign88360_e134631_d_n5;
        locals.var_ps0ld_dn6 = assign88360_e134631_d_n6;
        locals.var_ps0ld_dn7 = assign88360_e134631_d_n7;
        locals.var_ps0ld_dn8 = assign88360_e134631_d_n8;
        locals.var_ps0ld_dn9 = assign88360_e134631_d_n9;
        locals.var_ps0ld_dn10 = assign88360_e134631_d_n10;
        locals.var_ps0ld_dn13 = assign88360_e134631_d_n13;

        let assign88370_e134633: f64 = (locals.var_chi).abs();
        let assign88370_e134635: f64 = if assign88370_e134633 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard2047 = assign88370_e134635;

        let (assign88380_e134650, assign88380_e134650_d_n0, assign88380_e134650_d_n2, assign88380_e134650_d_n4, assign88380_e134650_d_n5, assign88380_e134650_d_n6, assign88380_e134650_d_n7, assign88380_e134650_d_n8, assign88380_e134650_d_n9, assign88380_e134650_d_n10, assign88380_e134650_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2047 != 0.0)) {
        let assign88380_e134644: f64 = (locals.var_chi - 1.0);
        let assign88380_e134646: f64 = (-locals.var_chi);
        let assign88380_e134647: f64 = (assign88380_e134646).exp();
        let assign88380_e134648: f64 = (assign88380_e134644 + assign88380_e134647);
        (assign88380_e134648, (locals.var_chi_dn0 + (assign88380_e134647 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign88380_e134647 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign88380_e134647 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign88380_e134647 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign88380_e134647 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign88380_e134647 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign88380_e134647 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign88380_e134647 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign88380_e134647 * (-locals.var_chi_dn10))), (locals.var_chi_dn13 + (assign88380_e134647 * (-locals.var_chi_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88380_e134650;
        locals.var_t1_dn0 = assign88380_e134650_d_n0;
        locals.var_t1_dn2 = assign88380_e134650_d_n2;
        locals.var_t1_dn4 = assign88380_e134650_d_n4;
        locals.var_t1_dn5 = assign88380_e134650_d_n5;
        locals.var_t1_dn6 = assign88380_e134650_d_n6;
        locals.var_t1_dn7 = assign88380_e134650_d_n7;
        locals.var_t1_dn8 = assign88380_e134650_d_n8;
        locals.var_t1_dn9 = assign88380_e134650_d_n9;
        locals.var_t1_dn10 = assign88380_e134650_d_n10;
        locals.var_t1_dn13 = assign88380_e134650_d_n13;

        let (assign88390_e134660, assign88390_e134660_d_n0, assign88390_e134660_d_n2, assign88390_e134660_d_n4, assign88390_e134660_d_n5, assign88390_e134660_d_n6, assign88390_e134660_d_n7, assign88390_e134660_d_n8, assign88390_e134660_d_n9, assign88390_e134660_d_n10, assign88390_e134660_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2047 != 0.0)) {
        let assign88390_e134658: f64 = (locals.var_t1).sqrt();
        (assign88390_e134658, (locals.var_t1_dn0 / (2.0 * assign88390_e134658)), (locals.var_t1_dn2 / (2.0 * assign88390_e134658)), (locals.var_t1_dn4 / (2.0 * assign88390_e134658)), (locals.var_t1_dn5 / (2.0 * assign88390_e134658)), (locals.var_t1_dn6 / (2.0 * assign88390_e134658)), (locals.var_t1_dn7 / (2.0 * assign88390_e134658)), (locals.var_t1_dn8 / (2.0 * assign88390_e134658)), (locals.var_t1_dn9 / (2.0 * assign88390_e134658)), (locals.var_t1_dn10 / (2.0 * assign88390_e134658)), (locals.var_t1_dn13 / (2.0 * assign88390_e134658)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88390_e134660;
        locals.var_t2_dn0 = assign88390_e134660_d_n0;
        locals.var_t2_dn2 = assign88390_e134660_d_n2;
        locals.var_t2_dn4 = assign88390_e134660_d_n4;
        locals.var_t2_dn5 = assign88390_e134660_d_n5;
        locals.var_t2_dn6 = assign88390_e134660_d_n6;
        locals.var_t2_dn7 = assign88390_e134660_d_n7;
        locals.var_t2_dn8 = assign88390_e134660_d_n8;
        locals.var_t2_dn9 = assign88390_e134660_d_n9;
        locals.var_t2_dn10 = assign88390_e134660_d_n10;
        locals.var_t2_dn13 = assign88390_e134660_d_n13;

        let (assign88410_e134691, assign88410_e134691_d_n0, assign88410_e134691_d_n2, assign88410_e134691_d_n4, assign88410_e134691_d_n5, assign88410_e134691_d_n6, assign88410_e134691_d_n7, assign88410_e134691_d_n8, assign88410_e134691_d_n9, assign88410_e134691_d_n10, assign88410_e134691_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2047 == 0.0)) {
        let assign88410_e134682: f64 = (0.7071067811865475 * locals.var_chi);
        let assign88410_e134686: f64 = (locals.var_chi * 0.3333333333333333);
        let assign88410_e134687: f64 = (1.0 - assign88410_e134686);
        let assign88410_e134688: f64 = (assign88410_e134687).sqrt();
        let assign88410_e134689: f64 = (assign88410_e134682 * assign88410_e134688);
        (assign88410_e134689, (((0.7071067811865475 * locals.var_chi_dn0) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))), (((0.7071067811865475 * locals.var_chi_dn13) * assign88410_e134688) + (assign88410_e134682 * ((-(locals.var_chi_dn13 * 0.3333333333333333)) / (2.0 * assign88410_e134688)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign88410_e134691;
        locals.var_t2_dn0 = assign88410_e134691_d_n0;
        locals.var_t2_dn2 = assign88410_e134691_d_n2;
        locals.var_t2_dn4 = assign88410_e134691_d_n4;
        locals.var_t2_dn5 = assign88410_e134691_d_n5;
        locals.var_t2_dn6 = assign88410_e134691_d_n6;
        locals.var_t2_dn7 = assign88410_e134691_d_n7;
        locals.var_t2_dn8 = assign88410_e134691_d_n8;
        locals.var_t2_dn9 = assign88410_e134691_d_n9;
        locals.var_t2_dn10 = assign88410_e134691_d_n10;
        locals.var_t2_dn13 = assign88410_e134691_d_n13;

        let (assign88420_e134700, assign88420_e134700_d_n0, assign88420_e134700_d_n2, assign88420_e134700_d_n4, assign88420_e134700_d_n5, assign88420_e134700_d_n6, assign88420_e134700_d_n7, assign88420_e134700_d_n8, assign88420_e134700_d_n9, assign88420_e134700_d_n10, assign88420_e134700_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign88420_e134698: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign88420_e134698, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign88420_e134700;
        locals.var_qbuld_dn0 = assign88420_e134700_d_n0;
        locals.var_qbuld_dn2 = assign88420_e134700_d_n2;
        locals.var_qbuld_dn4 = assign88420_e134700_d_n4;
        locals.var_qbuld_dn5 = assign88420_e134700_d_n5;
        locals.var_qbuld_dn6 = assign88420_e134700_d_n6;
        locals.var_qbuld_dn7 = assign88420_e134700_d_n7;
        locals.var_qbuld_dn8 = assign88420_e134700_d_n8;
        locals.var_qbuld_dn9 = assign88420_e134700_d_n9;
        locals.var_qbuld_dn10 = assign88420_e134700_d_n10;
        locals.var_qbuld_dn13 = assign88420_e134700_d_n13;

        let (assign88430_e134711, assign88430_e134711_d_n0, assign88430_e134711_d_n2, assign88430_e134711_d_n4, assign88430_e134711_d_n5, assign88430_e134711_d_n6, assign88430_e134711_d_n7, assign88430_e134711_d_n8, assign88430_e134711_d_n9, assign88430_e134711_d_n10, assign88430_e134711_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign88430_e134708: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign88430_e134709: f64 = (locals.var_cox0_func * assign88430_e134708);
        (assign88430_e134709, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (-locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn13)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign88430_e134711;
        locals.var_qsuld_dn0 = assign88430_e134711_d_n0;
        locals.var_qsuld_dn2 = assign88430_e134711_d_n2;
        locals.var_qsuld_dn4 = assign88430_e134711_d_n4;
        locals.var_qsuld_dn5 = assign88430_e134711_d_n5;
        locals.var_qsuld_dn6 = assign88430_e134711_d_n6;
        locals.var_qsuld_dn7 = assign88430_e134711_d_n7;
        locals.var_qsuld_dn8 = assign88430_e134711_d_n8;
        locals.var_qsuld_dn9 = assign88430_e134711_d_n9;
        locals.var_qsuld_dn10 = assign88430_e134711_d_n10;
        locals.var_qsuld_dn13 = assign88430_e134711_d_n13;

        let (assign88440_e134720, assign88440_e134720_d_n0, assign88440_e134720_d_n2, assign88440_e134720_d_n4, assign88440_e134720_d_n5, assign88440_e134720_d_n6, assign88440_e134720_d_n7, assign88440_e134720_d_n8, assign88440_e134720_d_n9, assign88440_e134720_d_n10, assign88440_e134720_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        let assign88440_e134718: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk2004);
        (assign88440_e134718, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk2004), (locals.var_qbuld_dn13 / locals.var_q_nsubld__blk2004),)
    } else {
        (locals.var_wdld0__blk2048, locals.var_wdld0__blk2048_dn0, locals.var_wdld0__blk2048_dn2, locals.var_wdld0__blk2048_dn4, locals.var_wdld0__blk2048_dn5, locals.var_wdld0__blk2048_dn6, locals.var_wdld0__blk2048_dn7, locals.var_wdld0__blk2048_dn8, locals.var_wdld0__blk2048_dn9, locals.var_wdld0__blk2048_dn10, locals.var_wdld0__blk2048_dn13,)
    }
};
        locals.var_wdld0__blk2048 = assign88440_e134720;
        locals.var_wdld0__blk2048_dn0 = assign88440_e134720_d_n0;
        locals.var_wdld0__blk2048_dn2 = assign88440_e134720_d_n2;
        locals.var_wdld0__blk2048_dn4 = assign88440_e134720_d_n4;
        locals.var_wdld0__blk2048_dn5 = assign88440_e134720_d_n5;
        locals.var_wdld0__blk2048_dn6 = assign88440_e134720_d_n6;
        locals.var_wdld0__blk2048_dn7 = assign88440_e134720_d_n7;
        locals.var_wdld0__blk2048_dn8 = assign88440_e134720_d_n8;
        locals.var_wdld0__blk2048_dn9 = assign88440_e134720_d_n9;
        locals.var_wdld0__blk2048_dn10 = assign88440_e134720_d_n10;
        locals.var_wdld0__blk2048_dn13 = assign88440_e134720_d_n13;

        let assign88450_e134723: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2050 = assign88450_e134723;

        let assign88460_e134728: f64 = (locals.var_ddriftldc * 0.1);
        let assign88460_e134729: f64 = (locals.var_ddriftldc - assign88460_e134728);
        let assign88460_e134733: f64 = (locals.var_ddriftldc * 0.1);
        let assign88460_e134736: f64 = if ((locals.var_wdld0__blk2048 > assign88460_e134729) && (assign88460_e134733 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2051 = assign88460_e134736;

        let (assign88470_e134753, assign88470_e134753_d_n0, assign88470_e134753_d_n2, assign88470_e134753_d_n4, assign88470_e134753_d_n5, assign88470_e134753_d_n6, assign88470_e134753_d_n7, assign88470_e134753_d_n8, assign88470_e134753_d_n9, assign88470_e134753_d_n10, assign88470_e134753_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88470_e134747: f64 = (locals.var_wdld0__blk2048 - locals.var_ddriftldc);
        let assign88470_e134750: f64 = (locals.var_ddriftldc * 0.1);
        let assign88470_e134751: f64 = (assign88470_e134747 + assign88470_e134750);
        (assign88470_e134751, ((locals.var_wdld0__blk2048_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk2048_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk2048_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk2048_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk2048_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk2048_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk2048_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk2048_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk2048_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk2048_dn13 - locals.var_ddriftldc_dn13) + (locals.var_ddriftldc_dn13 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign88470_e134753;
        locals.var_tmf1_dn0 = assign88470_e134753_d_n0;
        locals.var_tmf1_dn2 = assign88470_e134753_d_n2;
        locals.var_tmf1_dn4 = assign88470_e134753_d_n4;
        locals.var_tmf1_dn5 = assign88470_e134753_d_n5;
        locals.var_tmf1_dn6 = assign88470_e134753_d_n6;
        locals.var_tmf1_dn7 = assign88470_e134753_d_n7;
        locals.var_tmf1_dn8 = assign88470_e134753_d_n8;
        locals.var_tmf1_dn9 = assign88470_e134753_d_n9;
        locals.var_tmf1_dn10 = assign88470_e134753_d_n10;
        locals.var_tmf1_dn13 = assign88470_e134753_d_n13;

        let (assign88480_e134766, assign88480_e134766_d_n0, assign88480_e134766_d_n2, assign88480_e134766_d_n4, assign88480_e134766_d_n5, assign88480_e134766_d_n6, assign88480_e134766_d_n7, assign88480_e134766_d_n8, assign88480_e134766_d_n9, assign88480_e134766_d_n10, assign88480_e134766_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88480_e134764: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign88480_e134764, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign88480_e134766;
        locals.var_x2_dn0 = assign88480_e134766_d_n0;
        locals.var_x2_dn2 = assign88480_e134766_d_n2;
        locals.var_x2_dn4 = assign88480_e134766_d_n4;
        locals.var_x2_dn5 = assign88480_e134766_d_n5;
        locals.var_x2_dn6 = assign88480_e134766_d_n6;
        locals.var_x2_dn7 = assign88480_e134766_d_n7;
        locals.var_x2_dn8 = assign88480_e134766_d_n8;
        locals.var_x2_dn9 = assign88480_e134766_d_n9;
        locals.var_x2_dn10 = assign88480_e134766_d_n10;
        locals.var_x2_dn13 = assign88480_e134766_d_n13;

        let (assign88490_e134783, assign88490_e134783_d_n0, assign88490_e134783_d_n2, assign88490_e134783_d_n4, assign88490_e134783_d_n5, assign88490_e134783_d_n6, assign88490_e134783_d_n7, assign88490_e134783_d_n8, assign88490_e134783_d_n9, assign88490_e134783_d_n10, assign88490_e134783_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88490_e134777: f64 = (locals.var_ddriftldc * 0.1);
        let assign88490_e134780: f64 = (locals.var_ddriftldc * 0.1);
        let assign88490_e134781: f64 = (assign88490_e134777 * assign88490_e134780);
        (assign88490_e134781, (((locals.var_ddriftldc_dn0 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn13 * 0.1) * assign88490_e134780) + (assign88490_e134777 * (locals.var_ddriftldc_dn13 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign88490_e134783;
        locals.var_xmax2_dn0 = assign88490_e134783_d_n0;
        locals.var_xmax2_dn2 = assign88490_e134783_d_n2;
        locals.var_xmax2_dn4 = assign88490_e134783_d_n4;
        locals.var_xmax2_dn5 = assign88490_e134783_d_n5;
        locals.var_xmax2_dn6 = assign88490_e134783_d_n6;
        locals.var_xmax2_dn7 = assign88490_e134783_d_n7;
        locals.var_xmax2_dn8 = assign88490_e134783_d_n8;
        locals.var_xmax2_dn9 = assign88490_e134783_d_n9;
        locals.var_xmax2_dn10 = assign88490_e134783_d_n10;
        locals.var_xmax2_dn13 = assign88490_e134783_d_n13;

        let (assign88500_e134794, assign88500_e134794_d_n0, assign88500_e134794_d_n2, assign88500_e134794_d_n4, assign88500_e134794_d_n5, assign88500_e134794_d_n6, assign88500_e134794_d_n7, assign88500_e134794_d_n8, assign88500_e134794_d_n9, assign88500_e134794_d_n10, assign88500_e134794_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign88500_e134794;
        locals.var_xp_dn0 = assign88500_e134794_d_n0;
        locals.var_xp_dn2 = assign88500_e134794_d_n2;
        locals.var_xp_dn4 = assign88500_e134794_d_n4;
        locals.var_xp_dn5 = assign88500_e134794_d_n5;
        locals.var_xp_dn6 = assign88500_e134794_d_n6;
        locals.var_xp_dn7 = assign88500_e134794_d_n7;
        locals.var_xp_dn8 = assign88500_e134794_d_n8;
        locals.var_xp_dn9 = assign88500_e134794_d_n9;
        locals.var_xp_dn10 = assign88500_e134794_d_n10;
        locals.var_xp_dn13 = assign88500_e134794_d_n13;

        let (assign88510_e134805, assign88510_e134805_d_n0, assign88510_e134805_d_n2, assign88510_e134805_d_n4, assign88510_e134805_d_n5, assign88510_e134805_d_n6, assign88510_e134805_d_n7, assign88510_e134805_d_n8, assign88510_e134805_d_n9, assign88510_e134805_d_n10, assign88510_e134805_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign88510_e134805;
        locals.var_xmp_dn0 = assign88510_e134805_d_n0;
        locals.var_xmp_dn2 = assign88510_e134805_d_n2;
        locals.var_xmp_dn4 = assign88510_e134805_d_n4;
        locals.var_xmp_dn5 = assign88510_e134805_d_n5;
        locals.var_xmp_dn6 = assign88510_e134805_d_n6;
        locals.var_xmp_dn7 = assign88510_e134805_d_n7;
        locals.var_xmp_dn8 = assign88510_e134805_d_n8;
        locals.var_xmp_dn9 = assign88510_e134805_d_n9;
        locals.var_xmp_dn10 = assign88510_e134805_d_n10;
        locals.var_xmp_dn13 = assign88510_e134805_d_n13;

        let (assign88520_e134816,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88520_e134816;

        let (assign88530_e134827,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88530_e134827;

    }

    pub(super) fn stamp_transient_block_311(
        locals: &mut StampLocals,
    ) {
        let (assign88540_e134838, assign88540_e134838_d_n0, assign88540_e134838_d_n2, assign88540_e134838_d_n4, assign88540_e134838_d_n5, assign88540_e134838_d_n6, assign88540_e134838_d_n7, assign88540_e134838_d_n8, assign88540_e134838_d_n9, assign88540_e134838_d_n10, assign88540_e134838_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign88540_e134838;
        locals.var_arg_dn0 = assign88540_e134838_d_n0;
        locals.var_arg_dn2 = assign88540_e134838_d_n2;
        locals.var_arg_dn4 = assign88540_e134838_d_n4;
        locals.var_arg_dn5 = assign88540_e134838_d_n5;
        locals.var_arg_dn6 = assign88540_e134838_d_n6;
        locals.var_arg_dn7 = assign88540_e134838_d_n7;
        locals.var_arg_dn8 = assign88540_e134838_d_n8;
        locals.var_arg_dn9 = assign88540_e134838_d_n9;
        locals.var_arg_dn10 = assign88540_e134838_d_n10;
        locals.var_arg_dn13 = assign88540_e134838_d_n13;

        let (assign88550_e134849, assign88550_e134849_d_n0, assign88550_e134849_d_n2, assign88550_e134849_d_n4, assign88550_e134849_d_n5, assign88550_e134849_d_n6, assign88550_e134849_d_n7, assign88550_e134849_d_n8, assign88550_e134849_d_n9, assign88550_e134849_d_n10, assign88550_e134849_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign88550_e134849;
        locals.var_dnm_dn0 = assign88550_e134849_d_n0;
        locals.var_dnm_dn2 = assign88550_e134849_d_n2;
        locals.var_dnm_dn4 = assign88550_e134849_d_n4;
        locals.var_dnm_dn5 = assign88550_e134849_d_n5;
        locals.var_dnm_dn6 = assign88550_e134849_d_n6;
        locals.var_dnm_dn7 = assign88550_e134849_d_n7;
        locals.var_dnm_dn8 = assign88550_e134849_d_n8;
        locals.var_dnm_dn9 = assign88550_e134849_d_n9;
        locals.var_dnm_dn10 = assign88550_e134849_d_n10;
        locals.var_dnm_dn13 = assign88550_e134849_d_n13;

        let (assign88560_e134862, assign88560_e134862_d_n0, assign88560_e134862_d_n2, assign88560_e134862_d_n4, assign88560_e134862_d_n5, assign88560_e134862_d_n6, assign88560_e134862_d_n7, assign88560_e134862_d_n8, assign88560_e134862_d_n9, assign88560_e134862_d_n10, assign88560_e134862_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88560_e134860: f64 = (locals.var_xp * locals.var_x2);
        (assign88560_e134860, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign88560_e134862;
        locals.var_xp_dn0 = assign88560_e134862_d_n0;
        locals.var_xp_dn2 = assign88560_e134862_d_n2;
        locals.var_xp_dn4 = assign88560_e134862_d_n4;
        locals.var_xp_dn5 = assign88560_e134862_d_n5;
        locals.var_xp_dn6 = assign88560_e134862_d_n6;
        locals.var_xp_dn7 = assign88560_e134862_d_n7;
        locals.var_xp_dn8 = assign88560_e134862_d_n8;
        locals.var_xp_dn9 = assign88560_e134862_d_n9;
        locals.var_xp_dn10 = assign88560_e134862_d_n10;
        locals.var_xp_dn13 = assign88560_e134862_d_n13;

        let (assign88570_e134875, assign88570_e134875_d_n0, assign88570_e134875_d_n2, assign88570_e134875_d_n4, assign88570_e134875_d_n5, assign88570_e134875_d_n6, assign88570_e134875_d_n7, assign88570_e134875_d_n8, assign88570_e134875_d_n9, assign88570_e134875_d_n10, assign88570_e134875_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88570_e134873: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign88570_e134873, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign88570_e134875;
        locals.var_xmp_dn0 = assign88570_e134875_d_n0;
        locals.var_xmp_dn2 = assign88570_e134875_d_n2;
        locals.var_xmp_dn4 = assign88570_e134875_d_n4;
        locals.var_xmp_dn5 = assign88570_e134875_d_n5;
        locals.var_xmp_dn6 = assign88570_e134875_d_n6;
        locals.var_xmp_dn7 = assign88570_e134875_d_n7;
        locals.var_xmp_dn8 = assign88570_e134875_d_n8;
        locals.var_xmp_dn9 = assign88570_e134875_d_n9;
        locals.var_xmp_dn10 = assign88570_e134875_d_n10;
        locals.var_xmp_dn13 = assign88570_e134875_d_n13;

        let (assign88580_e134888, assign88580_e134888_d_n0, assign88580_e134888_d_n2, assign88580_e134888_d_n4, assign88580_e134888_d_n5, assign88580_e134888_d_n6, assign88580_e134888_d_n7, assign88580_e134888_d_n8, assign88580_e134888_d_n9, assign88580_e134888_d_n10, assign88580_e134888_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88580_e134886: f64 = (locals.var_xp * locals.var_x2);
        (assign88580_e134886, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign88580_e134888;
        locals.var_xp_dn0 = assign88580_e134888_d_n0;
        locals.var_xp_dn2 = assign88580_e134888_d_n2;
        locals.var_xp_dn4 = assign88580_e134888_d_n4;
        locals.var_xp_dn5 = assign88580_e134888_d_n5;
        locals.var_xp_dn6 = assign88580_e134888_d_n6;
        locals.var_xp_dn7 = assign88580_e134888_d_n7;
        locals.var_xp_dn8 = assign88580_e134888_d_n8;
        locals.var_xp_dn9 = assign88580_e134888_d_n9;
        locals.var_xp_dn10 = assign88580_e134888_d_n10;
        locals.var_xp_dn13 = assign88580_e134888_d_n13;

        let (assign88590_e134901, assign88590_e134901_d_n0, assign88590_e134901_d_n2, assign88590_e134901_d_n4, assign88590_e134901_d_n5, assign88590_e134901_d_n6, assign88590_e134901_d_n7, assign88590_e134901_d_n8, assign88590_e134901_d_n9, assign88590_e134901_d_n10, assign88590_e134901_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88590_e134899: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign88590_e134899, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign88590_e134901;
        locals.var_xmp_dn0 = assign88590_e134901_d_n0;
        locals.var_xmp_dn2 = assign88590_e134901_d_n2;
        locals.var_xmp_dn4 = assign88590_e134901_d_n4;
        locals.var_xmp_dn5 = assign88590_e134901_d_n5;
        locals.var_xmp_dn6 = assign88590_e134901_d_n6;
        locals.var_xmp_dn7 = assign88590_e134901_d_n7;
        locals.var_xmp_dn8 = assign88590_e134901_d_n8;
        locals.var_xmp_dn9 = assign88590_e134901_d_n9;
        locals.var_xmp_dn10 = assign88590_e134901_d_n10;
        locals.var_xmp_dn13 = assign88590_e134901_d_n13;

        let (assign88600_e134914, assign88600_e134914_d_n0, assign88600_e134914_d_n2, assign88600_e134914_d_n4, assign88600_e134914_d_n5, assign88600_e134914_d_n6, assign88600_e134914_d_n7, assign88600_e134914_d_n8, assign88600_e134914_d_n9, assign88600_e134914_d_n10, assign88600_e134914_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88600_e134912: f64 = (locals.var_xp + locals.var_xmp);
        (assign88600_e134912, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign88600_e134914;
        locals.var_arg_dn0 = assign88600_e134914_d_n0;
        locals.var_arg_dn2 = assign88600_e134914_d_n2;
        locals.var_arg_dn4 = assign88600_e134914_d_n4;
        locals.var_arg_dn5 = assign88600_e134914_d_n5;
        locals.var_arg_dn6 = assign88600_e134914_d_n6;
        locals.var_arg_dn7 = assign88600_e134914_d_n7;
        locals.var_arg_dn8 = assign88600_e134914_d_n8;
        locals.var_arg_dn9 = assign88600_e134914_d_n9;
        locals.var_arg_dn10 = assign88600_e134914_d_n10;
        locals.var_arg_dn13 = assign88600_e134914_d_n13;

        let (assign88610_e134925, assign88610_e134925_d_n0, assign88610_e134925_d_n2, assign88610_e134925_d_n4, assign88610_e134925_d_n5, assign88610_e134925_d_n6, assign88610_e134925_d_n7, assign88610_e134925_d_n8, assign88610_e134925_d_n9, assign88610_e134925_d_n10, assign88610_e134925_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign88610_e134925;
        locals.var_dnm_dn0 = assign88610_e134925_d_n0;
        locals.var_dnm_dn2 = assign88610_e134925_d_n2;
        locals.var_dnm_dn4 = assign88610_e134925_d_n4;
        locals.var_dnm_dn5 = assign88610_e134925_d_n5;
        locals.var_dnm_dn6 = assign88610_e134925_d_n6;
        locals.var_dnm_dn7 = assign88610_e134925_d_n7;
        locals.var_dnm_dn8 = assign88610_e134925_d_n8;
        locals.var_dnm_dn9 = assign88610_e134925_d_n9;
        locals.var_dnm_dn10 = assign88610_e134925_d_n10;
        locals.var_dnm_dn13 = assign88610_e134925_d_n13;

        let assign88620_e134940: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2052 = assign88620_e134940;

        let assign88630_e134943: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2053 = assign88630_e134943;

        let (assign88640_e134958,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88640_e134958;

        let assign88650_e134961: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2054 = assign88650_e134961;

        let (assign88660_e134979,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 == 0.0)) && (locals.var_guard2054 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88660_e134979;

        let assign88670_e134982: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2055 = assign88670_e134982;

        let (assign88680_e135003,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 == 0.0)) && (locals.var_guard2054 == 0.0)) && (locals.var_guard2055 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88680_e135003;

        let assign88690_e135006: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2056 = assign88690_e135006;

        let (assign88700_e135030,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_guard2053 == 0.0)) && (locals.var_guard2054 == 0.0)) && (locals.var_guard2055 == 0.0)) && (locals.var_guard2056 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88700_e135030;

        let (assign88710_e135043,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88710_e135043;

        let mut assign88720_loop_guard: usize = 0;
        while {
            let assign88720_cond_e135057: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign88720_cond_e135057 != 0.0
        } {
            assign88720_loop_guard += 1;
            assert!(assign88720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign88720_body0_e135071, assign88720_body0_e135071_d_n0, assign88720_body0_e135071_d_n2, assign88720_body0_e135071_d_n4, assign88720_body0_e135071_d_n5, assign88720_body0_e135071_d_n6, assign88720_body0_e135071_d_n7, assign88720_body0_e135071_d_n8, assign88720_body0_e135071_d_n9, assign88720_body0_e135071_d_n10, assign88720_body0_e135071_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) {
        let assign88720_body0_e135069: f64 = (locals.var_dnm).sqrt();
        (assign88720_body0_e135069, (locals.var_dnm_dn0 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn2 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn4 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn5 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn6 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn7 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn8 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn9 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn10 / (2.0 * assign88720_body0_e135069)), (locals.var_dnm_dn13 / (2.0 * assign88720_body0_e135069)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign88720_body0_e135071;
            locals.var_dnm_dn0 = assign88720_body0_e135071_d_n0;
            locals.var_dnm_dn2 = assign88720_body0_e135071_d_n2;
            locals.var_dnm_dn4 = assign88720_body0_e135071_d_n4;
            locals.var_dnm_dn5 = assign88720_body0_e135071_d_n5;
            locals.var_dnm_dn6 = assign88720_body0_e135071_d_n6;
            locals.var_dnm_dn7 = assign88720_body0_e135071_d_n7;
            locals.var_dnm_dn8 = assign88720_body0_e135071_d_n8;
            locals.var_dnm_dn9 = assign88720_body0_e135071_d_n9;
            locals.var_dnm_dn10 = assign88720_body0_e135071_d_n10;
            locals.var_dnm_dn13 = assign88720_body0_e135071_d_n13;
            let (assign88720_body1_e135086,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 != 0.0)) {
        let assign88720_body1_e135084: f64 = (locals.var_m0 + 1.0);
        (assign88720_body1_e135084,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign88720_body1_e135086;
        }

        let (assign88730_e135111, assign88730_e135111_d_n0, assign88730_e135111_d_n2, assign88730_e135111_d_n4, assign88730_e135111_d_n5, assign88730_e135111_d_n6, assign88730_e135111_d_n7, assign88730_e135111_d_n8, assign88730_e135111_d_n9, assign88730_e135111_d_n10, assign88730_e135111_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) && (locals.var_guard2052 == 0.0)) {
        let (assign88730_e135109, assign88730_e135109_d_n0, assign88730_e135109_d_n2, assign88730_e135109_d_n4, assign88730_e135109_d_n5, assign88730_e135109_d_n6, assign88730_e135109_d_n7, assign88730_e135109_d_n8, assign88730_e135109_d_n9, assign88730_e135109_d_n10, assign88730_e135109_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign88730_e135106: f64 = (2.0 * 2.0);
                let assign88730_e135107: f64 = (1.0 / assign88730_e135106);
                let assign88730_e135108: f64 = (locals.var_dnm).powf(assign88730_e135107);
                (assign88730_e135108, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn0)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn2)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn4)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn5)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn6)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn7)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn8)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn9)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn10)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign88730_e135107) as f64).is_finite() && ((assign88730_e135107) as f64).fract() == 0.0 { if assign88730_e135107 == 0.0 { 0.0 } else { (assign88730_e135107 * ((locals.var_dnm).powf(assign88730_e135107 - 1.0) * locals.var_dnm_dn13)) } } else { (assign88730_e135108 * (assign88730_e135107 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign88730_e135109, assign88730_e135109_d_n0, assign88730_e135109_d_n2, assign88730_e135109_d_n4, assign88730_e135109_d_n5, assign88730_e135109_d_n6, assign88730_e135109_d_n7, assign88730_e135109_d_n8, assign88730_e135109_d_n9, assign88730_e135109_d_n10, assign88730_e135109_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign88730_e135111;
        locals.var_dnm_dn0 = assign88730_e135111_d_n0;
        locals.var_dnm_dn2 = assign88730_e135111_d_n2;
        locals.var_dnm_dn4 = assign88730_e135111_d_n4;
        locals.var_dnm_dn5 = assign88730_e135111_d_n5;
        locals.var_dnm_dn6 = assign88730_e135111_d_n6;
        locals.var_dnm_dn7 = assign88730_e135111_d_n7;
        locals.var_dnm_dn8 = assign88730_e135111_d_n8;
        locals.var_dnm_dn9 = assign88730_e135111_d_n9;
        locals.var_dnm_dn10 = assign88730_e135111_d_n10;
        locals.var_dnm_dn13 = assign88730_e135111_d_n13;

        let (assign88740_e135124, assign88740_e135124_d_n0, assign88740_e135124_d_n2, assign88740_e135124_d_n4, assign88740_e135124_d_n5, assign88740_e135124_d_n6, assign88740_e135124_d_n7, assign88740_e135124_d_n8, assign88740_e135124_d_n9, assign88740_e135124_d_n10, assign88740_e135124_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88740_e135122: f64 = (1.0 / locals.var_dnm);
        (assign88740_e135122, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign88740_e135124;
        locals.var_dnm_dn0 = assign88740_e135124_d_n0;
        locals.var_dnm_dn2 = assign88740_e135124_d_n2;
        locals.var_dnm_dn4 = assign88740_e135124_d_n4;
        locals.var_dnm_dn5 = assign88740_e135124_d_n5;
        locals.var_dnm_dn6 = assign88740_e135124_d_n6;
        locals.var_dnm_dn7 = assign88740_e135124_d_n7;
        locals.var_dnm_dn8 = assign88740_e135124_d_n8;
        locals.var_dnm_dn9 = assign88740_e135124_d_n9;
        locals.var_dnm_dn10 = assign88740_e135124_d_n10;
        locals.var_dnm_dn13 = assign88740_e135124_d_n13;

        let (assign88750_e135141, assign88750_e135141_d_n0, assign88750_e135141_d_n2, assign88750_e135141_d_n4, assign88750_e135141_d_n5, assign88750_e135141_d_n6, assign88750_e135141_d_n7, assign88750_e135141_d_n8, assign88750_e135141_d_n9, assign88750_e135141_d_n10, assign88750_e135141_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88750_e135136: f64 = (locals.var_ddriftldc * 0.1);
        let assign88750_e135137: f64 = (locals.var_tmf1 * assign88750_e135136);
        let assign88750_e135139: f64 = (assign88750_e135137 * locals.var_dnm);
        (assign88750_e135139, ((((locals.var_tmf1_dn0 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign88750_e135136) + (locals.var_tmf1 * (locals.var_ddriftldc_dn13 * 0.1))) * locals.var_dnm) + (assign88750_e135137 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign88750_e135141;
        locals.var_tmf0_dn0 = assign88750_e135141_d_n0;
        locals.var_tmf0_dn2 = assign88750_e135141_d_n2;
        locals.var_tmf0_dn4 = assign88750_e135141_d_n4;
        locals.var_tmf0_dn5 = assign88750_e135141_d_n5;
        locals.var_tmf0_dn6 = assign88750_e135141_d_n6;
        locals.var_tmf0_dn7 = assign88750_e135141_d_n7;
        locals.var_tmf0_dn8 = assign88750_e135141_d_n8;
        locals.var_tmf0_dn9 = assign88750_e135141_d_n9;
        locals.var_tmf0_dn10 = assign88750_e135141_d_n10;
        locals.var_tmf0_dn13 = assign88750_e135141_d_n13;

        let (assign88760_e135160, assign88760_e135160_d_n0, assign88760_e135160_d_n2, assign88760_e135160_d_n4, assign88760_e135160_d_n5, assign88760_e135160_d_n6, assign88760_e135160_d_n7, assign88760_e135160_d_n8, assign88760_e135160_d_n9, assign88760_e135160_d_n10, assign88760_e135160_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88760_e135152: f64 = (locals.var_ddriftldc * 0.1);
        let assign88760_e135154: f64 = (assign88760_e135152 * locals.var_xmp);
        let assign88760_e135156: f64 = (assign88760_e135154 * locals.var_dnm);
        let assign88760_e135158: f64 = (assign88760_e135156 / locals.var_arg);
        (assign88760_e135158, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn0)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn2)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn4)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn5)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn6)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn7)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn8)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn9)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn10)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn13 * 0.1) * locals.var_xmp) + (assign88760_e135152 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign88760_e135154 * locals.var_dnm_dn13)) * locals.var_arg) - (assign88760_e135156 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88760_e135160;
        locals.var_t0_dn0 = assign88760_e135160_d_n0;
        locals.var_t0_dn2 = assign88760_e135160_d_n2;
        locals.var_t0_dn4 = assign88760_e135160_d_n4;
        locals.var_t0_dn5 = assign88760_e135160_d_n5;
        locals.var_t0_dn6 = assign88760_e135160_d_n6;
        locals.var_t0_dn7 = assign88760_e135160_d_n7;
        locals.var_t0_dn8 = assign88760_e135160_d_n8;
        locals.var_t0_dn9 = assign88760_e135160_d_n9;
        locals.var_t0_dn10 = assign88760_e135160_d_n10;
        locals.var_t0_dn13 = assign88760_e135160_d_n13;

        let (assign88770_e135177, assign88770_e135177_d_n0, assign88770_e135177_d_n2, assign88770_e135177_d_n4, assign88770_e135177_d_n5, assign88770_e135177_d_n6, assign88770_e135177_d_n7, assign88770_e135177_d_n8, assign88770_e135177_d_n9, assign88770_e135177_d_n10, assign88770_e135177_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        let assign88770_e135172: f64 = (locals.var_ddriftldc * 0.1);
        let assign88770_e135173: f64 = (locals.var_ddriftldc - assign88770_e135172);
        let assign88770_e135175: f64 = (assign88770_e135173 + locals.var_tmf0);
        (assign88770_e135175, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn13 - (locals.var_ddriftldc_dn13 * 0.1)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88770_e135177;
        locals.var_t1_dn0 = assign88770_e135177_d_n0;
        locals.var_t1_dn2 = assign88770_e135177_d_n2;
        locals.var_t1_dn4 = assign88770_e135177_d_n4;
        locals.var_t1_dn5 = assign88770_e135177_d_n5;
        locals.var_t1_dn6 = assign88770_e135177_d_n6;
        locals.var_t1_dn7 = assign88770_e135177_d_n7;
        locals.var_t1_dn8 = assign88770_e135177_d_n8;
        locals.var_t1_dn9 = assign88770_e135177_d_n9;
        locals.var_t1_dn10 = assign88770_e135177_d_n10;
        locals.var_t1_dn13 = assign88770_e135177_d_n13;

        let (assign88780_e135188, assign88780_e135188_d_n0, assign88780_e135188_d_n2, assign88780_e135188_d_n4, assign88780_e135188_d_n5, assign88780_e135188_d_n6, assign88780_e135188_d_n7, assign88780_e135188_d_n8, assign88780_e135188_d_n9, assign88780_e135188_d_n10, assign88780_e135188_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88780_e135188;
        locals.var_t0_dn0 = assign88780_e135188_d_n0;
        locals.var_t0_dn2 = assign88780_e135188_d_n2;
        locals.var_t0_dn4 = assign88780_e135188_d_n4;
        locals.var_t0_dn5 = assign88780_e135188_d_n5;
        locals.var_t0_dn6 = assign88780_e135188_d_n6;
        locals.var_t0_dn7 = assign88780_e135188_d_n7;
        locals.var_t0_dn8 = assign88780_e135188_d_n8;
        locals.var_t0_dn9 = assign88780_e135188_d_n9;
        locals.var_t0_dn10 = assign88780_e135188_d_n10;
        locals.var_t0_dn13 = assign88780_e135188_d_n13;

        let (assign88790_e135200, assign88790_e135200_d_n0, assign88790_e135200_d_n2, assign88790_e135200_d_n4, assign88790_e135200_d_n5, assign88790_e135200_d_n6, assign88790_e135200_d_n7, assign88790_e135200_d_n8, assign88790_e135200_d_n9, assign88790_e135200_d_n10, assign88790_e135200_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 == 0.0)) {
        (locals.var_wdld0__blk2048, locals.var_wdld0__blk2048_dn0, locals.var_wdld0__blk2048_dn2, locals.var_wdld0__blk2048_dn4, locals.var_wdld0__blk2048_dn5, locals.var_wdld0__blk2048_dn6, locals.var_wdld0__blk2048_dn7, locals.var_wdld0__blk2048_dn8, locals.var_wdld0__blk2048_dn9, locals.var_wdld0__blk2048_dn10, locals.var_wdld0__blk2048_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88790_e135200;
        locals.var_t1_dn0 = assign88790_e135200_d_n0;
        locals.var_t1_dn2 = assign88790_e135200_d_n2;
        locals.var_t1_dn4 = assign88790_e135200_d_n4;
        locals.var_t1_dn5 = assign88790_e135200_d_n5;
        locals.var_t1_dn6 = assign88790_e135200_d_n6;
        locals.var_t1_dn7 = assign88790_e135200_d_n7;
        locals.var_t1_dn8 = assign88790_e135200_d_n8;
        locals.var_t1_dn9 = assign88790_e135200_d_n9;
        locals.var_t1_dn10 = assign88790_e135200_d_n10;
        locals.var_t1_dn13 = assign88790_e135200_d_n13;

        let (assign88800_e135212, assign88800_e135212_d_n0, assign88800_e135212_d_n2, assign88800_e135212_d_n4, assign88800_e135212_d_n5, assign88800_e135212_d_n6, assign88800_e135212_d_n7, assign88800_e135212_d_n8, assign88800_e135212_d_n9, assign88800_e135212_d_n10, assign88800_e135212_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2051 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign88800_e135212;
        locals.var_t0_dn0 = assign88800_e135212_d_n0;
        locals.var_t0_dn2 = assign88800_e135212_d_n2;
        locals.var_t0_dn4 = assign88800_e135212_d_n4;
        locals.var_t0_dn5 = assign88800_e135212_d_n5;
        locals.var_t0_dn6 = assign88800_e135212_d_n6;
        locals.var_t0_dn7 = assign88800_e135212_d_n7;
        locals.var_t0_dn8 = assign88800_e135212_d_n8;
        locals.var_t0_dn9 = assign88800_e135212_d_n9;
        locals.var_t0_dn10 = assign88800_e135212_d_n10;
        locals.var_t0_dn13 = assign88800_e135212_d_n13;

        let assign88810_e135215: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2057 = assign88810_e135215;

        let (assign88820_e135228,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 != 0.0)) && (locals.var_guard2057 != 0.0)) {
        let assign88820_e135226: f64 = (locals.var_flg_fd_mode__blk2010 + 2.0);
        (assign88820_e135226,)
    } else {
        (locals.var_flg_fd_mode__blk2010,)
    }
};
        locals.var_flg_fd_mode__blk2010 = assign88820_e135228;

        let (assign88830_e135243, assign88830_e135243_d_n0, assign88830_e135243_d_n2, assign88830_e135243_d_n4, assign88830_e135243_d_n5, assign88830_e135243_d_n6, assign88830_e135243_d_n7, assign88830_e135243_d_n8, assign88830_e135243_d_n9, assign88830_e135243_d_n10, assign88830_e135243_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 == 0.0)) {
        let (assign88830_e135241, assign88830_e135241_d_n0, assign88830_e135241_d_n2, assign88830_e135241_d_n4, assign88830_e135241_d_n5, assign88830_e135241_d_n6, assign88830_e135241_d_n7, assign88830_e135241_d_n8, assign88830_e135241_d_n9, assign88830_e135241_d_n10, assign88830_e135241_d_n13,) = {
            if (locals.var_wdld0__blk2048 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk2048, locals.var_wdld0__blk2048_dn0, locals.var_wdld0__blk2048_dn2, locals.var_wdld0__blk2048_dn4, locals.var_wdld0__blk2048_dn5, locals.var_wdld0__blk2048_dn6, locals.var_wdld0__blk2048_dn7, locals.var_wdld0__blk2048_dn8, locals.var_wdld0__blk2048_dn9, locals.var_wdld0__blk2048_dn10, locals.var_wdld0__blk2048_dn13,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
            }
        };
        (assign88830_e135241, assign88830_e135241_d_n0, assign88830_e135241_d_n2, assign88830_e135241_d_n4, assign88830_e135241_d_n5, assign88830_e135241_d_n6, assign88830_e135241_d_n7, assign88830_e135241_d_n8, assign88830_e135241_d_n9, assign88830_e135241_d_n10, assign88830_e135241_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign88830_e135243;
        locals.var_t1_dn0 = assign88830_e135243_d_n0;
        locals.var_t1_dn2 = assign88830_e135243_d_n2;
        locals.var_t1_dn4 = assign88830_e135243_d_n4;
        locals.var_t1_dn5 = assign88830_e135243_d_n5;
        locals.var_t1_dn6 = assign88830_e135243_d_n6;
        locals.var_t1_dn7 = assign88830_e135243_d_n7;
        locals.var_t1_dn8 = assign88830_e135243_d_n8;
        locals.var_t1_dn9 = assign88830_e135243_d_n9;
        locals.var_t1_dn10 = assign88830_e135243_d_n10;
        locals.var_t1_dn13 = assign88830_e135243_d_n13;

        let assign88840_e135246: f64 = if locals.var_wdld0__blk2048 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard2058 = assign88840_e135246;

        let (assign88850_e135260,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2050 == 0.0)) && (locals.var_guard2058 != 0.0)) {
        let assign88850_e135258: f64 = (locals.var_flg_fd_mode__blk2010 + 2.0);
        (assign88850_e135258,)
    } else {
        (locals.var_flg_fd_mode__blk2010,)
    }
};
        locals.var_flg_fd_mode__blk2010 = assign88850_e135260;

        let assign88860_e135263: f64 = if locals.var_flg_fd_mode__blk2010 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2059 = assign88860_e135263;

        let (assign88870_e135272, assign88870_e135272_d_n0, assign88870_e135272_d_n2, assign88870_e135272_d_n4, assign88870_e135272_d_n5, assign88870_e135272_d_n6, assign88870_e135272_d_n7, assign88870_e135272_d_n8, assign88870_e135272_d_n9, assign88870_e135272_d_n10, assign88870_e135272_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_bef1__blk2049, locals.var_ps0ld_bef1__blk2049_dn0, locals.var_ps0ld_bef1__blk2049_dn2, locals.var_ps0ld_bef1__blk2049_dn4, locals.var_ps0ld_bef1__blk2049_dn5, locals.var_ps0ld_bef1__blk2049_dn6, locals.var_ps0ld_bef1__blk2049_dn7, locals.var_ps0ld_bef1__blk2049_dn8, locals.var_ps0ld_bef1__blk2049_dn9, locals.var_ps0ld_bef1__blk2049_dn10, locals.var_ps0ld_bef1__blk2049_dn13,)
    }
};
        locals.var_ps0ld_bef1__blk2049 = assign88870_e135272;
        locals.var_ps0ld_bef1__blk2049_dn0 = assign88870_e135272_d_n0;
        locals.var_ps0ld_bef1__blk2049_dn2 = assign88870_e135272_d_n2;
        locals.var_ps0ld_bef1__blk2049_dn4 = assign88870_e135272_d_n4;
        locals.var_ps0ld_bef1__blk2049_dn5 = assign88870_e135272_d_n5;
        locals.var_ps0ld_bef1__blk2049_dn6 = assign88870_e135272_d_n6;
        locals.var_ps0ld_bef1__blk2049_dn7 = assign88870_e135272_d_n7;
        locals.var_ps0ld_bef1__blk2049_dn8 = assign88870_e135272_d_n8;
        locals.var_ps0ld_bef1__blk2049_dn9 = assign88870_e135272_d_n9;
        locals.var_ps0ld_bef1__blk2049_dn10 = assign88870_e135272_d_n10;
        locals.var_ps0ld_bef1__blk2049_dn13 = assign88870_e135272_d_n13;

    }

    pub(super) fn stamp_transient_block_312(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign88880_e135283, assign88880_e135283_d_n0, assign88880_e135283_d_n2, assign88880_e135283_d_n4, assign88880_e135283_d_n5, assign88880_e135283_d_n6, assign88880_e135283_d_n7, assign88880_e135283_d_n8, assign88880_e135283_d_n9, assign88880_e135283_d_n10, assign88880_e135283_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) {
        let assign88880_e135281: f64 = (locals.var_t1 * locals.var_q_nsubld__blk2004);
        (assign88880_e135281, (locals.var_t1_dn0 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn2 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn4 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn5 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn6 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn7 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn8 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn9 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn10 * locals.var_q_nsubld__blk2004), (locals.var_t1_dn13 * locals.var_q_nsubld__blk2004),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign88880_e135283;
        locals.var_qbuld_dn0 = assign88880_e135283_d_n0;
        locals.var_qbuld_dn2 = assign88880_e135283_d_n2;
        locals.var_qbuld_dn4 = assign88880_e135283_d_n4;
        locals.var_qbuld_dn5 = assign88880_e135283_d_n5;
        locals.var_qbuld_dn6 = assign88880_e135283_d_n6;
        locals.var_qbuld_dn7 = assign88880_e135283_d_n7;
        locals.var_qbuld_dn8 = assign88880_e135283_d_n8;
        locals.var_qbuld_dn9 = assign88880_e135283_d_n9;
        locals.var_qbuld_dn10 = assign88880_e135283_d_n10;
        locals.var_qbuld_dn13 = assign88880_e135283_d_n13;

        let (assign88890_e135296, assign88890_e135296_d_n0, assign88890_e135296_d_n2, assign88890_e135296_d_n4, assign88890_e135296_d_n5, assign88890_e135296_d_n6, assign88890_e135296_d_n7, assign88890_e135296_d_n8, assign88890_e135296_d_n9, assign88890_e135296_d_n10, assign88890_e135296_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) {
        let assign88890_e135293: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign88890_e135294: f64 = (locals.var_vgpld - assign88890_e135293);
        (assign88890_e135294, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (locals.var_vgpld_dn6 - (locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (-(locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn13 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign88890_e135296;
        locals.var_ps0ld_dn0 = assign88890_e135296_d_n0;
        locals.var_ps0ld_dn2 = assign88890_e135296_d_n2;
        locals.var_ps0ld_dn4 = assign88890_e135296_d_n4;
        locals.var_ps0ld_dn5 = assign88890_e135296_d_n5;
        locals.var_ps0ld_dn6 = assign88890_e135296_d_n6;
        locals.var_ps0ld_dn7 = assign88890_e135296_d_n7;
        locals.var_ps0ld_dn8 = assign88890_e135296_d_n8;
        locals.var_ps0ld_dn9 = assign88890_e135296_d_n9;
        locals.var_ps0ld_dn10 = assign88890_e135296_d_n10;
        locals.var_ps0ld_dn13 = assign88890_e135296_d_n13;

        let assign88900_e135299: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2060 = assign88900_e135299;

        let assign88910_e135303: f64 = (locals.var_ps0ld_bef1__blk2049 - 0.1);
        let assign88910_e135308: f64 = if ((locals.var_ps0ld > assign88910_e135303) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2061 = assign88910_e135308;

        let (assign88920_e135325, assign88920_e135325_d_n0, assign88920_e135325_d_n2, assign88920_e135325_d_n4, assign88920_e135325_d_n5, assign88920_e135325_d_n6, assign88920_e135325_d_n7, assign88920_e135325_d_n8, assign88920_e135325_d_n9, assign88920_e135325_d_n10, assign88920_e135325_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign88920_e135321: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk2049);
        let assign88920_e135323: f64 = (assign88920_e135321 + 0.1);
        (assign88920_e135323, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk2049_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk2049_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk2049_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk2049_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk2049_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk2049_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk2049_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk2049_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk2049_dn10), (locals.var_ps0ld_dn13 - locals.var_ps0ld_bef1__blk2049_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign88920_e135325;
        locals.var_tmf1_dn0 = assign88920_e135325_d_n0;
        locals.var_tmf1_dn2 = assign88920_e135325_d_n2;
        locals.var_tmf1_dn4 = assign88920_e135325_d_n4;
        locals.var_tmf1_dn5 = assign88920_e135325_d_n5;
        locals.var_tmf1_dn6 = assign88920_e135325_d_n6;
        locals.var_tmf1_dn7 = assign88920_e135325_d_n7;
        locals.var_tmf1_dn8 = assign88920_e135325_d_n8;
        locals.var_tmf1_dn9 = assign88920_e135325_d_n9;
        locals.var_tmf1_dn10 = assign88920_e135325_d_n10;
        locals.var_tmf1_dn13 = assign88920_e135325_d_n13;

        let (assign88930_e135340, assign88930_e135340_d_n0, assign88930_e135340_d_n2, assign88930_e135340_d_n4, assign88930_e135340_d_n5, assign88930_e135340_d_n6, assign88930_e135340_d_n7, assign88930_e135340_d_n8, assign88930_e135340_d_n9, assign88930_e135340_d_n10, assign88930_e135340_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign88930_e135338: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign88930_e135338, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign88930_e135340;
        locals.var_x2_dn0 = assign88930_e135340_d_n0;
        locals.var_x2_dn2 = assign88930_e135340_d_n2;
        locals.var_x2_dn4 = assign88930_e135340_d_n4;
        locals.var_x2_dn5 = assign88930_e135340_d_n5;
        locals.var_x2_dn6 = assign88930_e135340_d_n6;
        locals.var_x2_dn7 = assign88930_e135340_d_n7;
        locals.var_x2_dn8 = assign88930_e135340_d_n8;
        locals.var_x2_dn9 = assign88930_e135340_d_n9;
        locals.var_x2_dn10 = assign88930_e135340_d_n10;
        locals.var_x2_dn13 = assign88930_e135340_d_n13;

        let (assign88940_e135355, assign88940_e135355_d_n0, assign88940_e135355_d_n2, assign88940_e135355_d_n4, assign88940_e135355_d_n5, assign88940_e135355_d_n6, assign88940_e135355_d_n7, assign88940_e135355_d_n8, assign88940_e135355_d_n9, assign88940_e135355_d_n10, assign88940_e135355_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign88940_e135353: f64 = (0.1 * 0.1);
        (assign88940_e135353, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign88940_e135355;
        locals.var_xmax2_dn0 = assign88940_e135355_d_n0;
        locals.var_xmax2_dn2 = assign88940_e135355_d_n2;
        locals.var_xmax2_dn4 = assign88940_e135355_d_n4;
        locals.var_xmax2_dn5 = assign88940_e135355_d_n5;
        locals.var_xmax2_dn6 = assign88940_e135355_d_n6;
        locals.var_xmax2_dn7 = assign88940_e135355_d_n7;
        locals.var_xmax2_dn8 = assign88940_e135355_d_n8;
        locals.var_xmax2_dn9 = assign88940_e135355_d_n9;
        locals.var_xmax2_dn10 = assign88940_e135355_d_n10;
        locals.var_xmax2_dn13 = assign88940_e135355_d_n13;

        let (assign88950_e135368, assign88950_e135368_d_n0, assign88950_e135368_d_n2, assign88950_e135368_d_n4, assign88950_e135368_d_n5, assign88950_e135368_d_n6, assign88950_e135368_d_n7, assign88950_e135368_d_n8, assign88950_e135368_d_n9, assign88950_e135368_d_n10, assign88950_e135368_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign88950_e135368;
        locals.var_xp_dn0 = assign88950_e135368_d_n0;
        locals.var_xp_dn2 = assign88950_e135368_d_n2;
        locals.var_xp_dn4 = assign88950_e135368_d_n4;
        locals.var_xp_dn5 = assign88950_e135368_d_n5;
        locals.var_xp_dn6 = assign88950_e135368_d_n6;
        locals.var_xp_dn7 = assign88950_e135368_d_n7;
        locals.var_xp_dn8 = assign88950_e135368_d_n8;
        locals.var_xp_dn9 = assign88950_e135368_d_n9;
        locals.var_xp_dn10 = assign88950_e135368_d_n10;
        locals.var_xp_dn13 = assign88950_e135368_d_n13;

        let (assign88960_e135381, assign88960_e135381_d_n0, assign88960_e135381_d_n2, assign88960_e135381_d_n4, assign88960_e135381_d_n5, assign88960_e135381_d_n6, assign88960_e135381_d_n7, assign88960_e135381_d_n8, assign88960_e135381_d_n9, assign88960_e135381_d_n10, assign88960_e135381_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign88960_e135381;
        locals.var_xmp_dn0 = assign88960_e135381_d_n0;
        locals.var_xmp_dn2 = assign88960_e135381_d_n2;
        locals.var_xmp_dn4 = assign88960_e135381_d_n4;
        locals.var_xmp_dn5 = assign88960_e135381_d_n5;
        locals.var_xmp_dn6 = assign88960_e135381_d_n6;
        locals.var_xmp_dn7 = assign88960_e135381_d_n7;
        locals.var_xmp_dn8 = assign88960_e135381_d_n8;
        locals.var_xmp_dn9 = assign88960_e135381_d_n9;
        locals.var_xmp_dn10 = assign88960_e135381_d_n10;
        locals.var_xmp_dn13 = assign88960_e135381_d_n13;

        let (assign88970_e135394,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign88970_e135394;

        let (assign88980_e135407,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign88980_e135407;

        let (assign88990_e135420, assign88990_e135420_d_n0, assign88990_e135420_d_n2, assign88990_e135420_d_n4, assign88990_e135420_d_n5, assign88990_e135420_d_n6, assign88990_e135420_d_n7, assign88990_e135420_d_n8, assign88990_e135420_d_n9, assign88990_e135420_d_n10, assign88990_e135420_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign88990_e135420;
        locals.var_arg_dn0 = assign88990_e135420_d_n0;
        locals.var_arg_dn2 = assign88990_e135420_d_n2;
        locals.var_arg_dn4 = assign88990_e135420_d_n4;
        locals.var_arg_dn5 = assign88990_e135420_d_n5;
        locals.var_arg_dn6 = assign88990_e135420_d_n6;
        locals.var_arg_dn7 = assign88990_e135420_d_n7;
        locals.var_arg_dn8 = assign88990_e135420_d_n8;
        locals.var_arg_dn9 = assign88990_e135420_d_n9;
        locals.var_arg_dn10 = assign88990_e135420_d_n10;
        locals.var_arg_dn13 = assign88990_e135420_d_n13;

        let (assign89000_e135433, assign89000_e135433_d_n0, assign89000_e135433_d_n2, assign89000_e135433_d_n4, assign89000_e135433_d_n5, assign89000_e135433_d_n6, assign89000_e135433_d_n7, assign89000_e135433_d_n8, assign89000_e135433_d_n9, assign89000_e135433_d_n10, assign89000_e135433_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign89000_e135433;
        locals.var_dnm_dn0 = assign89000_e135433_d_n0;
        locals.var_dnm_dn2 = assign89000_e135433_d_n2;
        locals.var_dnm_dn4 = assign89000_e135433_d_n4;
        locals.var_dnm_dn5 = assign89000_e135433_d_n5;
        locals.var_dnm_dn6 = assign89000_e135433_d_n6;
        locals.var_dnm_dn7 = assign89000_e135433_d_n7;
        locals.var_dnm_dn8 = assign89000_e135433_d_n8;
        locals.var_dnm_dn9 = assign89000_e135433_d_n9;
        locals.var_dnm_dn10 = assign89000_e135433_d_n10;
        locals.var_dnm_dn13 = assign89000_e135433_d_n13;

        let (assign89010_e135448, assign89010_e135448_d_n0, assign89010_e135448_d_n2, assign89010_e135448_d_n4, assign89010_e135448_d_n5, assign89010_e135448_d_n6, assign89010_e135448_d_n7, assign89010_e135448_d_n8, assign89010_e135448_d_n9, assign89010_e135448_d_n10, assign89010_e135448_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89010_e135446: f64 = (locals.var_xp * locals.var_x2);
        (assign89010_e135446, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign89010_e135448;
        locals.var_xp_dn0 = assign89010_e135448_d_n0;
        locals.var_xp_dn2 = assign89010_e135448_d_n2;
        locals.var_xp_dn4 = assign89010_e135448_d_n4;
        locals.var_xp_dn5 = assign89010_e135448_d_n5;
        locals.var_xp_dn6 = assign89010_e135448_d_n6;
        locals.var_xp_dn7 = assign89010_e135448_d_n7;
        locals.var_xp_dn8 = assign89010_e135448_d_n8;
        locals.var_xp_dn9 = assign89010_e135448_d_n9;
        locals.var_xp_dn10 = assign89010_e135448_d_n10;
        locals.var_xp_dn13 = assign89010_e135448_d_n13;

        let (assign89020_e135463, assign89020_e135463_d_n0, assign89020_e135463_d_n2, assign89020_e135463_d_n4, assign89020_e135463_d_n5, assign89020_e135463_d_n6, assign89020_e135463_d_n7, assign89020_e135463_d_n8, assign89020_e135463_d_n9, assign89020_e135463_d_n10, assign89020_e135463_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89020_e135461: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign89020_e135461, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign89020_e135463;
        locals.var_xmp_dn0 = assign89020_e135463_d_n0;
        locals.var_xmp_dn2 = assign89020_e135463_d_n2;
        locals.var_xmp_dn4 = assign89020_e135463_d_n4;
        locals.var_xmp_dn5 = assign89020_e135463_d_n5;
        locals.var_xmp_dn6 = assign89020_e135463_d_n6;
        locals.var_xmp_dn7 = assign89020_e135463_d_n7;
        locals.var_xmp_dn8 = assign89020_e135463_d_n8;
        locals.var_xmp_dn9 = assign89020_e135463_d_n9;
        locals.var_xmp_dn10 = assign89020_e135463_d_n10;
        locals.var_xmp_dn13 = assign89020_e135463_d_n13;

        let (assign89030_e135478, assign89030_e135478_d_n0, assign89030_e135478_d_n2, assign89030_e135478_d_n4, assign89030_e135478_d_n5, assign89030_e135478_d_n6, assign89030_e135478_d_n7, assign89030_e135478_d_n8, assign89030_e135478_d_n9, assign89030_e135478_d_n10, assign89030_e135478_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89030_e135476: f64 = (locals.var_xp * locals.var_x2);
        (assign89030_e135476, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign89030_e135478;
        locals.var_xp_dn0 = assign89030_e135478_d_n0;
        locals.var_xp_dn2 = assign89030_e135478_d_n2;
        locals.var_xp_dn4 = assign89030_e135478_d_n4;
        locals.var_xp_dn5 = assign89030_e135478_d_n5;
        locals.var_xp_dn6 = assign89030_e135478_d_n6;
        locals.var_xp_dn7 = assign89030_e135478_d_n7;
        locals.var_xp_dn8 = assign89030_e135478_d_n8;
        locals.var_xp_dn9 = assign89030_e135478_d_n9;
        locals.var_xp_dn10 = assign89030_e135478_d_n10;
        locals.var_xp_dn13 = assign89030_e135478_d_n13;

        let (assign89040_e135493, assign89040_e135493_d_n0, assign89040_e135493_d_n2, assign89040_e135493_d_n4, assign89040_e135493_d_n5, assign89040_e135493_d_n6, assign89040_e135493_d_n7, assign89040_e135493_d_n8, assign89040_e135493_d_n9, assign89040_e135493_d_n10, assign89040_e135493_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89040_e135491: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign89040_e135491, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign89040_e135493;
        locals.var_xmp_dn0 = assign89040_e135493_d_n0;
        locals.var_xmp_dn2 = assign89040_e135493_d_n2;
        locals.var_xmp_dn4 = assign89040_e135493_d_n4;
        locals.var_xmp_dn5 = assign89040_e135493_d_n5;
        locals.var_xmp_dn6 = assign89040_e135493_d_n6;
        locals.var_xmp_dn7 = assign89040_e135493_d_n7;
        locals.var_xmp_dn8 = assign89040_e135493_d_n8;
        locals.var_xmp_dn9 = assign89040_e135493_d_n9;
        locals.var_xmp_dn10 = assign89040_e135493_d_n10;
        locals.var_xmp_dn13 = assign89040_e135493_d_n13;

        let (assign89050_e135508, assign89050_e135508_d_n0, assign89050_e135508_d_n2, assign89050_e135508_d_n4, assign89050_e135508_d_n5, assign89050_e135508_d_n6, assign89050_e135508_d_n7, assign89050_e135508_d_n8, assign89050_e135508_d_n9, assign89050_e135508_d_n10, assign89050_e135508_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89050_e135506: f64 = (locals.var_xp + locals.var_xmp);
        (assign89050_e135506, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign89050_e135508;
        locals.var_arg_dn0 = assign89050_e135508_d_n0;
        locals.var_arg_dn2 = assign89050_e135508_d_n2;
        locals.var_arg_dn4 = assign89050_e135508_d_n4;
        locals.var_arg_dn5 = assign89050_e135508_d_n5;
        locals.var_arg_dn6 = assign89050_e135508_d_n6;
        locals.var_arg_dn7 = assign89050_e135508_d_n7;
        locals.var_arg_dn8 = assign89050_e135508_d_n8;
        locals.var_arg_dn9 = assign89050_e135508_d_n9;
        locals.var_arg_dn10 = assign89050_e135508_d_n10;
        locals.var_arg_dn13 = assign89050_e135508_d_n13;

        let (assign89060_e135521, assign89060_e135521_d_n0, assign89060_e135521_d_n2, assign89060_e135521_d_n4, assign89060_e135521_d_n5, assign89060_e135521_d_n6, assign89060_e135521_d_n7, assign89060_e135521_d_n8, assign89060_e135521_d_n9, assign89060_e135521_d_n10, assign89060_e135521_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign89060_e135521;
        locals.var_dnm_dn0 = assign89060_e135521_d_n0;
        locals.var_dnm_dn2 = assign89060_e135521_d_n2;
        locals.var_dnm_dn4 = assign89060_e135521_d_n4;
        locals.var_dnm_dn5 = assign89060_e135521_d_n5;
        locals.var_dnm_dn6 = assign89060_e135521_d_n6;
        locals.var_dnm_dn7 = assign89060_e135521_d_n7;
        locals.var_dnm_dn8 = assign89060_e135521_d_n8;
        locals.var_dnm_dn9 = assign89060_e135521_d_n9;
        locals.var_dnm_dn10 = assign89060_e135521_d_n10;
        locals.var_dnm_dn13 = assign89060_e135521_d_n13;

        let assign89070_e135536: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2062 = assign89070_e135536;

        let assign89080_e135539: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2063 = assign89080_e135539;

        let (assign89090_e135556,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89090_e135556;

        let assign89100_e135559: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2064 = assign89100_e135559;

        let (assign89110_e135579,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 == 0.0)) && (locals.var_guard2064 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89110_e135579;

        let assign89120_e135582: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2065 = assign89120_e135582;

        let (assign89130_e135605,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 == 0.0)) && (locals.var_guard2064 == 0.0)) && (locals.var_guard2065 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89130_e135605;

        let assign89140_e135608: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2066 = assign89140_e135608;

        let (assign89150_e135634,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_guard2063 == 0.0)) && (locals.var_guard2064 == 0.0)) && (locals.var_guard2065 == 0.0)) && (locals.var_guard2066 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign89150_e135634;

        let (assign89160_e135649,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign89160_e135649;

        let mut assign89170_loop_guard: usize = 0;
        while {
            let assign89170_cond_e135665: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign89170_cond_e135665 != 0.0
        } {
            assign89170_loop_guard += 1;
            assert!(assign89170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89170_body0_e135681, assign89170_body0_e135681_d_n0, assign89170_body0_e135681_d_n2, assign89170_body0_e135681_d_n4, assign89170_body0_e135681_d_n5, assign89170_body0_e135681_d_n6, assign89170_body0_e135681_d_n7, assign89170_body0_e135681_d_n8, assign89170_body0_e135681_d_n9, assign89170_body0_e135681_d_n10, assign89170_body0_e135681_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) {
        let assign89170_body0_e135679: f64 = (locals.var_dnm).sqrt();
        (assign89170_body0_e135679, (locals.var_dnm_dn0 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn2 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn4 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn5 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn6 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn7 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn8 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn9 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn10 / (2.0 * assign89170_body0_e135679)), (locals.var_dnm_dn13 / (2.0 * assign89170_body0_e135679)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign89170_body0_e135681;
            locals.var_dnm_dn0 = assign89170_body0_e135681_d_n0;
            locals.var_dnm_dn2 = assign89170_body0_e135681_d_n2;
            locals.var_dnm_dn4 = assign89170_body0_e135681_d_n4;
            locals.var_dnm_dn5 = assign89170_body0_e135681_d_n5;
            locals.var_dnm_dn6 = assign89170_body0_e135681_d_n6;
            locals.var_dnm_dn7 = assign89170_body0_e135681_d_n7;
            locals.var_dnm_dn8 = assign89170_body0_e135681_d_n8;
            locals.var_dnm_dn9 = assign89170_body0_e135681_d_n9;
            locals.var_dnm_dn10 = assign89170_body0_e135681_d_n10;
            locals.var_dnm_dn13 = assign89170_body0_e135681_d_n13;
            let (assign89170_body1_e135698,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 != 0.0)) {
        let assign89170_body1_e135696: f64 = (locals.var_m0 + 1.0);
        (assign89170_body1_e135696,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign89170_body1_e135698;
        }

        let (assign89180_e135725, assign89180_e135725_d_n0, assign89180_e135725_d_n2, assign89180_e135725_d_n4, assign89180_e135725_d_n5, assign89180_e135725_d_n6, assign89180_e135725_d_n7, assign89180_e135725_d_n8, assign89180_e135725_d_n9, assign89180_e135725_d_n10, assign89180_e135725_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) && (locals.var_guard2062 == 0.0)) {
        let (assign89180_e135723, assign89180_e135723_d_n0, assign89180_e135723_d_n2, assign89180_e135723_d_n4, assign89180_e135723_d_n5, assign89180_e135723_d_n6, assign89180_e135723_d_n7, assign89180_e135723_d_n8, assign89180_e135723_d_n9, assign89180_e135723_d_n10, assign89180_e135723_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89180_e135720: f64 = (2.0 * 2.0);
                let assign89180_e135721: f64 = (1.0 / assign89180_e135720);
                let assign89180_e135722: f64 = (locals.var_dnm).powf(assign89180_e135721);
                (assign89180_e135722, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn0)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn2)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn4)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn5)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn6)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn7)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn8)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn9)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn10)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign89180_e135721) as f64).is_finite() && ((assign89180_e135721) as f64).fract() == 0.0 { if assign89180_e135721 == 0.0 { 0.0 } else { (assign89180_e135721 * ((locals.var_dnm).powf(assign89180_e135721 - 1.0) * locals.var_dnm_dn13)) } } else { (assign89180_e135722 * (assign89180_e135721 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign89180_e135723, assign89180_e135723_d_n0, assign89180_e135723_d_n2, assign89180_e135723_d_n4, assign89180_e135723_d_n5, assign89180_e135723_d_n6, assign89180_e135723_d_n7, assign89180_e135723_d_n8, assign89180_e135723_d_n9, assign89180_e135723_d_n10, assign89180_e135723_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign89180_e135725;
        locals.var_dnm_dn0 = assign89180_e135725_d_n0;
        locals.var_dnm_dn2 = assign89180_e135725_d_n2;
        locals.var_dnm_dn4 = assign89180_e135725_d_n4;
        locals.var_dnm_dn5 = assign89180_e135725_d_n5;
        locals.var_dnm_dn6 = assign89180_e135725_d_n6;
        locals.var_dnm_dn7 = assign89180_e135725_d_n7;
        locals.var_dnm_dn8 = assign89180_e135725_d_n8;
        locals.var_dnm_dn9 = assign89180_e135725_d_n9;
        locals.var_dnm_dn10 = assign89180_e135725_d_n10;
        locals.var_dnm_dn13 = assign89180_e135725_d_n13;

        let (assign89190_e135740, assign89190_e135740_d_n0, assign89190_e135740_d_n2, assign89190_e135740_d_n4, assign89190_e135740_d_n5, assign89190_e135740_d_n6, assign89190_e135740_d_n7, assign89190_e135740_d_n8, assign89190_e135740_d_n9, assign89190_e135740_d_n10, assign89190_e135740_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89190_e135738: f64 = (1.0 / locals.var_dnm);
        (assign89190_e135738, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign89190_e135740;
        locals.var_dnm_dn0 = assign89190_e135740_d_n0;
        locals.var_dnm_dn2 = assign89190_e135740_d_n2;
        locals.var_dnm_dn4 = assign89190_e135740_d_n4;
        locals.var_dnm_dn5 = assign89190_e135740_d_n5;
        locals.var_dnm_dn6 = assign89190_e135740_d_n6;
        locals.var_dnm_dn7 = assign89190_e135740_d_n7;
        locals.var_dnm_dn8 = assign89190_e135740_d_n8;
        locals.var_dnm_dn9 = assign89190_e135740_d_n9;
        locals.var_dnm_dn10 = assign89190_e135740_d_n10;
        locals.var_dnm_dn13 = assign89190_e135740_d_n13;

        let (assign89200_e135757, assign89200_e135757_d_n0, assign89200_e135757_d_n2, assign89200_e135757_d_n4, assign89200_e135757_d_n5, assign89200_e135757_d_n6, assign89200_e135757_d_n7, assign89200_e135757_d_n8, assign89200_e135757_d_n9, assign89200_e135757_d_n10, assign89200_e135757_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89200_e135753: f64 = (locals.var_tmf1 * 0.1);
        let assign89200_e135755: f64 = (assign89200_e135753 * locals.var_dnm);
        (assign89200_e135755, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign89200_e135753 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign89200_e135757;
        locals.var_tmf0_dn0 = assign89200_e135757_d_n0;
        locals.var_tmf0_dn2 = assign89200_e135757_d_n2;
        locals.var_tmf0_dn4 = assign89200_e135757_d_n4;
        locals.var_tmf0_dn5 = assign89200_e135757_d_n5;
        locals.var_tmf0_dn6 = assign89200_e135757_d_n6;
        locals.var_tmf0_dn7 = assign89200_e135757_d_n7;
        locals.var_tmf0_dn8 = assign89200_e135757_d_n8;
        locals.var_tmf0_dn9 = assign89200_e135757_d_n9;
        locals.var_tmf0_dn10 = assign89200_e135757_d_n10;
        locals.var_tmf0_dn13 = assign89200_e135757_d_n13;

        let (assign89210_e135776, assign89210_e135776_d_n0, assign89210_e135776_d_n2, assign89210_e135776_d_n4, assign89210_e135776_d_n5, assign89210_e135776_d_n6, assign89210_e135776_d_n7, assign89210_e135776_d_n8, assign89210_e135776_d_n9, assign89210_e135776_d_n10, assign89210_e135776_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89210_e135770: f64 = (0.1 * locals.var_xmp);
        let assign89210_e135772: f64 = (assign89210_e135770 * locals.var_dnm);
        let assign89210_e135774: f64 = (assign89210_e135772 / locals.var_arg);
        (assign89210_e135774, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn0)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn2)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn4)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn5)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn6)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn7)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn8)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn9)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn10)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign89210_e135770 * locals.var_dnm_dn13)) * locals.var_arg) - (assign89210_e135772 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89210_e135776;
        locals.var_t0_dn0 = assign89210_e135776_d_n0;
        locals.var_t0_dn2 = assign89210_e135776_d_n2;
        locals.var_t0_dn4 = assign89210_e135776_d_n4;
        locals.var_t0_dn5 = assign89210_e135776_d_n5;
        locals.var_t0_dn6 = assign89210_e135776_d_n6;
        locals.var_t0_dn7 = assign89210_e135776_d_n7;
        locals.var_t0_dn8 = assign89210_e135776_d_n8;
        locals.var_t0_dn9 = assign89210_e135776_d_n9;
        locals.var_t0_dn10 = assign89210_e135776_d_n10;
        locals.var_t0_dn13 = assign89210_e135776_d_n13;

    }

    pub(super) fn stamp_transient_block_313(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89220_e135793, assign89220_e135793_d_n0, assign89220_e135793_d_n2, assign89220_e135793_d_n4, assign89220_e135793_d_n5, assign89220_e135793_d_n6, assign89220_e135793_d_n7, assign89220_e135793_d_n8, assign89220_e135793_d_n9, assign89220_e135793_d_n10, assign89220_e135793_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        let assign89220_e135789: f64 = (locals.var_ps0ld_bef1__blk2049 - 0.1);
        let assign89220_e135791: f64 = (assign89220_e135789 + locals.var_tmf0);
        (assign89220_e135791, (locals.var_ps0ld_bef1__blk2049_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk2049_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk2049_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk2049_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk2049_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk2049_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk2049_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk2049_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk2049_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk2049_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign89220_e135793;
        locals.var_ps0ld_dn0 = assign89220_e135793_d_n0;
        locals.var_ps0ld_dn2 = assign89220_e135793_d_n2;
        locals.var_ps0ld_dn4 = assign89220_e135793_d_n4;
        locals.var_ps0ld_dn5 = assign89220_e135793_d_n5;
        locals.var_ps0ld_dn6 = assign89220_e135793_d_n6;
        locals.var_ps0ld_dn7 = assign89220_e135793_d_n7;
        locals.var_ps0ld_dn8 = assign89220_e135793_d_n8;
        locals.var_ps0ld_dn9 = assign89220_e135793_d_n9;
        locals.var_ps0ld_dn10 = assign89220_e135793_d_n10;
        locals.var_ps0ld_dn13 = assign89220_e135793_d_n13;

        let (assign89230_e135806, assign89230_e135806_d_n0, assign89230_e135806_d_n2, assign89230_e135806_d_n4, assign89230_e135806_d_n5, assign89230_e135806_d_n6, assign89230_e135806_d_n7, assign89230_e135806_d_n8, assign89230_e135806_d_n9, assign89230_e135806_d_n10, assign89230_e135806_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89230_e135806;
        locals.var_t0_dn0 = assign89230_e135806_d_n0;
        locals.var_t0_dn2 = assign89230_e135806_d_n2;
        locals.var_t0_dn4 = assign89230_e135806_d_n4;
        locals.var_t0_dn5 = assign89230_e135806_d_n5;
        locals.var_t0_dn6 = assign89230_e135806_d_n6;
        locals.var_t0_dn7 = assign89230_e135806_d_n7;
        locals.var_t0_dn8 = assign89230_e135806_d_n8;
        locals.var_t0_dn9 = assign89230_e135806_d_n9;
        locals.var_t0_dn10 = assign89230_e135806_d_n10;
        locals.var_t0_dn13 = assign89230_e135806_d_n13;

        let (assign89240_e135820, assign89240_e135820_d_n0, assign89240_e135820_d_n2, assign89240_e135820_d_n4, assign89240_e135820_d_n5, assign89240_e135820_d_n6, assign89240_e135820_d_n7, assign89240_e135820_d_n8, assign89240_e135820_d_n9, assign89240_e135820_d_n10, assign89240_e135820_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign89240_e135820;
        locals.var_ps0ld_dn0 = assign89240_e135820_d_n0;
        locals.var_ps0ld_dn2 = assign89240_e135820_d_n2;
        locals.var_ps0ld_dn4 = assign89240_e135820_d_n4;
        locals.var_ps0ld_dn5 = assign89240_e135820_d_n5;
        locals.var_ps0ld_dn6 = assign89240_e135820_d_n6;
        locals.var_ps0ld_dn7 = assign89240_e135820_d_n7;
        locals.var_ps0ld_dn8 = assign89240_e135820_d_n8;
        locals.var_ps0ld_dn9 = assign89240_e135820_d_n9;
        locals.var_ps0ld_dn10 = assign89240_e135820_d_n10;
        locals.var_ps0ld_dn13 = assign89240_e135820_d_n13;

        let (assign89250_e135834, assign89250_e135834_d_n0, assign89250_e135834_d_n2, assign89250_e135834_d_n4, assign89250_e135834_d_n5, assign89250_e135834_d_n6, assign89250_e135834_d_n7, assign89250_e135834_d_n8, assign89250_e135834_d_n9, assign89250_e135834_d_n10, assign89250_e135834_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 != 0.0)) && (locals.var_guard2061 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89250_e135834;
        locals.var_t0_dn0 = assign89250_e135834_d_n0;
        locals.var_t0_dn2 = assign89250_e135834_d_n2;
        locals.var_t0_dn4 = assign89250_e135834_d_n4;
        locals.var_t0_dn5 = assign89250_e135834_d_n5;
        locals.var_t0_dn6 = assign89250_e135834_d_n6;
        locals.var_t0_dn7 = assign89250_e135834_d_n7;
        locals.var_t0_dn8 = assign89250_e135834_d_n8;
        locals.var_t0_dn9 = assign89250_e135834_d_n9;
        locals.var_t0_dn10 = assign89250_e135834_d_n10;
        locals.var_t0_dn13 = assign89250_e135834_d_n13;

        let (assign89260_e135851, assign89260_e135851_d_n0, assign89260_e135851_d_n2, assign89260_e135851_d_n4, assign89260_e135851_d_n5, assign89260_e135851_d_n6, assign89260_e135851_d_n7, assign89260_e135851_d_n8, assign89260_e135851_d_n9, assign89260_e135851_d_n10, assign89260_e135851_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2059 != 0.0)) && (locals.var_guard2060 == 0.0)) {
        let (assign89260_e135849, assign89260_e135849_d_n0, assign89260_e135849_d_n2, assign89260_e135849_d_n4, assign89260_e135849_d_n5, assign89260_e135849_d_n6, assign89260_e135849_d_n7, assign89260_e135849_d_n8, assign89260_e135849_d_n9, assign89260_e135849_d_n10, assign89260_e135849_d_n13,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk2049) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
            } else {
                (locals.var_ps0ld_bef1__blk2049, locals.var_ps0ld_bef1__blk2049_dn0, locals.var_ps0ld_bef1__blk2049_dn2, locals.var_ps0ld_bef1__blk2049_dn4, locals.var_ps0ld_bef1__blk2049_dn5, locals.var_ps0ld_bef1__blk2049_dn6, locals.var_ps0ld_bef1__blk2049_dn7, locals.var_ps0ld_bef1__blk2049_dn8, locals.var_ps0ld_bef1__blk2049_dn9, locals.var_ps0ld_bef1__blk2049_dn10, locals.var_ps0ld_bef1__blk2049_dn13,)
            }
        };
        (assign89260_e135849, assign89260_e135849_d_n0, assign89260_e135849_d_n2, assign89260_e135849_d_n4, assign89260_e135849_d_n5, assign89260_e135849_d_n6, assign89260_e135849_d_n7, assign89260_e135849_d_n8, assign89260_e135849_d_n9, assign89260_e135849_d_n10, assign89260_e135849_d_n13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign89260_e135851;
        locals.var_ps0ld_dn0 = assign89260_e135851_d_n0;
        locals.var_ps0ld_dn2 = assign89260_e135851_d_n2;
        locals.var_ps0ld_dn4 = assign89260_e135851_d_n4;
        locals.var_ps0ld_dn5 = assign89260_e135851_d_n5;
        locals.var_ps0ld_dn6 = assign89260_e135851_d_n6;
        locals.var_ps0ld_dn7 = assign89260_e135851_d_n7;
        locals.var_ps0ld_dn8 = assign89260_e135851_d_n8;
        locals.var_ps0ld_dn9 = assign89260_e135851_d_n9;
        locals.var_ps0ld_dn10 = assign89260_e135851_d_n10;
        locals.var_ps0ld_dn13 = assign89260_e135851_d_n13;

        let (assign89270_e135858, assign89270_e135858_d_n0, assign89270_e135858_d_n2, assign89270_e135858_d_n4, assign89270_e135858_d_n5, assign89270_e135858_d_n6, assign89270_e135858_d_n7, assign89270_e135858_d_n8, assign89270_e135858_d_n9, assign89270_e135858_d_n10, assign89270_e135858_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk2011, locals.var_ps0ld_ini__blk2011_dn0, locals.var_ps0ld_ini__blk2011_dn2, locals.var_ps0ld_ini__blk2011_dn4, locals.var_ps0ld_ini__blk2011_dn5, locals.var_ps0ld_ini__blk2011_dn6, locals.var_ps0ld_ini__blk2011_dn7, locals.var_ps0ld_ini__blk2011_dn8, locals.var_ps0ld_ini__blk2011_dn9, locals.var_ps0ld_ini__blk2011_dn10, locals.var_ps0ld_ini__blk2011_dn13,)
    }
};
        locals.var_ps0ld_ini__blk2011 = assign89270_e135858;
        locals.var_ps0ld_ini__blk2011_dn0 = assign89270_e135858_d_n0;
        locals.var_ps0ld_ini__blk2011_dn2 = assign89270_e135858_d_n2;
        locals.var_ps0ld_ini__blk2011_dn4 = assign89270_e135858_d_n4;
        locals.var_ps0ld_ini__blk2011_dn5 = assign89270_e135858_d_n5;
        locals.var_ps0ld_ini__blk2011_dn6 = assign89270_e135858_d_n6;
        locals.var_ps0ld_ini__blk2011_dn7 = assign89270_e135858_d_n7;
        locals.var_ps0ld_ini__blk2011_dn8 = assign89270_e135858_d_n8;
        locals.var_ps0ld_ini__blk2011_dn9 = assign89270_e135858_d_n9;
        locals.var_ps0ld_ini__blk2011_dn10 = assign89270_e135858_d_n10;
        locals.var_ps0ld_ini__blk2011_dn13 = assign89270_e135858_d_n13;

        let assign89280_e135861: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2067 = assign89280_e135861;

        let (assign89290_e135870,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign89290_e135870;

        let (assign89300_e135886, assign89300_e135886_d_n0, assign89300_e135886_d_n2, assign89300_e135886_d_n4, assign89300_e135886_d_n5, assign89300_e135886_d_n6, assign89300_e135886_d_n7, assign89300_e135886_d_n8, assign89300_e135886_d_n9, assign89300_e135886_d_n10, assign89300_e135886_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89300_e135880: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2004);
        let assign89300_e135882: f64 = (assign89300_e135880 * locals.var_beta_inv);
        let assign89300_e135883: f64 = (2.0 * assign89300_e135882);
        let assign89300_e135884: f64 = (assign89300_e135883).sqrt();
        (assign89300_e135884, ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn0)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn2)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn4)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn5)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn6)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn7)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn8)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn9)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn10)) / (2.0 * assign89300_e135884)), ((2.0 * (assign89300_e135880 * locals.var_beta_inv_dn13)) / (2.0 * assign89300_e135884)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign89300_e135886;
        locals.var_c_w_ld_dn0 = assign89300_e135886_d_n0;
        locals.var_c_w_ld_dn2 = assign89300_e135886_d_n2;
        locals.var_c_w_ld_dn4 = assign89300_e135886_d_n4;
        locals.var_c_w_ld_dn5 = assign89300_e135886_d_n5;
        locals.var_c_w_ld_dn6 = assign89300_e135886_d_n6;
        locals.var_c_w_ld_dn7 = assign89300_e135886_d_n7;
        locals.var_c_w_ld_dn8 = assign89300_e135886_d_n8;
        locals.var_c_w_ld_dn9 = assign89300_e135886_d_n9;
        locals.var_c_w_ld_dn10 = assign89300_e135886_d_n10;
        locals.var_c_w_ld_dn13 = assign89300_e135886_d_n13;

        let assign89310_e135889: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2068 = assign89310_e135889;

        let (assign89320_e135902, assign89320_e135902_d_n0, assign89320_e135902_d_n2, assign89320_e135902_d_n4, assign89320_e135902_d_n5, assign89320_e135902_d_n6, assign89320_e135902_d_n7, assign89320_e135902_d_n8, assign89320_e135902_d_n9, assign89320_e135902_d_n10, assign89320_e135902_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 != 0.0)) {
        let assign89320_e135900: f64 = (p.p334 - locals.var_wdep_func);
        (assign89320_e135900, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89320_e135902;
        locals.var_t2_dn0 = assign89320_e135902_d_n0;
        locals.var_t2_dn2 = assign89320_e135902_d_n2;
        locals.var_t2_dn4 = assign89320_e135902_d_n4;
        locals.var_t2_dn5 = assign89320_e135902_d_n5;
        locals.var_t2_dn6 = assign89320_e135902_d_n6;
        locals.var_t2_dn7 = assign89320_e135902_d_n7;
        locals.var_t2_dn8 = assign89320_e135902_d_n8;
        locals.var_t2_dn9 = assign89320_e135902_d_n9;
        locals.var_t2_dn10 = assign89320_e135902_d_n10;
        locals.var_t2_dn13 = assign89320_e135902_d_n13;

        let (assign89330_e135927, assign89330_e135927_d_n0, assign89330_e135927_d_n2, assign89330_e135927_d_n4, assign89330_e135927_d_n5, assign89330_e135927_d_n6, assign89330_e135927_d_n7, assign89330_e135927_d_n8, assign89330_e135927_d_n9, assign89330_e135927_d_n10, assign89330_e135927_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89330_e135914: f64 = (locals.var_vdsi + p.p137);
        let assign89330_e135917: f64 = (locals.var_vdsi + p.p137);
        let assign89330_e135918: f64 = (assign89330_e135914 * assign89330_e135917);
        let assign89330_e135921: f64 = (4.0 * 0.1);
        let assign89330_e135923: f64 = (assign89330_e135921 * 0.1);
        let assign89330_e135924: f64 = (assign89330_e135918 + assign89330_e135923);
        let assign89330_e135925: f64 = (assign89330_e135924).sqrt();
        (assign89330_e135925, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign89330_e135917) + (assign89330_e135914 * locals.var_vdsi_dn5)) / (2.0 * assign89330_e135925)), 0.0, (((locals.var_vdsi_dn7 * assign89330_e135917) + (assign89330_e135914 * locals.var_vdsi_dn7)) / (2.0 * assign89330_e135925)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign89330_e135927;
        locals.var_tmf2_dn0 = assign89330_e135927_d_n0;
        locals.var_tmf2_dn2 = assign89330_e135927_d_n2;
        locals.var_tmf2_dn4 = assign89330_e135927_d_n4;
        locals.var_tmf2_dn5 = assign89330_e135927_d_n5;
        locals.var_tmf2_dn6 = assign89330_e135927_d_n6;
        locals.var_tmf2_dn7 = assign89330_e135927_d_n7;
        locals.var_tmf2_dn8 = assign89330_e135927_d_n8;
        locals.var_tmf2_dn9 = assign89330_e135927_d_n9;
        locals.var_tmf2_dn10 = assign89330_e135927_d_n10;
        locals.var_tmf2_dn13 = assign89330_e135927_d_n13;

        let (assign89340_e135947, assign89340_e135947_d_n0, assign89340_e135947_d_n2, assign89340_e135947_d_n4, assign89340_e135947_d_n5, assign89340_e135947_d_n6, assign89340_e135947_d_n7, assign89340_e135947_d_n8, assign89340_e135947_d_n9, assign89340_e135947_d_n10, assign89340_e135947_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89340_e135941: f64 = (locals.var_vdsi + p.p137);
        let assign89340_e135943: f64 = (assign89340_e135941 / locals.var_tmf2);
        let assign89340_e135944: f64 = (1.0 + assign89340_e135943);
        let assign89340_e135945: f64 = (0.5 * assign89340_e135944);
        (assign89340_e135945, (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign89340_e135941 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign89340_e135941 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89340_e135941 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89340_e135947;
        locals.var_t9_dn0 = assign89340_e135947_d_n0;
        locals.var_t9_dn2 = assign89340_e135947_d_n2;
        locals.var_t9_dn4 = assign89340_e135947_d_n4;
        locals.var_t9_dn5 = assign89340_e135947_d_n5;
        locals.var_t9_dn6 = assign89340_e135947_d_n6;
        locals.var_t9_dn7 = assign89340_e135947_d_n7;
        locals.var_t9_dn8 = assign89340_e135947_d_n8;
        locals.var_t9_dn9 = assign89340_e135947_d_n9;
        locals.var_t9_dn10 = assign89340_e135947_d_n10;
        locals.var_t9_dn13 = assign89340_e135947_d_n13;

        let (assign89350_e135965, assign89350_e135965_d_n0, assign89350_e135965_d_n2, assign89350_e135965_d_n4, assign89350_e135965_d_n5, assign89350_e135965_d_n6, assign89350_e135965_d_n7, assign89350_e135965_d_n8, assign89350_e135965_d_n9, assign89350_e135965_d_n10, assign89350_e135965_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89350_e135960: f64 = (locals.var_vdsi + p.p137);
        let assign89350_e135962: f64 = (assign89350_e135960 + locals.var_tmf2);
        let assign89350_e135963: f64 = (0.5 * assign89350_e135962);
        (assign89350_e135963, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89350_e135965;
        locals.var_t2_dn0 = assign89350_e135965_d_n0;
        locals.var_t2_dn2 = assign89350_e135965_d_n2;
        locals.var_t2_dn4 = assign89350_e135965_d_n4;
        locals.var_t2_dn5 = assign89350_e135965_d_n5;
        locals.var_t2_dn6 = assign89350_e135965_d_n6;
        locals.var_t2_dn7 = assign89350_e135965_d_n7;
        locals.var_t2_dn8 = assign89350_e135965_d_n8;
        locals.var_t2_dn9 = assign89350_e135965_d_n9;
        locals.var_t2_dn10 = assign89350_e135965_d_n10;
        locals.var_t2_dn13 = assign89350_e135965_d_n13;

        let assign89360_e135968: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2069 = assign89360_e135968;

        let (assign89370_e135982, assign89370_e135982_d_n0, assign89370_e135982_d_n2, assign89370_e135982_d_n4, assign89370_e135982_d_n5, assign89370_e135982_d_n6, assign89370_e135982_d_n7, assign89370_e135982_d_n8, assign89370_e135982_d_n9, assign89370_e135982_d_n10, assign89370_e135982_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89370_e135982;
        locals.var_t2_dn0 = assign89370_e135982_d_n0;
        locals.var_t2_dn2 = assign89370_e135982_d_n2;
        locals.var_t2_dn4 = assign89370_e135982_d_n4;
        locals.var_t2_dn5 = assign89370_e135982_d_n5;
        locals.var_t2_dn6 = assign89370_e135982_d_n6;
        locals.var_t2_dn7 = assign89370_e135982_d_n7;
        locals.var_t2_dn8 = assign89370_e135982_d_n8;
        locals.var_t2_dn9 = assign89370_e135982_d_n9;
        locals.var_t2_dn10 = assign89370_e135982_d_n10;
        locals.var_t2_dn13 = assign89370_e135982_d_n13;

        let (assign89380_e135996, assign89380_e135996_d_n0, assign89380_e135996_d_n2, assign89380_e135996_d_n4, assign89380_e135996_d_n5, assign89380_e135996_d_n6, assign89380_e135996_d_n7, assign89380_e135996_d_n8, assign89380_e135996_d_n9, assign89380_e135996_d_n10, assign89380_e135996_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) && (locals.var_guard2069 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89380_e135996;
        locals.var_t9_dn0 = assign89380_e135996_d_n0;
        locals.var_t9_dn2 = assign89380_e135996_d_n2;
        locals.var_t9_dn4 = assign89380_e135996_d_n4;
        locals.var_t9_dn5 = assign89380_e135996_d_n5;
        locals.var_t9_dn6 = assign89380_e135996_d_n6;
        locals.var_t9_dn7 = assign89380_e135996_d_n7;
        locals.var_t9_dn8 = assign89380_e135996_d_n8;
        locals.var_t9_dn9 = assign89380_e135996_d_n9;
        locals.var_t9_dn10 = assign89380_e135996_d_n10;
        locals.var_t9_dn13 = assign89380_e135996_d_n13;

        let (assign89390_e136013, assign89390_e136013_d_n0, assign89390_e136013_d_n2, assign89390_e136013_d_n4, assign89390_e136013_d_n5, assign89390_e136013_d_n6, assign89390_e136013_d_n7, assign89390_e136013_d_n8, assign89390_e136013_d_n9, assign89390_e136013_d_n10, assign89390_e136013_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89390_e136008: f64 = (locals.var_kjunc * locals.var_t2);
        let assign89390_e136009: f64 = (assign89390_e136008).sqrt();
        let assign89390_e136011: f64 = (assign89390_e136009 * p.p432);
        (assign89390_e136011, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign89390_e136009)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign89390_e136009)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign89390_e136013;
        locals.var_wjunc0_dn0 = assign89390_e136013_d_n0;
        locals.var_wjunc0_dn2 = assign89390_e136013_d_n2;
        locals.var_wjunc0_dn4 = assign89390_e136013_d_n4;
        locals.var_wjunc0_dn5 = assign89390_e136013_d_n5;
        locals.var_wjunc0_dn6 = assign89390_e136013_d_n6;
        locals.var_wjunc0_dn7 = assign89390_e136013_d_n7;
        locals.var_wjunc0_dn8 = assign89390_e136013_d_n8;
        locals.var_wjunc0_dn9 = assign89390_e136013_d_n9;
        locals.var_wjunc0_dn10 = assign89390_e136013_d_n10;
        locals.var_wjunc0_dn13 = assign89390_e136013_d_n13;

        let (assign89400_e136027, assign89400_e136027_d_n0, assign89400_e136027_d_n2, assign89400_e136027_d_n4, assign89400_e136027_d_n5, assign89400_e136027_d_n6, assign89400_e136027_d_n7, assign89400_e136027_d_n8, assign89400_e136027_d_n9, assign89400_e136027_d_n10, assign89400_e136027_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2068 == 0.0)) {
        let assign89400_e136025: f64 = (p.p334 - locals.var_wjunc0);
        (assign89400_e136025, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89400_e136027;
        locals.var_t2_dn0 = assign89400_e136027_d_n0;
        locals.var_t2_dn2 = assign89400_e136027_d_n2;
        locals.var_t2_dn4 = assign89400_e136027_d_n4;
        locals.var_t2_dn5 = assign89400_e136027_d_n5;
        locals.var_t2_dn6 = assign89400_e136027_d_n6;
        locals.var_t2_dn7 = assign89400_e136027_d_n7;
        locals.var_t2_dn8 = assign89400_e136027_d_n8;
        locals.var_t2_dn9 = assign89400_e136027_d_n9;
        locals.var_t2_dn10 = assign89400_e136027_d_n10;
        locals.var_t2_dn13 = assign89400_e136027_d_n13;

        let (assign89410_e136049, assign89410_e136049_d_n0, assign89410_e136049_d_n2, assign89410_e136049_d_n4, assign89410_e136049_d_n5, assign89410_e136049_d_n6, assign89410_e136049_d_n7, assign89410_e136049_d_n8, assign89410_e136049_d_n9, assign89410_e136049_d_n10, assign89410_e136049_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89410_e136036: f64 = (locals.var_t2 * locals.var_t2);
        let assign89410_e136040: f64 = (p.p334 * 0.01);
        let assign89410_e136041: f64 = (4.0 * assign89410_e136040);
        let assign89410_e136044: f64 = (p.p334 * 0.01);
        let assign89410_e136045: f64 = (assign89410_e136041 * assign89410_e136044);
        let assign89410_e136046: f64 = (assign89410_e136036 + assign89410_e136045);
        let assign89410_e136047: f64 = (assign89410_e136046).sqrt();
        (assign89410_e136047, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign89410_e136047)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign89410_e136047)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign89410_e136049;
        locals.var_tmf2_dn0 = assign89410_e136049_d_n0;
        locals.var_tmf2_dn2 = assign89410_e136049_d_n2;
        locals.var_tmf2_dn4 = assign89410_e136049_d_n4;
        locals.var_tmf2_dn5 = assign89410_e136049_d_n5;
        locals.var_tmf2_dn6 = assign89410_e136049_d_n6;
        locals.var_tmf2_dn7 = assign89410_e136049_d_n7;
        locals.var_tmf2_dn8 = assign89410_e136049_d_n8;
        locals.var_tmf2_dn9 = assign89410_e136049_d_n9;
        locals.var_tmf2_dn10 = assign89410_e136049_d_n10;
        locals.var_tmf2_dn13 = assign89410_e136049_d_n13;

        let (assign89420_e136064, assign89420_e136064_d_n0, assign89420_e136064_d_n2, assign89420_e136064_d_n4, assign89420_e136064_d_n5, assign89420_e136064_d_n6, assign89420_e136064_d_n7, assign89420_e136064_d_n8, assign89420_e136064_d_n9, assign89420_e136064_d_n10, assign89420_e136064_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89420_e136060: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign89420_e136061: f64 = (1.0 + assign89420_e136060);
        let assign89420_e136062: f64 = (0.5 * assign89420_e136061);
        (assign89420_e136062, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89420_e136064;
        locals.var_t9_dn0 = assign89420_e136064_d_n0;
        locals.var_t9_dn2 = assign89420_e136064_d_n2;
        locals.var_t9_dn4 = assign89420_e136064_d_n4;
        locals.var_t9_dn5 = assign89420_e136064_d_n5;
        locals.var_t9_dn6 = assign89420_e136064_d_n6;
        locals.var_t9_dn7 = assign89420_e136064_d_n7;
        locals.var_t9_dn8 = assign89420_e136064_d_n8;
        locals.var_t9_dn9 = assign89420_e136064_d_n9;
        locals.var_t9_dn10 = assign89420_e136064_d_n10;
        locals.var_t9_dn13 = assign89420_e136064_d_n13;

        let (assign89430_e136077, assign89430_e136077_d_n0, assign89430_e136077_d_n2, assign89430_e136077_d_n4, assign89430_e136077_d_n5, assign89430_e136077_d_n6, assign89430_e136077_d_n7, assign89430_e136077_d_n8, assign89430_e136077_d_n9, assign89430_e136077_d_n10, assign89430_e136077_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89430_e136074: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign89430_e136075: f64 = (0.5 * assign89430_e136074);
        (assign89430_e136075, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89430_e136077;
        locals.var_t2_dn0 = assign89430_e136077_d_n0;
        locals.var_t2_dn2 = assign89430_e136077_d_n2;
        locals.var_t2_dn4 = assign89430_e136077_d_n4;
        locals.var_t2_dn5 = assign89430_e136077_d_n5;
        locals.var_t2_dn6 = assign89430_e136077_d_n6;
        locals.var_t2_dn7 = assign89430_e136077_d_n7;
        locals.var_t2_dn8 = assign89430_e136077_d_n8;
        locals.var_t2_dn9 = assign89430_e136077_d_n9;
        locals.var_t2_dn10 = assign89430_e136077_d_n10;
        locals.var_t2_dn13 = assign89430_e136077_d_n13;

        let assign89440_e136080: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2070 = assign89440_e136080;

        let (assign89450_e136091, assign89450_e136091_d_n0, assign89450_e136091_d_n2, assign89450_e136091_d_n4, assign89450_e136091_d_n5, assign89450_e136091_d_n6, assign89450_e136091_d_n7, assign89450_e136091_d_n8, assign89450_e136091_d_n9, assign89450_e136091_d_n10, assign89450_e136091_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2070 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89450_e136091;
        locals.var_t2_dn0 = assign89450_e136091_d_n0;
        locals.var_t2_dn2 = assign89450_e136091_d_n2;
        locals.var_t2_dn4 = assign89450_e136091_d_n4;
        locals.var_t2_dn5 = assign89450_e136091_d_n5;
        locals.var_t2_dn6 = assign89450_e136091_d_n6;
        locals.var_t2_dn7 = assign89450_e136091_d_n7;
        locals.var_t2_dn8 = assign89450_e136091_d_n8;
        locals.var_t2_dn9 = assign89450_e136091_d_n9;
        locals.var_t2_dn10 = assign89450_e136091_d_n10;
        locals.var_t2_dn13 = assign89450_e136091_d_n13;

        let (assign89460_e136102, assign89460_e136102_d_n0, assign89460_e136102_d_n2, assign89460_e136102_d_n4, assign89460_e136102_d_n5, assign89460_e136102_d_n6, assign89460_e136102_d_n7, assign89460_e136102_d_n8, assign89460_e136102_d_n9, assign89460_e136102_d_n10, assign89460_e136102_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2070 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89460_e136102;
        locals.var_t9_dn0 = assign89460_e136102_d_n0;
        locals.var_t9_dn2 = assign89460_e136102_d_n2;
        locals.var_t9_dn4 = assign89460_e136102_d_n4;
        locals.var_t9_dn5 = assign89460_e136102_d_n5;
        locals.var_t9_dn6 = assign89460_e136102_d_n6;
        locals.var_t9_dn7 = assign89460_e136102_d_n7;
        locals.var_t9_dn8 = assign89460_e136102_d_n8;
        locals.var_t9_dn9 = assign89460_e136102_d_n9;
        locals.var_t9_dn10 = assign89460_e136102_d_n10;
        locals.var_t9_dn13 = assign89460_e136102_d_n13;

        let (assign89470_e136111, assign89470_e136111_d_n0, assign89470_e136111_d_n2, assign89470_e136111_d_n4, assign89470_e136111_d_n5, assign89470_e136111_d_n6, assign89470_e136111_d_n7, assign89470_e136111_d_n8, assign89470_e136111_d_n9, assign89470_e136111_d_n10, assign89470_e136111_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign89470_e136111;
        locals.var_ddriftldc_dn0 = assign89470_e136111_d_n0;
        locals.var_ddriftldc_dn2 = assign89470_e136111_d_n2;
        locals.var_ddriftldc_dn4 = assign89470_e136111_d_n4;
        locals.var_ddriftldc_dn5 = assign89470_e136111_d_n5;
        locals.var_ddriftldc_dn6 = assign89470_e136111_d_n6;
        locals.var_ddriftldc_dn7 = assign89470_e136111_d_n7;
        locals.var_ddriftldc_dn8 = assign89470_e136111_d_n8;
        locals.var_ddriftldc_dn9 = assign89470_e136111_d_n9;
        locals.var_ddriftldc_dn10 = assign89470_e136111_d_n10;
        locals.var_ddriftldc_dn13 = assign89470_e136111_d_n13;

        let (assign89480_e136128, assign89480_e136128_d_n0, assign89480_e136128_d_n2, assign89480_e136128_d_n4, assign89480_e136128_d_n5, assign89480_e136128_d_n6, assign89480_e136128_d_n7, assign89480_e136128_d_n8, assign89480_e136128_d_n9, assign89480_e136128_d_n10, assign89480_e136128_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89480_e136120: f64 = (locals.var_q_nsubld__blk2004 * locals.var_ddriftldc);
        let assign89480_e136122: f64 = (assign89480_e136120 * locals.var_ddriftldc);
        let assign89480_e136124: f64 = (assign89480_e136122 / 2.0);
        let assign89480_e136126: f64 = (assign89480_e136124 / 1.034943e-10);
        (assign89480_e136126, (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign89480_e136120 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign89480_e136128;
        locals.var_dphi_sb_dn0 = assign89480_e136128_d_n0;
        locals.var_dphi_sb_dn2 = assign89480_e136128_d_n2;
        locals.var_dphi_sb_dn4 = assign89480_e136128_d_n4;
        locals.var_dphi_sb_dn5 = assign89480_e136128_d_n5;
        locals.var_dphi_sb_dn6 = assign89480_e136128_d_n6;
        locals.var_dphi_sb_dn7 = assign89480_e136128_d_n7;
        locals.var_dphi_sb_dn8 = assign89480_e136128_d_n8;
        locals.var_dphi_sb_dn9 = assign89480_e136128_d_n9;
        locals.var_dphi_sb_dn10 = assign89480_e136128_d_n10;
        locals.var_dphi_sb_dn13 = assign89480_e136128_d_n13;

        let (assign89490_e136142, assign89490_e136142_d_n0, assign89490_e136142_d_n2, assign89490_e136142_d_n4, assign89490_e136142_d_n5, assign89490_e136142_d_n6, assign89490_e136142_d_n7, assign89490_e136142_d_n8, assign89490_e136142_d_n9, assign89490_e136142_d_n10, assign89490_e136142_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89490_e136137: f64 = (2.0 * locals.var_beta);
        let assign89490_e136139: f64 = (assign89490_e136137 * locals.var_dphi_sb);
        let assign89490_e136140: f64 = (assign89490_e136139).sqrt();
        (assign89490_e136140, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn0)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn2)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn4)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn5)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn6)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn7)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn8)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn9)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn10)) / (2.0 * assign89490_e136140)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign89490_e136137 * locals.var_dphi_sb_dn13)) / (2.0 * assign89490_e136140)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89490_e136142;
        locals.var_t0_dn0 = assign89490_e136142_d_n0;
        locals.var_t0_dn2 = assign89490_e136142_d_n2;
        locals.var_t0_dn4 = assign89490_e136142_d_n4;
        locals.var_t0_dn5 = assign89490_e136142_d_n5;
        locals.var_t0_dn6 = assign89490_e136142_d_n6;
        locals.var_t0_dn7 = assign89490_e136142_d_n7;
        locals.var_t0_dn8 = assign89490_e136142_d_n8;
        locals.var_t0_dn9 = assign89490_e136142_d_n9;
        locals.var_t0_dn10 = assign89490_e136142_d_n10;
        locals.var_t0_dn13 = assign89490_e136142_d_n13;

    }

    pub(super) fn stamp_transient_block_314(
        locals: &mut StampLocals,
    ) {
        let (assign89500_e136158, assign89500_e136158_d_n0, assign89500_e136158_d_n2, assign89500_e136158_d_n4, assign89500_e136158_d_n5, assign89500_e136158_d_n6, assign89500_e136158_d_n7, assign89500_e136158_d_n8, assign89500_e136158_d_n9, assign89500_e136158_d_n10, assign89500_e136158_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89500_e136150: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89500_e136152: f64 = (-locals.var_t0);
        let assign89500_e136153: f64 = { let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89500_e136154: f64 = (assign89500_e136150 + assign89500_e136153);
        let assign89500_e136156: f64 = (assign89500_e136154 / 2.0);
        (assign89500_e136156, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign89500_e136152; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign89500_e136158;
        locals.var_t1_dn0 = assign89500_e136158_d_n0;
        locals.var_t1_dn2 = assign89500_e136158_d_n2;
        locals.var_t1_dn4 = assign89500_e136158_d_n4;
        locals.var_t1_dn5 = assign89500_e136158_d_n5;
        locals.var_t1_dn6 = assign89500_e136158_d_n6;
        locals.var_t1_dn7 = assign89500_e136158_d_n7;
        locals.var_t1_dn8 = assign89500_e136158_d_n8;
        locals.var_t1_dn9 = assign89500_e136158_d_n9;
        locals.var_t1_dn10 = assign89500_e136158_d_n10;
        locals.var_t1_dn13 = assign89500_e136158_d_n13;

        let (assign89510_e136170, assign89510_e136170_d_n0, assign89510_e136170_d_n2, assign89510_e136170_d_n4, assign89510_e136170_d_n5, assign89510_e136170_d_n6, assign89510_e136170_d_n7, assign89510_e136170_d_n8, assign89510_e136170_d_n9, assign89510_e136170_d_n10, assign89510_e136170_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89510_e136166: f64 = (locals.var_t1).ln();
        let assign89510_e136168: f64 = (assign89510_e136166 / locals.var_dphi_sb);
        (assign89510_e136168, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign89510_e136166 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign89510_e136170;
        locals.var_c_sb_dn0 = assign89510_e136170_d_n0;
        locals.var_c_sb_dn2 = assign89510_e136170_d_n2;
        locals.var_c_sb_dn4 = assign89510_e136170_d_n4;
        locals.var_c_sb_dn5 = assign89510_e136170_d_n5;
        locals.var_c_sb_dn6 = assign89510_e136170_d_n6;
        locals.var_c_sb_dn7 = assign89510_e136170_d_n7;
        locals.var_c_sb_dn8 = assign89510_e136170_d_n8;
        locals.var_c_sb_dn9 = assign89510_e136170_d_n9;
        locals.var_c_sb_dn10 = assign89510_e136170_d_n10;
        locals.var_c_sb_dn13 = assign89510_e136170_d_n13;

        let (assign89520_e136179,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign89520_e136179;

    }

    pub(super) fn stamp_transient_block_315(
        locals: &mut StampLocals,
    ) {
        let mut assign89530_loop_guard: usize = 0;
        while {
            let assign89530_cond_e136189: f64 = (locals.var_lp_s0_max + 1.0);
            let assign89530_cond_e136191: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_lp_s0 <= assign89530_cond_e136189)) { 1.0 } else { 0.0 };
            assign89530_cond_e136191 != 0.0
        } {
            assign89530_loop_guard += 1;
            assert!(assign89530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89530_body3_e136227, assign89530_body3_e136227_d_n0, assign89530_body3_e136227_d_n2, assign89530_body3_e136227_d_n4, assign89530_body3_e136227_d_n5, assign89530_body3_e136227_d_n6, assign89530_body3_e136227_d_n7, assign89530_body3_e136227_d_n8, assign89530_body3_e136227_d_n9, assign89530_body3_e136227_d_n10, assign89530_body3_e136227_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body3_e136225: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign89530_body3_e136225, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign89530_body3_e136227;
            locals.var_ps0ld_vxb_dn0 = assign89530_body3_e136227_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign89530_body3_e136227_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign89530_body3_e136227_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign89530_body3_e136227_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign89530_body3_e136227_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign89530_body3_e136227_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign89530_body3_e136227_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign89530_body3_e136227_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign89530_body3_e136227_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign89530_body3_e136227_d_n13;
            let (assign89530_body4_e136238, assign89530_body4_e136238_d_n0, assign89530_body4_e136238_d_n2, assign89530_body4_e136238_d_n4, assign89530_body4_e136238_d_n5, assign89530_body4_e136238_d_n6, assign89530_body4_e136238_d_n7, assign89530_body4_e136238_d_n8, assign89530_body4_e136238_d_n9, assign89530_body4_e136238_d_n10, assign89530_body4_e136238_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body4_e136236: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign89530_body4_e136236, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign89530_body4_e136238;
            locals.var_chi_dn0 = assign89530_body4_e136238_d_n0;
            locals.var_chi_dn2 = assign89530_body4_e136238_d_n2;
            locals.var_chi_dn4 = assign89530_body4_e136238_d_n4;
            locals.var_chi_dn5 = assign89530_body4_e136238_d_n5;
            locals.var_chi_dn6 = assign89530_body4_e136238_d_n6;
            locals.var_chi_dn7 = assign89530_body4_e136238_d_n7;
            locals.var_chi_dn8 = assign89530_body4_e136238_d_n8;
            locals.var_chi_dn9 = assign89530_body4_e136238_d_n9;
            locals.var_chi_dn10 = assign89530_body4_e136238_d_n10;
            locals.var_chi_dn13 = assign89530_body4_e136238_d_n13;
            let (assign89530_body5_e136251, assign89530_body5_e136251_d_n0, assign89530_body5_e136251_d_n2, assign89530_body5_e136251_d_n4, assign89530_body5_e136251_d_n5, assign89530_body5_e136251_d_n6, assign89530_body5_e136251_d_n7, assign89530_body5_e136251_d_n8, assign89530_body5_e136251_d_n9, assign89530_body5_e136251_d_n10, assign89530_body5_e136251_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body5_e136248: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign89530_body5_e136249: f64 = (locals.var_c_sb * assign89530_body5_e136248);
        (assign89530_body5_e136249, ((locals.var_c_sb_dn0 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign89530_body5_e136248) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign89530_body5_e136251;
            locals.var_ty_dn0 = assign89530_body5_e136251_d_n0;
            locals.var_ty_dn2 = assign89530_body5_e136251_d_n2;
            locals.var_ty_dn4 = assign89530_body5_e136251_d_n4;
            locals.var_ty_dn5 = assign89530_body5_e136251_d_n5;
            locals.var_ty_dn6 = assign89530_body5_e136251_d_n6;
            locals.var_ty_dn7 = assign89530_body5_e136251_d_n7;
            locals.var_ty_dn8 = assign89530_body5_e136251_d_n8;
            locals.var_ty_dn9 = assign89530_body5_e136251_d_n9;
            locals.var_ty_dn10 = assign89530_body5_e136251_d_n10;
            locals.var_ty_dn13 = assign89530_body5_e136251_d_n13;
            let assign89530_body6_e136254: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2072 = assign89530_body6_e136254;
            let (assign89530_body7_e136266, assign89530_body7_e136266_d_n0, assign89530_body7_e136266_d_n2, assign89530_body7_e136266_d_n4, assign89530_body7_e136266_d_n5, assign89530_body7_e136266_d_n6, assign89530_body7_e136266_d_n7, assign89530_body7_e136266_d_n8, assign89530_body7_e136266_d_n9, assign89530_body7_e136266_d_n10, assign89530_body7_e136266_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body7_e136264: f64 = (locals.var_ty).exp();
        (assign89530_body7_e136264, (assign89530_body7_e136264 * locals.var_ty_dn0), (assign89530_body7_e136264 * locals.var_ty_dn2), (assign89530_body7_e136264 * locals.var_ty_dn4), (assign89530_body7_e136264 * locals.var_ty_dn5), (assign89530_body7_e136264 * locals.var_ty_dn6), (assign89530_body7_e136264 * locals.var_ty_dn7), (assign89530_body7_e136264 * locals.var_ty_dn8), (assign89530_body7_e136264 * locals.var_ty_dn9), (assign89530_body7_e136264 * locals.var_ty_dn10), (assign89530_body7_e136264 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body7_e136266;
            locals.var_t1_dn0 = assign89530_body7_e136266_d_n0;
            locals.var_t1_dn2 = assign89530_body7_e136266_d_n2;
            locals.var_t1_dn4 = assign89530_body7_e136266_d_n4;
            locals.var_t1_dn5 = assign89530_body7_e136266_d_n5;
            locals.var_t1_dn6 = assign89530_body7_e136266_d_n6;
            locals.var_t1_dn7 = assign89530_body7_e136266_d_n7;
            locals.var_t1_dn8 = assign89530_body7_e136266_d_n8;
            locals.var_t1_dn9 = assign89530_body7_e136266_d_n9;
            locals.var_t1_dn10 = assign89530_body7_e136266_d_n10;
            locals.var_t1_dn13 = assign89530_body7_e136266_d_n13;
            let (assign89530_body8_e136281, assign89530_body8_e136281_d_n0, assign89530_body8_e136281_d_n2, assign89530_body8_e136281_d_n4, assign89530_body8_e136281_d_n5, assign89530_body8_e136281_d_n6, assign89530_body8_e136281_d_n7, assign89530_body8_e136281_d_n8, assign89530_body8_e136281_d_n9, assign89530_body8_e136281_d_n10, assign89530_body8_e136281_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body8_e136276: f64 = (-locals.var_c_sb);
        let assign89530_body8_e136278: f64 = (assign89530_body8_e136276 * locals.var_dphi_sb);
        let assign89530_body8_e136279: f64 = (assign89530_body8_e136278).exp();
        (assign89530_body8_e136279, (assign89530_body8_e136279 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn0))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn2))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn4))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn5))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn6))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn7))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn8))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn9))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn10))), (assign89530_body8_e136279 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign89530_body8_e136276 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body8_e136281;
            locals.var_t0_dn0 = assign89530_body8_e136281_d_n0;
            locals.var_t0_dn2 = assign89530_body8_e136281_d_n2;
            locals.var_t0_dn4 = assign89530_body8_e136281_d_n4;
            locals.var_t0_dn5 = assign89530_body8_e136281_d_n5;
            locals.var_t0_dn6 = assign89530_body8_e136281_d_n6;
            locals.var_t0_dn7 = assign89530_body8_e136281_d_n7;
            locals.var_t0_dn8 = assign89530_body8_e136281_d_n8;
            locals.var_t0_dn9 = assign89530_body8_e136281_d_n9;
            locals.var_t0_dn10 = assign89530_body8_e136281_d_n10;
            locals.var_t0_dn13 = assign89530_body8_e136281_d_n13;
            let (assign89530_body9_e136294, assign89530_body9_e136294_d_n0, assign89530_body9_e136294_d_n2, assign89530_body9_e136294_d_n4, assign89530_body9_e136294_d_n5, assign89530_body9_e136294_d_n6, assign89530_body9_e136294_d_n7, assign89530_body9_e136294_d_n8, assign89530_body9_e136294_d_n9, assign89530_body9_e136294_d_n10, assign89530_body9_e136294_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body9_e136292: f64 = (locals.var_t1 - locals.var_t0);
        (assign89530_body9_e136292, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign89530_body9_e136294;
            locals.var_t2_dn0 = assign89530_body9_e136294_d_n0;
            locals.var_t2_dn2 = assign89530_body9_e136294_d_n2;
            locals.var_t2_dn4 = assign89530_body9_e136294_d_n4;
            locals.var_t2_dn5 = assign89530_body9_e136294_d_n5;
            locals.var_t2_dn6 = assign89530_body9_e136294_d_n6;
            locals.var_t2_dn7 = assign89530_body9_e136294_d_n7;
            locals.var_t2_dn8 = assign89530_body9_e136294_d_n8;
            locals.var_t2_dn9 = assign89530_body9_e136294_d_n9;
            locals.var_t2_dn10 = assign89530_body9_e136294_d_n10;
            locals.var_t2_dn13 = assign89530_body9_e136294_d_n13;
            let (assign89530_body10_e136310, assign89530_body10_e136310_d_n0, assign89530_body10_e136310_d_n2, assign89530_body10_e136310_d_n4, assign89530_body10_e136310_d_n5, assign89530_body10_e136310_d_n6, assign89530_body10_e136310_d_n7, assign89530_body10_e136310_d_n8, assign89530_body10_e136310_d_n9, assign89530_body10_e136310_d_n10, assign89530_body10_e136310_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body10_e136305: f64 = (1.0 + locals.var_t2);
        let assign89530_body10_e136306: f64 = (assign89530_body10_e136305).ln();
        let assign89530_body10_e136308: f64 = (assign89530_body10_e136306 / locals.var_c_sb);
        (assign89530_body10_e136308, ((((locals.var_t2_dn0 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign89530_body10_e136305) * locals.var_c_sb) - (assign89530_body10_e136306 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign89530_body10_e136310;
            locals.var_phi_b_dn0 = assign89530_body10_e136310_d_n0;
            locals.var_phi_b_dn2 = assign89530_body10_e136310_d_n2;
            locals.var_phi_b_dn4 = assign89530_body10_e136310_d_n4;
            locals.var_phi_b_dn5 = assign89530_body10_e136310_d_n5;
            locals.var_phi_b_dn6 = assign89530_body10_e136310_d_n6;
            locals.var_phi_b_dn7 = assign89530_body10_e136310_d_n7;
            locals.var_phi_b_dn8 = assign89530_body10_e136310_d_n8;
            locals.var_phi_b_dn9 = assign89530_body10_e136310_d_n9;
            locals.var_phi_b_dn10 = assign89530_body10_e136310_d_n10;
            locals.var_phi_b_dn13 = assign89530_body10_e136310_d_n13;
            let (assign89530_body11_e136325, assign89530_body11_e136325_d_n0, assign89530_body11_e136325_d_n2, assign89530_body11_e136325_d_n4, assign89530_body11_e136325_d_n5, assign89530_body11_e136325_d_n6, assign89530_body11_e136325_d_n7, assign89530_body11_e136325_d_n8, assign89530_body11_e136325_d_n9, assign89530_body11_e136325_d_n10, assign89530_body11_e136325_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 != 0.0)) {
        let assign89530_body11_e136322: f64 = (1.0 + locals.var_t2);
        let assign89530_body11_e136323: f64 = (locals.var_t1 / assign89530_body11_e136322);
        (assign89530_body11_e136323, (((locals.var_t1_dn0 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn0)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn2 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn2)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn4 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn4)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn5 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn5)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn6 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn6)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn7 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn7)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn8 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn8)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn9 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn9)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn10 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn10)) / (assign89530_body11_e136322 * assign89530_body11_e136322)), (((locals.var_t1_dn13 * assign89530_body11_e136322) - (locals.var_t1 * locals.var_t2_dn13)) / (assign89530_body11_e136322 * assign89530_body11_e136322)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign89530_body11_e136325;
            locals.var_phi_b_dpss_dn0 = assign89530_body11_e136325_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89530_body11_e136325_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89530_body11_e136325_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89530_body11_e136325_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89530_body11_e136325_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89530_body11_e136325_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89530_body11_e136325_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89530_body11_e136325_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89530_body11_e136325_d_n10;
            locals.var_phi_b_dpss_dn13 = assign89530_body11_e136325_d_n13;
            let (assign89530_body13_e136353, assign89530_body13_e136353_d_n0, assign89530_body13_e136353_d_n2, assign89530_body13_e136353_d_n4, assign89530_body13_e136353_d_n5, assign89530_body13_e136353_d_n6, assign89530_body13_e136353_d_n7, assign89530_body13_e136353_d_n8, assign89530_body13_e136353_d_n9, assign89530_body13_e136353_d_n10, assign89530_body13_e136353_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 == 0.0)) {
        let assign89530_body13_e136351: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign89530_body13_e136351, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign89530_body13_e136353;
            locals.var_phi_b_dn0 = assign89530_body13_e136353_d_n0;
            locals.var_phi_b_dn2 = assign89530_body13_e136353_d_n2;
            locals.var_phi_b_dn4 = assign89530_body13_e136353_d_n4;
            locals.var_phi_b_dn5 = assign89530_body13_e136353_d_n5;
            locals.var_phi_b_dn6 = assign89530_body13_e136353_d_n6;
            locals.var_phi_b_dn7 = assign89530_body13_e136353_d_n7;
            locals.var_phi_b_dn8 = assign89530_body13_e136353_d_n8;
            locals.var_phi_b_dn9 = assign89530_body13_e136353_d_n9;
            locals.var_phi_b_dn10 = assign89530_body13_e136353_d_n10;
            locals.var_phi_b_dn13 = assign89530_body13_e136353_d_n13;
            let (assign89530_body14_e136365, assign89530_body14_e136365_d_n0, assign89530_body14_e136365_d_n2, assign89530_body14_e136365_d_n4, assign89530_body14_e136365_d_n5, assign89530_body14_e136365_d_n6, assign89530_body14_e136365_d_n7, assign89530_body14_e136365_d_n8, assign89530_body14_e136365_d_n9, assign89530_body14_e136365_d_n10, assign89530_body14_e136365_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2072 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign89530_body14_e136365;
            locals.var_phi_b_dpss_dn0 = assign89530_body14_e136365_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89530_body14_e136365_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89530_body14_e136365_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89530_body14_e136365_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89530_body14_e136365_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89530_body14_e136365_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89530_body14_e136365_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89530_body14_e136365_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89530_body14_e136365_d_n10;
            locals.var_phi_b_dpss_dn13 = assign89530_body14_e136365_d_n13;
            let (assign89530_body15_e136376, assign89530_body15_e136376_d_n0, assign89530_body15_e136376_d_n2, assign89530_body15_e136376_d_n4, assign89530_body15_e136376_d_n5, assign89530_body15_e136376_d_n6, assign89530_body15_e136376_d_n7, assign89530_body15_e136376_d_n8, assign89530_body15_e136376_d_n9, assign89530_body15_e136376_d_n10, assign89530_body15_e136376_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body15_e136374: f64 = (locals.var_beta * locals.var_phi_b);
        (assign89530_body15_e136374, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign89530_body15_e136376;
            locals.var_chib_dn0 = assign89530_body15_e136376_d_n0;
            locals.var_chib_dn2 = assign89530_body15_e136376_d_n2;
            locals.var_chib_dn4 = assign89530_body15_e136376_d_n4;
            locals.var_chib_dn5 = assign89530_body15_e136376_d_n5;
            locals.var_chib_dn6 = assign89530_body15_e136376_d_n6;
            locals.var_chib_dn7 = assign89530_body15_e136376_d_n7;
            locals.var_chib_dn8 = assign89530_body15_e136376_d_n8;
            locals.var_chib_dn9 = assign89530_body15_e136376_d_n9;
            locals.var_chib_dn10 = assign89530_body15_e136376_d_n10;
            locals.var_chib_dn13 = assign89530_body15_e136376_d_n13;
            let assign89530_body16_e136379: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2073 = assign89530_body16_e136379;
            let (assign89530_body18_e136404, assign89530_body18_e136404_d_n0, assign89530_body18_e136404_d_n2, assign89530_body18_e136404_d_n4, assign89530_body18_e136404_d_n5, assign89530_body18_e136404_d_n6, assign89530_body18_e136404_d_n7, assign89530_body18_e136404_d_n8, assign89530_body18_e136404_d_n9, assign89530_body18_e136404_d_n10, assign89530_body18_e136404_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 != 0.0)) {
        let assign89530_body18_e136402: f64 = (-0.7071067811865475);
        (assign89530_body18_e136402, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body18_e136404;
            locals.var_t0_dn0 = assign89530_body18_e136404_d_n0;
            locals.var_t0_dn2 = assign89530_body18_e136404_d_n2;
            locals.var_t0_dn4 = assign89530_body18_e136404_d_n4;
            locals.var_t0_dn5 = assign89530_body18_e136404_d_n5;
            locals.var_t0_dn6 = assign89530_body18_e136404_d_n6;
            locals.var_t0_dn7 = assign89530_body18_e136404_d_n7;
            locals.var_t0_dn8 = assign89530_body18_e136404_d_n8;
            locals.var_t0_dn9 = assign89530_body18_e136404_d_n9;
            locals.var_t0_dn10 = assign89530_body18_e136404_d_n10;
            locals.var_t0_dn13 = assign89530_body18_e136404_d_n13;
            let (assign89530_body19_e136417, assign89530_body19_e136417_d_n0, assign89530_body19_e136417_d_n2, assign89530_body19_e136417_d_n4, assign89530_body19_e136417_d_n5, assign89530_body19_e136417_d_n6, assign89530_body19_e136417_d_n7, assign89530_body19_e136417_d_n8, assign89530_body19_e136417_d_n9, assign89530_body19_e136417_d_n10, assign89530_body19_e136417_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 != 0.0)) {
        let assign89530_body19_e136415: f64 = (locals.var_chi * locals.var_t0);
        (assign89530_body19_e136415, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body19_e136417;
            locals.var_fb_dn0 = assign89530_body19_e136417_d_n0;
            locals.var_fb_dn2 = assign89530_body19_e136417_d_n2;
            locals.var_fb_dn4 = assign89530_body19_e136417_d_n4;
            locals.var_fb_dn5 = assign89530_body19_e136417_d_n5;
            locals.var_fb_dn6 = assign89530_body19_e136417_d_n6;
            locals.var_fb_dn7 = assign89530_body19_e136417_d_n7;
            locals.var_fb_dn8 = assign89530_body19_e136417_d_n8;
            locals.var_fb_dn9 = assign89530_body19_e136417_d_n9;
            locals.var_fb_dn10 = assign89530_body19_e136417_d_n10;
            locals.var_fb_dn13 = assign89530_body19_e136417_d_n13;
            let (assign89530_body20_e136430, assign89530_body20_e136430_d_n0, assign89530_body20_e136430_d_n2, assign89530_body20_e136430_d_n4, assign89530_body20_e136430_d_n5, assign89530_body20_e136430_d_n6, assign89530_body20_e136430_d_n7, assign89530_body20_e136430_d_n8, assign89530_body20_e136430_d_n9, assign89530_body20_e136430_d_n10, assign89530_body20_e136430_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 != 0.0)) {
        let assign89530_body20_e136428: f64 = (locals.var_beta * locals.var_t0);
        (assign89530_body20_e136428, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body20_e136430;
            locals.var_fb_dpss_dn0 = assign89530_body20_e136430_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body20_e136430_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body20_e136430_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body20_e136430_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body20_e136430_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body20_e136430_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body20_e136430_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body20_e136430_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body20_e136430_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body20_e136430_d_n13;
            let assign89530_body21_e136433: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2074 = assign89530_body21_e136433;
            let (assign89530_body23_e136485, assign89530_body23_e136485_d_n0, assign89530_body23_e136485_d_n2, assign89530_body23_e136485_d_n4, assign89530_body23_e136485_d_n5, assign89530_body23_e136485_d_n6, assign89530_body23_e136485_d_n7, assign89530_body23_e136485_d_n8, assign89530_body23_e136485_d_n9, assign89530_body23_e136485_d_n10, assign89530_body23_e136485_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body23_e136463: f64 = (locals.var_chi * locals.var_chi);
        let assign89530_body23_e136465: f64 = (assign89530_body23_e136463 / 2.0);
        let assign89530_body23_e136469: f64 = (locals.var_chi / 3.0);
        let assign89530_body23_e136473: f64 = (locals.var_chi / 4.0);
        let assign89530_body23_e136477: f64 = (locals.var_chi / 5.0);
        let assign89530_body23_e136478: f64 = (1.0 - assign89530_body23_e136477);
        let assign89530_body23_e136479: f64 = (assign89530_body23_e136473 * assign89530_body23_e136478);
        let assign89530_body23_e136480: f64 = (1.0 - assign89530_body23_e136479);
        let assign89530_body23_e136481: f64 = (assign89530_body23_e136469 * assign89530_body23_e136480);
        let assign89530_body23_e136482: f64 = (1.0 - assign89530_body23_e136481);
        let assign89530_body23_e136483: f64 = (assign89530_body23_e136465 * assign89530_body23_e136482);
        (assign89530_body23_e136483, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn0 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn0 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn2 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn2 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn4 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn4 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn5 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn5 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn6 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn6 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn7 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn7 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn8 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn8 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn9 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn9 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn10 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn10 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign89530_body23_e136482) + (assign89530_body23_e136465 * (-(((locals.var_chi_dn13 / 3.0) * assign89530_body23_e136480) + (assign89530_body23_e136469 * (-(((locals.var_chi_dn13 / 4.0) * assign89530_body23_e136478) + (assign89530_body23_e136473 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body23_e136485;
            locals.var_t0_dn0 = assign89530_body23_e136485_d_n0;
            locals.var_t0_dn2 = assign89530_body23_e136485_d_n2;
            locals.var_t0_dn4 = assign89530_body23_e136485_d_n4;
            locals.var_t0_dn5 = assign89530_body23_e136485_d_n5;
            locals.var_t0_dn6 = assign89530_body23_e136485_d_n6;
            locals.var_t0_dn7 = assign89530_body23_e136485_d_n7;
            locals.var_t0_dn8 = assign89530_body23_e136485_d_n8;
            locals.var_t0_dn9 = assign89530_body23_e136485_d_n9;
            locals.var_t0_dn10 = assign89530_body23_e136485_d_n10;
            locals.var_t0_dn13 = assign89530_body23_e136485_d_n13;
            let (assign89530_body24_e136517, assign89530_body24_e136517_d_n0, assign89530_body24_e136517_d_n2, assign89530_body24_e136517_d_n4, assign89530_body24_e136517_d_n5, assign89530_body24_e136517_d_n6, assign89530_body24_e136517_d_n7, assign89530_body24_e136517_d_n8, assign89530_body24_e136517_d_n9, assign89530_body24_e136517_d_n10, assign89530_body24_e136517_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body24_e136501: f64 = (locals.var_chi / 2.0);
        let assign89530_body24_e136505: f64 = (locals.var_chi / 3.0);
        let assign89530_body24_e136509: f64 = (locals.var_chi / 4.0);
        let assign89530_body24_e136510: f64 = (1.0 - assign89530_body24_e136509);
        let assign89530_body24_e136511: f64 = (assign89530_body24_e136505 * assign89530_body24_e136510);
        let assign89530_body24_e136512: f64 = (1.0 - assign89530_body24_e136511);
        let assign89530_body24_e136513: f64 = (assign89530_body24_e136501 * assign89530_body24_e136512);
        let assign89530_body24_e136514: f64 = (1.0 - assign89530_body24_e136513);
        let assign89530_body24_e136515: f64 = (locals.var_chi * assign89530_body24_e136514);
        (assign89530_body24_e136515, ((locals.var_chi_dn0 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn0 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn2 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn4 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn5 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn6 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn7 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn8 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn9 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn10 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign89530_body24_e136514) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign89530_body24_e136512) + (assign89530_body24_e136501 * (-(((locals.var_chi_dn13 / 3.0) * assign89530_body24_e136510) + (assign89530_body24_e136505 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body24_e136517;
            locals.var_t1_dn0 = assign89530_body24_e136517_d_n0;
            locals.var_t1_dn2 = assign89530_body24_e136517_d_n2;
            locals.var_t1_dn4 = assign89530_body24_e136517_d_n4;
            locals.var_t1_dn5 = assign89530_body24_e136517_d_n5;
            locals.var_t1_dn6 = assign89530_body24_e136517_d_n6;
            locals.var_t1_dn7 = assign89530_body24_e136517_d_n7;
            locals.var_t1_dn8 = assign89530_body24_e136517_d_n8;
            locals.var_t1_dn9 = assign89530_body24_e136517_d_n9;
            locals.var_t1_dn10 = assign89530_body24_e136517_d_n10;
            locals.var_t1_dn13 = assign89530_body24_e136517_d_n13;
            let (assign89530_body25_e136553, assign89530_body25_e136553_d_n0, assign89530_body25_e136553_d_n2, assign89530_body25_e136553_d_n4, assign89530_body25_e136553_d_n5, assign89530_body25_e136553_d_n6, assign89530_body25_e136553_d_n7, assign89530_body25_e136553_d_n8, assign89530_body25_e136553_d_n9, assign89530_body25_e136553_d_n10, assign89530_body25_e136553_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body25_e136531: f64 = (locals.var_chib * locals.var_chib);
        let assign89530_body25_e136533: f64 = (assign89530_body25_e136531 / 2.0);
        let assign89530_body25_e136537: f64 = (locals.var_chib / 3.0);
        let assign89530_body25_e136541: f64 = (locals.var_chib / 4.0);
        let assign89530_body25_e136545: f64 = (locals.var_chib / 5.0);
        let assign89530_body25_e136546: f64 = (1.0 - assign89530_body25_e136545);
        let assign89530_body25_e136547: f64 = (assign89530_body25_e136541 * assign89530_body25_e136546);
        let assign89530_body25_e136548: f64 = (1.0 - assign89530_body25_e136547);
        let assign89530_body25_e136549: f64 = (assign89530_body25_e136537 * assign89530_body25_e136548);
        let assign89530_body25_e136550: f64 = (1.0 - assign89530_body25_e136549);
        let assign89530_body25_e136551: f64 = (assign89530_body25_e136533 * assign89530_body25_e136550);
        (assign89530_body25_e136551, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn0 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn0 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn2 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn2 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn4 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn4 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn5 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn5 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn6 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn6 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn7 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn7 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn8 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn8 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn9 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn9 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn10 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn10 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign89530_body25_e136550) + (assign89530_body25_e136533 * (-(((locals.var_chib_dn13 / 3.0) * assign89530_body25_e136548) + (assign89530_body25_e136537 * (-(((locals.var_chib_dn13 / 4.0) * assign89530_body25_e136546) + (assign89530_body25_e136541 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign89530_body25_e136553;
            locals.var_t2_dn0 = assign89530_body25_e136553_d_n0;
            locals.var_t2_dn2 = assign89530_body25_e136553_d_n2;
            locals.var_t2_dn4 = assign89530_body25_e136553_d_n4;
            locals.var_t2_dn5 = assign89530_body25_e136553_d_n5;
            locals.var_t2_dn6 = assign89530_body25_e136553_d_n6;
            locals.var_t2_dn7 = assign89530_body25_e136553_d_n7;
            locals.var_t2_dn8 = assign89530_body25_e136553_d_n8;
            locals.var_t2_dn9 = assign89530_body25_e136553_d_n9;
            locals.var_t2_dn10 = assign89530_body25_e136553_d_n10;
            locals.var_t2_dn13 = assign89530_body25_e136553_d_n13;
            let (assign89530_body26_e136585, assign89530_body26_e136585_d_n0, assign89530_body26_e136585_d_n2, assign89530_body26_e136585_d_n4, assign89530_body26_e136585_d_n5, assign89530_body26_e136585_d_n6, assign89530_body26_e136585_d_n7, assign89530_body26_e136585_d_n8, assign89530_body26_e136585_d_n9, assign89530_body26_e136585_d_n10, assign89530_body26_e136585_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body26_e136569: f64 = (locals.var_chib / 2.0);
        let assign89530_body26_e136573: f64 = (locals.var_chib / 3.0);
        let assign89530_body26_e136577: f64 = (locals.var_chib / 4.0);
        let assign89530_body26_e136578: f64 = (1.0 - assign89530_body26_e136577);
        let assign89530_body26_e136579: f64 = (assign89530_body26_e136573 * assign89530_body26_e136578);
        let assign89530_body26_e136580: f64 = (1.0 - assign89530_body26_e136579);
        let assign89530_body26_e136581: f64 = (assign89530_body26_e136569 * assign89530_body26_e136580);
        let assign89530_body26_e136582: f64 = (1.0 - assign89530_body26_e136581);
        let assign89530_body26_e136583: f64 = (locals.var_chib * assign89530_body26_e136582);
        (assign89530_body26_e136583, ((locals.var_chib_dn0 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn0 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn2 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn4 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn5 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn6 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn7 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn8 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn9 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn10 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign89530_body26_e136582) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign89530_body26_e136580) + (assign89530_body26_e136569 * (-(((locals.var_chib_dn13 / 3.0) * assign89530_body26_e136578) + (assign89530_body26_e136573 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign89530_body26_e136585;
            locals.var_t3_dn0 = assign89530_body26_e136585_d_n0;
            locals.var_t3_dn2 = assign89530_body26_e136585_d_n2;
            locals.var_t3_dn4 = assign89530_body26_e136585_d_n4;
            locals.var_t3_dn5 = assign89530_body26_e136585_d_n5;
            locals.var_t3_dn6 = assign89530_body26_e136585_d_n6;
            locals.var_t3_dn7 = assign89530_body26_e136585_d_n7;
            locals.var_t3_dn8 = assign89530_body26_e136585_d_n8;
            locals.var_t3_dn9 = assign89530_body26_e136585_d_n9;
            locals.var_t3_dn10 = assign89530_body26_e136585_d_n10;
            locals.var_t3_dn13 = assign89530_body26_e136585_d_n13;
            let (assign89530_body27_e136601, assign89530_body27_e136601_d_n0, assign89530_body27_e136601_d_n2, assign89530_body27_e136601_d_n4, assign89530_body27_e136601_d_n5, assign89530_body27_e136601_d_n6, assign89530_body27_e136601_d_n7, assign89530_body27_e136601_d_n8, assign89530_body27_e136601_d_n9, assign89530_body27_e136601_d_n10, assign89530_body27_e136601_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) {
        let assign89530_body27_e136599: f64 = (locals.var_t0 - locals.var_t2);
        (assign89530_body27_e136599, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign89530_body27_e136601;
            locals.var_t4_dn0 = assign89530_body27_e136601_d_n0;
            locals.var_t4_dn2 = assign89530_body27_e136601_d_n2;
            locals.var_t4_dn4 = assign89530_body27_e136601_d_n4;
            locals.var_t4_dn5 = assign89530_body27_e136601_d_n5;
            locals.var_t4_dn6 = assign89530_body27_e136601_d_n6;
            locals.var_t4_dn7 = assign89530_body27_e136601_d_n7;
            locals.var_t4_dn8 = assign89530_body27_e136601_d_n8;
            locals.var_t4_dn9 = assign89530_body27_e136601_d_n9;
            locals.var_t4_dn10 = assign89530_body27_e136601_d_n10;
            locals.var_t4_dn13 = assign89530_body27_e136601_d_n13;
            let assign89530_body28_e136604: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2075 = assign89530_body28_e136604;
            let (assign89530_body29_e136621, assign89530_body29_e136621_d_n0, assign89530_body29_e136621_d_n2, assign89530_body29_e136621_d_n4, assign89530_body29_e136621_d_n5, assign89530_body29_e136621_d_n6, assign89530_body29_e136621_d_n7, assign89530_body29_e136621_d_n8, assign89530_body29_e136621_d_n9, assign89530_body29_e136621_d_n10, assign89530_body29_e136621_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) && (locals.var_guard2075 != 0.0)) {
        let assign89530_body29_e136619: f64 = (locals.var_t4).sqrt();
        (assign89530_body29_e136619, (locals.var_t4_dn0 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn2 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn4 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn5 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn6 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn7 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn8 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn9 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn10 / (2.0 * assign89530_body29_e136619)), (locals.var_t4_dn13 / (2.0 * assign89530_body29_e136619)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body29_e136621;
            locals.var_fb_dn0 = assign89530_body29_e136621_d_n0;
            locals.var_fb_dn2 = assign89530_body29_e136621_d_n2;
            locals.var_fb_dn4 = assign89530_body29_e136621_d_n4;
            locals.var_fb_dn5 = assign89530_body29_e136621_d_n5;
            locals.var_fb_dn6 = assign89530_body29_e136621_d_n6;
            locals.var_fb_dn7 = assign89530_body29_e136621_d_n7;
            locals.var_fb_dn8 = assign89530_body29_e136621_d_n8;
            locals.var_fb_dn9 = assign89530_body29_e136621_d_n9;
            locals.var_fb_dn10 = assign89530_body29_e136621_d_n10;
            locals.var_fb_dn13 = assign89530_body29_e136621_d_n13;
            let (assign89530_body30_e136647, assign89530_body30_e136647_d_n0, assign89530_body30_e136647_d_n2, assign89530_body30_e136647_d_n4, assign89530_body30_e136647_d_n5, assign89530_body30_e136647_d_n6, assign89530_body30_e136647_d_n7, assign89530_body30_e136647_d_n8, assign89530_body30_e136647_d_n9, assign89530_body30_e136647_d_n10, assign89530_body30_e136647_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) && (locals.var_guard2075 != 0.0)) {
        let assign89530_body30_e136637: f64 = (locals.var_beta * 0.5);
        let assign89530_body30_e136641: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign89530_body30_e136642: f64 = (locals.var_t1 - assign89530_body30_e136641);
        let assign89530_body30_e136643: f64 = (assign89530_body30_e136637 * assign89530_body30_e136642);
        let assign89530_body30_e136645: f64 = (assign89530_body30_e136643 / locals.var_fb);
        (assign89530_body30_e136645, ((((((locals.var_beta_dn0 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign89530_body30_e136642) + (assign89530_body30_e136637 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))) * locals.var_fb) - (assign89530_body30_e136643 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body30_e136647;
            locals.var_fb_dpss_dn0 = assign89530_body30_e136647_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body30_e136647_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body30_e136647_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body30_e136647_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body30_e136647_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body30_e136647_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body30_e136647_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body30_e136647_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body30_e136647_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body30_e136647_d_n13;
            let (assign89530_body32_e136683, assign89530_body32_e136683_d_n0, assign89530_body32_e136683_d_n2, assign89530_body32_e136683_d_n4, assign89530_body32_e136683_d_n5, assign89530_body32_e136683_d_n6, assign89530_body32_e136683_d_n7, assign89530_body32_e136683_d_n8, assign89530_body32_e136683_d_n9, assign89530_body32_e136683_d_n10, assign89530_body32_e136683_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) && (locals.var_guard2075 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body32_e136683;
            locals.var_fb_dn0 = assign89530_body32_e136683_d_n0;
            locals.var_fb_dn2 = assign89530_body32_e136683_d_n2;
            locals.var_fb_dn4 = assign89530_body32_e136683_d_n4;
            locals.var_fb_dn5 = assign89530_body32_e136683_d_n5;
            locals.var_fb_dn6 = assign89530_body32_e136683_d_n6;
            locals.var_fb_dn7 = assign89530_body32_e136683_d_n7;
            locals.var_fb_dn8 = assign89530_body32_e136683_d_n8;
            locals.var_fb_dn9 = assign89530_body32_e136683_d_n9;
            locals.var_fb_dn10 = assign89530_body32_e136683_d_n10;
            locals.var_fb_dn13 = assign89530_body32_e136683_d_n13;
            let (assign89530_body33_e136700, assign89530_body33_e136700_d_n0, assign89530_body33_e136700_d_n2, assign89530_body33_e136700_d_n4, assign89530_body33_e136700_d_n5, assign89530_body33_e136700_d_n6, assign89530_body33_e136700_d_n7, assign89530_body33_e136700_d_n8, assign89530_body33_e136700_d_n9, assign89530_body33_e136700_d_n10, assign89530_body33_e136700_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 != 0.0)) && (locals.var_guard2075 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body33_e136700;
            locals.var_fb_dpss_dn0 = assign89530_body33_e136700_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body33_e136700_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body33_e136700_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body33_e136700_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body33_e136700_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body33_e136700_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body33_e136700_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body33_e136700_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body33_e136700_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body33_e136700_d_n13;
            let (assign89530_body34_e136717, assign89530_body34_e136717_d_n0, assign89530_body34_e136717_d_n2, assign89530_body34_e136717_d_n4, assign89530_body34_e136717_d_n5, assign89530_body34_e136717_d_n6, assign89530_body34_e136717_d_n7, assign89530_body34_e136717_d_n8, assign89530_body34_e136717_d_n9, assign89530_body34_e136717_d_n10, assign89530_body34_e136717_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) {
        let assign89530_body34_e136714: f64 = (-locals.var_chi);
        let assign89530_body34_e136715: f64 = (assign89530_body34_e136714).exp();
        (assign89530_body34_e136715, (assign89530_body34_e136715 * (-locals.var_chi_dn0)), (assign89530_body34_e136715 * (-locals.var_chi_dn2)), (assign89530_body34_e136715 * (-locals.var_chi_dn4)), (assign89530_body34_e136715 * (-locals.var_chi_dn5)), (assign89530_body34_e136715 * (-locals.var_chi_dn6)), (assign89530_body34_e136715 * (-locals.var_chi_dn7)), (assign89530_body34_e136715 * (-locals.var_chi_dn8)), (assign89530_body34_e136715 * (-locals.var_chi_dn9)), (assign89530_body34_e136715 * (-locals.var_chi_dn10)), (assign89530_body34_e136715 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body34_e136717;
            locals.var_t0_dn0 = assign89530_body34_e136717_d_n0;
            locals.var_t0_dn2 = assign89530_body34_e136717_d_n2;
            locals.var_t0_dn4 = assign89530_body34_e136717_d_n4;
            locals.var_t0_dn5 = assign89530_body34_e136717_d_n5;
            locals.var_t0_dn6 = assign89530_body34_e136717_d_n6;
            locals.var_t0_dn7 = assign89530_body34_e136717_d_n7;
            locals.var_t0_dn8 = assign89530_body34_e136717_d_n8;
            locals.var_t0_dn9 = assign89530_body34_e136717_d_n9;
            locals.var_t0_dn10 = assign89530_body34_e136717_d_n10;
            locals.var_t0_dn13 = assign89530_body34_e136717_d_n13;
            let (assign89530_body35_e136734, assign89530_body35_e136734_d_n0, assign89530_body35_e136734_d_n2, assign89530_body35_e136734_d_n4, assign89530_body35_e136734_d_n5, assign89530_body35_e136734_d_n6, assign89530_body35_e136734_d_n7, assign89530_body35_e136734_d_n8, assign89530_body35_e136734_d_n9, assign89530_body35_e136734_d_n10, assign89530_body35_e136734_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) {
        let assign89530_body35_e136731: f64 = (-locals.var_chib);
        let assign89530_body35_e136732: f64 = (assign89530_body35_e136731).exp();
        (assign89530_body35_e136732, (assign89530_body35_e136732 * (-locals.var_chib_dn0)), (assign89530_body35_e136732 * (-locals.var_chib_dn2)), (assign89530_body35_e136732 * (-locals.var_chib_dn4)), (assign89530_body35_e136732 * (-locals.var_chib_dn5)), (assign89530_body35_e136732 * (-locals.var_chib_dn6)), (assign89530_body35_e136732 * (-locals.var_chib_dn7)), (assign89530_body35_e136732 * (-locals.var_chib_dn8)), (assign89530_body35_e136732 * (-locals.var_chib_dn9)), (assign89530_body35_e136732 * (-locals.var_chib_dn10)), (assign89530_body35_e136732 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body35_e136734;
            locals.var_t1_dn0 = assign89530_body35_e136734_d_n0;
            locals.var_t1_dn2 = assign89530_body35_e136734_d_n2;
            locals.var_t1_dn4 = assign89530_body35_e136734_d_n4;
            locals.var_t1_dn5 = assign89530_body35_e136734_d_n5;
            locals.var_t1_dn6 = assign89530_body35_e136734_d_n6;
            locals.var_t1_dn7 = assign89530_body35_e136734_d_n7;
            locals.var_t1_dn8 = assign89530_body35_e136734_d_n8;
            locals.var_t1_dn9 = assign89530_body35_e136734_d_n9;
            locals.var_t1_dn10 = assign89530_body35_e136734_d_n10;
            locals.var_t1_dn13 = assign89530_body35_e136734_d_n13;
            let (assign89530_body36_e136755, assign89530_body36_e136755_d_n0, assign89530_body36_e136755_d_n2, assign89530_body36_e136755_d_n4, assign89530_body36_e136755_d_n5, assign89530_body36_e136755_d_n6, assign89530_body36_e136755_d_n7, assign89530_body36_e136755_d_n8, assign89530_body36_e136755_d_n9, assign89530_body36_e136755_d_n10, assign89530_body36_e136755_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) {
        let assign89530_body36_e136749: f64 = (locals.var_chi - locals.var_chib);
        let assign89530_body36_e136752: f64 = (locals.var_t0 - locals.var_t1);
        let assign89530_body36_e136753: f64 = (assign89530_body36_e136749 + assign89530_body36_e136752);
        (assign89530_body36_e136753, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign89530_body36_e136755;
            locals.var_t4_dn0 = assign89530_body36_e136755_d_n0;
            locals.var_t4_dn2 = assign89530_body36_e136755_d_n2;
            locals.var_t4_dn4 = assign89530_body36_e136755_d_n4;
            locals.var_t4_dn5 = assign89530_body36_e136755_d_n5;
            locals.var_t4_dn6 = assign89530_body36_e136755_d_n6;
            locals.var_t4_dn7 = assign89530_body36_e136755_d_n7;
            locals.var_t4_dn8 = assign89530_body36_e136755_d_n8;
            locals.var_t4_dn9 = assign89530_body36_e136755_d_n9;
            locals.var_t4_dn10 = assign89530_body36_e136755_d_n10;
            locals.var_t4_dn13 = assign89530_body36_e136755_d_n13;
            let assign89530_body37_e136758: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2076 = assign89530_body37_e136758;
            let (assign89530_body38_e136776, assign89530_body38_e136776_d_n0, assign89530_body38_e136776_d_n2, assign89530_body38_e136776_d_n4, assign89530_body38_e136776_d_n5, assign89530_body38_e136776_d_n6, assign89530_body38_e136776_d_n7, assign89530_body38_e136776_d_n8, assign89530_body38_e136776_d_n9, assign89530_body38_e136776_d_n10, assign89530_body38_e136776_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89530_body38_e136774: f64 = (locals.var_t4).sqrt();
        (assign89530_body38_e136774, (locals.var_t4_dn0 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn2 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn4 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn5 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn6 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn7 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn8 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn9 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn10 / (2.0 * assign89530_body38_e136774)), (locals.var_t4_dn13 / (2.0 * assign89530_body38_e136774)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body38_e136776;
            locals.var_fb_dn0 = assign89530_body38_e136776_d_n0;
            locals.var_fb_dn2 = assign89530_body38_e136776_d_n2;
            locals.var_fb_dn4 = assign89530_body38_e136776_d_n4;
            locals.var_fb_dn5 = assign89530_body38_e136776_d_n5;
            locals.var_fb_dn6 = assign89530_body38_e136776_d_n6;
            locals.var_fb_dn7 = assign89530_body38_e136776_d_n7;
            locals.var_fb_dn8 = assign89530_body38_e136776_d_n8;
            locals.var_fb_dn9 = assign89530_body38_e136776_d_n9;
            locals.var_fb_dn10 = assign89530_body38_e136776_d_n10;
            locals.var_fb_dn13 = assign89530_body38_e136776_d_n13;
            let (assign89530_body39_e136807, assign89530_body39_e136807_d_n0, assign89530_body39_e136807_d_n2, assign89530_body39_e136807_d_n4, assign89530_body39_e136807_d_n5, assign89530_body39_e136807_d_n6, assign89530_body39_e136807_d_n7, assign89530_body39_e136807_d_n8, assign89530_body39_e136807_d_n9, assign89530_body39_e136807_d_n10, assign89530_body39_e136807_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) && (locals.var_guard2076 != 0.0)) {
        let assign89530_body39_e136793: f64 = (locals.var_beta * 0.5);
        let assign89530_body39_e136796: f64 = (1.0 - locals.var_t0);
        let assign89530_body39_e136800: f64 = (1.0 - locals.var_t1);
        let assign89530_body39_e136801: f64 = (locals.var_phi_b_dpss * assign89530_body39_e136800);
        let assign89530_body39_e136802: f64 = (assign89530_body39_e136796 - assign89530_body39_e136801);
        let assign89530_body39_e136803: f64 = (assign89530_body39_e136793 * assign89530_body39_e136802);
        let assign89530_body39_e136805: f64 = (assign89530_body39_e136803 / locals.var_fb);
        (assign89530_body39_e136805, ((((((locals.var_beta_dn0 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign89530_body39_e136802) + (assign89530_body39_e136793 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign89530_body39_e136800) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign89530_body39_e136803 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body39_e136807;
            locals.var_fb_dpss_dn0 = assign89530_body39_e136807_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body39_e136807_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body39_e136807_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body39_e136807_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body39_e136807_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body39_e136807_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body39_e136807_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body39_e136807_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body39_e136807_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body39_e136807_d_n13;
            let (assign89530_body41_e136845, assign89530_body41_e136845_d_n0, assign89530_body41_e136845_d_n2, assign89530_body41_e136845_d_n4, assign89530_body41_e136845_d_n5, assign89530_body41_e136845_d_n6, assign89530_body41_e136845_d_n7, assign89530_body41_e136845_d_n8, assign89530_body41_e136845_d_n9, assign89530_body41_e136845_d_n10, assign89530_body41_e136845_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) && (locals.var_guard2076 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign89530_body41_e136845;
            locals.var_fb_dn0 = assign89530_body41_e136845_d_n0;
            locals.var_fb_dn2 = assign89530_body41_e136845_d_n2;
            locals.var_fb_dn4 = assign89530_body41_e136845_d_n4;
            locals.var_fb_dn5 = assign89530_body41_e136845_d_n5;
            locals.var_fb_dn6 = assign89530_body41_e136845_d_n6;
            locals.var_fb_dn7 = assign89530_body41_e136845_d_n7;
            locals.var_fb_dn8 = assign89530_body41_e136845_d_n8;
            locals.var_fb_dn9 = assign89530_body41_e136845_d_n9;
            locals.var_fb_dn10 = assign89530_body41_e136845_d_n10;
            locals.var_fb_dn13 = assign89530_body41_e136845_d_n13;
            let (assign89530_body42_e136863, assign89530_body42_e136863_d_n0, assign89530_body42_e136863_d_n2, assign89530_body42_e136863_d_n4, assign89530_body42_e136863_d_n5, assign89530_body42_e136863_d_n6, assign89530_body42_e136863_d_n7, assign89530_body42_e136863_d_n8, assign89530_body42_e136863_d_n9, assign89530_body42_e136863_d_n10, assign89530_body42_e136863_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2073 == 0.0)) && (locals.var_guard2074 == 0.0)) && (locals.var_guard2076 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign89530_body42_e136863;
            locals.var_fb_dpss_dn0 = assign89530_body42_e136863_d_n0;
            locals.var_fb_dpss_dn2 = assign89530_body42_e136863_d_n2;
            locals.var_fb_dpss_dn4 = assign89530_body42_e136863_d_n4;
            locals.var_fb_dpss_dn5 = assign89530_body42_e136863_d_n5;
            locals.var_fb_dpss_dn6 = assign89530_body42_e136863_d_n6;
            locals.var_fb_dpss_dn7 = assign89530_body42_e136863_d_n7;
            locals.var_fb_dpss_dn8 = assign89530_body42_e136863_d_n8;
            locals.var_fb_dpss_dn9 = assign89530_body42_e136863_d_n9;
            locals.var_fb_dpss_dn10 = assign89530_body42_e136863_d_n10;
            locals.var_fb_dpss_dn13 = assign89530_body42_e136863_d_n13;
            let assign89530_body43_e136866: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2077 = assign89530_body43_e136866;
            let (assign89530_body45_e136890, assign89530_body45_e136890_d_n0, assign89530_body45_e136890_d_n2, assign89530_body45_e136890_d_n4, assign89530_body45_e136890_d_n5, assign89530_body45_e136890_d_n6, assign89530_body45_e136890_d_n7, assign89530_body45_e136890_d_n8, assign89530_body45_e136890_d_n9, assign89530_body45_e136890_d_n10, assign89530_body45_e136890_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89530_body45_e136890;
            locals.var_fs01_dn0 = assign89530_body45_e136890_d_n0;
            locals.var_fs01_dn2 = assign89530_body45_e136890_d_n2;
            locals.var_fs01_dn4 = assign89530_body45_e136890_d_n4;
            locals.var_fs01_dn5 = assign89530_body45_e136890_d_n5;
            locals.var_fs01_dn6 = assign89530_body45_e136890_d_n6;
            locals.var_fs01_dn7 = assign89530_body45_e136890_d_n7;
            locals.var_fs01_dn8 = assign89530_body45_e136890_d_n8;
            locals.var_fs01_dn9 = assign89530_body45_e136890_d_n9;
            locals.var_fs01_dn10 = assign89530_body45_e136890_d_n10;
            locals.var_fs01_dn13 = assign89530_body45_e136890_d_n13;
            let (assign89530_body46_e136901, assign89530_body46_e136901_d_n0, assign89530_body46_e136901_d_n2, assign89530_body46_e136901_d_n4, assign89530_body46_e136901_d_n5, assign89530_body46_e136901_d_n6, assign89530_body46_e136901_d_n7, assign89530_body46_e136901_d_n8, assign89530_body46_e136901_d_n9, assign89530_body46_e136901_d_n10, assign89530_body46_e136901_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89530_body46_e136901;
            locals.var_fs01_dps0_dn0 = assign89530_body46_e136901_d_n0;
            locals.var_fs01_dps0_dn2 = assign89530_body46_e136901_d_n2;
            locals.var_fs01_dps0_dn4 = assign89530_body46_e136901_d_n4;
            locals.var_fs01_dps0_dn5 = assign89530_body46_e136901_d_n5;
            locals.var_fs01_dps0_dn6 = assign89530_body46_e136901_d_n6;
            locals.var_fs01_dps0_dn7 = assign89530_body46_e136901_d_n7;
            locals.var_fs01_dps0_dn8 = assign89530_body46_e136901_d_n8;
            locals.var_fs01_dps0_dn9 = assign89530_body46_e136901_d_n9;
            locals.var_fs01_dps0_dn10 = assign89530_body46_e136901_d_n10;
            locals.var_fs01_dps0_dn13 = assign89530_body46_e136901_d_n13;
            let (assign89530_body47_e136913, assign89530_body47_e136913_d_n0, assign89530_body47_e136913_d_n2, assign89530_body47_e136913_d_n4, assign89530_body47_e136913_d_n5, assign89530_body47_e136913_d_n6, assign89530_body47_e136913_d_n7, assign89530_body47_e136913_d_n8, assign89530_body47_e136913_d_n9, assign89530_body47_e136913_d_n10, assign89530_body47_e136913_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        let assign89530_body47_e136911: f64 = (-locals.var_fb);
        (assign89530_body47_e136911, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89530_body47_e136913;
            locals.var_fs02_dn0 = assign89530_body47_e136913_d_n0;
            locals.var_fs02_dn2 = assign89530_body47_e136913_d_n2;
            locals.var_fs02_dn4 = assign89530_body47_e136913_d_n4;
            locals.var_fs02_dn5 = assign89530_body47_e136913_d_n5;
            locals.var_fs02_dn6 = assign89530_body47_e136913_d_n6;
            locals.var_fs02_dn7 = assign89530_body47_e136913_d_n7;
            locals.var_fs02_dn8 = assign89530_body47_e136913_d_n8;
            locals.var_fs02_dn9 = assign89530_body47_e136913_d_n9;
            locals.var_fs02_dn10 = assign89530_body47_e136913_d_n10;
            locals.var_fs02_dn13 = assign89530_body47_e136913_d_n13;
            let (assign89530_body48_e136925, assign89530_body48_e136925_d_n0, assign89530_body48_e136925_d_n2, assign89530_body48_e136925_d_n4, assign89530_body48_e136925_d_n5, assign89530_body48_e136925_d_n6, assign89530_body48_e136925_d_n7, assign89530_body48_e136925_d_n8, assign89530_body48_e136925_d_n9, assign89530_body48_e136925_d_n10, assign89530_body48_e136925_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 != 0.0)) {
        let assign89530_body48_e136923: f64 = (-locals.var_fb_dpss);
        (assign89530_body48_e136923, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89530_body48_e136925;
            locals.var_fs02_dps0_dn0 = assign89530_body48_e136925_d_n0;
            locals.var_fs02_dps0_dn2 = assign89530_body48_e136925_d_n2;
            locals.var_fs02_dps0_dn4 = assign89530_body48_e136925_d_n4;
            locals.var_fs02_dps0_dn5 = assign89530_body48_e136925_d_n5;
            locals.var_fs02_dps0_dn6 = assign89530_body48_e136925_d_n6;
            locals.var_fs02_dps0_dn7 = assign89530_body48_e136925_d_n7;
            locals.var_fs02_dps0_dn8 = assign89530_body48_e136925_d_n8;
            locals.var_fs02_dps0_dn9 = assign89530_body48_e136925_d_n9;
            locals.var_fs02_dps0_dn10 = assign89530_body48_e136925_d_n10;
            locals.var_fs02_dps0_dn13 = assign89530_body48_e136925_d_n13;
            let assign89530_body49_e136928: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2078 = assign89530_body49_e136928;
            let assign89530_body50_e136931: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2079 = assign89530_body50_e136931;
            let (assign89530_body51_e136969, assign89530_body51_e136969_d_n0, assign89530_body51_e136969_d_n2, assign89530_body51_e136969_d_n4, assign89530_body51_e136969_d_n5, assign89530_body51_e136969_d_n6, assign89530_body51_e136969_d_n7, assign89530_body51_e136969_d_n8, assign89530_body51_e136969_d_n9, assign89530_body51_e136969_d_n10, assign89530_body51_e136969_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89530_body51_e136947: f64 = (locals.var_chi * locals.var_chi);
        let assign89530_body51_e136949: f64 = (assign89530_body51_e136947 / 2.0);
        let assign89530_body51_e136953: f64 = (locals.var_chi / 3.0);
        let assign89530_body51_e136957: f64 = (locals.var_chi / 4.0);
        let assign89530_body51_e136961: f64 = (locals.var_chi / 5.0);
        let assign89530_body51_e136962: f64 = (1.0 + assign89530_body51_e136961);
        let assign89530_body51_e136963: f64 = (assign89530_body51_e136957 * assign89530_body51_e136962);
        let assign89530_body51_e136964: f64 = (1.0 + assign89530_body51_e136963);
        let assign89530_body51_e136965: f64 = (assign89530_body51_e136953 * assign89530_body51_e136964);
        let assign89530_body51_e136966: f64 = (1.0 + assign89530_body51_e136965);
        let assign89530_body51_e136967: f64 = (assign89530_body51_e136949 * assign89530_body51_e136966);
        (assign89530_body51_e136967, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn0 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn0 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn2 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn2 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn4 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn4 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn5 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn5 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn6 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn6 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn7 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn7 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn8 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn8 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn9 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn9 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn10 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn10 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign89530_body51_e136966) + (assign89530_body51_e136949 * (((locals.var_chi_dn13 / 3.0) * assign89530_body51_e136964) + (assign89530_body51_e136953 * (((locals.var_chi_dn13 / 4.0) * assign89530_body51_e136962) + (assign89530_body51_e136957 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89530_body51_e136969;
            locals.var_t0_dn0 = assign89530_body51_e136969_d_n0;
            locals.var_t0_dn2 = assign89530_body51_e136969_d_n2;
            locals.var_t0_dn4 = assign89530_body51_e136969_d_n4;
            locals.var_t0_dn5 = assign89530_body51_e136969_d_n5;
            locals.var_t0_dn6 = assign89530_body51_e136969_d_n6;
            locals.var_t0_dn7 = assign89530_body51_e136969_d_n7;
            locals.var_t0_dn8 = assign89530_body51_e136969_d_n8;
            locals.var_t0_dn9 = assign89530_body51_e136969_d_n9;
            locals.var_t0_dn10 = assign89530_body51_e136969_d_n10;
            locals.var_t0_dn13 = assign89530_body51_e136969_d_n13;
            let (assign89530_body52_e137003, assign89530_body52_e137003_d_n0, assign89530_body52_e137003_d_n2, assign89530_body52_e137003_d_n4, assign89530_body52_e137003_d_n5, assign89530_body52_e137003_d_n6, assign89530_body52_e137003_d_n7, assign89530_body52_e137003_d_n8, assign89530_body52_e137003_d_n9, assign89530_body52_e137003_d_n10, assign89530_body52_e137003_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89530_body52_e136987: f64 = (locals.var_chi / 2.0);
        let assign89530_body52_e136991: f64 = (locals.var_chi / 3.0);
        let assign89530_body52_e136995: f64 = (locals.var_chi / 4.0);
        let assign89530_body52_e136996: f64 = (1.0 + assign89530_body52_e136995);
        let assign89530_body52_e136997: f64 = (assign89530_body52_e136991 * assign89530_body52_e136996);
        let assign89530_body52_e136998: f64 = (1.0 + assign89530_body52_e136997);
        let assign89530_body52_e136999: f64 = (assign89530_body52_e136987 * assign89530_body52_e136998);
        let assign89530_body52_e137000: f64 = (1.0 + assign89530_body52_e136999);
        let assign89530_body52_e137001: f64 = (locals.var_chi * assign89530_body52_e137000);
        (assign89530_body52_e137001, ((locals.var_chi_dn0 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn0 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn2 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn4 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn5 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn6 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn7 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn8 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn9 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn10 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign89530_body52_e137000) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign89530_body52_e136998) + (assign89530_body52_e136987 * (((locals.var_chi_dn13 / 3.0) * assign89530_body52_e136996) + (assign89530_body52_e136991 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body52_e137003;
            locals.var_t1_dn0 = assign89530_body52_e137003_d_n0;
            locals.var_t1_dn2 = assign89530_body52_e137003_d_n2;
            locals.var_t1_dn4 = assign89530_body52_e137003_d_n4;
            locals.var_t1_dn5 = assign89530_body52_e137003_d_n5;
            locals.var_t1_dn6 = assign89530_body52_e137003_d_n6;
            locals.var_t1_dn7 = assign89530_body52_e137003_d_n7;
            locals.var_t1_dn8 = assign89530_body52_e137003_d_n8;
            locals.var_t1_dn9 = assign89530_body52_e137003_d_n9;
            locals.var_t1_dn10 = assign89530_body52_e137003_d_n10;
            locals.var_t1_dn13 = assign89530_body52_e137003_d_n13;
            let (assign89530_body53_e137021, assign89530_body53_e137021_d_n0, assign89530_body53_e137021_d_n2, assign89530_body53_e137021_d_n4, assign89530_body53_e137021_d_n5, assign89530_body53_e137021_d_n6, assign89530_body53_e137021_d_n7, assign89530_body53_e137021_d_n8, assign89530_body53_e137021_d_n9, assign89530_body53_e137021_d_n10, assign89530_body53_e137021_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89530_body53_e137019: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign89530_body53_e137019, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89530_body53_e137021;
            locals.var_fs01_dn0 = assign89530_body53_e137021_d_n0;
            locals.var_fs01_dn2 = assign89530_body53_e137021_d_n2;
            locals.var_fs01_dn4 = assign89530_body53_e137021_d_n4;
            locals.var_fs01_dn5 = assign89530_body53_e137021_d_n5;
            locals.var_fs01_dn6 = assign89530_body53_e137021_d_n6;
            locals.var_fs01_dn7 = assign89530_body53_e137021_d_n7;
            locals.var_fs01_dn8 = assign89530_body53_e137021_d_n8;
            locals.var_fs01_dn9 = assign89530_body53_e137021_d_n9;
            locals.var_fs01_dn10 = assign89530_body53_e137021_d_n10;
            locals.var_fs01_dn13 = assign89530_body53_e137021_d_n13;
            let (assign89530_body54_e137041, assign89530_body54_e137041_d_n0, assign89530_body54_e137041_d_n2, assign89530_body54_e137041_d_n4, assign89530_body54_e137041_d_n5, assign89530_body54_e137041_d_n6, assign89530_body54_e137041_d_n7, assign89530_body54_e137041_d_n8, assign89530_body54_e137041_d_n9, assign89530_body54_e137041_d_n10, assign89530_body54_e137041_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 != 0.0)) {
        let assign89530_body54_e137037: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign89530_body54_e137039: f64 = (assign89530_body54_e137037 * locals.var_beta);
        (assign89530_body54_e137039, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign89530_body54_e137037 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89530_body54_e137041;
            locals.var_fs01_dps0_dn0 = assign89530_body54_e137041_d_n0;
            locals.var_fs01_dps0_dn2 = assign89530_body54_e137041_d_n2;
            locals.var_fs01_dps0_dn4 = assign89530_body54_e137041_d_n4;
            locals.var_fs01_dps0_dn5 = assign89530_body54_e137041_d_n5;
            locals.var_fs01_dps0_dn6 = assign89530_body54_e137041_d_n6;
            locals.var_fs01_dps0_dn7 = assign89530_body54_e137041_d_n7;
            locals.var_fs01_dps0_dn8 = assign89530_body54_e137041_d_n8;
            locals.var_fs01_dps0_dn9 = assign89530_body54_e137041_d_n9;
            locals.var_fs01_dps0_dn10 = assign89530_body54_e137041_d_n10;
            locals.var_fs01_dps0_dn13 = assign89530_body54_e137041_d_n13;
            let (assign89530_body55_e137059, assign89530_body55_e137059_d_n0, assign89530_body55_e137059_d_n2, assign89530_body55_e137059_d_n4, assign89530_body55_e137059_d_n5, assign89530_body55_e137059_d_n6, assign89530_body55_e137059_d_n7, assign89530_body55_e137059_d_n8, assign89530_body55_e137059_d_n9, assign89530_body55_e137059_d_n10, assign89530_body55_e137059_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 == 0.0)) {
        let assign89530_body55_e137057: f64 = (locals.var_chi).exp();
        (assign89530_body55_e137057, (assign89530_body55_e137057 * locals.var_chi_dn0), (assign89530_body55_e137057 * locals.var_chi_dn2), (assign89530_body55_e137057 * locals.var_chi_dn4), (assign89530_body55_e137057 * locals.var_chi_dn5), (assign89530_body55_e137057 * locals.var_chi_dn6), (assign89530_body55_e137057 * locals.var_chi_dn7), (assign89530_body55_e137057 * locals.var_chi_dn8), (assign89530_body55_e137057 * locals.var_chi_dn9), (assign89530_body55_e137057 * locals.var_chi_dn10), (assign89530_body55_e137057 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign89530_body55_e137059;
            locals.var_exp_chi_dn0 = assign89530_body55_e137059_d_n0;
            locals.var_exp_chi_dn2 = assign89530_body55_e137059_d_n2;
            locals.var_exp_chi_dn4 = assign89530_body55_e137059_d_n4;
            locals.var_exp_chi_dn5 = assign89530_body55_e137059_d_n5;
            locals.var_exp_chi_dn6 = assign89530_body55_e137059_d_n6;
            locals.var_exp_chi_dn7 = assign89530_body55_e137059_d_n7;
            locals.var_exp_chi_dn8 = assign89530_body55_e137059_d_n8;
            locals.var_exp_chi_dn9 = assign89530_body55_e137059_d_n9;
            locals.var_exp_chi_dn10 = assign89530_body55_e137059_d_n10;
            locals.var_exp_chi_dn13 = assign89530_body55_e137059_d_n13;
            let (assign89530_body56_e137078, assign89530_body56_e137078_d_n0, assign89530_body56_e137078_d_n2, assign89530_body56_e137078_d_n4, assign89530_body56_e137078_d_n5, assign89530_body56_e137078_d_n6, assign89530_body56_e137078_d_n7, assign89530_body56_e137078_d_n8, assign89530_body56_e137078_d_n9, assign89530_body56_e137078_d_n10, assign89530_body56_e137078_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 == 0.0)) {
        let assign89530_body56_e137076: f64 = (locals.var_exp_chi - 1.0);
        (assign89530_body56_e137076, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89530_body56_e137078;
            locals.var_t1_dn0 = assign89530_body56_e137078_d_n0;
            locals.var_t1_dn2 = assign89530_body56_e137078_d_n2;
            locals.var_t1_dn4 = assign89530_body56_e137078_d_n4;
            locals.var_t1_dn5 = assign89530_body56_e137078_d_n5;
            locals.var_t1_dn6 = assign89530_body56_e137078_d_n6;
            locals.var_t1_dn7 = assign89530_body56_e137078_d_n7;
            locals.var_t1_dn8 = assign89530_body56_e137078_d_n8;
            locals.var_t1_dn9 = assign89530_body56_e137078_d_n9;
            locals.var_t1_dn10 = assign89530_body56_e137078_d_n10;
            locals.var_t1_dn13 = assign89530_body56_e137078_d_n13;
            let (assign89530_body57_e137099, assign89530_body57_e137099_d_n0, assign89530_body57_e137099_d_n2, assign89530_body57_e137099_d_n4, assign89530_body57_e137099_d_n5, assign89530_body57_e137099_d_n6, assign89530_body57_e137099_d_n7, assign89530_body57_e137099_d_n8, assign89530_body57_e137099_d_n9, assign89530_body57_e137099_d_n10, assign89530_body57_e137099_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 == 0.0)) {
        let assign89530_body57_e137096: f64 = (locals.var_t1 - locals.var_chi);
        let assign89530_body57_e137097: f64 = (locals.var_cfs1 * assign89530_body57_e137096);
        (assign89530_body57_e137097, ((locals.var_cfs1_dn0 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign89530_body57_e137096) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89530_body57_e137099;
            locals.var_fs01_dn0 = assign89530_body57_e137099_d_n0;
            locals.var_fs01_dn2 = assign89530_body57_e137099_d_n2;
            locals.var_fs01_dn4 = assign89530_body57_e137099_d_n4;
            locals.var_fs01_dn5 = assign89530_body57_e137099_d_n5;
            locals.var_fs01_dn6 = assign89530_body57_e137099_d_n6;
            locals.var_fs01_dn7 = assign89530_body57_e137099_d_n7;
            locals.var_fs01_dn8 = assign89530_body57_e137099_d_n8;
            locals.var_fs01_dn9 = assign89530_body57_e137099_d_n9;
            locals.var_fs01_dn10 = assign89530_body57_e137099_d_n10;
            locals.var_fs01_dn13 = assign89530_body57_e137099_d_n13;
            let (assign89530_body58_e137120, assign89530_body58_e137120_d_n0, assign89530_body58_e137120_d_n2, assign89530_body58_e137120_d_n4, assign89530_body58_e137120_d_n5, assign89530_body58_e137120_d_n6, assign89530_body58_e137120_d_n7, assign89530_body58_e137120_d_n8, assign89530_body58_e137120_d_n9, assign89530_body58_e137120_d_n10, assign89530_body58_e137120_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 != 0.0)) && (locals.var_guard2079 == 0.0)) {
        let assign89530_body58_e137116: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign89530_body58_e137118: f64 = (assign89530_body58_e137116 * locals.var_t1);
        (assign89530_body58_e137118, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign89530_body58_e137116 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89530_body58_e137120;
            locals.var_fs01_dps0_dn0 = assign89530_body58_e137120_d_n0;
            locals.var_fs01_dps0_dn2 = assign89530_body58_e137120_d_n2;
            locals.var_fs01_dps0_dn4 = assign89530_body58_e137120_d_n4;
            locals.var_fs01_dps0_dn5 = assign89530_body58_e137120_d_n5;
            locals.var_fs01_dps0_dn6 = assign89530_body58_e137120_d_n6;
            locals.var_fs01_dps0_dn7 = assign89530_body58_e137120_d_n7;
            locals.var_fs01_dps0_dn8 = assign89530_body58_e137120_d_n8;
            locals.var_fs01_dps0_dn9 = assign89530_body58_e137120_d_n9;
            locals.var_fs01_dps0_dn10 = assign89530_body58_e137120_d_n10;
            locals.var_fs01_dps0_dn13 = assign89530_body58_e137120_d_n13;
            let (assign89530_body60_e137155, assign89530_body60_e137155_d_n0, assign89530_body60_e137155_d_n2, assign89530_body60_e137155_d_n4, assign89530_body60_e137155_d_n5, assign89530_body60_e137155_d_n6, assign89530_body60_e137155_d_n7, assign89530_body60_e137155_d_n8, assign89530_body60_e137155_d_n9, assign89530_body60_e137155_d_n10, assign89530_body60_e137155_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 == 0.0)) {
        let assign89530_body60_e137152: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign89530_body60_e137153: f64 = (assign89530_body60_e137152).exp();
        (assign89530_body60_e137153, (assign89530_body60_e137153 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign89530_body60_e137153 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign89530_body60_e137153 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign89530_body60_e137153 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign89530_body60_e137153 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign89530_body60_e137153 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign89530_body60_e137153 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign89530_body60_e137153 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign89530_body60_e137153 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign89530_body60_e137153 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign89530_body60_e137155;
            locals.var_exp_bps0_dn0 = assign89530_body60_e137155_d_n0;
            locals.var_exp_bps0_dn2 = assign89530_body60_e137155_d_n2;
            locals.var_exp_bps0_dn4 = assign89530_body60_e137155_d_n4;
            locals.var_exp_bps0_dn5 = assign89530_body60_e137155_d_n5;
            locals.var_exp_bps0_dn6 = assign89530_body60_e137155_d_n6;
            locals.var_exp_bps0_dn7 = assign89530_body60_e137155_d_n7;
            locals.var_exp_bps0_dn8 = assign89530_body60_e137155_d_n8;
            locals.var_exp_bps0_dn9 = assign89530_body60_e137155_d_n9;
            locals.var_exp_bps0_dn10 = assign89530_body60_e137155_d_n10;
            locals.var_exp_bps0_dn13 = assign89530_body60_e137155_d_n13;
            let (assign89530_body61_e137178, assign89530_body61_e137178_d_n0, assign89530_body61_e137178_d_n2, assign89530_body61_e137178_d_n4, assign89530_body61_e137178_d_n5, assign89530_body61_e137178_d_n6, assign89530_body61_e137178_d_n7, assign89530_body61_e137178_d_n8, assign89530_body61_e137178_d_n9, assign89530_body61_e137178_d_n10, assign89530_body61_e137178_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 == 0.0)) {
        let assign89530_body61_e137173: f64 = (locals.var_chi + 1.0);
        let assign89530_body61_e137174: f64 = (locals.var_exp_bvbs * assign89530_body61_e137173);
        let assign89530_body61_e137175: f64 = (locals.var_exp_bps0 - assign89530_body61_e137174);
        let assign89530_body61_e137176: f64 = (locals.var_cnst1over * assign89530_body61_e137175);
        (assign89530_body61_e137176, ((locals.var_cnst1over_dn0 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign89530_body61_e137175) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign89530_body61_e137173) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89530_body61_e137178;
            locals.var_fs01_dn0 = assign89530_body61_e137178_d_n0;
            locals.var_fs01_dn2 = assign89530_body61_e137178_d_n2;
            locals.var_fs01_dn4 = assign89530_body61_e137178_d_n4;
            locals.var_fs01_dn5 = assign89530_body61_e137178_d_n5;
            locals.var_fs01_dn6 = assign89530_body61_e137178_d_n6;
            locals.var_fs01_dn7 = assign89530_body61_e137178_d_n7;
            locals.var_fs01_dn8 = assign89530_body61_e137178_d_n8;
            locals.var_fs01_dn9 = assign89530_body61_e137178_d_n9;
            locals.var_fs01_dn10 = assign89530_body61_e137178_d_n10;
            locals.var_fs01_dn13 = assign89530_body61_e137178_d_n13;
            let (assign89530_body62_e137199, assign89530_body62_e137199_d_n0, assign89530_body62_e137199_d_n2, assign89530_body62_e137199_d_n4, assign89530_body62_e137199_d_n5, assign89530_body62_e137199_d_n6, assign89530_body62_e137199_d_n7, assign89530_body62_e137199_d_n8, assign89530_body62_e137199_d_n9, assign89530_body62_e137199_d_n10, assign89530_body62_e137199_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2078 == 0.0)) {
        let assign89530_body62_e137193: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign89530_body62_e137196: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign89530_body62_e137197: f64 = (assign89530_body62_e137193 * assign89530_body62_e137196);
        (assign89530_body62_e137197, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign89530_body62_e137196) + (assign89530_body62_e137193 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89530_body62_e137199;
            locals.var_fs01_dps0_dn0 = assign89530_body62_e137199_d_n0;
            locals.var_fs01_dps0_dn2 = assign89530_body62_e137199_d_n2;
            locals.var_fs01_dps0_dn4 = assign89530_body62_e137199_d_n4;
            locals.var_fs01_dps0_dn5 = assign89530_body62_e137199_d_n5;
            locals.var_fs01_dps0_dn6 = assign89530_body62_e137199_d_n6;
            locals.var_fs01_dps0_dn7 = assign89530_body62_e137199_d_n7;
            locals.var_fs01_dps0_dn8 = assign89530_body62_e137199_d_n8;
            locals.var_fs01_dps0_dn9 = assign89530_body62_e137199_d_n9;
            locals.var_fs01_dps0_dn10 = assign89530_body62_e137199_d_n10;
            locals.var_fs01_dps0_dn13 = assign89530_body62_e137199_d_n13;
            let assign89530_body63_e137202: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2080 = assign89530_body63_e137202;
            let (assign89530_body64_e137221, assign89530_body64_e137221_d_n0, assign89530_body64_e137221_d_n2, assign89530_body64_e137221_d_n4, assign89530_body64_e137221_d_n5, assign89530_body64_e137221_d_n6, assign89530_body64_e137221_d_n7, assign89530_body64_e137221_d_n8, assign89530_body64_e137221_d_n9, assign89530_body64_e137221_d_n10, assign89530_body64_e137221_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2080 != 0.0)) {
        let assign89530_body64_e137216: f64 = (locals.var_fb * locals.var_fb);
        let assign89530_body64_e137218: f64 = (assign89530_body64_e137216 + locals.var_fs01);
        let assign89530_body64_e137219: f64 = (assign89530_body64_e137218).sqrt();
        (assign89530_body64_e137219, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign89530_body64_e137219)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign89530_body64_e137219)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89530_body64_e137221;
            locals.var_fs02_dn0 = assign89530_body64_e137221_d_n0;
            locals.var_fs02_dn2 = assign89530_body64_e137221_d_n2;
            locals.var_fs02_dn4 = assign89530_body64_e137221_d_n4;
            locals.var_fs02_dn5 = assign89530_body64_e137221_d_n5;
            locals.var_fs02_dn6 = assign89530_body64_e137221_d_n6;
            locals.var_fs02_dn7 = assign89530_body64_e137221_d_n7;
            locals.var_fs02_dn8 = assign89530_body64_e137221_d_n8;
            locals.var_fs02_dn9 = assign89530_body64_e137221_d_n9;
            locals.var_fs02_dn10 = assign89530_body64_e137221_d_n10;
            locals.var_fs02_dn13 = assign89530_body64_e137221_d_n13;
            let (assign89530_body65_e137245, assign89530_body65_e137245_d_n0, assign89530_body65_e137245_d_n2, assign89530_body65_e137245_d_n4, assign89530_body65_e137245_d_n5, assign89530_body65_e137245_d_n6, assign89530_body65_e137245_d_n7, assign89530_body65_e137245_d_n8, assign89530_body65_e137245_d_n9, assign89530_body65_e137245_d_n10, assign89530_body65_e137245_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2080 != 0.0)) {
        let assign89530_body65_e137236: f64 = (2.0 * locals.var_fb_dpss);
        let assign89530_body65_e137238: f64 = (assign89530_body65_e137236 * locals.var_fb);
        let assign89530_body65_e137240: f64 = (assign89530_body65_e137238 + locals.var_fs01_dps0);
        let assign89530_body65_e137241: f64 = (0.5 * assign89530_body65_e137240);
        let assign89530_body65_e137243: f64 = (assign89530_body65_e137241 / locals.var_fs02);
        (assign89530_body65_e137243, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn13) * locals.var_fb) + (assign89530_body65_e137236 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign89530_body65_e137241 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89530_body65_e137245;
            locals.var_fs02_dps0_dn0 = assign89530_body65_e137245_d_n0;
            locals.var_fs02_dps0_dn2 = assign89530_body65_e137245_d_n2;
            locals.var_fs02_dps0_dn4 = assign89530_body65_e137245_d_n4;
            locals.var_fs02_dps0_dn5 = assign89530_body65_e137245_d_n5;
            locals.var_fs02_dps0_dn6 = assign89530_body65_e137245_d_n6;
            locals.var_fs02_dps0_dn7 = assign89530_body65_e137245_d_n7;
            locals.var_fs02_dps0_dn8 = assign89530_body65_e137245_d_n8;
            locals.var_fs02_dps0_dn9 = assign89530_body65_e137245_d_n9;
            locals.var_fs02_dps0_dn10 = assign89530_body65_e137245_d_n10;
            locals.var_fs02_dps0_dn13 = assign89530_body65_e137245_d_n13;
            let (assign89530_body67_e137277, assign89530_body67_e137277_d_n0, assign89530_body67_e137277_d_n2, assign89530_body67_e137277_d_n4, assign89530_body67_e137277_d_n5, assign89530_body67_e137277_d_n6, assign89530_body67_e137277_d_n7, assign89530_body67_e137277_d_n8, assign89530_body67_e137277_d_n9, assign89530_body67_e137277_d_n10, assign89530_body67_e137277_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2080 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89530_body67_e137277;
            locals.var_fs02_dn0 = assign89530_body67_e137277_d_n0;
            locals.var_fs02_dn2 = assign89530_body67_e137277_d_n2;
            locals.var_fs02_dn4 = assign89530_body67_e137277_d_n4;
            locals.var_fs02_dn5 = assign89530_body67_e137277_d_n5;
            locals.var_fs02_dn6 = assign89530_body67_e137277_d_n6;
            locals.var_fs02_dn7 = assign89530_body67_e137277_d_n7;
            locals.var_fs02_dn8 = assign89530_body67_e137277_d_n8;
            locals.var_fs02_dn9 = assign89530_body67_e137277_d_n9;
            locals.var_fs02_dn10 = assign89530_body67_e137277_d_n10;
            locals.var_fs02_dn13 = assign89530_body67_e137277_d_n13;
            let (assign89530_body68_e137292, assign89530_body68_e137292_d_n0, assign89530_body68_e137292_d_n2, assign89530_body68_e137292_d_n4, assign89530_body68_e137292_d_n5, assign89530_body68_e137292_d_n6, assign89530_body68_e137292_d_n7, assign89530_body68_e137292_d_n8, assign89530_body68_e137292_d_n9, assign89530_body68_e137292_d_n10, assign89530_body68_e137292_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2077 == 0.0)) && (locals.var_guard2080 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89530_body68_e137292;
            locals.var_fs02_dps0_dn0 = assign89530_body68_e137292_d_n0;
            locals.var_fs02_dps0_dn2 = assign89530_body68_e137292_d_n2;
            locals.var_fs02_dps0_dn4 = assign89530_body68_e137292_d_n4;
            locals.var_fs02_dps0_dn5 = assign89530_body68_e137292_d_n5;
            locals.var_fs02_dps0_dn6 = assign89530_body68_e137292_d_n6;
            locals.var_fs02_dps0_dn7 = assign89530_body68_e137292_d_n7;
            locals.var_fs02_dps0_dn8 = assign89530_body68_e137292_d_n8;
            locals.var_fs02_dps0_dn9 = assign89530_body68_e137292_d_n9;
            locals.var_fs02_dps0_dn10 = assign89530_body68_e137292_d_n10;
            locals.var_fs02_dps0_dn13 = assign89530_body68_e137292_d_n13;
            let (assign89530_body69_e137308, assign89530_body69_e137308_d_n0, assign89530_body69_e137308_d_n2, assign89530_body69_e137308_d_n4, assign89530_body69_e137308_d_n5, assign89530_body69_e137308_d_n6, assign89530_body69_e137308_d_n7, assign89530_body69_e137308_d_n8, assign89530_body69_e137308_d_n9, assign89530_body69_e137308_d_n10, assign89530_body69_e137308_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body69_e137300: f64 = (-locals.var_vgpld);
        let assign89530_body69_e137302: f64 = (assign89530_body69_e137300 + locals.var_ps0ld);
        let assign89530_body69_e137305: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign89530_body69_e137306: f64 = (assign89530_body69_e137302 + assign89530_body69_e137305);
        (assign89530_body69_e137306, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign89530_body69_e137308;
            locals.var_fs0_dn0 = assign89530_body69_e137308_d_n0;
            locals.var_fs0_dn2 = assign89530_body69_e137308_d_n2;
            locals.var_fs0_dn4 = assign89530_body69_e137308_d_n4;
            locals.var_fs0_dn5 = assign89530_body69_e137308_d_n5;
            locals.var_fs0_dn6 = assign89530_body69_e137308_d_n6;
            locals.var_fs0_dn7 = assign89530_body69_e137308_d_n7;
            locals.var_fs0_dn8 = assign89530_body69_e137308_d_n8;
            locals.var_fs0_dn9 = assign89530_body69_e137308_d_n9;
            locals.var_fs0_dn10 = assign89530_body69_e137308_d_n10;
            locals.var_fs0_dn13 = assign89530_body69_e137308_d_n13;
            let (assign89530_body70_e137321, assign89530_body70_e137321_d_n0, assign89530_body70_e137321_d_n2, assign89530_body70_e137321_d_n4, assign89530_body70_e137321_d_n5, assign89530_body70_e137321_d_n6, assign89530_body70_e137321_d_n7, assign89530_body70_e137321_d_n8, assign89530_body70_e137321_d_n9, assign89530_body70_e137321_d_n10, assign89530_body70_e137321_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body70_e137318: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign89530_body70_e137319: f64 = (1.0 + assign89530_body70_e137318);
        (assign89530_body70_e137319, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign89530_body70_e137321;
            locals.var_fs0_dps0_dn0 = assign89530_body70_e137321_d_n0;
            locals.var_fs0_dps0_dn2 = assign89530_body70_e137321_d_n2;
            locals.var_fs0_dps0_dn4 = assign89530_body70_e137321_d_n4;
            locals.var_fs0_dps0_dn5 = assign89530_body70_e137321_d_n5;
            locals.var_fs0_dps0_dn6 = assign89530_body70_e137321_d_n6;
            locals.var_fs0_dps0_dn7 = assign89530_body70_e137321_d_n7;
            locals.var_fs0_dps0_dn8 = assign89530_body70_e137321_d_n8;
            locals.var_fs0_dps0_dn9 = assign89530_body70_e137321_d_n9;
            locals.var_fs0_dps0_dn10 = assign89530_body70_e137321_d_n10;
            locals.var_fs0_dps0_dn13 = assign89530_body70_e137321_d_n13;
            let assign89530_body71_e137324: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard2081 = assign89530_body71_e137324;
            let (assign89530_body72_e137337,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 != 0.0)) {
        let assign89530_body72_e137335: f64 = (locals.var_lp_s0_max + 1.0);
        (assign89530_body72_e137335,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89530_body72_e137337;
            let (assign89530_body73_e137352, assign89530_body73_e137352_d_n0, assign89530_body73_e137352_d_n2, assign89530_body73_e137352_d_n4, assign89530_body73_e137352_d_n5, assign89530_body73_e137352_d_n6, assign89530_body73_e137352_d_n7, assign89530_body73_e137352_d_n8, assign89530_body73_e137352_d_n9, assign89530_body73_e137352_d_n10, assign89530_body73_e137352_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89530_body73_e137348: f64 = (-locals.var_fs0);
        let assign89530_body73_e137350: f64 = (assign89530_body73_e137348 / locals.var_fs0_dps0);
        (assign89530_body73_e137350, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign89530_body73_e137348 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign89530_body73_e137352;
            locals.var_dps0_dn0 = assign89530_body73_e137352_d_n0;
            locals.var_dps0_dn2 = assign89530_body73_e137352_d_n2;
            locals.var_dps0_dn4 = assign89530_body73_e137352_d_n4;
            locals.var_dps0_dn5 = assign89530_body73_e137352_d_n5;
            locals.var_dps0_dn6 = assign89530_body73_e137352_d_n6;
            locals.var_dps0_dn7 = assign89530_body73_e137352_d_n7;
            locals.var_dps0_dn8 = assign89530_body73_e137352_d_n8;
            locals.var_dps0_dn9 = assign89530_body73_e137352_d_n9;
            locals.var_dps0_dn10 = assign89530_body73_e137352_d_n10;
            locals.var_dps0_dn13 = assign89530_body73_e137352_d_n13;
            let (assign89530_body74_e137377, assign89530_body74_e137377_d_n0, assign89530_body74_e137377_d_n2, assign89530_body74_e137377_d_n4, assign89530_body74_e137377_d_n5, assign89530_body74_e137377_d_n6, assign89530_body74_e137377_d_n7, assign89530_body74_e137377_d_n8, assign89530_body74_e137377_d_n9, assign89530_body74_e137377_d_n10, assign89530_body74_e137377_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89530_body74_e137364: f64 = (0.5 * 0.1);
        let assign89530_body74_e137368: f64 = (locals.var_ps0ld).abs();
        let (assign89530_body74_e137373, assign89530_body74_e137373_d_n0, assign89530_body74_e137373_d_n2, assign89530_body74_e137373_d_n4, assign89530_body74_e137373_d_n5, assign89530_body74_e137373_d_n6, assign89530_body74_e137373_d_n7, assign89530_body74_e137373_d_n8, assign89530_body74_e137373_d_n9, assign89530_body74_e137373_d_n10, assign89530_body74_e137373_d_n13,) = {
            if (1.0 >= assign89530_body74_e137368) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89530_body74_e137372: f64 = (locals.var_ps0ld).abs();
                (assign89530_body74_e137372, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign89530_body74_e137374: f64 = (1.0 + assign89530_body74_e137373);
        let assign89530_body74_e137375: f64 = (assign89530_body74_e137364 * assign89530_body74_e137374);
        (assign89530_body74_e137375, (assign89530_body74_e137364 * assign89530_body74_e137373_d_n0), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n2), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n4), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n5), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n6), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n7), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n8), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n9), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n10), (assign89530_body74_e137364 * assign89530_body74_e137373_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign89530_body74_e137377;
            locals.var_dplim_dn0 = assign89530_body74_e137377_d_n0;
            locals.var_dplim_dn2 = assign89530_body74_e137377_d_n2;
            locals.var_dplim_dn4 = assign89530_body74_e137377_d_n4;
            locals.var_dplim_dn5 = assign89530_body74_e137377_d_n5;
            locals.var_dplim_dn6 = assign89530_body74_e137377_d_n6;
            locals.var_dplim_dn7 = assign89530_body74_e137377_d_n7;
            locals.var_dplim_dn8 = assign89530_body74_e137377_d_n8;
            locals.var_dplim_dn9 = assign89530_body74_e137377_d_n9;
            locals.var_dplim_dn10 = assign89530_body74_e137377_d_n10;
            locals.var_dplim_dn13 = assign89530_body74_e137377_d_n13;
            let assign89530_body75_e137379: f64 = (locals.var_dps0).abs();
            let assign89530_body75_e137381: f64 = if assign89530_body75_e137379 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2082 = assign89530_body75_e137381;
            let (assign89530_body76_e137403, assign89530_body76_e137403_d_n0, assign89530_body76_e137403_d_n2, assign89530_body76_e137403_d_n4, assign89530_body76_e137403_d_n5, assign89530_body76_e137403_d_n6, assign89530_body76_e137403_d_n7, assign89530_body76_e137403_d_n8, assign89530_body76_e137403_d_n9, assign89530_body76_e137403_d_n10, assign89530_body76_e137403_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) && (locals.var_guard2082 != 0.0)) {
        let (assign89530_body76_e137400,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign89530_body76_e137399: f64 = (-1.0);
                (assign89530_body76_e137399,)
            }
        };
        let assign89530_body76_e137401: f64 = (locals.var_dplim * assign89530_body76_e137400);
        (assign89530_body76_e137401, (locals.var_dplim_dn0 * assign89530_body76_e137400), (locals.var_dplim_dn2 * assign89530_body76_e137400), (locals.var_dplim_dn4 * assign89530_body76_e137400), (locals.var_dplim_dn5 * assign89530_body76_e137400), (locals.var_dplim_dn6 * assign89530_body76_e137400), (locals.var_dplim_dn7 * assign89530_body76_e137400), (locals.var_dplim_dn8 * assign89530_body76_e137400), (locals.var_dplim_dn9 * assign89530_body76_e137400), (locals.var_dplim_dn10 * assign89530_body76_e137400), (locals.var_dplim_dn13 * assign89530_body76_e137400),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign89530_body76_e137403;
            locals.var_dps0_dn0 = assign89530_body76_e137403_d_n0;
            locals.var_dps0_dn2 = assign89530_body76_e137403_d_n2;
            locals.var_dps0_dn4 = assign89530_body76_e137403_d_n4;
            locals.var_dps0_dn5 = assign89530_body76_e137403_d_n5;
            locals.var_dps0_dn6 = assign89530_body76_e137403_d_n6;
            locals.var_dps0_dn7 = assign89530_body76_e137403_d_n7;
            locals.var_dps0_dn8 = assign89530_body76_e137403_d_n8;
            locals.var_dps0_dn9 = assign89530_body76_e137403_d_n9;
            locals.var_dps0_dn10 = assign89530_body76_e137403_d_n10;
            locals.var_dps0_dn13 = assign89530_body76_e137403_d_n13;
            let (assign89530_body77_e137417, assign89530_body77_e137417_d_n0, assign89530_body77_e137417_d_n2, assign89530_body77_e137417_d_n4, assign89530_body77_e137417_d_n5, assign89530_body77_e137417_d_n6, assign89530_body77_e137417_d_n7, assign89530_body77_e137417_d_n8, assign89530_body77_e137417_d_n9, assign89530_body77_e137417_d_n10, assign89530_body77_e137417_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) {
        let assign89530_body77_e137415: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign89530_body77_e137415, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign89530_body77_e137417;
            locals.var_ps0ld_dn0 = assign89530_body77_e137417_d_n0;
            locals.var_ps0ld_dn2 = assign89530_body77_e137417_d_n2;
            locals.var_ps0ld_dn4 = assign89530_body77_e137417_d_n4;
            locals.var_ps0ld_dn5 = assign89530_body77_e137417_d_n5;
            locals.var_ps0ld_dn6 = assign89530_body77_e137417_d_n6;
            locals.var_ps0ld_dn7 = assign89530_body77_e137417_d_n7;
            locals.var_ps0ld_dn8 = assign89530_body77_e137417_d_n8;
            locals.var_ps0ld_dn9 = assign89530_body77_e137417_d_n9;
            locals.var_ps0ld_dn10 = assign89530_body77_e137417_d_n10;
            locals.var_ps0ld_dn13 = assign89530_body77_e137417_d_n13;
            let assign89530_body78_e137419: f64 = (locals.var_dps0).abs();
            let assign89530_body78_e137423: f64 = (locals.var_fs0).abs();
            let assign89530_body78_e137426: f64 = if ((assign89530_body78_e137419 <= 1e-12) && (assign89530_body78_e137423 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2083 = assign89530_body78_e137426;
            let (assign89530_body79_e137440,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) && (locals.var_guard2081 == 0.0)) && (locals.var_guard2083 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign89530_body79_e137440;
            let (assign89530_body80_e137451,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89530_body80_e137449: f64 = (locals.var_lp_s0 + 1.0);
        (assign89530_body80_e137449,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89530_body80_e137451;
        }

    }

    pub(super) fn stamp_transient_block_316(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89550_e137465, assign89550_e137465_d_n0, assign89550_e137465_d_n2, assign89550_e137465_d_n4, assign89550_e137465_d_n5, assign89550_e137465_d_n6, assign89550_e137465_d_n7, assign89550_e137465_d_n8, assign89550_e137465_d_n9, assign89550_e137465_d_n10, assign89550_e137465_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89550_e137463: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign89550_e137463, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk2002, locals.var_wdld__blk2002_dn0, locals.var_wdld__blk2002_dn2, locals.var_wdld__blk2002_dn4, locals.var_wdld__blk2002_dn5, locals.var_wdld__blk2002_dn6, locals.var_wdld__blk2002_dn7, locals.var_wdld__blk2002_dn8, locals.var_wdld__blk2002_dn9, locals.var_wdld__blk2002_dn10, locals.var_wdld__blk2002_dn13,)
    }
};
        locals.var_wdld__blk2002 = assign89550_e137465;
        locals.var_wdld__blk2002_dn0 = assign89550_e137465_d_n0;
        locals.var_wdld__blk2002_dn2 = assign89550_e137465_d_n2;
        locals.var_wdld__blk2002_dn4 = assign89550_e137465_d_n4;
        locals.var_wdld__blk2002_dn5 = assign89550_e137465_d_n5;
        locals.var_wdld__blk2002_dn6 = assign89550_e137465_d_n6;
        locals.var_wdld__blk2002_dn7 = assign89550_e137465_d_n7;
        locals.var_wdld__blk2002_dn8 = assign89550_e137465_d_n8;
        locals.var_wdld__blk2002_dn9 = assign89550_e137465_d_n9;
        locals.var_wdld__blk2002_dn10 = assign89550_e137465_d_n10;
        locals.var_wdld__blk2002_dn13 = assign89550_e137465_d_n13;

        let (assign89560_e137476, assign89560_e137476_d_n0, assign89560_e137476_d_n2, assign89560_e137476_d_n4, assign89560_e137476_d_n5, assign89560_e137476_d_n6, assign89560_e137476_d_n7, assign89560_e137476_d_n8, assign89560_e137476_d_n9, assign89560_e137476_d_n10, assign89560_e137476_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89560_e137474: f64 = (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002);
        (assign89560_e137474, (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn0), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn2), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn4), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn5), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn6), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn7), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn8), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn9), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn10), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn13),)
    } else {
        (locals.var_q_dep_ld__blk2003, locals.var_q_dep_ld__blk2003_dn0, locals.var_q_dep_ld__blk2003_dn2, locals.var_q_dep_ld__blk2003_dn4, locals.var_q_dep_ld__blk2003_dn5, locals.var_q_dep_ld__blk2003_dn6, locals.var_q_dep_ld__blk2003_dn7, locals.var_q_dep_ld__blk2003_dn8, locals.var_q_dep_ld__blk2003_dn9, locals.var_q_dep_ld__blk2003_dn10, locals.var_q_dep_ld__blk2003_dn13,)
    }
};
        locals.var_q_dep_ld__blk2003 = assign89560_e137476;
        locals.var_q_dep_ld__blk2003_dn0 = assign89560_e137476_d_n0;
        locals.var_q_dep_ld__blk2003_dn2 = assign89560_e137476_d_n2;
        locals.var_q_dep_ld__blk2003_dn4 = assign89560_e137476_d_n4;
        locals.var_q_dep_ld__blk2003_dn5 = assign89560_e137476_d_n5;
        locals.var_q_dep_ld__blk2003_dn6 = assign89560_e137476_d_n6;
        locals.var_q_dep_ld__blk2003_dn7 = assign89560_e137476_d_n7;
        locals.var_q_dep_ld__blk2003_dn8 = assign89560_e137476_d_n8;
        locals.var_q_dep_ld__blk2003_dn9 = assign89560_e137476_d_n9;
        locals.var_q_dep_ld__blk2003_dn10 = assign89560_e137476_d_n10;
        locals.var_q_dep_ld__blk2003_dn13 = assign89560_e137476_d_n13;

        let (assign89570_e137491, assign89570_e137491_d_n0, assign89570_e137491_d_n2, assign89570_e137491_d_n4, assign89570_e137491_d_n5, assign89570_e137491_d_n6, assign89570_e137491_d_n7, assign89570_e137491_d_n8, assign89570_e137491_d_n9, assign89570_e137491_d_n10, assign89570_e137491_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89570_e137485: f64 = (locals.var_q_dep_ld__blk2003 / locals.var_cnst0over_func);
        let assign89570_e137488: f64 = (10.0 * 2.220446049250313e-16);
        let assign89570_e137489: f64 = (assign89570_e137485 + assign89570_e137488);
        (assign89570_e137489, (((locals.var_q_dep_ld__blk2003_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign89570_e137491;
        locals.var_xi0p12_dn0 = assign89570_e137491_d_n0;
        locals.var_xi0p12_dn2 = assign89570_e137491_d_n2;
        locals.var_xi0p12_dn4 = assign89570_e137491_d_n4;
        locals.var_xi0p12_dn5 = assign89570_e137491_d_n5;
        locals.var_xi0p12_dn6 = assign89570_e137491_d_n6;
        locals.var_xi0p12_dn7 = assign89570_e137491_d_n7;
        locals.var_xi0p12_dn8 = assign89570_e137491_d_n8;
        locals.var_xi0p12_dn9 = assign89570_e137491_d_n9;
        locals.var_xi0p12_dn10 = assign89570_e137491_d_n10;
        locals.var_xi0p12_dn13 = assign89570_e137491_d_n13;

        let (assign89580_e137502, assign89580_e137502_d_n0, assign89580_e137502_d_n2, assign89580_e137502_d_n4, assign89580_e137502_d_n5, assign89580_e137502_d_n6, assign89580_e137502_d_n7, assign89580_e137502_d_n8, assign89580_e137502_d_n9, assign89580_e137502_d_n10, assign89580_e137502_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89580_e137500: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign89580_e137500, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign89580_e137502;
        locals.var_qbuld_dn0 = assign89580_e137502_d_n0;
        locals.var_qbuld_dn2 = assign89580_e137502_d_n2;
        locals.var_qbuld_dn4 = assign89580_e137502_d_n4;
        locals.var_qbuld_dn5 = assign89580_e137502_d_n5;
        locals.var_qbuld_dn6 = assign89580_e137502_d_n6;
        locals.var_qbuld_dn7 = assign89580_e137502_d_n7;
        locals.var_qbuld_dn8 = assign89580_e137502_d_n8;
        locals.var_qbuld_dn9 = assign89580_e137502_d_n9;
        locals.var_qbuld_dn10 = assign89580_e137502_d_n10;
        locals.var_qbuld_dn13 = assign89580_e137502_d_n13;

        let (assign89590_e137515, assign89590_e137515_d_n0, assign89590_e137515_d_n2, assign89590_e137515_d_n4, assign89590_e137515_d_n5, assign89590_e137515_d_n6, assign89590_e137515_d_n7, assign89590_e137515_d_n8, assign89590_e137515_d_n9, assign89590_e137515_d_n10, assign89590_e137515_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89590_e137512: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign89590_e137513: f64 = (1.0 / assign89590_e137512);
        (assign89590_e137513, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign89590_e137512 * assign89590_e137512))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign89590_e137512 * assign89590_e137512))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign89590_e137515;
        locals.var_t1_dn0 = assign89590_e137515_d_n0;
        locals.var_t1_dn2 = assign89590_e137515_d_n2;
        locals.var_t1_dn4 = assign89590_e137515_d_n4;
        locals.var_t1_dn5 = assign89590_e137515_d_n5;
        locals.var_t1_dn6 = assign89590_e137515_d_n6;
        locals.var_t1_dn7 = assign89590_e137515_d_n7;
        locals.var_t1_dn8 = assign89590_e137515_d_n8;
        locals.var_t1_dn9 = assign89590_e137515_d_n9;
        locals.var_t1_dn10 = assign89590_e137515_d_n10;
        locals.var_t1_dn13 = assign89590_e137515_d_n13;

        let (assign89600_e137528, assign89600_e137528_d_n0, assign89600_e137528_d_n2, assign89600_e137528_d_n4, assign89600_e137528_d_n5, assign89600_e137528_d_n6, assign89600_e137528_d_n7, assign89600_e137528_d_n8, assign89600_e137528_d_n9, assign89600_e137528_d_n10, assign89600_e137528_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89600_e137524: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign89600_e137526: f64 = (assign89600_e137524 * locals.var_t1);
        (assign89600_e137526, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign89600_e137524 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign89600_e137528;
        locals.var_qiuld_dn0 = assign89600_e137528_d_n0;
        locals.var_qiuld_dn2 = assign89600_e137528_d_n2;
        locals.var_qiuld_dn4 = assign89600_e137528_d_n4;
        locals.var_qiuld_dn5 = assign89600_e137528_d_n5;
        locals.var_qiuld_dn6 = assign89600_e137528_d_n6;
        locals.var_qiuld_dn7 = assign89600_e137528_d_n7;
        locals.var_qiuld_dn8 = assign89600_e137528_d_n8;
        locals.var_qiuld_dn9 = assign89600_e137528_d_n9;
        locals.var_qiuld_dn10 = assign89600_e137528_d_n10;
        locals.var_qiuld_dn13 = assign89600_e137528_d_n13;

        let (assign89610_e137539, assign89610_e137539_d_n0, assign89610_e137539_d_n2, assign89610_e137539_d_n4, assign89610_e137539_d_n5, assign89610_e137539_d_n6, assign89610_e137539_d_n7, assign89610_e137539_d_n8, assign89610_e137539_d_n9, assign89610_e137539_d_n10, assign89610_e137539_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2028 == 0.0)) && (locals.var_guard2067 != 0.0)) {
        let assign89610_e137537: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign89610_e137537, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign89610_e137539;
        locals.var_qsuld_dn0 = assign89610_e137539_d_n0;
        locals.var_qsuld_dn2 = assign89610_e137539_d_n2;
        locals.var_qsuld_dn4 = assign89610_e137539_d_n4;
        locals.var_qsuld_dn5 = assign89610_e137539_d_n5;
        locals.var_qsuld_dn6 = assign89610_e137539_d_n6;
        locals.var_qsuld_dn7 = assign89610_e137539_d_n7;
        locals.var_qsuld_dn8 = assign89610_e137539_d_n8;
        locals.var_qsuld_dn9 = assign89610_e137539_d_n9;
        locals.var_qsuld_dn10 = assign89610_e137539_d_n10;
        locals.var_qsuld_dn13 = assign89610_e137539_d_n13;

        let assign89620_e137542: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2085 = assign89620_e137542;

        let (assign89630_e137552, assign89630_e137552_d_n0, assign89630_e137552_d_n2, assign89630_e137552_d_n4, assign89630_e137552_d_n5, assign89630_e137552_d_n6, assign89630_e137552_d_n7, assign89630_e137552_d_n8, assign89630_e137552_d_n9, assign89630_e137552_d_n10, assign89630_e137552_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89630_e137548: f64 = (-locals.var_vxbgmtcl);
        let assign89630_e137549: f64 = (locals.var_beta * assign89630_e137548);
        let assign89630_e137550: f64 = (assign89630_e137549).exp();
        (assign89630_e137550, (assign89630_e137550 * ((locals.var_beta_dn0 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign89630_e137550 * ((locals.var_beta_dn2 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign89630_e137550 * ((locals.var_beta_dn4 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign89630_e137550 * ((locals.var_beta_dn5 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign89630_e137550 * ((locals.var_beta_dn6 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign89630_e137550 * ((locals.var_beta_dn7 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign89630_e137550 * ((locals.var_beta_dn8 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign89630_e137550 * ((locals.var_beta_dn9 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign89630_e137550 * ((locals.var_beta_dn10 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign89630_e137550 * ((locals.var_beta_dn13 * assign89630_e137548) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign89630_e137552;
        locals.var_exp_bvbs_dn0 = assign89630_e137552_d_n0;
        locals.var_exp_bvbs_dn2 = assign89630_e137552_d_n2;
        locals.var_exp_bvbs_dn4 = assign89630_e137552_d_n4;
        locals.var_exp_bvbs_dn5 = assign89630_e137552_d_n5;
        locals.var_exp_bvbs_dn6 = assign89630_e137552_d_n6;
        locals.var_exp_bvbs_dn7 = assign89630_e137552_d_n7;
        locals.var_exp_bvbs_dn8 = assign89630_e137552_d_n8;
        locals.var_exp_bvbs_dn9 = assign89630_e137552_d_n9;
        locals.var_exp_bvbs_dn10 = assign89630_e137552_d_n10;
        locals.var_exp_bvbs_dn13 = assign89630_e137552_d_n13;

        let (assign89640_e137560, assign89640_e137560_d_n0, assign89640_e137560_d_n2, assign89640_e137560_d_n4, assign89640_e137560_d_n5, assign89640_e137560_d_n6, assign89640_e137560_d_n7, assign89640_e137560_d_n8, assign89640_e137560_d_n9, assign89640_e137560_d_n10, assign89640_e137560_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89640_e137558: f64 = (locals.var_nin / locals.var_nover_func);
        (assign89640_e137558, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89640_e137560;
        locals.var_t0_dn0 = assign89640_e137560_d_n0;
        locals.var_t0_dn2 = assign89640_e137560_d_n2;
        locals.var_t0_dn4 = assign89640_e137560_d_n4;
        locals.var_t0_dn5 = assign89640_e137560_d_n5;
        locals.var_t0_dn6 = assign89640_e137560_d_n6;
        locals.var_t0_dn7 = assign89640_e137560_d_n7;
        locals.var_t0_dn8 = assign89640_e137560_d_n8;
        locals.var_t0_dn9 = assign89640_e137560_d_n9;
        locals.var_t0_dn10 = assign89640_e137560_d_n10;
        locals.var_t0_dn13 = assign89640_e137560_d_n13;

        let (assign89650_e137568, assign89650_e137568_d_n0, assign89650_e137568_d_n2, assign89650_e137568_d_n4, assign89650_e137568_d_n5, assign89650_e137568_d_n6, assign89650_e137568_d_n7, assign89650_e137568_d_n8, assign89650_e137568_d_n9, assign89650_e137568_d_n10, assign89650_e137568_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89650_e137566: f64 = (locals.var_t0 * locals.var_t0);
        (assign89650_e137566, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign89650_e137568;
        locals.var_cnst1over_dn0 = assign89650_e137568_d_n0;
        locals.var_cnst1over_dn2 = assign89650_e137568_d_n2;
        locals.var_cnst1over_dn4 = assign89650_e137568_d_n4;
        locals.var_cnst1over_dn5 = assign89650_e137568_d_n5;
        locals.var_cnst1over_dn6 = assign89650_e137568_d_n6;
        locals.var_cnst1over_dn7 = assign89650_e137568_d_n7;
        locals.var_cnst1over_dn8 = assign89650_e137568_d_n8;
        locals.var_cnst1over_dn9 = assign89650_e137568_d_n9;
        locals.var_cnst1over_dn10 = assign89650_e137568_d_n10;
        locals.var_cnst1over_dn13 = assign89650_e137568_d_n13;

        let (assign89660_e137576, assign89660_e137576_d_n0, assign89660_e137576_d_n2, assign89660_e137576_d_n4, assign89660_e137576_d_n5, assign89660_e137576_d_n6, assign89660_e137576_d_n7, assign89660_e137576_d_n8, assign89660_e137576_d_n9, assign89660_e137576_d_n10, assign89660_e137576_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89660_e137574: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign89660_e137574, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign89660_e137576;
        locals.var_cfs1_dn0 = assign89660_e137576_d_n0;
        locals.var_cfs1_dn2 = assign89660_e137576_d_n2;
        locals.var_cfs1_dn4 = assign89660_e137576_d_n4;
        locals.var_cfs1_dn5 = assign89660_e137576_d_n5;
        locals.var_cfs1_dn6 = assign89660_e137576_d_n6;
        locals.var_cfs1_dn7 = assign89660_e137576_d_n7;
        locals.var_cfs1_dn8 = assign89660_e137576_d_n8;
        locals.var_cfs1_dn9 = assign89660_e137576_d_n9;
        locals.var_cfs1_dn10 = assign89660_e137576_d_n10;
        locals.var_cfs1_dn13 = assign89660_e137576_d_n13;

        let (assign89670_e137582, assign89670_e137582_d_n0, assign89670_e137582_d_n2, assign89670_e137582_d_n4, assign89670_e137582_d_n5, assign89670_e137582_d_n6, assign89670_e137582_d_n7, assign89670_e137582_d_n8, assign89670_e137582_d_n9, assign89670_e137582_d_n10, assign89670_e137582_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        (locals.var_ps0ld_ini__blk2011, locals.var_ps0ld_ini__blk2011_dn0, locals.var_ps0ld_ini__blk2011_dn2, locals.var_ps0ld_ini__blk2011_dn4, locals.var_ps0ld_ini__blk2011_dn5, locals.var_ps0ld_ini__blk2011_dn6, locals.var_ps0ld_ini__blk2011_dn7, locals.var_ps0ld_ini__blk2011_dn8, locals.var_ps0ld_ini__blk2011_dn9, locals.var_ps0ld_ini__blk2011_dn10, locals.var_ps0ld_ini__blk2011_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign89670_e137582;
        locals.var_ps0ld_dn0 = assign89670_e137582_d_n0;
        locals.var_ps0ld_dn2 = assign89670_e137582_d_n2;
        locals.var_ps0ld_dn4 = assign89670_e137582_d_n4;
        locals.var_ps0ld_dn5 = assign89670_e137582_d_n5;
        locals.var_ps0ld_dn6 = assign89670_e137582_d_n6;
        locals.var_ps0ld_dn7 = assign89670_e137582_d_n7;
        locals.var_ps0ld_dn8 = assign89670_e137582_d_n8;
        locals.var_ps0ld_dn9 = assign89670_e137582_d_n9;
        locals.var_ps0ld_dn10 = assign89670_e137582_d_n10;
        locals.var_ps0ld_dn13 = assign89670_e137582_d_n13;

        let (assign89680_e137588,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign89680_e137588;

        let (assign89690_e137601, assign89690_e137601_d_n0, assign89690_e137601_d_n2, assign89690_e137601_d_n4, assign89690_e137601_d_n5, assign89690_e137601_d_n6, assign89690_e137601_d_n7, assign89690_e137601_d_n8, assign89690_e137601_d_n9, assign89690_e137601_d_n10, assign89690_e137601_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89690_e137595: f64 = (1.034943e-10 / locals.var_q_nsubld__blk2004);
        let assign89690_e137597: f64 = (assign89690_e137595 * locals.var_beta_inv);
        let assign89690_e137598: f64 = (2.0 * assign89690_e137597);
        let assign89690_e137599: f64 = (assign89690_e137598).sqrt();
        (assign89690_e137599, ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn0)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn2)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn4)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn5)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn6)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn7)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn8)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn9)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn10)) / (2.0 * assign89690_e137599)), ((2.0 * (assign89690_e137595 * locals.var_beta_inv_dn13)) / (2.0 * assign89690_e137599)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign89690_e137601;
        locals.var_c_w_ld_dn0 = assign89690_e137601_d_n0;
        locals.var_c_w_ld_dn2 = assign89690_e137601_d_n2;
        locals.var_c_w_ld_dn4 = assign89690_e137601_d_n4;
        locals.var_c_w_ld_dn5 = assign89690_e137601_d_n5;
        locals.var_c_w_ld_dn6 = assign89690_e137601_d_n6;
        locals.var_c_w_ld_dn7 = assign89690_e137601_d_n7;
        locals.var_c_w_ld_dn8 = assign89690_e137601_d_n8;
        locals.var_c_w_ld_dn9 = assign89690_e137601_d_n9;
        locals.var_c_w_ld_dn10 = assign89690_e137601_d_n10;
        locals.var_c_w_ld_dn13 = assign89690_e137601_d_n13;

        let assign89700_e137604: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2086 = assign89700_e137604;

        let (assign89710_e137614, assign89710_e137614_d_n0, assign89710_e137614_d_n2, assign89710_e137614_d_n4, assign89710_e137614_d_n5, assign89710_e137614_d_n6, assign89710_e137614_d_n7, assign89710_e137614_d_n8, assign89710_e137614_d_n9, assign89710_e137614_d_n10, assign89710_e137614_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 != 0.0)) {
        let assign89710_e137612: f64 = (p.p334 - locals.var_wdep_func);
        (assign89710_e137612, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89710_e137614;
        locals.var_t2_dn0 = assign89710_e137614_d_n0;
        locals.var_t2_dn2 = assign89710_e137614_d_n2;
        locals.var_t2_dn4 = assign89710_e137614_d_n4;
        locals.var_t2_dn5 = assign89710_e137614_d_n5;
        locals.var_t2_dn6 = assign89710_e137614_d_n6;
        locals.var_t2_dn7 = assign89710_e137614_d_n7;
        locals.var_t2_dn8 = assign89710_e137614_d_n8;
        locals.var_t2_dn9 = assign89710_e137614_d_n9;
        locals.var_t2_dn10 = assign89710_e137614_d_n10;
        locals.var_t2_dn13 = assign89710_e137614_d_n13;

        let (assign89720_e137636, assign89720_e137636_d_n0, assign89720_e137636_d_n2, assign89720_e137636_d_n4, assign89720_e137636_d_n5, assign89720_e137636_d_n6, assign89720_e137636_d_n7, assign89720_e137636_d_n8, assign89720_e137636_d_n9, assign89720_e137636_d_n10, assign89720_e137636_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89720_e137623: f64 = (locals.var_vdsi + p.p137);
        let assign89720_e137626: f64 = (locals.var_vdsi + p.p137);
        let assign89720_e137627: f64 = (assign89720_e137623 * assign89720_e137626);
        let assign89720_e137630: f64 = (4.0 * 0.1);
        let assign89720_e137632: f64 = (assign89720_e137630 * 0.1);
        let assign89720_e137633: f64 = (assign89720_e137627 + assign89720_e137632);
        let assign89720_e137634: f64 = (assign89720_e137633).sqrt();
        (assign89720_e137634, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign89720_e137626) + (assign89720_e137623 * locals.var_vdsi_dn5)) / (2.0 * assign89720_e137634)), 0.0, (((locals.var_vdsi_dn7 * assign89720_e137626) + (assign89720_e137623 * locals.var_vdsi_dn7)) / (2.0 * assign89720_e137634)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign89720_e137636;
        locals.var_tmf2_dn0 = assign89720_e137636_d_n0;
        locals.var_tmf2_dn2 = assign89720_e137636_d_n2;
        locals.var_tmf2_dn4 = assign89720_e137636_d_n4;
        locals.var_tmf2_dn5 = assign89720_e137636_d_n5;
        locals.var_tmf2_dn6 = assign89720_e137636_d_n6;
        locals.var_tmf2_dn7 = assign89720_e137636_d_n7;
        locals.var_tmf2_dn8 = assign89720_e137636_d_n8;
        locals.var_tmf2_dn9 = assign89720_e137636_d_n9;
        locals.var_tmf2_dn10 = assign89720_e137636_d_n10;
        locals.var_tmf2_dn13 = assign89720_e137636_d_n13;

        let (assign89730_e137653, assign89730_e137653_d_n0, assign89730_e137653_d_n2, assign89730_e137653_d_n4, assign89730_e137653_d_n5, assign89730_e137653_d_n6, assign89730_e137653_d_n7, assign89730_e137653_d_n8, assign89730_e137653_d_n9, assign89730_e137653_d_n10, assign89730_e137653_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89730_e137647: f64 = (locals.var_vdsi + p.p137);
        let assign89730_e137649: f64 = (assign89730_e137647 / locals.var_tmf2);
        let assign89730_e137650: f64 = (1.0 + assign89730_e137649);
        let assign89730_e137651: f64 = (0.5 * assign89730_e137650);
        (assign89730_e137651, (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign89730_e137647 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign89730_e137647 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign89730_e137647 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89730_e137653;
        locals.var_t9_dn0 = assign89730_e137653_d_n0;
        locals.var_t9_dn2 = assign89730_e137653_d_n2;
        locals.var_t9_dn4 = assign89730_e137653_d_n4;
        locals.var_t9_dn5 = assign89730_e137653_d_n5;
        locals.var_t9_dn6 = assign89730_e137653_d_n6;
        locals.var_t9_dn7 = assign89730_e137653_d_n7;
        locals.var_t9_dn8 = assign89730_e137653_d_n8;
        locals.var_t9_dn9 = assign89730_e137653_d_n9;
        locals.var_t9_dn10 = assign89730_e137653_d_n10;
        locals.var_t9_dn13 = assign89730_e137653_d_n13;

        let (assign89740_e137668, assign89740_e137668_d_n0, assign89740_e137668_d_n2, assign89740_e137668_d_n4, assign89740_e137668_d_n5, assign89740_e137668_d_n6, assign89740_e137668_d_n7, assign89740_e137668_d_n8, assign89740_e137668_d_n9, assign89740_e137668_d_n10, assign89740_e137668_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89740_e137663: f64 = (locals.var_vdsi + p.p137);
        let assign89740_e137665: f64 = (assign89740_e137663 + locals.var_tmf2);
        let assign89740_e137666: f64 = (0.5 * assign89740_e137665);
        (assign89740_e137666, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89740_e137668;
        locals.var_t2_dn0 = assign89740_e137668_d_n0;
        locals.var_t2_dn2 = assign89740_e137668_d_n2;
        locals.var_t2_dn4 = assign89740_e137668_d_n4;
        locals.var_t2_dn5 = assign89740_e137668_d_n5;
        locals.var_t2_dn6 = assign89740_e137668_d_n6;
        locals.var_t2_dn7 = assign89740_e137668_d_n7;
        locals.var_t2_dn8 = assign89740_e137668_d_n8;
        locals.var_t2_dn9 = assign89740_e137668_d_n9;
        locals.var_t2_dn10 = assign89740_e137668_d_n10;
        locals.var_t2_dn13 = assign89740_e137668_d_n13;

        let assign89750_e137671: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2087 = assign89750_e137671;

        let (assign89760_e137682, assign89760_e137682_d_n0, assign89760_e137682_d_n2, assign89760_e137682_d_n4, assign89760_e137682_d_n5, assign89760_e137682_d_n6, assign89760_e137682_d_n7, assign89760_e137682_d_n8, assign89760_e137682_d_n9, assign89760_e137682_d_n10, assign89760_e137682_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) && (locals.var_guard2087 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89760_e137682;
        locals.var_t2_dn0 = assign89760_e137682_d_n0;
        locals.var_t2_dn2 = assign89760_e137682_d_n2;
        locals.var_t2_dn4 = assign89760_e137682_d_n4;
        locals.var_t2_dn5 = assign89760_e137682_d_n5;
        locals.var_t2_dn6 = assign89760_e137682_d_n6;
        locals.var_t2_dn7 = assign89760_e137682_d_n7;
        locals.var_t2_dn8 = assign89760_e137682_d_n8;
        locals.var_t2_dn9 = assign89760_e137682_d_n9;
        locals.var_t2_dn10 = assign89760_e137682_d_n10;
        locals.var_t2_dn13 = assign89760_e137682_d_n13;

        let (assign89770_e137693, assign89770_e137693_d_n0, assign89770_e137693_d_n2, assign89770_e137693_d_n4, assign89770_e137693_d_n5, assign89770_e137693_d_n6, assign89770_e137693_d_n7, assign89770_e137693_d_n8, assign89770_e137693_d_n9, assign89770_e137693_d_n10, assign89770_e137693_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) && (locals.var_guard2087 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89770_e137693;
        locals.var_t9_dn0 = assign89770_e137693_d_n0;
        locals.var_t9_dn2 = assign89770_e137693_d_n2;
        locals.var_t9_dn4 = assign89770_e137693_d_n4;
        locals.var_t9_dn5 = assign89770_e137693_d_n5;
        locals.var_t9_dn6 = assign89770_e137693_d_n6;
        locals.var_t9_dn7 = assign89770_e137693_d_n7;
        locals.var_t9_dn8 = assign89770_e137693_d_n8;
        locals.var_t9_dn9 = assign89770_e137693_d_n9;
        locals.var_t9_dn10 = assign89770_e137693_d_n10;
        locals.var_t9_dn13 = assign89770_e137693_d_n13;

        let (assign89780_e137707, assign89780_e137707_d_n0, assign89780_e137707_d_n2, assign89780_e137707_d_n4, assign89780_e137707_d_n5, assign89780_e137707_d_n6, assign89780_e137707_d_n7, assign89780_e137707_d_n8, assign89780_e137707_d_n9, assign89780_e137707_d_n10, assign89780_e137707_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89780_e137702: f64 = (locals.var_kjunc * locals.var_t2);
        let assign89780_e137703: f64 = (assign89780_e137702).sqrt();
        let assign89780_e137705: f64 = (assign89780_e137703 * p.p432);
        (assign89780_e137705, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign89780_e137703)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign89780_e137703)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign89780_e137707;
        locals.var_wjunc0_dn0 = assign89780_e137707_d_n0;
        locals.var_wjunc0_dn2 = assign89780_e137707_d_n2;
        locals.var_wjunc0_dn4 = assign89780_e137707_d_n4;
        locals.var_wjunc0_dn5 = assign89780_e137707_d_n5;
        locals.var_wjunc0_dn6 = assign89780_e137707_d_n6;
        locals.var_wjunc0_dn7 = assign89780_e137707_d_n7;
        locals.var_wjunc0_dn8 = assign89780_e137707_d_n8;
        locals.var_wjunc0_dn9 = assign89780_e137707_d_n9;
        locals.var_wjunc0_dn10 = assign89780_e137707_d_n10;
        locals.var_wjunc0_dn13 = assign89780_e137707_d_n13;

        let (assign89790_e137718, assign89790_e137718_d_n0, assign89790_e137718_d_n2, assign89790_e137718_d_n4, assign89790_e137718_d_n5, assign89790_e137718_d_n6, assign89790_e137718_d_n7, assign89790_e137718_d_n8, assign89790_e137718_d_n9, assign89790_e137718_d_n10, assign89790_e137718_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2086 == 0.0)) {
        let assign89790_e137716: f64 = (p.p334 - locals.var_wjunc0);
        (assign89790_e137716, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89790_e137718;
        locals.var_t2_dn0 = assign89790_e137718_d_n0;
        locals.var_t2_dn2 = assign89790_e137718_d_n2;
        locals.var_t2_dn4 = assign89790_e137718_d_n4;
        locals.var_t2_dn5 = assign89790_e137718_d_n5;
        locals.var_t2_dn6 = assign89790_e137718_d_n6;
        locals.var_t2_dn7 = assign89790_e137718_d_n7;
        locals.var_t2_dn8 = assign89790_e137718_d_n8;
        locals.var_t2_dn9 = assign89790_e137718_d_n9;
        locals.var_t2_dn10 = assign89790_e137718_d_n10;
        locals.var_t2_dn13 = assign89790_e137718_d_n13;

        let (assign89800_e137737, assign89800_e137737_d_n0, assign89800_e137737_d_n2, assign89800_e137737_d_n4, assign89800_e137737_d_n5, assign89800_e137737_d_n6, assign89800_e137737_d_n7, assign89800_e137737_d_n8, assign89800_e137737_d_n9, assign89800_e137737_d_n10, assign89800_e137737_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89800_e137724: f64 = (locals.var_t2 * locals.var_t2);
        let assign89800_e137728: f64 = (p.p334 * 0.01);
        let assign89800_e137729: f64 = (4.0 * assign89800_e137728);
        let assign89800_e137732: f64 = (p.p334 * 0.01);
        let assign89800_e137733: f64 = (assign89800_e137729 * assign89800_e137732);
        let assign89800_e137734: f64 = (assign89800_e137724 + assign89800_e137733);
        let assign89800_e137735: f64 = (assign89800_e137734).sqrt();
        (assign89800_e137735, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign89800_e137735)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign89800_e137735)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign89800_e137737;
        locals.var_tmf2_dn0 = assign89800_e137737_d_n0;
        locals.var_tmf2_dn2 = assign89800_e137737_d_n2;
        locals.var_tmf2_dn4 = assign89800_e137737_d_n4;
        locals.var_tmf2_dn5 = assign89800_e137737_d_n5;
        locals.var_tmf2_dn6 = assign89800_e137737_d_n6;
        locals.var_tmf2_dn7 = assign89800_e137737_d_n7;
        locals.var_tmf2_dn8 = assign89800_e137737_d_n8;
        locals.var_tmf2_dn9 = assign89800_e137737_d_n9;
        locals.var_tmf2_dn10 = assign89800_e137737_d_n10;
        locals.var_tmf2_dn13 = assign89800_e137737_d_n13;

        let (assign89810_e137749, assign89810_e137749_d_n0, assign89810_e137749_d_n2, assign89810_e137749_d_n4, assign89810_e137749_d_n5, assign89810_e137749_d_n6, assign89810_e137749_d_n7, assign89810_e137749_d_n8, assign89810_e137749_d_n9, assign89810_e137749_d_n10, assign89810_e137749_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89810_e137745: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign89810_e137746: f64 = (1.0 + assign89810_e137745);
        let assign89810_e137747: f64 = (0.5 * assign89810_e137746);
        (assign89810_e137747, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89810_e137749;
        locals.var_t9_dn0 = assign89810_e137749_d_n0;
        locals.var_t9_dn2 = assign89810_e137749_d_n2;
        locals.var_t9_dn4 = assign89810_e137749_d_n4;
        locals.var_t9_dn5 = assign89810_e137749_d_n5;
        locals.var_t9_dn6 = assign89810_e137749_d_n6;
        locals.var_t9_dn7 = assign89810_e137749_d_n7;
        locals.var_t9_dn8 = assign89810_e137749_d_n8;
        locals.var_t9_dn9 = assign89810_e137749_d_n9;
        locals.var_t9_dn10 = assign89810_e137749_d_n10;
        locals.var_t9_dn13 = assign89810_e137749_d_n13;

    }

    pub(super) fn stamp_transient_block_317(
        locals: &mut StampLocals,
    ) {
        let (assign89820_e137759, assign89820_e137759_d_n0, assign89820_e137759_d_n2, assign89820_e137759_d_n4, assign89820_e137759_d_n5, assign89820_e137759_d_n6, assign89820_e137759_d_n7, assign89820_e137759_d_n8, assign89820_e137759_d_n9, assign89820_e137759_d_n10, assign89820_e137759_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89820_e137756: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign89820_e137757: f64 = (0.5 * assign89820_e137756);
        (assign89820_e137757, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89820_e137759;
        locals.var_t2_dn0 = assign89820_e137759_d_n0;
        locals.var_t2_dn2 = assign89820_e137759_d_n2;
        locals.var_t2_dn4 = assign89820_e137759_d_n4;
        locals.var_t2_dn5 = assign89820_e137759_d_n5;
        locals.var_t2_dn6 = assign89820_e137759_d_n6;
        locals.var_t2_dn7 = assign89820_e137759_d_n7;
        locals.var_t2_dn8 = assign89820_e137759_d_n8;
        locals.var_t2_dn9 = assign89820_e137759_d_n9;
        locals.var_t2_dn10 = assign89820_e137759_d_n10;
        locals.var_t2_dn13 = assign89820_e137759_d_n13;

        let assign89830_e137762: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2088 = assign89830_e137762;

        let (assign89840_e137770, assign89840_e137770_d_n0, assign89840_e137770_d_n2, assign89840_e137770_d_n4, assign89840_e137770_d_n5, assign89840_e137770_d_n6, assign89840_e137770_d_n7, assign89840_e137770_d_n8, assign89840_e137770_d_n9, assign89840_e137770_d_n10, assign89840_e137770_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2088 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign89840_e137770;
        locals.var_t2_dn0 = assign89840_e137770_d_n0;
        locals.var_t2_dn2 = assign89840_e137770_d_n2;
        locals.var_t2_dn4 = assign89840_e137770_d_n4;
        locals.var_t2_dn5 = assign89840_e137770_d_n5;
        locals.var_t2_dn6 = assign89840_e137770_d_n6;
        locals.var_t2_dn7 = assign89840_e137770_d_n7;
        locals.var_t2_dn8 = assign89840_e137770_d_n8;
        locals.var_t2_dn9 = assign89840_e137770_d_n9;
        locals.var_t2_dn10 = assign89840_e137770_d_n10;
        locals.var_t2_dn13 = assign89840_e137770_d_n13;

        let (assign89850_e137778, assign89850_e137778_d_n0, assign89850_e137778_d_n2, assign89850_e137778_d_n4, assign89850_e137778_d_n5, assign89850_e137778_d_n6, assign89850_e137778_d_n7, assign89850_e137778_d_n8, assign89850_e137778_d_n9, assign89850_e137778_d_n10, assign89850_e137778_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2088 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign89850_e137778;
        locals.var_t9_dn0 = assign89850_e137778_d_n0;
        locals.var_t9_dn2 = assign89850_e137778_d_n2;
        locals.var_t9_dn4 = assign89850_e137778_d_n4;
        locals.var_t9_dn5 = assign89850_e137778_d_n5;
        locals.var_t9_dn6 = assign89850_e137778_d_n6;
        locals.var_t9_dn7 = assign89850_e137778_d_n7;
        locals.var_t9_dn8 = assign89850_e137778_d_n8;
        locals.var_t9_dn9 = assign89850_e137778_d_n9;
        locals.var_t9_dn10 = assign89850_e137778_d_n10;
        locals.var_t9_dn13 = assign89850_e137778_d_n13;

        let (assign89860_e137784, assign89860_e137784_d_n0, assign89860_e137784_d_n2, assign89860_e137784_d_n4, assign89860_e137784_d_n5, assign89860_e137784_d_n6, assign89860_e137784_d_n7, assign89860_e137784_d_n8, assign89860_e137784_d_n9, assign89860_e137784_d_n10, assign89860_e137784_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign89860_e137784;
        locals.var_ddriftldc_dn0 = assign89860_e137784_d_n0;
        locals.var_ddriftldc_dn2 = assign89860_e137784_d_n2;
        locals.var_ddriftldc_dn4 = assign89860_e137784_d_n4;
        locals.var_ddriftldc_dn5 = assign89860_e137784_d_n5;
        locals.var_ddriftldc_dn6 = assign89860_e137784_d_n6;
        locals.var_ddriftldc_dn7 = assign89860_e137784_d_n7;
        locals.var_ddriftldc_dn8 = assign89860_e137784_d_n8;
        locals.var_ddriftldc_dn9 = assign89860_e137784_d_n9;
        locals.var_ddriftldc_dn10 = assign89860_e137784_d_n10;
        locals.var_ddriftldc_dn13 = assign89860_e137784_d_n13;

        let (assign89870_e137798, assign89870_e137798_d_n0, assign89870_e137798_d_n2, assign89870_e137798_d_n4, assign89870_e137798_d_n5, assign89870_e137798_d_n6, assign89870_e137798_d_n7, assign89870_e137798_d_n8, assign89870_e137798_d_n9, assign89870_e137798_d_n10, assign89870_e137798_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89870_e137790: f64 = (locals.var_q_nsubld__blk2004 * locals.var_ddriftldc);
        let assign89870_e137792: f64 = (assign89870_e137790 * locals.var_ddriftldc);
        let assign89870_e137794: f64 = (assign89870_e137792 / 2.0);
        let assign89870_e137796: f64 = (assign89870_e137794 / 1.034943e-10);
        (assign89870_e137796, (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk2004 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign89870_e137790 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign89870_e137798;
        locals.var_dphi_sb_dn0 = assign89870_e137798_d_n0;
        locals.var_dphi_sb_dn2 = assign89870_e137798_d_n2;
        locals.var_dphi_sb_dn4 = assign89870_e137798_d_n4;
        locals.var_dphi_sb_dn5 = assign89870_e137798_d_n5;
        locals.var_dphi_sb_dn6 = assign89870_e137798_d_n6;
        locals.var_dphi_sb_dn7 = assign89870_e137798_d_n7;
        locals.var_dphi_sb_dn8 = assign89870_e137798_d_n8;
        locals.var_dphi_sb_dn9 = assign89870_e137798_d_n9;
        locals.var_dphi_sb_dn10 = assign89870_e137798_d_n10;
        locals.var_dphi_sb_dn13 = assign89870_e137798_d_n13;

        let (assign89880_e137809, assign89880_e137809_d_n0, assign89880_e137809_d_n2, assign89880_e137809_d_n4, assign89880_e137809_d_n5, assign89880_e137809_d_n6, assign89880_e137809_d_n7, assign89880_e137809_d_n8, assign89880_e137809_d_n9, assign89880_e137809_d_n10, assign89880_e137809_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89880_e137804: f64 = (2.0 * locals.var_beta);
        let assign89880_e137806: f64 = (assign89880_e137804 * locals.var_dphi_sb);
        let assign89880_e137807: f64 = (assign89880_e137806).sqrt();
        (assign89880_e137807, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn0)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn2)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn4)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn5)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn6)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn7)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn8)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn9)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn10)) / (2.0 * assign89880_e137807)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign89880_e137804 * locals.var_dphi_sb_dn13)) / (2.0 * assign89880_e137807)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign89880_e137809;
        locals.var_t0_dn0 = assign89880_e137809_d_n0;
        locals.var_t0_dn2 = assign89880_e137809_d_n2;
        locals.var_t0_dn4 = assign89880_e137809_d_n4;
        locals.var_t0_dn5 = assign89880_e137809_d_n5;
        locals.var_t0_dn6 = assign89880_e137809_d_n6;
        locals.var_t0_dn7 = assign89880_e137809_d_n7;
        locals.var_t0_dn8 = assign89880_e137809_d_n8;
        locals.var_t0_dn9 = assign89880_e137809_d_n9;
        locals.var_t0_dn10 = assign89880_e137809_d_n10;
        locals.var_t0_dn13 = assign89880_e137809_d_n13;

        let (assign89890_e137822, assign89890_e137822_d_n0, assign89890_e137822_d_n2, assign89890_e137822_d_n4, assign89890_e137822_d_n5, assign89890_e137822_d_n6, assign89890_e137822_d_n7, assign89890_e137822_d_n8, assign89890_e137822_d_n9, assign89890_e137822_d_n10, assign89890_e137822_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89890_e137814: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89890_e137816: f64 = (-locals.var_t0);
        let assign89890_e137817: f64 = { let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign89890_e137818: f64 = (assign89890_e137814 + assign89890_e137817);
        let assign89890_e137820: f64 = (assign89890_e137818 / 2.0);
        (assign89890_e137820, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign89890_e137816; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign89890_e137822;
        locals.var_t1_dn0 = assign89890_e137822_d_n0;
        locals.var_t1_dn2 = assign89890_e137822_d_n2;
        locals.var_t1_dn4 = assign89890_e137822_d_n4;
        locals.var_t1_dn5 = assign89890_e137822_d_n5;
        locals.var_t1_dn6 = assign89890_e137822_d_n6;
        locals.var_t1_dn7 = assign89890_e137822_d_n7;
        locals.var_t1_dn8 = assign89890_e137822_d_n8;
        locals.var_t1_dn9 = assign89890_e137822_d_n9;
        locals.var_t1_dn10 = assign89890_e137822_d_n10;
        locals.var_t1_dn13 = assign89890_e137822_d_n13;

        let (assign89900_e137831, assign89900_e137831_d_n0, assign89900_e137831_d_n2, assign89900_e137831_d_n4, assign89900_e137831_d_n5, assign89900_e137831_d_n6, assign89900_e137831_d_n7, assign89900_e137831_d_n8, assign89900_e137831_d_n9, assign89900_e137831_d_n10, assign89900_e137831_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89900_e137827: f64 = (locals.var_t1).ln();
        let assign89900_e137829: f64 = (assign89900_e137827 / locals.var_dphi_sb);
        (assign89900_e137829, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign89900_e137827 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign89900_e137831;
        locals.var_c_sb_dn0 = assign89900_e137831_d_n0;
        locals.var_c_sb_dn2 = assign89900_e137831_d_n2;
        locals.var_c_sb_dn4 = assign89900_e137831_d_n4;
        locals.var_c_sb_dn5 = assign89900_e137831_d_n5;
        locals.var_c_sb_dn6 = assign89900_e137831_d_n6;
        locals.var_c_sb_dn7 = assign89900_e137831_d_n7;
        locals.var_c_sb_dn8 = assign89900_e137831_d_n8;
        locals.var_c_sb_dn9 = assign89900_e137831_d_n9;
        locals.var_c_sb_dn10 = assign89900_e137831_d_n10;
        locals.var_c_sb_dn13 = assign89900_e137831_d_n13;

        let (assign89910_e137837,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign89910_e137837;

    }

    pub(super) fn stamp_transient_block_318(
        locals: &mut StampLocals,
    ) {
        let mut assign89920_loop_guard: usize = 0;
        while {
            let assign89920_cond_e137844: f64 = (locals.var_lp_s0_max + 1.0);
            let assign89920_cond_e137846: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_lp_s0 <= assign89920_cond_e137844)) { 1.0 } else { 0.0 };
            assign89920_cond_e137846 != 0.0
        } {
            assign89920_loop_guard += 1;
            assert!(assign89920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign89920_body3_e137873, assign89920_body3_e137873_d_n0, assign89920_body3_e137873_d_n2, assign89920_body3_e137873_d_n4, assign89920_body3_e137873_d_n5, assign89920_body3_e137873_d_n6, assign89920_body3_e137873_d_n7, assign89920_body3_e137873_d_n8, assign89920_body3_e137873_d_n9, assign89920_body3_e137873_d_n10, assign89920_body3_e137873_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body3_e137871: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign89920_body3_e137871, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign89920_body3_e137873;
            locals.var_ps0ld_vxb_dn0 = assign89920_body3_e137873_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign89920_body3_e137873_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign89920_body3_e137873_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign89920_body3_e137873_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign89920_body3_e137873_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign89920_body3_e137873_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign89920_body3_e137873_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign89920_body3_e137873_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign89920_body3_e137873_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign89920_body3_e137873_d_n13;
            let (assign89920_body4_e137881, assign89920_body4_e137881_d_n0, assign89920_body4_e137881_d_n2, assign89920_body4_e137881_d_n4, assign89920_body4_e137881_d_n5, assign89920_body4_e137881_d_n6, assign89920_body4_e137881_d_n7, assign89920_body4_e137881_d_n8, assign89920_body4_e137881_d_n9, assign89920_body4_e137881_d_n10, assign89920_body4_e137881_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body4_e137879: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign89920_body4_e137879, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign89920_body4_e137881;
            locals.var_chi_dn0 = assign89920_body4_e137881_d_n0;
            locals.var_chi_dn2 = assign89920_body4_e137881_d_n2;
            locals.var_chi_dn4 = assign89920_body4_e137881_d_n4;
            locals.var_chi_dn5 = assign89920_body4_e137881_d_n5;
            locals.var_chi_dn6 = assign89920_body4_e137881_d_n6;
            locals.var_chi_dn7 = assign89920_body4_e137881_d_n7;
            locals.var_chi_dn8 = assign89920_body4_e137881_d_n8;
            locals.var_chi_dn9 = assign89920_body4_e137881_d_n9;
            locals.var_chi_dn10 = assign89920_body4_e137881_d_n10;
            locals.var_chi_dn13 = assign89920_body4_e137881_d_n13;
            let (assign89920_body5_e137891, assign89920_body5_e137891_d_n0, assign89920_body5_e137891_d_n2, assign89920_body5_e137891_d_n4, assign89920_body5_e137891_d_n5, assign89920_body5_e137891_d_n6, assign89920_body5_e137891_d_n7, assign89920_body5_e137891_d_n8, assign89920_body5_e137891_d_n9, assign89920_body5_e137891_d_n10, assign89920_body5_e137891_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body5_e137888: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign89920_body5_e137889: f64 = (locals.var_c_sb * assign89920_body5_e137888);
        (assign89920_body5_e137889, ((locals.var_c_sb_dn0 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign89920_body5_e137888) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign89920_body5_e137891;
            locals.var_ty_dn0 = assign89920_body5_e137891_d_n0;
            locals.var_ty_dn2 = assign89920_body5_e137891_d_n2;
            locals.var_ty_dn4 = assign89920_body5_e137891_d_n4;
            locals.var_ty_dn5 = assign89920_body5_e137891_d_n5;
            locals.var_ty_dn6 = assign89920_body5_e137891_d_n6;
            locals.var_ty_dn7 = assign89920_body5_e137891_d_n7;
            locals.var_ty_dn8 = assign89920_body5_e137891_d_n8;
            locals.var_ty_dn9 = assign89920_body5_e137891_d_n9;
            locals.var_ty_dn10 = assign89920_body5_e137891_d_n10;
            locals.var_ty_dn13 = assign89920_body5_e137891_d_n13;
            let assign89920_body6_e137894: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2090 = assign89920_body6_e137894;
            let (assign89920_body7_e137903, assign89920_body7_e137903_d_n0, assign89920_body7_e137903_d_n2, assign89920_body7_e137903_d_n4, assign89920_body7_e137903_d_n5, assign89920_body7_e137903_d_n6, assign89920_body7_e137903_d_n7, assign89920_body7_e137903_d_n8, assign89920_body7_e137903_d_n9, assign89920_body7_e137903_d_n10, assign89920_body7_e137903_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body7_e137901: f64 = (locals.var_ty).exp();
        (assign89920_body7_e137901, (assign89920_body7_e137901 * locals.var_ty_dn0), (assign89920_body7_e137901 * locals.var_ty_dn2), (assign89920_body7_e137901 * locals.var_ty_dn4), (assign89920_body7_e137901 * locals.var_ty_dn5), (assign89920_body7_e137901 * locals.var_ty_dn6), (assign89920_body7_e137901 * locals.var_ty_dn7), (assign89920_body7_e137901 * locals.var_ty_dn8), (assign89920_body7_e137901 * locals.var_ty_dn9), (assign89920_body7_e137901 * locals.var_ty_dn10), (assign89920_body7_e137901 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body7_e137903;
            locals.var_t1_dn0 = assign89920_body7_e137903_d_n0;
            locals.var_t1_dn2 = assign89920_body7_e137903_d_n2;
            locals.var_t1_dn4 = assign89920_body7_e137903_d_n4;
            locals.var_t1_dn5 = assign89920_body7_e137903_d_n5;
            locals.var_t1_dn6 = assign89920_body7_e137903_d_n6;
            locals.var_t1_dn7 = assign89920_body7_e137903_d_n7;
            locals.var_t1_dn8 = assign89920_body7_e137903_d_n8;
            locals.var_t1_dn9 = assign89920_body7_e137903_d_n9;
            locals.var_t1_dn10 = assign89920_body7_e137903_d_n10;
            locals.var_t1_dn13 = assign89920_body7_e137903_d_n13;
            let (assign89920_body8_e137915, assign89920_body8_e137915_d_n0, assign89920_body8_e137915_d_n2, assign89920_body8_e137915_d_n4, assign89920_body8_e137915_d_n5, assign89920_body8_e137915_d_n6, assign89920_body8_e137915_d_n7, assign89920_body8_e137915_d_n8, assign89920_body8_e137915_d_n9, assign89920_body8_e137915_d_n10, assign89920_body8_e137915_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body8_e137910: f64 = (-locals.var_c_sb);
        let assign89920_body8_e137912: f64 = (assign89920_body8_e137910 * locals.var_dphi_sb);
        let assign89920_body8_e137913: f64 = (assign89920_body8_e137912).exp();
        (assign89920_body8_e137913, (assign89920_body8_e137913 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn0))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn2))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn4))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn5))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn6))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn7))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn8))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn9))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn10))), (assign89920_body8_e137913 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign89920_body8_e137910 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89920_body8_e137915;
            locals.var_t0_dn0 = assign89920_body8_e137915_d_n0;
            locals.var_t0_dn2 = assign89920_body8_e137915_d_n2;
            locals.var_t0_dn4 = assign89920_body8_e137915_d_n4;
            locals.var_t0_dn5 = assign89920_body8_e137915_d_n5;
            locals.var_t0_dn6 = assign89920_body8_e137915_d_n6;
            locals.var_t0_dn7 = assign89920_body8_e137915_d_n7;
            locals.var_t0_dn8 = assign89920_body8_e137915_d_n8;
            locals.var_t0_dn9 = assign89920_body8_e137915_d_n9;
            locals.var_t0_dn10 = assign89920_body8_e137915_d_n10;
            locals.var_t0_dn13 = assign89920_body8_e137915_d_n13;
            let (assign89920_body9_e137925, assign89920_body9_e137925_d_n0, assign89920_body9_e137925_d_n2, assign89920_body9_e137925_d_n4, assign89920_body9_e137925_d_n5, assign89920_body9_e137925_d_n6, assign89920_body9_e137925_d_n7, assign89920_body9_e137925_d_n8, assign89920_body9_e137925_d_n9, assign89920_body9_e137925_d_n10, assign89920_body9_e137925_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body9_e137923: f64 = (locals.var_t1 - locals.var_t0);
        (assign89920_body9_e137923, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign89920_body9_e137925;
            locals.var_t2_dn0 = assign89920_body9_e137925_d_n0;
            locals.var_t2_dn2 = assign89920_body9_e137925_d_n2;
            locals.var_t2_dn4 = assign89920_body9_e137925_d_n4;
            locals.var_t2_dn5 = assign89920_body9_e137925_d_n5;
            locals.var_t2_dn6 = assign89920_body9_e137925_d_n6;
            locals.var_t2_dn7 = assign89920_body9_e137925_d_n7;
            locals.var_t2_dn8 = assign89920_body9_e137925_d_n8;
            locals.var_t2_dn9 = assign89920_body9_e137925_d_n9;
            locals.var_t2_dn10 = assign89920_body9_e137925_d_n10;
            locals.var_t2_dn13 = assign89920_body9_e137925_d_n13;
            let (assign89920_body10_e137938, assign89920_body10_e137938_d_n0, assign89920_body10_e137938_d_n2, assign89920_body10_e137938_d_n4, assign89920_body10_e137938_d_n5, assign89920_body10_e137938_d_n6, assign89920_body10_e137938_d_n7, assign89920_body10_e137938_d_n8, assign89920_body10_e137938_d_n9, assign89920_body10_e137938_d_n10, assign89920_body10_e137938_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body10_e137933: f64 = (1.0 + locals.var_t2);
        let assign89920_body10_e137934: f64 = (assign89920_body10_e137933).ln();
        let assign89920_body10_e137936: f64 = (assign89920_body10_e137934 / locals.var_c_sb);
        (assign89920_body10_e137936, ((((locals.var_t2_dn0 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign89920_body10_e137933) * locals.var_c_sb) - (assign89920_body10_e137934 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign89920_body10_e137938;
            locals.var_phi_b_dn0 = assign89920_body10_e137938_d_n0;
            locals.var_phi_b_dn2 = assign89920_body10_e137938_d_n2;
            locals.var_phi_b_dn4 = assign89920_body10_e137938_d_n4;
            locals.var_phi_b_dn5 = assign89920_body10_e137938_d_n5;
            locals.var_phi_b_dn6 = assign89920_body10_e137938_d_n6;
            locals.var_phi_b_dn7 = assign89920_body10_e137938_d_n7;
            locals.var_phi_b_dn8 = assign89920_body10_e137938_d_n8;
            locals.var_phi_b_dn9 = assign89920_body10_e137938_d_n9;
            locals.var_phi_b_dn10 = assign89920_body10_e137938_d_n10;
            locals.var_phi_b_dn13 = assign89920_body10_e137938_d_n13;
            let (assign89920_body11_e137950, assign89920_body11_e137950_d_n0, assign89920_body11_e137950_d_n2, assign89920_body11_e137950_d_n4, assign89920_body11_e137950_d_n5, assign89920_body11_e137950_d_n6, assign89920_body11_e137950_d_n7, assign89920_body11_e137950_d_n8, assign89920_body11_e137950_d_n9, assign89920_body11_e137950_d_n10, assign89920_body11_e137950_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 != 0.0)) {
        let assign89920_body11_e137947: f64 = (1.0 + locals.var_t2);
        let assign89920_body11_e137948: f64 = (locals.var_t1 / assign89920_body11_e137947);
        (assign89920_body11_e137948, (((locals.var_t1_dn0 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn0)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn2 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn2)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn4 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn4)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn5 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn5)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn6 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn6)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn7 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn7)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn8 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn8)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn9 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn9)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn10 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn10)) / (assign89920_body11_e137947 * assign89920_body11_e137947)), (((locals.var_t1_dn13 * assign89920_body11_e137947) - (locals.var_t1 * locals.var_t2_dn13)) / (assign89920_body11_e137947 * assign89920_body11_e137947)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign89920_body11_e137950;
            locals.var_phi_b_dpss_dn0 = assign89920_body11_e137950_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89920_body11_e137950_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89920_body11_e137950_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89920_body11_e137950_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89920_body11_e137950_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89920_body11_e137950_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89920_body11_e137950_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89920_body11_e137950_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89920_body11_e137950_d_n10;
            locals.var_phi_b_dpss_dn13 = assign89920_body11_e137950_d_n13;
            let (assign89920_body12_e137961, assign89920_body12_e137961_d_n0, assign89920_body12_e137961_d_n2, assign89920_body12_e137961_d_n4, assign89920_body12_e137961_d_n5, assign89920_body12_e137961_d_n6, assign89920_body12_e137961_d_n7, assign89920_body12_e137961_d_n8, assign89920_body12_e137961_d_n9, assign89920_body12_e137961_d_n10, assign89920_body12_e137961_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 == 0.0)) {
        let assign89920_body12_e137959: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign89920_body12_e137959, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign89920_body12_e137961;
            locals.var_phi_b_dn0 = assign89920_body12_e137961_d_n0;
            locals.var_phi_b_dn2 = assign89920_body12_e137961_d_n2;
            locals.var_phi_b_dn4 = assign89920_body12_e137961_d_n4;
            locals.var_phi_b_dn5 = assign89920_body12_e137961_d_n5;
            locals.var_phi_b_dn6 = assign89920_body12_e137961_d_n6;
            locals.var_phi_b_dn7 = assign89920_body12_e137961_d_n7;
            locals.var_phi_b_dn8 = assign89920_body12_e137961_d_n8;
            locals.var_phi_b_dn9 = assign89920_body12_e137961_d_n9;
            locals.var_phi_b_dn10 = assign89920_body12_e137961_d_n10;
            locals.var_phi_b_dn13 = assign89920_body12_e137961_d_n13;
            let (assign89920_body13_e137970, assign89920_body13_e137970_d_n0, assign89920_body13_e137970_d_n2, assign89920_body13_e137970_d_n4, assign89920_body13_e137970_d_n5, assign89920_body13_e137970_d_n6, assign89920_body13_e137970_d_n7, assign89920_body13_e137970_d_n8, assign89920_body13_e137970_d_n9, assign89920_body13_e137970_d_n10, assign89920_body13_e137970_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2090 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign89920_body13_e137970;
            locals.var_phi_b_dpss_dn0 = assign89920_body13_e137970_d_n0;
            locals.var_phi_b_dpss_dn2 = assign89920_body13_e137970_d_n2;
            locals.var_phi_b_dpss_dn4 = assign89920_body13_e137970_d_n4;
            locals.var_phi_b_dpss_dn5 = assign89920_body13_e137970_d_n5;
            locals.var_phi_b_dpss_dn6 = assign89920_body13_e137970_d_n6;
            locals.var_phi_b_dpss_dn7 = assign89920_body13_e137970_d_n7;
            locals.var_phi_b_dpss_dn8 = assign89920_body13_e137970_d_n8;
            locals.var_phi_b_dpss_dn9 = assign89920_body13_e137970_d_n9;
            locals.var_phi_b_dpss_dn10 = assign89920_body13_e137970_d_n10;
            locals.var_phi_b_dpss_dn13 = assign89920_body13_e137970_d_n13;
            let (assign89920_body14_e137978, assign89920_body14_e137978_d_n0, assign89920_body14_e137978_d_n2, assign89920_body14_e137978_d_n4, assign89920_body14_e137978_d_n5, assign89920_body14_e137978_d_n6, assign89920_body14_e137978_d_n7, assign89920_body14_e137978_d_n8, assign89920_body14_e137978_d_n9, assign89920_body14_e137978_d_n10, assign89920_body14_e137978_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body14_e137976: f64 = (locals.var_beta * locals.var_phi_b);
        (assign89920_body14_e137976, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign89920_body14_e137978;
            locals.var_chib_dn0 = assign89920_body14_e137978_d_n0;
            locals.var_chib_dn2 = assign89920_body14_e137978_d_n2;
            locals.var_chib_dn4 = assign89920_body14_e137978_d_n4;
            locals.var_chib_dn5 = assign89920_body14_e137978_d_n5;
            locals.var_chib_dn6 = assign89920_body14_e137978_d_n6;
            locals.var_chib_dn7 = assign89920_body14_e137978_d_n7;
            locals.var_chib_dn8 = assign89920_body14_e137978_d_n8;
            locals.var_chib_dn9 = assign89920_body14_e137978_d_n9;
            locals.var_chib_dn10 = assign89920_body14_e137978_d_n10;
            locals.var_chib_dn13 = assign89920_body14_e137978_d_n13;
            let assign89920_body15_e137980: f64 = (locals.var_chi).abs();
            let assign89920_body15_e137982: f64 = if assign89920_body15_e137980 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard2091 = assign89920_body15_e137982;
            let (assign89920_body17_e138028, assign89920_body17_e138028_d_n0, assign89920_body17_e138028_d_n2, assign89920_body17_e138028_d_n4, assign89920_body17_e138028_d_n5, assign89920_body17_e138028_d_n6, assign89920_body17_e138028_d_n7, assign89920_body17_e138028_d_n8, assign89920_body17_e138028_d_n9, assign89920_body17_e138028_d_n10, assign89920_body17_e138028_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body17_e138006: f64 = (locals.var_chi * locals.var_chi);
        let assign89920_body17_e138008: f64 = (assign89920_body17_e138006 / 2.0);
        let assign89920_body17_e138012: f64 = (locals.var_chi / 3.0);
        let assign89920_body17_e138016: f64 = (locals.var_chi / 4.0);
        let assign89920_body17_e138020: f64 = (locals.var_chi / 5.0);
        let assign89920_body17_e138021: f64 = (1.0 - assign89920_body17_e138020);
        let assign89920_body17_e138022: f64 = (assign89920_body17_e138016 * assign89920_body17_e138021);
        let assign89920_body17_e138023: f64 = (1.0 - assign89920_body17_e138022);
        let assign89920_body17_e138024: f64 = (assign89920_body17_e138012 * assign89920_body17_e138023);
        let assign89920_body17_e138025: f64 = (1.0 - assign89920_body17_e138024);
        let assign89920_body17_e138026: f64 = (assign89920_body17_e138008 * assign89920_body17_e138025);
        (assign89920_body17_e138026, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn0 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn0 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn2 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn2 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn4 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn4 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn5 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn5 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn6 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn6 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn7 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn7 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn8 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn8 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn9 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn9 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn10 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn10 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign89920_body17_e138025) + (assign89920_body17_e138008 * (-(((locals.var_chi_dn13 / 3.0) * assign89920_body17_e138023) + (assign89920_body17_e138012 * (-(((locals.var_chi_dn13 / 4.0) * assign89920_body17_e138021) + (assign89920_body17_e138016 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89920_body17_e138028;
            locals.var_t0_dn0 = assign89920_body17_e138028_d_n0;
            locals.var_t0_dn2 = assign89920_body17_e138028_d_n2;
            locals.var_t0_dn4 = assign89920_body17_e138028_d_n4;
            locals.var_t0_dn5 = assign89920_body17_e138028_d_n5;
            locals.var_t0_dn6 = assign89920_body17_e138028_d_n6;
            locals.var_t0_dn7 = assign89920_body17_e138028_d_n7;
            locals.var_t0_dn8 = assign89920_body17_e138028_d_n8;
            locals.var_t0_dn9 = assign89920_body17_e138028_d_n9;
            locals.var_t0_dn10 = assign89920_body17_e138028_d_n10;
            locals.var_t0_dn13 = assign89920_body17_e138028_d_n13;
            let (assign89920_body18_e138054, assign89920_body18_e138054_d_n0, assign89920_body18_e138054_d_n2, assign89920_body18_e138054_d_n4, assign89920_body18_e138054_d_n5, assign89920_body18_e138054_d_n6, assign89920_body18_e138054_d_n7, assign89920_body18_e138054_d_n8, assign89920_body18_e138054_d_n9, assign89920_body18_e138054_d_n10, assign89920_body18_e138054_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body18_e138038: f64 = (locals.var_chi / 2.0);
        let assign89920_body18_e138042: f64 = (locals.var_chi / 3.0);
        let assign89920_body18_e138046: f64 = (locals.var_chi / 4.0);
        let assign89920_body18_e138047: f64 = (1.0 - assign89920_body18_e138046);
        let assign89920_body18_e138048: f64 = (assign89920_body18_e138042 * assign89920_body18_e138047);
        let assign89920_body18_e138049: f64 = (1.0 - assign89920_body18_e138048);
        let assign89920_body18_e138050: f64 = (assign89920_body18_e138038 * assign89920_body18_e138049);
        let assign89920_body18_e138051: f64 = (1.0 - assign89920_body18_e138050);
        let assign89920_body18_e138052: f64 = (locals.var_chi * assign89920_body18_e138051);
        (assign89920_body18_e138052, ((locals.var_chi_dn0 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn0 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn2 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn4 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn5 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn6 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn7 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn8 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn9 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn10 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign89920_body18_e138051) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign89920_body18_e138049) + (assign89920_body18_e138038 * (-(((locals.var_chi_dn13 / 3.0) * assign89920_body18_e138047) + (assign89920_body18_e138042 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body18_e138054;
            locals.var_t1_dn0 = assign89920_body18_e138054_d_n0;
            locals.var_t1_dn2 = assign89920_body18_e138054_d_n2;
            locals.var_t1_dn4 = assign89920_body18_e138054_d_n4;
            locals.var_t1_dn5 = assign89920_body18_e138054_d_n5;
            locals.var_t1_dn6 = assign89920_body18_e138054_d_n6;
            locals.var_t1_dn7 = assign89920_body18_e138054_d_n7;
            locals.var_t1_dn8 = assign89920_body18_e138054_d_n8;
            locals.var_t1_dn9 = assign89920_body18_e138054_d_n9;
            locals.var_t1_dn10 = assign89920_body18_e138054_d_n10;
            locals.var_t1_dn13 = assign89920_body18_e138054_d_n13;
            let (assign89920_body19_e138084, assign89920_body19_e138084_d_n0, assign89920_body19_e138084_d_n2, assign89920_body19_e138084_d_n4, assign89920_body19_e138084_d_n5, assign89920_body19_e138084_d_n6, assign89920_body19_e138084_d_n7, assign89920_body19_e138084_d_n8, assign89920_body19_e138084_d_n9, assign89920_body19_e138084_d_n10, assign89920_body19_e138084_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body19_e138062: f64 = (locals.var_chib * locals.var_chib);
        let assign89920_body19_e138064: f64 = (assign89920_body19_e138062 / 2.0);
        let assign89920_body19_e138068: f64 = (locals.var_chib / 3.0);
        let assign89920_body19_e138072: f64 = (locals.var_chib / 4.0);
        let assign89920_body19_e138076: f64 = (locals.var_chib / 5.0);
        let assign89920_body19_e138077: f64 = (1.0 - assign89920_body19_e138076);
        let assign89920_body19_e138078: f64 = (assign89920_body19_e138072 * assign89920_body19_e138077);
        let assign89920_body19_e138079: f64 = (1.0 - assign89920_body19_e138078);
        let assign89920_body19_e138080: f64 = (assign89920_body19_e138068 * assign89920_body19_e138079);
        let assign89920_body19_e138081: f64 = (1.0 - assign89920_body19_e138080);
        let assign89920_body19_e138082: f64 = (assign89920_body19_e138064 * assign89920_body19_e138081);
        (assign89920_body19_e138082, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn0 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn0 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn2 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn2 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn4 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn4 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn5 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn5 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn6 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn6 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn7 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn7 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn8 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn8 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn9 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn9 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn10 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn10 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign89920_body19_e138081) + (assign89920_body19_e138064 * (-(((locals.var_chib_dn13 / 3.0) * assign89920_body19_e138079) + (assign89920_body19_e138068 * (-(((locals.var_chib_dn13 / 4.0) * assign89920_body19_e138077) + (assign89920_body19_e138072 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign89920_body19_e138084;
            locals.var_t2_dn0 = assign89920_body19_e138084_d_n0;
            locals.var_t2_dn2 = assign89920_body19_e138084_d_n2;
            locals.var_t2_dn4 = assign89920_body19_e138084_d_n4;
            locals.var_t2_dn5 = assign89920_body19_e138084_d_n5;
            locals.var_t2_dn6 = assign89920_body19_e138084_d_n6;
            locals.var_t2_dn7 = assign89920_body19_e138084_d_n7;
            locals.var_t2_dn8 = assign89920_body19_e138084_d_n8;
            locals.var_t2_dn9 = assign89920_body19_e138084_d_n9;
            locals.var_t2_dn10 = assign89920_body19_e138084_d_n10;
            locals.var_t2_dn13 = assign89920_body19_e138084_d_n13;
            let (assign89920_body20_e138110, assign89920_body20_e138110_d_n0, assign89920_body20_e138110_d_n2, assign89920_body20_e138110_d_n4, assign89920_body20_e138110_d_n5, assign89920_body20_e138110_d_n6, assign89920_body20_e138110_d_n7, assign89920_body20_e138110_d_n8, assign89920_body20_e138110_d_n9, assign89920_body20_e138110_d_n10, assign89920_body20_e138110_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body20_e138094: f64 = (locals.var_chib / 2.0);
        let assign89920_body20_e138098: f64 = (locals.var_chib / 3.0);
        let assign89920_body20_e138102: f64 = (locals.var_chib / 4.0);
        let assign89920_body20_e138103: f64 = (1.0 - assign89920_body20_e138102);
        let assign89920_body20_e138104: f64 = (assign89920_body20_e138098 * assign89920_body20_e138103);
        let assign89920_body20_e138105: f64 = (1.0 - assign89920_body20_e138104);
        let assign89920_body20_e138106: f64 = (assign89920_body20_e138094 * assign89920_body20_e138105);
        let assign89920_body20_e138107: f64 = (1.0 - assign89920_body20_e138106);
        let assign89920_body20_e138108: f64 = (locals.var_chib * assign89920_body20_e138107);
        (assign89920_body20_e138108, ((locals.var_chib_dn0 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn0 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn2 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn4 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn5 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn6 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn7 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn8 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn9 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn10 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign89920_body20_e138107) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign89920_body20_e138105) + (assign89920_body20_e138094 * (-(((locals.var_chib_dn13 / 3.0) * assign89920_body20_e138103) + (assign89920_body20_e138098 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign89920_body20_e138110;
            locals.var_t3_dn0 = assign89920_body20_e138110_d_n0;
            locals.var_t3_dn2 = assign89920_body20_e138110_d_n2;
            locals.var_t3_dn4 = assign89920_body20_e138110_d_n4;
            locals.var_t3_dn5 = assign89920_body20_e138110_d_n5;
            locals.var_t3_dn6 = assign89920_body20_e138110_d_n6;
            locals.var_t3_dn7 = assign89920_body20_e138110_d_n7;
            locals.var_t3_dn8 = assign89920_body20_e138110_d_n8;
            locals.var_t3_dn9 = assign89920_body20_e138110_d_n9;
            locals.var_t3_dn10 = assign89920_body20_e138110_d_n10;
            locals.var_t3_dn13 = assign89920_body20_e138110_d_n13;
            let (assign89920_body21_e138120, assign89920_body21_e138120_d_n0, assign89920_body21_e138120_d_n2, assign89920_body21_e138120_d_n4, assign89920_body21_e138120_d_n5, assign89920_body21_e138120_d_n6, assign89920_body21_e138120_d_n7, assign89920_body21_e138120_d_n8, assign89920_body21_e138120_d_n9, assign89920_body21_e138120_d_n10, assign89920_body21_e138120_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body21_e138118: f64 = (locals.var_t0 - locals.var_t2);
        (assign89920_body21_e138118, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq__blk2012, locals.var_fbsq__blk2012_dn0, locals.var_fbsq__blk2012_dn2, locals.var_fbsq__blk2012_dn4, locals.var_fbsq__blk2012_dn5, locals.var_fbsq__blk2012_dn6, locals.var_fbsq__blk2012_dn7, locals.var_fbsq__blk2012_dn8, locals.var_fbsq__blk2012_dn9, locals.var_fbsq__blk2012_dn10, locals.var_fbsq__blk2012_dn13,)
    }
};
            locals.var_fbsq__blk2012 = assign89920_body21_e138120;
            locals.var_fbsq__blk2012_dn0 = assign89920_body21_e138120_d_n0;
            locals.var_fbsq__blk2012_dn2 = assign89920_body21_e138120_d_n2;
            locals.var_fbsq__blk2012_dn4 = assign89920_body21_e138120_d_n4;
            locals.var_fbsq__blk2012_dn5 = assign89920_body21_e138120_d_n5;
            locals.var_fbsq__blk2012_dn6 = assign89920_body21_e138120_d_n6;
            locals.var_fbsq__blk2012_dn7 = assign89920_body21_e138120_d_n7;
            locals.var_fbsq__blk2012_dn8 = assign89920_body21_e138120_d_n8;
            locals.var_fbsq__blk2012_dn9 = assign89920_body21_e138120_d_n9;
            locals.var_fbsq__blk2012_dn10 = assign89920_body21_e138120_d_n10;
            locals.var_fbsq__blk2012_dn13 = assign89920_body21_e138120_d_n13;
            let (assign89920_body22_e138134, assign89920_body22_e138134_d_n0, assign89920_body22_e138134_d_n2, assign89920_body22_e138134_d_n4, assign89920_body22_e138134_d_n5, assign89920_body22_e138134_d_n6, assign89920_body22_e138134_d_n7, assign89920_body22_e138134_d_n8, assign89920_body22_e138134_d_n9, assign89920_body22_e138134_d_n10, assign89920_body22_e138134_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 != 0.0)) {
        let assign89920_body22_e138130: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign89920_body22_e138131: f64 = (locals.var_t1 - assign89920_body22_e138130);
        let assign89920_body22_e138132: f64 = (locals.var_beta * assign89920_body22_e138131);
        (assign89920_body22_e138132, ((locals.var_beta_dn0 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign89920_body22_e138131) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss__blk2013, locals.var_fbsq_dpss__blk2013_dn0, locals.var_fbsq_dpss__blk2013_dn2, locals.var_fbsq_dpss__blk2013_dn4, locals.var_fbsq_dpss__blk2013_dn5, locals.var_fbsq_dpss__blk2013_dn6, locals.var_fbsq_dpss__blk2013_dn7, locals.var_fbsq_dpss__blk2013_dn8, locals.var_fbsq_dpss__blk2013_dn9, locals.var_fbsq_dpss__blk2013_dn10, locals.var_fbsq_dpss__blk2013_dn13,)
    }
};
            locals.var_fbsq_dpss__blk2013 = assign89920_body22_e138134;
            locals.var_fbsq_dpss__blk2013_dn0 = assign89920_body22_e138134_d_n0;
            locals.var_fbsq_dpss__blk2013_dn2 = assign89920_body22_e138134_d_n2;
            locals.var_fbsq_dpss__blk2013_dn4 = assign89920_body22_e138134_d_n4;
            locals.var_fbsq_dpss__blk2013_dn5 = assign89920_body22_e138134_d_n5;
            locals.var_fbsq_dpss__blk2013_dn6 = assign89920_body22_e138134_d_n6;
            locals.var_fbsq_dpss__blk2013_dn7 = assign89920_body22_e138134_d_n7;
            locals.var_fbsq_dpss__blk2013_dn8 = assign89920_body22_e138134_d_n8;
            locals.var_fbsq_dpss__blk2013_dn9 = assign89920_body22_e138134_d_n9;
            locals.var_fbsq_dpss__blk2013_dn10 = assign89920_body22_e138134_d_n10;
            locals.var_fbsq_dpss__blk2013_dn13 = assign89920_body22_e138134_d_n13;
            let (assign89920_body24_e138162, assign89920_body24_e138162_d_n0, assign89920_body24_e138162_d_n2, assign89920_body24_e138162_d_n4, assign89920_body24_e138162_d_n5, assign89920_body24_e138162_d_n6, assign89920_body24_e138162_d_n7, assign89920_body24_e138162_d_n8, assign89920_body24_e138162_d_n9, assign89920_body24_e138162_d_n10, assign89920_body24_e138162_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 == 0.0)) {
        let assign89920_body24_e138159: f64 = (-locals.var_chi);
        let assign89920_body24_e138160: f64 = (assign89920_body24_e138159).exp();
        (assign89920_body24_e138160, (assign89920_body24_e138160 * (-locals.var_chi_dn0)), (assign89920_body24_e138160 * (-locals.var_chi_dn2)), (assign89920_body24_e138160 * (-locals.var_chi_dn4)), (assign89920_body24_e138160 * (-locals.var_chi_dn5)), (assign89920_body24_e138160 * (-locals.var_chi_dn6)), (assign89920_body24_e138160 * (-locals.var_chi_dn7)), (assign89920_body24_e138160 * (-locals.var_chi_dn8)), (assign89920_body24_e138160 * (-locals.var_chi_dn9)), (assign89920_body24_e138160 * (-locals.var_chi_dn10)), (assign89920_body24_e138160 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89920_body24_e138162;
            locals.var_t0_dn0 = assign89920_body24_e138162_d_n0;
            locals.var_t0_dn2 = assign89920_body24_e138162_d_n2;
            locals.var_t0_dn4 = assign89920_body24_e138162_d_n4;
            locals.var_t0_dn5 = assign89920_body24_e138162_d_n5;
            locals.var_t0_dn6 = assign89920_body24_e138162_d_n6;
            locals.var_t0_dn7 = assign89920_body24_e138162_d_n7;
            locals.var_t0_dn8 = assign89920_body24_e138162_d_n8;
            locals.var_t0_dn9 = assign89920_body24_e138162_d_n9;
            locals.var_t0_dn10 = assign89920_body24_e138162_d_n10;
            locals.var_t0_dn13 = assign89920_body24_e138162_d_n13;
            let (assign89920_body25_e138173, assign89920_body25_e138173_d_n0, assign89920_body25_e138173_d_n2, assign89920_body25_e138173_d_n4, assign89920_body25_e138173_d_n5, assign89920_body25_e138173_d_n6, assign89920_body25_e138173_d_n7, assign89920_body25_e138173_d_n8, assign89920_body25_e138173_d_n9, assign89920_body25_e138173_d_n10, assign89920_body25_e138173_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 == 0.0)) {
        let assign89920_body25_e138170: f64 = (-locals.var_chib);
        let assign89920_body25_e138171: f64 = (assign89920_body25_e138170).exp();
        (assign89920_body25_e138171, (assign89920_body25_e138171 * (-locals.var_chib_dn0)), (assign89920_body25_e138171 * (-locals.var_chib_dn2)), (assign89920_body25_e138171 * (-locals.var_chib_dn4)), (assign89920_body25_e138171 * (-locals.var_chib_dn5)), (assign89920_body25_e138171 * (-locals.var_chib_dn6)), (assign89920_body25_e138171 * (-locals.var_chib_dn7)), (assign89920_body25_e138171 * (-locals.var_chib_dn8)), (assign89920_body25_e138171 * (-locals.var_chib_dn9)), (assign89920_body25_e138171 * (-locals.var_chib_dn10)), (assign89920_body25_e138171 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body25_e138173;
            locals.var_t1_dn0 = assign89920_body25_e138173_d_n0;
            locals.var_t1_dn2 = assign89920_body25_e138173_d_n2;
            locals.var_t1_dn4 = assign89920_body25_e138173_d_n4;
            locals.var_t1_dn5 = assign89920_body25_e138173_d_n5;
            locals.var_t1_dn6 = assign89920_body25_e138173_d_n6;
            locals.var_t1_dn7 = assign89920_body25_e138173_d_n7;
            locals.var_t1_dn8 = assign89920_body25_e138173_d_n8;
            locals.var_t1_dn9 = assign89920_body25_e138173_d_n9;
            locals.var_t1_dn10 = assign89920_body25_e138173_d_n10;
            locals.var_t1_dn13 = assign89920_body25_e138173_d_n13;
            let (assign89920_body26_e138188, assign89920_body26_e138188_d_n0, assign89920_body26_e138188_d_n2, assign89920_body26_e138188_d_n4, assign89920_body26_e138188_d_n5, assign89920_body26_e138188_d_n6, assign89920_body26_e138188_d_n7, assign89920_body26_e138188_d_n8, assign89920_body26_e138188_d_n9, assign89920_body26_e138188_d_n10, assign89920_body26_e138188_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 == 0.0)) {
        let assign89920_body26_e138182: f64 = (locals.var_chi - locals.var_chib);
        let assign89920_body26_e138185: f64 = (locals.var_t0 - locals.var_t1);
        let assign89920_body26_e138186: f64 = (assign89920_body26_e138182 + assign89920_body26_e138185);
        (assign89920_body26_e138186, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq__blk2012, locals.var_fbsq__blk2012_dn0, locals.var_fbsq__blk2012_dn2, locals.var_fbsq__blk2012_dn4, locals.var_fbsq__blk2012_dn5, locals.var_fbsq__blk2012_dn6, locals.var_fbsq__blk2012_dn7, locals.var_fbsq__blk2012_dn8, locals.var_fbsq__blk2012_dn9, locals.var_fbsq__blk2012_dn10, locals.var_fbsq__blk2012_dn13,)
    }
};
            locals.var_fbsq__blk2012 = assign89920_body26_e138188;
            locals.var_fbsq__blk2012_dn0 = assign89920_body26_e138188_d_n0;
            locals.var_fbsq__blk2012_dn2 = assign89920_body26_e138188_d_n2;
            locals.var_fbsq__blk2012_dn4 = assign89920_body26_e138188_d_n4;
            locals.var_fbsq__blk2012_dn5 = assign89920_body26_e138188_d_n5;
            locals.var_fbsq__blk2012_dn6 = assign89920_body26_e138188_d_n6;
            locals.var_fbsq__blk2012_dn7 = assign89920_body26_e138188_d_n7;
            locals.var_fbsq__blk2012_dn8 = assign89920_body26_e138188_d_n8;
            locals.var_fbsq__blk2012_dn9 = assign89920_body26_e138188_d_n9;
            locals.var_fbsq__blk2012_dn10 = assign89920_body26_e138188_d_n10;
            locals.var_fbsq__blk2012_dn13 = assign89920_body26_e138188_d_n13;
            let (assign89920_body27_e138207, assign89920_body27_e138207_d_n0, assign89920_body27_e138207_d_n2, assign89920_body27_e138207_d_n4, assign89920_body27_e138207_d_n5, assign89920_body27_e138207_d_n6, assign89920_body27_e138207_d_n7, assign89920_body27_e138207_d_n8, assign89920_body27_e138207_d_n9, assign89920_body27_e138207_d_n10, assign89920_body27_e138207_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2091 == 0.0)) {
        let assign89920_body27_e138198: f64 = (1.0 - locals.var_t0);
        let assign89920_body27_e138202: f64 = (1.0 - locals.var_t1);
        let assign89920_body27_e138203: f64 = (locals.var_phi_b_dpss * assign89920_body27_e138202);
        let assign89920_body27_e138204: f64 = (assign89920_body27_e138198 - assign89920_body27_e138203);
        let assign89920_body27_e138205: f64 = (locals.var_beta * assign89920_body27_e138204);
        (assign89920_body27_e138205, ((locals.var_beta_dn0 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign89920_body27_e138204) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign89920_body27_e138202) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss__blk2013, locals.var_fbsq_dpss__blk2013_dn0, locals.var_fbsq_dpss__blk2013_dn2, locals.var_fbsq_dpss__blk2013_dn4, locals.var_fbsq_dpss__blk2013_dn5, locals.var_fbsq_dpss__blk2013_dn6, locals.var_fbsq_dpss__blk2013_dn7, locals.var_fbsq_dpss__blk2013_dn8, locals.var_fbsq_dpss__blk2013_dn9, locals.var_fbsq_dpss__blk2013_dn10, locals.var_fbsq_dpss__blk2013_dn13,)
    }
};
            locals.var_fbsq_dpss__blk2013 = assign89920_body27_e138207;
            locals.var_fbsq_dpss__blk2013_dn0 = assign89920_body27_e138207_d_n0;
            locals.var_fbsq_dpss__blk2013_dn2 = assign89920_body27_e138207_d_n2;
            locals.var_fbsq_dpss__blk2013_dn4 = assign89920_body27_e138207_d_n4;
            locals.var_fbsq_dpss__blk2013_dn5 = assign89920_body27_e138207_d_n5;
            locals.var_fbsq_dpss__blk2013_dn6 = assign89920_body27_e138207_d_n6;
            locals.var_fbsq_dpss__blk2013_dn7 = assign89920_body27_e138207_d_n7;
            locals.var_fbsq_dpss__blk2013_dn8 = assign89920_body27_e138207_d_n8;
            locals.var_fbsq_dpss__blk2013_dn9 = assign89920_body27_e138207_d_n9;
            locals.var_fbsq_dpss__blk2013_dn10 = assign89920_body27_e138207_d_n10;
            locals.var_fbsq_dpss__blk2013_dn13 = assign89920_body27_e138207_d_n13;
            let assign89920_body28_e138209: f64 = (locals.var_chi).abs();
            let assign89920_body28_e138211: f64 = if assign89920_body28_e138209 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard2092 = assign89920_body28_e138211;
            let (assign89920_body29_e138241, assign89920_body29_e138241_d_n0, assign89920_body29_e138241_d_n2, assign89920_body29_e138241_d_n4, assign89920_body29_e138241_d_n5, assign89920_body29_e138241_d_n6, assign89920_body29_e138241_d_n7, assign89920_body29_e138241_d_n8, assign89920_body29_e138241_d_n9, assign89920_body29_e138241_d_n10, assign89920_body29_e138241_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89920_body29_e138219: f64 = (locals.var_chi * locals.var_chi);
        let assign89920_body29_e138221: f64 = (assign89920_body29_e138219 / 2.0);
        let assign89920_body29_e138225: f64 = (locals.var_chi / 3.0);
        let assign89920_body29_e138229: f64 = (locals.var_chi / 4.0);
        let assign89920_body29_e138233: f64 = (locals.var_chi / 5.0);
        let assign89920_body29_e138234: f64 = (1.0 + assign89920_body29_e138233);
        let assign89920_body29_e138235: f64 = (assign89920_body29_e138229 * assign89920_body29_e138234);
        let assign89920_body29_e138236: f64 = (1.0 + assign89920_body29_e138235);
        let assign89920_body29_e138237: f64 = (assign89920_body29_e138225 * assign89920_body29_e138236);
        let assign89920_body29_e138238: f64 = (1.0 + assign89920_body29_e138237);
        let assign89920_body29_e138239: f64 = (assign89920_body29_e138221 * assign89920_body29_e138238);
        (assign89920_body29_e138239, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn0 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn0 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn2 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn2 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn4 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn4 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn5 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn5 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn6 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn6 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn7 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn7 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn8 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn8 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn9 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn9 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn10 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn10 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign89920_body29_e138238) + (assign89920_body29_e138221 * (((locals.var_chi_dn13 / 3.0) * assign89920_body29_e138236) + (assign89920_body29_e138225 * (((locals.var_chi_dn13 / 4.0) * assign89920_body29_e138234) + (assign89920_body29_e138229 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign89920_body29_e138241;
            locals.var_t0_dn0 = assign89920_body29_e138241_d_n0;
            locals.var_t0_dn2 = assign89920_body29_e138241_d_n2;
            locals.var_t0_dn4 = assign89920_body29_e138241_d_n4;
            locals.var_t0_dn5 = assign89920_body29_e138241_d_n5;
            locals.var_t0_dn6 = assign89920_body29_e138241_d_n6;
            locals.var_t0_dn7 = assign89920_body29_e138241_d_n7;
            locals.var_t0_dn8 = assign89920_body29_e138241_d_n8;
            locals.var_t0_dn9 = assign89920_body29_e138241_d_n9;
            locals.var_t0_dn10 = assign89920_body29_e138241_d_n10;
            locals.var_t0_dn13 = assign89920_body29_e138241_d_n13;
            let (assign89920_body30_e138267, assign89920_body30_e138267_d_n0, assign89920_body30_e138267_d_n2, assign89920_body30_e138267_d_n4, assign89920_body30_e138267_d_n5, assign89920_body30_e138267_d_n6, assign89920_body30_e138267_d_n7, assign89920_body30_e138267_d_n8, assign89920_body30_e138267_d_n9, assign89920_body30_e138267_d_n10, assign89920_body30_e138267_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89920_body30_e138251: f64 = (locals.var_chi / 2.0);
        let assign89920_body30_e138255: f64 = (locals.var_chi / 3.0);
        let assign89920_body30_e138259: f64 = (locals.var_chi / 4.0);
        let assign89920_body30_e138260: f64 = (1.0 + assign89920_body30_e138259);
        let assign89920_body30_e138261: f64 = (assign89920_body30_e138255 * assign89920_body30_e138260);
        let assign89920_body30_e138262: f64 = (1.0 + assign89920_body30_e138261);
        let assign89920_body30_e138263: f64 = (assign89920_body30_e138251 * assign89920_body30_e138262);
        let assign89920_body30_e138264: f64 = (1.0 + assign89920_body30_e138263);
        let assign89920_body30_e138265: f64 = (locals.var_chi * assign89920_body30_e138264);
        (assign89920_body30_e138265, ((locals.var_chi_dn0 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn0 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn2 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn4 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn5 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn6 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn7 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn8 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn9 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn10 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign89920_body30_e138264) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign89920_body30_e138262) + (assign89920_body30_e138251 * (((locals.var_chi_dn13 / 3.0) * assign89920_body30_e138260) + (assign89920_body30_e138255 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body30_e138267;
            locals.var_t1_dn0 = assign89920_body30_e138267_d_n0;
            locals.var_t1_dn2 = assign89920_body30_e138267_d_n2;
            locals.var_t1_dn4 = assign89920_body30_e138267_d_n4;
            locals.var_t1_dn5 = assign89920_body30_e138267_d_n5;
            locals.var_t1_dn6 = assign89920_body30_e138267_d_n6;
            locals.var_t1_dn7 = assign89920_body30_e138267_d_n7;
            locals.var_t1_dn8 = assign89920_body30_e138267_d_n8;
            locals.var_t1_dn9 = assign89920_body30_e138267_d_n9;
            locals.var_t1_dn10 = assign89920_body30_e138267_d_n10;
            locals.var_t1_dn13 = assign89920_body30_e138267_d_n13;
            let (assign89920_body31_e138277, assign89920_body31_e138277_d_n0, assign89920_body31_e138277_d_n2, assign89920_body31_e138277_d_n4, assign89920_body31_e138277_d_n5, assign89920_body31_e138277_d_n6, assign89920_body31_e138277_d_n7, assign89920_body31_e138277_d_n8, assign89920_body31_e138277_d_n9, assign89920_body31_e138277_d_n10, assign89920_body31_e138277_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89920_body31_e138275: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign89920_body31_e138275, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89920_body31_e138277;
            locals.var_fs01_dn0 = assign89920_body31_e138277_d_n0;
            locals.var_fs01_dn2 = assign89920_body31_e138277_d_n2;
            locals.var_fs01_dn4 = assign89920_body31_e138277_d_n4;
            locals.var_fs01_dn5 = assign89920_body31_e138277_d_n5;
            locals.var_fs01_dn6 = assign89920_body31_e138277_d_n6;
            locals.var_fs01_dn7 = assign89920_body31_e138277_d_n7;
            locals.var_fs01_dn8 = assign89920_body31_e138277_d_n8;
            locals.var_fs01_dn9 = assign89920_body31_e138277_d_n9;
            locals.var_fs01_dn10 = assign89920_body31_e138277_d_n10;
            locals.var_fs01_dn13 = assign89920_body31_e138277_d_n13;
            let (assign89920_body32_e138289, assign89920_body32_e138289_d_n0, assign89920_body32_e138289_d_n2, assign89920_body32_e138289_d_n4, assign89920_body32_e138289_d_n5, assign89920_body32_e138289_d_n6, assign89920_body32_e138289_d_n7, assign89920_body32_e138289_d_n8, assign89920_body32_e138289_d_n9, assign89920_body32_e138289_d_n10, assign89920_body32_e138289_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 != 0.0)) {
        let assign89920_body32_e138285: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign89920_body32_e138287: f64 = (assign89920_body32_e138285 * locals.var_beta);
        (assign89920_body32_e138287, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign89920_body32_e138285 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89920_body32_e138289;
            locals.var_fs01_dps0_dn0 = assign89920_body32_e138289_d_n0;
            locals.var_fs01_dps0_dn2 = assign89920_body32_e138289_d_n2;
            locals.var_fs01_dps0_dn4 = assign89920_body32_e138289_d_n4;
            locals.var_fs01_dps0_dn5 = assign89920_body32_e138289_d_n5;
            locals.var_fs01_dps0_dn6 = assign89920_body32_e138289_d_n6;
            locals.var_fs01_dps0_dn7 = assign89920_body32_e138289_d_n7;
            locals.var_fs01_dps0_dn8 = assign89920_body32_e138289_d_n8;
            locals.var_fs01_dps0_dn9 = assign89920_body32_e138289_d_n9;
            locals.var_fs01_dps0_dn10 = assign89920_body32_e138289_d_n10;
            locals.var_fs01_dps0_dn13 = assign89920_body32_e138289_d_n13;
            let assign89920_body33_e138291: f64 = (locals.var_chi).abs();
            let assign89920_body33_e138293: f64 = if assign89920_body33_e138291 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard2093 = assign89920_body33_e138293;
            let (assign89920_body35_e138324, assign89920_body35_e138324_d_n0, assign89920_body35_e138324_d_n2, assign89920_body35_e138324_d_n4, assign89920_body35_e138324_d_n5, assign89920_body35_e138324_d_n6, assign89920_body35_e138324_d_n7, assign89920_body35_e138324_d_n8, assign89920_body35_e138324_d_n9, assign89920_body35_e138324_d_n10, assign89920_body35_e138324_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89920_body35_e138322: f64 = (locals.var_chi).exp();
        (assign89920_body35_e138322, (assign89920_body35_e138322 * locals.var_chi_dn0), (assign89920_body35_e138322 * locals.var_chi_dn2), (assign89920_body35_e138322 * locals.var_chi_dn4), (assign89920_body35_e138322 * locals.var_chi_dn5), (assign89920_body35_e138322 * locals.var_chi_dn6), (assign89920_body35_e138322 * locals.var_chi_dn7), (assign89920_body35_e138322 * locals.var_chi_dn8), (assign89920_body35_e138322 * locals.var_chi_dn9), (assign89920_body35_e138322 * locals.var_chi_dn10), (assign89920_body35_e138322 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign89920_body35_e138324;
            locals.var_exp_chi_dn0 = assign89920_body35_e138324_d_n0;
            locals.var_exp_chi_dn2 = assign89920_body35_e138324_d_n2;
            locals.var_exp_chi_dn4 = assign89920_body35_e138324_d_n4;
            locals.var_exp_chi_dn5 = assign89920_body35_e138324_d_n5;
            locals.var_exp_chi_dn6 = assign89920_body35_e138324_d_n6;
            locals.var_exp_chi_dn7 = assign89920_body35_e138324_d_n7;
            locals.var_exp_chi_dn8 = assign89920_body35_e138324_d_n8;
            locals.var_exp_chi_dn9 = assign89920_body35_e138324_d_n9;
            locals.var_exp_chi_dn10 = assign89920_body35_e138324_d_n10;
            locals.var_exp_chi_dn13 = assign89920_body35_e138324_d_n13;
            let (assign89920_body36_e138337, assign89920_body36_e138337_d_n0, assign89920_body36_e138337_d_n2, assign89920_body36_e138337_d_n4, assign89920_body36_e138337_d_n5, assign89920_body36_e138337_d_n6, assign89920_body36_e138337_d_n7, assign89920_body36_e138337_d_n8, assign89920_body36_e138337_d_n9, assign89920_body36_e138337_d_n10, assign89920_body36_e138337_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89920_body36_e138335: f64 = (locals.var_exp_chi - 1.0);
        (assign89920_body36_e138335, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign89920_body36_e138337;
            locals.var_t1_dn0 = assign89920_body36_e138337_d_n0;
            locals.var_t1_dn2 = assign89920_body36_e138337_d_n2;
            locals.var_t1_dn4 = assign89920_body36_e138337_d_n4;
            locals.var_t1_dn5 = assign89920_body36_e138337_d_n5;
            locals.var_t1_dn6 = assign89920_body36_e138337_d_n6;
            locals.var_t1_dn7 = assign89920_body36_e138337_d_n7;
            locals.var_t1_dn8 = assign89920_body36_e138337_d_n8;
            locals.var_t1_dn9 = assign89920_body36_e138337_d_n9;
            locals.var_t1_dn10 = assign89920_body36_e138337_d_n10;
            locals.var_t1_dn13 = assign89920_body36_e138337_d_n13;
            let (assign89920_body37_e138352, assign89920_body37_e138352_d_n0, assign89920_body37_e138352_d_n2, assign89920_body37_e138352_d_n4, assign89920_body37_e138352_d_n5, assign89920_body37_e138352_d_n6, assign89920_body37_e138352_d_n7, assign89920_body37_e138352_d_n8, assign89920_body37_e138352_d_n9, assign89920_body37_e138352_d_n10, assign89920_body37_e138352_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89920_body37_e138349: f64 = (locals.var_t1 - locals.var_chi);
        let assign89920_body37_e138350: f64 = (locals.var_cfs1 * assign89920_body37_e138349);
        (assign89920_body37_e138350, ((locals.var_cfs1_dn0 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign89920_body37_e138349) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89920_body37_e138352;
            locals.var_fs01_dn0 = assign89920_body37_e138352_d_n0;
            locals.var_fs01_dn2 = assign89920_body37_e138352_d_n2;
            locals.var_fs01_dn4 = assign89920_body37_e138352_d_n4;
            locals.var_fs01_dn5 = assign89920_body37_e138352_d_n5;
            locals.var_fs01_dn6 = assign89920_body37_e138352_d_n6;
            locals.var_fs01_dn7 = assign89920_body37_e138352_d_n7;
            locals.var_fs01_dn8 = assign89920_body37_e138352_d_n8;
            locals.var_fs01_dn9 = assign89920_body37_e138352_d_n9;
            locals.var_fs01_dn10 = assign89920_body37_e138352_d_n10;
            locals.var_fs01_dn13 = assign89920_body37_e138352_d_n13;
            let (assign89920_body38_e138367, assign89920_body38_e138367_d_n0, assign89920_body38_e138367_d_n2, assign89920_body38_e138367_d_n4, assign89920_body38_e138367_d_n5, assign89920_body38_e138367_d_n6, assign89920_body38_e138367_d_n7, assign89920_body38_e138367_d_n8, assign89920_body38_e138367_d_n9, assign89920_body38_e138367_d_n10, assign89920_body38_e138367_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 != 0.0)) {
        let assign89920_body38_e138363: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign89920_body38_e138365: f64 = (assign89920_body38_e138363 * locals.var_t1);
        (assign89920_body38_e138365, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign89920_body38_e138363 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89920_body38_e138367;
            locals.var_fs01_dps0_dn0 = assign89920_body38_e138367_d_n0;
            locals.var_fs01_dps0_dn2 = assign89920_body38_e138367_d_n2;
            locals.var_fs01_dps0_dn4 = assign89920_body38_e138367_d_n4;
            locals.var_fs01_dps0_dn5 = assign89920_body38_e138367_d_n5;
            locals.var_fs01_dps0_dn6 = assign89920_body38_e138367_d_n6;
            locals.var_fs01_dps0_dn7 = assign89920_body38_e138367_d_n7;
            locals.var_fs01_dps0_dn8 = assign89920_body38_e138367_d_n8;
            locals.var_fs01_dps0_dn9 = assign89920_body38_e138367_d_n9;
            locals.var_fs01_dps0_dn10 = assign89920_body38_e138367_d_n10;
            locals.var_fs01_dps0_dn13 = assign89920_body38_e138367_d_n13;
            let (assign89920_body40_e138402, assign89920_body40_e138402_d_n0, assign89920_body40_e138402_d_n2, assign89920_body40_e138402_d_n4, assign89920_body40_e138402_d_n5, assign89920_body40_e138402_d_n6, assign89920_body40_e138402_d_n7, assign89920_body40_e138402_d_n8, assign89920_body40_e138402_d_n9, assign89920_body40_e138402_d_n10, assign89920_body40_e138402_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89920_body40_e138399: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign89920_body40_e138400: f64 = (assign89920_body40_e138399).exp();
        (assign89920_body40_e138400, (assign89920_body40_e138400 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign89920_body40_e138400 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign89920_body40_e138400 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign89920_body40_e138400 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign89920_body40_e138400 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign89920_body40_e138400 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign89920_body40_e138400 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign89920_body40_e138400 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign89920_body40_e138400 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign89920_body40_e138400 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign89920_body40_e138402;
            locals.var_exp_bps0_dn0 = assign89920_body40_e138402_d_n0;
            locals.var_exp_bps0_dn2 = assign89920_body40_e138402_d_n2;
            locals.var_exp_bps0_dn4 = assign89920_body40_e138402_d_n4;
            locals.var_exp_bps0_dn5 = assign89920_body40_e138402_d_n5;
            locals.var_exp_bps0_dn6 = assign89920_body40_e138402_d_n6;
            locals.var_exp_bps0_dn7 = assign89920_body40_e138402_d_n7;
            locals.var_exp_bps0_dn8 = assign89920_body40_e138402_d_n8;
            locals.var_exp_bps0_dn9 = assign89920_body40_e138402_d_n9;
            locals.var_exp_bps0_dn10 = assign89920_body40_e138402_d_n10;
            locals.var_exp_bps0_dn13 = assign89920_body40_e138402_d_n13;
            let (assign89920_body41_e138422, assign89920_body41_e138422_d_n0, assign89920_body41_e138422_d_n2, assign89920_body41_e138422_d_n4, assign89920_body41_e138422_d_n5, assign89920_body41_e138422_d_n6, assign89920_body41_e138422_d_n7, assign89920_body41_e138422_d_n8, assign89920_body41_e138422_d_n9, assign89920_body41_e138422_d_n10, assign89920_body41_e138422_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89920_body41_e138417: f64 = (locals.var_chi + 1.0);
        let assign89920_body41_e138418: f64 = (locals.var_exp_bvbs * assign89920_body41_e138417);
        let assign89920_body41_e138419: f64 = (locals.var_exp_bps0 - assign89920_body41_e138418);
        let assign89920_body41_e138420: f64 = (locals.var_cnst1over * assign89920_body41_e138419);
        (assign89920_body41_e138420, ((locals.var_cnst1over_dn0 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign89920_body41_e138419) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign89920_body41_e138417) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign89920_body41_e138422;
            locals.var_fs01_dn0 = assign89920_body41_e138422_d_n0;
            locals.var_fs01_dn2 = assign89920_body41_e138422_d_n2;
            locals.var_fs01_dn4 = assign89920_body41_e138422_d_n4;
            locals.var_fs01_dn5 = assign89920_body41_e138422_d_n5;
            locals.var_fs01_dn6 = assign89920_body41_e138422_d_n6;
            locals.var_fs01_dn7 = assign89920_body41_e138422_d_n7;
            locals.var_fs01_dn8 = assign89920_body41_e138422_d_n8;
            locals.var_fs01_dn9 = assign89920_body41_e138422_d_n9;
            locals.var_fs01_dn10 = assign89920_body41_e138422_d_n10;
            locals.var_fs01_dn13 = assign89920_body41_e138422_d_n13;
            let (assign89920_body42_e138440, assign89920_body42_e138440_d_n0, assign89920_body42_e138440_d_n2, assign89920_body42_e138440_d_n4, assign89920_body42_e138440_d_n5, assign89920_body42_e138440_d_n6, assign89920_body42_e138440_d_n7, assign89920_body42_e138440_d_n8, assign89920_body42_e138440_d_n9, assign89920_body42_e138440_d_n10, assign89920_body42_e138440_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2092 == 0.0)) && (locals.var_guard2093 == 0.0)) {
        let assign89920_body42_e138434: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign89920_body42_e138437: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign89920_body42_e138438: f64 = (assign89920_body42_e138434 * assign89920_body42_e138437);
        (assign89920_body42_e138438, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign89920_body42_e138437) + (assign89920_body42_e138434 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign89920_body42_e138440;
            locals.var_fs01_dps0_dn0 = assign89920_body42_e138440_d_n0;
            locals.var_fs01_dps0_dn2 = assign89920_body42_e138440_d_n2;
            locals.var_fs01_dps0_dn4 = assign89920_body42_e138440_d_n4;
            locals.var_fs01_dps0_dn5 = assign89920_body42_e138440_d_n5;
            locals.var_fs01_dps0_dn6 = assign89920_body42_e138440_d_n6;
            locals.var_fs01_dps0_dn7 = assign89920_body42_e138440_d_n7;
            locals.var_fs01_dps0_dn8 = assign89920_body42_e138440_d_n8;
            locals.var_fs01_dps0_dn9 = assign89920_body42_e138440_d_n9;
            locals.var_fs01_dps0_dn10 = assign89920_body42_e138440_d_n10;
            locals.var_fs01_dps0_dn13 = assign89920_body42_e138440_d_n13;
            let assign89920_body43_e138443: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2094 = assign89920_body43_e138443;
            let (assign89920_body44_e138454, assign89920_body44_e138454_d_n0, assign89920_body44_e138454_d_n2, assign89920_body44_e138454_d_n4, assign89920_body44_e138454_d_n5, assign89920_body44_e138454_d_n6, assign89920_body44_e138454_d_n7, assign89920_body44_e138454_d_n8, assign89920_body44_e138454_d_n9, assign89920_body44_e138454_d_n10, assign89920_body44_e138454_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89920_body44_e138451: f64 = (locals.var_fbsq__blk2012 + locals.var_fs01);
        let assign89920_body44_e138452: f64 = (assign89920_body44_e138451).sqrt();
        (assign89920_body44_e138452, ((locals.var_fbsq__blk2012_dn0 + locals.var_fs01_dn0) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn2 + locals.var_fs01_dn2) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn4 + locals.var_fs01_dn4) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn5 + locals.var_fs01_dn5) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn6 + locals.var_fs01_dn6) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn7 + locals.var_fs01_dn7) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn8 + locals.var_fs01_dn8) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn9 + locals.var_fs01_dn9) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn10 + locals.var_fs01_dn10) / (2.0 * assign89920_body44_e138452)), ((locals.var_fbsq__blk2012_dn13 + locals.var_fs01_dn13) / (2.0 * assign89920_body44_e138452)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89920_body44_e138454;
            locals.var_fs02_dn0 = assign89920_body44_e138454_d_n0;
            locals.var_fs02_dn2 = assign89920_body44_e138454_d_n2;
            locals.var_fs02_dn4 = assign89920_body44_e138454_d_n4;
            locals.var_fs02_dn5 = assign89920_body44_e138454_d_n5;
            locals.var_fs02_dn6 = assign89920_body44_e138454_d_n6;
            locals.var_fs02_dn7 = assign89920_body44_e138454_d_n7;
            locals.var_fs02_dn8 = assign89920_body44_e138454_d_n8;
            locals.var_fs02_dn9 = assign89920_body44_e138454_d_n9;
            locals.var_fs02_dn10 = assign89920_body44_e138454_d_n10;
            locals.var_fs02_dn13 = assign89920_body44_e138454_d_n13;
            let (assign89920_body45_e138468, assign89920_body45_e138468_d_n0, assign89920_body45_e138468_d_n2, assign89920_body45_e138468_d_n4, assign89920_body45_e138468_d_n5, assign89920_body45_e138468_d_n6, assign89920_body45_e138468_d_n7, assign89920_body45_e138468_d_n8, assign89920_body45_e138468_d_n9, assign89920_body45_e138468_d_n10, assign89920_body45_e138468_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 != 0.0)) {
        let assign89920_body45_e138463: f64 = (locals.var_fbsq_dpss__blk2013 + locals.var_fs01_dps0);
        let assign89920_body45_e138464: f64 = (0.5 * assign89920_body45_e138463);
        let assign89920_body45_e138466: f64 = (assign89920_body45_e138464 / locals.var_fs02);
        (assign89920_body45_e138466, ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk2013_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign89920_body45_e138464 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89920_body45_e138468;
            locals.var_fs02_dps0_dn0 = assign89920_body45_e138468_d_n0;
            locals.var_fs02_dps0_dn2 = assign89920_body45_e138468_d_n2;
            locals.var_fs02_dps0_dn4 = assign89920_body45_e138468_d_n4;
            locals.var_fs02_dps0_dn5 = assign89920_body45_e138468_d_n5;
            locals.var_fs02_dps0_dn6 = assign89920_body45_e138468_d_n6;
            locals.var_fs02_dps0_dn7 = assign89920_body45_e138468_d_n7;
            locals.var_fs02_dps0_dn8 = assign89920_body45_e138468_d_n8;
            locals.var_fs02_dps0_dn9 = assign89920_body45_e138468_d_n9;
            locals.var_fs02_dps0_dn10 = assign89920_body45_e138468_d_n10;
            locals.var_fs02_dps0_dn13 = assign89920_body45_e138468_d_n13;
            let assign89920_body46_e138471: f64 = if locals.var_fbsq__blk2012 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2095 = assign89920_body46_e138471;
            let (assign89920_body47_e138483, assign89920_body47_e138483_d_n0, assign89920_body47_e138483_d_n2, assign89920_body47_e138483_d_n4, assign89920_body47_e138483_d_n5, assign89920_body47_e138483_d_n6, assign89920_body47_e138483_d_n7, assign89920_body47_e138483_d_n8, assign89920_body47_e138483_d_n9, assign89920_body47_e138483_d_n10, assign89920_body47_e138483_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89920_body47_e138481: f64 = (locals.var_fbsq__blk2012).sqrt();
        (assign89920_body47_e138481, (locals.var_fbsq__blk2012_dn0 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn2 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn4 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn5 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn6 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn7 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn8 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn9 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn10 / (2.0 * assign89920_body47_e138481)), (locals.var_fbsq__blk2012_dn13 / (2.0 * assign89920_body47_e138481)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89920_body47_e138483;
            locals.var_fs02_dn0 = assign89920_body47_e138483_d_n0;
            locals.var_fs02_dn2 = assign89920_body47_e138483_d_n2;
            locals.var_fs02_dn4 = assign89920_body47_e138483_d_n4;
            locals.var_fs02_dn5 = assign89920_body47_e138483_d_n5;
            locals.var_fs02_dn6 = assign89920_body47_e138483_d_n6;
            locals.var_fs02_dn7 = assign89920_body47_e138483_d_n7;
            locals.var_fs02_dn8 = assign89920_body47_e138483_d_n8;
            locals.var_fs02_dn9 = assign89920_body47_e138483_d_n9;
            locals.var_fs02_dn10 = assign89920_body47_e138483_d_n10;
            locals.var_fs02_dn13 = assign89920_body47_e138483_d_n13;
            let (assign89920_body48_e138498, assign89920_body48_e138498_d_n0, assign89920_body48_e138498_d_n2, assign89920_body48_e138498_d_n4, assign89920_body48_e138498_d_n5, assign89920_body48_e138498_d_n6, assign89920_body48_e138498_d_n7, assign89920_body48_e138498_d_n8, assign89920_body48_e138498_d_n9, assign89920_body48_e138498_d_n10, assign89920_body48_e138498_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 != 0.0)) {
        let assign89920_body48_e138494: f64 = (0.5 * locals.var_fbsq_dpss__blk2013);
        let assign89920_body48_e138496: f64 = (assign89920_body48_e138494 / locals.var_fs02);
        (assign89920_body48_e138496, ((((0.5 * locals.var_fbsq_dpss__blk2013_dn0) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn2) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn4) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn5) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn6) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn7) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn8) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn9) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn10) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk2013_dn13) * locals.var_fs02) - (assign89920_body48_e138494 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89920_body48_e138498;
            locals.var_fs02_dps0_dn0 = assign89920_body48_e138498_d_n0;
            locals.var_fs02_dps0_dn2 = assign89920_body48_e138498_d_n2;
            locals.var_fs02_dps0_dn4 = assign89920_body48_e138498_d_n4;
            locals.var_fs02_dps0_dn5 = assign89920_body48_e138498_d_n5;
            locals.var_fs02_dps0_dn6 = assign89920_body48_e138498_d_n6;
            locals.var_fs02_dps0_dn7 = assign89920_body48_e138498_d_n7;
            locals.var_fs02_dps0_dn8 = assign89920_body48_e138498_d_n8;
            locals.var_fs02_dps0_dn9 = assign89920_body48_e138498_d_n9;
            locals.var_fs02_dps0_dn10 = assign89920_body48_e138498_d_n10;
            locals.var_fs02_dps0_dn13 = assign89920_body48_e138498_d_n13;
            let (assign89920_body49_e138510, assign89920_body49_e138510_d_n0, assign89920_body49_e138510_d_n2, assign89920_body49_e138510_d_n4, assign89920_body49_e138510_d_n5, assign89920_body49_e138510_d_n6, assign89920_body49_e138510_d_n7, assign89920_body49_e138510_d_n8, assign89920_body49_e138510_d_n9, assign89920_body49_e138510_d_n10, assign89920_body49_e138510_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89920_body49_e138510;
            locals.var_fs02_dn0 = assign89920_body49_e138510_d_n0;
            locals.var_fs02_dn2 = assign89920_body49_e138510_d_n2;
            locals.var_fs02_dn4 = assign89920_body49_e138510_d_n4;
            locals.var_fs02_dn5 = assign89920_body49_e138510_d_n5;
            locals.var_fs02_dn6 = assign89920_body49_e138510_d_n6;
            locals.var_fs02_dn7 = assign89920_body49_e138510_d_n7;
            locals.var_fs02_dn8 = assign89920_body49_e138510_d_n8;
            locals.var_fs02_dn9 = assign89920_body49_e138510_d_n9;
            locals.var_fs02_dn10 = assign89920_body49_e138510_d_n10;
            locals.var_fs02_dn13 = assign89920_body49_e138510_d_n13;
            let (assign89920_body50_e138522, assign89920_body50_e138522_d_n0, assign89920_body50_e138522_d_n2, assign89920_body50_e138522_d_n4, assign89920_body50_e138522_d_n5, assign89920_body50_e138522_d_n6, assign89920_body50_e138522_d_n7, assign89920_body50_e138522_d_n8, assign89920_body50_e138522_d_n9, assign89920_body50_e138522_d_n10, assign89920_body50_e138522_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2094 == 0.0)) && (locals.var_guard2095 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89920_body50_e138522;
            locals.var_fs02_dps0_dn0 = assign89920_body50_e138522_d_n0;
            locals.var_fs02_dps0_dn2 = assign89920_body50_e138522_d_n2;
            locals.var_fs02_dps0_dn4 = assign89920_body50_e138522_d_n4;
            locals.var_fs02_dps0_dn5 = assign89920_body50_e138522_d_n5;
            locals.var_fs02_dps0_dn6 = assign89920_body50_e138522_d_n6;
            locals.var_fs02_dps0_dn7 = assign89920_body50_e138522_d_n7;
            locals.var_fs02_dps0_dn8 = assign89920_body50_e138522_d_n8;
            locals.var_fs02_dps0_dn9 = assign89920_body50_e138522_d_n9;
            locals.var_fs02_dps0_dn10 = assign89920_body50_e138522_d_n10;
            locals.var_fs02_dps0_dn13 = assign89920_body50_e138522_d_n13;
            let (assign89920_body51_e138536, assign89920_body51_e138536_d_n0, assign89920_body51_e138536_d_n2, assign89920_body51_e138536_d_n4, assign89920_body51_e138536_d_n5, assign89920_body51_e138536_d_n6, assign89920_body51_e138536_d_n7, assign89920_body51_e138536_d_n8, assign89920_body51_e138536_d_n9, assign89920_body51_e138536_d_n10, assign89920_body51_e138536_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let (assign89920_body51_e138532,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign89920_body51_e138531: f64 = (-1.0);
                (assign89920_body51_e138531,)
            }
        };
        let assign89920_body51_e138534: f64 = (assign89920_body51_e138532 * locals.var_fs02);
        (assign89920_body51_e138534, (assign89920_body51_e138532 * locals.var_fs02_dn0), (assign89920_body51_e138532 * locals.var_fs02_dn2), (assign89920_body51_e138532 * locals.var_fs02_dn4), (assign89920_body51_e138532 * locals.var_fs02_dn5), (assign89920_body51_e138532 * locals.var_fs02_dn6), (assign89920_body51_e138532 * locals.var_fs02_dn7), (assign89920_body51_e138532 * locals.var_fs02_dn8), (assign89920_body51_e138532 * locals.var_fs02_dn9), (assign89920_body51_e138532 * locals.var_fs02_dn10), (assign89920_body51_e138532 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign89920_body51_e138536;
            locals.var_fs02_dn0 = assign89920_body51_e138536_d_n0;
            locals.var_fs02_dn2 = assign89920_body51_e138536_d_n2;
            locals.var_fs02_dn4 = assign89920_body51_e138536_d_n4;
            locals.var_fs02_dn5 = assign89920_body51_e138536_d_n5;
            locals.var_fs02_dn6 = assign89920_body51_e138536_d_n6;
            locals.var_fs02_dn7 = assign89920_body51_e138536_d_n7;
            locals.var_fs02_dn8 = assign89920_body51_e138536_d_n8;
            locals.var_fs02_dn9 = assign89920_body51_e138536_d_n9;
            locals.var_fs02_dn10 = assign89920_body51_e138536_d_n10;
            locals.var_fs02_dn13 = assign89920_body51_e138536_d_n13;
            let (assign89920_body52_e138550, assign89920_body52_e138550_d_n0, assign89920_body52_e138550_d_n2, assign89920_body52_e138550_d_n4, assign89920_body52_e138550_d_n5, assign89920_body52_e138550_d_n6, assign89920_body52_e138550_d_n7, assign89920_body52_e138550_d_n8, assign89920_body52_e138550_d_n9, assign89920_body52_e138550_d_n10, assign89920_body52_e138550_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let (assign89920_body52_e138546,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign89920_body52_e138545: f64 = (-1.0);
                (assign89920_body52_e138545,)
            }
        };
        let assign89920_body52_e138548: f64 = (assign89920_body52_e138546 * locals.var_fs02_dps0);
        (assign89920_body52_e138548, (assign89920_body52_e138546 * locals.var_fs02_dps0_dn0), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn2), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn4), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn5), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn6), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn7), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn8), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn9), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn10), (assign89920_body52_e138546 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign89920_body52_e138550;
            locals.var_fs02_dps0_dn0 = assign89920_body52_e138550_d_n0;
            locals.var_fs02_dps0_dn2 = assign89920_body52_e138550_d_n2;
            locals.var_fs02_dps0_dn4 = assign89920_body52_e138550_d_n4;
            locals.var_fs02_dps0_dn5 = assign89920_body52_e138550_d_n5;
            locals.var_fs02_dps0_dn6 = assign89920_body52_e138550_d_n6;
            locals.var_fs02_dps0_dn7 = assign89920_body52_e138550_d_n7;
            locals.var_fs02_dps0_dn8 = assign89920_body52_e138550_d_n8;
            locals.var_fs02_dps0_dn9 = assign89920_body52_e138550_d_n9;
            locals.var_fs02_dps0_dn10 = assign89920_body52_e138550_d_n10;
            locals.var_fs02_dps0_dn13 = assign89920_body52_e138550_d_n13;
            let (assign89920_body53_e138563, assign89920_body53_e138563_d_n0, assign89920_body53_e138563_d_n2, assign89920_body53_e138563_d_n4, assign89920_body53_e138563_d_n5, assign89920_body53_e138563_d_n6, assign89920_body53_e138563_d_n7, assign89920_body53_e138563_d_n8, assign89920_body53_e138563_d_n9, assign89920_body53_e138563_d_n10, assign89920_body53_e138563_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body53_e138555: f64 = (-locals.var_vgpld);
        let assign89920_body53_e138557: f64 = (assign89920_body53_e138555 + locals.var_ps0ld);
        let assign89920_body53_e138560: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign89920_body53_e138561: f64 = (assign89920_body53_e138557 + assign89920_body53_e138560);
        (assign89920_body53_e138561, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign89920_body53_e138563;
            locals.var_fs0_dn0 = assign89920_body53_e138563_d_n0;
            locals.var_fs0_dn2 = assign89920_body53_e138563_d_n2;
            locals.var_fs0_dn4 = assign89920_body53_e138563_d_n4;
            locals.var_fs0_dn5 = assign89920_body53_e138563_d_n5;
            locals.var_fs0_dn6 = assign89920_body53_e138563_d_n6;
            locals.var_fs0_dn7 = assign89920_body53_e138563_d_n7;
            locals.var_fs0_dn8 = assign89920_body53_e138563_d_n8;
            locals.var_fs0_dn9 = assign89920_body53_e138563_d_n9;
            locals.var_fs0_dn10 = assign89920_body53_e138563_d_n10;
            locals.var_fs0_dn13 = assign89920_body53_e138563_d_n13;
            let (assign89920_body54_e138573, assign89920_body54_e138573_d_n0, assign89920_body54_e138573_d_n2, assign89920_body54_e138573_d_n4, assign89920_body54_e138573_d_n5, assign89920_body54_e138573_d_n6, assign89920_body54_e138573_d_n7, assign89920_body54_e138573_d_n8, assign89920_body54_e138573_d_n9, assign89920_body54_e138573_d_n10, assign89920_body54_e138573_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body54_e138570: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign89920_body54_e138571: f64 = (1.0 + assign89920_body54_e138570);
        (assign89920_body54_e138571, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign89920_body54_e138573;
            locals.var_fs0_dps0_dn0 = assign89920_body54_e138573_d_n0;
            locals.var_fs0_dps0_dn2 = assign89920_body54_e138573_d_n2;
            locals.var_fs0_dps0_dn4 = assign89920_body54_e138573_d_n4;
            locals.var_fs0_dps0_dn5 = assign89920_body54_e138573_d_n5;
            locals.var_fs0_dps0_dn6 = assign89920_body54_e138573_d_n6;
            locals.var_fs0_dps0_dn7 = assign89920_body54_e138573_d_n7;
            locals.var_fs0_dps0_dn8 = assign89920_body54_e138573_d_n8;
            locals.var_fs0_dps0_dn9 = assign89920_body54_e138573_d_n9;
            locals.var_fs0_dps0_dn10 = assign89920_body54_e138573_d_n10;
            locals.var_fs0_dps0_dn13 = assign89920_body54_e138573_d_n13;
            let assign89920_body55_e138576: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard2096 = assign89920_body55_e138576;
            let (assign89920_body56_e138586,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 != 0.0)) {
        let assign89920_body56_e138584: f64 = (locals.var_lp_s0_max + 1.0);
        (assign89920_body56_e138584,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89920_body56_e138586;
            let (assign89920_body57_e138598, assign89920_body57_e138598_d_n0, assign89920_body57_e138598_d_n2, assign89920_body57_e138598_d_n4, assign89920_body57_e138598_d_n5, assign89920_body57_e138598_d_n6, assign89920_body57_e138598_d_n7, assign89920_body57_e138598_d_n8, assign89920_body57_e138598_d_n9, assign89920_body57_e138598_d_n10, assign89920_body57_e138598_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) {
        let assign89920_body57_e138594: f64 = (-locals.var_fs0);
        let assign89920_body57_e138596: f64 = (assign89920_body57_e138594 / locals.var_fs0_dps0);
        (assign89920_body57_e138596, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign89920_body57_e138594 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign89920_body57_e138598;
            locals.var_dps0_dn0 = assign89920_body57_e138598_d_n0;
            locals.var_dps0_dn2 = assign89920_body57_e138598_d_n2;
            locals.var_dps0_dn4 = assign89920_body57_e138598_d_n4;
            locals.var_dps0_dn5 = assign89920_body57_e138598_d_n5;
            locals.var_dps0_dn6 = assign89920_body57_e138598_d_n6;
            locals.var_dps0_dn7 = assign89920_body57_e138598_d_n7;
            locals.var_dps0_dn8 = assign89920_body57_e138598_d_n8;
            locals.var_dps0_dn9 = assign89920_body57_e138598_d_n9;
            locals.var_dps0_dn10 = assign89920_body57_e138598_d_n10;
            locals.var_dps0_dn13 = assign89920_body57_e138598_d_n13;
            let (assign89920_body58_e138620, assign89920_body58_e138620_d_n0, assign89920_body58_e138620_d_n2, assign89920_body58_e138620_d_n4, assign89920_body58_e138620_d_n5, assign89920_body58_e138620_d_n6, assign89920_body58_e138620_d_n7, assign89920_body58_e138620_d_n8, assign89920_body58_e138620_d_n9, assign89920_body58_e138620_d_n10, assign89920_body58_e138620_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) {
        let assign89920_body58_e138607: f64 = (0.5 * 0.1);
        let assign89920_body58_e138611: f64 = (locals.var_ps0ld).abs();
        let (assign89920_body58_e138616, assign89920_body58_e138616_d_n0, assign89920_body58_e138616_d_n2, assign89920_body58_e138616_d_n4, assign89920_body58_e138616_d_n5, assign89920_body58_e138616_d_n6, assign89920_body58_e138616_d_n7, assign89920_body58_e138616_d_n8, assign89920_body58_e138616_d_n9, assign89920_body58_e138616_d_n10, assign89920_body58_e138616_d_n13,) = {
            if (1.0 >= assign89920_body58_e138611) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign89920_body58_e138615: f64 = (locals.var_ps0ld).abs();
                (assign89920_body58_e138615, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign89920_body58_e138617: f64 = (1.0 + assign89920_body58_e138616);
        let assign89920_body58_e138618: f64 = (assign89920_body58_e138607 * assign89920_body58_e138617);
        (assign89920_body58_e138618, (assign89920_body58_e138607 * assign89920_body58_e138616_d_n0), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n2), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n4), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n5), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n6), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n7), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n8), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n9), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n10), (assign89920_body58_e138607 * assign89920_body58_e138616_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign89920_body58_e138620;
            locals.var_dplim_dn0 = assign89920_body58_e138620_d_n0;
            locals.var_dplim_dn2 = assign89920_body58_e138620_d_n2;
            locals.var_dplim_dn4 = assign89920_body58_e138620_d_n4;
            locals.var_dplim_dn5 = assign89920_body58_e138620_d_n5;
            locals.var_dplim_dn6 = assign89920_body58_e138620_d_n6;
            locals.var_dplim_dn7 = assign89920_body58_e138620_d_n7;
            locals.var_dplim_dn8 = assign89920_body58_e138620_d_n8;
            locals.var_dplim_dn9 = assign89920_body58_e138620_d_n9;
            locals.var_dplim_dn10 = assign89920_body58_e138620_d_n10;
            locals.var_dplim_dn13 = assign89920_body58_e138620_d_n13;
            let assign89920_body59_e138622: f64 = (locals.var_dps0).abs();
            let assign89920_body59_e138624: f64 = if assign89920_body59_e138622 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard2097 = assign89920_body59_e138624;
            let (assign89920_body60_e138643, assign89920_body60_e138643_d_n0, assign89920_body60_e138643_d_n2, assign89920_body60_e138643_d_n4, assign89920_body60_e138643_d_n5, assign89920_body60_e138643_d_n6, assign89920_body60_e138643_d_n7, assign89920_body60_e138643_d_n8, assign89920_body60_e138643_d_n9, assign89920_body60_e138643_d_n10, assign89920_body60_e138643_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2097 != 0.0)) {
        let (assign89920_body60_e138640,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign89920_body60_e138639: f64 = (-1.0);
                (assign89920_body60_e138639,)
            }
        };
        let assign89920_body60_e138641: f64 = (locals.var_dplim * assign89920_body60_e138640);
        (assign89920_body60_e138641, (locals.var_dplim_dn0 * assign89920_body60_e138640), (locals.var_dplim_dn2 * assign89920_body60_e138640), (locals.var_dplim_dn4 * assign89920_body60_e138640), (locals.var_dplim_dn5 * assign89920_body60_e138640), (locals.var_dplim_dn6 * assign89920_body60_e138640), (locals.var_dplim_dn7 * assign89920_body60_e138640), (locals.var_dplim_dn8 * assign89920_body60_e138640), (locals.var_dplim_dn9 * assign89920_body60_e138640), (locals.var_dplim_dn10 * assign89920_body60_e138640), (locals.var_dplim_dn13 * assign89920_body60_e138640),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign89920_body60_e138643;
            locals.var_dps0_dn0 = assign89920_body60_e138643_d_n0;
            locals.var_dps0_dn2 = assign89920_body60_e138643_d_n2;
            locals.var_dps0_dn4 = assign89920_body60_e138643_d_n4;
            locals.var_dps0_dn5 = assign89920_body60_e138643_d_n5;
            locals.var_dps0_dn6 = assign89920_body60_e138643_d_n6;
            locals.var_dps0_dn7 = assign89920_body60_e138643_d_n7;
            locals.var_dps0_dn8 = assign89920_body60_e138643_d_n8;
            locals.var_dps0_dn9 = assign89920_body60_e138643_d_n9;
            locals.var_dps0_dn10 = assign89920_body60_e138643_d_n10;
            locals.var_dps0_dn13 = assign89920_body60_e138643_d_n13;
            let (assign89920_body61_e138654, assign89920_body61_e138654_d_n0, assign89920_body61_e138654_d_n2, assign89920_body61_e138654_d_n4, assign89920_body61_e138654_d_n5, assign89920_body61_e138654_d_n6, assign89920_body61_e138654_d_n7, assign89920_body61_e138654_d_n8, assign89920_body61_e138654_d_n9, assign89920_body61_e138654_d_n10, assign89920_body61_e138654_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) {
        let assign89920_body61_e138652: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign89920_body61_e138652, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign89920_body61_e138654;
            locals.var_ps0ld_dn0 = assign89920_body61_e138654_d_n0;
            locals.var_ps0ld_dn2 = assign89920_body61_e138654_d_n2;
            locals.var_ps0ld_dn4 = assign89920_body61_e138654_d_n4;
            locals.var_ps0ld_dn5 = assign89920_body61_e138654_d_n5;
            locals.var_ps0ld_dn6 = assign89920_body61_e138654_d_n6;
            locals.var_ps0ld_dn7 = assign89920_body61_e138654_d_n7;
            locals.var_ps0ld_dn8 = assign89920_body61_e138654_d_n8;
            locals.var_ps0ld_dn9 = assign89920_body61_e138654_d_n9;
            locals.var_ps0ld_dn10 = assign89920_body61_e138654_d_n10;
            locals.var_ps0ld_dn13 = assign89920_body61_e138654_d_n13;
            let assign89920_body62_e138656: f64 = (locals.var_dps0).abs();
            let assign89920_body62_e138660: f64 = (locals.var_fs0).abs();
            let assign89920_body62_e138663: f64 = if ((assign89920_body62_e138656 <= 1e-12) && (assign89920_body62_e138660 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard2098 = assign89920_body62_e138663;
            let (assign89920_body63_e138676,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) && (locals.var_guard2096 == 0.0)) && (locals.var_guard2098 != 0.0)) {
        let assign89920_body63_e138674: f64 = (locals.var_flg_conv + 2.0);
        (assign89920_body63_e138674,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign89920_body63_e138676;
            let (assign89920_body64_e138684,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89920_body64_e138682: f64 = (locals.var_lp_s0 + 1.0);
        (assign89920_body64_e138682,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign89920_body64_e138684;
        }

    }

    pub(super) fn stamp_transient_block_319(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign89940_e138707, assign89940_e138707_d_n0, assign89940_e138707_d_n2, assign89940_e138707_d_n4, assign89940_e138707_d_n5, assign89940_e138707_d_n6, assign89940_e138707_d_n7, assign89940_e138707_d_n8, assign89940_e138707_d_n9, assign89940_e138707_d_n10, assign89940_e138707_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let (assign89940_e138705, assign89940_e138705_d_n0, assign89940_e138705_d_n2, assign89940_e138705_d_n4, assign89940_e138705_d_n5, assign89940_e138705_d_n6, assign89940_e138705_d_n7, assign89940_e138705_d_n8, assign89940_e138705_d_n9, assign89940_e138705_d_n10, assign89940_e138705_d_n13,) = {
            if (locals.var_fbsq__blk2012 >= 0.0) {
                let (assign89940_e138700,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign89940_e138699: f64 = (-1.0);
                        (assign89940_e138699,)
                    }
                };
                let assign89940_e138702: f64 = (locals.var_fbsq__blk2012).sqrt();
                let assign89940_e138703: f64 = (assign89940_e138700 * assign89940_e138702);
                (assign89940_e138703, (assign89940_e138700 * (locals.var_fbsq__blk2012_dn0 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn2 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn4 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn5 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn6 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn7 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn8 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn9 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn10 / (2.0 * assign89940_e138702))), (assign89940_e138700 * (locals.var_fbsq__blk2012_dn13 / (2.0 * assign89940_e138702))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign89940_e138705, assign89940_e138705_d_n0, assign89940_e138705_d_n2, assign89940_e138705_d_n4, assign89940_e138705_d_n5, assign89940_e138705_d_n6, assign89940_e138705_d_n7, assign89940_e138705_d_n8, assign89940_e138705_d_n9, assign89940_e138705_d_n10, assign89940_e138705_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign89940_e138707;
        locals.var_fb_dn0 = assign89940_e138707_d_n0;
        locals.var_fb_dn2 = assign89940_e138707_d_n2;
        locals.var_fb_dn4 = assign89940_e138707_d_n4;
        locals.var_fb_dn5 = assign89940_e138707_d_n5;
        locals.var_fb_dn6 = assign89940_e138707_d_n6;
        locals.var_fb_dn7 = assign89940_e138707_d_n7;
        locals.var_fb_dn8 = assign89940_e138707_d_n8;
        locals.var_fb_dn9 = assign89940_e138707_d_n9;
        locals.var_fb_dn10 = assign89940_e138707_d_n10;
        locals.var_fb_dn13 = assign89940_e138707_d_n13;

        let (assign89950_e138715, assign89950_e138715_d_n0, assign89950_e138715_d_n2, assign89950_e138715_d_n4, assign89950_e138715_d_n5, assign89950_e138715_d_n6, assign89950_e138715_d_n7, assign89950_e138715_d_n8, assign89950_e138715_d_n9, assign89950_e138715_d_n10, assign89950_e138715_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89950_e138713: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign89950_e138713, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk2002, locals.var_wdld__blk2002_dn0, locals.var_wdld__blk2002_dn2, locals.var_wdld__blk2002_dn4, locals.var_wdld__blk2002_dn5, locals.var_wdld__blk2002_dn6, locals.var_wdld__blk2002_dn7, locals.var_wdld__blk2002_dn8, locals.var_wdld__blk2002_dn9, locals.var_wdld__blk2002_dn10, locals.var_wdld__blk2002_dn13,)
    }
};
        locals.var_wdld__blk2002 = assign89950_e138715;
        locals.var_wdld__blk2002_dn0 = assign89950_e138715_d_n0;
        locals.var_wdld__blk2002_dn2 = assign89950_e138715_d_n2;
        locals.var_wdld__blk2002_dn4 = assign89950_e138715_d_n4;
        locals.var_wdld__blk2002_dn5 = assign89950_e138715_d_n5;
        locals.var_wdld__blk2002_dn6 = assign89950_e138715_d_n6;
        locals.var_wdld__blk2002_dn7 = assign89950_e138715_d_n7;
        locals.var_wdld__blk2002_dn8 = assign89950_e138715_d_n8;
        locals.var_wdld__blk2002_dn9 = assign89950_e138715_d_n9;
        locals.var_wdld__blk2002_dn10 = assign89950_e138715_d_n10;
        locals.var_wdld__blk2002_dn13 = assign89950_e138715_d_n13;

        let (assign89960_e138723, assign89960_e138723_d_n0, assign89960_e138723_d_n2, assign89960_e138723_d_n4, assign89960_e138723_d_n5, assign89960_e138723_d_n6, assign89960_e138723_d_n7, assign89960_e138723_d_n8, assign89960_e138723_d_n9, assign89960_e138723_d_n10, assign89960_e138723_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89960_e138721: f64 = (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002);
        (assign89960_e138721, (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn0), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn2), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn4), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn5), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn6), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn7), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn8), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn9), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn10), (locals.var_q_nsubld__blk2004 * locals.var_wdld__blk2002_dn13),)
    } else {
        (locals.var_q_dep_ld__blk2003, locals.var_q_dep_ld__blk2003_dn0, locals.var_q_dep_ld__blk2003_dn2, locals.var_q_dep_ld__blk2003_dn4, locals.var_q_dep_ld__blk2003_dn5, locals.var_q_dep_ld__blk2003_dn6, locals.var_q_dep_ld__blk2003_dn7, locals.var_q_dep_ld__blk2003_dn8, locals.var_q_dep_ld__blk2003_dn9, locals.var_q_dep_ld__blk2003_dn10, locals.var_q_dep_ld__blk2003_dn13,)
    }
};
        locals.var_q_dep_ld__blk2003 = assign89960_e138723;
        locals.var_q_dep_ld__blk2003_dn0 = assign89960_e138723_d_n0;
        locals.var_q_dep_ld__blk2003_dn2 = assign89960_e138723_d_n2;
        locals.var_q_dep_ld__blk2003_dn4 = assign89960_e138723_d_n4;
        locals.var_q_dep_ld__blk2003_dn5 = assign89960_e138723_d_n5;
        locals.var_q_dep_ld__blk2003_dn6 = assign89960_e138723_d_n6;
        locals.var_q_dep_ld__blk2003_dn7 = assign89960_e138723_d_n7;
        locals.var_q_dep_ld__blk2003_dn8 = assign89960_e138723_d_n8;
        locals.var_q_dep_ld__blk2003_dn9 = assign89960_e138723_d_n9;
        locals.var_q_dep_ld__blk2003_dn10 = assign89960_e138723_d_n10;
        locals.var_q_dep_ld__blk2003_dn13 = assign89960_e138723_d_n13;

        let (assign89970_e138735, assign89970_e138735_d_n0, assign89970_e138735_d_n2, assign89970_e138735_d_n4, assign89970_e138735_d_n5, assign89970_e138735_d_n6, assign89970_e138735_d_n7, assign89970_e138735_d_n8, assign89970_e138735_d_n9, assign89970_e138735_d_n10, assign89970_e138735_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89970_e138729: f64 = (locals.var_q_dep_ld__blk2003 / locals.var_cnst0over_func);
        let assign89970_e138732: f64 = (10.0 * 2.220446049250313e-16);
        let assign89970_e138733: f64 = (assign89970_e138729 + assign89970_e138732);
        (assign89970_e138733, (((locals.var_q_dep_ld__blk2003_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk2003_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk2003 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign89970_e138735;
        locals.var_xi0p12_dn0 = assign89970_e138735_d_n0;
        locals.var_xi0p12_dn2 = assign89970_e138735_d_n2;
        locals.var_xi0p12_dn4 = assign89970_e138735_d_n4;
        locals.var_xi0p12_dn5 = assign89970_e138735_d_n5;
        locals.var_xi0p12_dn6 = assign89970_e138735_d_n6;
        locals.var_xi0p12_dn7 = assign89970_e138735_d_n7;
        locals.var_xi0p12_dn8 = assign89970_e138735_d_n8;
        locals.var_xi0p12_dn9 = assign89970_e138735_d_n9;
        locals.var_xi0p12_dn10 = assign89970_e138735_d_n10;
        locals.var_xi0p12_dn13 = assign89970_e138735_d_n13;

        let (assign89980_e138743, assign89980_e138743_d_n0, assign89980_e138743_d_n2, assign89980_e138743_d_n4, assign89980_e138743_d_n5, assign89980_e138743_d_n6, assign89980_e138743_d_n7, assign89980_e138743_d_n8, assign89980_e138743_d_n9, assign89980_e138743_d_n10, assign89980_e138743_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89980_e138741: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign89980_e138741, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign89980_e138743;
        locals.var_qbuld_dn0 = assign89980_e138743_d_n0;
        locals.var_qbuld_dn2 = assign89980_e138743_d_n2;
        locals.var_qbuld_dn4 = assign89980_e138743_d_n4;
        locals.var_qbuld_dn5 = assign89980_e138743_d_n5;
        locals.var_qbuld_dn6 = assign89980_e138743_d_n6;
        locals.var_qbuld_dn7 = assign89980_e138743_d_n7;
        locals.var_qbuld_dn8 = assign89980_e138743_d_n8;
        locals.var_qbuld_dn9 = assign89980_e138743_d_n9;
        locals.var_qbuld_dn10 = assign89980_e138743_d_n10;
        locals.var_qbuld_dn13 = assign89980_e138743_d_n13;

        let (assign89990_e138753, assign89990_e138753_d_n0, assign89990_e138753_d_n2, assign89990_e138753_d_n4, assign89990_e138753_d_n5, assign89990_e138753_d_n6, assign89990_e138753_d_n7, assign89990_e138753_d_n8, assign89990_e138753_d_n9, assign89990_e138753_d_n10, assign89990_e138753_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign89990_e138750: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign89990_e138751: f64 = (1.0 / assign89990_e138750);
        (assign89990_e138751, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign89990_e138750 * assign89990_e138750))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign89990_e138750 * assign89990_e138750))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign89990_e138753;
        locals.var_t1_dn0 = assign89990_e138753_d_n0;
        locals.var_t1_dn2 = assign89990_e138753_d_n2;
        locals.var_t1_dn4 = assign89990_e138753_d_n4;
        locals.var_t1_dn5 = assign89990_e138753_d_n5;
        locals.var_t1_dn6 = assign89990_e138753_d_n6;
        locals.var_t1_dn7 = assign89990_e138753_d_n7;
        locals.var_t1_dn8 = assign89990_e138753_d_n8;
        locals.var_t1_dn9 = assign89990_e138753_d_n9;
        locals.var_t1_dn10 = assign89990_e138753_d_n10;
        locals.var_t1_dn13 = assign89990_e138753_d_n13;

        let (assign90000_e138763, assign90000_e138763_d_n0, assign90000_e138763_d_n2, assign90000_e138763_d_n4, assign90000_e138763_d_n5, assign90000_e138763_d_n6, assign90000_e138763_d_n7, assign90000_e138763_d_n8, assign90000_e138763_d_n9, assign90000_e138763_d_n10, assign90000_e138763_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign90000_e138759: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign90000_e138761: f64 = (assign90000_e138759 * locals.var_t1);
        (assign90000_e138761, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign90000_e138759 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign90000_e138763;
        locals.var_qiuld_dn0 = assign90000_e138763_d_n0;
        locals.var_qiuld_dn2 = assign90000_e138763_d_n2;
        locals.var_qiuld_dn4 = assign90000_e138763_d_n4;
        locals.var_qiuld_dn5 = assign90000_e138763_d_n5;
        locals.var_qiuld_dn6 = assign90000_e138763_d_n6;
        locals.var_qiuld_dn7 = assign90000_e138763_d_n7;
        locals.var_qiuld_dn8 = assign90000_e138763_d_n8;
        locals.var_qiuld_dn9 = assign90000_e138763_d_n9;
        locals.var_qiuld_dn10 = assign90000_e138763_d_n10;
        locals.var_qiuld_dn13 = assign90000_e138763_d_n13;

        let (assign90010_e138771, assign90010_e138771_d_n0, assign90010_e138771_d_n2, assign90010_e138771_d_n4, assign90010_e138771_d_n5, assign90010_e138771_d_n6, assign90010_e138771_d_n7, assign90010_e138771_d_n8, assign90010_e138771_d_n9, assign90010_e138771_d_n10, assign90010_e138771_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2085 != 0.0)) {
        let assign90010_e138769: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign90010_e138769, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign90010_e138771;
        locals.var_qsuld_dn0 = assign90010_e138771_d_n0;
        locals.var_qsuld_dn2 = assign90010_e138771_d_n2;
        locals.var_qsuld_dn4 = assign90010_e138771_d_n4;
        locals.var_qsuld_dn5 = assign90010_e138771_d_n5;
        locals.var_qsuld_dn6 = assign90010_e138771_d_n6;
        locals.var_qsuld_dn7 = assign90010_e138771_d_n7;
        locals.var_qsuld_dn8 = assign90010_e138771_d_n8;
        locals.var_qsuld_dn9 = assign90010_e138771_d_n9;
        locals.var_qsuld_dn10 = assign90010_e138771_d_n10;
        locals.var_qsuld_dn13 = assign90010_e138771_d_n13;

        let (assign90020_e138777, assign90020_e138777_d_n0, assign90020_e138777_d_n2, assign90020_e138777_d_n4, assign90020_e138777_d_n5, assign90020_e138777_d_n6, assign90020_e138777_d_n7, assign90020_e138777_d_n8, assign90020_e138777_d_n9, assign90020_e138777_d_n10, assign90020_e138777_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign90020_e138775: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign90020_e138775, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign90020_e138777;
        locals.var_qiuld_dn0 = assign90020_e138777_d_n0;
        locals.var_qiuld_dn2 = assign90020_e138777_d_n2;
        locals.var_qiuld_dn4 = assign90020_e138777_d_n4;
        locals.var_qiuld_dn5 = assign90020_e138777_d_n5;
        locals.var_qiuld_dn6 = assign90020_e138777_d_n6;
        locals.var_qiuld_dn7 = assign90020_e138777_d_n7;
        locals.var_qiuld_dn8 = assign90020_e138777_d_n8;
        locals.var_qiuld_dn9 = assign90020_e138777_d_n9;
        locals.var_qiuld_dn10 = assign90020_e138777_d_n10;
        locals.var_qiuld_dn13 = assign90020_e138777_d_n13;

        let assign90030_e138780: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2100 = assign90030_e138780;

        let (assign90040_e138787, assign90040_e138787_d_n0, assign90040_e138787_d_n2, assign90040_e138787_d_n4, assign90040_e138787_d_n5, assign90040_e138787_d_n6, assign90040_e138787_d_n7, assign90040_e138787_d_n8, assign90040_e138787_d_n9, assign90040_e138787_d_n10, assign90040_e138787_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) {
        let assign90040_e138785: f64 = (-locals.var_lover_func);
        (assign90040_e138785, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign90040_e138787;
        locals.var_lover_func_dn0 = assign90040_e138787_d_n0;
        locals.var_lover_func_dn2 = assign90040_e138787_d_n2;
        locals.var_lover_func_dn4 = assign90040_e138787_d_n4;
        locals.var_lover_func_dn5 = assign90040_e138787_d_n5;
        locals.var_lover_func_dn6 = assign90040_e138787_d_n6;
        locals.var_lover_func_dn7 = assign90040_e138787_d_n7;
        locals.var_lover_func_dn8 = assign90040_e138787_d_n8;
        locals.var_lover_func_dn9 = assign90040_e138787_d_n9;
        locals.var_lover_func_dn10 = assign90040_e138787_d_n10;
        locals.var_lover_func_dn13 = assign90040_e138787_d_n13;

        let assign90050_e138790: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2101 = assign90050_e138790;

        let assign90060_e138793: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2102 = assign90060_e138793;

        let (assign90070_e138804, assign90070_e138804_d_n0, assign90070_e138804_d_n2, assign90070_e138804_d_n4, assign90070_e138804_d_n5, assign90070_e138804_d_n6, assign90070_e138804_d_n7, assign90070_e138804_d_n8, assign90070_e138804_d_n9, assign90070_e138804_d_n10, assign90070_e138804_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) && (locals.var_guard2102 != 0.0)) {
        let assign90070_e138802: f64 = (-locals.var_ps0ld);
        (assign90070_e138802, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx__blk2005, locals.var_vx__blk2005_dn0, locals.var_vx__blk2005_dn2, locals.var_vx__blk2005_dn4, locals.var_vx__blk2005_dn5, locals.var_vx__blk2005_dn6, locals.var_vx__blk2005_dn7, locals.var_vx__blk2005_dn8, locals.var_vx__blk2005_dn9, locals.var_vx__blk2005_dn10, locals.var_vx__blk2005_dn13,)
    }
};
        locals.var_vx__blk2005 = assign90070_e138804;
        locals.var_vx__blk2005_dn0 = assign90070_e138804_d_n0;
        locals.var_vx__blk2005_dn2 = assign90070_e138804_d_n2;
        locals.var_vx__blk2005_dn4 = assign90070_e138804_d_n4;
        locals.var_vx__blk2005_dn5 = assign90070_e138804_d_n5;
        locals.var_vx__blk2005_dn6 = assign90070_e138804_d_n6;
        locals.var_vx__blk2005_dn7 = assign90070_e138804_d_n7;
        locals.var_vx__blk2005_dn8 = assign90070_e138804_d_n8;
        locals.var_vx__blk2005_dn9 = assign90070_e138804_d_n9;
        locals.var_vx__blk2005_dn10 = assign90070_e138804_d_n10;
        locals.var_vx__blk2005_dn13 = assign90070_e138804_d_n13;

        let (assign90080_e138815, assign90080_e138815_d_n0, assign90080_e138815_d_n2, assign90080_e138815_d_n4, assign90080_e138815_d_n5, assign90080_e138815_d_n6, assign90080_e138815_d_n7, assign90080_e138815_d_n8, assign90080_e138815_d_n9, assign90080_e138815_d_n10, assign90080_e138815_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) && (locals.var_guard2102 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx__blk2005, locals.var_vx__blk2005_dn0, locals.var_vx__blk2005_dn2, locals.var_vx__blk2005_dn4, locals.var_vx__blk2005_dn5, locals.var_vx__blk2005_dn6, locals.var_vx__blk2005_dn7, locals.var_vx__blk2005_dn8, locals.var_vx__blk2005_dn9, locals.var_vx__blk2005_dn10, locals.var_vx__blk2005_dn13,)
    }
};
        locals.var_vx__blk2005 = assign90080_e138815;
        locals.var_vx__blk2005_dn0 = assign90080_e138815_d_n0;
        locals.var_vx__blk2005_dn2 = assign90080_e138815_d_n2;
        locals.var_vx__blk2005_dn4 = assign90080_e138815_d_n4;
        locals.var_vx__blk2005_dn5 = assign90080_e138815_d_n5;
        locals.var_vx__blk2005_dn6 = assign90080_e138815_d_n6;
        locals.var_vx__blk2005_dn7 = assign90080_e138815_d_n7;
        locals.var_vx__blk2005_dn8 = assign90080_e138815_d_n8;
        locals.var_vx__blk2005_dn9 = assign90080_e138815_d_n9;
        locals.var_vx__blk2005_dn10 = assign90080_e138815_d_n10;
        locals.var_vx__blk2005_dn13 = assign90080_e138815_d_n13;

        let (assign90090_e138836, assign90090_e138836_d_n0, assign90090_e138836_d_n2, assign90090_e138836_d_n4, assign90090_e138836_d_n5, assign90090_e138836_d_n6, assign90090_e138836_d_n7, assign90090_e138836_d_n8, assign90090_e138836_d_n9, assign90090_e138836_d_n10, assign90090_e138836_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90090_e138823: f64 = (locals.var_vx__blk2005 + p.p137);
        let assign90090_e138826: f64 = (locals.var_vx__blk2005 + p.p137);
        let assign90090_e138827: f64 = (assign90090_e138823 * assign90090_e138826);
        let assign90090_e138830: f64 = (4.0 * 0.1);
        let assign90090_e138832: f64 = (assign90090_e138830 * 0.1);
        let assign90090_e138833: f64 = (assign90090_e138827 + assign90090_e138832);
        let assign90090_e138834: f64 = (assign90090_e138833).sqrt();
        (assign90090_e138834, (((locals.var_vx__blk2005_dn0 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn0)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn2 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn2)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn4 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn4)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn5 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn5)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn6 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn6)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn7 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn7)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn8 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn8)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn9 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn9)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn10 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn10)) / (2.0 * assign90090_e138834)), (((locals.var_vx__blk2005_dn13 * assign90090_e138826) + (assign90090_e138823 * locals.var_vx__blk2005_dn13)) / (2.0 * assign90090_e138834)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90090_e138836;
        locals.var_tmf2_dn0 = assign90090_e138836_d_n0;
        locals.var_tmf2_dn2 = assign90090_e138836_d_n2;
        locals.var_tmf2_dn4 = assign90090_e138836_d_n4;
        locals.var_tmf2_dn5 = assign90090_e138836_d_n5;
        locals.var_tmf2_dn6 = assign90090_e138836_d_n6;
        locals.var_tmf2_dn7 = assign90090_e138836_d_n7;
        locals.var_tmf2_dn8 = assign90090_e138836_d_n8;
        locals.var_tmf2_dn9 = assign90090_e138836_d_n9;
        locals.var_tmf2_dn10 = assign90090_e138836_d_n10;
        locals.var_tmf2_dn13 = assign90090_e138836_d_n13;

        let (assign90100_e138852, assign90100_e138852_d_n0, assign90100_e138852_d_n2, assign90100_e138852_d_n4, assign90100_e138852_d_n5, assign90100_e138852_d_n6, assign90100_e138852_d_n7, assign90100_e138852_d_n8, assign90100_e138852_d_n9, assign90100_e138852_d_n10, assign90100_e138852_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90100_e138846: f64 = (locals.var_vx__blk2005 + p.p137);
        let assign90100_e138848: f64 = (assign90100_e138846 / locals.var_tmf2);
        let assign90100_e138849: f64 = (1.0 + assign90100_e138848);
        let assign90100_e138850: f64 = (0.5 * assign90100_e138849);
        (assign90100_e138850, (0.5 * (((locals.var_vx__blk2005_dn0 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn2 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn4 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn5 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn6 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn7 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn8 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn9 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn10 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk2005_dn13 * locals.var_tmf2) - (assign90100_e138846 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign90100_e138852;
        locals.var_t9_dn0 = assign90100_e138852_d_n0;
        locals.var_t9_dn2 = assign90100_e138852_d_n2;
        locals.var_t9_dn4 = assign90100_e138852_d_n4;
        locals.var_t9_dn5 = assign90100_e138852_d_n5;
        locals.var_t9_dn6 = assign90100_e138852_d_n6;
        locals.var_t9_dn7 = assign90100_e138852_d_n7;
        locals.var_t9_dn8 = assign90100_e138852_d_n8;
        locals.var_t9_dn9 = assign90100_e138852_d_n9;
        locals.var_t9_dn10 = assign90100_e138852_d_n10;
        locals.var_t9_dn13 = assign90100_e138852_d_n13;

        let (assign90110_e138866, assign90110_e138866_d_n0, assign90110_e138866_d_n2, assign90110_e138866_d_n4, assign90110_e138866_d_n5, assign90110_e138866_d_n6, assign90110_e138866_d_n7, assign90110_e138866_d_n8, assign90110_e138866_d_n9, assign90110_e138866_d_n10, assign90110_e138866_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90110_e138861: f64 = (locals.var_vx__blk2005 + p.p137);
        let assign90110_e138863: f64 = (assign90110_e138861 + locals.var_tmf2);
        let assign90110_e138864: f64 = (0.5 * assign90110_e138863);
        (assign90110_e138864, (0.5 * (locals.var_vx__blk2005_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk2005_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk2005_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk2005_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk2005_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk2005_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk2005_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk2005_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk2005_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk2005_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign90110_e138866;
        locals.var_t2_dn0 = assign90110_e138866_d_n0;
        locals.var_t2_dn2 = assign90110_e138866_d_n2;
        locals.var_t2_dn4 = assign90110_e138866_d_n4;
        locals.var_t2_dn5 = assign90110_e138866_d_n5;
        locals.var_t2_dn6 = assign90110_e138866_d_n6;
        locals.var_t2_dn7 = assign90110_e138866_d_n7;
        locals.var_t2_dn8 = assign90110_e138866_d_n8;
        locals.var_t2_dn9 = assign90110_e138866_d_n9;
        locals.var_t2_dn10 = assign90110_e138866_d_n10;
        locals.var_t2_dn13 = assign90110_e138866_d_n13;

        let assign90120_e138869: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2103 = assign90120_e138869;

        let (assign90130_e138879, assign90130_e138879_d_n0, assign90130_e138879_d_n2, assign90130_e138879_d_n4, assign90130_e138879_d_n5, assign90130_e138879_d_n6, assign90130_e138879_d_n7, assign90130_e138879_d_n8, assign90130_e138879_d_n9, assign90130_e138879_d_n10, assign90130_e138879_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign90130_e138879;
        locals.var_t2_dn0 = assign90130_e138879_d_n0;
        locals.var_t2_dn2 = assign90130_e138879_d_n2;
        locals.var_t2_dn4 = assign90130_e138879_d_n4;
        locals.var_t2_dn5 = assign90130_e138879_d_n5;
        locals.var_t2_dn6 = assign90130_e138879_d_n6;
        locals.var_t2_dn7 = assign90130_e138879_d_n7;
        locals.var_t2_dn8 = assign90130_e138879_d_n8;
        locals.var_t2_dn9 = assign90130_e138879_d_n9;
        locals.var_t2_dn10 = assign90130_e138879_d_n10;
        locals.var_t2_dn13 = assign90130_e138879_d_n13;

        let (assign90140_e138889, assign90140_e138889_d_n0, assign90140_e138889_d_n2, assign90140_e138889_d_n4, assign90140_e138889_d_n5, assign90140_e138889_d_n6, assign90140_e138889_d_n7, assign90140_e138889_d_n8, assign90140_e138889_d_n9, assign90140_e138889_d_n10, assign90140_e138889_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) && (locals.var_guard2103 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign90140_e138889;
        locals.var_t9_dn0 = assign90140_e138889_d_n0;
        locals.var_t9_dn2 = assign90140_e138889_d_n2;
        locals.var_t9_dn4 = assign90140_e138889_d_n4;
        locals.var_t9_dn5 = assign90140_e138889_d_n5;
        locals.var_t9_dn6 = assign90140_e138889_d_n6;
        locals.var_t9_dn7 = assign90140_e138889_d_n7;
        locals.var_t9_dn8 = assign90140_e138889_d_n8;
        locals.var_t9_dn9 = assign90140_e138889_d_n9;
        locals.var_t9_dn10 = assign90140_e138889_d_n10;
        locals.var_t9_dn13 = assign90140_e138889_d_n13;

        let (assign90150_e138902, assign90150_e138902_d_n0, assign90150_e138902_d_n2, assign90150_e138902_d_n4, assign90150_e138902_d_n5, assign90150_e138902_d_n6, assign90150_e138902_d_n7, assign90150_e138902_d_n8, assign90150_e138902_d_n9, assign90150_e138902_d_n10, assign90150_e138902_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90150_e138897: f64 = (locals.var_kjunc * locals.var_t2);
        let assign90150_e138898: f64 = (assign90150_e138897).sqrt();
        let assign90150_e138900: f64 = (assign90150_e138898 * p.p432);
        (assign90150_e138900, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign90150_e138898)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign90150_e138898)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign90150_e138902;
        locals.var_wjunc0_dn0 = assign90150_e138902_d_n0;
        locals.var_wjunc0_dn2 = assign90150_e138902_d_n2;
        locals.var_wjunc0_dn4 = assign90150_e138902_d_n4;
        locals.var_wjunc0_dn5 = assign90150_e138902_d_n5;
        locals.var_wjunc0_dn6 = assign90150_e138902_d_n6;
        locals.var_wjunc0_dn7 = assign90150_e138902_d_n7;
        locals.var_wjunc0_dn8 = assign90150_e138902_d_n8;
        locals.var_wjunc0_dn9 = assign90150_e138902_d_n9;
        locals.var_wjunc0_dn10 = assign90150_e138902_d_n10;
        locals.var_wjunc0_dn13 = assign90150_e138902_d_n13;

        let (assign90160_e138916, assign90160_e138916_d_n0, assign90160_e138916_d_n2, assign90160_e138916_d_n4, assign90160_e138916_d_n5, assign90160_e138916_d_n6, assign90160_e138916_d_n7, assign90160_e138916_d_n8, assign90160_e138916_d_n9, assign90160_e138916_d_n10, assign90160_e138916_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90160_e138910: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign90160_e138913: f64 = (0.1 * locals.var_lover_func);
        let assign90160_e138914: f64 = (assign90160_e138910 - assign90160_e138913);
        (assign90160_e138914, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign90160_e138916;
        locals.var_tmf1_dn0 = assign90160_e138916_d_n0;
        locals.var_tmf1_dn2 = assign90160_e138916_d_n2;
        locals.var_tmf1_dn4 = assign90160_e138916_d_n4;
        locals.var_tmf1_dn5 = assign90160_e138916_d_n5;
        locals.var_tmf1_dn6 = assign90160_e138916_d_n6;
        locals.var_tmf1_dn7 = assign90160_e138916_d_n7;
        locals.var_tmf1_dn8 = assign90160_e138916_d_n8;
        locals.var_tmf1_dn9 = assign90160_e138916_d_n9;
        locals.var_tmf1_dn10 = assign90160_e138916_d_n10;
        locals.var_tmf1_dn13 = assign90160_e138916_d_n13;

        let (assign90170_e138930, assign90170_e138930_d_n0, assign90170_e138930_d_n2, assign90170_e138930_d_n4, assign90170_e138930_d_n5, assign90170_e138930_d_n6, assign90170_e138930_d_n7, assign90170_e138930_d_n8, assign90170_e138930_d_n9, assign90170_e138930_d_n10, assign90170_e138930_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90170_e138924: f64 = (4.0 * locals.var_lover_func);
        let assign90170_e138927: f64 = (0.1 * locals.var_lover_func);
        let assign90170_e138928: f64 = (assign90170_e138924 * assign90170_e138927);
        (assign90170_e138928, (((4.0 * locals.var_lover_func_dn0) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign90170_e138927) + (assign90170_e138924 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90170_e138930;
        locals.var_tmf2_dn0 = assign90170_e138930_d_n0;
        locals.var_tmf2_dn2 = assign90170_e138930_d_n2;
        locals.var_tmf2_dn4 = assign90170_e138930_d_n4;
        locals.var_tmf2_dn5 = assign90170_e138930_d_n5;
        locals.var_tmf2_dn6 = assign90170_e138930_d_n6;
        locals.var_tmf2_dn7 = assign90170_e138930_d_n7;
        locals.var_tmf2_dn8 = assign90170_e138930_d_n8;
        locals.var_tmf2_dn9 = assign90170_e138930_d_n9;
        locals.var_tmf2_dn10 = assign90170_e138930_d_n10;
        locals.var_tmf2_dn13 = assign90170_e138930_d_n13;

        let (assign90180_e138944, assign90180_e138944_d_n0, assign90180_e138944_d_n2, assign90180_e138944_d_n4, assign90180_e138944_d_n5, assign90180_e138944_d_n6, assign90180_e138944_d_n7, assign90180_e138944_d_n8, assign90180_e138944_d_n9, assign90180_e138944_d_n10, assign90180_e138944_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let (assign90180_e138942, assign90180_e138942_d_n0, assign90180_e138942_d_n2, assign90180_e138942_d_n4, assign90180_e138942_d_n5, assign90180_e138942_d_n6, assign90180_e138942_d_n7, assign90180_e138942_d_n8, assign90180_e138942_d_n9, assign90180_e138942_d_n10, assign90180_e138942_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign90180_e138941: f64 = (-locals.var_tmf2);
                (assign90180_e138941, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign90180_e138942, assign90180_e138942_d_n0, assign90180_e138942_d_n2, assign90180_e138942_d_n4, assign90180_e138942_d_n5, assign90180_e138942_d_n6, assign90180_e138942_d_n7, assign90180_e138942_d_n8, assign90180_e138942_d_n9, assign90180_e138942_d_n10, assign90180_e138942_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90180_e138944;
        locals.var_tmf2_dn0 = assign90180_e138944_d_n0;
        locals.var_tmf2_dn2 = assign90180_e138944_d_n2;
        locals.var_tmf2_dn4 = assign90180_e138944_d_n4;
        locals.var_tmf2_dn5 = assign90180_e138944_d_n5;
        locals.var_tmf2_dn6 = assign90180_e138944_d_n6;
        locals.var_tmf2_dn7 = assign90180_e138944_d_n7;
        locals.var_tmf2_dn8 = assign90180_e138944_d_n8;
        locals.var_tmf2_dn9 = assign90180_e138944_d_n9;
        locals.var_tmf2_dn10 = assign90180_e138944_d_n10;
        locals.var_tmf2_dn13 = assign90180_e138944_d_n13;

        let (assign90190_e138957, assign90190_e138957_d_n0, assign90190_e138957_d_n2, assign90190_e138957_d_n4, assign90190_e138957_d_n5, assign90190_e138957_d_n6, assign90190_e138957_d_n7, assign90190_e138957_d_n8, assign90190_e138957_d_n9, assign90190_e138957_d_n10, assign90190_e138957_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2100 != 0.0)) && (locals.var_guard2101 != 0.0)) {
        let assign90190_e138952: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign90190_e138954: f64 = (assign90190_e138952 + locals.var_tmf2);
        let assign90190_e138955: f64 = (assign90190_e138954).sqrt();
        (assign90190_e138955, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign90190_e138955)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign90190_e138955)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign90190_e138957;
        locals.var_tmf2_dn0 = assign90190_e138957_d_n0;
        locals.var_tmf2_dn2 = assign90190_e138957_d_n2;
        locals.var_tmf2_dn4 = assign90190_e138957_d_n4;
        locals.var_tmf2_dn5 = assign90190_e138957_d_n5;
        locals.var_tmf2_dn6 = assign90190_e138957_d_n6;
        locals.var_tmf2_dn7 = assign90190_e138957_d_n7;
        locals.var_tmf2_dn8 = assign90190_e138957_d_n8;
        locals.var_tmf2_dn9 = assign90190_e138957_d_n9;
        locals.var_tmf2_dn10 = assign90190_e138957_d_n10;
        locals.var_tmf2_dn13 = assign90190_e138957_d_n13;

    }
}
